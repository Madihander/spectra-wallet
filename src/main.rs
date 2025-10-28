use aes_gcm::aead::rand_core::le;
use anyhow::Result;
use clap::{Parser};
use std::{f32::consts::E, path::PathBuf};

use mantishash::{KeyMaterial, Wallet, WalletBuilder, hex_color_to_bytes, process_image};

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

    let image1 = fs::read("/img.png").expect("Failed to read the image file");    
    let image2 = fs::read("img2.png").expect("Failed to read the image file");
    
    let transaction_data1 = "Sample transaction data";
    let emoji:&str = "👻";
    let color = "#ff5733";
    let blockchain: &'static str = "solana";  
    
    let wallet = WalletBuilder::generate_wallet(image1, emoji,color, blockchain)?;
    let signature = wallet.sign_transaction(transaction_data1.as_bytes())?;
    let is_valid = wallet.verify_signature(transaction_data1.as_bytes(), &signature)?;

    Ok(())

}