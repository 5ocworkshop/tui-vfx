// <FILE>tui-vfx-content/tests/utils/test_fnc_graphemes.rs</FILE> - <DESC>Tests for grapheme utils</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-16T20:35:28Z</VERS>
// <WCTX>Verifying unicode safety</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
#[test]
fn test_emoji_count() {
    // Wave + Earth = 2 graphemes
    let input = "👋🌍";
    assert_eq!(len_graphemes(input), 2);
    assert_eq!(input.len(), 8); // 4 bytes each
}
#[test]
fn test_slice_graphemes() {
    let input = "Hello👋World";
    // "Hello" = 5
    // "👋" = 1
    // "World" = 5
    // Slice "👋"
    assert_eq!(slice_graphemes(input, 5, 6), "👋");
    // Slice "Hello"
    assert_eq!(slice_graphemes(input, 0, 5), "Hello");
    // Slice "World"
    assert_eq!(slice_graphemes(input, 6, 11), "World");
}
#[test]
fn test_slice_out_of_bounds() {
    let input = "Hi";
    assert_eq!(slice_graphemes(input, 0, 5), "Hi");
    assert_eq!(slice_graphemes(input, 5, 10), "");
}

// <FILE>tui-vfx-content/tests/utils/test_fnc_graphemes.rs</FILE> - <DESC>Tests for grapheme utils</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-16T20:35:28Z</VERS>
