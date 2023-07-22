use crate::pow::ProofOfWork;
use chrono::prelude::*;

pub struct Block {
    timestamp: i64,
    data: Vec<u8>,
    prev_block_hash: Vec<u8>,
    hash: Vec<u8>,
    nonce: u32,
}

impl Block {
    pub fn new(data: &str, prev_block_hash: &[u8]) -> Block {
        let mut block = Block {
            timestamp: Utc::now().timestamp(),
            data: data.as_bytes().to_vec(),
            prev_block_hash: prev_block_hash.to_vec(),
            hash: vec![],
            nonce: 0,
        };

        let pow = ProofOfWork::new(&block, 8);
        let (nonce, hash) = pow.run();

        block.hash = hash;
        block.nonce = nonce;
        block
    }

    pub fn hash(&self) -> &[u8] {
        &self.hash
    }

    pub fn timestamp(&self) -> i64 {
        self.timestamp
    }

    pub fn prev_block_hash(&self) -> &[u8] {
        &self.prev_block_hash
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn nonce(&self) -> u32 {
        self.nonce
    }
}
