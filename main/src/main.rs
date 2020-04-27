use core::*;
use std::thread;
use std::time::Duration;


fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    bc.add_block("a -> b : 5btc".to_string());
    //thread::sleep(Duration::from_secs(5));
    bc.add_block("c -> d : 5btc".to_string());
    bc.add_block("c -> d : 5btc".to_string());




    filewrite::Blockwrite(&bc);

    let length = netrpc::get_local_length();
    println!("{}", length);

    //测试读取文件中第一个区块
    let temp = 0;
    filewrite::Blockread(temp);

    let temp = 1;
    filewrite::Blockread(temp);

    let temp = 2;
    filewrite::Blockread(temp);

    let temp = 3;
    filewrite::Blockread(temp);



    // println!("Hello, world!");

    /*for b in bc.blocks{
    println!("++++++++++++++++++++++");
    println!("{:#?}", b);
    println!("");
    }*/

    /*filewrite::Filewrite(&bc);

    let bc1 = filewrite::Fileread();
    for b in bc1.blocks {
        println!("++++++++++++++++++++++");
        println!("{:#?}", b);
        println!(" ");
    }*/




    //netrpc::connect_send_length();
    //netrpc::send_data(1);
    // let x= netrpc::connect_recv_length();
    // println!("{}", x);
    //netrpc::connect_recv_block();
}
