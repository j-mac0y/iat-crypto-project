use std::net::TcpListener;
use std::io::Read;
use std::error::Error;
use std::net::TcpStream;

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
            // Validate the data by re-hashing it and checking it against the digest
            assert_eq!(ClientMessage::hash_using_md2(&client_message.data), client_message.digest);

            match String::from_utf8(client_message.data) {
                Ok(string) => println!("Received and validated data (against hash) from client: {}", string),
                Err(e) => println!("Error: {}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to decode the message from client: {}", e);
        }
    }
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
