
use super::signer::TransactionSigner;
use crate::{KeyMaterial, crypto::aes_encryptor, hex_color_to_bytes, salt_from_color};
use anyhow::{anyhow, Result};

#[derive(Debug)]
pub struct Wallet {
    pub encrypted_private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub seed_colors: Vec<String>,
    pub address: String,
    pub type_blockchain: String,
    pub image: Vec<u8>,
    pub emoji: String,
    pub color: String,

}

impl Wallet {
    pub fn new(
        encrypted_private_key: Vec<u8>, 
        public_key: Vec<u8>, 
        address: String,
        seed_colors: Vec<String>,
        type_blockchain: String,
        image: Vec<u8>,
        emoji: String,
        color: String,
    ) -> Self {
        Self {
            encrypted_private_key, 
            public_key,
            seed_colors, 
            address, 
            type_blockchain,
            image,
            emoji,
            color
        }
    }

    pub fn get_bytes_encrypted_sec(&self) -> &[u8] {
        &self.encrypted_private_key
    }

    pub fn get_bytes_pub(&self) -> &[u8] {
        &self.public_key
    }

    pub fn get_hex_encrypted_sec(&self) -> String {
        hex::encode(&self.encrypted_private_key)
    }

    pub fn get_hex_pub(&self) -> String {
        hex::encode(&self.public_key)
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_type_blockchain(&self) -> &str {
        &self.type_blockchain
    }

    pub fn get_seed_colors(&self) -> &Vec<String> {
        &self.seed_colors
    }

    pub fn get_image(&self) -> &Vec<u8> {
        &self.image
    }

    pub fn get_emoji(&self) -> &str {
        &self.emoji
    }

    pub fn get_color(&self) -> &str {
        &self.color
    }

    pub fn decryption_private_key(&self) -> Result<Vec<u8>> {
        let color_bytes = hex_color_to_bytes(&self.color);
        let decryption_key = KeyMaterial::derive_cipher_key(
            &self.image,
            self.emoji.as_bytes(),
            &color_bytes,
        );
        
        aes_encryptor::aes256_decrypt_private_key(
            &self.encrypted_private_key,
            &decryption_key,
        )
    }

    pub fn sign_transaction(
        &self,
        transaction_data: &[u8],
    ) -> Result<Vec<u8>> {
        // Decrypt the private key and handle the Result
        let private_key = self.decryption_private_key()?;
        
        match self.type_blockchain.as_str() {
            "solana" => TransactionSigner::sign_solana_transaction(&private_key, transaction_data),
            _ => Err(anyhow!("Unsupported blockchain for signing {}", self.type_blockchain)),
        }
    }

    pub fn verify_signature(
        &self,
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool> {
        match self.type_blockchain.as_str() {
            "solana" => TransactionSigner::verify_solana_signature(&self.public_key, message, signature),
            _ => Err(anyhow!("Unsupported blockchain for verification {}", self.type_blockchain)),
        }
    }
}


