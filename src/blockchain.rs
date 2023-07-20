use crate::block::Block;

pub struct BlockChain {
    blocks: Vec<Block>,
}

impl BlockChain {
    pub fn new() -> BlockChain {
        let mut blocks = Vec::new();
        blocks.push(BlockChain::new_genesis_block());
        BlockChain { blocks: blocks }
    }

    pub fn add_block(&mut self, data: &str) {
        let prev_block_hash = &self.blocks.last().unwrap().hash();
        let new_block = Block::new(data, &prev_block_hash);
        self.blocks.push(new_block);
    }

    pub fn blocks(&self) -> &Vec<Block> {
        &self.blocks
    }

    fn new_genesis_block() -> Block {
        let block = Block::new("Genesis Block", &vec![]);
        block
    }
}
