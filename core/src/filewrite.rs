use std::io::prelude::*;
use std::fs::File;
use crate::blockchain::Blockchain;
use utils::coder;

pub fn Filewrite(bc: &Blockchain) {
    //let Blockchain_length = bc.blocks.len().to_string();
    let mut file = File::create("text.txt").unwrap();
    // file.write(Blockchain_length.as_bytes()).expect("write failed");
    // file.write(b"\n").expect("write failed");


    /*let blocks = bc.blocks.iter();
    for b in blocks{
        let block = coder::my_serialize(&b);
        file.write(&block[..]).expect("write failed");
        file.write(b"\n").expect("write failed");

    }*/

    /*
    1. 直接将Blockchain序列化直接存在存到
    2. 存在的问题 有如果Blockchain太大，可能对对内存的要求过高
    3. 后续可能改为block一块一块的存入
    */

    let blocks = coder::my_serialize(&bc);
    file.write(&blocks[..]).expect("write failed");

}

//读取文件中的反序列化的blockchain
pub fn Fileread() -> Blockchain{
    let mut f = File::open("text.txt").expect("write failed");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);


    let bc: Blockchain = coder::my_deserialize(&buffer);
    println!("{}", bc.blocks.len());
    bc
}