use std::io::{Read, Write};
use std::error::Error;
use std::net::TcpListener;

use openssl::rsa::Rsa;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::error::ErrorStack;

mod certificate_request;
use certificate_request::CertificateRequest;

mod certificate;
use certificate::Certificate;

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
}

fn issue_cert(request: CertificateRequest, private_key: &PKeyRef<Private>) -> Result<Certificate, Box<dyn Error>> {
    Ok(Certificate::new(request.server_name, request.public_key, private_key)?)
}

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
    // Generate CA keypair and store in memory
    let keypair: Rsa<Private> = generate_keypair()?;
    // From the keypair, grab the public key
    let pkey = PKey::from_rsa(keypair)?;
    let pem: Vec<u8> = pkey.public_key_to_pem()?;
    let public_key= PKey::public_key_from_pem(&pem)?;

    // Listen for connections
    let port = 8111;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("CA listening on port {port}...");

    // Accept incoming connections
    for stream in listener.incoming() {
        let mut stream = stream?;
        let (request_type, received_data) = receive_message(&mut stream)?;
        println!("Received request: {:?}", request_type);

        match request_type.as_str() {
            // Handle request type "CSR" by signing the certificate request
            "CSR" => {
                let cert = issue_cert(CertificateRequest::decode(&received_data)?, &pkey)?;
                send_message(&mut stream, "CSR_Response", &cert.encode()?)?;
                println!("Issued certificate for: {}", String::from_utf8(cert.server_name)?);
            },
            // Handle request type "GetPubKey" by sending back the CA's public key
            "GetPubKey" => {
                send_message(&mut stream, "GetPubKey_Response", &public_key.public_key_to_pem()?)?;
                println!("Responded with public key");
            },
            _ => eprintln!("Error: invalid request type.")
        }
    }

    Ok(())
}