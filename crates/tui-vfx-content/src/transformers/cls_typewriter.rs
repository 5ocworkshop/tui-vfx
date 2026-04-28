// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>VERSION: 4.1.0</VERS>
// <WCTX>Packet 69-A: speed_variance is now VfxBindableValue so hosts can drive typing jitter via runtime bindings without recipe rewrites.</WCTX>
// <CLOG>4.1.0: MINOR — speed_variance field, Typewriter::new arg, Default impl, and the transform-time evaluate call all migrate from SignalOrFloat to VfxBindableValue. The evaluate call now passes ctx.runtime_params so {"binding": "key"} resolves against the host's ShaderRuntimeParams.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use tui_vfx_core::bindable::VfxBindableValue;

/// Typewriter effect that reveals text character-by-character.
///
/// Supports optional speed variance for organic, human-like typing rhythm.
#[derive(Debug, Clone)]
pub struct Typewriter {
    /// Per-character timing variance (0.0 = uniform, higher = more variation).
    ///
    /// Bindable: literal, host-supplied `{"binding": "name"}`, or `{"signal": ...}`.
    pub speed_variance: VfxBindableValue,
}

impl Typewriter {
    pub fn new(speed_variance: VfxBindableValue) -> Self {
        Self { speed_variance }
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            speed_variance: VfxBindableValue::Literal(0.0),
        }
    }
}

impl TextTransformer for Typewriter {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress <= 0.0 {
            return Cow::Borrowed("");
        }
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }

        let total = len_graphemes(target);
        if total == 0 {
            return Cow::Borrowed(target);
        }

        // Evaluate speed_variance per-frame; resolves literal / runtime binding /
        // signal expression through the bindable's three-arg evaluate. Fallback
        // to 0.0 on missing bindings or signal-build errors.
        let variance = f64::from(
            self.speed_variance
                .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
                .unwrap_or(0.0),
        );

        // Calculate visible characters with per-character variance
        let mut visible = 0;
        for i in 0..total {
            let base_threshold = (i + 1) as f64 / total as f64;

            // Apply deterministic per-character variance using hash
            let char_variance = if variance.abs() > 0.0001 {
                // Use hash_to_index to map character index to a deterministic variance value
                // Map from [0, u64::MAX] to [-variance, variance]
                let hash_input = ctx.signal_ctx.seed.wrapping_add(i as u64);
                let hash_val = hash_to_index(hash_input, 0, 10000); // Map to 0-9999
                let normalized = (hash_val as f64 / 10000.0) * 2.0 - 1.0; // Map to -1.0 to 1.0
                normalized * variance
            } else {
                0.0
            };

            let threshold = (base_threshold + char_variance).clamp(0.0, 1.0);

            if progress >= threshold {
                visible = i + 1;
            } else {
                break;
            }
        }

        Cow::Borrowed(slice_graphemes(target, 0, visible))
    }
}

use crate::cursor::{
    Cursor, CursorPaintOps, CursorState, fnc_advance_cursor, fnc_render_cursor,
    fnc_splice_cursor_into_text, fnc_typewriter_cursor_position,
};

impl Typewriter {
    /// Stateful reveal that splices a cursor glyph into the output string at the
    /// current reveal boundary, advancing the provided [`CursorState`] in place.
    ///
    /// Returns `(text, paint_ops)`. `paint_ops` carries the wake/primary paint
    /// info for consumers that want cell-level rendering via
    /// [`tui_vfx_style::models::CursorShader`] (see Task 28).
    ///
    /// # Arguments
    ///
    /// * `target`   — Full text being revealed.
    /// * `progress` — Reveal progress 0..1 (same semantics as [`Typewriter`]'s
    ///   [`TextTransformer::transform`] impl).
    /// * `ctx`      — Per-call context bundle. See [`TransformContext`]; the
    ///   [`SignalContext`](mixed_signals::prelude::SignalContext) inside is
    ///   forwarded to the cursor advance and render helpers as well as to the
    ///   base reveal.
    /// * `cursor`   — Cursor configuration (usually `tcursor.cursor`).
    /// * `state`    — Mutable cursor state owned by the caller.
    /// * `now`      — Wall-clock seconds (same value used for signal sampling).
    /// * `dt`       — Wall-clock seconds since the previous frame.
    #[allow(clippy::too_many_arguments)]
    pub fn transform_with_cursor<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
        cursor: &Cursor,
        state: &mut CursorState,
        now: f64,
        dt: f64,
    ) -> (Cow<'a, str>, CursorPaintOps) {
        // Run the base reveal.
        let revealed = self.transform(target, progress, ctx);

        // Resolve cursor position from progress and advance the primitive.
        let idx = fnc_typewriter_cursor_position(target, progress).unwrap_or(0);
        // Row is always 0 for a single-line typewriter reveal; callers wrapping
        // multi-line text must drive the cursor externally.
        let pos = (0u16, idx as u16);
        fnc_advance_cursor(state, cursor, Some(pos), now, dt, ctx.signal_ctx);

        // Render paint ops and splice the primary glyph in.
        let ops = fnc_render_cursor(state, cursor, now, ctx.signal_ctx);
        let text = match ops.primary.as_ref() {
            Some(p) if !p.glyph.is_empty() => Cow::Owned(fnc_splice_cursor_into_text(
                revealed.as_ref(),
                idx,
                &p.glyph,
            )),
            _ => revealed,
        };
        (text, ops)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mixed_signals::prelude::SignalContext;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    #[test]
    fn speed_variance_binding_resolves_from_runtime_params() {
        // The Binding arm on speed_variance must read through ctx.runtime_params.
        // Verify by comparing the bindable's evaluate() with the host value
        // directly — the transformer wires that result into per-char threshold
        // perturbation, but the resolution itself is what packet 69-A enables.
        let bound = VfxBindableValue::Binding("typing_jitter".to_string());
        let mut params = ShaderRuntimeParams::new();
        params.insert("typing_jitter", 0.5_f32);
        let sig = SignalContext::default();
        let resolved = bound.evaluate(0.0, &sig, &params).expect("binding hit");
        assert!((resolved - 0.5).abs() < 1e-6);
        // Sanity-check that the transformer accepts the bindable and produces
        // some prefix (bound to the same params at the same progress); the
        // exact prefix is governed by the hash function and is not asserted.
        let tw = Typewriter::new(bound);
        let result = tw.transform("Hello World", 0.5, &TransformContext::new(&sig, &params));
        assert!("Hello World".starts_with(result.as_ref()));
    }

    #[test]
    fn missing_runtime_binding_falls_back_to_zero_variance() {
        // No host params supplied; the binding misses, evaluate() returns
        // None, and transform() falls back to variance=0.0 (steady reveal).
        let bound = Typewriter::new(VfxBindableValue::Binding("typing_jitter".to_string()));
        let sig = SignalContext::default();
        let params = ShaderRuntimeParams::new();
        let result = bound
            .transform("Hello World", 0.5, &TransformContext::new(&sig, &params))
            .into_owned();
        let baseline = Typewriter::new(VfxBindableValue::Literal(0.0))
            .transform("Hello World", 0.5, &TransformContext::new(&sig, &params))
            .into_owned();
        assert_eq!(
            result, baseline,
            "missing binding must match the steady-reveal baseline"
        );
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>END OF VERSION: 4.1.0</VERS>
