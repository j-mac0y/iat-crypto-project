use std::io::{Read, Write};
use std::error::Error;

use openssl::rsa::Rsa;
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use openssl::error::ErrorStack;

mod certificate_request;
use certificate_request::CertificateRequest;

mod certificate;
use certificate::Certificate;

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
}

fn handle_certificate_request(request: CertificateRequest, private_key: &PKeyRef<Private>) -> Result<Certificate, Box<dyn Error>> {
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
    // Generate CA keypair and store
    let keypair: Rsa<Private> = generate_keypair()?;
    // From the keypair, grab the public key
    let pkey = PKey::from_rsa(keypair)?;
    let pem: Vec<u8> = pkey.public_key_to_pem()?;
    let public_key= PKey::public_key_from_pem(&pem)?;

    // Listen for connections on 8111

    // Handle request type "CSR" by signing the certificate request

    // Handle request type "GetPubKey" by sending back the CA's public key

    Ok(())
}