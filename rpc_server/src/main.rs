use std::io::{BufReader, BufWriter};
use std::net::TcpListener;

use rpc_interface::*;
use rpc_lib::Server;

struct HelloServiceImpl;
impl HelloService for HelloServiceImpl {
    fn test(&mut self, test: String) -> String {
        test
    }
}

fn main() {
    let listener = TcpListener::bind("0.0.0.0:3000").expect("Unable to bind to 0.0.0.0:3000");
    println!("[srv] Listening on 0.0.0.0:3000");
    let mut server = Server::new();
    server.register_handler("HelloService", Box::new(HelloServiceWrapper::new(HelloServiceImpl{})));
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        println!("[srv] Incoming connection!");
        let mut reader = BufReader::new(&stream);
        let mut writer = BufWriter::new(&stream);
        server.handle(&mut reader, &mut writer);
    }
}
