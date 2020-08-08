use core::*;
use std::thread;
use std::time::Duration;
use core::netrpc::{server_info, client_get_info, get_local_length, server_recv_block, client_send_block};


fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    //区块生成测试
    // bc.add_block("a -> 1 : 5btc".to_string());
    // //thread::sleep(Duration::from_secs(5));
    // bc.add_block("c -> 2 : 5btc".to_string());
    // bc.add_block("c -> 3 : 5btc".to_string());
    // bc.add_block("e -> 4 : 5btc".to_string());
    // bc.add_block("e -> 5 : 5btc".to_string());
    //
    //
    //
    //区块链写入文件测试
    // filewrite::Blockwrite(&bc);

    let length = netrpc::get_local_length();
    println!("{}", length);

    //测试读取文件中第一个区块
    let temp = 0;
    filewrite::Blockread(temp);

    let temp = 1;
    filewrite::Blockread(temp);

    let temp = 2;
    filewrite::Blockread(temp);

    let temp = 5;
    filewrite::Blockread(temp);



    //Server测试
    // server_info();
    // server_recv_block();

    //Client测试
    // println!("client: {}", get_local_length());
    // let remote_len = client_get_info();
    // let local_len = get_local_length();
    // let mut num = local_len - remote_len ;
    // while num > 0 {
    //     client_send_block(local_len - num + 1);
    //     num = num - 1;
    // }





    /*let a = client_get_info();
    println!("{}", a);

    if a < get_local_length() {

    }*/

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
