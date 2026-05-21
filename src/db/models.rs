use sqlx::FromRow;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: String,
    pub created_at: i64,
    pub bot: i32,
    pub public_flags: i32,
    pub username: String,
    pub password_hash: Option<String>,
    pub global_name: Option<String>,
    pub discriminator: Option<String>,
    pub avatar: Option<String>,
    pub banner: Option<String>
}

// #[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
// pub struct GatewayUser {
//     pub id: String,
//     pub created_at: i64,
//     pub bot: i32,
//     pub public_flags: i32,
//     pub username: String,
//     pub global_name: Option<String>,
//     pub discriminator: Option<String>,
//     pub avatar: Option<String>,
//     pub banner: Option<String>
// }