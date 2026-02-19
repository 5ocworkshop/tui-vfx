// <FILE>crates/tui-vfx-types/src/braille.rs</FILE> - <DESC>Braille character utilities for visual effects</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Document braille conversion safety invariant</WCTX>
// <CLOG>Clarify unsafe braille conversion bounds</CLOG>

//! Braille character utilities for visual effects.
//!
//! Unicode braille patterns (U+2800–U+28FF) use a 2×4 dot grid:
//!
//! ```text
//! ┌───┐
//! │1 4│   Dot positions (1-8)
//! │2 5│   Left column:  1, 2, 3, 7
//! │3 6│   Right column: 4, 5, 6, 8
//! │7 8│
//! └───┘
//! ```
//!
//! The Unicode codepoint is `U+2800 + bits` where each bit corresponds to a dot:
//! - bit 0 = dot 1, bit 1 = dot 2, ..., bit 7 = dot 8
//!
//! This gives 256 possible patterns (2^8), from empty (U+2800) to full (U+28FF).

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Region masks - combine with & to filter patterns
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Left column dots (1, 2, 3, 7) - bits 0, 1, 2, 6
pub const LEFT_COLUMN: u8 = 0b0100_0111;

/// Right column dots (4, 5, 6, 8) - bits 3, 4, 5, 7
pub const RIGHT_COLUMN: u8 = 0b1011_1000;

/// Top row dots (1, 4) - bits 0, 3
pub const TOP_ROW: u8 = 0b0000_1001;

/// Bottom row dots (7, 8) - bits 6, 7
pub const BOTTOM_ROW: u8 = 0b1100_0000;

/// Upper half dots (1, 2, 4, 5) - bits 0, 1, 3, 4
pub const UPPER_HALF: u8 = 0b0001_1011;

/// Lower half dots (3, 6, 7, 8) - bits 2, 5, 6, 7
pub const LOWER_HALF: u8 = 0b1110_0100;

/// All dots
pub const ALL_DOTS: u8 = 0b1111_1111;

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Core conversion
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Convert a bit pattern to a braille character.
///
/// All 256 possible u8 values map to valid braille characters.
///
/// # Example
/// ```
/// use tui_vfx_types::braille::braille;
/// assert_eq!(braille(0b0000_0001), '⠁'); // dot 1 only
/// assert_eq!(braille(0b0000_1001), '⠉'); // dots 1 and 4 (top corners)
/// assert_eq!(braille(0b1111_1111), '⣿'); // all 8 dots
/// ```
#[inline]
pub fn braille(bits: u8) -> char {
    // The Braille Patterns Unicode block is U+2800..U+28FF.
    // 0x2800 + bits (0-255) covers this range, which are all valid scalar values.
    char::from_u32(0x2800 + bits as u32).expect("valid braille char")
}

/// Convert a braille character back to its bit pattern.
///
/// Returns `None` if the character is not a braille pattern (U+2800–U+28FF).
///
/// # Example
/// ```
/// use tui_vfx_types::braille::braille_bits;
/// assert_eq!(braille_bits('⠁'), Some(0b0000_0001));
/// assert_eq!(braille_bits('A'), None);
/// ```
#[inline]
pub fn braille_bits(ch: char) -> Option<u8> {
    let code = ch as u32;
    if (0x2800..=0x28FF).contains(&code) {
        Some((code - 0x2800) as u8)
    } else {
        None
    }
}

