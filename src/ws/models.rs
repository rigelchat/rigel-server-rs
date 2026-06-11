use serde::{Deserialize, Serialize};

use crate::utils::constants::gateway::Opcode;

#[derive(Serialize, Deserialize)]
pub struct GatewayPayload<T = serde_json::Value> {
    pub op: Opcode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>
}

impl<T> GatewayPayload<T> {
    pub fn new(op: Opcode, d: Option<T>) -> Self {
        return Self {
            op,
            d,
            s: None,
            t: None
        };
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPayload {
    pub heartbeat_interval: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPayload {
    pub token: String,
    pub properties: serde_json::Value
}