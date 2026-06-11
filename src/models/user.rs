use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
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

#[derive(Serialize, FromRow)]
pub struct PublicUser {
    pub id: String,
    pub created_at: u64,
    pub bot: bool,
    pub public_flags: u32,
    pub username: String,
    pub global_name: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>
}

#[derive(Serialize)]
pub struct MessageAuthor {
    pub id: String,
    pub username: String,
    pub global_name: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>,
    pub public_flags: u32
}