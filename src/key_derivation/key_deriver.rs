use anyhow::Result;
use std::path::PathBuf;
use serde::{Serialize, Deserialize};

use argon2::{Argon2};
use super::image_loader;
use super::entropy;
use crate::crypto::{blake2b_hash, salt_from_color};
use super::utils;
use crate::recovery::{master_seed_to_colors, colors_to_master_seed};

use zeroize::Zeroize;

#[derive(Debug, Clone)]
pub struct KeyMaterial {
    pub master_seed: [u8; 48],        // master seed (deterministic)
    pub cipher_key: Option<[u8; 64]>, // Maybe None when restoring
    pub seed_colors: Vec<String>,
}

impl KeyMaterial {
    /// Creating new KeyMaterial from images, emojis, and colors
    pub fn generate(
        primary_image: &[u8],
        emoji: &str,
        color: &str,
    ) -> Result<Self> {
        // 1. Extracting the data
        let pixels = image_loader::process_image(primary_image)?;
        let emoji_bytes = emoji.as_bytes();
        let color_bytes = utils::hex_color_to_bytes(color);
        
        // 2. Generating entropy
        let primary_entropy = entropy::entropy_by_frequency(&pixels, false, 16);
        let color_salt = salt_from_color(&color_bytes);

        // 3. Generating master_seed via Argon2
        let master_seed = Self::derive_master_seed(&primary_entropy, emoji_bytes, &color_salt);

        // 4. Color seed generation
        let seed_colors = master_seed_to_colors(&master_seed, 16, color)?;

        // 5. Generating a key to encrypt a private key
        let cipher_key = Some(Self::derive_cipher_key(&pixels, emoji_bytes, &color_bytes));

        Ok(Self {
            master_seed,
            cipher_key,
            seed_colors
        })
    }

    /// Restoring master_seed from colors (without image)
    pub fn recover_from_colors(
        colors: &[String],
    ) -> Result<Self> {
        let (master_seed_vec, salt_color) = colors_to_master_seed(colors)?;
        let mut master_seed = [0u8; 48];
        master_seed.copy_from_slice(&master_seed_vec[..48]);

        Ok(Self {
            master_seed,
            cipher_key: None, // cipher_key will be created later when entering new data.
            seed_colors: colors.to_vec()
        })
    }

    /// After recovery, you can recreate cipher_key,
    /// by adding a new image and emoji
    pub fn regenerate_cipher_key(
        &mut self,
        primary_image: &[u8],
        emoji: &str,
    ) -> Result<()> {
        let pixels = image_loader::process_image(primary_image)?;
        let emoji_bytes = emoji.as_bytes();

        // The last color from seed_colors is our "color"
        let last_color = self.seed_colors.last().unwrap();
        let color_bytes = utils::hex_color_to_bytes(last_color);

        self.cipher_key = Some(Self::derive_cipher_key(&pixels, emoji_bytes, &color_bytes));
        Ok(())
    }

    pub fn derive_master_seed(
        primary_entropy: &[u8],
        emoji_bytes: &[u8],
        color_salt: &[u8],
    ) -> [u8; 48] {
        let intermediate = blake2b_hash(&[primary_entropy, emoji_bytes].concat());
        let argon2 = Argon2::default();
        let mut master_seed = [0u8; 48];
        argon2
            .hash_password_into(&intermediate, color_salt, &mut master_seed)
            .expect("Argon2 hashing failed");
        master_seed
    }

    pub fn derive_cipher_key(
        pixels: &[u8],
        emoji_bytes: &[u8],
        color_bytes: &[u8],
    ) -> [u8; 64] {
        let cipher_key = blake2b_hash(&[pixels, emoji_bytes, color_bytes].concat());
        cipher_key
    }

    pub fn get_master_seed(&self) -> &[u8; 48] {
        &self.master_seed
    }
    
    pub fn get_master_seed_hex(&self) -> String {
        hex::encode(self.master_seed)
    }

    pub fn get_cipher_key(&self) -> Option<&[u8; 64]> {
        self.cipher_key.as_ref()
    }

    pub fn get_cipher_key_hex(&self) -> Option<String> {
        self.cipher_key.as_ref().map(|k| hex::encode(k))
    }

    pub fn get_seed_colors(&self) -> &Vec<String> {
        &self.seed_colors
    }

    pub fn clear_seed_colors(&mut self) {
        for color in &mut self.seed_colors {
            color.zeroize(); // безопасно стирает содержимое строки
        }
        self.seed_colors.clear();
    }

}