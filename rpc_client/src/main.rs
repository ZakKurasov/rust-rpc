use rpc_interface::*;

fn main() {
    let mut client = HelloServiceClient::new("127.0.0.1:3000");
    println!("[cli] Connected!");
    println!("[cli] client.test(\"zkr\") = `{}`", client.test(String::from("zkr")));
    println!("[cli] Done!");
}
