use std::net::TcpListener;
use std::io::Read;
use std::error::Error;
use std::net::TcpStream;

use openssl::pkey::PKey;
use openssl::sign::Verifier;
use openssl::hash::MessageDigest;

// Declare the `client_message` module so it can be used in this file
mod client_message;
use client_message::ClientMessage;

fn handle_client(mut stream: TcpStream) {
    // Inititalise a buffer to store the received file.
    let mut buffer = Vec::new();
    
    // Read the data from the client, handling errors.
    if let Err(e) = stream.read_to_end(&mut buffer) {
        eprintln!("Failed to read from client: {}", e);
        return;
    }

    match ClientMessage::decode(&buffer) {
        Ok(client_message) => {
            if is_signature_valid(&client_message.signature, &client_message.public_key, &client_message.data) {
                println!("Received and validated data from client (using signature): {:#?}", String::from_utf8(client_message.data).unwrap());
                return
            }
        }
        Err(e) => {
            eprintln!("Failed to decode the message from client: {}", e);
        }
    }
}

fn is_signature_valid(signature: &Vec<u8>, public_key: &Vec<u8>, data: &Vec<u8>) -> bool {
    // Verify the data using the client's public key
    let pkey = &PKey::public_key_from_pem(public_key).unwrap();
    let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
    verifier.update(&data).unwrap();
    return verifier.verify(&signature).unwrap()
}

fn main() -> Result<(), Box<dyn Error>> {
    let port = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("Server listening on port {port}...");

    // Loop through incoming connection streams indefinitely
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                eprint!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}
