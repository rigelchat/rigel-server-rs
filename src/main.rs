use axum::{Router, http::Method};
use tower_http::cors::{Any, CorsLayer};
use dotenvy::dotenv;
use tracing::{info, error};
use tracing_subscriber::EnvFilter;
use std::env;

mod routes;
mod db;
mod utils;
mod ws;

pub use db::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    // Initialisation de la BDD
    let db_pool = match db::init_db().await {
        Ok(pool) => {
            info!("Database connected successfully.");
            pool
        }
        Err(e) => {
            error!("Failed to connect to the database: {:?}", e);
            std::process::exit(1);
        }
    };

    let state = AppState { db: db_pool };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::PATCH])
        .allow_headers(Any);

    let api_routes = routes::api_router();

    let app = Router::new()
        .nest("/.well-known", routes::well_known::router())
        .nest("/api", api_routes.clone())
        .nest("/api/v0", api_routes)
        .route("/gateway", axum::routing::get(ws::gateway::handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("{}:{}", env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()), env::var("PORT").unwrap_or_else(|_| "3000".to_string()));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    info!("API listening at http://{}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}