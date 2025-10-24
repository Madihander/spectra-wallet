
use anyhow::{anyhow, Result};
use ed25519_dalek::{SigningKey, VerifyingKey};
use secp256k1::{Secp256k1, SecretKey, PublicKey};
use base58::{ToBase58};
use sha2::{Sha256, Digest};
use ripemd::{Ripemd160};
use tiny_keccak::{Keccak, Hasher};

use crate::crypto::aes_encryptor::aes256_encrypt_private_key;

use super::Wallet;
pub struct WalletBuilder;

impl WalletBuilder {
    pub fn generate(master_seed: &[u8; 64], encryption_key: &[u8; 64], blockchain: &str) -> Result<Wallet> {
        match blockchain.to_lowercase().as_str() {
            "solana" => Self::generate_solana(master_seed, encryption_key),
            "ethereum" => Self::generate_ethereum(master_seed, encryption_key),
            "bitcoin" => Self::generate_bitcoin(master_seed, encryption_key),
            _ => Err(anyhow!("Unsupported blockchain: {}", blockchain))
        }
    }

    fn generate_solana(seed: &[u8; 64], encryption_key: &[u8; 64]) -> Result<Wallet> {
        let seed_array: [u8; 32] = seed[0..32].try_into()
            .map_err(|_| anyhow::anyhow!("Failed to convert slice to array"))?;
    
        let (public_key, encrypted_private_key) = {
            let private_key = SigningKey::from_bytes(&seed_array);
            let public_key = VerifyingKey::from(&private_key);
            
            let private_key_bytes = private_key.to_bytes();
            let encrypted_private_key = aes256_encrypt_private_key(&private_key_bytes, encryption_key);
            // 
            let mut sensetive_date = private_key_bytes;
            zeroize::Zeroize::zeroize(&mut sensetive_date);
            
            (public_key,  encrypted_private_key)

        };

        let address = Self::generate_solana_address(&public_key);
        let type_blockchain = String::from("solana");
        // let encrypted_private_key = aes256_encrypt_private_key(&private_key.to_bytes(), &encryption_key);

        Ok(Wallet::new( 
            encrypted_private_key.to_vec(),
            public_key.to_bytes().to_vec(),
            address,
            type_blockchain
        ))
    }

    fn generate_ethereum(seed: &[u8; 64], encryption_key: &[u8; 64]) -> Result<Wallet> {
        let secp = Secp256k1::new();
        let seed_array: [u8; 32] = seed[0..32].try_into()
            .map_err(|_| anyhow!("Failed to convert slice to array"))?;
        let private_key = SecretKey::from_byte_array(seed_array)?;
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let address = Self::generate_ethereum_address(&public_key);
        let type_blockchain = String::from("ethereum");
        let encrypted_private_key = aes256_encrypt_private_key(&private_key.secret_bytes(), &encryption_key);

        Ok(Wallet::new(
            encrypted_private_key.to_vec(),
            public_key.serialize_uncompressed().to_vec(),
            address,
            type_blockchain
        ))
    }

    fn generate_bitcoin(seed: &[u8; 64], encryption_key: &[u8; 64]) -> Result<Wallet> {
        let secp = Secp256k1::new();
        let seed_array: [u8; 32] = seed[32..64].try_into()
            .map_err(|_| anyhow!("Failed to convert slice to array"))?;
        let private_key = SecretKey::from_byte_array(seed_array)?;
        let public_key = PublicKey::from_secret_key(&secp, &private_key);
        let address = Self::generate_bitcoin_address(&public_key);
        let type_blockchain = String::from("bitcoin");
        let encrypted_private_key = aes256_encrypt_private_key(&private_key.secret_bytes(), &encryption_key);


        Ok(Wallet::new(
            encrypted_private_key.to_vec(),
            public_key.serialize_uncompressed().to_vec(),
            address,
            type_blockchain
        ))
    }


    fn generate_solana_address(public_key: &ed25519_dalek::VerifyingKey) -> String {
        public_key.as_bytes().to_base58()
    }

    fn generate_ethereum_address(public_key: &secp256k1::PublicKey) -> String {
        let serialized = public_key.serialize_uncompressed();
        let public_key_bytes = &serialized[1..];
        
        let mut hasher = Keccak::v256();
        hasher.update(public_key_bytes);
        let mut hash = [0u8; 32];
        hasher.finalize(&mut hash);
    
        format!("0x{}", hex::encode(&hash[12..]))
    }

    fn generate_bitcoin_address(public_key: &secp256k1::PublicKey) -> String {
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

}

