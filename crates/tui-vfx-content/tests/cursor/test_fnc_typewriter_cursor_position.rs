// <FILE>tui-vfx-content/tests/cursor/test_fnc_typewriter_cursor_position.rs</FILE> - <DESC>Tests for typewriter cursor position helper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: typewriter cursor position from progress</WCTX>
// <CLOG>Initial tests</CLOG>

use tui_vfx_content::cursor::fnc_typewriter_cursor_position;

#[test]
fn progress_zero_is_none_or_zero() {
    // At progress 0.0 nothing is revealed yet; cursor sits before char 0.
    let pos = fnc_typewriter_cursor_position("hello", 0.0);
    assert_eq!(pos, Some(0));
}

#[test]
fn progress_full_points_past_last_char() {
    let pos = fnc_typewriter_cursor_position("hello", 1.0);
    assert_eq!(pos, Some(5));
}

#[test]
fn progress_half_on_even_length_rounds_down() {
    let pos = fnc_typewriter_cursor_position("abcdef", 0.5);
    assert_eq!(pos, Some(3));
}

#[test]
fn empty_target_returns_zero() {
    let pos = fnc_typewriter_cursor_position("", 0.5);
    assert_eq!(pos, Some(0));
}

#[test]
fn multi_byte_characters_counted_by_grapheme() {
    // Two graphemes: "é" + "x"
    let pos = fnc_typewriter_cursor_position("éx", 0.5);
    assert_eq!(pos, Some(1));
}
// <FILE>tui-vfx-content/tests/cursor/test_fnc_typewriter_cursor_position.rs</FILE> - <DESC>Tests for typewriter cursor position helper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
