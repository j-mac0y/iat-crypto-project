use std::net::TcpStream;
use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::io::Write;

// Declare the `client_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

fn read_message_from_file() -> Result<Vec<u8>, std::io::Error> {
    // Read the file to send
    let file_path = "src/message.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the "server" (assume client has been provided the MITM port by a malicious actor)
    let server_port = 8666; // MITM port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    // Write first request to server
    stream.write_all(b"give_public_key")?;
    stream.flush()?;
    println!("Sent public key request to server");

    // Read the response from the server
    let mut buffer = [0; 1024]; // Allocate a buffer to hold response
    let n = stream.read(&mut buffer)?; // Read data into buffer
    let empty_message = EncryptedMessage::decode(&buffer[..n])?;
    println!("Received response from server");

    // Add message from client, encrypting it with the public key from the server's empty message
    let message = EncryptedMessage::update_encrypted_data(
        empty_message, read_message_from_file()?)?.encode()?;
    // Send the encrypted data to the server
    stream.write_all(&message)?;
    stream.flush()?;
    println!("Encrypted data sent to server!");

    Ok(())
}