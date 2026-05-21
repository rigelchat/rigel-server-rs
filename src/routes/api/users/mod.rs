pub mod me;

use axum::{
    extract::{Path, State},
    http::{StatusCode},
    routing::get,
    Json, Router,
};
use sqlx::FromRow; 
use crate::db::AppState;
use serde::Serialize;

pub fn router() -> Router<AppState> {
    return Router::new()
        .nest("/@me", me::router())
        .route("/{id}/profile", get(get_user_profile));
}

// GET /api/users/{id}/profile
async fn get_user_profile(
    State(state): State<AppState>,
    Path(user_id): Path<String>,
) -> Result<Json<UserProfileRes>, (StatusCode, String)> {
    let profile_result = sqlx::query_as::<_, UserProfileRow>("SELECT * FROM user_profiles WHERE id = ?")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match profile_result {
        Some(profile) => Ok(Json(profile.into())),
        None => Err((StatusCode::NOT_FOUND, "User profile not found".to_string())),
    }
}

#[derive(FromRow)]
struct UserProfileRow {
    bio: Option<String>,
    pronouns: Option<String>,
    accent_color: Option<u64>,
    theme_color_primary: Option<u64>,
    theme_color_secondary: Option<u64>,
}

#[derive(Serialize)]
pub struct UserProfileData {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub accent_color: Option<u64>,
    pub theme_colors: Option<[u64; 2]>,
}

#[derive(Serialize)]
pub struct UserProfileRes {
    pub user_profile: UserProfileData,
}

impl From<UserProfileRow> for UserProfileRes {
    fn from(row: UserProfileRow) -> Self {
        let theme_colors = match (
            row.theme_color_primary,
            row.theme_color_secondary,
        ) {
            (Some(primary), Some(secondary)) => Some([primary, secondary]),
            _ => None,
        };

        Self {
            user_profile: UserProfileData {
                bio: row.bio,
                pronouns: row.pronouns,
                accent_color: row.accent_color,
                theme_colors,
            },
        }
    }
}