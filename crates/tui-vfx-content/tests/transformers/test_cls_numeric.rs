// <FILE>tui-vfx-content/tests/transformers/test_cls_numeric.rs</FILE> - <DESC>Tests for Numeric</DESC>
// <VERS>VERSION: 1.1.0Z</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::Numeric;

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
fn test_integers() {
    let tx = Numeric::default(); // Default format "{}"
    // 0 -> 100. At 0.5 -> 50
    assert_eq!(tx.transform("100", 0.5, &test_signal_ctx()), "50");
}
#[test]
fn test_floats() {
    let tx = Numeric::new("{:.1}");
    // 0.0 -> 10.0. At 0.5 -> 5.0
    assert_eq!(tx.transform("10.0", 0.5, &test_signal_ctx()), "5.0");
}
#[test]
fn test_non_numeric() {
    let tx = Numeric::default();
    // Should pass through unchanged
    assert_eq!(tx.transform("Hello", 0.5, &test_signal_ctx()), "Hello");
}
#[test]
fn test_negative() {
    let tx = Numeric::default();
    // 0 -> -100. At 0.5 -> -50
    assert_eq!(tx.transform("-100", 0.5, &test_signal_ctx()), "-50");
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_numeric.rs</FILE> - <DESC>Tests for Numeric</DESC>
// <VERS>END OF VERSION: 1.1.0Z</VERS>
