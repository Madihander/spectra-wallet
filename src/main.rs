use anyhow::Result;
use clap::{Parser};
use std::path::PathBuf;

use mantishash::{KeyMaterial, WalletBuilder, aes256_decrypt_private_key};

mod key_derivation; 
pub mod crypto;
mod recovery;
// use key_derivation::key_deriver::KeyMaterial;
use std::fs;


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
    
    let image_path = "img.png";
    let image2 = fs::read("img2.png").expect("Failed to read the image file");
    // println!("image2: {:#?}", image2);
    // Read the image file into a Vec<u8>
    let image_data = fs::read(image_path).expect("Failed to read the image file");
    let emoji:&str = "👻";
    let color = "#ff5733";

    let key_material = KeyMaterial::generate(&image_data, &emoji, &color)?;
    println!("KEY MATERIAL DERIVED SUCCESSFULLY!");
    println!("Master Seed: {}\n", hex::encode(key_material.master_seed));
    println!("Cipher Key: {:?}\n", hex::encode(key_material.cipher_key.unwrap()));


    let blockchain: &'static str = "solana";

    let wallet = WalletBuilder::generate_wallet(&key_material.master_seed, &key_material.cipher_key.unwrap(), blockchain)?;

    let transaction_data1 = "Sample transaction data";

    let decryption_key = &key_material.cipher_key.unwrap();
    let decrypted_private_key = aes256_decrypt_private_key(&wallet.get_bytes_encrypted_sec(), decryption_key)?;
    let signature = wallet.sign_transaction(decryption_key,transaction_data1.as_bytes())?;
    let is_valid = wallet.verify_signature(transaction_data1.as_bytes(), &signature)?;

    println!("Is the signature 1 valid? {}\n", is_valid);
    println!("Wallet Address: {}", wallet.get_address());
    println!("Wallet Encrypted Private Key: {}", wallet.get_hex_encrypted_sec());
    println!("Wallet Private Key: {}", hex::encode(decrypted_private_key));
    println!("Wallet Public Key: {}", wallet.get_hex_pub());
    
    let colors = key_material.get_seed_colors();
    let mut new_key_material = KeyMaterial::recover_from_colors(colors)?;
    new_key_material.regenerate_cipher_key(&image2, "👺")?;
    
    println!("\n---Recovered Key Material---\n");
    println!("Recovered Master Seed: {}\n", hex::encode(new_key_material.master_seed));
    print!("Recovered Cipher Key: {}\n", hex::encode(new_key_material.cipher_key.unwrap()));
    let wallet2 = WalletBuilder::generate_wallet(&new_key_material.master_seed, &new_key_material.cipher_key.unwrap(), blockchain)?;
    let decryption_key2 = &key_material.cipher_key.unwrap();
    let decrypted_private_key2 = aes256_decrypt_private_key(&wallet.get_bytes_encrypted_sec(), decryption_key2)?;
    let signature2 = wallet.sign_transaction(decryption_key,transaction_data1.as_bytes())?;
    let is_valid2 = wallet.verify_signature(transaction_data1.as_bytes(), &signature2)?;
    
    println!("\n---Recovered Wallet Info---\n");
    println!("Is the signature 2 valid? {}\n", is_valid2);
    println!("Wallet Address: {}", wallet2.get_address());
    println!("Wallet Encrypted Private Key: {}", wallet2.get_hex_encrypted_sec());
    println!("Wallet Private Key: {}", hex::encode(decrypted_private_key2));
    println!("Wallet Public Key: {}", wallet2.get_hex_pub());

    Ok(())

}