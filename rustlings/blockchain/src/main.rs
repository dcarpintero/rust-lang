// https://dev.to/ecj222/how-to-build-a-blockchain-from-scratch-in-rust-46
// https://hackernoon.com/rusty-chains-a-basic-blockchain-implementation-written-in-pure-rust-gk2m3uri
// https://blog.logrocket.com/how-to-build-a-blockchain-in-rust/


use std::fmt;
use std::time::{Duration};
use chrono::Utc;
use sha2::{Sha256, Digest};


#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: u64,
    previous_hash: String,
    data: String,
    hash: String,
}

impl Block {
    fn new(index: u64, previous_hash: String, data: String) -> Block {
        let timestamp = Utc::now().timestamp_millis() as u64;
        let hash = Block::calculate_hash(&index, &timestamp, &previous_hash, &data);
        Block {
            index,
            timestamp,
            previous_hash,
            data,
            hash,
        }
    }

    fn calculate_hash(index: &u64, timestamp: &u64, previous_hash: &String, data: &String) -> String {
        let mut hasher = Sha256::new();
        hasher.update(index.to_string());
        hasher.update(timestamp.to_string());
        hasher.update(data);
        hasher.update(previous_hash);
        let result = hasher.finalize();
        format!("{:x}", result)
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Block #{} [Hash: {}, Previous Hash: {}, Timestamp: {}, Data: {}]",
               self.index, self.hash, self.previous_hash, self.timestamp, self.data)
    }
}

fn main() {
    let mut blockchain = vec![Block::new(0, String::from("0"), String::from("Genesis Block"))];
    let mut previous_hash = blockchain[0].hash.clone();

    for i in 1..5 {
        std::thread::sleep(Duration::from_secs(1));
        let block = Block::new(i, previous_hash, format!("This is Block#{}", i.to_string()));
        previous_hash = block.hash.clone();
        blockchain.push(block);
    }

    for block in blockchain {
        println!("{}", block);
    }
}