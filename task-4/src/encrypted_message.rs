use std::error::Error;

use bincode::{Encode, Decode, config};

use openssl::rsa::Padding;
use openssl::pkey::{PKey, Private, Public, PKeyRef};
use openssl::encrypt::{Encrypter, Decrypter};

#[derive(Encode, Decode, Debug)]
pub struct EncryptedMessage {
    pub encrypted_data: Vec<u8>,
    pub public_key: Vec<u8>
}

impl EncryptedMessage {
    // Constructor for a new `EncryptedMessage` with hashed digest.
    #[allow(dead_code)]
    pub fn new(data: Vec<u8>, public_key: &PKey<Public>) -> Result<Self, Box<dyn Error>> {
        let encrypted_data = Self::encrypt(data, &public_key)?;
        let pem: Vec<u8> = public_key.public_key_to_pem().unwrap();
    
        Ok(Self {
            encrypted_data,
            public_key: pem
        })
    }

    // Encodes `EncryptedMessage` into a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(&self, config::standard())
    }

    // Decodes `EncryptedMessage` from a `Vec<u8>` using `bincode`.
    #[allow(dead_code)]
    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_slice(encoded, config::standard()).map(|(msg, _)| msg)
    }

    // Encrypt the message using the public key of the intended recipient
    // Ref: https://docs.rs/openssl/latest/openssl/encrypt/index.html
    pub fn encrypt(data: Vec<u8>, public_key_of_recipient: &PKey<Public>) -> Result<Vec<u8>, Box<dyn Error>> {
        // Encrypt the data with RSA PKCS1
        let mut encrypter = Encrypter::new(public_key_of_recipient).unwrap();
        encrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        // Create an output buffer
        let buffer_len = encrypter.encrypt_len(&data).unwrap();
        let mut encrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let encrypted_len = encrypter.encrypt(&data, &mut encrypted).unwrap();
        encrypted.truncate(encrypted_len);
        Ok(encrypted)
    }

    // Decrypt the message using the private key of the intended recipient
    // Ref: https://docs.rs/openssl/latest/openssl/encrypt/index.html
    #[allow(dead_code)]
    pub fn decrypt(encrypted: &[u8], private_key: &PKeyRef<Private>) -> Result<Vec<u8>,  Box<dyn Error>> {
        // Decrypt the data
        let mut decrypter = Decrypter::new(&private_key).unwrap();
        decrypter.set_rsa_padding(Padding::PKCS1).unwrap();
        // Create an output buffer
        let buffer_len = decrypter.decrypt_len(&encrypted).unwrap();
        let mut decrypted = vec![0; buffer_len];
        // Encrypt and truncate the buffer
        let decrypted_len = decrypter.decrypt(&encrypted, &mut decrypted).unwrap();
        decrypted.truncate(decrypted_len);
        Ok(decrypted)
    }

    // Add new data to the message and encrypt it with the public key. Useful for a MITM!
    #[allow(dead_code)]
    pub fn update_encrypted_data(mut self, new_data: Vec<u8>) -> Result<Self, Box<dyn Error>> {
        let public_key_of_recipient = PKey::public_key_from_pem(&self.public_key)?;
        self.encrypted_data = Self::encrypt(new_data, &public_key_of_recipient)?;
        Ok(self)
    }
}