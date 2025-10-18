use anyhow::Result;
use clap::{ValueEnum, Parser};
use std::path::PathBuf;

mod entropy;
mod hash;
mod image_loader;
mod keygen;
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

    // use secp256k1 instead of ed25519
    #[arg(long, default_value_t = false)]
    secp256k1: bool,

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
    
    let (private_key, public_key) = keygen::generate_ed25519(&image_hash)?;

    println!("## Generated keys:");
    println!("Private Key: {}", hex::encode(private_key.as_bytes()));
    println!("Public  Key: {}", hex::encode(public_key.as_bytes()));
    // Private Key: 8188f85cac9111189b59db6b9b09ca0e497bb00f33af6da15e18c2043f0ac9a0
    // Public  Key: c7bea50ccd47af6f924718d46e16568b1a7d5b098d9e64e8eac7e73b5fecd9eb 
    Ok(())
}