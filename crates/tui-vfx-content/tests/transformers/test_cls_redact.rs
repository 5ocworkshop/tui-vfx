// <FILE>tui-vfx-content/tests/transformers/test_cls_redact.rs</FILE> - <DESC>Tests for Redact</DESC>
// <VERS>VERSION: 1.1.0Z</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::Redact;

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
fn test_redaction_full() {
    let tx = Redact::new('*');
    // At 0.0, everything is redacted
    assert_eq!(tx.transform("1234", 0.0, &test_signal_ctx()), "****");
}
#[test]
fn test_redaction_partial() {
    let tx = Redact::new('#');
    // At 0.5, half revealed
    assert_eq!(tx.transform("1234", 0.5, &test_signal_ctx()), "12##");
}
#[test]
fn test_redaction_complete() {
    let tx = Redact::new('?');
    assert_eq!(tx.transform("Done", 1.0, &test_signal_ctx()), "Done");
}
#[test]
fn test_unicode_redaction() {
    let tx = Redact::new('█');
    let input = "ABC";
    // 3 chars. 0.34 -> 1 revealed. 2 redacted.
    assert_eq!(tx.transform(input, 0.34, &test_signal_ctx()), "A██");
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_redact.rs</FILE> - <DESC>Tests for Redact</DESC>
// <VERS>END OF VERSION: 1.1.0Z</VERS>
