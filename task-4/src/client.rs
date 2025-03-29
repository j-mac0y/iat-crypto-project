use std::net::TcpStream;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};


// Declare the `encrypted_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

fn read_message_from_file() -> Result<Vec<u8>, std::io::Error> {
    // Read the file to send
    let file_path = "./message.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn send_message(stream: &mut impl Write, message: &[u8]) -> std::io::Result<()> {
    let len = message.len() as u32;
    let len_bytes = len.to_be_bytes();
    stream.write_all(&len_bytes)?; // Include the message length as the first 4 bytes
    stream.write_all(message)?;
    stream.flush()?;
    Ok(())
}

fn receive_message(stream: &mut impl Read) -> std::io::Result<Vec<u8>> {
    let mut len_bytes = [0u8; 4];
    stream.read_exact(&mut len_bytes)?;  // Read the message exactly, using the message length
    let len = u32::from_be_bytes(len_bytes);
    let mut buffer = vec![0; len as usize];
    stream.read_exact(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the "server" (assume client has been provided the MITM port by a malicious actor)
    let server_port = 8666; // MITM port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    // Get public key from server
    send_message(&mut stream, b"give_public_key")?;
    println!("Sent public key request to server");
    // Read the response from the server
    let empty_message = EncryptedMessage::decode(&receive_message(&mut stream)?)?;
    println!("Received response from server");

    // Add message from client, encrypting it with the public key from the server's empty message
    let message = EncryptedMessage::update_encrypted_data(
        empty_message, read_message_from_file()?)?.encode()?;
    // Send the encrypted data to the server
    send_message(&mut stream, &message)?;
    println!("Encrypted data sent to server!");

    Ok(())
}