// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE> - <DESC>TextTransformer trait definition</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: bundle context into TransformContext so the trait absorbs a second context piece (ShaderRuntimeParams) and future ones (substitutions, asset resolver) without further churn.</WCTX>
// <CLOG>3.0.0: BREAKING — replace `signal_ctx: &SignalContext` with `ctx: &TransformContext<'_>`. Per-call context is now bundled. Migration: replace `signal_ctx` reads with `ctx.signal_ctx`; transformers needing runtime params read `ctx.runtime_params`.</CLOG>

use crate::traits::TransformContext;
use std::borrow::Cow;

/// A trait for applying visual effects to text strings.
///
/// Implementors should use [`Cow<str>`] to return the original string slice
/// if no transformation is needed, avoiding unnecessary allocations.
pub trait TextTransformer {
    /// Transforms the target string based on the current progress (0.0 to 1.0).
    ///
    /// # Arguments
    /// * `target` - The final string to display.
    /// * `progress` - Animation progress from 0.0 (start) to 1.0 (end).
    /// * `ctx` - Per-call context bundle. See [`TransformContext`].
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str>;
}

// <FILE>tui-vfx-content/src/traits/text_transformer.rs</FILE> - <DESC>TextTransformer trait definition</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
