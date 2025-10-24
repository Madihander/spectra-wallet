# MantisHash
A Rust CLI application that generates a cryptographic key pair and wallet address using an image as an entropy source.
The user provides the path to the image - the program extracts the color palette, converts it into a deterministic hash, and generates a private key, public key, and address via KDF.

### How to run
Example: cargo run -- --first ./img.png --second ./img.png --color '#ff5733' 
