use std::env;

use p2p_node_handshake::p2p::perform_bitcoin_handshake;

#[tokio::test]
async fn it_perform_bitcoin_handshake() {
    let node_address = env::var("NODE_ADDRESS").unwrap();

    let result = perform_bitcoin_handshake(node_address, "/Satoshi:23.0.0/".to_string())
        .await
        .unwrap();

    assert!(result.eq("Bitcoin handshake successful!"));
}
