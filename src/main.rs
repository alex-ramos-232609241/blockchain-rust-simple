use chrono::prelude::*; 
use sha2::{Sha256, Digest}; 

#[derive(Debug)]
struct Block {
    index: u64,
    timestamp: String,
    data: String,
    previous_hash: String,
    hash: String,
}

impl Block {
    fn new(index: u64, data: String, previous_hash: String) -> Self {
        let timestamp = Utc::now().to_rfc3339(); 
        let hash = Self::calculate_hash(index, &timestamp, &data, &previous_hash); 

        Block {
            index,
            timestamp,
            data,
            previous_hash,
            hash,
        }
    }

    fn calculate_hash(index: u64, timestamp: &str, data: &str, previous_hash: &str) -> String {
        let mut hash = Sha256::new();
        hash.update(index.to_string());
        hash.update(timestamp);
        hash.update(data);
        hash.update(previous_hash);
        format!("{:x}", hash.finalize()) 
    }
}

struct Blockchain {
    chain: Vec<Block>, 
}

impl Blockchain {
    fn new() -> Self {
        let genesis_block = Block::new(0, "Genesis Block".to_string(), "0".to_string());
        Blockchain {
            chain: vec![genesis_block],
        }
    }

    fn add_block(&mut self, data: String) {
        let previous_block = self.chain.last().unwrap(); 
        let new_block = Block::new(
            previous_block.index + 1,
            data,
            previous_block.hash.clone(),
        );
        self.chain.push(new_block); 
    }
}

fn main() {
    let mut blockchain = Blockchain::new();
    blockchain.add_block("First block after Genesis".to_string());
    blockchain.add_block("Second block after Genesis".to_string());

    for block in &blockchain.chain {
        println!(
            "Index: {}, Timestamp: {}, Data: {}, Previous Hash: {}, Hash: {}",
            block.index, block.timestamp, block.data, block.previous_hash, block.hash
        );
    }
}

