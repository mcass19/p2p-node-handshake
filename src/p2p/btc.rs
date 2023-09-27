use super::P2PError;
use bitcoin::{
    consensus::{deserialize_partial, encode::serialize},
    network::{
        address::Address,
        constants::{Magic, ServiceFlags},
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
    },
};
use bytes::{Buf, BytesMut};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

pub const SUCCESS_MESSAGE: &str = "Bitcoin handshake successful!";

pub async fn perform_handshake(
    bitcoin_node_address: String,
    user_agent: String,
) -> Result<String, P2PError> {
    let mut socket = TcpStream::connect(&bitcoin_node_address).await?;
    println!("Connected to Bitcoin node at {:?}", bitcoin_node_address);

    // Trigger the handshake by sending the first version message
    let version_message = create_version_message(bitcoin_node_address, user_agent);
    send_message(&mut socket, &version_message).await?;

    // Continuously listening for messages, until a verack is received
    loop {
        let received_message = receive_message(&mut socket).await?;

        match received_message.payload {
            NetworkMessage::Version(_version_msg) => {
                // println!("Received Version message: {:?}", version_msg);
                let verack_message = create_verack_message();
                send_message(&mut socket, &verack_message).await?;
            }
            NetworkMessage::Ping(nonce) => {
                // println!("Received Ping message - nonce: {:?}", nonce);
                let pong_message = create_pong_message(nonce);
                send_message(&mut socket, &pong_message).await?;
            }
            NetworkMessage::Verack => {
                // println!("Received Verack message. {}", SUCCESS_MESSAGE);
                return Ok(SUCCESS_MESSAGE.to_string());
            }
            _other_message => {
                // println!("Unexpected message type during handshake: {:?}", other_message);
                continue;
            }
        }
    }
}

// ---
// Send and receive
async fn send_message(socket: &mut TcpStream, message: &RawNetworkMessage) -> Result<(), P2PError> {
    let serialized_message = serialize(message);
    socket.write_all(serialized_message.as_slice()).await?;

    Ok(())
}

async fn receive_message(socket: &mut TcpStream) -> Result<RawNetworkMessage, P2PError> {
    let mut buffer = BytesMut::with_capacity(1024);

    loop {
        socket.read_buf(&mut buffer).await?;

        if let Ok((message, count)) = deserialize_partial::<RawNetworkMessage>(&buffer) {
            buffer.advance(count);

            return Ok(message);
        }
    }
}

// ---
// Version message
fn create_version_message(dest_socket: String, user_agent: String) -> RawNetworkMessage {
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64;

    let no_address = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), 0);
    let node_socket = SocketAddr::from_str(&dest_socket).unwrap();

    let version = VersionMessage::new(
        ServiceFlags::NONE,
        now,
        Address::new(&node_socket, ServiceFlags::NONE),
        Address::new(&no_address, ServiceFlags::NONE),
        now as u64,
        user_agent.to_owned(),
        0,
    );

    RawNetworkMessage {
        magic: Magic::BITCOIN,
        payload: NetworkMessage::Version(version),
    }
}

// ---
// Verack message
fn create_verack_message() -> RawNetworkMessage {
    RawNetworkMessage {
        magic: Magic::BITCOIN,
        payload: NetworkMessage::Verack,
    }
}

// ---
// Pong message
fn create_pong_message(nonce: u64) -> RawNetworkMessage {
    RawNetworkMessage {
        magic: Magic::BITCOIN,
        payload: NetworkMessage::Pong(nonce),
    }
}
