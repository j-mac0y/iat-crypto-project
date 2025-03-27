use std::net::TcpStream;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

mod client_message;
use client_message::ClientMessage;

fn create_message_from_file() -> Result<Vec<u8>, std:error:io:error> {
    // Read the file to send
    let file_path = "src/message.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;

    // Create an object with plaintext data and digest, an encode it for transmission.
    let client_message = ClientMessage::new(buffer).unwrap();
    let encoded: Vec<u8> = client_message.encode().unwrap();


}

fn get_public_key_from_server() -> Result<PKey<Public>, Box<dyn Error> {

}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the "server" (assume client has been provided the MITM port by a malicious actor)
    let server_port = 8666; // MITM port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    stream.write_all

    // Send the encrypted data to the server
    stream.write_all(&encrypted)?;
    println!("Encrypted data sent to client!");

    Ok(())
}