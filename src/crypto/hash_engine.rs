use blake2::{Blake2b512, Blake2s256, Digest};
// Return fixed 64-byte array
pub fn blake2b_hash(data: &[u8]) -> [u8;64] {
    let mut hasher = Blake2b512::new();
    hasher.update(data);
    let result = hasher.finalize();
    let mut arr = [0u8; 64];
    arr.copy_from_slice(&result);
    return arr;
}

pub fn salt_from_color(color_salt: &[u8]) -> [u8; 16] {
    let mut hasher = Blake2s256::new();
    hasher.update(color_salt);
    let result = hasher.finalize();
    let mut arr = [0u8; 16];
    arr.copy_from_slice(&result[..16]);
    arr
}


// Return Vec<u8> 64 bytes
pub fn blake2b_hash_vec(data: &[u8]) -> Vec<u8> {
    blake2b_hash(data).to_vec()
}