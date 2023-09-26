use p2p_node_handshake::p2p::{btc::SUCCESS_MESSAGE, perform_handshake, P2PResult};
use std::env;

#[tokio::test]
async fn it_perform_bitcoin_handshake() {
    let nodes_addrs: Vec<String> = env::var("NODE_ADDRESSES")
        .unwrap_or("185.202.236.25:8333".to_string())
        .split_whitespace()
        .map(|s| s.to_string())
        .collect();
    let user_agent = env::var("USER_AGENT").unwrap_or("/Satoshi:25.0.0/".to_string());

    perform_handshake(nodes_addrs, user_agent)
        .await
        .unwrap()
        .iter()
        .for_each(assert_result);
}

fn assert_result(result: &P2PResult) {
    assert!(result.result().unwrap().eq(SUCCESS_MESSAGE));
}
