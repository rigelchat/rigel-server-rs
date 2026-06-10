pub mod queries;

use tracing::{info};
use std::sync::Arc;
use tokio::sync::RwLock;
use std::collections::HashMap;
use sqlx::mysql::{MySqlPool, MySqlPoolOptions};
use std::env;
use crate::ws::session::WsSession;

#[derive(Clone)]
pub struct AppState { 
    pub sessions: Arc<RwLock<HashMap<String, WsSession>>>,
    pub db: MySqlPool
}

pub async fn init() -> Result<MySqlPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined for the database connection");

    let pool = MySqlPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    info!("Database connected successfully.");

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            created_at BIGINT UNSIGNED NOT NULL,
            bot BOOLEAN NOT NULL DEFAULT FALSE,
            public_flags INT UNSIGNED NOT NULL DEFAULT 0,
            username VARCHAR(32) NOT NULL,
            password_hash VARCHAR(255) DEFAULT NULL,
            global_name VARCHAR(32) DEFAULT NULL,
            discriminator VARCHAR(4) NOT NULL DEFAULT "0",
            avatar VARCHAR(255) DEFAULT NULL,
            banner VARCHAR(255) DEFAULT NULL
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS user_profiles (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            bio VARCHAR(190) DEFAULT "",
            pronouns VARCHAR(40) DEFAULT "",
            accent_color INT UNSIGNED DEFAULT NULL,
            theme_color_primary INT UNSIGNED DEFAULT NULL,
            theme_color_secondary INT UNSIGNED DEFAULT NULL
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS user_settings (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            status VARCHAR(10) NOT NULL DEFAULT "online",
            afk_timeout INT UNSIGNED NOT NULL DEFAULT 600,
            locale VARCHAR(5) NOT NULL DEFAULT "en-US",
            theme VARCHAR(10) NOT NULL DEFAULT "dark",
            background_gradient_preset VARCHAR(32) DEFAULT NULL,
            developer_mode BOOLEAN NOT NULL DEFAULT FALSE,
            FOREIGN KEY (id) REFERENCES users(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS user_sessions (
            id VARCHAR(6) NOT NULL PRIMARY KEY,
            created_at BIGINT UNSIGNED NOT NULL,
            last_used_at BIGINT UNSIGNED DEFAULT NULL,
            user_id VARCHAR(20) NOT NULL,
            os TEXT DEFAULT NULL,
            platform TEXT DEFAULT NULL,
            country_code VARCHAR(2) DEFAULT NULL,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS guilds (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            created_at BIGINT UNSIGNED NOT NULL,
            owner_id VARCHAR(20) NOT NULL,
            name VARCHAR(100) NOT NULL,
            icon VARCHAR(255) DEFAULT NULL,
            banner VARCHAR(255) DEFAULT NULL,
            description VARCHAR(300) DEFAULT NULL,
            rules_channel_id VARCHAR(20) DEFAULT NULL,
            vanity_url_code VARCHAR(20) DEFAULT NULL UNIQUE,
            afk_channel_id VARCHAR(20) DEFAULT NULL,
            afk_timeout INT UNSIGNED NOT NULL DEFAULT 300,
            system_channel_id VARCHAR(20) DEFAULT NULL,
            system_channel_flag INT NOT NULL DEFAULT 0,
            discoverable BOOLEAN NOT NULL DEFAULT FALSE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS guild_roles (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            guild_id VARCHAR(20) NOT NULL,
            position INT UNSIGNED NOT NULL,
            name VARCHAR(100) NOT NULL,
            color INT UNSIGNED NOT NULL DEFAULT 0,
            unicode_emoji VARCHAR(4) DEFAULT NULL,
            hoist BOOLEAN NOT NULL DEFAULT FALSE,
            mentionable BOOLEAN NOT NULL DEFAULT FALSE,
            permissions BIGINT UNSIGNED NOT NULL DEFAULT 0,
            FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS channels (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            type INT UNSIGNED NOT NULL,
            position INT UNSIGNED NOT NULL,
            guild_id VARCHAR(20) NOT NULL,
            parent_id VARCHAR(20) DEFAULT NULL,
            name TEXT NOT NULL,
            FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS channel_permission_overwrites (
            type INT UNSIGNED NOT NULL,
            channel_id VARCHAR(20) NOT NULL,
            target_id VARCHAR(20) NOT NULL,
            allow BIGINT UNSIGNED NOT NULL DEFAULT 0,
            deny BIGINT UNSIGNED NOT NULL DEFAULT 0,
            PRIMARY KEY (channel_id, target_id),
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS guild_members (
            joined_at BIGINT UNSIGNED NOT NULL,
            guild_id VARCHAR(20) NOT NULL,
            user_id VARCHAR(20) NOT NULL,
            PRIMARY KEY (guild_id, user_id),
            FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS guild_member_roles (
            guild_id VARCHAR(20) NOT NULL,
            user_id VARCHAR(20) NOT NULL,
            role_id VARCHAR(20) NOT NULL,
            PRIMARY KEY (guild_id, user_id, role_id),
            FOREIGN KEY (guild_id, user_id) REFERENCES guild_members(guild_id, user_id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES guild_roles(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS guild_bans (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            banned_at BIGINT UNSIGNED NOT NULL,
            guild_id VARCHAR(20) NOT NULL,
            user_id VARCHAR(20) NOT NULL,
            reason TEXT DEFAULT NULL,
            FOREIGN KEY (guild_id) REFERENCES guilds(id) ON DELETE CASCADE,
            UNIQUE (guild_id, user_id)
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS messages (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            timestamp BIGINT UNSIGNED NOT NULL,
            edited_timestamp BIGINT UNSIGNED DEFAULT NULL,
            channel_id VARCHAR(20) NOT NULL,
            author_id VARCHAR(20) NOT NULL,
            type INT UNSIGNED NOT NULL DEFAULT 0,
            flags INT UNSIGNED NOT NULL DEFAULT 0,
            content TEXT DEFAULT NULL,
            FOREIGN KEY (channel_id) REFERENCES channels(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    sqlx::query(r#"
        CREATE TABLE IF NOT EXISTS gateway_sessions (
            id VARCHAR(36) NOT NULL PRIMARY KEY,
            created_at BIGINT UNSIGNED NOT NULL,
            expires_at BIGINT UNSIGNED NOT NULL,
            user_id VARCHAR(20) DEFAULT NULL,
            user_session_id VARCHAR(6) DEFAULT NULL,
            encoding VARCHAR(4) NOT NULL,
            compression VARCHAR(6),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (user_session_id) REFERENCES user_sessions(id) ON DELETE CASCADE
        )
    "#).execute(&pool).await?;

    return Ok(pool);
}