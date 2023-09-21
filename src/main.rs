use bitcoin::network::message::NetworkMessage;
use byteorder::LittleEndian;
use std::error::Error;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Replace with the actual Bitcoin node's IP address and port
    let bitcoin_node_address = "217.76.51.25:8333";

    // Connect to the Bitcoin node
    let mut socket = TcpStream::connect(bitcoin_node_address).await?;
    println!("Connected to Bitcoin node at {}", bitcoin_node_address);

    // Perform the Bitcoin handshake
    if let Err(err) = perform_bitcoin_handshake(&mut socket).await {
        eprintln!("Bitcoin handshake failed: {}", err);
        return Ok(());
    }

    println!("Bitcoin handshake successful!");

    // You can now communicate with the Bitcoin node

    Ok(())
}

async fn perform_bitcoin_handshake(socket: &mut TcpStream) -> Result<(), Box<dyn Error>> {
    // Bitcoin handshake protocol:
    // 1. Send a version message
    // 2. Receive a version message from the Bitcoin node
    // 3. Send a verack message
    // 4. Receive a verack message

    // Prepare and send the version message
    let version_message = create_version_message();
    send_message(socket, &version_message).await?;

    // Receive and validate the version message from the Bitcoin node
    let received_version_message = receive_message(socket).await?;
    validate_version_message(&received_version_message)?;

    // Send the verack message
    send_message(socket, &NetworkMessage::Verack).await?;

    // Receive and validate the verack message from the Bitcoin node
    let received_verack_message = receive_message(socket).await?;
    validate_verack_message(&received_verack_message)?;

    Ok(())
}

fn create_version_message() -> NetworkMessage {
    // Create a simplified version message (you can add more details)
    NetworkMessage::Version(bitcoin::network::message::NetworkMessage {
        version: 70015, // Protocol version
        services: 1,    // Service flags (e.g., NODE_NETWORK)
        timestamp: chrono::Utc::now().timestamp() as i64,
        receiver: bitcoin::network::address::Address::new(
            bitcoin::network::constants::ServiceFlags::NETWORK,
            std::net::SocketAddr::from(([0, 0, 0, 0], 8333)),
        ),
        sender: bitcoin::network::address::Address::new(
            bitcoin::network::constants::ServiceFlags::NETWORK,
            std::net::SocketAddr::from(([0, 0, 0, 0], 8333)),
        ),
        nonce: 1234567890, // Random nonce
        user_agent: String::from("MyRustClient"),
        start_height: 0,
        relay: false,
    })
}

async fn send_message(
    socket: &mut TcpStream,
    message: &NetworkMessage,
) -> Result<(), Box<dyn Error>> {
    let serialized_message = message.serialize();
    socket.write_all(&serialized_message).await?;
    Ok(())
}

async fn receive_message(socket: &mut TcpStream) -> Result<NetworkMessage, Box<dyn Error>> {
    let mut header = [0u8; 24];
    socket.read_exact(&mut header).await?;

    let payload_length = (&header[16..20]).read_u32::<LittleEndian>()? as usize;
    let mut payload = vec![0u8; payload_length];
    socket.read_exact(&mut payload).await?;

    let raw_message = [&header[..], &payload[..]].concat();
    let network_message = NetworkMessage::deserialize(&raw_message)?;

    Ok(network_message)
}

fn validate_version_message(message: &NetworkMessage) -> Result<(), Box<dyn Error>> {
    match message {
        NetworkMessage::Version(version_msg) => {
            // Add validation checks for the version message here
            // Example: Check protocol version, services, user agent, etc.

            println!("Received Version message: {:?}", version_msg);
            Ok(())
        }
        _ => Err("Unexpected message type during version handshake".into()),
    }
}

fn validate_verack_message(message: &NetworkMessage) -> Result<(), Box<dyn Error>> {
    match message {
        NetworkMessage::Verack => {
            // Add validation checks for the verack message here
            // Example: Ensure it's a Verack message

            println!("Received Verack message");
            Ok(())
        }
        _ => Err("Unexpected message type during verack handshake".into()),
    }
}
