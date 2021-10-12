use std::net::TcpStream;
use rpc_interface::*;

fn main() {
    let stream = TcpStream::connect("127.0.0.1:3000").expect("Unable to connect");
    let mut client = HelloServiceClient::new(Box::new(stream));
    println!("[cli] Connected!");
    println!("[cli] client.test(\"zkr\") = `{}`", client.test(String::from("zkr")));
    println!("[cli] Done!");
}
