use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::models::role::Role;

#[derive(Serialize, FromRow)]
pub struct BaseGuildMember {
    pub joined_at: u64,
    pub guild_id: String,
    pub user_id: String
}

#[derive(Serialize, FromRow)]
pub struct GuildMember {
    #[serde(flatten)]
    pub base: BaseGuildMember,
    pub roles: Vec<Role>
}

#[derive(FromRow)]
pub struct GuildMemberRole {
    pub user_id: String,
    pub role_id: String
}