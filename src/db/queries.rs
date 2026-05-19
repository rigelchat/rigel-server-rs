use crate::db::models::{User, GatewayUser};
use crate::AppState;

pub async fn get_user_by_login(state: &AppState, login: &str) -> Result<Option<User>, sqlx::Error> {
    return sqlx::query_as::<_, User>("SELECT * FROM users WHERE username = ?")
        .bind(login)
        .fetch_optional(&state.db)
        .await;
}

pub async fn insert_user(state: &AppState, user: &User) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO users (id, created_at, bot, public_flags, username, password_hash, global_name, discriminator, avatar, banner) 
         VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
    )
        .bind(&user.id)
        .bind(user.created_at)
        .bind(user.bot)
        .bind(user.public_flags)
        .bind(&user.username)
        .bind(&user.password_hash)
        .bind(&user.global_name)
        .bind(&user.discriminator)
        .bind(&user.avatar)
        .bind(&user.banner)
        .execute(&state.db)
        .await?;

    return Ok(());
}

pub async fn get_user_by_id(state: &AppState, user_id: &str) -> Result<Option<GatewayUser>, sqlx::Error> {
    return sqlx::query_as::<_, GatewayUser>("
        SELECT id, created_at, bot, public_flags, username, global_name, discriminator, avatar, banner
        FROM users
        WHERE id = ?
    ")
        .bind(user_id)
        .fetch_optional(&state.db)
        .await;
}