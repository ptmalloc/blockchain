use crate::block;
use std::hash::Hash;
use crate::block::Block;

pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain{
    pub fn add_block(&mut self, data: String){
        let pre_block = &self.blocks[self.blocks.len() -1];
        let new_block = block::Block::new_block(data, pre_block.hash.clone());
        self.blocks.push(new_block);

    }

    pub fn new_genesis_block() -> block::Block{
        block::Block::new_block("This is genesis block".to_string(), "".to_string())
    }

    pub fn new_blockchain() -> Blockchain{
        Blockchain{
            blocks: vec![Blockchain::new_genesis_block()],
        }
    }
}