use p2p_node_handshake::p2p::perform_handshake;
use std::env;

#[tokio::main]
async fn main() {
    let nodes_addrs: Vec<String> = env::var("NODE_ADDRESSES")
        .unwrap_or("127.0.0.1:8332".to_string())
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let user_agent = env::var("USER_AGENT").unwrap_or("/Satoshi:25.0.0/".to_string());

    match perform_handshake(nodes_addrs, user_agent).await {
        Ok(result) => result.iter().for_each(|res| println!("{}", res)),
        Err(err) => println!("{}", err),
    }
}
