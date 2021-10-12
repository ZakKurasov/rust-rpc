extern crate rpc_lib;

use rpc_lib::rpc_service;

#[rpc_service]
pub trait HelloService {
    fn test(&mut self, test: String) -> String;
}
