use serde::{Deserialize, Serialize};
use serde_json::Value;
use crate::utils::constants::gateway::Opcode;

#[derive(Debug, Serialize, Deserialize)]
pub struct GatewayPayload {
    pub op: Opcode,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub d: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub t: Option<String>
}

impl GatewayPayload {
    pub fn new(op: Opcode, d: Option<Value>) -> Self {
        return Self {
            op,
            d,
            s: None,
            t: None
        };
    }

    // pub fn dispatch(t: String, s: u64, d: Value) -> Self {
    //     return Self {
    //         op: Opcode::Dispatch,
    //         d: Some(d),
    //         s: Some(s),
    //         t: Some(t)
    //     };
    // }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloPayload {
    pub heartbeat_interval: u32
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IdentifyPayload {
    pub token: String,
    pub properties: Value
}