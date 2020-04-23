use crate::block;
use crate::block::Block;
use utils::coder;
use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize, PartialEq, Eq,Debug)]
pub struct Blockchain {
    pub blocks: Vec<block::Block>,
}

impl Blockchain{
    pub fn add_block(&mut self, data: String){

        //对前一个区块头做hash
        let pre_block = &self.blocks[self.blocks.len() -1];
        let pre_hash_se = coder::my_serialize(&pre_block.header);
        let pre_hash = coder::get_hash(&pre_hash_se[..]);

        let new_block = block::Block::new_block(data, pre_hash.clone());
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