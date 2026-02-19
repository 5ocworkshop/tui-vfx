// <FILE>tui-vfx-content/tests/transformers/test_cls_marquee.rs</FILE> - <DESC>Tests for Marquee</DESC>
// <VERS>VERSION: 1.1.0Z</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::transformers::Marquee;

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
fn test_scroll_start() {
    let tx = Marquee::new(5, mixed_signals::prelude::SignalOrFloat::Static(1.0)); // Width 5
    // "Hello World"
    // t=0 -> "Hello"
    assert_eq!(
        tx.transform("Hello World", 0.0, &test_signal_ctx()),
        "Hello"
    );
}
#[test]
fn test_scroll_mid() {
    let tx = Marquee::new(5, mixed_signals::prelude::SignalOrFloat::Static(1.0));
    // "0123456789" (10 chars)
    // t=0.5 -> Offset 5 -> "56789"
    assert_eq!(tx.transform("0123456789", 0.5, &test_signal_ctx()), "56789");
}
#[test]
fn test_wrap_around() {
    let tx = Marquee::new(3, mixed_signals::prelude::SignalOrFloat::Static(1.0));
    // "ABCDE" (5 chars)
    // t=0.8 -> Offset 4 ('E')
    // Window: "E" + "A" + "B" -> "EAB"
    assert_eq!(tx.transform("ABCDE", 0.8, &test_signal_ctx()), "EAB");
}
#[test]
fn test_unicode_marquee() {
    let tx = Marquee::new(2, mixed_signals::prelude::SignalOrFloat::Static(1.0));
    // "👋🌍🚀" (3 graphemes)
    // t=0.34 -> 1.02 -> Offset 1 ("🌍")
    // Window: "🌍" + "🚀" -> "🌍🚀"
    assert_eq!(tx.transform("👋🌍🚀", 0.34, &test_signal_ctx()), "🌍🚀");
}

#[test]
fn test_non_wrapping_returns_borrowed() {
    let tx = Marquee::new(5, mixed_signals::prelude::SignalOrFloat::Static(1.0));
    let result = tx.transform("Hello World", 0.0, &test_signal_ctx());
    assert!(matches!(result, Cow::Borrowed(_)));
}

#[test]
fn test_wrapping_returns_owned() {
    let tx = Marquee::new(3, mixed_signals::prelude::SignalOrFloat::Static(1.0));
    let result = tx.transform("ABCDE", 0.8, &test_signal_ctx());
    assert!(matches!(result, Cow::Owned(_)));
}

// <FILE>tui-vfx-content/tests/transformers/test_cls_marquee.rs</FILE> - <DESC>Tests for Marquee</DESC>
// <VERS>END OF VERSION: 1.1.0Z</VERS>
