use std::net::TcpStream;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

mod client_message;
use client_message::ClientMessage;

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

    // Create an object with plaintext data and digest, an encode it for transmission.
    let client_message = ClientMessage::new(buffer);

    // Send the encoded data to the server
    stream.write_all(&client_message.encode()?)?;

    println!("Data sent!");

    Ok(())
}