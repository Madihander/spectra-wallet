use rand::RngCore;

// Generate random byte using OS RNG
pub fn random_bytes(n: usize) -> Vec<u8> {
    let mut  v = vec![0u8; n];
    let mut rng = rand::rngs::OsRng;
    rng.fill_bytes(&mut v);
    v
}