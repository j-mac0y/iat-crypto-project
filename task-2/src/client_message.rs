use bincode::{Encode, Decode, config};
use hex::encode;
use md2::{Md2, Digest};

#[derive(Encode, Decode)]
pub struct ClientMessage {
    pub data: Vec<u8>,
    pub digest: String
}

impl ClientMessage {
    // Constructor for a new `ClientMessage` with hashed digest.
    pub fn new(data: Vec<u8>) -> Self {
        let digest = Self::hash_using_md2(&data);

        Self {
            data,
            digest,
        }
    }

    // Encodes `ClientMessage` into a `Vec<u8>` using `bincode`.
    pub fn encode(&self) -> Result<Vec<u8>, bincode::error::EncodeError> {
        bincode::encode_to_vec(self, config::standard())
    }

    // Decodes `ClientMessage` from a `Vec<u8>` using `bincode`.
    pub fn decode(encoded: &[u8]) -> Result<Self, bincode::error::DecodeError> {
        bincode::decode_from_slice(encoded, config::standard()).map(|(msg, _)| msg)
    }

    pub fn hash_using_md2(data: &[u8]) -> String {
        // Hash the data using MD2
        let mut hasher = Md2::new();
        hasher.update(&data);
        encode(hasher.finalize())
    }
}