#![allow(dead_code)]

use serde::{Deserialize, Serialize};

pub const HEARTBEAT_INTERVAL: u32 = 41250;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum Opcode {
    Dispatch = 0,
    Heartbeat = 1,
    Identify = 2,
    PresenceUpdate = 3,
    VoiceStateUpdate = 4,
    Resume = 6,
    Reconnect = 7,
    RequestGuildMembers = 8,
    InvalidSession = 9,
    Hello = 10,
    HeartbeatAck = 11
}

impl From<u8> for Opcode {
    fn from(op: u8) -> Self {
        match op {
            1  => Opcode::Heartbeat,
            2  => Opcode::Identify,
            3  => Opcode::PresenceUpdate,
            4  => Opcode::VoiceStateUpdate,
            6  => Opcode::Resume,
            7  => Opcode::Reconnect,
            8  => Opcode::RequestGuildMembers,
            9  => Opcode::InvalidSession,
            10 => Opcode::Hello,
            11 => Opcode::HeartbeatAck,
            _  => Opcode::Dispatch
        }
    }
}

impl From<Opcode> for u8 {
    fn from(op: Opcode) -> u8 {
        return op as u8;
    }
}

pub mod close_codes {
    pub const UNKNOWN_ERROR: (u16, &'static str) = (4000, "We're not sure what went wrong. Try reconnecting?");
    pub const UNKNOWN_OPCODE: (u16, &'static str) = (4001, "You sent an invalid Gateway opcode or an invalid payload for an opcode. Don't do that!");
    pub const DECODE_ERROR: (u16, &'static str) = (4002, "You sent an invalid payload to Discord. Don't do that!");
    pub const NOT_AUTHENTICATED: (u16, &'static str) = (4003, "You sent us a payload prior to identifying, or this session has been invalidated.");
    pub const AUTHENTICATION_FAILED: (u16, &'static str) = (4004, "The account token sent with your identify payload is incorrect.");
    pub const ALREADY_AUTHENTICATED: (u16, &'static str) = (4005, "You sent more than one identify payload. Don't do that!");
    pub const INVALID_SEQ: (u16, &'static str) = (4007, "The sequence sent when resuming the session was invalid. Reconnect and start a new session.");
    pub const RATE_LIMITED: (u16, &'static str) = (4008, "Woah nelly! You're sending payloads to us too quickly. Slow it down! You will be disconnected on receiving this.");
    pub const SESSION_TIMED_OUT: (u16, &'static str) = (4009, "Your session timed out. Reconnect and start a new one.");
    pub const INVALID_SHARD: (u16, &'static str) = (4010, "You sent us an invalid shard when identifying.");
    pub const SHARDING_REQUIRED: (u16, &'static str) = (4011, "The session would have handled too many guilds - you are required to shard your connection in order to connect.");
    pub const INVALID_API_VERSION: (u16, &'static str) = (4012, "You sent an invalid version for the gateway.");
    pub const INVALID_INTENTS: (u16, &'static str) = (4013, "You sent an invalid intent for a Gateway Intent. You may have incorrectly calculated the bitwise value.");
    pub const DISALLOWED_INTENTS: (u16, &'static str) = (4014, "You sent a disallowed intent for a Gateway Intent. You may have tried to specify an intent that you have not enabled or are not approved for.");
}

pub mod events {
    pub const GUILD_CREATE: &str = "GUILD_CREATE";
    pub const GUILD_DELETE: &str = "GUILD_DELETE";
    pub const GUILD_MEMBER_ADD: &str = "GUILD_MEMBER_ADD";
    pub const GUILD_MEMBER_REMOVE: &str = "GUILD_MEMBER_REMOVE";
    pub const READY: &str = "READY";
    pub const RESUMED: &str = "RESUMED";
    pub const PRESENCE_UPDATE: &str = "PRESENCE_UPDATE";
    pub const USER_SETTINGS_UPDATE: &str = "USER_SETTINGS_UPDATE";
}