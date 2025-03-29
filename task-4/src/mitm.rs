use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::error::Error;
use std::net::TcpStream;
use std::fs::File;
use std::io::{self, copy};

use openssl::pkey::{PKey, Public};

// Declare the `client_message` module so it can be used in this file
mod encrypted_message;
use encrypted_message::EncryptedMessage;

fn handle_client(mut client: TcpStream) -> io::Result<()> {
    // Craft a new encrypted message and sign using a new keypair, making this an active MITM.
    // let malicious_message: EncryptedMessage = EncryptedMessage::new(read_message_from_file()?, public_key)?;
    // Connect to the real server
    let server_port = 8000;
    let mut server = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;

    let mut client_buf = [0; 1024]; // Buffer for client data
    let mut server_buf = [0; 1024]; // Buffer for server data

    loop {
        // Forward data from client to server
        if let Ok(n) = client.read(&mut client_buf) {
            if n == 0 {
                break; // Client closed connection
            }
            server.write_all(&client_buf[..n])?;
            println!("Forwarded {} bytes from client to server", n);
        }

        // Forward data from server to client
        if let Ok(n) = server.read(&mut server_buf) {
            if n == 0 {
                break; // Server closed connection
            }
            client.write_all(&server_buf[..n])?;
            println!("Forwarded {} bytes from server to client", n);
        }
    }

    Ok(())
}

fn read_message_from_file() -> Result<Vec<u8>, std::io::Error> {
    // Read the file to send
    let file_path = "src/malicious.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn forward_to_server(buffer: Vec<u8>) -> Result<(), Box<dyn Error>> {
    // Connect to the real server
    let server_port = 8000; // Real server port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    stream.write_all(&buffer)?;
    println!("Data forwarded to server!");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let port = 8666;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("MITM listening on port {port}...");

    // Loop through incoming connection streams indefinitely
    for stream in listener.incoming() {
        match stream {
            Ok(client) => {
                handle_client(client)?;
            }
            Err(e) => {
                eprint!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}