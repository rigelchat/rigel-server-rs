use serde::Serialize;
use sqlx::prelude::FromRow;

#[derive(Serialize, FromRow)]
pub struct GuildChannel {
    pub id: String,
    #[serde(rename="type")]
    #[sqlx(rename = "type")]
    pub kind: u32,
    pub position: u32,
    pub guild_id: String,
    pub parent_id: Option<String>,
    pub name: String
}