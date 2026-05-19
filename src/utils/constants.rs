use serde::{Deserialize, Serialize};
use bitflags::bitflags;

pub const GATEWAY_HEARTBEAT_INTERVAL: u32 = 41250;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(from = "u8", into = "u8")]
pub enum GatewayOpcode {
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
    HeartbeatAck = 11,
}

impl From<u8> for GatewayOpcode {
    fn from(op: u8) -> Self {
        match op {
            1 => GatewayOpcode::Heartbeat,
            2 => GatewayOpcode::Identify,
            3 => GatewayOpcode::PresenceUpdate,
            4 => GatewayOpcode::VoiceStateUpdate,
            6 => GatewayOpcode::Resume,
            7 => GatewayOpcode::Reconnect,
            8 => GatewayOpcode::RequestGuildMembers,
            9 => GatewayOpcode::InvalidSession,
            10 => GatewayOpcode::Hello,
            11 => GatewayOpcode::HeartbeatAck,
            _ => GatewayOpcode::Dispatch,
        }
    }
}

impl From<GatewayOpcode> for u8 {
    fn from(op: GatewayOpcode) -> u8 {
        op as u8
    }
}

pub struct GatewayCloseEventCode;

impl GatewayCloseEventCode {
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

pub const GATEWAY_EVENTS_GUILD_CREATE: &str = "GUILD_CREATE";
pub const GATEWAY_EVENTS_GUILD_DELETE: &str = "GUILD_DELETE";
pub const GATEWAY_EVENTS_GUILD_MEMBER_ADD: &str = "GUILD_MEMBER_ADD";
pub const GATEWAY_EVENTS_GUILD_MEMBER_REMOVE: &str = "GUILD_MEMBER_REMOVE";
pub const GATEWAY_EVENTS_READY: &str = "READY";
pub const GATEWAY_EVENTS_RESUMED: &str = "RESUMED";
pub const GATEWAY_EVENTS_PRESENCE_UPDATE: &str = "PRESENCE_UPDATE";
pub const GATEWAY_EVENTS_USER_SETTINGS_UPDATE: &str = "USER_SETTINGS_UPDATE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChannelType {
    GuildText = 0,
    GuildVoice = 2,
    GuildCategory = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MessageType {
    Default = 0,
    UserJoin = 7,
    Reply = 19,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InviteType {
    Guild = 0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OverwriteType {
    Role = 0,
    Member = 1,
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct UserFlags: u32 {
        const STAFF = 1 << 0;
        const PARTNERED = 1 << 1;
        const VERIFIED_BOT = 1 << 16;
        const VERIFIED_DEVELOPER = 1 << 17;
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct PermissionFlags: u64 {
        const CREATE_INSTANT_INVITE = 1 << 0;
        const KICK_MEMBERS = 1 << 1;
        const BAN_MEMBERS = 1 << 2;
        const ADMINISTRATOR = 1 << 3;
        const MANAGE_CHANNELS = 1 << 4;
        const MANAGE_GUILD = 1 << 5;
        const ADD_REACTIONS = 1 << 6;
        const VIEW_AUDIT_LOG = 1 << 7;
        const PRIORITY_SPEAKER = 1 << 8;
        const STREAM = 1 << 9;
        const VIEW_CHANNEL = 1 << 10;
        const SEND_MESSAGES = 1 << 11;
        const SEND_TTS_MESSAGES = 1 << 12;
        const MANAGE_MESSAGES = 1 << 13;
        const EMBED_LINKS = 1 << 14;
        const ATTACH_FILES = 1 << 15;
        const READ_MESSAGE_HISTORY = 1 << 16;
        const MENTION_EVERYONE = 1 << 17;
        const USE_EXTERNAL_EMOJIS = 1 << 18;
        const VIEW_GUILD_INSIGHTS = 1 << 19;
        const CONNECT = 1 << 20;
        const SPEAK = 1 << 21;
        const MUTE_MEMBERS = 1 << 22;
        const DEAFEN_MEMBERS = 1 << 23;
        const MOVE_MEMBERS = 1 << 24;
        const USE_VAD = 1 << 25;
        const CHANGE_NICKNAME = 1 << 26;
        const MANAGE_NICKNAMES = 1 << 27;
        const MANAGE_ROLES = 1 << 28;
        const MANAGE_WEBHOOKS = 1 << 29;
        const MANAGE_GUILD_EXPRESSIONS = 1 << 30;
        const USE_APPLICATION_COMMANDS = 1 << 31;
        const REQUEST_TO_SPEAK = 1 << 32;
        const MANAGE_EVENTS = 1 << 33;
        const MANAGE_THREADS = 1 << 34;
        const CREATE_PUBLIC_THREADS = 1 << 35;
        const CREATE_PRIVATE_THREADS = 1 << 36;
        const USE_EXTERNAL_STICKERS = 1 << 37;
        const SEND_MESSAGES_IN_THREADS = 1 << 38;
        const USE_EMBEDDED_ACTIVITIES = 1 << 39;
        const MODERATE_MEMBERS = 1 << 40;
        const VIEW_CREATOR_MONETIZATION_ANALYTICS = 1 << 41;
        const USE_SOUNDBOARD = 1 << 42;
        const CREATE_GUILD_EXPRESSIONS = 1 << 43;
        const CREATE_EVENTS = 1 << 44;
        const USE_EXTERNAL_SOUNDS = 1 << 45;
        const SEND_VOICE_MESSAGES = 1 << 46;
        const SEND_POLLS = 1 << 49;
        const USE_EXTERNAL_APPS = 1 << 50;
    }
}