use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::error::Error;
use std::net::TcpStream;

use openssl::pkey::PKey;
use openssl::sign::Verifier;
use openssl::hash::MessageDigest;

// Declare the `client_message` module so it can be used in this file
mod client_message;
use client_message::ClientMessage;
use tokio::time::error::Elapsed;

fn handle_client_message(mut stream: TcpStream) {
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
                println!("Received and validated data from client: {:#?}", String::from_utf8(client_message.data).unwrap());
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

    // Loop through incoming connection streams indefinitely. 
    // If it is an odd connection, return an emtpy message so client gets public key.
    // If it is an even connection, read the encrypted message from the client.
    let mut counter: i32 = 1;
    for stream in listener.incoming() {
        if &counter%2 == 1 {
            let mut stream = stream.unwrap();
            let empty_message = ClientMessage::new(Vec::new())?.encode()?;
            stream.write_all(&empty_message);
        } else if &counter%2 == 2 {
            handle_client_message(stream?);
        } else {
            println!("Error in handling connection from client.")
        }

        counter += 1;
    }

    Ok(())
}
