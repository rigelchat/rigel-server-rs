use std::env;
use axum::{Router, routing::get, Json};
use serde::Serialize;
use crate::AppState;

#[derive(Serialize)]
struct Instance {
    name: String,
    description: String,
    image: String,
    #[serde(rename = "correspondenceEmail")]
    correspondence_email: String,
    #[serde(rename = "frontPage")]
    front_page: String,
    #[serde(rename = "tosPage")]
    tos_page: String
}

#[derive(Serialize)]
struct PingResponse {
    ping: String,
    instance: Instance
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_ping))
}

async fn get_ping() -> Json<PingResponse> {
    Json(PingResponse {
        ping: "pong!".to_string(),
        instance: Instance {
            name: env::var("INSTANCE_NAME").unwrap_or_default(),
            description: env::var("INSTANCE_DESCRIPTION").unwrap_or_default(),
            image: env::var("INSTANCE_IMAGE").unwrap_or_default(),
            correspondence_email: env::var("INSTANCE_CORRESPONDENCE_EMAIL").unwrap_or_default(),
            front_page: env::var("INSTANCE_FRONT_PAGE").unwrap_or_default(),
            tos_page: env::var("INSTANCE_TOS_PAGE").unwrap_or_default()
        }
    })
}