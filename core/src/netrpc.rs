use std::net::UdpSocket;
use crate::blockchain::Blockchain;

//通信、广播机制

pub fn find_long(blockchain: Blockchain) {
    let chain_lenght = blockchain::blocks.len();
    let socket = UdpSocket::bind("127.0.0.1:3400").expect("couldn't bind to address");
}
