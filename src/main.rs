use p2p_node_handshake::p2p::{perform_bitcoin_handshake, P2PError};
use std::env;

#[tokio::main]
async fn main() -> Result<(), P2PError> {
    let bitcoin_node_address = env::var("NODE_ADDRESS").unwrap();
    let user_agent = env::var("USER_AGENT").unwrap();

    if let Err(err) =
        perform_bitcoin_handshake(bitcoin_node_address.to_string(), user_agent.to_string()).await
    {
        eprintln!("Bitcoin handshake failed: {}", err);
        return Ok(());
    }

    // You can now further communicate with the Bitcoin node

    Ok(())
}
