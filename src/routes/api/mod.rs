pub mod auth;
pub mod channels;
pub mod discoverable_guilds;
pub mod gateway;
pub mod guilds;
pub mod ping;
pub mod users;

use axum::Router;
use crate::db::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .nest("/auth", auth::router())
        .nest("/channels", channels::router())
        .nest("/discoverable-guilds", discoverable_guilds::router())
        .nest("/gateway", gateway::router())
        .nest("/guilds", guilds::router())
        .nest("/ping", ping::router())
        .nest("/users", users::router());
}