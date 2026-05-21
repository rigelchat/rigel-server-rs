use axum::{Router, routing::post, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
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

    if let Ok(Some(user)) = get_user_by_login(&state, &payload.login).await {
        if let Some(hash_str) = &user.password_hash {
            if verify(password, hash_str).unwrap_or(false) {
                let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                // Insert into user_settings and user_profiles just in case
                sqlx::query("INSERT OR IGNORE INTO user_settings (id) VALUES (?)")
                    .bind(&user.id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                sqlx::query("INSERT OR IGNORE INTO user_profiles (id) VALUES (?)")
                    .bind(&user.id)
                    .execute(&mut *tx)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
                let token_data = token::sign_token(&user.id, &secret);

                // Insert into user_sessions
                sqlx::query("INSERT INTO user_sessions (id, user_id, created_at) VALUES (?, ?, ?)")
                    .bind(&token_data.timestamp64)
                    .bind(&user.id)
                    .bind(token_data.timestamp as i64)
                    .execute(&mut *tx)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                return Ok(Json(LoginResponse { token: token_data.token }));
            };
        };
    };

    return Err(StatusCode::UNAUTHORIZED);
}

// POST /api/auth/register
async fn register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let raw_password = payload.password.unwrap_or_else(|| "".to_string());
    
    // Hasher le mot de passe
    let hashed = hash(raw_password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let mut tx = state.db.begin().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users WHERE bot = 0")
        .fetch_one(&mut *tx)
        .await
        .unwrap_or((1,));
    let is_first_user = count.0 == 0;

    let public_flags: i32 = if is_first_user { 1 } else { 0 };

    let new_user = User {
        id: DISCORD_SNOWFLAKE.generate(None).to_string(),
        created_at: Utc::now().timestamp_millis(),
        bot: 0,
        public_flags,
        username: payload.username.clone(),
        password_hash: Some(hashed),
        global_name: None,
        discriminator: Some("0".to_string()),
        avatar: None,
        banner: None,
    };
    
    // Insérer dans la BDD
    sqlx::query(
        "INSERT INTO users (id, created_at, bot, public_flags, username, password_hash, global_name, discriminator, avatar, banner) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
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
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT OR IGNORE INTO user_settings (id) VALUES (?)")
        .bind(&new_user.id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    sqlx::query("INSERT OR IGNORE INTO user_profiles (id) VALUES (?)")
        .bind(&new_user.id)
        .execute(&mut *tx)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Générer le token et retourner
    let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
    let token_data = token::sign_token(&new_user.id, &secret);
    
    sqlx::query(
        r#"
        INSERT INTO user_sessions (id, user_id, created_at)
        VALUES (?, ?, ?)
        "#
    )
    .bind(&token_data.timestamp64)
    .bind(&new_user.id)
    .bind(token_data.timestamp as i64)
    .execute(&mut *tx)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    tx.commit().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse { token: token_data.token }))
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