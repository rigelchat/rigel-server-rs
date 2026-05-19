pub mod models;
pub mod queries;

use sqlx::any::AnyPoolOptions;
use sqlx::AnyPool;
use std::env;

#[derive(Clone)]
pub struct AppState {
    pub db: AnyPool,
}

pub async fn init_db() -> Result<AnyPool, sqlx::Error> {
    sqlx::any::install_default_drivers();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be defined for the database connection");

    let pool = AnyPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    let schema = r#"
        CREATE TABLE IF NOT EXISTS users (
            id VARCHAR(20) NOT NULL PRIMARY KEY,
            created_at BIGINT UNSIGNED NOT NULL,
            bot TINYINT NOT NULL DEFAULT 0,
            public_flags INT UNSIGNED NOT NULL DEFAULT 0,
            username VARCHAR(32) NOT NULL,
            password_hash VARCHAR(255) DEFAULT NULL,
            global_name VARCHAR(32) DEFAULT NULL,
            discriminator VARCHAR(4) NOT NULL DEFAULT "0",
            avatar VARCHAR(255) DEFAULT NULL,
            banner VARCHAR(255) DEFAULT NULL
        );
    "#;

    sqlx::query(schema).execute(&pool).await?;

    Ok(pool)
}