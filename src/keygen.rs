use ed25519_dalek::{SigningKey, VerifyingKey};
use anyhow::{anyhow,Result};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use base58::{ToBase58, FromBase58};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160, Digest as RipemdDigest};
use tiny_keccak::{Keccak, Hasher};

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

pub fn generate_solana_address(public_key: &ed25519_dalek::VerifyingKey) -> String {
    public_key.as_bytes().to_base58()
}

pub fn generate_ethereum_address(public_key: &secp256k1::PublicKey) -> String {
    let serialized = public_key.serialize_uncompressed();
    let public_key_bytes = &serialized[1..];
    
    let mut hasher = Keccak::v256();
    hasher.update(public_key_bytes);
    let mut hash = [0u8; 32];
    hasher.finalize(&mut hash);

    format!("0x{}", hex::encode(&hash[12..]))
}

pub fn generate_bitcoin_address(public_key: &secp256k1::PublicKey) -> String {
    let serialized = public_key.serialize_uncompressed();
    let public_key_bytes = &serialized[1..];

    let mut sha256 = Sha256::new();
    sha256.update(public_key_bytes);
    let sha256_result = sha256.finalize();

    let mut ripemd160 = Ripemd160::new();
    ripemd160.update(sha256_result);
    let ripemd160_result = ripemd160.finalize();

    // add network byte
    let mut with_network_byte = vec![0x00];
    with_network_byte.extend_from_slice(&ripemd160_result);

    let mut sha256_checksum1 = Sha256::new();
    sha256_checksum1.update(&with_network_byte);
    let checksum1 = sha256_checksum1.finalize();

    let mut sha256_checksum2 = Sha256::new();
    sha256_checksum2.update(checksum1);
    let final_checksum = sha256_checksum2.finalize();
    
    with_network_byte.extend_from_slice(&final_checksum[0..4]);

    with_network_byte.to_base58()
}