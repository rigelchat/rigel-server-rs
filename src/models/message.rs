use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::models::user::MessageAuthor;

#[derive(Serialize, FromRow)]
pub struct Message {
    pub id: String,
    pub timestamp: u64,
    pub edited_timestamp: Option<u64>,
    pub channel_id: String,
    pub author_id: String,
    #[serde(rename="type")]
    #[sqlx(rename = "type")]
    pub kind: u32,
    pub flags: u32,
    pub content: Option<String>,
    pub author: MessageAuthor
}