use anyhow::Result;
use clap::{Parser};
use std::path::PathBuf;

mod keygen{
    pub mod generator;
    pub mod keypair;
    pub mod aes_256_gcm;

}


use keygen::generator::{WalletGenerator};
use keygen::keypair::{KeyPair};
use crate::keygen::aes_256_gcm;

mod entropy;
mod hash;
mod image_loader;
mod utils;


fn print_type<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

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



#[derive(Parser, Debug)]
#[command(name = "mantis_hash")]
#[command(about = "Generate cryptographic keys from images using visual entropy.")]

struct Args {
    // Path to first image file
    #[arg(short = 'f', long = "first")]
    image1: PathBuf,

    // Path to second image file
    #[arg(short = 's', long = "second")]
    image2: PathBuf,

    #[arg(short = 'c', long = "color")]
    color: String, // HEX #ff1234

    #[arg(short = 'b', long = "blockchain", default_value = "solana")]
    blockchain:String,

}

fn main() -> Result<()> {
    let args = Args::parse();
    let wallet = generate_wallet_seeds(&args.image1, &args.image2, &args.color)?;


    // println!("{:?}", wallet.master_seed);
    // println!("{:?}", wallet.key_encryption_key);

    let keypairs: Vec<KeyPair> = ["solana", "ethereum", "bitcoin"]
        .iter()
        .map(|&chain| WalletGenerator::generate(&wallet.master_seed, &wallet.key_encryption_key, chain))
        .collect::<Result<_>>()?;
    // }
    for (i, keypair) in keypairs.iter().enumerate() {
        
        match aes_256_gcm::decrypt_private_key(&keypair.get_bytes_encrypted_sec(), &wallet.key_encryption_key) {
            Ok(decrypted_bytes) => {
                let decrypted_secret_key = hex::encode(&decrypted_bytes);
                
                println!("KeyPair {} :", keypair.get_type_blockchain());
                println!("Secret Key: {} ##\n", keypair.get_hex_sec());
                
                println!("Encrypted Secret Key: {:?} ##", keypair.get_hex_encrypted_sec());
                println!("Decrypted Secret Key: {:?} ##\n", decrypted_secret_key);
                
                println!("Public Key: {} ##", keypair.get_hex_pub());
                println!("Address: {} ##\n", keypair.get_address());
            }
            Err(e) => {
                eprintln!("Decryption failed: {}", e);
            }
        }
    }
    Ok(())
}