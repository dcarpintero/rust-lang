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
        let timestamp = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
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
}