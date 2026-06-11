pub mod messages;

use axum::Router;

use crate::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .nest("/{id}/messages", messages::router());
}