// <FILE>tui-vfx-compositor/src/filters/cls_glyph_timeline.rs</FILE> - <DESC>Per-cell discrete-frame, variable-dwell, one-shot glyph + color timeline filter for TTE-style scripted scenes</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>TTE effects port phase 4b — add PerCellSchedule trigger variant so author-supplied per-cell trigger arrays (e.g. from poisson_burst_schedule) drive the timeline; closes the Beams cadence-fidelity gap by letting recipe-side schedule generators compose mixed-signals primitives without inventing new filter machinery.</WCTX>
// <CLOG>0.2.0: add TimelineTrigger::PerCellSchedule { trigger_times: Arc<Vec<f64>>, width: u16 } variant — flat per-cell trigger time array indexed [y*width+x]; out-of-bounds reads return f64::INFINITY (cell never fires). Existing Immediate / PhaseOffset / Wavefront variants unchanged.</CLOG>

//! Per-cell scripted glyph + color timeline.
//!
//! [`AnimatedGlyphRamp`](super::cls_animated_glyph_ramp::AnimatedGlyphRamp)
//! cycles glyph + color in continuous lockstep, at uniform per-frame
//! dwell, looping forever. `GlyphTimeline` is the discrete-frame,
//! variable-dwell, one-shot equivalent: each cell, when "triggered,"
//! plays a `Vec<Frame>` once with explicit per-frame durations. After
//! the last frame, behavior is configurable: `Hold` (stay on the last
//! frame), `Hide` (revert the cell), or `Loop` (wrap back to frame 0).
//!
//! This is the primitive TTE Beams (per-cell beam-glyph timelines) and
//! TTE Sweep (per-cell block-cycle then settle) want. The reference
//! algorithm lives at `pro/main.rs:475-519` (`Character::tick`) with
//! frame catalogs at `pro/main.rs:1054-1067` and `pro/main.rs:1202-1222`.
//!
//! # Trigger model
//!
//! Each cell starts its timeline when `t` reaches its per-cell trigger
//! time. The three trigger variants:
//!
//! - **`Immediate`** — every cell starts at `t = 0`. Useful for
//!   scripted UI animations that play in unison.
//! - **`PhaseOffset`** — linear `base + x * x_ms + y * y_ms`. Same
//!   shape as `AnimatedGlyphRamp`'s `phase_offset_*_ms` model. Use
//!   for simple horizontal or vertical reveals.
//! - **`Wavefront`** — axis-driven sweep with optional easing and
//!   deterministic per-cell jitter. Covers six axes (left/right/top/
//!   bottom + both diagonals); easing reshapes the axis ratio (e.g.
//!   `CircInOut` for TTE Sweep's eased pacing); jitter applies a
//!   seeded position-keyed wobble (TTE Beams' per-row randomized
//!   beam speeds).
//!
//! # Frame model
//!
//! Frames carry `duration_ticks` (TTE convention: 60 ticks/sec). At
//! construction time durations are summed into cumulative end-times
//! (in seconds) so the active-frame lookup is a binary search rather
//! than a linear walk per cell per frame.
//!
//! # Completion modes
//!
//! - `Hold` — last frame stays on the cell indefinitely. TTE default.
//! - `Hide` — cell glyph + color revert (cell unchanged after end).
//!   Useful for transient effects that should leave no trace.
//! - `Loop` — timeline wraps to frame 0. Behaves like
//!   `AnimatedGlyphRamp` with explicit per-frame dwells; preferred
//!   when the dwell schedule is non-uniform.

use std::sync::Arc;

use crate::traits::filter::Filter;
use mixed_signals::random::hash_to_index;
use tui_vfx_geometry::types::EasingCurve;
use tui_vfx_types::{Cell, Color};

use super::cls_charset_noise::AffectMode;

/// Which channel(s) the timeline writes into. Mirrors
/// [`super::cls_animated_glyph_ramp::AnimatedGlyphRampApplyTo`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum GlyphTimelineApplyTo {
    /// Write into `cell.fg` only (default).
    #[default]
    Foreground,
    /// Write into `cell.bg` only.
    Background,
    /// Write into both `cell.fg` and `cell.bg`.
    Both,
}

/// What happens after the last frame's duration elapses for a cell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum TimelineCompletion {
    /// Last frame stays rendered indefinitely. TTE default.
    #[default]
    Hold,
    /// Cell is left untouched after the end (subsequent filter passes
    /// see the cell as it was before the timeline triggered).
    Hide,
    /// Timeline wraps to frame 0 and continues forever.
    Loop,
}

