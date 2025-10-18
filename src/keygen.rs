use ed25519_dalek::{SigningKey, VerifyingKey};
use anyhow::Result;

fn print_type<T>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn generate_ed25519(seed: &[u8; 64]) -> Result<(SigningKey, VerifyingKey)> {
    // Берем первые 32 байта в качестве seed
    let seed_array: [u8; 32] = seed[0..32].try_into()
        .map_err(|_| anyhow::anyhow!("Failed to convert slice to array"))?;
    
    let secret = SigningKey::from_bytes(&seed_array);
    let public = VerifyingKey::from(&secret);
    Ok((secret, public))
}