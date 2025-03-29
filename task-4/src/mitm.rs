use std::net::TcpListener;
use std::io::{Read, Write};
use std::error::Error;
use std::net::TcpStream;

fn handle_client(mut client: &TcpStream) -> std::io::Result<()> {
    // Connect to the real server
    let server_port = 8000;
    let mut server = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;

    // Craft new data for the server to accept


    // Infinitely forward messages between client and server until one of the connections is closed
    loop {
        // Forward data from client to server
        if let Ok(received) = receive_message(&mut client) {
            if received.len() == 0 {
                break; // Client closed connection
            }
            send_message(&mut server, &received)?;
            println!("Forwarded {} bytes to server", received.len());
        }

        // Forward data from server to client
        if let Ok(received) = receive_message(&mut server) {
            if received.len() == 0 {
                break; // Server closed connection
            }
            send_message(&mut client, &received)?;
            println!("Forwarded {} bytes to client", received.len());
        }
    }

    Ok(())
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