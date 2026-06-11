pub mod api;
pub mod well_known;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .nest("/api", api::router())
        .nest("/api/v0", api::router())
        .nest("/.well-known", well_known::router());
}