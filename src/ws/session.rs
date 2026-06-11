use std::{collections::HashSet, sync::atomic::{AtomicU64, Ordering}};
use tokio::sync::mpsc;

use crate::ws::models::GatewayPayload;
use crate::utils::constants::gateway::Opcode;

pub struct GatewaySession {
    pub id: String,
    pub user_id: Option<String>,
    pub auth_session_id: Option<String>,
    pub sender: mpsc::Sender<String>,
    pub sequence: AtomicU64,
    pub guilds: HashSet<String>
}

impl GatewaySession {
    pub fn new(id: String, sender: mpsc::Sender<String>) -> Self {
        Self {
            id,
            user_id: None,
            auth_session_id: None,
            sender,
            sequence: AtomicU64::new(0),
            guilds: HashSet::new()
        }
    }

    pub async fn send(&self, op: Opcode, t: Option<&str>, d: Option<serde_json::Value>) {
        let s = self.sequence.fetch_add(1, Ordering::Relaxed);

        let payload = GatewayPayload {
            op,
            d,
            t: t.map(|s| s.to_string()),
            s: Some(s)
        };

        let text = match serde_json::to_string(&payload) {
            Ok(t) => t,
            Err(err) => {
                tracing::error!(error = %err, "Failed to serialize gateway payload to JSON string");
                return;
            }
        };

        if let Err(_err) = self.sender.send(text).await {
             tracing::debug!("Failed to send to a client (probably disconnected)");
        };
    }

    pub async fn dispatch(&self, t: &str, d: serde_json::Value) {
        self.send(Opcode::Dispatch, Some(t), Some(d)).await;
    }

    // todo: remove
    pub async fn send_event(&self, op: u8, t: Option<&str>, d: serde_json::Value) {
        let payload = serde_json::json!({
            "op": op,
            "t": t,
            "d": d
        });
        if let Ok(text) = serde_json::to_string(&payload) {
            let _ = self.sender.send(text).await;
        }
    }
}
