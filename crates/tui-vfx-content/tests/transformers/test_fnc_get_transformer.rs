// <FILE>tui-vfx-content/tests/transformers/test_fnc_get_transformer.rs</FILE> - <DESC>Tests for factory</DESC>
// <VERS>VERSION: 1.4.0</VERS>
// <WCTX>SlideShift barrier span support</WCTX>
// <CLOG>Include SlideShift shift_width in factory test</CLOG>

use mixed_signals::prelude::SignalContext;
use mixed_signals::types::SignalOrFloat;
use tui_vfx_content::transformers::get_transformer;
use tui_vfx_content::types::{
    ContentEffect, ScrambleCharset, SlideShiftFlowMode, SlideShiftLineMode,
};

// Helper for creating test SignalContext
fn test_signal_ctx() -> SignalContext {
    SignalContext {
        frame: 0,
        seed: 0,
        width: 80,
        height: 24,
        phase: None,
        phase_t: None,
        loop_t: None,
        absolute_t: None,
        char_index: None,
    }
}

#[test]
fn test_factory_typewriter() {
    let config = ContentEffect::Typewriter {
        speed_variance: SignalOrFloat::Static(0.0),
        cursor: None,
    };
    let tx = get_transformer(&config);
    // Smoke test
    assert_eq!(tx.transform("Hello", 0.5, &test_signal_ctx()), "He");
}

#[test]
fn test_factory_redact() {
    let config = ContentEffect::Redact { symbol: '*' };
    let tx = get_transformer(&config);
    assert_eq!(tx.transform("1234", 0.5, &test_signal_ctx()), "12**");
}

#[test]
fn test_factory_scramble() {
    let config = ContentEffect::Scramble {
        resolve_pace: SignalOrFloat::Static(1.0),
        charset: ScrambleCharset::Binary,
        seed: 123,
    };
    let tx = get_transformer(&config);
    // Ensure it's actually using the seed/charset
    let out = tx.transform("ABC", 0.0, &test_signal_ctx());
    assert!(out.chars().all(|c| c == '0' || c == '1'));
}

#[test]
fn test_factory_glitch_shift_before_window() {
    let config = ContentEffect::GlitchShift {
        shift_amount: 5,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
        seed: 42,
    };
    let tx = get_transformer(&config);
    // Before glitch window - no shift
    assert_eq!(tx.transform("hello", 0.1, &test_signal_ctx()), "hello");
}

#[test]
fn test_factory_glitch_shift_during_window() {
    let config = ContentEffect::GlitchShift {
        shift_amount: 5,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
        seed: 42,
    };
    let tx = get_transformer(&config);
    // During glitch window - should have 5 spaces prepended
    assert_eq!(
        tx.transform("hello", 0.35, &test_signal_ctx()),
        "     hello"
    );
}

#[test]
fn test_factory_glitch_shift_after_window() {
    let config = ContentEffect::GlitchShift {
        shift_amount: 5,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
        seed: 42,
    };
    let tx = get_transformer(&config);
    // After glitch window - no shift
    assert_eq!(tx.transform("hello", 0.5, &test_signal_ctx()), "hello");
}

#[test]
fn test_factory_scramble_glitch_shift_before_window() {
    let config = ContentEffect::ScrambleGlitchShift {
        resolve_pace: SignalOrFloat::Static(1.0),
        charset: ScrambleCharset::Binary,
        scramble_seed: 42,
        shift_amount: 3,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
    };
    let tx = get_transformer(&config);
    // At t=0.0, text should be scrambled but no shift
    let out = tx.transform("ABC", 0.0, &test_signal_ctx());
    // Should be 3 binary chars, no leading spaces
    assert_eq!(out.len(), 3);
    assert!(out.chars().all(|c| c == '0' || c == '1'));
}

#[test]
fn test_factory_scramble_glitch_shift_during_window() {
    let config = ContentEffect::ScrambleGlitchShift {
        resolve_pace: SignalOrFloat::Static(1.0),
        charset: ScrambleCharset::Binary,
        scramble_seed: 42,
        shift_amount: 3,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
    };
    let tx = get_transformer(&config);
    // During glitch window - should have shift applied
    let out = tx.transform("ABC", 0.35, &test_signal_ctx());
    // Should start with 3 spaces
    assert!(
        out.starts_with("   "),
        "Expected 3 leading spaces, got: {:?}",
        out
    );
}

#[test]
fn test_factory_scramble_glitch_shift_resolved() {
    let config = ContentEffect::ScrambleGlitchShift {
        resolve_pace: SignalOrFloat::Static(1.0),
        charset: ScrambleCharset::Binary,
        scramble_seed: 42,
        shift_amount: 3,
        glitch_start: SignalOrFloat::Static(0.3),
        glitch_end: SignalOrFloat::Static(0.4),
    };
    let tx = get_transformer(&config);
    // At t=1.0, text should be fully resolved (original text), no shift
    let out = tx.transform("ABC", 1.0, &test_signal_ctx());
    assert_eq!(out, "ABC");
}

#[test]
fn test_factory_slide_shift() {
    let config = ContentEffect::SlideShift {
        start_col: 0,
        end_col: 20,
        start_row: 1,
        shift_col: 10,
        shift_width: 1,
        row_shift: -1,
        line_mode: SlideShiftLineMode::Block,
        flow_mode: SlideShiftFlowMode::StayShifted,
    };
    let tx = get_transformer(&config);
    let out = tx.transform("Hello", 0.6, &test_signal_ctx());
    assert_eq!(out, "            Hello");
}

// <FILE>tui-vfx-content/tests/transformers/test_fnc_get_transformer.rs</FILE> - <DESC>Tests for factory</DESC>
// <VERS>END OF VERSION: 1.4.0</VERS>
