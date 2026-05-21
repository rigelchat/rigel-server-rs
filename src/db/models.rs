use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub created_at: u64,
    pub bot: bool,
    pub public_flags: u32,
    pub username: String,
    pub password_hash: Option<String>,
    pub global_name: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>
}