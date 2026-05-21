pub mod messages;

use axum::Router;
use crate::db::AppState;

pub fn router() -> Router<AppState> {
    return Router::new();
}