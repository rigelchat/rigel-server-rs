use futures_util::{SinkExt, StreamExt};
use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use serde_json::json;
use tracing::{debug, error};
use tokio::sync::mpsc;
use uuid::Uuid;

use crate::{
    db::AppState,
    utils::constants::gateway::{Opcode, HEARTBEAT_INTERVAL}
};
use crate::ws::models::{GatewayPayload, HelloPayload, IdentifyPayload};
use crate::ws::session::WsSession;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |ws| handle_socket(ws, state))
}

async fn handle_socket(ws: WebSocket, state: AppState) {
    debug!("New WebSocket connection established");

    let (mut socket_sender, mut socket_receiver) = ws.split();

    let session_id = Uuid::new_v4().to_string();
    let (tx, mut rx) = mpsc::channel::<String>(100);

    {
        let mut sessions = state.sessions.write().await;
        sessions.insert(session_id.clone(), WsSession::new(session_id.clone(), tx.clone()));
    }

    let hello = GatewayPayload::new(
        Opcode::Hello,
        Some(json!(HelloPayload {
            heartbeat_interval: HEARTBEAT_INTERVAL
        })),
    );

    if let Ok(text) = serde_json::to_string(&hello) {
        let _ = socket_sender.send(Message::Text(text.into())).await;
    }

    let session_id_clone = session_id.clone();
    let state_clone = state.clone();

    let mut send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if socket_sender.send(Message::Text(msg.into())).await.is_err() {
                break;
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(msg) = socket_receiver.next().await {
            let msg = match msg {
                Ok(msg) => msg,
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    break;
                }
            };

            match msg {
                Message::Text(text) => {
                    if let Ok(payload) = serde_json::from_str::<GatewayPayload>(&text) {
                        match payload.op {
                            Opcode::Identify => {
                                if let Some(d) = payload.d {
                                    if let Ok(identify) = serde_json::from_value::<IdentifyPayload>(d) {
                                        crate::ws::handlers::identify::handle(&session_id_clone, &state_clone, identify).await;
                                    }
                                }
                            }
                            Opcode::Heartbeat => {
                                crate::ws::handlers::heartbeat::handle(&session_id_clone, &state_clone).await;
                            }
                            _ => {}
                        }
                    }
                }
                Message::Close(_) => {
                    break;
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = (&mut send_task) => recv_task.abort(),
        _ = (&mut recv_task) => send_task.abort(),
    }

    {
        let mut sessions = state.sessions.write().await;
        sessions.remove(&session_id);
    }

    debug!("WebSocket disconnected and session destroyed");
}
