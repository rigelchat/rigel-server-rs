use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::patch,
    Json,
    Router
};
use serde::{Deserialize, Serialize};
use sqlx::{MySql, FromRow, QueryBuilder}; 
use crate::db::AppState;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/profile", patch(update_profile))
        .route("/settings", patch(update_settings));
}

// PATCH /api/users/@me/settings
async fn update_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSettingsReq>,
) -> Result<Json<UserSettings>, (StatusCode, String)> {
    
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = crate::utils::token::verify_token(auth_header, &secret).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let mut query = QueryBuilder::<MySql>::new("UPDATE user_settings SET ");
    let mut separated = query.separated(", ");
    let mut has_updates = false;

    if let Some(status) = &payload.status {
        separated.push("status = ");
        separated.push_bind_unseparated(status);
        has_updates = true;
    }
    if let Some(locale) = &payload.locale {
        separated.push("locale = ");
        separated.push_bind_unseparated(locale);
        has_updates = true;
    }
    if let Some(theme) = &payload.theme {
        separated.push("theme = ");
        separated.push_bind_unseparated(theme);
        has_updates = true;
    }
    if let Some(bg) = &payload.background_gradient_preset {
        separated.push("background_gradient_preset = ");
        separated.push_bind_unseparated(bg);
        has_updates = true;
    }
    if let Some(dev_mode) = payload.developer_mode {
        separated.push("developer_mode = ");
        separated.push_bind_unseparated(dev_mode); 
        has_updates = true;
    }

    if has_updates {
        query.push(" WHERE id = ");
        query.push_bind(user_id.clone());

        query.build()
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let settings: UserSettings = sqlx::query_as("SELECT * FROM user_settings WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(settings))
}

// PATCH /api/users/@me/profile
async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileReq>,
) -> Result<Json<UserProfileRes>, (StatusCode, String)> {
    
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = crate::utils::token::verify_token(auth_header, &secret).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let mut query = QueryBuilder::<MySql>::new("UPDATE user_profiles SET ");
    let mut separated = query.separated(", ");
    let mut has_updates = false;

    if let Some(bio) = &payload.bio {
        separated.push("bio = ");
        separated.push_bind_unseparated(bio);
        has_updates = true;
    }
    if let Some(pronouns) = &payload.pronouns {
        separated.push("pronouns = ");
        separated.push_bind_unseparated(pronouns);
        has_updates = true;
    }
    if let Some(accent_color) = &payload.accent_color {
        separated.push("accent_color = ");
        separated.push_bind_unseparated(accent_color);
        has_updates = true;
    }
    if let Some(theme_colors) = &payload.theme_colors {
        let primary = theme_colors.get(0).cloned().flatten();
        let secondary = theme_colors.get(1).cloned().flatten();

        separated.push("theme_color_primary = ");
        separated.push_bind_unseparated(primary);
        
        separated.push("theme_color_secondary = ");
        separated.push_bind_unseparated(secondary);
        has_updates = true;
    }

    if has_updates {
        query.push(" WHERE id = ");
        query.push_bind(user_id.clone());

        query.build()
            .execute(&state.db)
            .await
            .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;
    }

    let profile: UserProfileRow = sqlx::query_as("SELECT * FROM user_profiles WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok(Json(profile.into()))
}

#[derive(Deserialize)]
pub struct UpdateSettingsReq {
    pub status: Option<String>,
    pub locale: Option<String>,
    pub theme: Option<String>,
    pub background_gradient_preset: Option<String>,
    pub developer_mode: Option<bool>,
}

#[derive(Serialize, FromRow)]
pub struct UserSettings {
    pub id: String, 
    pub status: Option<String>,
    pub locale: Option<String>,
    pub theme: Option<String>,
    pub background_gradient_preset: Option<String>,
    #[serde(serialize_with = "serialize_as_bool")]
    pub developer_mode: i64, 
}

fn serialize_as_bool<S>(value: &i64, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_bool(*value != 0)
}

#[derive(Deserialize)]
pub struct UpdateProfileReq {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub accent_color: Option<i64>,
    pub theme_colors: Option<Vec<Option<i64>>>,
}

#[derive(FromRow)]
struct UserProfileRow {
    bio: Option<String>,
    pronouns: Option<String>,
    accent_color: Option<i64>,
    theme_color_primary: Option<i64>,
    theme_color_secondary: Option<i64>,
}

#[derive(Serialize)]
pub struct UserProfileRes {
    pub user_profile: UserProfileData,
}

#[derive(Serialize)]
pub struct UserProfileData {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub accent_color: Option<i64>,
    pub theme_colors: Option<[i64; 2]>,
}

impl From<UserProfileRow> for UserProfileRes {
    fn from(row: UserProfileRow) -> Self {
        let theme_colors = match (row.theme_color_primary, row.theme_color_secondary) {
            (Some(p), Some(s)) => Some([p, s]),
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