pub mod members;
pub mod roles;

use std::collections::HashMap;

use axum::{Router, routing::post, Json, extract::State, http::{StatusCode, HeaderMap}};
use serde::{Serialize, Deserialize};
use tracing::error;

use crate::{AppState, models::guild_member::GuildMember};
use crate::services;
use crate::models::{
    channel::GuildChannel,
    guild::BaseGuild,
    guild_member::{BaseGuildMember, GuildMemberRole},
    role::Role,
    user::User
};
use crate::utils::constants::flags::UserFlags;
use crate::utils::token::verify;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", post(create_guild));
}

// POST /api/guilds
async fn create_guild(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateGuildRequest>
) -> Result<Json<CreateGuildResponse>, StatusCode> {
    // Auth Check
    let auth_header = headers.get("authorization").and_then(|h| h.to_str().ok()).ok_or(StatusCode::UNAUTHORIZED)?;
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = verify(auth_header, &secret).map_err(|_| StatusCode::UNAUTHORIZED)?;

    // Staff Check
    let user: User = sqlx::query_as("SELECT * FROM users WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    let user_flags = UserFlags::from_bits_truncate(user.public_flags);

    if !user_flags.contains(UserFlags::STAFF) {
        return Err(StatusCode::FORBIDDEN);
    };

    let guild_id = services::create_guild(&state, &payload.name, &user_id)
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to create guild");
            return StatusCode::INTERNAL_SERVER_ERROR;
        })
        .unwrap();

    let _ = services::add_member_to_guild(&state, &user_id, &guild_id)
        .await
        .map_err(|err| {
            error!(error = %err, "Failed to add guild member");
            return StatusCode::INTERNAL_SERVER_ERROR;
        });

    let guild = sqlx::query_as::<_, BaseGuild>("SELECT * FROM guilds WHERE id = ?")
        .bind(&guild_id)
        .fetch_one(&state.db)
        .await
        .unwrap();

    let channels = sqlx::query_as::<_, GuildChannel>("SELECT * FROM channels WHERE guild_id = ?")
        .bind(&guild_id)
        .fetch_all(&state.db)
        .await
        .unwrap();

    let roles = sqlx::query_as::<_, Role>("SELECT * FROM roles WHERE guild_id = ?")
        .bind(&guild_id)
        .fetch_all(&state.db)
        .await
        .unwrap();

    let base_guild_members = sqlx::query_as::<_, BaseGuildMember>("SELECT * FROM guild_members WHERE guild_id = ?")
        .bind(&guild_id)
        .fetch_all(&state.db)
        .await
        .unwrap();

    let guild_members_roles = sqlx::query_as::<_, GuildMemberRole>("SELECT * FROM guild_member_roles WHERE guild_id = ?")
        .bind(&guild_id)
        .fetch_all(&state.db)
        .await
        .unwrap();

    let roles_map: HashMap<String, Role> = roles
        .iter()
        .map(|role| (role.id.clone(), role.clone()))
        .collect();

    let mut guild_members_roles_map: HashMap<String, Vec<String>> = HashMap::new();
    for rel in guild_members_roles {
        guild_members_roles_map
            .entry(rel.user_id)
            .or_default()
            .push(rel.role_id);
    }

    let guild_members: Vec<GuildMember> = base_guild_members
        .into_iter()
        .map(|base| {
            let member_roles = if let Some(role_ids) = guild_members_roles_map.get(&base.user_id) {
                role_ids
                    .iter()
                    .filter_map(|role_id| roles_map.get(role_id).cloned())
                    .collect()
            } else {
                Vec::new()
            };

            return GuildMember {
                base,
                roles: member_roles
            };
        })
        .collect();

    return Ok(Json(CreateGuildResponse {
        base: guild,
        channels,
        roles,
        members: guild_members
    }));
}

#[derive(Deserialize)]
struct CreateGuildRequest {
    name: String,
}

#[derive(Serialize)]
struct CreateGuildResponse {
    #[serde(flatten)]
    base: BaseGuild,
    channels: Vec<GuildChannel>,
    roles: Vec<Role>,
    members: Vec<GuildMember>
}