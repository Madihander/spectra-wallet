use ed25519_dalek::{SigningKey, VerifyingKey};
use anyhow::{anyhow,Result};
use secp256k1::{Secp256k1, SecretKey, PublicKey};

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn generate_ed25519(seed: &[u8; 64]) -> Result<(SigningKey, VerifyingKey)> {
    // Берем первые 32 байта в качестве seed
    let seed_array: [u8; 32] = seed[0..32].try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert slice to array"))?;
    
    let secret_key = SigningKey::from_bytes(&seed_array);
    let public_key = VerifyingKey::from(&secret_key);

    Ok((secret_key, public_key))
}

pub fn generate_secp256k1(seed: &[u8; 64]) -> Result<(SecretKey, PublicKey)> {
    let secp = Secp256k1::new();

    let seed_array: [u8; 32] = seed[0..32].try_into()
        .map_err(|_| anyhow!("Failed to convert slice to array"))?;
    let secret_key = SecretKey::from_byte_array(seed_array)?;
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);

    Ok((secret_key, public_key))
}

