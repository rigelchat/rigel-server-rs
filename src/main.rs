mod db;
mod models;
mod routes;
mod services;
mod state;
mod ws;
mod utils;

use axum::Router;
use tower_http::{cors::{Any as CorsAny, CorsLayer}, services::ServeDir};
use std::{sync::Arc, collections::HashMap, env};
use tokio::{net::TcpListener, sync::RwLock};
use dotenvy::dotenv;
use tracing::info;
use tracing_subscriber::EnvFilter;

use crate::state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let db_pool = db::init()
        .await
        .expect("Failed to connect to the database");

    let state = AppState { 
        db: db_pool,
        sessions: Arc::new(RwLock::new(HashMap::new()))
    };

    let cors = CorsLayer::new()
        .allow_origin(CorsAny)
        .allow_methods(CorsAny)
        .allow_headers(CorsAny);

    let app = Router::new()
        .nest_service("/cdn", ServeDir::new("static"))
        .merge(routes::router())
        .route("/gateway", axum::routing::get(ws::gateway::handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()), env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    let listener = TcpListener::bind(addr).await.unwrap();
    info!("API listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}