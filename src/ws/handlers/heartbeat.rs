use crate::AppState;
use crate::utils::constants::gateway::Opcode;

pub async fn handle(session_id: &str, state: &AppState) {
    let sessions = state.sessions.read().await;
    if let Some(session) = sessions.get(session_id) {
        let _ = session.send(Opcode::HeartbeatAck, None, None).await;
    };
}