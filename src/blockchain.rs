use crate::block::Block;
use rocksdb::{Options, WriteBatch, DB};
use std::sync::Arc;

pub struct BlockChain {
    tip: Vec<u8>,
    database: Arc<DB>,
}

pub struct BlockchainIterator {
    current_hash: Vec<u8>,
    database: Arc<DB>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let tip: Vec<u8>;

        let path = "./blockchain.db";
        let mut options = Options::default();
        options.create_if_missing(true);
        let database = Arc::new(DB::open(&options, path).unwrap());

        let last_hash = database.get(b"l").unwrap();

        if last_hash.is_none() {
            let genesis_block = BlockChain::new_genesis_block();
            let genesis_hash = genesis_block.hash().to_vec();
            database
                .put(&genesis_hash, &genesis_block.serialize())
                .unwrap();
            database.put(b"l", &genesis_hash).unwrap();
            tip = genesis_hash;
        } else {
            tip = last_hash.unwrap().to_vec();
        }

        BlockChain {
            tip: tip,
            database: database,
        }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block_hash: Vec<u8> = self.database.get(b"l").unwrap().unwrap().to_vec();
        let new_block = Block::new(data, &prev_block_hash);

        let mut batch = WriteBatch::default();
        batch.put(&new_block.hash(), &new_block.serialize());
        batch.put(b"l", &new_block.hash());
        self.database.write(batch).unwrap();
        self.tip = new_block.hash().to_vec();
    }

    pub fn iterator(&self) -> BlockchainIterator {
        BlockchainIterator::new(Arc::clone(&self.database), self.tip.clone())
    }

    fn new_genesis_block() -> Block {
        let block = Block::new("Genesis Block", &vec![]);
        block
    }
}

impl BlockchainIterator {
    pub fn new(database: Arc<DB>, current_hash: Vec<u8>) -> BlockchainIterator {
        BlockchainIterator {
            current_hash: current_hash,
            database: database,
        }
    }

    pub fn next(&mut self) -> Option<Block> {
        let encoded_block = self.database.get(&self.current_hash).unwrap();
        if encoded_block.is_none() {
            return None;
        }

        let block = Block::deserialize(&encoded_block.unwrap());
        self.current_hash = block.prev_block_hash().to_vec();
        Some(block)
    }
}
