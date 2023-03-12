use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    difficulty: u16,    
    previous_hash: String,
    data: String,
    hash: String
}

impl Block {
    fn new(index: u64, difficulty: u16, previous_hash: String, data: String) -> Block {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let hash = Block::calculate_hash(&index, &timestamp, &difficulty, &previous_hash, &data);
        Block {
            index,
            timestamp,
            difficulty,
            previous_hash,
            data,
            hash,
        }
    }

    fn calculate_hash() -> String {
        "0x"
    }
}