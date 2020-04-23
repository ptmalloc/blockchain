use std::io::prelude::*;
use std::fs::File;
use crate::blockchain::*;
use utils::coder;
use crate::block::*;

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




//将区块序列化保存在文件中
pub fn Blockwrite(bc: &Blockchain) {
    let Blocks_length = bc.blocks.len();
    let mut x= 0;

    let bciter = bc.blocks.iter();
    for b in bciter{
        let filename = String::from("Database\\") + x.to_string().as_str();
        let mut file = File::create(filename).unwrap();
        x = x+1;
        let bx = coder::my_serialize(&b);
        file.write(&bx[..]).expect("write failed");
    }
    let filename = String::from("Database\\") + String::from("length").as_str();
    let mut file = File::create(filename).unwrap();
    let content = Blocks_length.to_string();
    file.write(content.as_bytes()).expect("write failed");
}


//从文件中读取区块的信息
pub fn Blockread(x: i32) {
    let filename = String::from("Database\\") + x.to_string().as_str();
    let mut f = File::open(filename).expect("write failed");
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer);
    let b: Block = coder::my_deserialize(&buffer);
    println!("{:#?}", b);
}