
use std::path::PathBuf;

use aes_gcm::aes::cipher;
use anyhow::{anyhow, Result};
use ed25519_dalek::{SigningKey, VerifyingKey};
use image::load;
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use base58::{ToBase58};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use tiny_keccak::{Keccak, Hasher};

use crate::crypto::aes_encryptor::aes256_encrypt_private_key;
use crate::key_derivation::key_deriver::KeyMaterial;
use super::Wallet;

use crate::key_derivation::image_loader::load_image;
pub struct WalletBuilder;
impl WalletBuilder {
    pub fn generate_wallet(image_data:&PathBuf , emoji: &str, color: &str, blockchain:&str) -> Result<Wallet> {
        
        let wallet_data = KeyMaterial::generate(&image_data, &emoji, &color)?;
        let new_image_data = load_image(&image_data)?;
        match blockchain.to_lowercase().as_str() {
            "solana" => Self::generate_solana(
                &wallet_data.master_seed,
                &wallet_data.cipher_key.unwrap(),
                wallet_data.seed_colors,
                new_image_data, emoji, color
            ),

            "ethereum" => Self::generate_ethereum(
                &wallet_data.master_seed, 
                &wallet_data.cipher_key.unwrap(),
                wallet_data.seed_colors,
                new_image_data, emoji, color),
            
            "bitcoin" => Self::generate_bitcoin(
                &wallet_data.master_seed,
                &wallet_data.cipher_key.unwrap(),
                wallet_data.seed_colors,
                new_image_data, emoji, color),
            
            _ => Err(anyhow!("Unsupported blockchain: {}", blockchain))
        }
    }

    pub fn recover_wallet(colors: &[String],primary_image: &PathBuf,
        emoji: &str, ) -> Result<Wallet>{
        let mut wallet_data = KeyMaterial::recover_from_colors(colors)?;
        let new_image = load_image(primary_image)?;
        wallet_data.regenerate_cipher_key(primary_image, emoji)?;
        Self::generate_solana(
            &wallet_data.master_seed,
            &wallet_data.cipher_key.unwrap(),
            wallet_data.seed_colors,
            new_image, emoji, &colors[16]
        )

    }

    fn generate_solana(
        seed: &[u8; 48], 
        encryption_key: &[u8; 64],
        seed_color: Vec<String>,
        image: Vec<u8>, emoji: &str, color:&str) -> Result<Wallet> {
            
        let seed_array: [u8; 32] = seed[0..32].try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert slice to array"))?;
    
        let (public_key, encrypted_private_key) = {
            let private_key = SigningKey::from_bytes(&seed_array);
            let public_key = VerifyingKey::from(&private_key);
            let private_key_bytes = private_key.to_bytes();            
            let encrypted_private_key = aes256_encrypt_private_key(&private_key_bytes, encryption_key)?;
            let mut sensetive_date = private_key_bytes;
            zeroize::Zeroize::zeroize(&mut sensetive_date);
            
            (public_key,  encrypted_private_key)
        };

        let address = Self::generate_solana_address(&public_key);
        let type_blockchain = String::from("solana");
        Ok(Wallet::new( 
            encrypted_private_key.to_vec(),
            public_key.to_bytes().to_vec(),
            address,
            seed_color,
            type_blockchain,
            image,
            String::from(emoji),
            String::from(color)
        ))
    }

    fn generate_ethereum(seed: &[u8; 48], encryption_key: &[u8; 64],seed_color: Vec<String>,
        image: Vec<u8>, emoji: &str, color:&str) -> Result<Wallet> {
        let secp = Secp256k1::new();
        let seed_array: [u8; 32] = seed[0..32].try_into()
            .map_err(|_| anyhow!("Failed to convert slice to array"))?;
        
        let (public_key, encrypted_private_key) = {
            let private_key = SecretKey::from_byte_array(seed_array)?;
            let public_key = PublicKey::from_secret_key(&secp, &private_key);
            
            let private_key_bytes = private_key.secret_bytes();
            let encrypted_private_key = aes256_encrypt_private_key(&private_key_bytes, encryption_key)?;
            // 
            let mut sensetive_date = private_key_bytes;
            zeroize::Zeroize::zeroize(&mut sensetive_date);
            
            (public_key,  encrypted_private_key)
        };
       
        let address = Self::generate_ethereum_address(&public_key);
        let type_blockchain = String::from("ethereum");

        Ok(Wallet::new(
            encrypted_private_key.to_vec(),
            public_key.serialize_uncompressed().to_vec(),
            address,
            seed_color,
            type_blockchain,
            image,
            String::from(emoji),
            String::from(color)
        ))
    }

    fn generate_bitcoin(seed: &[u8; 48], encryption_key: &[u8; 64], seed_color: Vec<String>,
        image: Vec<u8>, emoji: &str, color:&str) -> Result<Wallet> {
        let secp = Secp256k1::new();
        let seed_array: [u8; 32] = seed[32..64].try_into()
            .map_err(|_| anyhow!("Failed to convert slice to array"))?;
        
        let (public_key, encrypted_private_key) = {
            let private_key = SecretKey::from_byte_array(seed_array)?;
            let public_key = PublicKey::from_secret_key(&secp, &private_key);
                
            let private_key_bytes = private_key.secret_bytes();
            let encrypted_private_key = aes256_encrypt_private_key(&private_key_bytes, encryption_key)?;
             
            let mut sensetive_date = private_key_bytes;
            zeroize::Zeroize::zeroize(&mut sensetive_date);
                
            (public_key,  encrypted_private_key)
        };

        let address = Self::generate_bitcoin_address(&public_key);
        let type_blockchain = String::from("bitcoin");

        Ok(Wallet::new(
            encrypted_private_key.to_vec(),
            public_key.serialize_uncompressed().to_vec(),
            address,
            seed_color,
            type_blockchain,
            image,
            String::from(emoji),
            String::from(color)
        ))
    }


    fn generate_solana_address(public_key: &ed25519_dalek::VerifyingKey) -> String {
        public_key.as_bytes().to_base58()
    }

    fn generate_ethereum_address(public_key: &secp256k1::PublicKey) -> String {
        let serialized = public_key.serialize_uncompressed();
        let public_key_bytes = &serialized[1..];
        
        let mut hasher = Keccak::v256();
        hasher.update(public_key_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
    
        format!("0x{}", hex::encode(&hash[12..]))
    }

    fn generate_bitcoin_address(public_key: &secp256k1::PublicKey) -> String {
        let serialized = public_key.serialize_uncompressed();
        let public_key_bytes = &serialized[1..];
    
        let mut sha256 = Sha256::new();
        sha256.update(public_key_bytes);
        let sha256_result = sha256.finalize();
    
        let mut ripemd160 = Ripemd160::new();
        ripemd160.update(sha256_result);
        let ripemd160_result = ripemd160.finalize();
    
        // add network byte
        let mut with_network_byte = vec![0x00];
        with_network_byte.extend_from_slice(&ripemd160_result);
    
        let mut sha256_checksum1 = Sha256::new();
        sha256_checksum1.update(&with_network_byte);
        let checksum1 = sha256_checksum1.finalize();
    
        let mut sha256_checksum2 = Sha256::new();
        sha256_checksum2.update(checksum1);
        let final_checksum = sha256_checksum2.finalize();
        
        with_network_byte.extend_from_slice(&final_checksum[0..4]);
    
        with_network_byte.to_base58()
    }

}

