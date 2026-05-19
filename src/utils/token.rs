use base64::{Engine as _, engine::general_purpose::URL_SAFE_NO_PAD};
use hmac::{Hmac, Mac};
use sha2::Sha256;
use std::time::{SystemTime, UNIX_EPOCH};

type HmacSha256 = Hmac<Sha256>;
const DISCORD_EPOCH: u64 = 1420070400000;

pub fn sign_token(user_id: &str, secret: &str) -> String {
    let payload64 = URL_SAFE_NO_PAD.encode(user_id);

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;

    let seconds_since_epoch = ((now - DISCORD_EPOCH) / 1000) as u32;
    let ts_bytes = seconds_since_epoch.to_be_bytes();
    let timestamp64 = URL_SAFE_NO_PAD.encode(ts_bytes);

    let msg = format!("{}.{}", payload64, timestamp64);

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    mac.update(msg.as_bytes());
    let signature_bytes = mac.finalize().into_bytes();
    let signature64 = URL_SAFE_NO_PAD.encode(signature_bytes);

    return format!("{}.{}", msg, signature64);
}

pub fn verify_token(token: &str, secret: &str) -> Result<String, &'static str> {
    let token = token.trim();
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err("token malformed");
    }

    let payload64 = parts[0];
    let timestamp64 = parts[1];
    let signature64 = parts[2];

    if timestamp64.len() != 6 {
        return Err("invalid timestamp length");
    }

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).expect("HMAC can take key of any size");
    let msg = format!("{}.{}", payload64, timestamp64);
    mac.update(msg.as_bytes());

    let sig_bytes = URL_SAFE_NO_PAD.decode(signature64).map_err(|_| "invalid signature encoding")?;

    if mac.verify_slice(&sig_bytes).is_err() {
        return Err("invalid signature");
    }

    let user_id_bytes = URL_SAFE_NO_PAD.decode(payload64).map_err(|_| "token payload decode error")?;
    let user_id = String::from_utf8(user_id_bytes).map_err(|_| "token payload is not utf8")?;

    return Ok(user_id);
}