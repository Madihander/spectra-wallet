pub mod crypto;
pub mod key_derivation;
pub mod wallet;

pub use key_derivation::KeyMaterial;
pub use wallet::{Wallet, WalletBuilder};
pub use crypto::aes_encryptor::aes256_decrypt_private_key;