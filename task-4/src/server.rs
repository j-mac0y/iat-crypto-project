use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::error::Error;
use std::net::TcpStream;

use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::rsa::Rsa;
use openssl::error::ErrorStack;

// Declare the `client_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

// fn handle_client_message(mut stream: TcpStream, private_key: &PKeyRef<Private>) {
//     match EncryptedMessage::decode(&buffer) {
//         Ok(client_message) => {
//             let decrypted_message = EncryptedMessage::decrypt(&client_message.encrypted_data, private_key);
//             println!("Message decrypted from client reads: {:#?}", decrypted_message);
//         }
//         Err(e) => {
//             eprintln!("Failed to decode the message from client: {}", e);
//         }
//     }
// }

// fn is_signature_valid(signature: &Vec<u8>, public_key: &Vec<u8>, data: &Vec<u8>) -> bool {
//     // Verify the data using the client's public key
//     let pkey = &PKey::public_key_from_pem(public_key).unwrap();
//     let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
//     verifier.update(&data).unwrap();
//     return verifier.verify(&signature).unwrap()
// }

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
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
        println!("Request received from client");

        // Read connections using a fixed size buffer to avoid blocked state
        let mut buffer = [0; 1024]; // Fixed-size buffer
        let n = match stream.read(&mut buffer) {
            Ok(n) if n > 0 => n,
            Ok(_) => {
                println!("Client closed connection.");
                continue;
            }
            Err(e) => {
                eprintln!("Failed to read from client: {}", e);
                continue;
            }
        };

        let received_data = &buffer[..n];
        if received_data == b"give_public_key" {
            println!("Handling public key request");
            let empty_message = EncryptedMessage::new(Vec::new(), &public_key)?.encode()?;
            stream.write_all(&empty_message)?;
            stream.flush()?; // Ensure data is sent immediately
        } else {
            println!("Handling other request");
        }
    }

    Ok(())
}
