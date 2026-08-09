use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    routing::patch,
    Json,
    Router
};
use serde::{Deserialize, Deserializer, Serialize};
use sqlx::{MySql, FromRow, QueryBuilder}; 
use tracing::error;

use crate::AppState;
use crate::models::user::PublicUser;
use crate::services::get_public_user;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", patch(update_user))
        .route("/profile", patch(update_profile))
        .route("/settings", patch(update_settings));
}

// PATCH /api/users/@me
async fn update_user(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateUserRequest>
) -> Result<Json<PublicUser>, (StatusCode, String)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap();
    let user_id = crate::utils::token::verify(auth_header, &secret).map_err(|err| (StatusCode::UNAUTHORIZED, err.to_string()))?;

    let mut query = QueryBuilder::new("UPDATE users SET ");
    let mut has_updates = false;

    if let Some(global_name) = &payload.global_name {
        query.push("global_name = ");

        if let Some(global_name) = global_name {
            let global_name = global_name.trim();

            if global_name.is_empty() || global_name.len() > 32 {
                return Err((StatusCode::BAD_REQUEST, "global_name must be between 1 and 32 characters".to_string())); // todo: regrouper toutes les erreurs (ajouter un help/util pour generé les erreurs json)
            };

            query.push_bind(global_name);
        } else {
            query.push_bind(Option::<String>::None);
        };

        has_updates = true;
    };

    if let Some(avatar) = &payload.avatar {
        query.push("avatar = ");

        if let Some(avatar) = avatar {

        } else {
            query.push_bind(Option::<String>::None);
        };

        has_updates = true;
    };

    if let Some(banner) = &payload.banner {
        query.push("banner = ");

        if let Some(banner) = banner {

        } else {
            query.push_bind(Option::<String>::None);
        };

        has_updates = true;
    };

    if !has_updates {
        return Err((StatusCode::BAD_REQUEST, "rien a modifier".to_string()));
    };

    query.push(" WHERE id = ");
    query.push_bind(&user_id);

    query
        .build()
        .execute(&state.db)
        .await
        .map_err(|err| {
            error!(error = %err, user_id = %user_id, "Failed to update user");
            return (StatusCode::INTERNAL_SERVER_ERROR, "".to_string());
        })?;

    let new_user = get_public_user(&state, &user_id)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let _ = state.dispatch_all("USER_UPDATE", &new_user).await;

    return Ok(Json(new_user));
}

// PATCH /api/users/@me/profile
async fn update_profile(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = crate::utils::token::verify(auth_header, &secret).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

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

    let new_profile: UserProfileRow = sqlx::query_as("SELECT * FROM user_profiles WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|evt| (StatusCode::INTERNAL_SERVER_ERROR, evt.to_string()))?;

    Ok(Json(new_profile.into()))
}

// PATCH /api/users/@me/settings
async fn update_settings(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateSettingsRequest>,
) -> Result<Json<UserSettings>, (StatusCode, String)> {
    let auth_header = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = crate::utils::token::verify(auth_header, &secret).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))?;

    let mut query = QueryBuilder::<MySql>::new("UPDATE user_settings SET ");
    let mut separated = query.separated(", ");
    let mut has_updates = false;

    if let Some(status) = &payload.status {
        separated.push("status = ");
        separated.push_bind_unseparated(status);
        has_updates = true;
    };

    if let Some(locale) = &payload.locale {
        separated.push("locale = ");
        separated.push_bind_unseparated(locale);
        has_updates = true;
    };

    if let Some(theme) = &payload.theme {
        separated.push("theme = ");
        separated.push_bind_unseparated(theme);
        has_updates = true;
    };

    if let Some(bg) = &payload.background_gradient_preset {
        separated.push("background_gradient_preset = ");
        separated.push_bind_unseparated(bg.as_deref());
    };

    if let Some(dev_mode) = payload.developer_mode {
        separated.push("developer_mode = ");
        separated.push_bind_unseparated(dev_mode); 
        has_updates = true;
    };

    if !has_updates {
        return Err((StatusCode::BAD_REQUEST, "rien a modifier".to_string()));
    };

    query.push(" WHERE id = ");
    query.push_bind(user_id.clone());

    query.build()
        .execute(&state.db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let new_settings = sqlx::query_as::<_, UserSettings>("SELECT * FROM user_settings WHERE id = ?")
        .bind(&user_id)
        .fetch_one(&state.db)
        .await
        .map_err(|err| (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()))?;

    let _ = state.dispatch_user("USER_SETTINGS_UPDATE", &new_settings, &user_id).await;

    return Ok(Json(new_settings));
}

#[derive(Deserialize)]
struct UpdateUserRequest {
    #[serde(deserialize_with = "deserialize_double_option")]
    global_name: Option<Option<String>>,
    #[serde(deserialize_with = "deserialize_double_option")]
    avatar: Option<Option<String>>,
    #[serde(deserialize_with = "deserialize_double_option")]
    banner: Option<Option<String>>
}

#[derive(Deserialize)]
pub struct UpdateProfileRequest {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub accent_color: Option<u64>,
    pub theme_colors: Option<Vec<Option<u64>>>
}

#[derive(FromRow)]
struct UserProfileRow {
    bio: Option<String>,
    pronouns: Option<String>,
    accent_color: Option<u64>,
    theme_color_primary: Option<u64>,
    theme_color_secondary: Option<u64>
}

#[derive(Serialize)]
pub struct UserProfileResponse {
    pub user_profile: UserProfileData
}

#[derive(Serialize)]
pub struct UserProfileData {
    pub bio: Option<String>,
    pub pronouns: Option<String>,
    pub accent_color: Option<u64>,
    pub theme_colors: Option<[u64; 2]>
}

impl From<UserProfileRow> for UserProfileResponse {
    fn from(row: UserProfileRow) -> Self {
        let theme_colors = match (row.theme_color_primary, row.theme_color_secondary) {
            (Some(p), Some(s)) => Some([p, s]),
            _ => None
        };
        Self {
            user_profile: UserProfileData {
                bio: row.bio,
                pronouns: row.pronouns,
                accent_color: row.accent_color,
                theme_colors
            },
        }
    }
}

#[derive(Deserialize)]
pub struct UpdateSettingsRequest {
    pub status: Option<String>,
    pub locale: Option<String>,
    pub theme: Option<String>,
    #[serde(deserialize_with = "deserialize_double_option")]
    pub background_gradient_preset: Option<Option<String>>,
    pub developer_mode: Option<bool>
}

#[derive(Serialize, FromRow)]
pub struct UserSettings {
    pub id: String, 
    pub status: Option<String>,
    pub locale: Option<String>,
    pub theme: Option<String>,
    pub background_gradient_preset: Option<String>,
    pub developer_mode: bool
}

fn deserialize_double_option<'de, T, D>(deserializer: D) -> Result<Option<Option<T>>, D::Error>
where
    T: Deserialize<'de>,
    D: Deserializer<'de>
{
    return Option::<T>::deserialize(deserializer).map(Some);
}