use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use bitcoin_hashes::{sha256, Hash};

const TESTNET_DNS_SEEDS: [&str; 4] = [
    "seed.testnet.bitcoin.sprovoost.nl:18333",
    "testnet-seed.bitcoin.jonasschnelli.ch:18333",
    "seed.tbtc.petertodd.org:18333",
    "testnet-seed.bluematt.me:18333",];

#[tokio::main]
async fn main() -> io::Result<()>{
    println!("Connecting to Bitcoin Testnet...");

    for seed in &TESTNET_DNS_SEEDS {
        match TcpStream::connect(seed).await {
            Ok(mut stream) => {
                println!("Connected to seed: {seed}");

                let version_message = create_version_packet();
                stream.write_all(&version_message).await?;
                print!("Sent version message.");

                let mut buffer = [0; 1024];
                let n= stream.read(&mut buffer).await?;
                println!("Received: {:?}", &buffer[..n]);

                if let Some((command, length)) = parse_message_header(&buffer) {
                    println!("Received command: {}, length: {}", command, length);
                } else {
                    println!("Failed to parse message header.");
                }
                
                return Ok(())
            }
            Err(e) => {
                println!("Failed to connect to {seed}: {e}")
            }
        }
    }
    println!("Unable to connect to any seeds.");
    Ok(())
}

fn create_version_message() -> Vec<u8> {
    let protocol_version: i32 = 70015;
    let services: u64 = 0;
    let timestamp: i64 = chrono::Utc::now().timestamp();
    let addr_recv = [0u8; 26];
    let addr_from = [0u8; 26];
    let nonce: u64 = rand::random();
    let user_agent = b"/rust-node:0.1/";
    let start_height: i32 = 0;
    let relay: u8 = 0;

    let mut message = vec![];

    message.extend(&protocol_version.to_le_bytes());
    message.extend(&services.to_le_bytes());
    message.extend(&timestamp.to_le_bytes());
    message.extend(&addr_recv);
    message.extend(&addr_from);
    message.extend(&nonce.to_le_bytes());
    message.push(user_agent.len() as u8);
    message.extend(user_agent);
    message.extend(&start_height.to_le_bytes());
    message.push(relay);

    message
}


fn create_version_packet() -> Vec<u8> {
    let payload = create_version_message();

    // Magic number for testnet
    let magic: [u8; 4] = [0x0B, 0x11, 0x09, 0x07];

    // "version" Command
    let mut command = [0u8; 12];
    let cmd_str = b"version";
    command[..cmd_str.len()].copy_from_slice(cmd_str);

    // payload length
    let length = (payload.len() as u32).to_le_bytes();

    // Checksum: double sha256 for payload
    let hash1 = sha256::Hash::hash(&payload);
    let hash2 = sha256::Hash::hash(&hash1);
    let checksum = &hash2[..4];

    // Build full message
    let mut message = vec![];
    message.extend_from_slice(&magic);
    message.extend_from_slice(&command);
    message.extend_from_slice(&length);
    message.extend_from_slice(checksum);
    message.extend_from_slice(&payload);

    message
}

fn parse_message_header(data: &[u8]) -> Option<(&str, usize)> {
    if data.len() < 24 {
        return None;
    }

    let magic = &data[0..4];
    if magic != &[0x0B, 0x11, 0x09, 0x07] {
        return None; // Magic number incorrect
    }

    let command = std::str::from_utf8(&data[4..16])
        .ok()?
        .trim_end_matches('\0'); // Convert to string
    let length = u32::from_le_bytes(data[16..20].try_into().unwrap()) as usize;

    Some((command, length))
}