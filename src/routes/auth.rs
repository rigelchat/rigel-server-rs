use axum::{Router, routing::post, Json, extract::State, http::StatusCode};
use serde::{Deserialize, Serialize};
use crate::AppState;
use crate::db::models::User;
use crate::db::queries::{get_user_by_login, insert_user};
use crate::utils::{snowflake, token};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;

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

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/login", post(post_login))
        .route("/register", post(post_register))
}

async fn post_login(State(state): State<AppState>, Json(payload): Json<LoginRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let password = payload.password.as_deref().unwrap_or("");

    if let Ok(Some(user)) = get_user_by_login(&state, &payload.login).await {
        if let Some(hash_str) = &user.password_hash {
            if verify(password, hash_str).unwrap_or(false) {
                let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
                let auth_token = token::sign_token(&user.id, &secret);
                return Ok(Json(LoginResponse { token: auth_token }));
            }
        }
    }

    Err(StatusCode::UNAUTHORIZED)
}

async fn post_register(State(state): State<AppState>, Json(payload): Json<RegisterRequest>) -> Result<Json<LoginResponse>, StatusCode> {
    let raw_password = payload.password.unwrap_or_else(|| "".to_string());
    
    // Hasher le mot de passe
    let hashed = hash(raw_password, DEFAULT_COST).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    let new_user = User {
        id: snowflake::generate(),
        created_at: Utc::now().timestamp_millis(),
        bot: 0,
        public_flags: 0,
        username: payload.username,
        password_hash: Some(hashed),
        global_name: None,
        discriminator: Some("0".to_string()),
        avatar: None,
        banner: None,
    };
    
    // Insérer dans la BDD
    insert_user(&state, &new_user).await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    // Générer le token et retourner
    let secret = std::env::var("AUTH_SECRET").unwrap_or_default();
    let auth_token = token::sign_token(&new_user.id, &secret);
    Ok(Json(LoginResponse { token: auth_token }))
}