use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::constants::GatewayOpcode;
use crate::db::models::{GatewayUser};

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: GatewayOpcode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>,
}

impl GatewayPayload {
    pub fn new(op: GatewayOpcode, d: Option<Value>) -> Self {
        Self {
            op,
            d,
            s: None,
            t: None,
        }
    }

    pub fn dispatch(t: String, s: u64, d: Value) -> Self {
        Self {
            op: GatewayOpcode::Dispatch,
            d: Some(d),
            s: Some(s),
            t: Some(t),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPayload {
    pub heartbeat_interval: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPayload {
    pub token: String,
    pub properties: Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ReadyPayload {
    pub v: u8,
    pub user: GatewayUser,
    pub guilds: Vec<Value>,
}