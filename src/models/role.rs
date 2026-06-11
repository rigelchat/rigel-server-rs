use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Clone, Serialize, FromRow)]
pub struct Role {
    pub id: String,
    pub guild_id: String,
    pub position: u32,
    pub name: String,
    pub color: u32,
    pub unicode_emoji: Option<String>,
    pub hoist: bool,
    pub mentionable: bool,
    pub permissions: u64
}