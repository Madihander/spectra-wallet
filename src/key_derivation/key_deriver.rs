use anyhow::Result;
use std::path::PathBuf;


use super::image_loader;
use super::utils;
use super::entropy;

use crate::crypto::hash_engine::blake2b_hash;

#[derive(Debug, Clone)]
pub struct KeyMaterial {
    pub master_seed: [u8; 64],           // Main Seed for key generation
    pub cipher_key: [u8; 64],    // The key for encrypting the private key
}
impl KeyMaterial {
    pub fn new(master_seed: [u8; 64], cipher_key: [u8; 64]) -> Self {
        Self { master_seed, cipher_key }
    }

    pub fn derive_key_material(
        primary_image:&PathBuf,
        salt_image: &PathBuf,
        color: &str,
    ) -> Result<KeyMaterial> {
        
        // 1) load image -> raw RGB bytes
        let pixels1 = image_loader::load_image(primary_image)?;
        let pixels2 = image_loader::load_image(salt_image)?;
        let color_salt = utils::hex_color_to_bytes(color);
            
        // 2) get entropy blob (raw pixels for MVP)
        let primary_entropy = entropy::entropy_by_frequency(&pixels1, false, 16);
        let salt_entropy = entropy::from_raw_pixels(&pixels2);
            
        // 3) initial hash (Blake2b-512)
        // let master_seed = hash::blake2b_hash(&[primary_entropy, salt_entropy.clone(), color_salt.clone()].concat());
        let master_seed = Self::derive_master_seed(&primary_entropy, &salt_entropy, &color_salt);

        // 4) hash for encryption key
        // let key_encryption_key = hash::blake2b_hash(&[salt_entropy, color_salt].concat());    
        let cipher_key = Self::derive_cipher_key(&salt_entropy, &color_salt);

        Ok(KeyMaterial::new(master_seed, cipher_key))
    } 

    pub fn derive_master_seed(
        primary_entropy: &[u8],
        salt_entropy: &[u8],
        color_salt: &[u8],
    ) -> [u8; 64] {
        blake2b_hash(&[primary_entropy, salt_entropy, color_salt].concat())
    }

    pub fn derive_cipher_key(
        salt_entropy: &[u8],
        color_salt: &[u8],
    ) -> [u8; 64] {
        blake2b_hash(&[salt_entropy, color_salt].concat())
    }
    
    /// Альтернативный конструктор - если уже есть готовые компоненты
    pub fn from_entropy_components(
        primary_entropy: &[u8],
        salt_entropy: &[u8], 
        color_salt: &[u8],
    ) -> Result<KeyMaterial> {
        let master_seed = Self::derive_master_seed(primary_entropy, salt_entropy, color_salt);
        let cipher_key = Self::derive_cipher_key(salt_entropy, color_salt);
        
        Ok(KeyMaterial::new(master_seed, cipher_key))
    }
}