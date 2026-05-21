use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(gateway))
        .route("/bot", get(bot_gateway));
}

// GET /api/gateway
async fn gateway() -> Json<GatewayResponse> {
    return Json(GatewayResponse {
        url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default())
    });
}

// GET /api/gateway/bot
async fn bot_gateway() -> Json<GatewayBotResponse> {
    return Json(GatewayBotResponse {
        url: format!("{}/gateway", env::var("PUBLIC_WS_URL").unwrap_or_default()),
        shards: 1,
        session_start_limit: SessionStartLimit {
            remaining: 0,
            total: 1,
            max_concurrency: 1,
            reset_after: 14400000
        },
    });
}

#[derive(Serialize)]
struct GatewayResponse {
    url: String
}

#[derive(Serialize)]
struct SessionStartLimit {
    remaining: u8,
    total: u8,
    max_concurrency: u8,
    reset_after: u64
}

#[derive(Serialize)]
struct GatewayBotResponse {
    url: String,
    shards: u8,
    session_start_limit: SessionStartLimit
}