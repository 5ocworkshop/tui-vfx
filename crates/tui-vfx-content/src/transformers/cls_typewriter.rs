// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>VERSION: 3.0.1</VERS>
// <WCTX>feat/cursor-primitive T31: clippy clean-up (doc-overindented-list-items on the transform_with_cursor argument list; too_many_arguments for the same method is unavoidable since the cursor API needs cursor + state + now + dt on top of the base transform signature — mark with #[allow])</WCTX>
// <CLOG>PATCH: unindent argument bullet wrap lines; annotate transform_with_cursor with #[allow(clippy::too_many_arguments)] — the 8-arg signature is the cursor-primitive's canonical shape and not splittable without hiding state</CLOG>

use crate::traits::TextTransformer;
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;

/// Typewriter effect that reveals text character-by-character.
///
/// Supports optional speed variance for organic, human-like typing rhythm.
#[derive(Debug, Clone)]
pub struct Typewriter {
    /// Per-character timing variance (0.0 = uniform, higher = more variation)
    /// Can be static or signal-driven for time-varying effects.
    pub speed_variance: SignalOrFloat,
}

impl Typewriter {
    pub fn new(speed_variance: SignalOrFloat) -> Self {
        Self { speed_variance }
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            speed_variance: SignalOrFloat::Static(0.0),
        }
    }
}

impl TextTransformer for Typewriter {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
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

        // Evaluate speed_variance signal per-frame (unwrap with fallback to 0.0 on error)
        let variance = f64::from(
            self.speed_variance
                .evaluate(progress, signal_ctx)
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
                let hash_input = signal_ctx.seed.wrapping_add(i as u64);
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
    /// * `target`     — Full text being revealed.
    /// * `progress`   — Reveal progress 0..1 (same semantics as [`Typewriter`]'s
    ///   [`TextTransformer::transform`] impl).
    /// * `signal_ctx` — Signal evaluation context.
    /// * `cursor`     — Cursor configuration (usually `tcursor.cursor`).
    /// * `state`      — Mutable cursor state owned by the caller.
    /// * `now`        — Wall-clock seconds (same value used for signal sampling).
    /// * `dt`         — Wall-clock seconds since the previous frame.
    #[allow(clippy::too_many_arguments)]
    pub fn transform_with_cursor<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
        cursor: &Cursor,
        state: &mut CursorState,
        now: f64,
        dt: f64,
    ) -> (Cow<'a, str>, CursorPaintOps) {
        // Run the base reveal.
        let revealed = self.transform(target, progress, signal_ctx);

        // Resolve cursor position from progress and advance the primitive.
        let idx = fnc_typewriter_cursor_position(target, progress).unwrap_or(0);
        // Row is always 0 for a single-line typewriter reveal; callers wrapping
        // multi-line text must drive the cursor externally.
        let pos = (0u16, idx as u16);
        fnc_advance_cursor(state, cursor, Some(pos), now, dt, signal_ctx);

        // Render paint ops and splice the primary glyph in.
        let ops = fnc_render_cursor(state, cursor, now, signal_ctx);
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

// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>END OF VERSION: 3.0.1</VERS>
