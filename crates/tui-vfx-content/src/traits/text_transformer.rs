// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE> - <DESC>TextTransformer trait definition</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>Phase 2: Signal-driven content effects - TextTransformer trait update</WCTX>
// <CLOG>BREAKING: Added signal_ctx parameter to transform() method for signal-driven effects</CLOG>

use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;
/// A trait for applying visual effects to text strings.
///
/// Implementors should use `Cow<str>` to return the original string slice
/// if no transformation is needed, avoiding unnecessary allocations.
pub trait TextTransformer {
    /// Transforms the target string based on the current progress (0.0 to 1.0).
    ///
    /// # Arguments
    /// * `target` - The final string to display.
    /// * `progress` - Animation progress from 0.0 (start) to 1.0 (end).
    /// * `signal_ctx` - Signal context for random/dynamic effects (frame, seed, phase, etc.)
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str>;
}

// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE> - <DESC>TextTransformer trait definition</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
