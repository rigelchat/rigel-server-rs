use std::collections::HashSet;
use tokio::sync::mpsc;
use serde_json::Value;

pub struct WsSession {
    pub id: String,
    pub user_id: Option<String>,
    pub auth_session_id: Option<String>,
    pub sender: mpsc::Sender<String>,
    pub guilds: HashSet<String>,
}

impl WsSession {
    pub fn new(id: String, sender: mpsc::Sender<String>) -> Self {
        Self {
            id,
            user_id: None,
            auth_session_id: None,
            sender,
            guilds: HashSet::new(),
        }
    }

    pub async fn send_event(&self, op: u8, t: Option<&str>, d: Value) {
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