/// One-axis sweep direction for `TimelineTrigger::Wavefront`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WavefrontAxis {
    /// `x = 0` fires first, `x = w-1` fires last.
    LeftToRight,
    /// `x = w-1` fires first.
    RightToLeft,
    /// `y = 0` fires first.
    TopToBottom,
    /// `y = h-1` fires first.
    BottomToTop,
    /// Top-left fires first; sweep follows `(x - y)` along the
    /// `(1, -1)` direction. Bottom-right fires last.
    DiagonalTlBr,
    /// Top-right fires first; sweep follows `(x + y)` along the
    /// `(1, 1)` direction. Bottom-left fires last.
    DiagonalTrBl,
}

/// Deterministic per-cell jitter applied on top of a wavefront axis.
///
/// Jitter is keyed on `(seed, x, y)` via
/// [`mixed_signals::random::hash_to_index`] so the same `(seed, x, y)`
/// always produces the same offset — no recipe-render variance frame
/// to frame, and reproducible across processes.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct JitterConfig {
    pub seed: u64,
    /// Maximum absolute jitter applied per cell. Final per-cell offset
    /// is in `[-amount_seconds, +amount_seconds)`.
    pub amount_seconds: f64,
}

/// Configuration for a wavefront-driven trigger field.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WavefrontTriggerConfig {
    pub axis: WavefrontAxis,
    /// Total time the wavefront takes to traverse the canvas.
    pub total_duration_seconds: f64,
    /// Base offset added to every cell's trigger time.
    pub base_offset_seconds: f64,
    /// Optional reshape of the linear axis ratio before scaling to
    /// duration. `None` = linear sweep.
    pub easing: Option<EasingCurve>,
    /// Optional deterministic per-cell jitter.
    pub jitter: Option<JitterConfig>,
}

/// Source of per-cell trigger time.
///
/// `Immediate`, `PhaseOffset`, and `Wavefront` compute trigger time
/// from a small, declarative config. `PerCellSchedule` consumes a
/// pre-baked per-cell array — the recipe-side helper can compose any
/// mixed-signals signal graph (PerCharacterNoise, Keyframes, Sine,
/// physics envelopes) to produce the schedule, and the filter does
/// O(1) lookup. This is what closes the TTE Beams cadence-fidelity
/// gap: stochastic batch activation rhythms can't be expressed with
/// a single smooth wavefront, but they can be pre-baked into a Vec
/// using existing signal primitives.
#[derive(Debug, Clone, PartialEq)]
pub enum TimelineTrigger {
    /// Every cell fires at `t = 0`.
    Immediate,
    /// Linear `t_trigger(x, y) = base + x * x_ms / 1000 + y * y_ms / 1000`.
    /// Same shape as [`AnimatedGlyphRamp`](super::cls_animated_glyph_ramp::AnimatedGlyphRamp)
    /// uses for simple reveals.
    PhaseOffset {
        base_offset_seconds: f64,
        phase_offset_x_ms: f64,
        phase_offset_y_ms: f64,
    },
    /// Axis-driven sweep with optional easing and jitter.
    Wavefront(WavefrontTriggerConfig),
    /// Per-cell trigger time field, indexed `trigger_times[y * width + x]`.
    ///
    /// `width` is the canvas width the schedule was generated for;
    /// `trigger_times.len()` should equal `width * height` of that
    /// canvas. Out-of-bounds reads return `f64::INFINITY` (the cell
    /// never fires) — robust against canvas resizes that outpace
    /// schedule regeneration.
    ///
    /// `Arc` allows cheap cloning across multiple filter instances
    /// and across pipeline rebuilds; the trigger array is read-only
    /// after construction.
    PerCellSchedule {
        trigger_times: Arc<Vec<f64>>,
        width: u16,
    },
}

/// One frame in a glyph timeline.
///
/// Mirrors TTE's `FrameSpec` / `Visual` shape (`pro/main.rs:369-377`).
/// Use [`Frame::new`] which clamps `duration_ticks` to a minimum of 1
/// so the cumulative-end math is always well-defined.
/// Foreground color for a frame: either a single static color applied
/// to every cell, or a seeded palette from which the apply path picks
/// a per-cell-per-frame color via `hash_to_index`.
#[derive(Debug, Clone, PartialEq)]
pub enum FrameColor {
    Static(Color),
    Palette { colors: Vec<Color>, seed: u64 },
}

