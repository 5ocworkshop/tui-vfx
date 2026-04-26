// <FILE>tui-vfx-content/src/cursor/fnc_render_cursor.rs</FILE> - <DESC>Render cursor state into paint ops</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>feat/cursor-scan: while the cursor is parked (GrowInPhase::Visible) and scan is enabled, override the primary glyph with fnc_cursor_scan_glyph. Grow-in and grow-out still win within their windows so scan never fights the shape animation. Scan is only meaningful with period_ms > 0.</WCTX>
// <CLOG>MINOR: wire ScanMode into primary_op's Visible branch. phase = (now_ms % period_ms) / period_ms; passed through fnc_cursor_scan_glyph. period_ms ≤ 0 or ScanMode::Off is a no-op.</CLOG>

use super::{
    Cursor, CursorPaintOps, CursorState, GrowInPhase, PrimaryOp, ScanMode, TrailOp, WakeMode,
    fnc_cursor_grow_in_glyph, fnc_cursor_scan_glyph,
};
use mixed_signals::prelude::{SignalContext, SignalOrFloat};

/// Sample a grow-in curve, treating the default `Static(1.0)` as linear identity
/// (`t` — a no-op easing that produces the raw progress).
fn sample_grow_in_curve(curve: &SignalOrFloat, t: f32, ctx: &SignalContext) -> f32 {
    if matches!(curve, SignalOrFloat::Static(v) if (*v - 1.0).abs() < 1e-6) {
        return t.clamp(0.0, 1.0);
    }
    curve.evaluate(t as f64, ctx).unwrap_or(t).clamp(0.0, 1.0)
}

/// Sample a wake decay curve, treating the default `Static(1.0)` as linear decay
/// (`1 - t` — full alpha at age 0, zero alpha at decay boundary).
fn sample_wake_curve(curve: &SignalOrFloat, t: f32, ctx: &SignalContext) -> f32 {
    if matches!(curve, SignalOrFloat::Static(v) if (*v - 1.0).abs() < 1e-6) {
        return (1.0 - t).clamp(0.0, 1.0);
    }
    curve
        .evaluate(t as f64, ctx)
        .unwrap_or(1.0 - t)
        .clamp(0.0, 1.0)
}

/// Render cursor state to paint ops for the current frame.
///
/// The returned ops describe what the consumer should paint:
/// - `primary`: current cursor cell (may be a partial grow-in glyph).
/// - `trail`: wake cells aged into the decay window.
///
/// `now` is wall-clock seconds, matching the value passed to
/// [`crate::cursor::fnc_advance_cursor()`].
pub fn fnc_render_cursor(
    state: &CursorState,
    cursor: &Cursor,
    now: f64,
    ctx: &SignalContext,
) -> CursorPaintOps {
    let mut ops = CursorPaintOps::default();

    // --- Primary op (current cell) ---
    if let Some(pos) = state.position
        && !cursor.character.is_empty()
    {
        ops.primary = primary_op(state, cursor, pos, now, ctx);
    }

    // --- Trail ops (wake) ---
    if !matches!(cursor.wake.mode, WakeMode::Off) {
        let decay = cursor
            .wake
            .decay_seconds
            .evaluate(now, ctx)
            .unwrap_or(0.0)
            .max(0.0) as f64;
        if decay > 0.0 {
            for entry in state.history.iter() {
                let age = (now - entry.2).max(0.0);
                let age_t = (age / decay).clamp(0.0, 1.0) as f32;
                // curve is sampled with t in 0..1; decays from 1.0 at t=0 to 0.0 at t=1.
                let alpha = sample_wake_curve(&cursor.wake.curve, age_t, ctx);
                if alpha <= 0.0 {
                    continue;
                }
                match cursor.wake.mode {
                    WakeMode::Tint => ops.trail.push(TrailOp {
                        position: (entry.0, entry.1),
                        glyph: None,
                        alpha,
                    }),
                    WakeMode::Ghost => ops.trail.push(TrailOp {
                        position: (entry.0, entry.1),
                        glyph: Some(cursor.character.clone()),
                        alpha,
                    }),
                    WakeMode::Off => unreachable!(),
                }
            }
        }
    }

    ops
}

fn primary_op(
    state: &CursorState,
    cursor: &Cursor,
    pos: (u16, u16),
    now: f64,
    ctx: &SignalContext,
) -> Option<PrimaryOp> {
    match state.grow_in_phase {
        GrowInPhase::Hidden => None,
        GrowInPhase::Visible => {
            let alpha = clamp_unit(
                cursor
                    .visibility
                    .evaluate(now, ctx)
                    .unwrap_or(1.0)
                    .clamp(0.0, 1.0),
            );
            if alpha <= 0.0 {
                None
            } else {
                // Scan override — only in the Visible phase. Grow-in/out
                // take precedence above and are not touched.
                let glyph = if matches!(cursor.scan.mode, ScanMode::Off) {
                    cursor.character.clone()
                } else {
                    let period_ms = cursor
                        .scan
                        .period_ms
                        .evaluate(now, ctx)
                        .unwrap_or(0.0)
                        .max(0.0) as f64;
                    if period_ms <= 0.0 {
                        cursor.character.clone()
                    } else {
                        // now is seconds; period_ms is milliseconds.
                        let now_ms = now * 1000.0;
                        let phase = ((now_ms.rem_euclid(period_ms)) / period_ms) as f32;
                        fnc_cursor_scan_glyph(&cursor.character, phase, cursor.scan.mode)
                    }
                };
                Some(PrimaryOp {
                    position: pos,
                    glyph,
                    alpha,
                })
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
            let eased = sample_grow_in_curve(&cursor.grow_in.curve, t, ctx);
            let (glyph, alpha) =
                fnc_cursor_grow_in_glyph(&cursor.character, eased, cursor.grow_in.direction);
            if glyph.is_empty() || alpha <= 0.0 {
                None
            } else {
                Some(PrimaryOp {
                    position: pos,
                    glyph,
                    alpha,
                })
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
            let eased_out = sample_grow_in_curve(&cursor.grow_in.curve, t_out, ctx);
            let reverse = 1.0 - eased_out;
            let (glyph, alpha) =
                fnc_cursor_grow_in_glyph(&cursor.character, reverse, cursor.grow_in.direction);
            if glyph.is_empty() || alpha <= 0.0 {
                None
            } else {
                Some(PrimaryOp {
                    position: pos,
                    glyph,
                    alpha,
                })
            }
        }
    }
}

fn clamp_unit(v: f32) -> f32 {
    if v.is_finite() {
        v.clamp(0.0, 1.0)
    } else {
        0.0
    }
}

// <FILE>tui-vfx-content/src/cursor/fnc_render_cursor.rs</FILE> - <DESC>Render cursor state into paint ops</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
