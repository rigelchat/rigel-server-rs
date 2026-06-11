use std::{sync::Arc, collections::HashMap};
use tokio::sync::RwLock;
use serde::Serialize;
use sqlx::mysql::MySqlPool;

use crate::ws::session::GatewaySession;

#[derive(Clone)]
pub struct AppState { 
    pub sessions: Arc<RwLock<HashMap<String, GatewaySession>>>,
    pub db: MySqlPool
}

impl AppState {
    pub async fn dispatch_all<T: Serialize>(&self, event_name: &str, data: &T) {
        let value = match serde_json::to_value(data) {
            Ok(v) => v,
            Err(err) => {
                tracing::error!(error = %err, event = %event_name, "Failed to serialize payload to JSON Value");
                return;
            }
        };

        let sessions = self.sessions.read().await;
        for session in sessions.values() {
            let _ = session.dispatch(event_name, value.clone()).await;
        };
    }

    pub async fn dispatch_user<T: Serialize>(&self, event_name: &str, data: &T, user_id: &str) {
        let value = match serde_json::to_value(data) {
            Ok(v) => v,
            Err(err) => {
                tracing::error!(error = %err, event = %event_name, "Failed to serialize payload to JSON Value");
                return;
            }
        };

        let sessions = self.sessions.read().await;
        for session in sessions.values() {
            if session.user_id.as_deref() == Some(user_id) {
                let _ = session.dispatch(event_name, value.clone()).await;
            };
        };
    }

    pub async fn dispatch_guild<T: Serialize>(&self, event_name: &str, data: &T, guild_id: &str) {
        let value = match serde_json::to_value(data) {
            Ok(v) => v,
            Err(err) => {
                tracing::error!(error = %err, event = %event_name, "Failed to serialize payload to JSON Value");
                return;
            }
        };

        let sessions = self.sessions.read().await;
        for session in sessions.values() {
            if session.guilds.contains(guild_id) {
                let _ = session.dispatch(event_name, value.clone()).await;
            };
        };
    }
}