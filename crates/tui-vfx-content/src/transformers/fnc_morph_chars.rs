// <FILE>tui-vfx-content/src/transformers/fnc_morph_chars.rs</FILE> - <DESC>Character generators for morph text transitions</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract character generators from cls_morph.rs</WCTX>
// <CLOG>Initial extraction of density, binary, and braille character generators</CLOG>

/// Get the density block character for a given progress (0.0-1.0)
/// Progresses from sparse (░) to solid (█)
pub fn density_char(local_progress: f32) -> &'static str {
    if local_progress < 0.2 {
        "░"
    } else if local_progress < 0.5 {
        "▒"
    } else if local_progress < 0.8 {
        "▓"
    } else {
        "█"
    }
}

/// Get the density block character for a given progress (reverse: solid to sparse)
pub fn density_char_reverse(local_progress: f32) -> &'static str {
    if local_progress < 0.2 {
        "█"
    } else if local_progress < 0.5 {
        "▓"
    } else if local_progress < 0.8 {
        "▒"
    } else {
        "░"
    }
}

/// Get the binary character for a given progress with pseudo-random flickering
pub fn binary_char(local_progress: f32, seed: u64, i: usize) -> char {
    // Add some pseudo-random flickering
    let hash =
        ((seed.wrapping_add(i as u64)).wrapping_mul(0x517cc1b727220a95)) as f32 / u64::MAX as f32;
    let flicker = (hash * 0.3).min(0.15);
    if local_progress + flicker < 0.5 {
        '0'
    } else {
        '1'
    }
}

/// Get the braille character for a given progress (0.0-1.0)
/// Progresses from empty (⠀) through increasing dots to full (⣿)
/// Fills left column first (dots 1,2,3,7) then right column (4,5,6,8)
///
/// 8-dot braille layout:
/// ```text
///   1 4
///   2 5
///   3 6
///   7 8
/// ```
pub fn braille_char_up(local_progress: f32) -> &'static str {
    if local_progress < 0.111 {
        "⠀" // Empty (U+2800)
    } else if local_progress < 0.222 {
        "⠁" // dot 1 (top-left)
    } else if local_progress < 0.333 {
        "⠃" // dots 1,2
    } else if local_progress < 0.444 {
        "⠇" // dots 1,2,3
    } else if local_progress < 0.556 {
        "⡇" // dots 1,2,3,7 (left column complete)
    } else if local_progress < 0.667 {
        "⡏" // dots 1,2,3,7,4
    } else if local_progress < 0.778 {
        "⡟" // dots 1,2,3,7,4,5
    } else if local_progress < 0.889 {
        "⡿" // dots 1,2,3,7,4,5,6
    } else {
        "⣿" // all 8 dots (full)
    }
}

/// Get the braille character for a given progress (0.0-1.0)
/// Progresses from full (⣿) through decreasing dots to empty (⠀)
/// Empties right column first, then left column
pub fn braille_char_down(local_progress: f32) -> &'static str {
    // Inverse of braille_char_up: start full, empty to reveal
    if local_progress < 0.111 {
        "⣿" // all 8 dots (full)
    } else if local_progress < 0.222 {
        "⡿" // dots 1,2,3,7,4,5,6
    } else if local_progress < 0.333 {
        "⡟" // dots 1,2,3,7,4,5
    } else if local_progress < 0.444 {
        "⡏" // dots 1,2,3,7,4
    } else if local_progress < 0.556 {
        "⡇" // dots 1,2,3,7 (left column only)
    } else if local_progress < 0.667 {
        "⠇" // dots 1,2,3
    } else if local_progress < 0.778 {
        "⠃" // dots 1,2
    } else if local_progress < 0.889 {
        "⠁" // dot 1 (top-left only)
    } else {
        "⠀" // Empty (U+2800)
    }
}

/// Left column only braille (dots 1,2,3,7) - used for half-cell wipe leading edge
pub const BRAILLE_LEFT_COL: &str = "⡇";

/// Right column only braille (dots 4,5,6,8) - used for half-cell wipe leading edge
pub const BRAILLE_RIGHT_COL: &str = "⣸";

// <FILE>tui-vfx-content/src/transformers/fnc_morph_chars.rs</FILE> - <DESC>Character generators for morph text transitions</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
