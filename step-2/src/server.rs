use std::net::TcpListener;
use std::io::Read;
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

    match String::from_utf8(buffer) {
        Ok(string) => {
            println!("Received from client: {string}");
        }
        Err(e) => {
            eprintln!("Failed to convert buffer to string: {}", e);
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
