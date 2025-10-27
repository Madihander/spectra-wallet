// seed_color_generator.rs
//
// Transformations:
//  - master_seed (bytes) -> Vec<"#RRGGBB"> (num_colors) + appended salt_color
//  - Vec<"#RRGGBB"> (first N) -> master_seed (bytes)  (and extracts salt_color as the last element.)
//
// Important: for reversibility, the length of the master_seed MUST == num_colors * 3.
// If this is not the case, the function returns an error. This provides a simple, unambiguous and secure scheme.

use std::fmt;

#[derive(Debug)]
pub enum SeedColorError {
    InvalidSeedLength { got: usize, required: usize },
    InvalidColorFormat(String),
    InvalidHex(String),
    TooFewColors { got: usize, min: usize },
}

impl fmt::Display for SeedColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SeedColorError::InvalidSeedLength { got, required } => {
                write!(
                    f,
                    "invalid master_seed length: got {} bytes, required {} bytes (must be 3 * num_colors)",
                    got, required
                )
            }
            SeedColorError::InvalidColorFormat(s) => {
                write!(f, "invalid color format (expecting #RRGGBB): {}", s)
            }
            SeedColorError::InvalidHex(s) => {
                write!(f, "invalid hex in color: {}", s)
            }
            SeedColorError::TooFewColors { got, min } => {
                write!(f, "too few colors: got {}, need at least {}", got, min)
            }
        }
    }
}

impl std::error::Error for SeedColorError {}

/// Converts master_seed -> цвета.
/// - `master_seed` length must equal `num_colors * 3`.
/// - `salt_color` should be in the format "#RRGGBB" (will be added to the end of the result).
/// Returns a vector of strings of the form "#RRGGBB" with a length of num_colors+1 (last element = salt_color).
pub fn master_seed_to_colors(
    master_seed: &[u8],
    num_colors: usize,
    salt_color: &str,
) -> Result<Vec<String>, SeedColorError> {
    let required = num_colors * 3;
    if master_seed.len() != required {
        return Err(SeedColorError::InvalidSeedLength {
            got: master_seed.len(),
            required,
        });
    }

    // Checking the salt_color format
    if !is_valid_hex_color(salt_color) {
        return Err(SeedColorError::InvalidColorFormat(salt_color.to_string()));
    }

    let mut colors: Vec<String> = Vec::with_capacity(num_colors + 1);
    for i in 0..num_colors {
        let off = i * 3;
        let r = master_seed[off];
        let g = master_seed[off + 1];
        let b = master_seed[off + 2];
        // formatting in "#RRGGBB"
        colors.push(format!("#{:02x}{:02x}{:02x}", r, g, b));
    }

    // We add salt_color as the last element (for example, the 17th)
    colors.push(salt_color.to_lowercase());

    Ok(colors)
}

/// Restores the master_seed and salt_color from the color sequence.
/// - The last element is considered salt_color.
/// - Returns (master_seed_bytes, salt_color_string).
/// - The number of colors must be >= 2 (at least 1 color + 1 salt_color).
pub fn colors_to_master_seed(
    colors: &[String],
) -> Result<(Vec<u8>, String), SeedColorError> {
    if colors.len() < 2 {
        return Err(SeedColorError::TooFewColors {
            got: colors.len(),
            min: 2,
        });
    }

    // The last one is salt
    let salt_color = colors.last().unwrap().to_lowercase();

    if !is_valid_hex_color(&salt_color) {
        return Err(SeedColorError::InvalidColorFormat(salt_color));
    }

    // The rest are seed bytes
    let num_seed_colors = colors.len() - 1;
    let mut seed_bytes: Vec<u8> = Vec::with_capacity(num_seed_colors * 3);

    for c in &colors[..num_seed_colors] {
        if !is_valid_hex_color(c) {
            return Err(SeedColorError::InvalidColorFormat(c.clone()));
        }
        // c is "#RRGGBB"
        let hex = &c[1..]; // skip '#'
        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|_| SeedColorError::InvalidHex(c.clone()))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|_| SeedColorError::InvalidHex(c.clone()))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|_| SeedColorError::InvalidHex(c.clone()))?;
        seed_bytes.push(r);
        seed_bytes.push(g);
        seed_bytes.push(b);
    }

    Ok((seed_bytes, salt_color))
}

/// Utility: checking whether the string matches the format "#RRGGBB" (6 hex characters)
fn is_valid_hex_color(s: &str) -> bool {
    if s.len() != 7 { return false; }
    let bytes = s.as_bytes();
    if bytes[0] != b'#' { return false; }
    s[1..].chars().all(|c| c.is_digit(16))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip_basic() {
        // Creating a seed of length 3 * 4 = 12 bytes => 4 colors
        let seed: Vec<u8> = vec![
            0x12, 0x34, 0x56, // color1
            0xab, 0xcd, 0xef, // color2
            0x00, 0x11, 0x22, // color3
            0xff, 0x80, 0x01, // color4
        ];
        let salt = "#FF5733";
        let colors = master_seed_to_colors(&seed, 4, salt).expect("to colors");
        assert_eq!(colors.len(), 5);
        assert_eq!(colors[4], salt.to_lowercase());

        let (recovered_seed, recovered_salt) = colors_to_master_seed(&colors).expect("from colors");
        assert_eq!(recovered_seed, seed);
        assert_eq!(recovered_salt, salt.to_lowercase());
    }

    #[test]
    fn invalid_seed_len() {
        let seed: Vec<u8> = vec![1,2,3,4,5]; // not multiple of 3* num_colors
        let res = master_seed_to_colors(&seed, 2, "#112233");
        assert!(res.is_err());
    }

    #[test]
    fn invalid_color_format() {
        let bad = vec!["notcolor".to_string(), "#112233".to_string()];
        let res = colors_to_master_seed(&bad);
        assert!(res.is_err());
    }
}
