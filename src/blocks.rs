use std::time::{SystemTime, UNIX_EPOCH};

// use crate::get_timestamp;
use serde::Serialize;
use uuid::Uuid;

use crate::{get_timestamp, hashable::Hashable};

#[derive(Clone, Debug, Serialize)]
pub struct AppBlock {
    pub id: Uuid,
    pub timestamp: i64,
    pub hash: String,
    pub prev_hash: String,
    pub nonce: u64,
    pub payload: Vec<AppPayload>,
}

#[derive(Clone, Debug, Serialize)]
pub struct AppPayload {
    pub user_id: i64,
    pub data: AppDataEvent,
}

#[derive(Clone, Debug, Serialize)]
pub enum AppDataEvent {
    Create(AppCreateEvent),
    Change(AppChangeEvent),
}

#[derive(Clone, Debug, Serialize)]
pub struct AppCreateEvent {
    pub event_type: String,
    pub timestamp: i64,
    pub data: AppCreateDataPayload,
}

#[derive(Clone, Debug, Serialize)]
pub struct AppChangeEvent {
    pub event_type: AppEventType,
    pub timestamp: i64,
    pub old_data: AppChangeDataPayload,
    pub new_data: AppChangeDataPayload,
}

#[derive(Clone, Debug, Serialize)]
pub struct AppCreateDataPayload {
    pub hash: String,
    pub app_name: String,
    pub email: String,
    pub password: String,
    pub notes: String,
}

#[derive(Clone, Debug, Serialize)]
pub struct AppChangeDataPayload {
    pub data_hash: String,
    pub data: String,
}

#[derive(Clone, Debug, Serialize)]
pub enum AppEventType {
    AppNameChange,
    UsernameChange,
}

impl AppBlock {
    pub fn new(prev_hash: String, payload: Vec<AppPayload>) -> Self {
        let timestamp = get_timestamp();
        let nonce = Self::generate_nonce();

        let hash = Self::generate_hash(timestamp, &prev_hash, nonce);

        AppBlock {
            id: Uuid::new_v4(),
            timestamp,
            nonce,
            hash,
            prev_hash,
            payload,
        }
    }

    fn generate_nonce() -> u64 {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let nonce = start_time;
        nonce
    }

    pub fn mine_block(&mut self, prev_block: &AppBlock, difficulty: u32) {
        let start_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        if self.prev_hash != prev_block.hash {
            panic!("Previous block's hash is incorrect");
        }

        self.nonce = Self::generate_nonce();

        while !self.is_valid_block(difficulty) {
            self.nonce += 1;
            self.hash = Self::generate_hash(self.timestamp, &self.prev_hash, self.nonce);
            // Use the trait method
        }

        let mining_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs()
            - start_time;

        println!("Block mined in {} seconds: {}", mining_time, self.hash);
    }

    fn is_valid_block(&self, difficulty: u32) -> bool {
        let target_prefix = "0".repeat(difficulty as usize);
        self.hash.starts_with(&target_prefix)
    }
}

impl Hashable for AppBlock {
    fn bytes(&self, timestamp: i64, prev_hash: &str, nonce: u64) -> Vec<u8> {
        let mut bytes = vec![];

        bytes.extend(self.id.as_bytes());
        bytes.extend(&timestamp.to_be_bytes());
        bytes.extend(self.hash.as_bytes());
        bytes.extend(prev_hash.as_bytes());
        bytes.extend(&nonce.to_be_bytes());

        for payload in &self.payload {
            let json_str = serde_json::to_string(payload).unwrap();
            bytes.extend(json_str.as_bytes());
        }

        bytes
    }
}
