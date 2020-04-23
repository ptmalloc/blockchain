use chrono::prelude::*;
use utils::coder;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq,Debug)]
pub struct BlockHeader {
    pub version: u32,
    pub prevBlockHash: String,
    pub merkleRoot: String,
    pub time: i64,
    pub difficultyTarget: u32,
    pub nonce: u32,
}

#[derive(Serialize, Deserialize, PartialEq, Eq,Debug)]
pub struct Block {
    pub header: BlockHeader,
    pub data: String, //交易数据
}

impl Block {
    fn set_hash(&mut self){
        let header = coder::my_serialize(&(self.data));
        self.header.merkleRoot = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, prevBlockHash: String) -> Block {
        let version = 1;
        let merkleRoot = String::new();
        let time = Utc::now().timestamp();
        let difficultyTarget = 1;
        let nonce = 1;
        let mut block = Block{
            header: BlockHeader{
                version,
                prevBlockHash, //交易数据的hash
                merkleRoot,
                time,
                difficultyTarget,
                nonce,
            },
            data,
        };
        block.set_hash();
        block
    }
}
