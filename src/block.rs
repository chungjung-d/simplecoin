use chrono::prelude::*;
use sha2::{Digest, Sha256};

pub struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data: data.as_bytes().to_vec(),
            prev_block_hash: prev_block_hash.to_vec(),
            hash: vec![],
        };

        block.set_hash();
        block
    }

    pub fn set_hash(&mut self) {
        let timestamp = self.timestamp.to_string();
        let headers = [
            self.prev_block_hash.clone(),
            self.data.clone(),
            timestamp.as_bytes().to_vec(),
        ]
        .concat();

        self.hash = Sha256::digest(&headers).to_vec();
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn prev_block_hash(&self) -> &[u8] {
        &self.prev_block_hash
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
