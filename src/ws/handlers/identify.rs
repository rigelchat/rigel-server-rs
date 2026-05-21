use serde_json::json;
use sqlx::Row;
use std::collections::{HashMap, HashSet};
use tracing::{info, error};

use crate::db::AppState;
use crate::utils::token::verify;
use crate::utils::constants;
use crate::ws::models::IdentifyPayload;

pub async fn handle(
    session_id: &str,
    state: &AppState,
    payload: IdentifyPayload,
) {
    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
    let user_id = match verify(&payload.token, &secret) {
        Ok(id) => id,
        Err(e) => {
            error!("Identify token validation failed: {}", e);
            // In a real app we'd close the connection with 4004
            return;
        }
    };

    // Find token64 from token string
    let parts: Vec<&str> = payload.token.split('.').collect();
    if parts.len() != 3 { return; }
    let auth_session_id = parts[1].to_string();

    let os = payload.properties.get("os").and_then(|v| v.as_str()).unwrap_or("unknown");
    let browser = payload.properties.get("browser").and_then(|v| v.as_str()).unwrap_or("unknown");

    if let Err(e) = sqlx::query(
        "UPDATE user_sessions SET last_used_at = ?, os = ?, platform = ? WHERE id = ?"
    )
    .bind(chrono::Utc::now().timestamp_millis())
    .bind(os)
    .bind(browser)
    .bind(&auth_session_id)
    .execute(&state.db)
    .await {
        error!("Failed to update user_sessions: {}", e);
        return;
    }

    let user_row = match sqlx::query("SELECT * FROM users WHERE id = ?").bind(&user_id).fetch_one(&state.db).await {
        Ok(r) => r,
        Err(e) => {
            error!("User not found in identify: {}", e);
            return;
        }
    };

    let user_obj = json!({
        "id": user_row.try_get::<String, _>("id").unwrap(),
        "bot": user_row.try_get::<bool, _>("bot").unwrap(),
        "public_flags": user_row.try_get::<u32, _>("public_flags").unwrap(),
        "username": user_row.try_get::<String, _>("username").unwrap(),
        "global_name": user_row.try_get::<Option<String>, _>("global_name").unwrap(),
        "discriminator": user_row.try_get::<Option<String>, _>("discriminator").unwrap(),
        "avatar": user_row.try_get::<Option<String>, _>("avatar").unwrap(),
        "banner": user_row.try_get::<Option<String>, _>("banner").unwrap()
    });

    let user_settings_row = sqlx::query("SELECT * FROM user_settings WHERE id = ?")
        .bind(&user_id)
        .fetch_optional(&state.db)
        .await
        .unwrap_or(None);

    let user_settings_obj = if let Some(row) = user_settings_row {
        json!({
            "status": row.try_get::<String, _>("status").unwrap(),
            "afk_timeout": row.try_get::<u32, _>("afk_timeout").unwrap(),
            "locale": row.try_get::<String, _>("locale").unwrap(),
            "theme": row.try_get::<String, _>("theme").unwrap(),
            "background_gradient_preset": row.try_get::<Option<String>, _>("background_gradient_preset").unwrap(),
            "developer_mode": row.try_get::<bool, _>("developer_mode").unwrap()
        })
    } else {
        json!(null)
    };

    let guild_rows = match sqlx::query(
        "SELECT g.* FROM guilds g INNER JOIN guild_members gm ON g.id = gm.guild_id WHERE gm.user_id = ?"
    )
    .bind(&user_id)
    .fetch_all(&state.db)
    .await {
        Ok(r) => r,
        Err(_) => vec![],
    };

    let guild_ids: Vec<String> = guild_rows.iter().map(|r| r.try_get("id").unwrap_or_default()).collect();

    // Update Session logic
    {
        let mut sessions = state.sessions.write().await;
        if let Some(session) = sessions.get_mut(session_id) {
            session.user_id = Some(user_id.clone());
            session.auth_session_id = Some(auth_session_id.clone());
            session.guilds = guild_ids.iter().cloned().collect();
        }
    }

    if let Err(_) = sqlx::query("UPDATE gateway_sessions SET user_id = ?, user_session_id = ? WHERE id = ?")
        .bind(&user_id)
        .bind(&auth_session_id)
        .bind(session_id)
        .execute(&state.db)
        .await
    {}

    let mut channels = Vec::new();
    let mut roles = Vec::new();
    let mut guild_members = Vec::new();
    let mut all_guild_users = Vec::new();
    let mut guild_member_roles = Vec::new();

    if !guild_ids.is_empty() {
        let placeholders = guild_ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        
        let q_ch = format!("SELECT * FROM channels WHERE guild_id IN ({})", placeholders);
        let mut q_ch_bind = sqlx::query(&q_ch);
        for id in &guild_ids { q_ch_bind = q_ch_bind.bind(id); }
        if let Ok(rows) = q_ch_bind.fetch_all(&state.db).await { channels = rows; }

        let q_roles = format!("SELECT * FROM guild_roles WHERE guild_id IN ({})", placeholders);
        let mut q_roles_bind = sqlx::query(&q_roles);
        for id in &guild_ids { q_roles_bind = q_roles_bind.bind(id); }
        if let Ok(rows) = q_roles_bind.fetch_all(&state.db).await { roles = rows; }

        let q_gm = format!("SELECT * FROM guild_members WHERE guild_id IN ({})", placeholders);
        let mut q_gm_bind = sqlx::query(&q_gm);
        for id in &guild_ids { q_gm_bind = q_gm_bind.bind(id); }
        if let Ok(rows) = q_gm_bind.fetch_all(&state.db).await { guild_members = rows; }

        let q_u = format!("SELECT u.* FROM users u INNER JOIN guild_members gm ON u.id = gm.user_id WHERE gm.guild_id IN ({})", placeholders);
        let mut q_u_bind = sqlx::query(&q_u);
        for id in &guild_ids { q_u_bind = q_u_bind.bind(id); }
        if let Ok(rows) = q_u_bind.fetch_all(&state.db).await { all_guild_users = rows; }

        let q_gmr = format!("SELECT gmr.* FROM guild_member_roles gmr INNER JOIN guild_members gm ON gmr.user_id = gm.user_id AND gmr.guild_id = gm.guild_id WHERE gm.guild_id IN ({})", placeholders);
        let mut q_gmr_bind = sqlx::query(&q_gmr);
        for id in &guild_ids { q_gmr_bind = q_gmr_bind.bind(id); }
        if let Ok(rows) = q_gmr_bind.fetch_all(&state.db).await { guild_member_roles = rows; }
    }

    let mut users_map = HashMap::new();
    for u in all_guild_users {
        let u_id: String = u.try_get("id").unwrap();
        users_map.insert(u_id, json!({
            "id": u.try_get::<String, _>("id").unwrap(),
            "username": u.try_get::<String, _>("username").unwrap(),
            "global_name": u.try_get::<Option<String>, _>("global_name").unwrap(),
            "discriminator": u.try_get::<Option<String>, _>("discriminator").unwrap(),
            "avatar": u.try_get::<Option<String>, _>("avatar").unwrap(),
            "bot": u.try_get::<bool, _>("bot").unwrap()
        }));
    }

    let mut grouped_channels = HashMap::new();
    for c in channels {
        let gid: String = c.try_get("guild_id").unwrap();
        grouped_channels.entry(gid).or_insert_with(Vec::new).push(json!({
            "id": c.try_get::<String, _>("id").unwrap(),
            "type": c.try_get::<u32, _>("type").unwrap(),
            "position": c.try_get::<u32, _>("position").unwrap(),
            "guild_id": c.try_get::<String, _>("guild_id").unwrap(),
            "parent_id": c.try_get::<Option<String>, _>("parent_id").unwrap(),
            "name": c.try_get::<String, _>("name").unwrap()
        }));
    }

    let mut grouped_roles = HashMap::new();
    for r in roles {
        let gid: String = r.try_get("guild_id").unwrap();
        grouped_roles.entry(gid).or_insert_with(Vec::new).push(json!({
            "id": r.try_get::<String, _>("id").unwrap(),
            "guild_id": r.try_get::<String, _>("guild_id").unwrap(),
            "position": r.try_get::<u32, _>("position").unwrap(),
            "name": r.try_get::<String, _>("name").unwrap(),
            "color": r.try_get::<u32, _>("color").unwrap(),
            "permissions": r.try_get::<u64, _>("permissions").unwrap().to_string()
        }));
    }

    let mut grouped_gmr = HashMap::new();
    for gmr in guild_member_roles {
        let gid: String = gmr.try_get("guild_id").unwrap_or_default();
        let uid: String = gmr.try_get("user_id").unwrap_or_default();
        let rid: String = gmr.try_get("role_id").unwrap_or_default();
        grouped_gmr.entry(format!("{}_{}", gid, uid)).or_insert_with(Vec::new).push(rid);
    }

    // Prepare active sessions presences
    let mut online_users_per_guild: HashMap<String, HashSet<String>> = HashMap::new();
    {
        let sessions = state.sessions.read().await;
        for session in sessions.values() {
            if let Some(s_uid) = &session.user_id {
                for g in &session.guilds {
                    online_users_per_guild.entry(g.clone()).or_insert_with(HashSet::new).insert(s_uid.clone());
                }
            }
        }
    }

    let mut final_guilds = Vec::new();
    let mut merged_members = Vec::new();

    for g in guild_rows {
        let gid: String = g.try_get("id").unwrap_or_default();
        
        let g_chans = grouped_channels.remove(&gid).unwrap_or_default();
        let g_roles = grouped_roles.remove(&gid).unwrap_or_default();

        let mut members_in_guild = Vec::new();
        // find members for this guild
        for gm in &guild_members {
            let gm_gid: String = gm.try_get("guild_id").unwrap_or_default();
            if gm_gid == gid {
                let uid: String = gm.try_get("user_id").unwrap_or_default();
                let user_data = users_map.get(&uid).cloned().unwrap_or(json!(null));
                let roles = grouped_gmr.get(&format!("{}_{}", gid, uid)).cloned().unwrap_or_default();
                
                members_in_guild.push(json!({
                    "id": gm.try_get::<String, _>("id").unwrap_or_default(),
                    "user_id": uid,
                    "guild_id": gid.clone(),
                    "user": user_data,
                    "roles": roles,
                    "joined_at": gm.try_get::<u64, _>("joined_at").unwrap_or(0),
                }));
            }
        }

        let o_users = online_users_per_guild.get(&gid).cloned().unwrap_or_default();
        let mut presences = Vec::new();
        for uid in o_users {
            presences.push(json!({
                "user": { "id": uid },
                "status": "online" // mock default status
            }));
        }

        final_guilds.push(json!({
            "id": gid.clone(),
            "name": g.try_get::<String, _>("name").unwrap_or_default(),
            "owner_id": g.try_get::<String, _>("owner_id").unwrap_or_default(),
            "icon": g.try_get::<Option<String>, _>("icon").unwrap_or_default(),
            "channels": g_chans,
            "roles": g_roles,
            "members": members_in_guild,
            "presences": presences,
            "properties": {
                "id": gid.clone(),
                "name": g.try_get::<String, _>("name").unwrap_or_default(),
            }
        }));

        merged_members.push(members_in_guild);
    }

    let ready_payload = json!({
        "session_id": session_id,
        "auth_session_id_hash": auth_session_id,
        "resume_gateway_url": format!("{}/gateway", std::env::var("HOST").unwrap_or_else(|_| "http://localhost:3000".to_string())),
        "sessions": [
            {
                "session_id": "all",
                "status": "online",
                "client_info": { "os": "unknown", "client": "unknown" },
                "activities": [],
                "active": true
            }
        ],
        "user": user_obj,
        "user_settings": user_settings_obj,
        "merged_members": merged_members,
        "guilds": final_guilds,
    });

    info!("User {} identified, sending READY", user_id);

    {
        let sessions = state.sessions.read().await;
        if let Some(session) = sessions.get(session_id) {
            session.send_event(constants::gateway::Opcode::Dispatch as u8, Some(constants::gateway::events::READY), ready_payload).await;
        }
    }

    // Broadcast PresenceUpdate to peers
    let presence_d = json!({
        "user": { "id": user_id.clone() },
        "status": "online",
        "client_status": {
            "desktop": "online",
            "mobile": "online",
            "web": "online"
        },
        "activities": []
    });

    {
        let sessions = state.sessions.read().await;
        for s in sessions.values() {
            if let Some(ref peer_uid) = s.user_id {
                if peer_uid == &user_id { continue; } // Don't send to self over cross guilds ideally, wait we should filter by shared guilds

                let guild_ids_set = guild_ids.iter().cloned().collect::<HashSet<_>>();
                let shared_guilds: Vec<_> = s.guilds.intersection(&guild_ids_set).collect();
                for g in shared_guilds {
                    let mut d = presence_d.clone();
                    d["guild_id"] = json!(g);
                    s.send_event(constants::gateway::Opcode::Dispatch as u8, Some(constants::gateway::events::PRESENCE_UPDATE), d).await;
                }
            }
        }
    }
}
