use chrono::Utc;

use crate::AppState;
use crate::models::{
    guild::{BaseGuild, GatewayGuild},
    user::PublicUser
};
use crate::utils::constants::{
    flags::PermissionFlags,
    models::{ChannelType, OverwriteType}
};
use crate::utils::snowflake::DISCORD_SNOWFLAKE;

pub async fn create_guild(
    state: &AppState,
    name: &str,
    // icon: Option<String>,
    owner_id: &str,
) -> Result<String, sqlx::Error> {
    let now = Utc::now().timestamp_millis();

    let guild_id = DISCORD_SNOWFLAKE.generate(None).to_string();

    let text_category_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let voice_category_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let system_channel_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let afk_channel_id = DISCORD_SNOWFLAKE.generate(None).to_string();

    let default_permissions = PermissionFlags::VIEW_CHANNEL.bits()
        | PermissionFlags::CREATE_GUILD_EXPRESSIONS.bits()
        | PermissionFlags::SEND_MESSAGES.bits()
        | PermissionFlags::EMBED_LINKS.bits()
        | PermissionFlags::ATTACH_FILES.bits()
        | PermissionFlags::ADD_REACTIONS.bits()
        | PermissionFlags::READ_MESSAGE_HISTORY.bits()
        | PermissionFlags::CONNECT.bits()
        | PermissionFlags::SPEAK.bits();

    let channels = vec![
        (&text_category_id,  ChannelType::GuildCategory, 0, None,                     "Text channels"),
        (&voice_category_id, ChannelType::GuildCategory, 1, None,                     "Voice channels"),
        (&system_channel_id, ChannelType::GuildText,     0, Some(&text_category_id),  "general"),
        (&afk_channel_id,    ChannelType::GuildVoice,    0, Some(&voice_category_id), "General")
    ];

    let roles = vec![
        (&guild_id, 0, "@everyone", default_permissions)
    ];

    let mut tx = state.db.begin().await?;

    sqlx::query("INSERT INTO guilds (id, created_at, owner_id, name, system_channel_id) VALUES (?, ?, ?, ?, ?)")
        .bind(&guild_id)
        .bind(now)
        .bind(&owner_id)
        .bind(&name)
        .bind(&system_channel_id)
        .execute(&mut *tx)
        .await?;

    for (id, position, name, permissions) in roles.clone() {
        sqlx::query("INSERT INTO roles (id, guild_id, position, name, permissions) VALUES (?, ?, ?, ?, ?)")
            .bind(id)
            .bind(&guild_id)
            .bind(position)
            .bind(name)
            .bind(permissions)
            .execute(&mut *tx)
            .await?;
    };

    for (id, kind, position, parent_id, name) in channels {
        sqlx::query("INSERT INTO channels (id, type, position, guild_id, parent_id, name) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(kind as u32)
            .bind(position)
            .bind(&guild_id)
            .bind(parent_id)
            .bind(name)
            .execute(&mut *tx)
            .await?;

        for (role_id, _, _, _) in &roles {
            sqlx::query("INSERT INTO channel_permission_overwrites (type, channel_id, target_id) VALUES (?, ?, ?)")
                .bind(OverwriteType::Role as u32)
                .bind(id)
                .bind(role_id)
                .execute(&mut *tx)
                .await?;
        };
    };

    tx.commit().await?;

    return Ok(guild_id);
}

pub async fn add_member_to_guild(
    state: &AppState,
    user_id: &str,
    guild_id: &str,
) -> Result<(), sqlx::Error> {
    let now = Utc::now().timestamp_millis() as u64;

    let mut tx = state.db.begin().await?;

    sqlx::query("INSERT INTO guild_members (user_id, guild_id, joined_at) VALUES (?, ?, ?)")
        .bind(&user_id)
        .bind(&guild_id)
        .bind(now)
        .execute(&mut *tx)
        .await?;

    sqlx::query("INSERT INTO guild_member_roles (guild_id, user_id, role_id) VALUES (?, ?, ?)")
        .bind(&guild_id)
        .bind(&user_id)
        .bind(&guild_id)
        .execute(&mut *tx)
        .await?;

    let base_guild = sqlx::query_as::<_, BaseGuild>("SELECT * FROM guilds WHERE id = ?")
        .bind(&guild_id)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;

    let gateway_guild = GatewayGuild {
        properties: base_guild,
        channels: vec![], // ============
        roles: vec![],    // TODO: finish
        members: vec![]   // ============
    };

    state.dispatch_user("GUILD_CREATE", &gateway_guild, &user_id).await;
    // state.dispatch_guild("GUILD_MEMBER_ADD", &gateway_guild, &guild_id).await; // TODO: add guild member

    return Ok(());
}

pub async fn get_public_user(
    state: &AppState,
    user_id: &str
) -> Result<PublicUser, sqlx::Error> {
    return sqlx::query_as::<_, PublicUser>("SELECT * FROM users WHERE id = ?")
        .bind(user_id)
        .fetch_one(&state.db)
        .await;
}