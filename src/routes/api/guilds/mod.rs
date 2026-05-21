pub mod members;
pub mod roles;

use axum::{Router, routing::post, Json, extract::State, http::{StatusCode, HeaderMap}};
use serde::{Deserialize};
use crate::AppState;
use crate::utils::{snowflake::DISCORD_SNOWFLAKE, token::verify};
use crate::utils::constants::models::{ChannelType, MessageType, OverwriteType};
use crate::utils::constants::flags::{PermissionFlags};
use chrono::Utc;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", post(create_guild));
}

// POST /api/guilds
async fn create_guild(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateGuildRequest>,
) -> Result<Json<serde_json::Value>, StatusCode> {
    
    // Auth Check
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = verify(auth_header, &secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Staff Check
    let user_flags: (u32,) = sqlx::query_as("SELECT public_flags FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if user_flags.0 & 1 == 0 { // 1 = UserFlags::STAFF
        return Err(StatusCode::FORBIDDEN);
    }

    let guild_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let text_category_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let voice_category_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let system_channel_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let afk_channel_id = DISCORD_SNOWFLAKE.generate(None).to_string();

    let default_permissions = 
        PermissionFlags::VIEW_CHANNEL.bits() |
        PermissionFlags::CREATE_GUILD_EXPRESSIONS.bits() |
        PermissionFlags::SEND_MESSAGES.bits() |
        PermissionFlags::EMBED_LINKS.bits() |
        PermissionFlags::ATTACH_FILES.bits() |
        PermissionFlags::ADD_REACTIONS.bits() |
        PermissionFlags::READ_MESSAGE_HISTORY.bits() |
        PermissionFlags::CONNECT.bits() |
        PermissionFlags::SPEAK.bits();

    let now = Utc::now().timestamp_millis();

    let mut tx = state.db.begin().await.map_err(|_| {
        tracing::error!("Failed to begin transaction");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    sqlx::query("INSERT INTO guilds (id, created_at, owner_id, name, system_channel_id) VALUES (?, ?, ?, ?, ?)")
        .bind(&guild_id)
        .bind(now)
        .bind(&user_id)
        .bind(&payload.name)
        .bind(&system_channel_id)
        .execute(&mut *tx).await.map_err(|e| {
            tracing::error!("Failed to insert guild: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    sqlx::query("INSERT INTO guild_roles (id, guild_id, position, name, permissions) VALUES (?, ?, ?, ?, ?)")
        .bind(&guild_id)
        .bind(&guild_id)
        .bind(0)
        .bind("@everyone")
        .bind(default_permissions.to_string())
        .execute(&mut *tx).await.map_err(|e| {
            tracing::error!("Failed to insert everyone role: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let channels = vec![
        (&text_category_id, ChannelType::GuildCategory as u32, 0, &guild_id, None::<&String>, "Salons textuels"),
        (&voice_category_id, ChannelType::GuildCategory as u32, 1, &guild_id, None, "Salons vocaux"),
        (&system_channel_id, ChannelType::GuildText as u32, 0, &guild_id, Some(&text_category_id), "général"),
        (&afk_channel_id, ChannelType::GuildVoice as u32, 0, &guild_id, Some(&voice_category_id), "Général"),
    ];

    for (id, c_type, pos, g_id, parent, name) in channels {
        sqlx::query("INSERT INTO channels (id, type, position, guild_id, parent_id, name) VALUES (?, ?, ?, ?, ?, ?)")
            .bind(id)
            .bind(c_type)
            .bind(pos)
            .bind(g_id)
            .bind(parent)
            .bind(name)
            .execute(&mut *tx).await.map_err(|e| {
                tracing::error!("Failed to insert channel: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    let overwrites = vec![
        &text_category_id,
        &voice_category_id,
        &system_channel_id,
        &afk_channel_id,
    ];

    let overwrite_type = OverwriteType::Role as u32;
    for target_chan in overwrites {
        sqlx::query("INSERT INTO channel_permission_overwrites (id, type, channel_id, target_id) VALUES (?, ?, ?, ?)")
            .bind(DISCORD_SNOWFLAKE.generate(None).to_string())
            .bind(overwrite_type)
            .bind(target_chan)
            .bind(&guild_id)
            .execute(&mut *tx).await.map_err(|e| {
                tracing::error!("Failed to insert overwrites: {}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
    }

    let member_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    sqlx::query("INSERT INTO guild_members (id, user_id, guild_id, joined_at) VALUES (?, ?, ?, ?)")
        .bind(&member_id)
        .bind(&user_id)
        .bind(&guild_id)
        .bind(now)
        .execute(&mut *tx).await.map_err(|e| {
            tracing::error!("Failed to insert guild_members: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    sqlx::query("INSERT INTO guild_member_roles (id, guild_id, user_id, role_id) VALUES (?, ?, ?, ?)")
        .bind(DISCORD_SNOWFLAKE.generate(None).to_string())
        .bind(&guild_id)
        .bind(&user_id)
        .bind(&guild_id)
        .execute(&mut *tx).await.map_err(|e| {
            tracing::error!("Failed to insert guild_member_roles: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    sqlx::query("INSERT INTO messages (id, timestamp, channel_id, author_id, type) VALUES (?, ?, ?, ?, ?)")
        .bind(DISCORD_SNOWFLAKE.generate(None).to_string())
        .bind(now)
        .bind(&system_channel_id)
        .bind(&user_id)
        .bind(MessageType::UserJoin as u32)
        .execute(&mut *tx).await.map_err(|e| {
            tracing::error!("Failed to insert message: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    tx.commit().await.map_err(|e| {
        tracing::error!("Failed to commit transaction: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let res = serde_json::json!({
        "id": guild_id,
        "name": payload.name,
        "owner_id": user_id,
        "system_channel_id": system_channel_id,
        "channels": [
            { "id": text_category_id, "type": ChannelType::GuildCategory as u32, "name": "Salons textuels", "parent_id": None::<String> },
            { "id": voice_category_id, "type": ChannelType::GuildCategory as u32, "name": "Salons vocaux", "parent_id": None::<String> },
            { "id": system_channel_id, "type": ChannelType::GuildText as u32, "name": "général", "parent_id": text_category_id },
            { "id": afk_channel_id, "type": ChannelType::GuildVoice as u32, "name": "Général", "parent_id": voice_category_id }
        ],
        "roles": [
            { "id": guild_id, "name": "@everyone", "permissions": default_permissions.to_string() }
        ],
        "members": [
            { "user_id": user_id, "roles": [guild_id] }
        ]
    });

    // Broadcast event
    {
        let event_payload = serde_json::json!({
            "op": 0,
            "t": "GUILD_CREATE",
            "d": &res,
        });
        let sessions = state.sessions.read().await;
        for session in sessions.values() {
            if session.user_id.as_deref() == Some(user_id.as_str()) {
                if let Ok(text) = serde_json::to_string(&event_payload) {
                    let _ = session.sender.send(text).await;
                }
            }
        }
    }

    return Ok(Json(res));
}

#[derive(Deserialize)]
pub struct CreateGuildRequest {
    pub name: String,
}