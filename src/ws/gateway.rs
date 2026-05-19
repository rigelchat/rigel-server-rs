use axum::{
    extract::{ws::{Message, WebSocket, WebSocketUpgrade}, State},
    response::IntoResponse,
};
use serde_json::json;
use tracing::{info, error};

use crate::{
    db::AppState,
    db::queries::get_user_by_id,
    db::models::GatewayUser,
    utils::constants::{GATEWAY_EVENTS_READY, GATEWAY_HEARTBEAT_INTERVAL, GatewayOpcode}
};
use crate::ws::models::{GatewayPayload, HelloPayload, IdentifyPayload, ReadyPayload};
use crate::utils::token::verify_token;

pub async fn handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(mut socket: WebSocket, state: AppState) {
    info!("New WebSocket connection established");

    // Envoyer le message Hello (op 10) immédiatement après la connexion
    let hello = GatewayPayload::new(
        GatewayOpcode::Hello,
        Some(json!(HelloPayload {
            heartbeat_interval: GATEWAY_HEARTBEAT_INTERVAL,
        })),
    );

    if let Ok(text) = serde_json::to_string(&hello) {
        if socket.send(Message::Text(text.into())).await.is_err() {
            return;
        }
    }

    while let Some(msg) = socket.recv().await {
        let msg = match msg {
            Ok(msg) => msg,
            Err(e) => {
                error!("WebSocket error: {}", e);
                return;
            }
        };

        match msg {
            Message::Text(text) => {
                info!("Received text: {}", text);
                
                if let Ok(payload) = serde_json::from_str::<GatewayPayload>(&text) {
                    match payload.op {
                        GatewayOpcode::Identify => {
                            if let Some(d) = payload.d {
                                if let Ok(identify) = serde_json::from_value::<IdentifyPayload>(d) {
                                    // Utilisation d'un secret temporaire ou config
                                    let secret = std::env::var("AUTH_SECRET").unwrap_or_else(|_| "secret".to_string());
                                    
                                    match verify_token(&identify.token, &secret) {
                                        Ok(user_id) => {
                                            info!("User {} identified successfully", user_id);

                                            let user = match get_user_by_id(&state, &user_id).await {
                                                Ok(Some(u)) => u,
                                                Ok(None) => {
                                                    error!("User {} not found in database", user_id);
                                                    return;
                                                }
                                                Err(e) => {
                                                    error!("Database error fetching user {}: {}", user_id, e);
                                                    return;
                                                }
                                            };
                                            
                                            let ready = GatewayPayload::dispatch(
                                                GATEWAY_EVENTS_READY.to_string(),
                                                1,
                                                json!(ReadyPayload {
                                                    v: 10,
                                                    user,
                                                    guilds: vec![]
                                                })
                                            );

                                            if let Ok(ready_text) = serde_json::to_string(&ready) {
                                                let _ = socket.send(Message::Text(ready_text.into())).await;
                                            }
                                        }
                                        Err(e) => {
                                            error!("Identification failed: {}", e);
                                            return;
                                        }
                                    }
                                }
                            }
                        }
                        _ => {
                            // Echo temporaire pour les autres messages
                            if socket.send(Message::Text(format!("Echo: {}", text).into())).await.is_err() {
                                return;
                            }
                        }
                    }
                }
            }
            Message::Binary(bin) => {
                info!("Received binary of length: {}", bin.len());
                if socket.send(Message::Binary(bin)).await.is_err() {
                    return;
                }
            }
            Message::Close(_) => {
                info!("Client disconnected");
                return;
            }
            _ => {}
        }
    }
}