mod routes;
mod db;
mod utils;
mod ws;

use axum::{Router, http::Method};
use tower_http::{cors::{Any, CorsLayer}, services::ServeDir};
use dotenvy::dotenv;
use tracing::{info};
use tracing_subscriber::EnvFilter;
use std::env;
pub use db::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let db_pool = db::init().await.expect("Failed to connect to the database");

    let state = AppState { 
        db: db_pool,
        sessions: std::sync::Arc::new(tokio::sync::RwLock::new(std::collections::HashMap::new())),
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(Any);

    let app = Router::new()
        .nest_service("/cdn", ServeDir::new("static"))
        .merge(routes::router())
        .route("/gateway", axum::routing::get(ws::gateway::handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()), env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("API listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}