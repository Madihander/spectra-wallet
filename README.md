# Spectra Wallet

**A visual wallet for Web3, where your seed phrase is art.**

Spectra Wallet is an innovative approach to security and user experience in Web3. We are removing barriers in the form of long seed phrases and complex passwords, replacing them with something that the human brain remembers better and faster — visual images.

# Submission to 2025 Solana Colosseum Submission by:

Madi Kazhibekov:
- Github: https://github.com/Madihander
- Linkein: linkedin.com/in/madi-kazhibekov-2b87a62bb
- Telegram: @madihander

# Resources

  
# Problem and Solution
### The Problem With Traditional Wallets
When creating a regular Web3 wallet, the user does not control the process of generating the master key. The seed phrase is created automatically, leaving him with only the illusion of control through complex passwords.

The main disadvantages of this approach are:
- **Memorization difficulty**: 12/24 random words are difficult to keep in mind
- **Heavy passwords**: Complex passwords that are required for security are just as difficult to remember and are often lost.
- **Media dependence**: The constant need to store seed phrases and passwords on paper or in password managers
  
### Our Solution: Visual Access
We are radically changing the security paradigm, making the process intuitive and human-oriented.
Instead of random words and complex passwords, we use a unique combination of three visual elements.:
- 🖼️ **Image** - any picture that matters to you
- 😊 **Emoji** is a symbol that you will easily remember
- 🎨 **Color** is your favorite shade from the palette .

The advantages of this approach are that:
- **Full control**: You choose the components of the master key yourself.
- **Easy to remember**: Visual images are stored in memory better than text and complex passwords.
- **Personal Connection**: Your wallet becomes a unique digital artifact
- **Easy entry**: You use the same image, emoji, and color to access it - without passwords.

The solution to the problem of losing input data is a **Seed Colors**
- 16 colors instead of 24 words - visually easier to remember
- The color sequence is easier to reproduce mentally.
- Fast recovery through intuitive color interface

After recovery:
- You can update the visual data (image, emoji and color)
- Continue to use a convenient visual input

# Summary of Submission Features
Offline CLI tool
# Tech Stack
Core Language: Rust

Cryptographic Libraries:
  - Hashing: Blake2
  - KDF: Argon2
  - Encryption: AES
  - Cryptography: ed25519-dalek, secp256k1
Security: Zeroize for secure memory wiping
# Architecture
``` bash
src/
├── lib.rs                 # Library entry point
├── main.rs               # CLI entry point
├── cli.rs                # Command-line interface logic
│
├── crypto/               # Cryptographic operations
│   ├── mod.rs
│   ├── aes_encryptor.rs  # AES encryption/decryption
│   └── hash_engine.rs    # Hashing algorithms (Blake2)
│
├── key_derivation/       # Visual key generation system
│   ├── mod.rs
│   ├── entropy.rs        # Entropy collection and management
│   ├── image_loader.rs   # Image processing and analysis
│   ├── key_deriver.rs    # Main key derivation logic
│   └── utils.rs          # Helper functions
│
├── recovery/             # Backup and recovery system
│   ├── mod.rs
│   ├── emoji_finders.rs  # Emoji processing and validation
│   ├── image_generator.rs # Visual seed generation
│   └── seed_color_generator.rs # 16-color seed generation
│
└── wallet/               # Wallet management
    ├── mod.rs
    ├── wallet.rs         # Main wallet structure and logic
    ├── wallet_builder.rs # Wallet creation and recovery
    └── signer.rs         # Transaction signing operations
```
### Core Modules Overview

- **`crypto/`** - Low-level cryptographic primitives
- **`key_derivation/`** - Transforms visual data into cryptographic keys
- **`recovery/`** - Handles backup systems (seed colors) and visual recovery
- **`wallet/`** - High-level wallet management and operations
- **`cli.rs`** - User interface and command parsing
### Data Flow
1. **Visual Input** → `key_derivation/` → **Master Seed**
2. **Master Seed** → `wallet/` → **Wallet Instance**
3. **Recovery** → `recovery/` → **Seed Colors** ↔ **Wallet Access** 
# Quick start
To build Rust library:
```bash
cargo build
cargo run
```
Or If you don't want to set up the project locally, you can build and run directly in GitHub Codespaces.

Visual demo:
https://time-volt-83656013.figma.site/
