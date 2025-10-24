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
mod wallet_seeds;

fn print_type<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>());
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
    let wallet = wallet_seeds::generate_wallet_seeds(&args.image1, &args.image2, &args.color)?;


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