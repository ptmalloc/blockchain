use std::net::UdpSocket;
use std::fs::File;
use std::io::{Read, Write};
use base64::*;
use std::intrinsics::transmute;
use std::{str, mem};
use crate::blockchain::*;
use utils::coder;
use crate::block::*;

//通信、广播机制

//同步
pub fn connect_update() {
    let socket = UdpSocket::bind("127.0.0.1:38384").expect("couldn't bind to address");
    let mut buf = [0u8; 65535];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    let mut recv_str = str::from_utf8(&buf).expect("Could not write buffer as string").to_string();
    println!("recv: {}", recv_str.len());

    let mut recv_str = mem::ManuallyDrop::new(recv_str);
    let content = unsafe{
        String::from_raw_parts(recv_str.as_mut_ptr(), number_of_bytes, number_of_bytes)
    };
    //let content = String::from(recv_str);
    //let content = String::from_raw_parts(recv_str.as_mut_ptr(), recv_str.len(), recv_str.len());
    println!("recv: {}", content.len());

    let buffer = base64::decode(content).expect("faild decode");
    let b: Block = coder::my_deserialize(&buffer);
    println!("{:#?}", b);
    //println!("recv: {}", number_of_bytes);

    //recv_str.len() = recv_str.capacity();
    //let test = "AQAAAEAAAAAAAAAAYTk4NWVmYzM2ZjQ3OTg3YTU4MjY0NmUyMGVjODA0ZDFiZGE5OWRhN2IyODNiYWU1MGNiYTM0ZjZjYTI3MzZhNkAAAAAAAAAAMThjNTVlODg3NThiNzliZDRhOTk0YjhlMzc1Njg1NDA3NWE1ZTI4YWUzOGYzNDJjMWUxYzYxMzY0YmFjM2JmORj1pF4AAAAAAQAAAAEAAAANAAAAAAAAAGEgLT4gYiA6IDVidGM=".to_string();
    // recv_str.capacity() = recv_str.len();
    //println!("recv: {}", recv_str.len());

    //let buffer = base64::decode(test).expect("faild decode");
    // let b: Block = coder::my_deserialize(&buffer);
    // println!("{:#?}", b);

    /*let filename = String::from("Database\\") + String::from("test").as_str();
    let mut file = File::create(filename).unwrap();
    file.write(&buffer[..]).expect("write failed");*/

}

//发布
pub fn connect_send_length(){
    let filename = String::from("Database\\") + "length";
    let mut f = File::open(filename).expect("write failed");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    println!("{}", content);

    //绑定本地一端口的socket
    let socket = UdpSocket::bind("0.0.0.0:38383")
        .expect("couldn't bind to address");
    let remote_ip = "127.0.0.1:38384";
    socket.send_to(&content.as_bytes()[..], remote_ip).unwrap();
}

pub fn connect_recv_length() -> u32{
    let socket = UdpSocket::bind("127.0.0.1:38384").expect("couldn't bind to address");
    let mut buf = [0u8; 65535];
    let (number_of_bytes, src_addr) = socket.recv_from(&mut buf).expect("Didn't receive data");
    let mut recv_str = str::from_utf8(&buf).expect("Could not write buffer as string").to_string();
    println!("{}", recv_str);

    let mut recv_str = mem::ManuallyDrop::new(recv_str);

    let content = unsafe{
        String::from_raw_parts(recv_str.as_mut_ptr(), number_of_bytes, number_of_bytes)
    };

    let rx = content.parse().unwrap();
    rx


    /*let filename = String::from("Database\\") + "length";
    let mut f = File::open(filename).expect("write failed");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    println!("{}", content);

    //绑定本地一端口的socket
    let socket = UdpSocket::bind("0.0.0.0:38383")
        .expect("couldn't bind to address");
    let remote_ip = "127.0.0.1:38384";
    socket.send_to(&content.as_bytes()[..], remote_ip).unwrap();*/
}

//同步数据
pub fn send_data(x: u32) {
    let filename = String::from("Database\\") + x.to_string().as_str();
    let mut f = File::open(filename).expect("write failed");
    let mut content = Vec::new();
    f.read_to_end(&mut content)
        .expect("something went wrong reading the file");

    let b64_content = base64::encode(content.clone());

    println!("{}", b64_content);

    //绑定本地一端口的socket
    let socket = UdpSocket::bind("0.0.0.0:38383")
        .expect("couldn't bind to address");
    let remote_ip = "127.0.0.1:38384";
    socket.send_to(b64_content.as_bytes(), remote_ip)
         .expect("couldn't send data");
}