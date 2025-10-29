use std::fs;
use serde::{Deserialize, Serialize};
use base64::{Engine as _, engine::general_purpose};


// use super::signer::TransactionSigner;
use crate::{KeyMaterial, crypto::aes_encryptor, hex_color_to_bytes};
use crate::{TransactionSigner};

use anyhow::{anyhow, Result};

#[derive(Debug, Serialize, Deserialize)]
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

    pub fn get_hex_encrypted_sec(&self) -> String {
        hex::encode(&self.encrypted_private_key)
    }

    pub fn get_hex_pub(&self) -> String {
        hex::encode(&self.public_key)
    }

    pub fn get_seed_colors(&self) -> &Vec<String> {
        &self.seed_colors
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

        pub fn save(&self, path: &str) -> Result<()> {
        let encoded_wallet = serde_json::json!({
            "encrypted_private_key": general_purpose::STANDARD.encode(&self.encrypted_private_key),
            "public_key": general_purpose::STANDARD.encode(&self.public_key),
            "seed_colors": self.seed_colors,
            "address": self.address,
            "type_blockchain": self.type_blockchain,
            "emoji": self.emoji,
            "color": self.color,
            "image": general_purpose::STANDARD.encode(&self.image),
        });

        let json = serde_json::to_string_pretty(&encoded_wallet)?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> Result<Self> {
        let data = fs::read_to_string(path)?;
        let json_value: serde_json::Value = serde_json::from_str(&data)?;

        let encrypted_private_key =
            general_purpose::STANDARD.decode(json_value["encrypted_private_key"].as_str().unwrap())?;
        let public_key =
            general_purpose::STANDARD.decode(json_value["public_key"].as_str().unwrap())?;
        let image =
            general_purpose::STANDARD.decode(json_value["image"].as_str().unwrap())?;

        let seed_colors = json_value["seed_colors"]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect::<Vec<String>>();

        let address = json_value["address"].as_str().unwrap().to_string();
        let type_blockchain = json_value["type_blockchain"].as_str().unwrap().to_string();
        let emoji = json_value["emoji"].as_str().unwrap().to_string();
        let color = json_value["color"].as_str().unwrap().to_string();

        Ok(Self {
            encrypted_private_key,
            public_key,
            seed_colors,
            address,
            type_blockchain,
            image,
            emoji,
            color,
        })
    }
}


