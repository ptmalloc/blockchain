use core::blockchain;

fn main() {
    let mut bc = blockchain::Blockchain::new_blockchain();

    for b in bc.blocks{
        println!("++++++++++++++++++++++");
        println!("{:#?}", b);
        println!("");
    }

    println!("Hello, world!");
}
