use std::net::TcpStream;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;
use md2::{Md2, Digest};
use hex::encode;
use bincode::{config, Encode};

#[derive(Encode)]
struct ClientMessage {
    data: Vec<u8>,
    digest: String
}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the "server"
    let server_port = 8666; // MITM port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    // Read the file to send
    let file_path = "src/message.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Hash the data using MD2
    let mut hasher = Md2::new();
    hasher.update(&buffer);
    let hash = hasher.finalize();

    // Create an object with plaintext data and digest, an encode it for transmission.
    let client_message = ClientMessage {
        data: buffer,
        digest: encode(hash),
    };
    let encoded: Vec<u8> = bincode::encode_to_vec(&client_message, config::standard())?;

    // Send the encoded data to the server
    stream.write_all(&encoded)?;

    println!("Data sent!");

    Ok(())
}