#[derive(Debug)]
pub struct Wallet {
    pub encrypted_private_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: String,
    pub type_blockchain: String,
}

impl Wallet {
    pub fn new(encrypted_private_key: Vec<u8>, public_key: Vec<u8>, address: String, type_blockchain: String) -> Self {
        Self {encrypted_private_key, public_key, address, type_blockchain}
    }

    pub fn get_bytes_encrypted_sec(&self) -> &[u8] {
        &self.encrypted_private_key
    }

    pub fn get_bytes_pub(&self) -> &[u8] {
        &self.public_key
    }

    pub fn get_hex_encrypted_sec(&self) -> String {
        hex::encode(&self.encrypted_private_key)
    }

    pub fn get_hex_pub(&self) -> String {
        hex::encode(&self.public_key)
    }

    pub fn get_address(&self) -> &str {
        &self.address
    }

    pub fn get_type_blockchain(&self) -> &str {
        &self.type_blockchain
    }
}