#[derive(Debug, Clone)]
pub struct Frame {
    /// `None` means "preserve the underlying cell glyph for this
    /// frame" — used for color-only frames (e.g. TTE Beams' dim-letter
    /// fade where the input character stays visible while the
    /// foreground recolors).
    pub glyph: Option<char>,
    pub fg: Option<FrameColor>,
    pub bg: Option<Color>,
    pub duration_ticks: u16,
}

impl Frame {
    /// Convenience constructor for the common case: optional static
    /// foreground + background. For palette foregrounds use
    /// [`Frame::new_with_fg`].
    pub fn new(
        glyph: Option<char>,
        fg: Option<Color>,
        bg: Option<Color>,
        duration_ticks: u16,
    ) -> Self {
        Self {
            glyph,
            fg: fg.map(FrameColor::Static),
            bg,
            duration_ticks: duration_ticks.max(1),
        }
    }

    /// Constructor accepting a `FrameColor` directly (Static or
    /// Palette). Use this when lowering a `FrameColorSpec::Palette`
    /// from the recipe schema.
    pub fn new_with_fg(
        glyph: Option<char>,
        fg: Option<FrameColor>,
        bg: Option<Color>,
        duration_ticks: u16,
    ) -> Self {
        Self {
            glyph,
            fg,
            bg,
            duration_ticks: duration_ticks.max(1),
        }
    }
}

/// Number of ticks per second. TTE convention (60 fps clock).
const TICKS_PER_SECOND: f64 = 60.0;

/// Per-cell scripted glyph + color timeline filter. See module-level
/// docs for the trigger model, frame model, and completion semantics.
pub struct GlyphTimeline {
    frames: Vec<PreparedFrame>,
    /// Sum of all `duration_seconds` — used for `Loop` wrapping and
    /// `Hide` end detection.
    total_duration_seconds: f64,
    trigger: TimelineTrigger,
    on_complete: TimelineCompletion,
    apply_to: GlyphTimelineApplyTo,
    affect: AffectMode,
}

#[derive(Debug, Clone)]
struct PreparedFrame {
    /// `None` preserves the cell's existing glyph (color-only frame).
    glyph: Option<char>,
    fg: Option<FrameColor>,
    bg: Option<Color>,
    /// Cumulative end-time in seconds: sum of `duration_seconds` for
    /// frames `[0..=self_index]`. Used for binary-search frame lookup.
    cumulative_end_seconds: f64,
}

impl GlyphTimeline {
    /// Build a new glyph timeline. Empty `frames` builds an inert
    /// no-op filter (every cell is left untouched). Non-empty frames
    /// have each `duration_ticks` clamped to `>= 1` via [`Frame::new`].
    pub fn new(
        frames: Vec<Frame>,
        trigger: TimelineTrigger,
        on_complete: TimelineCompletion,
        apply_to: GlyphTimelineApplyTo,
        affect: AffectMode,
    ) -> Self {
        let mut prepared = Vec::with_capacity(frames.len());
        let mut cumulative = 0.0;
        for f in frames {
            let dur_seconds = f.duration_ticks.max(1) as f64 / TICKS_PER_SECOND;
            cumulative += dur_seconds;
            prepared.push(PreparedFrame {
                glyph: f.glyph,
                fg: f.fg,
                bg: f.bg,
                cumulative_end_seconds: cumulative,
            });
        }
        Self {
            frames: prepared,
            total_duration_seconds: cumulative,
            trigger,
            on_complete,
            apply_to,
            affect,
        }
    }

    fn should_affect(&self, cell: &Cell) -> bool {
        match self.affect {
            AffectMode::All => true,
            AffectMode::NonEmpty => !cell.ch.is_whitespace() && cell.ch != '\u{2800}',
        }
    }

