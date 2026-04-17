// <FILE>tui-vfx-content/src/cursor/fnc_typewriter_cursor_position.rs</FILE> - <DESC>Compute cursor grapheme index for typewriter reveal</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: typewriter cursor position</WCTX>
// <CLOG>Initial impl</CLOG>

use crate::utils::fnc_graphemes::len_graphemes;

/// Compute the grapheme index where a typewriter's cursor should sit at the
/// given reveal progress.
///
/// - `0.0` → index `0` (cursor at start, nothing revealed yet).
/// - `1.0` → index `len` (cursor past the last character, reveal complete).
/// - Intermediate progress rounds down so the cursor renders *after* the
///   last-revealed character, consistent with how `Typewriter::transform`
///   already computes its reveal slice.
pub fn fnc_typewriter_cursor_position(target: &str, progress: f64) -> Option<usize> {
    let total = len_graphemes(target);
    if total == 0 {
        return Some(0);
    }
    let p = progress.clamp(0.0, 1.0);
    let idx = (p * total as f64).floor() as usize;
    Some(idx.min(total))
}

// <FILE>tui-vfx-content/src/cursor/fnc_typewriter_cursor_position.rs</FILE> - <DESC>Compute cursor grapheme index for typewriter reveal</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
