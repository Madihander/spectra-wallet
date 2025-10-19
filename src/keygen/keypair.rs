#[derive(Debug)]
pub struct KeyPair {
    pub secret_key: Vec<u8>,
    pub public_key: Vec<u8>,
    pub address: String,
    pub type_blockchain: String,
}

impl KeyPair {
    pub fn new(secret_key: Vec<u8>, public_key: Vec<u8>, address: String, type_blockchain: String) -> Self {
        Self {secret_key, public_key, address, type_blockchain}
    }

    pub fn get_bytes_sec(&self) -> &[u8] {
        &self.secret_key
    }

    pub fn get_bytes_pub(&self) -> &[u8] {
        &self.public_key
    }

    pub fn get_hex_sec(&self) -> String {
        hex::encode(&self.secret_key)
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

