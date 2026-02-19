// <FILE>tui-vfx-content/src/utils/fnc_graphemes.rs</FILE> - <DESC>Unicode segmentation helpers</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-16T20:36:40Z</VERS>
// <WCTX>Fixed unused variable warning</WCTX>
// <CLOG>Removed dead code block</CLOG>

use unicode_segmentation::UnicodeSegmentation;
/// Returns the number of grapheme clusters in the string.
pub fn len_graphemes(s: &str) -> usize {
    s.graphemes(true).count()
}
/// Returns a slice of the string containing the specified range of grapheme clusters.
///
/// # Arguments
/// * `s` - The source string.
/// * `start` - The starting grapheme index (inclusive).
/// * `end` - The ending grapheme index (exclusive).
pub fn slice_graphemes(s: &str, start: usize, end: usize) -> &str {
    // Strategy: Iterate manually to find byte offsets for start and end graphemes.
    // We avoid grapheme_indices() with nth() for the start because we need to continue
    // to 'end' without restarting the iterator for O(N) efficiency.
    let mut iter = s.graphemes(true);
    let mut byte_len = 0;
    let mut current_grapheme_idx = 0;
    // Skip until start
    while current_grapheme_idx < start {
        match iter.next() {
            Some(g) => byte_len += g.len(),
            None => return &s[s.len()..], // Start is out of bounds
        }
        current_grapheme_idx += 1;
    }
    let start_byte = byte_len;
    // Take until end
    while current_grapheme_idx < end {
        match iter.next() {
            Some(g) => byte_len += g.len(),
            None => break,
        }
        current_grapheme_idx += 1;
    }
    &s[start_byte..byte_len]
}

// <FILE>tui-vfx-content/src/utils/fnc_graphemes.rs</FILE> - <DESC>Unicode segmentation helpers</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-16T20:36:40Z</VERS>
