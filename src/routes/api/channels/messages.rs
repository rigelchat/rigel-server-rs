use axum::{
    http::{HeaderMap, StatusCode},
    Router,
    routing::{get, post, patch, delete},
    extract::{State, Path},
    Json
};
use serde::Deserialize;
use serde_json::json;

use crate::AppState;
use crate::models::{user::MessageAuthor, message::Message};
use crate::utils::{snowflake::DISCORD_SNOWFLAKE, token::verify};
use crate::utils::constants::models::MessageType;

#[derive(Deserialize)]
pub struct CreateMessageReq {
    pub content: String,
    pub nonce: Option<String>
}

#[derive(Deserialize)]
pub struct EditMessageReq {
    pub content: String,
}

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(get_messages))
        .route("/", post(create_message))
        .route("/{message_id}", patch(edit_message))
        .route("/{message_id}", delete(delete_message));
}

async fn get_user_id_from_auth(headers: &HeaderMap) -> Result<String, (StatusCode, String)> {
    let auth_header = headers.get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or((StatusCode::UNAUTHORIZED, "Missing authorization".to_string()))?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    verify(auth_header, &secret).map_err(|e| (StatusCode::UNAUTHORIZED, e.to_string()))
}

async fn broadcast_to_guild(state: &AppState, guild_id: &str, evt_type: &str, payload: serde_json::Value) {
    let sessions = state.sessions.read().await;
    for session in sessions.values() {
        if session.guilds.contains(guild_id) {
            session.send_event(0, Some(evt_type), payload.clone()).await;
        }
    }
}

