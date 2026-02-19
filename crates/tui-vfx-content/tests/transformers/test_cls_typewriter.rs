// <FILE>tui-vfx-content/tests/transformers/test_cls_typewriter.rs</FILE> - <DESC>Tests for Typewriter</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::Typewriter;

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
fn test_progress_0() {
    let tx = Typewriter::default();
    assert_eq!(tx.transform("Hello", 0.0, &test_signal_ctx()), "");
}
#[test]
fn test_progress_half() {
    let tx = Typewriter::default();
    // 5 chars * 0.5 = 2.5 -> 2 chars
    assert_eq!(tx.transform("Hello", 0.5, &test_signal_ctx()), "He");
}
#[test]
fn test_progress_full() {
    let tx = Typewriter::default();
    assert_eq!(tx.transform("Hello", 1.0, &test_signal_ctx()), "Hello");
}
#[test]
fn test_unicode() {
    let tx = Typewriter::default();
    let input = "👋🌍🚀";
    // 3 graphemes. 0.34 -> 1.02 -> 1 grapheme
    assert_eq!(tx.transform(input, 0.34, &test_signal_ctx()), "👋");
    // 0.67 -> 2.01 -> 2 graphemes
    assert_eq!(tx.transform(input, 0.67, &test_signal_ctx()), "👋🌍");
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_typewriter.rs</FILE> - <DESC>Tests for Typewriter</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
