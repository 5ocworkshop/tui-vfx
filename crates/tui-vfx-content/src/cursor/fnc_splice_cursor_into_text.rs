// <FILE>tui-vfx-content/src/cursor/fnc_splice_cursor_into_text.rs</FILE> - <DESC>Splice cursor glyph into text at grapheme index</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: splice helper</WCTX>
// <CLOG>Initial impl</CLOG>

use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};

/// Splice the cursor `glyph` into `text` at grapheme index `at`.
///
/// Behavior:
/// - `glyph` empty → `text` returned unchanged.
/// - `at >= len_graphemes(text)` → glyph appended at end.
/// - `at < len_graphemes(text)` → glyph *replaces* the grapheme at `at`,
///   simulating an overwrite-style cursor (block / caret).
pub fn fnc_splice_cursor_into_text(text: &str, at: usize, glyph: &str) -> String {
    if glyph.is_empty() {
        return text.to_string();
    }
    let total = len_graphemes(text);
    if at >= total {
        let mut s = String::with_capacity(text.len() + glyph.len());
        s.push_str(text);
        s.push_str(glyph);
        return s;
    }
    let before = slice_graphemes(text, 0, at);
    let after = slice_graphemes(text, at + 1, total);
    let mut s = String::with_capacity(before.len() + glyph.len() + after.len());
    s.push_str(before);
    s.push_str(glyph);
    s.push_str(after);
    s
}

// <FILE>tui-vfx-content/src/cursor/fnc_splice_cursor_into_text.rs</FILE> - <DESC>Splice cursor glyph into text at grapheme index</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
