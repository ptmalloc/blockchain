use chrono::prelude::*;
use utils::coder;

pub struct BlockHeader {
    pub time: i64,
    pub tx_hash: String, //交易数据的hash
    pub pre_hash: String,
}


pub struct Block {
    pub header: BlockHeader,
    pub hash: String,
    pub data: String, //交易数据
}

impl Block {
    fn set_hash(&mut self){

        let header = coder::my_serialize(&(self.header));
        self.hash = coder::get_hash(&header[..]);
    }

    pub fn new_block(data: String, pre_hash: String) -> Block{
        let transactions = coder::my_serialize(&data);
        let tx_hash = coder::get_hash(&transactions[..]);

        let time = Utc::now().timestamp();
        let mut block = Block{
            header: BlockHeader{
                time,
                tx_hash, //交易数据的hash
                pre_hash,
            },
            hash: "".to_string(),
            data,
        };

    }
}
