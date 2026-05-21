use axum::{Router, routing::post, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use::tracing::error;
use crate::AppState;
use crate::db::models::User;
use crate::db::queries::get_user_by_login;
use crate::utils::{snowflake::DISCORD_SNOWFLAKE, token};

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/login", post(login))
        .route("/register", post(register));
}

// POST /api/auth/login
async fn login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let password = payload.password.as_deref().unwrap_or("");

    let user = get_user_by_login(&state, &payload.login)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let password_hash_str = user.password_hash.as_ref().ok_or(StatusCode::UNAUTHORIZED)?;
    if !verify(password, password_hash_str).unwrap_or(false) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT IGNORE INTO user_settings (id) VALUES (?)")
        .bind(&user.id)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    sqlx::query("INSERT IGNORE INTO user_profiles (id) VALUES (?)")
        .bind(&user.id)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
    let signed = token::sign(&user.id, &secret);

    sqlx::query("INSERT INTO user_sessions (id, user_id, created_at) VALUES (?, ?, ?)")
        .bind(&signed.timestamp64)
        .bind(&user.id)
        .bind(signed.timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(LoginResponse { token: signed.token }));
}

// POST /api/auth/register
async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let raw_password = payload.password.unwrap_or_else(|| "".to_string());
    let hashed_password = hash(raw_password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let count: (u64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE bot = 0")
        .fetch_one(&mut *tx)
        .await
        .unwrap_or((1,));

    let is_first_user = count.0 == 0;
    let public_flags: u32 = if is_first_user { 1 } else { 0 };

    let new_user = User {
        id: DISCORD_SNOWFLAKE.generate(None).to_string(),
        created_at: Utc::now().timestamp_millis() as u64,
        bot: false,
        public_flags,
        username: payload.username.clone(),
        password_hash: Some(hashed_password),
        global_name: None,
        discriminator: Some("0".to_string()),
        avatar: None,
        banner: None
    };

    sqlx::query("
        INSERT INTO users (id, created_at, bot, public_flags, username, password_hash, global_name, discriminator, avatar, banner) 
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    ")
        .bind(&new_user.id)
        .bind(new_user.created_at)
        .bind(new_user.bot)
        .bind(new_user.public_flags)
        .bind(&new_user.username)
        .bind(&new_user.password_hash)
        .bind(&new_user.global_name)
        .bind(&new_user.discriminator)
        .bind(&new_user.avatar)
        .bind(&new_user.banner)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    sqlx::query("INSERT IGNORE INTO user_settings (id) VALUES (?)")
        .bind(&new_user.id)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    sqlx::query("INSERT IGNORE INTO user_profiles (id) VALUES (?)")
        .bind(&new_user.id)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
    let signed = token::sign(&new_user.id, &secret);

    sqlx::query("INSERT INTO user_sessions (id, user_id, created_at) VALUES (?, ?, ?)" )
        .bind(&signed.timestamp64)
        .bind(&new_user.id)
        .bind(signed.timestamp)
        .execute(&mut *tx)
        .await
        .map_err(|err| {
            error!("{}", err);
            return StatusCode::INTERNAL_SERVER_ERROR;
        })?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    return Ok(Json(LoginResponse { token: signed.token }));
}

#[derive(Serialize)]
struct LoginResponse {
    token: String
}

#[derive(Deserialize)]
struct LoginRequest {
    login: String,
    password: Option<String>,
}

#[derive(Deserialize)]
struct RegisterRequest {
    username: String,
    password: Option<String>,
}