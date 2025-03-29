use bincode::{Encode, Decode, config};
use openssl::pkey::{PKey, PKeyRef, Private};
use openssl::sign::Signer;
use openssl::sign::Verifier;
use openssl::hash::MessageDigest;
use std::error::Error;

#[derive(Encode, Decode)]
pub struct Certificate {
    pub server_name: Vec<u8>,
    pub public_key: Vec<u8>,
    pub ca_signature: Vec<u8>
}

impl Certificate {
    #[allow(dead_code)]
    pub fn new(server_name: Vec<u8>, server_public_key: Vec<u8>, ca_private_key: &PKeyRef<Private>) -> Result<Self, Box<dyn Error>> {        
        let mut data_to_sign = Vec::new();
        data_to_sign.extend(server_name.clone());
        data_to_sign.extend(server_public_key.clone());
        
        Ok(Self {
            server_name,
            public_key: server_public_key,
            ca_signature: Self::generate_signature(ca_private_key, &data_to_sign)?
        })
    }

    // Ref: Task 3
    #[allow(dead_code)]
    fn generate_signature(private_key: &PKeyRef<Private>, data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), &private_key)?;
        signer.update(data)?;
        let signature = signer.sign_to_vec()?;
        Ok(signature)
    }

    // Encodes `Certificate` into a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }

    // Decodes `Certificate` from a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_slice(encoded, config::standard()).map(|(msg, _)| msg)
    }

    // Src: Task 3
    #[allow(dead_code)]
    pub fn is_signature_valid(signature: &Vec<u8>, signer_public_key: &Vec<u8>, data: &Vec<u8>) -> bool {
        // Verify the data using the public key of the signer
        let pkey = &PKey::public_key_from_pem(signer_public_key).unwrap();
        let mut verifier = Verifier::new(MessageDigest::sha256(), &pkey).unwrap();
        verifier.update(&data).unwrap();
        return verifier.verify(&signature).unwrap()
    }
}

   