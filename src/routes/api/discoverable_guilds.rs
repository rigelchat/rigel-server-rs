use axum::{Router, routing::get, Json, extract::State, http::StatusCode};
use serde::Serialize;
use sqlx::Row;

use crate::AppState;
use crate::models::guild::DiscoverableGuild;

pub fn router() -> Router<AppState> {
    return Router::new()
        .route("/", get(get_discoverable_guilds));
}

// GET /api/discoverable-guilds
async fn get_discoverable_guilds(State(state): State<AppState>) -> Result<Json<DiscoverableGuildsResponse>, StatusCode> {
    let connected_user_ids: Vec<String> = {
        let sessions = state.sessions.read().await;
        sessions.values().filter_map(|s| s.user_id.clone()).collect::<std::collections::HashSet<_>>().into_iter().collect()
    };

    let rows = match sqlx::query("
        SELECT
            g.id,
            g.name,
            g.icon,
            g.banner,
            g.description,
            g.vanity_url_code,
            COUNT(DISTINCT gm.user_id) as approximate_member_count
        FROM guilds g
        LEFT JOIN guild_members gm ON g.id = gm.guild_id
        WHERE g.discoverable = 1
        GROUP BY g.id
        ORDER BY g.name ASC
    ").fetch_all(&state.db).await {
        Ok(r) => r,
        Err(e) => {
            tracing::error!("Failed to fetch guilds: {}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let mut guilds = Vec::new();
    
    for row in rows {
        let id: String = row.try_get("id").unwrap_or_default();
        let name: String = row.try_get("name").unwrap_or_default();
        let icon: Option<String> = row.try_get("icon").ok();
        let banner: Option<String> = row.try_get("banner").ok();
        let description: Option<String> = row.try_get("description").ok();
        let vanity_url_code: Option<String> = row.try_get("vanity_url_code").ok();
        let member_count: u64 = row.try_get("approximate_member_count").unwrap_or(0);
        let mut presence_count: u64 = 0;

        if !connected_user_ids.is_empty() {
            let placeholders = vec!["?"; connected_user_ids.len()].join(", ");
            let p_query = format!("SELECT COUNT(DISTINCT user_id) as c FROM guild_members WHERE guild_id = '{}' AND user_id IN ({})", id, placeholders);

            let mut q = sqlx::query(&p_query);
            for uid in &connected_user_ids { q = q.bind(uid); };

            if let Ok(p_row) = q.fetch_one(&state.db).await {
                presence_count = p_row.try_get("c").unwrap_or(0);
            };
        };

        guilds.push(DiscoverableGuild {
            id,
            name,
            icon,
            banner,
            description,
            vanity_url_code,
            approximate_member_count: member_count,
            approximate_presence_count: presence_count
        });
    }

    let total = guilds.len() as u64;

    return Ok(Json(DiscoverableGuildsResponse {
        limit: total,
        offset: 0,
        total,
        guilds,
    }));
}

#[derive(Serialize)]
pub struct DiscoverableGuildsResponse {
    guilds: Vec<DiscoverableGuild>,
    offset: u64,
    limit: u64,
    total: u64
}