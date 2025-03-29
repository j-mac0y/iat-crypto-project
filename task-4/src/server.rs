use std::net::TcpListener;
use std::io::{Read, Write};
use std::error::Error;

use openssl::pkey::{PKey, Private};
use openssl::rsa::Rsa;
use openssl::error::ErrorStack;

// Declare the `encrypted_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
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
    let port = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("Server listening on port {port}...");

    // Generate the server's keypair
    let keypair: Rsa<Private> = generate_keypair()?;
    // From the keypair, grab the public key
    let pkey = PKey::from_rsa(keypair)?;
    let pem: Vec<u8> = pkey.public_key_to_pem()?;
    let public_key= PKey::public_key_from_pem(&pem)?;

    // Accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        println!("Connection received from client");

        let received_data: Vec<u8> = receive_message(&mut stream)?;
        println!("Received from client: {:?}", String::from_utf8(received_data.clone()));

        if received_data == b"give_public_key" {
            // Respond with an empty EncryptedMessage so that the client gets the public key
            println!("Handling public key request");
            let empty_message = EncryptedMessage::new(Vec::new(), &public_key)?.encode()?;
            send_message(&mut stream, &empty_message)?;

            // Now wait for the client’s subsequent message (e.g., encrypted message)
            let client_message: EncryptedMessage = EncryptedMessage::decode(&receive_message(&mut stream)?)?;
            // Decrypt the client's message using the server's private key.
            println!("Received client message: {:?}", String::from_utf8(EncryptedMessage::decrypt(&client_message.encrypted_data, &pkey)?)?);
        } else {
            println!("Error: invalid request received");
        }

        println!("Finished handling stream");
    }

    Ok(())
}
