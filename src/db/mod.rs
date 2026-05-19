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

    let schema = tokio::fs::read_to_string("schema.sql")
        .await
        .expect("Failed to read schema.sql");

    sqlx::query(&schema).execute(&pool).await?;

    Ok(pool)
}