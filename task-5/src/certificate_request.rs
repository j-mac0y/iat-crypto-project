use bincode::{Encode, Decode, config};
use std::error::Error;

#[derive(Encode, Decode)]
pub struct CertificateRequest {
    pub server_name: Vec<u8>,
    pub public_key: Vec<u8>
}

impl CertificateRequest {
    #[allow(dead_code)]
    pub fn new(server_name: Vec<u8>, public_key: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        Ok(Self {
            server_name,
            public_key
        })
    }

    // Encodes `CertificateRequest` into a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }

    // Decodes `CertificateRequest` from a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_slice(encoded, config::standard()).map(|(msg, _)| msg)
    }
}

   