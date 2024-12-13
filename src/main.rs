use tokio::net::TcpStream;
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};

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

                let version_message = create_version_message();
                stream.write_all(&version_message).await?;
                print!("Sent version message.");

                let mut buffer = [0; 1024];
                let n= stream.read(&mut buffer).await?;
                println!("Received: {:?}", &buffer[..n]);

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
    let mut message = vec![];
    message.extend(b"version".to_vec());
    message
}