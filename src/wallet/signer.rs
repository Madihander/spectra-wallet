use ed25519_dalek::{Signer, SigningKey, VerifyingKey};
use anyhow::{anyhow, Result};

pub struct TransactionSigner;

impl TransactionSigner {
    // Sign Solana transaction
    pub fn sign_solana_transaction(
        private_key: &[u8],
        transaction_data: &[u8],
    ) -> Result<Vec<u8>> {
        let private_key_array: [u8; 32] = private_key.try_into()
            .map_err(|_| anyhow::anyhow!("Invalid private key length for Solana, expected 32 bytes"))?;
        
        let signing_key = SigningKey::from_bytes(&private_key_array);
        let signature = signing_key.sign(transaction_data);
        
        let signature_bytes = signature.to_bytes().to_vec();

        Ok(signature_bytes)
    }
    // Verify Solana signature
    pub fn verify_solana_signature(
        public_key: &[u8],
        message: &[u8],
        signature: &[u8],
    ) -> Result<bool> {
        let public_key_array: [u8; 32] = public_key
            .try_into()
            .map_err(|_| anyhow!("Invalid public key length"))?;
        
        let verifying_key = VerifyingKey::from_bytes(&public_key_array)?;
    
        let sig_array: [u8; 64] = signature
            .try_into()
            .map_err(|_| anyhow!("Invalid signature length"))?;
        
        let signature = ed25519_dalek::Signature::from_bytes(&sig_array);
        
        Ok(verifying_key.verify_strict(message, &signature).is_ok())
    }
}