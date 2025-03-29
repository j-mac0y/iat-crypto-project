use std::io::{Read, Write};
use std::error::Error;
use std::net::TcpStream;

mod certificate;
use certificate::Certificate;

fn send_message(stream: &mut impl Write, identifier: &str, message: &[u8]) -> Result<(), Box<dyn Error>> {
    let identifier_bytes = identifier.as_bytes();
    let identifier_len = identifier_bytes.len() as u32;
    let message_len = message.len() as u32;

    stream.write_all(&identifier_len.to_be_bytes())?; // Send identifier length
    stream.write_all(identifier_bytes)?; // Send identifier
    stream.write_all(&message_len.to_be_bytes())?; // Send message length
    stream.write_all(message)?; // Send actual message
    stream.flush()?;

    Ok(())
}

fn receive_message(stream: &mut impl Read) -> Result<(String, Vec<u8>), Box<dyn Error>> {
    // Read identifier length
    let mut identifier_len_bytes = [0u8; 4];
    stream.read_exact(&mut identifier_len_bytes)?;
    let identifier_len = u32::from_be_bytes(identifier_len_bytes) as usize;

    // Read identifier
    let mut identifier_bytes = vec![0; identifier_len];
    stream.read_exact(&mut identifier_bytes)?;
    let identifier = String::from_utf8(identifier_bytes).map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Invalid UTF-8 identifier"))?;

    // Read message length
    let mut message_len_bytes = [0u8; 4];
    stream.read_exact(&mut message_len_bytes)?;
    let message_len = u32::from_be_bytes(message_len_bytes) as usize;

    // Read actual message
    let mut buffer = vec![0; message_len];
    stream.read_exact(&mut buffer)?;

    Ok((identifier, buffer))
}

fn main() -> Result<(), Box<dyn Error>> {
    // Step 1: Get the Certificate from the server
    // Connect to server
    let server_port = 8000;
    let mut server = TcpStream::connect(format!("127.0.0.1:{server_port}"))?;
    // Send "GetServerCertificate" request
    send_message(&mut server, "GetServerCertificate", b"")?;
    // Receive certificate back from server
    let cert: Certificate = Certificate::decode(&receive_message(&mut server)?.1)?;
    println!("Received certificate from server with name: {}", String::from_utf8(cert.server_name.clone())?);

    // Step 2: Verify the Certificate using the public key of the certificate authority
    // Connect to CA
    let ca_port = 8111; // MITM port
    let mut certificate_authority = TcpStream::connect(format!("127.0.0.1:{ca_port}"))?;
    // Send "GetPubKey" request to CA
    send_message(&mut certificate_authority, "GetPubKey", b"")?;
    println!("Got public key of certificate authority.");

    // Step 3: Validate the server's certificate using the CA's public key
    let ca_public_key = receive_message(&mut certificate_authority)?.1;
    let mut cert_data = Vec::new();
    cert_data.extend(&cert.server_name.clone());
    cert_data.extend(&cert.public_key.clone());
    if Certificate::is_signature_valid(&cert.ca_signature, &ca_public_key, &cert_data) {
        println!("Validated certificate from server against public key provided by CA.");
    } else {
        eprintln!("Error: certificate could not be validated.");
    }

    Ok(())
}