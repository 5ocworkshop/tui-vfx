// <FILE>tui-vfx-content/src/cursor/fnc_render_cursor.rs</FILE> - <DESC>Render cursor state into paint ops</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>feat/cursor-primitive: render — primary op (T16)</WCTX>
// <CLOG>Initial impl (primary only, wake follows in T17-T18)</CLOG>

use super::{
    fnc_cursor_grow_in_glyph, Cursor, CursorPaintOps, CursorState, GrowInPhase, PrimaryOp,
};
use mixed_signals::prelude::{SignalContext, SignalOrFloat};

/// Sample a curve signal, treating the default `Static(1.0)` as linear identity.
fn sample_curve(curve: &SignalOrFloat, t: f32, ctx: &SignalContext) -> f32 {
    if matches!(curve, SignalOrFloat::Static(v) if (*v - 1.0).abs() < 1e-6) {
        return t.clamp(0.0, 1.0);
    }
    curve
        .evaluate(t as f64, ctx)
        .unwrap_or(t)
        .clamp(0.0, 1.0)
}

/// Render cursor state to paint ops for the current frame.
///
/// The returned ops describe what the consumer should paint:
/// - `primary`: current cursor cell (may be a partial grow-in glyph).
/// - `trail`: wake cells (empty until T17 implements wake painting).
///
/// `now` is wall-clock seconds, matching the value passed to
/// [`crate::cursor::fnc_advance_cursor`].
pub fn fnc_render_cursor(
    state: &CursorState,
    cursor: &Cursor,
    now: f64,
    ctx: &SignalContext,
) -> CursorPaintOps {
    let mut ops = CursorPaintOps::default();

    let pos = match state.position {
        Some(p) => p,
        None => return ops,
    };

    if cursor.character.is_empty() {
        return ops;
    }

    let primary = match state.grow_in_phase {
        GrowInPhase::Hidden => None,
        GrowInPhase::Visible => {
            let alpha = clamp_unit(
                cursor.visibility.evaluate(now, ctx).unwrap_or(1.0).clamp(0.0, 1.0),
            );
            if alpha <= 0.0 {
                None
            } else {
                Some(PrimaryOp { position: pos, glyph: cursor.character.clone(), alpha })
            }
        }
        GrowInPhase::GrowingIn { elapsed_ms } => {
            let duration_ms = cursor
                .grow_in
                .duration_ms
                .evaluate(now, ctx)
                .unwrap_or(0.0)
                .max(1e-6) as f64;
            let t = (elapsed_ms / duration_ms).clamp(0.0, 1.0) as f32;
            let eased = sample_curve(&cursor.grow_in.curve, t, ctx);
            let (glyph, alpha) =
                fnc_cursor_grow_in_glyph(&cursor.character, eased, cursor.grow_in.direction);
            if glyph.is_empty() || alpha <= 0.0 {
                None
            } else {
                Some(PrimaryOp { position: pos, glyph, alpha })
            }
        }
        GrowInPhase::GrowingOut { elapsed_ms } => {
            let grow_out_ms = cursor
                .grow_in
                .grow_out_ms
                .evaluate(now, ctx)
                .unwrap_or(0.0)
                .max(1e-6) as f64;
            let t_out = (elapsed_ms / grow_out_ms).clamp(0.0, 1.0) as f32;
            let eased_out = sample_curve(&cursor.grow_in.curve, t_out, ctx);
            let reverse = 1.0 - eased_out;
            let (glyph, alpha) =
                fnc_cursor_grow_in_glyph(&cursor.character, reverse, cursor.grow_in.direction);
            if glyph.is_empty() || alpha <= 0.0 {
                None
            } else {
                Some(PrimaryOp { position: pos, glyph, alpha })
            }
        }
    };
    ops.primary = primary;
    ops
}

fn clamp_unit(v: f32) -> f32 {
    if v.is_finite() { v.clamp(0.0, 1.0) } else { 0.0 }
}

// <FILE>tui-vfx-content/src/cursor/fnc_render_cursor.rs</FILE> - <DESC>Render cursor state into paint ops</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
