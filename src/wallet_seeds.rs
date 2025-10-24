use anyhow::Result;
use std::path::PathBuf;
use crate::image_loader;
use crate::utils;
use crate::entropy;
use crate::hash;

#[derive(Debug, Clone)]
pub struct WalletSeeds {
    pub master_seed: [u8; 64],           // Main Seed for key generation
    pub key_encryption_key: [u8; 64],    // The key for encrypting the private key
}
impl WalletSeeds {
    pub fn new(master_seed: [u8; 64], key_encryption_key: [u8; 64]) -> Self {
        Self { master_seed, key_encryption_key }
    }
}

pub fn generate_wallet_seeds(
    primary_image:&PathBuf,
    salt_image: &PathBuf,
    color: &str,
) -> Result<WalletSeeds> {
    // 1) load image -> raw RGB bytes
    let pixels1 = image_loader::load_image(primary_image)?;
    let pixels2 = image_loader::load_image(salt_image)?;
    let color_salt = utils::hex_color_to_bytes(color);
        
    // 2) get entropy blob (raw pixels for MVP)
    let primary_entropy = entropy::entropy_by_frequency(&pixels1, false, 16);
    let salt_entropy = entropy::from_raw_pixels(&pixels2);
        
    // 3) initial hash (Blake2b-512)
    let master_seed = hash::blake2b_hash(&[primary_entropy, salt_entropy.clone(), color_salt.clone()].concat());
            
    // 4) hash for encryption key
    let key_encryption_key = hash::blake2b_hash(&[salt_entropy, color_salt].concat());    
    
    let walletseeds = WalletSeeds::new(master_seed, key_encryption_key);
    Ok(WalletSeeds::new(master_seed, key_encryption_key))
    }