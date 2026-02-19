// <FILE>tui-vfx-content/tests/transformers/test_cls_scramble.rs</FILE> - <DESC>Tests for Scramble</DESC>
// <VERS>VERSION: 1.1.0Z</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::Scramble;
use tui_vfx_content::types::ScrambleCharset;

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
fn test_determinism() {
    let tx = Scramble::new(
        12345,
        ScrambleCharset::Binary,
        mixed_signals::prelude::SignalOrFloat::Static(1.0),
    );
    let input = "Hello";
    // Same seed + same progress = same output
    let out1 = tx.transform(input, 0.5, &test_signal_ctx());
    let out2 = tx.transform(input, 0.5, &test_signal_ctx());
    assert_eq!(out1, out2);
}
#[test]
fn test_resolve_complete() {
    let tx = Scramble::new(
        1,
        ScrambleCharset::Alphanumeric,
        mixed_signals::prelude::SignalOrFloat::Static(1.0),
    );
    assert_eq!(tx.transform("Matrix", 1.0, &test_signal_ctx()), "Matrix");
}
#[test]
fn test_scramble_logic() {
    let tx = Scramble::new(
        42,
        ScrambleCharset::Binary,
        mixed_signals::prelude::SignalOrFloat::Static(1.0),
    ); // Only 0 and 1
    let input = "ABC";
    // At 0.0, all should be scrambled (0 or 1)
    let out = tx.transform(input, 0.0, &test_signal_ctx());
    assert_eq!(out.len(), 3);
    assert!(out.chars().all(|c| c == '0' || c == '1'));
    // At 0.4, first char (index 0) might be revealed if 0/3 < 0.4
    // 0 < 0.4 -> Revealed
    // 1/3 = 0.33 < 0.4 -> Revealed
    // 2/3 = 0.66 > 0.4 -> Scrambled
    let out_mid = tx.transform(input, 0.4, &test_signal_ctx());
    assert!(out_mid.starts_with("AB"));
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_scramble.rs</FILE> - <DESC>Tests for Scramble</DESC>
// <VERS>END OF VERSION: 1.1.0Z</VERS>
