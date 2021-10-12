pub use rpc_macro::rpc_service;
pub use bincode;

use std::collections::HashMap;
use std::io::{BufReader, BufWriter};
use std::net::TcpStream;

pub trait Handler {
    fn handle(&mut self, reader: &mut BufReader<&TcpStream>, writer: &mut BufWriter<&TcpStream>);
}

pub struct Server {
    handlers: HashMap<String, Box<dyn Handler>>,
}

impl Server {
    pub fn new() -> Self {
        Self {
            handlers: HashMap::new(),
        }
    }

    pub fn register_handler(&mut self, service_name: &str, service_impl: Box<dyn Handler>) {
        self.handlers.insert(String::from(service_name), service_impl);
    }

    pub fn handle(&mut self, mut reader: &mut BufReader<&TcpStream>, writer: &mut BufWriter<&TcpStream>) {
        loop {
            match bincode::deserialize_from::<_, String>(&mut reader) {
                Ok(service_name) => {
                    if let Some(handler) = self.handlers.get_mut(&service_name) {
                        handler.handle(reader, writer);
                    } else {
                        eprintln!("Unknown service: {}", service_name);
                    }
                }
                Err(error) => {
                    eprintln!("{}", error);
                    break
                }
            }
        }
    }
}
