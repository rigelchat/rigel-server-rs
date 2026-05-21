use crate::db::AppState;
use crate::ws::models::GatewayPayload;
use crate::utils::constants;

pub async fn handle(session_id: &str, state: &AppState) {
    let sessions = state.sessions.read().await;
    if let Some(session) = sessions.get(session_id) {
        let ack = GatewayPayload::new(constants::gateway::Opcode::HeartbeatAck, None);
        if let Ok(text) = serde_json::to_string(&ack) {
            let _ = session.sender.send(text).await;
        }
    }
}
