use anyhow::Result;
use clap::{Parser};
use std::{path::PathBuf};

use mantishash::{WalletBuilder};

mod key_derivation; 
pub mod crypto;
mod recovery;
mod cli;
// use key_derivation::key_deriver::KeyMaterial;

#[derive(Parser, Debug)]
#[command(name = "mantis_hash")]
#[command(about = "Generate cryptographic keys from images using visual entropy.")]

struct Args {
    // Path to first image file
    #[arg(short = 'f', long = "first")]
    image1: PathBuf,

    #[arg(short = 'e', long = "emoji")]
    emoji: String, // HEX #ff1234

    #[arg(short = 'c', long = "color")]
    color: String, // HEX #ff1234

    #[arg(short = 'b', long = "blockchain", default_value = "solana")]
    blockchain:String,

}

fn main() -> Result<()> {
    cli::run_cli();
    // let args = Args::parse();

    // // let image1 = fs::read("/img.png").expect("Failed to read the image file");    
    // // let image2 = fs::read("img2.png").expect("Failed to read the image file");
    
    // let transaction_data1 = "Sample transaction data";
    // // let emoji:&str = "🤡";
    // // let emoji2 = "🚀";
    // // let blockchain: &'static str = "solana";  

    // let wallet = WalletBuilder::generate_wallet(&args.image1, &args.emoji,&args.color, &args.blockchain)?;
    // let signature = wallet.sign_transaction(transaction_data1.as_bytes())?;
    // let is_valid = wallet.verify_signature(transaction_data1.as_bytes(), &signature)?;
    // println!("Is the signature valid? {}", is_valid);

    Ok(())

}