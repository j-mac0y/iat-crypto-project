use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::Rsa;
use openssl::error::ErrorStack;
use std::io::{Read, Write};
use std::error::Error;

mod certificate;
use certificate::Certificate;

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
}

// fn request_certificate(public_key: PKey<Public>, server_name: String) -> Result<Certificate, Box<dyn Error>> {

//     Ok()
// }

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
    // Step 1: Get Certificate from CA and store in memory 
    // Connect to CA

    // Send "CSR" request to CA

    // Receive response from CA (signed certificate)


    // Step 2: Listen for client requests and respond with the Certificate
    // Listen for requests on 8000

    // Handle request type "GetServerCertificate" by signing the certificate request

    Ok(())
}