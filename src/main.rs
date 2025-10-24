use anyhow::Result;
use clap::{Parser};
use std::path::PathBuf;

use mantishash::{KeyMaterial, Wallet, WalletBuilder, aes256_decrypt_private_key};

// mod keygen{
//     pub mod wallet_builder;
//     pub mod wallet;
//     pub mod aes_256_gcm;

// }
// use keygen::wallet_builder::{WalletBuilder};
// use keygen::wallet::{Wallet};
// use crate::keygen::aes_256_gcm;

// mod entropy;
// mod hash;
// mod image_loader;
// mod utils;
// mod key_deriver;
// use key_deriver::KeyMaterial;



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
    let key_material = KeyMaterial::derive_key_material(&args.image1, &args.image2, &args.color)?;

    let wallets: Vec<Wallet> = ["solana", "ethereum", "bitcoin"]
        .iter()
        .map(|&chain| WalletBuilder::generate(&key_material.master_seed, &key_material.cipher_key, chain))
        .collect::<Result<_>>()?;
    // }
    for (i, wallet) in wallets.iter().enumerate() {
        
        match aes256_decrypt_private_key(&wallet.get_bytes_encrypted_sec(), &key_material.cipher_key) {
            Ok(decrypted_bytes) => {
                let decrypted_private_key = hex::encode(&decrypted_bytes);
                
                println!("Blockchain {} :\n", wallet.get_type_blockchain());

                println!("Encrypted Secret Key: {:?} ##", wallet.get_hex_encrypted_sec());
                println!("Decrypted Secret Key: {:?} ##\n", decrypted_private_key);
                
                println!("Public Key: {} ##", wallet.get_hex_pub());
                println!("Address: {} ##\n", wallet.get_address());
            }
            Err(e) => {
                eprintln!("Decryption failed: {}", e);
            }
        }
    }
    Ok(())
}