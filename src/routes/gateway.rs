use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
struct GatewayResponse {
    url: String
}

#[derive(Serialize)]
struct SessionStartLimit {
    remaining: u8,
    total: u8,
    max_concurrency: u8,
    reset_after: u64,
}

#[derive(Serialize)]
struct GatewayBotResponse {
    url: String,
    shards: u8,
    session_start_limit: SessionStartLimit,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_gateway))
        .route("/bot", get(get_bot_gateway))
}

async fn get_gateway() -> Json<GatewayResponse> {
    Json(GatewayResponse {
        url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default())
    })
}

async fn get_bot_gateway() -> Json<GatewayBotResponse> {
    Json(GatewayBotResponse {
        url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default()),
        shards: 1,
        session_start_limit: SessionStartLimit {
            remaining: 0,
            total: 1,
            max_concurrency: 1,
            reset_after: 14400000,
        },
    })
}