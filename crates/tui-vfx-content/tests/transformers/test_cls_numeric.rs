// <FILE>tui-vfx-content/tests/transformers/test_cls_numeric.rs</FILE> - <DESC>Tests for Numeric</DESC>
// <VERS>VERSION: 1.1.0Z</VERS>
// <WCTX>Phase 2: Signal-driven content effects - Test updates</WCTX>
// <CLOG>Updated transform() calls to pass SignalContext</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_content::traits::TextTransformer;
use tui_vfx_content::traits::TransformContext;
use tui_vfx_content::transformers::Numeric;
use tui_vfx_style::traits::ShaderRuntimeParams;

static CTX_PARTS: std::sync::OnceLock<(SignalContext, ShaderRuntimeParams)> =
    std::sync::OnceLock::new();

// Helper returning a per-test TransformContext<'static>. The SignalContext
// half is constructed once via OnceLock and shared across all callers.
fn test_signal_ctx() -> TransformContext<'static> {
    let parts = CTX_PARTS.get_or_init(|| {
        let sig = {
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
                cell_x: None,
                cell_y: None,
                ..Default::default()
            }
        };
        (sig, ShaderRuntimeParams::new())
    });
    TransformContext::new(&parts.0, &parts.1)
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
