use std::cmp::Ordering;
use std::sync::atomic::{AtomicU16, Ordering as AtomicOrdering};
use std::time::{SystemTime, UNIX_EPOCH};

pub const MAXIMUM_WORKER_ID: u64 = 0b11111;
pub const MAXIMUM_PROCESS_ID: u64 = 0b11111;
pub const MAXIMUM_INCREMENT: u16 = 0b111111111111;

pub static DISCORD_SNOWFLAKE: Snowflake = Snowflake::new(1420070400000);
pub static TWITTER_SNOWFLAKE: Snowflake = Snowflake::new(1288834974657);

#[derive(Debug)]
pub struct Snowflake {
    epoch: u64,
    process_id: u64,
    worker_id: u64,
    increment: AtomicU16
}

#[derive(Default, Debug, Clone)]
pub struct GenerateOptions {
    pub increment: Option<u16>,
    pub timestamp: Option<u64>,
    pub worker_id: Option<u64>,
    pub process_id: Option<u64>
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeconstructedSnowflake {
    pub id: u64,
    pub timestamp: u64,
    pub worker_id: u64,
    pub process_id: u64,
    pub increment: u64,
    pub epoch: u64
}

impl Snowflake {
    pub const fn new(epoch: u64) -> Self {
        return Self {
            epoch,
            process_id: 1,
            worker_id: 0,
            increment: AtomicU16::new(0)
        };
    }

    pub fn epoch(&self) -> u64 {
        return self.epoch;
    }

    pub fn process_id(&self) -> u64 {
        return self.process_id;
    }

    pub fn set_process_id(&mut self, value: u64) {
        self.process_id = value & MAXIMUM_PROCESS_ID;
    }

    pub fn worker_id(&self) -> u64 {
        return self.worker_id;
    }

    pub fn set_worker_id(&mut self, value: u64) {
        self.worker_id = value & MAXIMUM_WORKER_ID;
    }

    pub fn generate(&self, options: Option<GenerateOptions>) -> u64 {
        let opts = options.unwrap_or_default();

        let timestamp = opts.timestamp.unwrap_or_else(|| {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Time went backwards")
                .as_millis() as u64
        });

        let worker_id = opts.worker_id.unwrap_or(self.worker_id) & MAXIMUM_WORKER_ID;
        let process_id = opts.process_id.unwrap_or(self.process_id) & MAXIMUM_PROCESS_ID;

        let increment = opts.increment.unwrap_or_else(|| {
            let current = self.increment.fetch_add(1, AtomicOrdering::Relaxed);
            return current & MAXIMUM_INCREMENT;
        }) as u64;

        return ((timestamp - self.epoch) << 22) | (worker_id << 17) | (process_id << 12) | increment;
    }

    pub fn deconstruct(&self, id: u64) -> DeconstructedSnowflake {
        return DeconstructedSnowflake {
            id,
            timestamp: (id >> 22) + self.epoch,
            worker_id: (id >> 17) & MAXIMUM_WORKER_ID,
            process_id: (id >> 12) & MAXIMUM_PROCESS_ID,
            increment: id & (MAXIMUM_INCREMENT as u64),
            epoch: self.epoch,
        };
    }

    pub fn decode(&self, id: u64) -> DeconstructedSnowflake {
        return self.deconstruct(id);
    }

    pub fn timestamp_from(&self, id: u64) -> u64 {
        return  (id >> 22) + self.epoch;
    }

    pub fn compare_strings(a: &str, b: &str) -> Ordering {
        if a.len() != b.len() {
            return a.len().cmp(&b.len());
        } else {
            return a.cmp(b);
        };
    }
}