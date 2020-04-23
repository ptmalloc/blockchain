use std::net::UdpSocket;
use std::fs::File;
use std::io::Read;


//通信、广播机制

//同步
pub fn connect_update() {

}

//发布
pub fn connect_send(){
    let filename = String::from("Database\\") + "length";
    let mut f = File::open(filename).expect("write failed");
    let mut content = String::new();
    f.read_to_string(&mut content)
        .expect("something went wrong reading the file");
    //println!("{}", content);

    let socket = UdpSocket::bind("0.0.0.0:38383").unwrap();
    let buf = [1u8; 60000];
    socket.send_to(&buf[0..count], string).unwrap();
}