    /// Compute per-cell trigger time in seconds. Public for tests and
    /// for tooling that wants to read the same trigger field the
    /// filter would use.
    pub fn trigger_time_for(&self, x: u16, y: u16, width: u16, height: u16) -> f64 {
        match &self.trigger {
            TimelineTrigger::Immediate => 0.0,
            TimelineTrigger::PhaseOffset {
                base_offset_seconds,
                phase_offset_x_ms,
                phase_offset_y_ms,
            } => {
                base_offset_seconds
                    + (x as f64) * phase_offset_x_ms / 1000.0
                    + (y as f64) * phase_offset_y_ms / 1000.0
            }
            TimelineTrigger::Wavefront(cfg) => {
                let ratio = axis_ratio(cfg.axis, x, y, width, height);
                let eased = match cfg.easing {
                    Some(curve) => curve.ease(ratio as f64) as f64,
                    None => ratio as f64,
                };
                let mut t = cfg.base_offset_seconds + eased * cfg.total_duration_seconds;
                if let Some(j) = cfg.jitter {
                    // Hash (x, y) into one u64 so adjacent cells get
                    // different draws; modulo 2^31 so we can map into
                    // a signed offset around zero.
                    let pos_seed = ((x as u64) << 32) | (y as u64);
                    let bucket = hash_to_index(j.seed, pos_seed, 2048) as f64;
                    let signed = (bucket / 1024.0) - 1.0; // [-1, 1)
                    t += signed * j.amount_seconds;
                }
                t.max(0.0)
            }
            TimelineTrigger::PerCellSchedule {
                trigger_times,
                width: schedule_width,
            } => {
                // O(1) lookup. Out-of-bounds (cell beyond what the
                // schedule was generated for) returns INFINITY so the
                // cell never fires — robust against canvas resize
                // outpacing schedule regen.
                let idx = (y as usize) * (*schedule_width as usize) + (x as usize);
                trigger_times.get(idx).copied().unwrap_or(f64::INFINITY)
            }
        }
    }

    /// Return the active frame index at local time `t_local_seconds`,
    /// applying the configured completion mode. Returns `None` if the
    /// timeline is `Hide`-completing and `t_local` is past the end
    /// (caller should leave the cell untouched).
    fn active_frame_index(&self, t_local_seconds: f64) -> Option<usize> {
        if self.frames.is_empty() {
            return None;
        }
        let total = self.total_duration_seconds;
        if t_local_seconds >= total {
            match self.on_complete {
                TimelineCompletion::Hold => return Some(self.frames.len() - 1),
                TimelineCompletion::Hide => return None,
                TimelineCompletion::Loop => {
                    let wrapped = if total > 0.0 {
                        t_local_seconds.rem_euclid(total)
                    } else {
                        0.0
                    };
                    return Some(self.find_frame(wrapped));
                }
            }
        }
        Some(self.find_frame(t_local_seconds.max(0.0)))
    }

    fn find_frame(&self, t: f64) -> usize {
        // Binary search for first frame whose cumulative_end > t.
        // We can't use slice::binary_search_by directly with f64 (no Ord),
        // so do it by hand. Frames is monotonically ascending.
        let mut lo = 0;
        let mut hi = self.frames.len();
        while lo < hi {
            let mid = (lo + hi) / 2;
            if self.frames[mid].cumulative_end_seconds <= t {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }
        lo.min(self.frames.len() - 1)
    }
}

impl Filter for GlyphTimeline {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        if self.frames.is_empty() || !self.should_affect(cell) {
            return;
        }
        let trigger = self.trigger_time_for(x, y, width, height);
        let t_local = t - trigger;
        if t_local < 0.0 {
            return; // cell hasn't fired yet
        }
        let Some(idx) = self.active_frame_index(t_local) else {
            return; // Hide-completed past end
        };
        let frame = &self.frames[idx];
        if let Some(g) = frame.glyph {
            cell.ch = g;
        }
        let resolved_fg = frame.fg.as_ref().map(|fc| match fc {
            FrameColor::Static(c) => *c,
            FrameColor::Palette { colors, seed } => {
                let pos_seed = ((x as u64) << 32) | (y as u64);
                let frame_seed = pos_seed
                    .wrapping_mul(0x9E37_79B9_7F4A_7C15)
                    .wrapping_add(idx as u64);
                let bucket = hash_to_index(*seed, frame_seed, colors.len().max(1));
                colors[bucket]
            }
        });
        match self.apply_to {
            GlyphTimelineApplyTo::Foreground => {
                if let Some(c) = resolved_fg {
                    cell.fg = c;
                }
            }
            GlyphTimelineApplyTo::Background => {
                if let Some(c) = frame.bg {
                    cell.bg = c;
                }
            }
            GlyphTimelineApplyTo::Both => {
                if let Some(c) = resolved_fg {
                    cell.fg = c;
                }
                if let Some(c) = frame.bg {
                    cell.bg = c;
                }
            }
        }
    }
}

