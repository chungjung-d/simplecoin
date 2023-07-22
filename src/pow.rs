use crate::block::Block;
use sha2::{Digest, Sha256};

use num_bigint::BigUint;

pub struct ProofOfWork<'a> {
    block: &'a Block,
    target: BigUint,
    target_bits: u16,
}

impl<'a> ProofOfWork<'a> {
    pub fn new(block: &'a Block, target_bits: u16) -> ProofOfWork<'a> {
        let target = BigUint::from(1u32);
        let target = target << (256 - target_bits);
        ProofOfWork {
            block,
            target,
            target_bits,
        }
    }

    pub fn run(&self) -> (u32, Vec<u8>) {
        let mut hash = vec![0; 32];
        let mut nonce = 0u32;

        println!(
            "Mining the block containing {}",
            String::from_utf8(self.block.data().to_vec()).unwrap()
        );

        while nonce < u32::MAX {
            let data = self.prepare_data(self.target_bits, nonce);
            let mut hasher = Sha256::new();
            hasher.update(&data);
            hash = hasher.finalize().to_vec();

            // println!("{:?}", hex::encode(&hash));

            let hash_int = BigUint::from_bytes_be(&hash);
            if hash_int < self.target {
                break;
            } else {
                nonce += 1;
            }
        }

        (nonce, hash)
    }

    pub fn validate(&self) -> bool {
        let data = self.prepare_data(self.target_bits, self.block.nonce());
        let mut hasher = Sha256::new();
        hasher.update(&data);
        let hash = hasher.finalize().to_vec();
        let hash_int = BigUint::from_bytes_be(&hash);
        hash_int < self.target
    }

    fn prepare_data(&self, target_bits: u16, nonce: u32) -> Vec<u8> {
        let mut data = vec![];

        data.extend_from_slice(self.block.prev_block_hash());
        data.extend_from_slice(self.block.data());
        data.extend_from_slice(&format!("{:x}", self.block.timestamp()).as_bytes());
        data.extend_from_slice(&format!("{:x}", target_bits).as_bytes());
        data.extend_from_slice(&format!("{:x}", nonce).as_bytes());

        data
    }
}
