// <FILE>tui-vfx-content/src/traits/cls_transform_context.rs</FILE> - <DESC>Per-call context bundle passed to TextTransformer::transform</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: bundle SignalContext + ShaderRuntimeParams so future additions extend the struct rather than churning the trait.</WCTX>
// <CLOG>1.0.0: introduce TransformContext { signal_ctx, runtime_params } with new() constructor and inline tests.</CLOG>

use mixed_signals::prelude::SignalContext;
use tui_vfx_style::traits::ShaderRuntimeParams;

/// Bundle of call-time context passed to every [`TextTransformer`].
///
/// Adding a new context piece in the future (for example a `Substitutions`
/// reference, an asset resolver, or a theme snapshot) extends this struct
/// without trait churn. Transformers ignore fields they don't need; the
/// struct is `Copy` and zero-cost — two reference-sized fields fitting in
/// a single cache line.
///
/// [`TextTransformer`]: crate::traits::TextTransformer
#[derive(Clone, Copy)]
pub struct TransformContext<'a> {
    /// Per-frame signal evaluation context (frame, seed, phase, normalized
    /// coords, char-index, etc.). Used by signal-driven parameter resolution
    /// such as [`mixed_signals::prelude::SignalOrFloat::evaluate`].
    pub signal_ctx: &'a SignalContext,
    /// Host-supplied runtime parameters. Carries values for `BindableString`
    /// / `BindableU16` / `BindableF32` / `BindableColor` fields whose recipe
    /// shape is `{"binding": "name"}`. An empty map is equivalent to "no
    /// host values supplied"; transformers must degrade gracefully (typically
    /// to a static or asset-default fallback).
    pub runtime_params: &'a ShaderRuntimeParams,
}

impl<'a> TransformContext<'a> {
    /// Construct a context bundle from explicit references. Most callers
    /// use this directly at the transform site; gt-design's render path
    /// already holds both pieces.
    pub fn new(signal_ctx: &'a SignalContext, runtime_params: &'a ShaderRuntimeParams) -> Self {
        Self {
            signal_ctx,
            runtime_params,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_constructs_context_with_field_access() {
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let ctx = TransformContext::new(&sig, &params);
        // Exercise field reads to confirm the struct shape compiles
        // against the trait callers' expectations.
        assert!(ctx.runtime_params.get("nonexistent").is_none());
        let _ = ctx.signal_ctx;
    }
}

// <FILE>tui-vfx-content/src/traits/cls_transform_context.rs</FILE>
// <VERS>END OF VERSION: 1.0.0</VERS>