/// Create a braille character from dot numbers (1-8).
///
/// Invalid dot numbers (0, >8) are silently ignored.
///
/// # Example
/// ```
/// use tui_vfx_types::braille::from_dots;
/// assert_eq!(from_dots(&[1]), '⠁');       // dot 1
/// assert_eq!(from_dots(&[1, 4]), '⠉');    // top corners
/// assert_eq!(from_dots(&[1, 2, 3, 7]), '⡇'); // left column
/// assert_eq!(from_dots(&[]), '⠀');        // empty
/// ```
pub fn from_dots(dots: &[u8]) -> char {
    let mut bits: u8 = 0;
    for &dot in dots {
        if (1..=8).contains(&dot) {
            bits |= 1 << (dot - 1);
        }
    }
    braille(bits)
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Properties
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Count the number of dots in a bit pattern.
#[inline]
pub fn dot_count(bits: u8) -> u8 {
    bits.count_ones() as u8
}

/// Check if a specific dot is set (dot numbers 1-8).
#[inline]
pub fn has_dot(bits: u8, dot: u8) -> bool {
    if (1..=8).contains(&dot) {
        bits & (1 << (dot - 1)) != 0
    } else {
        false
    }
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Pre-computed pattern sets by dot count
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// All 8 single-dot patterns.
pub const PATTERNS_1: [u8; 8] = [0x01, 0x02, 0x04, 0x08, 0x10, 0x20, 0x40, 0x80];

/// All 28 two-dot patterns (C(8,2) = 28).
pub const PATTERNS_2: [u8; 28] = [
    0x03, 0x05, 0x06, 0x09, 0x0A, 0x0C, 0x11, 0x12, 0x14, 0x18, 0x21, 0x22, 0x24, 0x28, 0x41, 0x42,
    0x44, 0x48, 0x30, 0x50, 0x60, 0x90, 0xA0, 0xC0, 0x81, 0x82, 0x84, 0x88,
];

/// Returns a slice of all bit patterns with exactly n dots.
///
/// - n=0: 1 pattern (empty)
/// - n=1: 8 patterns
/// - n=2: 28 patterns
/// - n=3: 56 patterns
/// - n=4: 70 patterns
/// - n=5: 56 patterns
/// - n=6: 28 patterns
/// - n=7: 8 patterns
/// - n=8: 1 pattern (full)
///
/// Returns empty slice for n > 8.
pub fn patterns_with_count(n: u8) -> &'static [u8] {
    match n {
        0 => &PATTERNS_0,
        1 => &PATTERNS_1,
        2 => &PATTERNS_2,
        3 => &PATTERNS_3,
        4 => &PATTERNS_4,
        5 => &PATTERNS_5,
        6 => &PATTERNS_6,
        7 => &PATTERNS_7,
        8 => &PATTERNS_8,
        _ => &[],
    }
}

// Pre-computed arrays for all dot counts
const PATTERNS_0: [u8; 1] = [0x00];
const PATTERNS_8: [u8; 1] = [0xFF];

// 7-dot patterns (complement of 1-dot)
const PATTERNS_7: [u8; 8] = [0xFE, 0xFD, 0xFB, 0xF7, 0xEF, 0xDF, 0xBF, 0x7F];

// 3-dot patterns (C(8,3) = 56)
const PATTERNS_3: [u8; 56] = [
    0x07, 0x0B, 0x0D, 0x0E, 0x13, 0x15, 0x16, 0x19, 0x1A, 0x1C, 0x23, 0x25, 0x26, 0x29, 0x2A, 0x2C,
    0x31, 0x32, 0x34, 0x38, 0x43, 0x45, 0x46, 0x49, 0x4A, 0x4C, 0x51, 0x52, 0x54, 0x58, 0x61, 0x62,
    0x64, 0x68, 0x70, 0x83, 0x85, 0x86, 0x89, 0x8A, 0x8C, 0x91, 0x92, 0x94, 0x98, 0xA1, 0xA2, 0xA4,
    0xA8, 0xB0, 0xC1, 0xC2, 0xC4, 0xC8, 0xD0, 0xE0,
];

// 4-dot patterns (C(8,4) = 70)
const PATTERNS_4: [u8; 70] = [
    0x0F, 0x17, 0x1B, 0x1D, 0x1E, 0x27, 0x2B, 0x2D, 0x2E, 0x33, 0x35, 0x36, 0x39, 0x3A, 0x3C, 0x47,
    0x4B, 0x4D, 0x4E, 0x53, 0x55, 0x56, 0x59, 0x5A, 0x5C, 0x63, 0x65, 0x66, 0x69, 0x6A, 0x6C, 0x71,
    0x72, 0x74, 0x78, 0x87, 0x8B, 0x8D, 0x8E, 0x93, 0x95, 0x96, 0x99, 0x9A, 0x9C, 0xA3, 0xA5, 0xA6,
    0xA9, 0xAA, 0xAC, 0xB1, 0xB2, 0xB4, 0xB8, 0xC3, 0xC5, 0xC6, 0xC9, 0xCA, 0xCC, 0xD1, 0xD2, 0xD4,
    0xD8, 0xE1, 0xE2, 0xE4, 0xE8, 0xF0,
];

// 5-dot patterns (C(8,5) = 56) - complement of 3-dot
const PATTERNS_5: [u8; 56] = [
    0x1F, 0x2F, 0x37, 0x3B, 0x3D, 0x3E, 0x4F, 0x57, 0x5B, 0x5D, 0x5E, 0x67, 0x6B, 0x6D, 0x6E, 0x73,
    0x75, 0x76, 0x79, 0x7A, 0x7C, 0x8F, 0x97, 0x9B, 0x9D, 0x9E, 0xA7, 0xAB, 0xAD, 0xAE, 0xB3, 0xB5,
    0xB6, 0xB9, 0xBA, 0xBC, 0xC7, 0xCB, 0xCD, 0xCE, 0xD3, 0xD5, 0xD6, 0xD9, 0xDA, 0xDC, 0xE3, 0xE5,
    0xE6, 0xE9, 0xEA, 0xEC, 0xF1, 0xF2, 0xF4, 0xF8,
];

// 6-dot patterns (C(8,6) = 28) - complement of 2-dot
const PATTERNS_6: [u8; 28] = [
    0x3F, 0x5F, 0x6F, 0x77, 0x7B, 0x7D, 0x7E, 0x9F, 0xAF, 0xB7, 0xBB, 0xBD, 0xBE, 0xCF, 0xD7, 0xDB,
    0xDD, 0xDE, 0xE7, 0xEB, 0xED, 0xEE, 0xF3, 0xF5, 0xF6, 0xF9, 0xFA, 0xFC,
];

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Random selection
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

/// Select a random pattern with exactly n dots.
///
/// `noise` should be a value in 0.0..1.0 (typically from a hash/noise function).
/// Returns empty braille if n > 8 or n == 0 returns the empty pattern.
///
/// # Example
/// ```
/// use tui_vfx_types::braille::{random_with_count, dot_count};
/// let ch = random_with_count(2, 0.5);
/// assert_eq!(dot_count(tui_vfx_types::braille::braille_bits(ch).unwrap()), 2);
/// ```
pub fn random_with_count(n: u8, noise: f32) -> char {
    let patterns = patterns_with_count(n);
    if patterns.is_empty() {
        return braille(0);
    }
    let index =
        ((noise.clamp(0.0, 0.9999) * patterns.len() as f32) as usize).min(patterns.len() - 1);
    braille(patterns[index])
}

/// Select a random pattern with 1 to max dots.
///
/// Distribution is weighted toward fewer dots for subtler effects.
///
/// # Example
/// ```
/// use tui_vfx_types::braille::random_up_to_count;
/// let ch = random_up_to_count(3, 0.5); // 1, 2, or 3 dots
/// ```
pub fn random_up_to_count(max: u8, noise: f32) -> char {
    if max == 0 {
        return braille(0);
    }
    let max = max.min(8);

    // Weight toward fewer dots: use noise to pick count, biased low
    let count_noise = noise * noise; // Square to bias toward lower values
    let count = (count_noise * max as f32).floor() as u8 + 1;
    let count = count.min(max);

    // Use a different part of noise for pattern selection
    let pattern_noise = (noise * 7.0).fract();
    random_with_count(count, pattern_noise)
}

/// Select a random pattern using only dots within a region mask.
///
/// # Example
/// ```
/// use tui_vfx_types::braille::{random_in_region, LEFT_COLUMN, braille_bits};
/// let ch = random_in_region(LEFT_COLUMN, 0.5);
/// let bits = braille_bits(ch).unwrap();
/// // All set bits should be in the left column
/// assert_eq!(bits & !LEFT_COLUMN, 0);
/// ```
pub fn random_in_region(region_mask: u8, noise: f32) -> char {
    if region_mask == 0 {
        return braille(0);
    }

    // Count available dots in region
    let available_dots = region_mask.count_ones() as u8;
    if available_dots == 0 {
        return braille(0);
    }

    // Pick how many dots (1 to available)
    let count = ((noise * available_dots as f32).floor() as u8 + 1).min(available_dots);

    // Build pattern by selecting from available positions
    let pattern_noise = (noise * 13.0).fract();
    random_with_count_in_region(count, region_mask, pattern_noise)
}

/// Select a random pattern with exactly n dots, constrained to a region.
///
/// If n exceeds the number of available dots in the region, uses all available.
pub fn random_with_count_in_region(n: u8, region_mask: u8, noise: f32) -> char {
    if region_mask == 0 || n == 0 {
        return braille(0);
    }

    // Get all valid positions (dot numbers 1-8 that are in the mask)
    let mut positions = [0u8; 8];
    let mut pos_count = 0;
    for dot in 1..=8 {
        if region_mask & (1 << (dot - 1)) != 0 {
            positions[pos_count] = dot;
            pos_count += 1;
        }
    }

    if pos_count == 0 {
        return braille(0);
    }

    let n = (n as usize).min(pos_count);

    // Simple selection: use noise to pick n positions
    // This is a simplified approach - for true uniformity we'd need
    // to enumerate all C(pos_count, n) combinations
    let mut bits: u8 = 0;
    let mut noise_val = noise;
    for i in 0..n {
        let remaining = pos_count - i;
        let pick = ((noise_val * remaining as f32) as usize).min(remaining - 1);
        let dot = positions[pick];
        bits |= 1 << (dot - 1);
        // Swap picked position to end so we don't pick it again
        positions[pick] = positions[remaining - 1];
        noise_val = (noise_val * 7.919).fract();
    }

    braille(bits)
}

// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
// Tests
// ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_braille_conversion() {
        assert_eq!(braille(0x00), '⠀'); // empty
        assert_eq!(braille(0x01), '⠁'); // dot 1
        assert_eq!(braille(0x08), '⠈'); // dot 4
        assert_eq!(braille(0xFF), '⣿'); // all dots
    }

    #[test]
    fn test_braille_bits_roundtrip() {
        for bits in 0..=255u8 {
            let ch = braille(bits);
            assert_eq!(braille_bits(ch), Some(bits));
        }
    }

    #[test]
    fn test_braille_bits_invalid() {
        assert_eq!(braille_bits('A'), None);
        assert_eq!(braille_bits(' '), None);
        assert_eq!(braille_bits('█'), None);
    }

    #[test]
    fn test_from_dots() {
        assert_eq!(from_dots(&[]), '⠀');
        assert_eq!(from_dots(&[1]), '⠁');
        assert_eq!(from_dots(&[1, 4]), '⠉');
        assert_eq!(from_dots(&[1, 2, 3, 4, 5, 6, 7, 8]), '⣿');
    }

    #[test]
    fn test_from_dots_ignores_invalid() {
        assert_eq!(from_dots(&[0, 1, 9, 10]), '⠁'); // only dot 1 is valid
    }

    #[test]
    fn test_dot_count() {
        assert_eq!(dot_count(0x00), 0);
        assert_eq!(dot_count(0x01), 1);
        assert_eq!(dot_count(0x03), 2);
        assert_eq!(dot_count(0xFF), 8);
    }

    #[test]
    fn test_has_dot() {
        let bits = 0b0000_1001; // dots 1 and 4
        assert!(has_dot(bits, 1));
        assert!(!has_dot(bits, 2));
        assert!(!has_dot(bits, 3));
        assert!(has_dot(bits, 4));
        assert!(!has_dot(bits, 0)); // invalid
        assert!(!has_dot(bits, 9)); // invalid
    }

    #[test]
    fn test_patterns_with_count_sizes() {
        assert_eq!(patterns_with_count(0).len(), 1);
        assert_eq!(patterns_with_count(1).len(), 8);
        assert_eq!(patterns_with_count(2).len(), 28);
        assert_eq!(patterns_with_count(3).len(), 56);
        assert_eq!(patterns_with_count(4).len(), 70);
        assert_eq!(patterns_with_count(5).len(), 56);
        assert_eq!(patterns_with_count(6).len(), 28);
        assert_eq!(patterns_with_count(7).len(), 8);
        assert_eq!(patterns_with_count(8).len(), 1);
        assert_eq!(patterns_with_count(9).len(), 0);
    }

    #[test]
    fn test_patterns_have_correct_dot_count() {
        for n in 0..=8 {
            for &bits in patterns_with_count(n) {
                assert_eq!(
                    dot_count(bits),
                    n,
                    "Pattern 0x{:02X} should have {} dots but has {}",
                    bits,
                    n,
                    dot_count(bits)
                );
            }
        }
    }

    #[test]
    fn test_random_with_count() {
        for n in 1..=8 {
            for i in 0..10 {
                let noise = i as f32 / 10.0;
                let ch = random_with_count(n, noise);
                let bits = braille_bits(ch).unwrap();
                assert_eq!(dot_count(bits), n);
            }
        }
    }

    #[test]
    fn test_random_in_region_respects_mask() {
        for i in 0..20 {
            let noise = i as f32 / 20.0;
            let ch = random_in_region(LEFT_COLUMN, noise);
            let bits = braille_bits(ch).unwrap();
            // All set bits should be within the mask
            assert_eq!(
                bits & !LEFT_COLUMN,
                0,
                "Pattern 0x{:02X} has dots outside LEFT_COLUMN",
                bits
            );
        }
    }

    #[test]
    fn test_region_masks_are_correct() {
        // Left column: dots 1,2,3,7 = bits 0,1,2,6
        assert_eq!(LEFT_COLUMN, 0b0100_0111);
        assert!(has_dot(LEFT_COLUMN, 1));
        assert!(has_dot(LEFT_COLUMN, 2));
        assert!(has_dot(LEFT_COLUMN, 3));
        assert!(has_dot(LEFT_COLUMN, 7));

        // Right column: dots 4,5,6,8 = bits 3,4,5,7
        assert_eq!(RIGHT_COLUMN, 0b1011_1000);
        assert!(has_dot(RIGHT_COLUMN, 4));
        assert!(has_dot(RIGHT_COLUMN, 5));
        assert!(has_dot(RIGHT_COLUMN, 6));
        assert!(has_dot(RIGHT_COLUMN, 8));

        // Should be complementary
        assert_eq!(LEFT_COLUMN | RIGHT_COLUMN, ALL_DOTS);
        assert_eq!(LEFT_COLUMN & RIGHT_COLUMN, 0);
    }
}

// <FILE>crates/tui-vfx-types/src/braille.rs</FILE> - <DESC>Braille character utilities for visual effects</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
