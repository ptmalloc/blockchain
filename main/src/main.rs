use core::*;
use std::thread;
use std::time::Duration;


fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    bc.add_block("a -> b : 5btc".to_string());
    //thread::sleep(Duration::from_secs(5));
    bc.add_block("c -> d : 5btc".to_string());

    filewrite::Filewrite(&bc);
    let bc1 = filewrite::Fileread();
    for b in bc1.blocks {
        println!("++++++++++++++++++++++");
        println!("{:#?}", b);
        println!(" ");
    }
    /*for b in bc.blocks{
        println!("++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }*/

    println!("Hello, world!");
}
