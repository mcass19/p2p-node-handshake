use bitcoin::consensus::{deserialize_partial, encode::serialize};
use bitcoin::network::address::Address;
use bitcoin::network::constants::{Magic, ServiceFlags};
use bitcoin::network::message::{NetworkMessage, RawNetworkMessage};
use bitcoin::network::message_network::VersionMessage;
use std::{
    error::Error,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    str::FromStr,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with the actual Bitcoin node's IP address, port and user agent
    let bitcoin_node_address = "94.231.253.18:8333";
    let user_agent = "/Satoshi:25.0.0/";

    let mut socket = TcpStream::connect(bitcoin_node_address).await?;
    println!("Connected to Bitcoin node at {}", bitcoin_node_address);

    if let Err(err) = perform_bitcoin_handshake(
        &mut socket,
        bitcoin_node_address.to_string(),
        user_agent.to_string(),
    )
    .await
    {
        eprintln!("Bitcoin handshake failed: {}", err);
        return Ok(());
    }

    println!("Bitcoin handshake successful!");

    // You can now further communicate with the Bitcoin node

    Ok(())
}

async fn perform_bitcoin_handshake(
    socket: &mut TcpStream,
    bitcoin_node_address: String,
    user_agent: String,
) -> Result<(), Box<dyn Error>> {
    // Prepare and send the version message
    let version_message = create_version_message(bitcoin_node_address, user_agent);
    send_message(socket, &version_message).await?;

    // Continuously listen for incoming messages
    loop {
        let received_message = receive_message(socket).await?;

        match received_message.payload {
            NetworkMessage::Version(version_msg) => {
                println!("Received Version message: {:?}", version_msg);

                let verack_message = create_verack_message();
                send_message(socket, &verack_message).await?;
            }
            NetworkMessage::Ping(nonce) => {
                println!("Received Ping message - nonce: {:?}", nonce);

                let pong_message = create_pong_message(nonce);
                send_message(socket, &pong_message).await?;
            }
            NetworkMessage::Verack => {
                println!("Received Verack message");

                return Ok(());
            }
            other_message => {
                return Err(format!(
                    "Unexpected message type during handshake: {:?}",
                    other_message
                )
                .into());
            }
        }
    }
}

// ---
// Send and receive
async fn send_message(
    socket: &mut TcpStream,
    message: &RawNetworkMessage,
) -> Result<(), Box<dyn Error>> {
    let serialized_message = serialize(message);
    socket.write_all(serialized_message.as_slice()).await?;

    Ok(())
}

async fn receive_message(socket: &mut TcpStream) -> Result<RawNetworkMessage, Box<dyn Error>> {
    let mut header = [0u8; 24];
    socket.read(&mut header).await?;

    let mut payload = [0u8; 442];
    socket.read(&mut payload).await?;

    let raw_message = [&header[..], &payload[..]].concat();
    let (network_message, _consumed) = deserialize_partial(&raw_message)?;

    Ok(network_message)
}

// ---
// Version message
pub fn create_version_message(dest_socket: String, user_agent: String) -> RawNetworkMessage {
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
