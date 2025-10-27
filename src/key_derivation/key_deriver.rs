use anyhow::Result;
use std::path::PathBuf;

use argon2::{Argon2};
use super::image_loader;
use super::entropy;
use crate::crypto::{blake2b_hash, salt_from_color};
use super::utils;


#[derive(Debug, Clone)]
pub struct KeyMaterial {
    pub master_seed: [u8; 48],           // Main Seed for key generation
    pub cipher_key: [u8; 64],    // The key for encrypting the private key
}
impl KeyMaterial {
    pub fn new(master_seed: [u8; 48], cipher_key: [u8; 64]) -> Self {
        Self { master_seed, cipher_key }
    }

    pub fn get_master_seed(&self) -> [u8; 48] {
        self.master_seed
    }

    pub fn get_cipher_key(&self) -> [u8; 64] {
        self.cipher_key
    }

    pub fn derive_key_material(
        primary_image:&Vec<u8>,
        emoji: &str,
        color: &str
    ) -> Result<KeyMaterial> {
        
        // 1) load image -> raw RGB bytes
        let pixels = image_loader::process_image(primary_image)?;
        let emojy_bytes = emoji.as_bytes();
        let color_bytes = utils::hex_color_to_bytes(color);
        
        // 2) get entropy blob (raw pixels for MVP)
        let primary_entropy = entropy::entropy_by_frequency(&pixels, false, 16);            
        let color_salt = salt_from_color(&color_bytes);
        
        // 3) initial hash (Blake2b-512)
        let master_seed = Self::derive_master_seed(&primary_entropy, &emojy_bytes, &color_salt);

        // 4) hash for encryption key
        // let key_encryption_key = hash::blake2b_hash(&[salt_entropy, color_salt].concat());    
        let cipher_key = Self::derive_cipher_key(&master_seed);

        Ok(KeyMaterial::new(master_seed, cipher_key))
    } 

    pub fn derive_master_seed(
        primary_entropy: &[u8],
        emoji_bytes: &[u8],
        color_salt: &[u8],
    ) -> [u8; 48] {

        let intermediate = blake2b_hash(&[primary_entropy, emoji_bytes].concat());
        let argon2 = Argon2::default();
        let mut master_seed = [0u8; 48];
        argon2
            .hash_password_into(&intermediate, color_salt, &mut master_seed)
            .expect("Argon2 hashing failed");
        master_seed
    }

    pub fn derive_cipher_key(
        seed: &[u8]
    ) -> [u8; 64] {
        blake2b_hash(seed)
    }
    
    // Альтернативный конструктор - если уже есть готовые компоненты
    // pub fn from_entropy_components(
    //     primary_entropy: &[u8],
    //     salt_entropy: &[u8], 
    //     color_salt: &[u8],
    // ) -> Result<KeyMaterial> {
    //     let master_seed = Self::derive_master_seed(primary_entropy, salt_entropy, color_salt);
    //     let cipher_key = Self::derive_cipher_key(salt_entropy, color_salt);
        
    //     Ok(KeyMaterial::new(master_seed, cipher_key))
    // }
}