// GET /api/channels/{id}/messages
async fn get_messages(
    State(state): State<AppState>,
    Path(channel_id): Path<String>
) -> Result<Json<Vec<Message>>, StatusCode> {
    // TODO: check member permission ViewChannel & ReadMessageHistory
    let rows = sqlx::query!("
        SELECT
            m.id,
            m.channel_id,
            m.content,
            m.timestamp,
            m.edited_timestamp,
            m.type as kind,
            m.flags,
            u.id as author_id,
            u.username,
            u.global_name,
            u.avatar,
            u.banner,
            u.public_flags
        FROM messages m
        INNER JOIN users u ON u.id = m.author_id
        WHERE m.channel_id = ?
        ORDER BY m.id DESC
        LIMIT 50
    ", channel_id)
        .fetch_all(&state.db)
        .await
        .map_err(|err| {
            tracing::error!(error = %err, channel_id = %channel_id, "Failed to fetch messages");
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let messages: Vec<Message> = rows.into_iter().map(|row| {
        Message {
            id: row.id,
            timestamp: row.timestamp, 
            edited_timestamp: row.edited_timestamp.map(|t| t as u64),
            channel_id: row.channel_id,
            content: row.content,
            author_id: row.author_id.clone(),
            kind: row.kind,
            flags: row.flags,
            author: MessageAuthor {
                id: row.author_id,
                username: row.username,
                global_name: row.global_name,
                avatar: row.avatar,
                banner: row.banner,
                public_flags: row.public_flags
            }
        }
    }).collect();

    return Ok(Json(messages));
}

// POST /api/channels/{id}/messages
async fn create_message(
    State(state): State<AppState>,
    Path(channel_id): Path<String>,
    headers: HeaderMap,
    Json(payload): Json<CreateMessageReq>,
) -> Result<(StatusCode, Json<Message>), (StatusCode, String)> {
    let user_id = get_user_id_from_auth(&headers).await?;

    // TODO: check member permission ViewChannel & SendMessages

    let channel = sqlx::query!("SELECT guild_id FROM channels WHERE id = ?", channel_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Channel not found".to_string()))?;

    let message_id = DISCORD_SNOWFLAKE.generate(None).to_string();
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    sqlx::query!(
        "INSERT INTO messages (id, channel_id, author_id, content, timestamp, type) VALUES (?, ?, ?, ?, ?, ?)",
        message_id,
        channel_id,
        user_id,
        payload.content,
        timestamp,
        MessageType::Default as u32
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row = sqlx::query!(
        "SELECT
            m.id, m.channel_id, m.content, m.timestamp, m.edited_timestamp, m.type AS kind, m.flags,
            u.id as author_id, u.username, u.global_name, u.avatar, u.banner, u.public_flags
        FROM messages m
        INNER JOIN users u ON u.id = m.author_id
        WHERE m.id = ?",
        message_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let msg = Message {
        id: row.id, timestamp: row.timestamp, edited_timestamp: row.edited_timestamp.map(|t| t as u64),
        channel_id: row.channel_id, content: row.content, author_id: row.author_id.clone(),
        kind: row.kind, flags: row.flags,
        author: MessageAuthor {
            id: row.author_id, username: row.username, global_name: row.global_name,
            avatar: row.avatar, banner: row.banner, public_flags: row.public_flags,
        }
    };
    
    let mut api_message = json!(msg);
    if let Some(nonce) = payload.nonce {
        api_message["nonce"] = json!(nonce);
    }

    // state.dispatch_guild(event_name, data, guild_id);
    broadcast_to_guild(&state, &channel.guild_id, "MESSAGE_CREATE", api_message).await;

    Ok((StatusCode::CREATED, Json(msg)))
}

// PATCH /api/channels/{id}/messages/{message_id}
async fn edit_message(
    State(state): State<AppState>,
    Path((channel_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
    Json(payload): Json<EditMessageReq>,
) -> Result<Json<Message>, (StatusCode, String)> {
    let user_id = get_user_id_from_auth(&headers).await?;

    let msg = sqlx::query!("SELECT author_id FROM messages WHERE id = ? AND channel_id = ?", message_id, channel_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Message not found".to_string()))?;

    if msg.author_id != user_id {
        return Err((StatusCode::FORBIDDEN, "Cannot edit message authored by another user".to_string()));
    }

    let channel = sqlx::query!("SELECT guild_id FROM channels WHERE id = ?", channel_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Channel not found".to_string()))?;

    let edited_timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    sqlx::query!(
        "UPDATE messages SET content = ?, edited_timestamp = ? WHERE id = ?",
        payload.content,
        edited_timestamp,
        message_id
    )
    .execute(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let row = sqlx::query!(
        "SELECT
            m.id, m.channel_id, m.content, m.timestamp, m.edited_timestamp, m.type AS kind, m.flags,
            u.id as author_id, u.username, u.global_name, u.avatar, u.banner, u.public_flags
        FROM messages m
        INNER JOIN users u ON u.id = m.author_id
        WHERE m.id = ?",
        message_id
    )
    .fetch_one(&state.db)
    .await
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    let updated_msg = Message {
        id: row.id, timestamp: row.timestamp, edited_timestamp: row.edited_timestamp.map(|t| t as u64),
        channel_id: row.channel_id, content: row.content, author_id: row.author_id.clone(),
        kind: row.kind, flags: row.flags,
        author: MessageAuthor {
            id: row.author_id, username: row.username, global_name: row.global_name,
            avatar: row.avatar, banner: row.banner, public_flags: row.public_flags,
        }
    };

    broadcast_to_guild(&state, &channel.guild_id, "MESSAGE_UPDATE", json!(updated_msg)).await;

    Ok(Json(updated_msg))
}

// DELETE /api/channels/{id}/messages/{message_id}
async fn delete_message(
    State(state): State<AppState>,
    Path((channel_id, message_id)): Path<(String, String)>,
    headers: HeaderMap,
) -> Result<StatusCode, (StatusCode, String)> {
    let user_id = get_user_id_from_auth(&headers).await?;

    let msg = sqlx::query!("SELECT author_id FROM messages WHERE id = ? AND channel_id = ?", message_id, channel_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Message not found".to_string()))?;

    // TODO: also check ManageMessages permission
    if msg.author_id != user_id {
        return Err((StatusCode::FORBIDDEN, "Missing Permissions".to_string()));
    }

    let channel = sqlx::query!("SELECT guild_id FROM channels WHERE id = ?", channel_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?
        .ok_or((StatusCode::NOT_FOUND, "Channel not found".to_string()))?;

    sqlx::query!("DELETE FROM messages WHERE id = ?", message_id)
        .execute(&state.db)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    broadcast_to_guild(&state, &channel.guild_id, "MESSAGE_DELETE", json!({
        "id": message_id,
        "channel_id": channel_id,
        "guild_id": channel.guild_id
    })).await;

    Ok(StatusCode::NO_CONTENT)
}
