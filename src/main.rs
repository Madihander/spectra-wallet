use anyhow::Result;
use clap::{ValueEnum, Parser};
use std::path::PathBuf;


mod keygen{
    pub mod generator;
    pub mod keypair;
}
use keygen::generator::{WalletGenerator};
use keygen::keypair::{KeyPair};


mod entropy;
mod hash;
mod image_loader;
mod utils;


fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

#[derive(Parser, Debug)]
#[command(name = "mantis_hash")]
#[command(about = "Generate cryptographic keys from images using visual entropy.")]

struct Args {
    // Path to image file
    #[arg(short, long)]
    image: PathBuf,

    // Optional password (if provided, Argon2id Will be used with image-hash as salt)
    #[arg(short, long)]
    password: Option<String>,

    // #[arg(short, long)]
    // blockchain: Option<String>,

    // Add random salt (non-deterministic generation)
    #[arg(long, default_value_t = false)]
    randomize: bool,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // 1) load image -> raw RGB bytes
    let pixels = image_loader::load_image(&args.image)?;

    // 2) get entropy blob (raw pixels for MVP)
    let entropy_blob = entropy::from_raw_pixels(&pixels);

    // 3) initial hash (Blake2b-512)
    let image_hash = hash::blake2b_hash(&entropy_blob); // 64 bytes Vec<u8>
    // println!("Length of image_hash: {}", image_hash.len());
    // println!("{:#?}", image_hash);
    // println!("TYPE OF IMAGE HASH");
    // print_type(&image_hash);

    let keypairs: Vec<KeyPair> = ["solana", "ethereum", "bitcoin"]
        .iter()
        .map(|&chain| WalletGenerator::generate(&image_hash, chain))
        .collect::<Result<_>>()?;
    // }
    for (i, keypair) in keypairs.iter().enumerate() {
        println!("KeyPair {} :",keypair.get_type_blockchain());
        println!("Secret Key: {} ##",keypair.get_hex_sec());
        println!("Public Key: {} ##",keypair.get_hex_pub());
        println!("Address: {} ##\n",keypair.get_address());
    }
    Ok(())
}