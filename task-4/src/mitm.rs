use std::net::TcpListener;
use std::io::{Read, BufReader, Write, BufWriter};
use std::error::Error;
use std::net::TcpStream;

fn handle_client(client: &TcpStream) -> std::io::Result<()> {
    // Connect to the real server
    let server_port = 8000;
    let server = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;

    // Infinitely forward messages between client and server until one of the connections is closed
    loop {
        // Forward data from client to server
        if let Ok(received) = receive(&client) {
            if received.len() == 0 {
                break; // Client closed connection
            }
            send(&server, &received)?;
            println!("Forwarded {} bytes to server", received.len());
        }

        // Forward data from server to client
        if let Ok(received) = receive(&server) {
            if received.len() == 0 {
                break; // Server closed connection
            }
            send(&client, &received)?;
            println!("Forwarded {} bytes to client", received.len());
        }
    }

    Ok(())
}

fn send(mut stream: &TcpStream, request: &[u8]) -> std::io::Result<()> {
    let mut writer = BufWriter::new(&mut stream);
    writer.write_all(&request)?;  // Send raw bytes
    writer.flush()?; // Ensure everything is sent
    Ok(())
}

fn receive(mut stream: &TcpStream) -> std::io::Result<Vec<u8>> {
    let mut reader = BufReader::new(&mut stream);
    let mut buffer = vec![0; 1024]; // Adjust size as needed
    let bytes_read = reader.read(&mut buffer)?; // Read raw bytes
    buffer.truncate(bytes_read); // Keep only the valid data
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