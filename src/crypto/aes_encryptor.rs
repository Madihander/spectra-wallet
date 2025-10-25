use anyhow::{anyhow, Result};

use aes_gcm::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    Aes256Gcm, Key
};

pub fn aes256_encrypt_private_key(private_key: &[u8], encryption_key: &[u8; 64]) -> Result<Vec<u8>> {
    // take the first 32 bytes from the hash for the AES key
    let key = Key::<Aes256Gcm>::from_slice(&encryption_key[..32]);
    let cipher = Aes256Gcm::new(key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher.encrypt(&nonce, private_key)
        .expect("encryption failure");
    
    // Combining nonce and ciphertext for storage
    Ok([nonce.to_vec(), ciphertext].concat())
}

pub fn aes256_decrypt_private_key(encrypted_data: &[u8], encryption_key: &[u8; 64]) -> Result<Vec<u8>> {
    let key = Key::<Aes256Gcm>::from_slice(&encryption_key[..32]);
    let cipher = Aes256Gcm::new(key);
    
    // nonce is usually 12 bytes for AES-GCM
    let nonce = &encrypted_data[..12];
    let ciphertext = &encrypted_data[12..];
    let decrypted = cipher
        .decrypt(nonce.into(), ciphertext)
        .map_err(|e| anyhow!("decryption failure: {}", e))?;
    Ok(decrypted)
}