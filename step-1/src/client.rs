use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::fs::File;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Connect to the server
    let server_port = 8000;
    let mut stream = TcpStream::connect(format!("127.0.0.1:{server_port}")).await?;
    println!("Connected to server on {server_port}");

    // Read the file to send
    let file_path = "src/message.txt";
    let mut file = File::open(file_path).await?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).await?;

    // Send the file data to the server
    stream.write_all(&buffer).await?;

    println!("File sent!");

    Ok(())
}