pub use rpc_macro::rpc_service;
pub use bincode;

use std::collections::HashMap;
use std::io::{Read, Write};

pub trait Stream: Read + Write {}

impl<T: Read + Write> Stream for T {}

pub trait Handler {
    fn handle(&mut self, reader: &mut dyn Read, writer: &mut dyn Write);
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

    pub fn handle(&mut self, mut reader: &mut dyn Read, writer: &mut dyn Write) {
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
                    eprintln!("Error: {}", error);
                    break
                }
            }
        }
    }
}