/// Axis ratio in `[0, 1]` for the given axis variant at `(x, y)` on a
/// `(width, height)` canvas.
fn axis_ratio(axis: WavefrontAxis, x: u16, y: u16, width: u16, height: u16) -> f32 {
    let w = (width.max(1) - 1).max(1) as f32;
    let h = (height.max(1) - 1).max(1) as f32;
    match axis {
        WavefrontAxis::LeftToRight => x as f32 / w,
        WavefrontAxis::RightToLeft => 1.0 - (x as f32 / w),
        WavefrontAxis::TopToBottom => y as f32 / h,
        WavefrontAxis::BottomToTop => 1.0 - (y as f32 / h),
        WavefrontAxis::DiagonalTlBr => {
            // (x - y) ranges from -(h-1) to (w-1); shift by h-1, divide
            // by (w-1)+(h-1) to get [0, 1].
            let raw = x as f32 - y as f32;
            let shifted = raw + h;
            (shifted / (w + h)).clamp(0.0, 1.0)
        }
        WavefrontAxis::DiagonalTrBl => {
            // (x + y) ranges 0..(w-1)+(h-1); divide by (w-1)+(h-1).
            ((x as f32 + y as f32) / (w + h)).clamp(0.0, 1.0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rgb(r: u8, g: u8, b: u8) -> Color {
        Color::rgb(r, g, b)
    }

    fn make_cell(ch: char) -> Cell {
        Cell {
            ch,
            ..Cell::default()
        }
    }

    fn three_frames() -> Vec<Frame> {
        vec![
            Frame::new(Some('A'), Some(rgb(255, 0, 0)), None, 6), // 0.10s
            Frame::new(Some('B'), Some(rgb(0, 255, 0)), None, 12), // 0.20s
            Frame::new(Some('C'), Some(rgb(0, 0, 255)), None, 6), // 0.10s
        ]
        // total = 24 ticks = 0.40s
    }

    fn immediate_hold(frames: Vec<Frame>) -> GlyphTimeline {
        GlyphTimeline::new(
            frames,
            TimelineTrigger::Immediate,
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        )
    }

    #[test]
    fn cell_at_t_zero_shows_first_frame_when_immediate() {
        let tl = immediate_hold(three_frames());
        let mut cell = make_cell('⣿');
        tl.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, 'A');
        assert_eq!(cell.fg, rgb(255, 0, 0));
    }

    #[test]
    fn timeline_advances_at_cumulative_durations() {
        // Frame durations: A=6t (0.10s), B=12t (0.20s), C=6t (0.10s).
        // Use interior times rather than exact boundaries so we don't
        // bump into f64 rounding at frame edges (e.g. 0.1+0.2 in f64 is
        // 0.3000...04, while the literal 0.3 is 0.2999...88).
        let tl = immediate_hold(three_frames());
        let mut cell = make_cell('⣿');
        // Mid-frame 0: 'A'
        tl.apply(&mut cell, 0, 0, 10, 10, 0.05);
        assert_eq!(cell.ch, 'A');
        // Mid-frame 1: 'B'
        cell.ch = '⣿';
        tl.apply(&mut cell, 0, 0, 10, 10, 0.20);
        assert_eq!(cell.ch, 'B');
        // Mid-frame 2: 'C'
        cell.ch = '⣿';
        tl.apply(&mut cell, 0, 0, 10, 10, 0.35);
        assert_eq!(cell.ch, 'C');
    }

    #[test]
    fn hold_mode_keeps_last_frame_past_end() {
        let tl = immediate_hold(three_frames());
        let mut cell = make_cell('⣿');
        tl.apply(&mut cell, 0, 0, 10, 10, 100.0);
        assert_eq!(cell.ch, 'C');
        assert_eq!(cell.fg, rgb(0, 0, 255));
    }

    #[test]
    fn hide_mode_leaves_cell_untouched_past_end() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Immediate,
            TimelineCompletion::Hide,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        let mut cell = make_cell('#');
        cell.fg = rgb(50, 50, 50);
        tl.apply(&mut cell, 0, 0, 10, 10, 100.0);
        assert_eq!(cell.ch, '#'); // unchanged
        assert_eq!(cell.fg, rgb(50, 50, 50));
    }

    #[test]
    fn loop_mode_wraps() {
        // total = 0.40s. Sample interior times rather than exact wrap
        // boundaries (0.40 in f64 is slightly less than the cumulative
        // 0.10+0.20+0.10 sum, so testing exactly at the boundary is
        // ambiguous).
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Immediate,
            TimelineCompletion::Loop,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // t=0.45 wraps to 0.05 → mid-frame 0 → 'A'
        let mut cell = make_cell('⣿');
        tl.apply(&mut cell, 0, 0, 10, 10, 0.45);
        assert_eq!(cell.ch, 'A');
        // t=0.60 wraps to 0.20 → mid-frame 1 → 'B'
        cell.ch = '⣿';
        tl.apply(&mut cell, 0, 0, 10, 10, 0.60);
        assert_eq!(cell.ch, 'B');
        // t=0.75 wraps to 0.35 → mid-frame 2 → 'C'
        cell.ch = '⣿';
        tl.apply(&mut cell, 0, 0, 10, 10, 0.75);
        assert_eq!(cell.ch, 'C');
    }

    #[test]
    fn cell_before_phase_offset_trigger_is_unchanged() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::PhaseOffset {
                base_offset_seconds: 0.0,
                phase_offset_x_ms: 100.0, // 0.1s per column
                phase_offset_y_ms: 0.0,
            },
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        let mut cell = make_cell('#');
        // Column 5 fires at 0.5s; at t=0.0 it shouldn't have started.
        tl.apply(&mut cell, 5, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, '#');
    }

    #[test]
    fn phase_offset_makes_columns_sweep() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::PhaseOffset {
                base_offset_seconds: 0.0,
                phase_offset_x_ms: 100.0,
                phase_offset_y_ms: 0.0,
            },
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // At t=0.5s: column 0 trigger=0, t_local=0.5 -> frame 2 ('C')
        let mut c0 = make_cell('⣿');
        tl.apply(&mut c0, 0, 0, 10, 10, 0.5);
        assert_eq!(c0.ch, 'C');
        // Column 5 trigger=0.5, t_local=0.0 -> frame 0 ('A')
        let mut c5 = make_cell('⣿');
        tl.apply(&mut c5, 5, 0, 10, 10, 0.5);
        assert_eq!(c5.ch, 'A');
    }

    #[test]
    fn wavefront_left_to_right_linear() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(WavefrontTriggerConfig {
                axis: WavefrontAxis::LeftToRight,
                total_duration_seconds: 1.0,
                base_offset_seconds: 0.0,
                easing: None,
                jitter: None,
            }),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // Column 0 of width=11: trigger=0
        assert!((tl.trigger_time_for(0, 0, 11, 5) - 0.0).abs() < 1e-6);
        // Column 10 of width=11: trigger=1.0
        assert!((tl.trigger_time_for(10, 0, 11, 5) - 1.0).abs() < 1e-6);
        // Column 5: trigger=0.5
        assert!((tl.trigger_time_for(5, 0, 11, 5) - 0.5).abs() < 1e-3);
    }

    #[test]
    fn wavefront_right_to_left_inverts() {
        let cfg = WavefrontTriggerConfig {
            axis: WavefrontAxis::RightToLeft,
            total_duration_seconds: 1.0,
            base_offset_seconds: 0.0,
            easing: None,
            jitter: None,
        };
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(cfg),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // Column 0 of width=11: trigger=1.0 (last)
        assert!((tl.trigger_time_for(0, 0, 11, 5) - 1.0).abs() < 1e-6);
        // Column 10: trigger=0.0 (first)
        assert!((tl.trigger_time_for(10, 0, 11, 5) - 0.0).abs() < 1e-6);
    }

    #[test]
    fn wavefront_diagonal_tl_br_far_corners() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(WavefrontTriggerConfig {
                axis: WavefrontAxis::DiagonalTlBr,
                total_duration_seconds: 1.0,
                base_offset_seconds: 0.0,
                easing: None,
                jitter: None,
            }),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // Top-right corner (max x-y) → ratio 1.0
        let tr = tl.trigger_time_for(10, 0, 11, 5);
        // Bottom-left corner (min x-y) → ratio 0.0
        let bl = tl.trigger_time_for(0, 4, 11, 5);
        assert!(tr > bl, "TR ({tr}) should fire after BL ({bl})");
    }

    #[test]
    fn wavefront_easing_circ_in_out_at_midpoint() {
        let cfg = WavefrontTriggerConfig {
            axis: WavefrontAxis::LeftToRight,
            total_duration_seconds: 1.0,
            base_offset_seconds: 0.0,
            easing: Some(EasingCurve::Type(
                tui_vfx_geometry::easing::EasingType::CircInOut,
            )),
            jitter: None,
        };
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(cfg),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // CircInOut(0.5) = 0.5 (symmetric); column at exact midpoint
        // should land at trigger=0.5.
        let mid = tl.trigger_time_for(5, 0, 11, 5);
        assert!((mid - 0.5).abs() < 1e-3, "midpoint {mid} should be ~0.5");
    }

    #[test]
    fn wavefront_jitter_is_deterministic_for_same_seed() {
        let cfg = WavefrontTriggerConfig {
            axis: WavefrontAxis::LeftToRight,
            total_duration_seconds: 1.0,
            base_offset_seconds: 0.0,
            easing: None,
            jitter: Some(JitterConfig {
                seed: 42,
                amount_seconds: 0.1,
            }),
        };
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(cfg),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        let a = tl.trigger_time_for(5, 2, 11, 5);
        let b = tl.trigger_time_for(5, 2, 11, 5);
        assert_eq!(a, b);
    }

    #[test]
    fn wavefront_jitter_differs_by_position() {
        let cfg = WavefrontTriggerConfig {
            axis: WavefrontAxis::LeftToRight,
            total_duration_seconds: 1.0,
            base_offset_seconds: 0.0,
            easing: None,
            jitter: Some(JitterConfig {
                seed: 42,
                amount_seconds: 0.1,
            }),
        };
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(cfg),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        let a = tl.trigger_time_for(5, 2, 11, 5);
        let b = tl.trigger_time_for(6, 2, 11, 5);
        assert_ne!(a, b, "adjacent cells should jitter differently");
    }

    #[test]
    fn wavefront_never_returns_negative_time() {
        let cfg = WavefrontTriggerConfig {
            axis: WavefrontAxis::LeftToRight,
            total_duration_seconds: 1.0,
            base_offset_seconds: 0.0,
            easing: None,
            jitter: Some(JitterConfig {
                seed: 1,
                amount_seconds: 5.0, // huge jitter
            }),
        };
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Wavefront(cfg),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        for x in 0..11 {
            for y in 0..5 {
                let t = tl.trigger_time_for(x, y, 11, 5);
                assert!(t >= 0.0, "negative trigger {t} at ({x},{y})");
            }
        }
    }

    #[test]
    fn affect_non_empty_skips_whitespace() {
        let tl = GlyphTimeline::new(
            three_frames(),
            TimelineTrigger::Immediate,
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::NonEmpty,
        );
        let mut space = make_cell(' ');
        let mut text = make_cell('X');
        tl.apply(&mut space, 0, 0, 10, 10, 0.0);
        tl.apply(&mut text, 0, 0, 10, 10, 0.0);
        assert_eq!(space.ch, ' '); // skipped
        assert_eq!(text.ch, 'A'); // first frame applied
    }

    #[test]
    fn apply_to_background_writes_bg_only() {
        let frames = vec![Frame::new(Some('X'), Some(rgb(99, 0, 0)), Some(rgb(0, 99, 0)), 5)];
        let tl = GlyphTimeline::new(
            frames,
            TimelineTrigger::Immediate,
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Background,
            AffectMode::All,
        );
        let mut cell = make_cell('⣿');
        cell.fg = rgb(1, 1, 1);
        tl.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, 'X');
        assert_eq!(cell.fg, rgb(1, 1, 1), "fg untouched");
        assert_eq!(cell.bg, rgb(0, 99, 0));
    }

    #[test]
    fn empty_frames_is_noop() {
        let tl = immediate_hold(vec![]);
        let mut cell = make_cell('⣿');
        cell.fg = rgb(1, 2, 3);
        tl.apply(&mut cell, 0, 0, 10, 10, 0.5);
        assert_eq!(cell.ch, '⣿');
        assert_eq!(cell.fg, rgb(1, 2, 3));
    }

    #[test]
    fn frame_new_clamps_zero_duration() {
        let f = Frame::new(Some('A'), None, None, 0);
        assert_eq!(f.duration_ticks, 1);
    }

    #[test]
    fn deterministic() {
        let tl = immediate_hold(three_frames());
        let mut a = make_cell('⣿');
        let mut b = make_cell('⣿');
        tl.apply(&mut a, 3, 4, 10, 10, 0.123);
        tl.apply(&mut b, 3, 4, 10, 10, 0.123);
        assert_eq!(a.ch, b.ch);
        assert_eq!(a.fg, b.fg);
    }

    // --- PerCellSchedule trigger variant ---

    fn schedule_trigger(times: Vec<f64>, width: u16) -> TimelineTrigger {
        TimelineTrigger::PerCellSchedule {
            trigger_times: Arc::new(times),
            width,
        }
    }

    #[test]
    fn per_cell_schedule_lookup_drives_trigger_time() {
        // 3x2 schedule: cell (0,0)=0.0, (1,0)=0.5, (2,0)=1.0,
        //               (0,1)=0.25, (1,1)=0.75, (2,1)=1.5
        let times = vec![0.0, 0.5, 1.0, 0.25, 0.75, 1.5];
        let tl = GlyphTimeline::new(
            three_frames(),
            schedule_trigger(times, 3),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        assert_eq!(tl.trigger_time_for(0, 0, 3, 2), 0.0);
        assert_eq!(tl.trigger_time_for(1, 0, 3, 2), 0.5);
        assert_eq!(tl.trigger_time_for(2, 0, 3, 2), 1.0);
        assert_eq!(tl.trigger_time_for(0, 1, 3, 2), 0.25);
        assert_eq!(tl.trigger_time_for(1, 1, 3, 2), 0.75);
        assert_eq!(tl.trigger_time_for(2, 1, 3, 2), 1.5);
    }

    #[test]
    fn per_cell_schedule_out_of_bounds_returns_infinity() {
        // Schedule for a 2x2 canvas. Asking for (5, 5) should yield
        // f64::INFINITY (cell never fires) rather than panic.
        let times = vec![0.0, 0.0, 0.0, 0.0];
        let tl = GlyphTimeline::new(
            three_frames(),
            schedule_trigger(times, 2),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        assert_eq!(tl.trigger_time_for(5, 5, 2, 2), f64::INFINITY);
    }

    #[test]
    fn per_cell_schedule_drives_apply_correctly() {
        // Cell (0,0) fires at 0.0; cell (1,0) fires at 0.5.
        // Timeline is 0.40s total (A:0.10, B:0.20, C:0.10).
        let times = vec![0.0, 0.5];
        let tl = GlyphTimeline::new(
            three_frames(),
            schedule_trigger(times, 2),
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        // At t=0.05: (0,0) t_local=0.05 → frame 0 ('A'); (1,0) hasn't fired.
        let mut c00 = make_cell('⣿');
        let mut c10 = make_cell('⣿');
        tl.apply(&mut c00, 0, 0, 2, 1, 0.05);
        tl.apply(&mut c10, 1, 0, 2, 1, 0.05);
        assert_eq!(c00.ch, 'A');
        assert_eq!(
            c10.ch, '⣿',
            "cell (1,0) trigger=0.5; at t=0.05 not yet fired"
        );
        // At t=0.55: (0,0) t_local=0.55 → past 0.40s end → Hold = 'C'.
        //           (1,0) t_local=0.05 → frame 0 ('A').
        let mut c00 = make_cell('⣿');
        let mut c10 = make_cell('⣿');
        tl.apply(&mut c00, 0, 0, 2, 1, 0.55);
        tl.apply(&mut c10, 1, 0, 2, 1, 0.55);
        assert_eq!(c00.ch, 'C', "(0,0) past end, Hold = last frame");
        assert_eq!(c10.ch, 'A', "(1,0) just fired");
    }

    #[test]
    fn per_cell_schedule_arc_clones_cheaply() {
        // Verify we can share the same Arc across multiple filter
        // instances without re-allocating the schedule. (Mostly a
        // type-system check; the asserts confirm the lookup still
        // returns the same value.)
        let times = Arc::new(vec![0.5, 1.5, 2.5]);
        let trigger_a = TimelineTrigger::PerCellSchedule {
            trigger_times: times.clone(),
            width: 3,
        };
        let trigger_b = TimelineTrigger::PerCellSchedule {
            trigger_times: times.clone(),
            width: 3,
        };
        assert_eq!(Arc::strong_count(&times), 3);
        let tl_a = GlyphTimeline::new(
            three_frames(),
            trigger_a,
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        let tl_b = GlyphTimeline::new(
            three_frames(),
            trigger_b,
            TimelineCompletion::Hold,
            GlyphTimelineApplyTo::Foreground,
            AffectMode::All,
        );
        assert_eq!(tl_a.trigger_time_for(1, 0, 3, 1), 1.5);
        assert_eq!(tl_b.trigger_time_for(1, 0, 3, 1), 1.5);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_glyph_timeline.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
