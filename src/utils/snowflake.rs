use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::atomic::{AtomicUsize, Ordering};

const EPOCH: u64 = 1420070400000;
static INCREMENT: AtomicUsize = AtomicUsize::new(0);

pub fn generate() -> String {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_millis() as u64;
    
    let timestamp = now - EPOCH;
    let inc = INCREMENT.fetch_add(1, Ordering::SeqCst) % 4096;

    let id = (timestamp << 22) | (0 << 17) | (0 << 12) | (inc as u64);
    id.to_string()
}