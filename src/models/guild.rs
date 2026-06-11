use serde::Serialize;
use sqlx::prelude::FromRow;

use crate::models::{channel::GuildChannel, guild_member::GuildMember, role::Role};

#[derive(Clone, Serialize, FromRow)]
pub struct BaseGuild {
    pub id: String,
    pub created_at: u64,
    pub owner_id: String,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub rules_channel_id: Option<String>,
    pub vanity_url_code: Option<String>,
    pub afk_channel_id: Option<String>,
    pub afk_timeout: u32,
    pub system_channel_id: Option<String>,
    pub system_channel_flags: u32,
    pub discoverable: bool
}

#[derive(Serialize)]
pub struct GatewayGuild {
    pub properties: BaseGuild,
    pub channels: Vec<GuildChannel>,
    pub roles: Vec<Role>,
    pub members: Vec<GuildMember>
}

#[derive(Serialize)]
pub struct DiscoverableGuild {
    pub id: String,
    pub name: String,
    pub icon: Option<String>,
    pub banner: Option<String>,
    pub description: Option<String>,
    pub vanity_url_code: Option<String>,
    pub approximate_member_count: i64,
    pub approximate_presence_count: i64
}