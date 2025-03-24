use std::net::TcpListener;
use std::io::Read;
use std::io::Write;
use std::error::Error;
use std::net::TcpStream;

fn handle_client(mut stream: TcpStream) {
    // Buffer to store the received file
    let mut buffer = Vec::new();
    
    // Handle errors from read_to_end
    if let Err(e) = stream.read_to_end(&mut buffer) {
        eprintln!("Failed to read from client: {}", e);
        return;
    }

    match String::from_utf8(buffer.clone()) {
        Ok(string) => {
            println!("Received from client: {string}");

            // Forward to the server
            if let Err(e) = forward_to_server(buffer) {
                eprint!("Failed to forward data to server: {}", e);
            }
        }
        Err(e) => {
            eprintln!("Failed to convert buffer to string: {}", e);
        }
    }
}

fn forward_to_server(buffer: Vec<u8>) -> Result<(), Box<dyn Error>> {
    // Connect to the real server
    let server_port = 8000; // Real server port
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    println!("Connected to server on {server_port}");

    stream.write_all(&buffer)?;
    println!("File forwarded to server!");

    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    let port = 8666;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("MITM listening on port {port}...");

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