// <FILE>tui-vfx-content/tests/transformers/test_typewriter_transform_with_cursor.rs</FILE> - <DESC>Tests for Typewriter::transform_with_cursor</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up (field_reassign_with_default)</WCTX>
// <CLOG>PATCH: rewrite cursor construction to struct-literal form with ..Cursor::default()</CLOG>

use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use tui_vfx_content::cursor::{Cursor, CursorState};
use tui_vfx_content::transformers::Typewriter;

fn ctx() -> SignalContext {
    SignalContext::new(0, 0)
}

#[test]
fn at_half_progress_splices_block_cursor_after_revealed_chars() {
    let mut state = CursorState::new();
    let cursor = Cursor::block(); // no animation, static block
    let tw = Typewriter::default();
    let (text, _ops) =
        tw.transform_with_cursor("hello world", 0.5, &ctx(), &cursor, &mut state, 0.0, 0.016);
    // 11 graphemes * 0.5 = 5 revealed → cursor at index 5 (past "hello").
    assert_eq!(text, "hello█");
}

#[test]
fn empty_cursor_character_produces_plain_reveal() {
    let mut state = CursorState::new();
    let cursor = Cursor {
        character: "".into(),
        ..Cursor::default()
    };
    let tw = Typewriter::default();
    let (text, _ops) =
        tw.transform_with_cursor("abcdef", 0.5, &ctx(), &cursor, &mut state, 0.0, 0.016);
    assert_eq!(text, "abc");
}

#[test]
fn at_full_progress_cursor_sits_past_last_char() {
    let mut state = CursorState::new();
    let cursor = Cursor::block();
    let tw = Typewriter::default();
    let (text, _ops) = tw.transform_with_cursor("ab", 1.0, &ctx(), &cursor, &mut state, 0.0, 0.016);
    assert_eq!(text, "ab█");
}

#[test]
fn grow_in_mid_animation_uses_partial_block_glyph() {
    let mut state = CursorState::new();
    let mut cursor = Cursor::block();
    cursor.grow_in.mode = tui_vfx_content::cursor::GrowInMode::Once;
    cursor.grow_in.duration_ms = SignalOrFloat::Static(200.0);
    cursor.visibility = SignalOrFloat::Static(1.0);
    let tw = Typewriter::default();
    // First advance at t=0 starts grow-in.
    let (_text0, _) = tw.transform_with_cursor("ab", 0.0, &ctx(), &cursor, &mut state, 0.0, 0.016);
    // Second advance at t=0.1s is ~half way through grow-in.
    let (text_mid, _) = tw.transform_with_cursor("ab", 0.5, &ctx(), &cursor, &mut state, 0.1, 0.1);
    // The spliced glyph should NOT be "█" — should be a partial.
    assert!(!text_mid.ends_with('█'));
    assert!(text_mid.starts_with('a'));
}

#[test]
fn transform_without_cursor_unchanged() {
    // Sanity: plain Typewriter::transform still matches pre-cursor behavior.
    // "hello" has 5 graphemes; per the 2.0.1 threshold = (i+1)/total rule,
    // progress 0.5 reveals graphemes with threshold <= 0.5, i.e. indices 0
    // ((0+1)/5=0.2) and 1 ((1+1)/5=0.4); index 2 is at 0.6 and is NOT
    // revealed. Hence 2 chars visible: "he".
    let tw = Typewriter::default();
    use tui_vfx_content::traits::TextTransformer;
    let out = tw.transform("hello", 0.5, &ctx());
    assert_eq!(out.as_ref(), "he");
}
// <FILE>tui-vfx-content/tests/transformers/test_typewriter_transform_with_cursor.rs</FILE> - <DESC>Tests for Typewriter::transform_with_cursor</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
