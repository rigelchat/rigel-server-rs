use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(get_instance_info));
}

// GET /api/ping
async fn get_instance_info() -> Json<PingResponse> {
    return Json(PingResponse {
        ping: "pong!".to_string(),
        instance: Instance {
            name: env::var("INSTANCE_NAME").unwrap_or_default(),
            description: env::var("INSTANCE_DESCRIPTION").unwrap_or_default(),
            image: env::var("INSTANCE_IMAGE").unwrap_or_default(),
            correspondence_email: env::var("INSTANCE_CORRESPONDENCE_EMAIL").unwrap_or_default(),
            front_page: env::var("INSTANCE_FRONT_PAGE").unwrap_or_default(),
            tos_page: env::var("INSTANCE_TOS_PAGE").unwrap_or_default()
        }
    });
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct Instance {
    name: String,
    description: String,
    image: String,
    correspondence_email: String,
    front_page: String,
    tos_page: String
}

#[derive(Serialize)]
struct PingResponse {
    ping: String,
    instance: Instance
}