use std::collections::HashMap;


pub fn from_raw_pixels(pixels: &[u8]) -> Vec<u8> {
    pixels.to_vec()
}

pub fn entropy_by_frequency(pixels: &[u8], use_rare: bool, top_n: usize) -> Vec<u8> {
    let mut freq = HashMap::new();
    for &pixel in pixels {
        *freq.entry(pixel).or_insert(0) += 1;
    }
    
    let mut sorted: Vec<_> = freq.into_iter().collect();
    sorted.sort_by_key(|(_, count)| if use_rare { *count } else { -*count });
    
    sorted.iter().take(top_n).map(|(pixel, _)| *pixel).collect()
}

