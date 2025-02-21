use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let port = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).await?;
    println!("Server listening on port {port}...");

    // Accept an incoming connection
    let (mut socket, _) = listener.accept().await?;

    // Buffer to store the received file
    let mut buffer = Vec::new();
    socket.read_to_end(&mut buffer).await?;

    match String::from_utf8(buffer) {
        Ok(string) => {
            println!("Received from client: {string}");
        }
        Err(e) => {
            eprintln!("Failed to convert buffer to string: {}", e);
        }
    }
    Ok(())
}
