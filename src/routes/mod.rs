pub mod api;
pub mod well_known;

use axum::{Router, routing::get, response::Html};

use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(index))
        .nest("/api", api::router())
        .nest("/api/v0", api::router())
        .nest("/.well-known", well_known::router());
}

async fn index() -> Html<&'static str> {
    return Html(include_str!("../../static/index.html"));
}