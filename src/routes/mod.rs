pub mod auth;
pub mod channels;
pub mod gateway;
pub mod guilds;
pub mod messages;
pub mod ping;
pub mod users;
pub mod well_known;

use axum::Router;
use crate::db::AppState;

pub fn api_router() -> Router<AppState> {
    return Router::new()
        .nest("/auth", auth::router())
        .nest("/gateway", gateway::router())
        .nest("/ping", ping::router());
}