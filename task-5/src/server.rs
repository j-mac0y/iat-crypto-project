use openssl::pkey::{PKey, Private, Public};
use openssl::rsa::Rsa;
use openssl::error::ErrorStack;
use std::io::{Read, Write};
use std::error::Error;
use std::net::{TcpStream, TcpListener};

mod certificate;
use certificate::Certificate;

mod certificate_request;
use certificate_request::CertificateRequest;

// Generate an RSA keypair. This needs to happen at the server so the private key remains in the server state.
fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
    Rsa::generate(2048)
}

fn request_certificate(public_key: PKey<Public>, server_name: &[u8], certificate_authority: &mut TcpStream) -> Result<Certificate, Box<dyn Error>> {
    send_message(certificate_authority, "CSR", 
    &CertificateRequest::new(server_name.to_vec(), public_key.public_key_to_pem()?)?.encode()?)?;
    Ok(Certificate::decode(&receive_message(certificate_authority)?.1)?)
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
    // Generate the server's keypair and store in memory
    let keypair: Rsa<Private> = generate_keypair()?;
    // From the keypair, grab the public key
    let pkey = PKey::from_rsa(keypair)?;
    let pem: Vec<u8> = pkey.public_key_to_pem()?;
    let public_key= PKey::public_key_from_pem(&pem)?;
    
    // Step 1: Get Certificate from CA and store in memory 
    // Connect to CA
    let ca_port = 8111; // MITM port
    let mut certificate_authority = TcpStream::connect(format!("127.0.0.1:{ca_port}"))?;
    // Send "CSR" request to CA
    let signed_cert: Certificate = request_certificate(public_key, b"James_Server_Inc", &mut certificate_authority)?;
    println!("Received signed certificate from CA for: {}", String::from_utf8(signed_cert.server_name.clone())?);
    
    // Step 2: Listen for client requests and respond with the Certificate
    // Listen for requests on 8000
    let port = 8000;
    let listener = TcpListener::bind(format!("127.0.0.1:{port}"))?;
    println!("Server listening on port {port}...");
    for stream in listener.incoming() {
        let mut stream = stream?;
        let (request_type, _received_data) = receive_message(&mut stream)?;
        println!("Received request: {:?}", request_type);

        // Handle request type "GetServerCertificate" by returning the certificate to the client
        match request_type.as_str() {
            "GetServerCertificate" => {
                send_message(&mut stream, "GetCertificate_Response", &signed_cert.encode()?)?;
                println!("Responded with certificate");
            },
            _ => eprintln!("Error: invalid request type.")
        }
    }

    Ok(())
}