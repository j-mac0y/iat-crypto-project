use std::error::Error;

use bincode::{Encode, Decode, config};

use openssl::rsa::{Rsa, Padding};
use openssl::pkey::{PKey, PKeyRef, Private, Public};
use openssl::encrypt::{Encrypter, Decrypter};
use openssl::sign::Signer;
use openssl::hash::MessageDigest;
use openssl::error::ErrorStack;

#[derive(Encode, Decode, Debug)]
pub struct ClientMessage {
    pub data: Vec<u8>,
    pub signature: Vec<u8>,
    pub public_key: Vec<u8>
}

impl ClientMessage {
    // Constructor for a new `ClientMessage` with hashed digest.
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        match Self::generate_keypair() {
            Ok(rsa_keypair) => {
                // From the keypair, grab the public key
                let pkey = PKey::from_rsa(rsa_keypair).unwrap();
                let public_key: Vec<u8> = pkey.public_key_to_pem().unwrap();

                let signature = Self::generate_signature(&pkey, &data).unwrap();
                Ok(Self {
                    data,
                    signature,
                    public_key
                })
            },
            Err(e) => {
                println!("Error: {}", e);
                Err(Box::new(e))
            }
        }
    }

    fn generate_signature(keypair: &PKeyRef<Private>, data: &[u8]) -> Result<Vec<u8>, openssl::error::ErrorStack> {
        let mut signer = Signer::new(MessageDigest::sha256(), &keypair)?;
        signer.update(data)?;
        let signature = signer.sign_to_vec()?;
        Ok(signature)
    }

    fn generate_keypair() -> Result<Rsa<Private>, ErrorStack> {
        Rsa::generate(2048)
    }

    // Encodes `ClientMessage` into a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }

    // Decodes `ClientMessage` from a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_slice(encoded, config::standard()).map(|(msg, _)| msg)
    }

    // Encrypt the message using the public key of the intended recipient
    // Ref: https://docs.rs/openssl/latest/openssl/encrypt/index.html
    pub fn encrypt(&self, public_key_of_recipient: PKey<Public>) -> Result<Vec<u8>, Box<dyn Error>> {
        // Encrypt the data with RSA PKCS1
        let mut encrypter = Encrypter::new(&public_key_of_recipient).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        let data = &self;
        // Create an output buffer
        let buffer_len = encrypter.encrypt_len().unwrap();
        let mut encrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let encrypted_len = encrypter.encrypt(data, &mut encrypted).unwrap();
        encrypted.truncate(encrypted_len);
        Ok(encrypted)
    }

    // Decrypt the message using the private key of the intended recipient
    // Ref: https://docs.rs/openssl/latest/openssl/encrypt/index.html
    pub fn decrypt(&self, private_key_of_recipient: PKey<Private>) -> Result<Self,  Box<dyn Error>> {
        // Decrypt the data
        let mut decrypter = Decrypter::new(&private_key_of_recipient).unwrap();
        decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        // Create an output buffer
        let buffer_len = decrypter.decrypt_len(&encrypted).unwrap();
        let mut decrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let decrypted_len = decrypter.decrypt(&encrypted, &mut decrypted).unwrap();
        decrypted.truncate(decrypted_len);
        Ok(decrypted)
    }
}