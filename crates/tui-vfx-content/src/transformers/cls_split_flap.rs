// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE> - <DESC>SplitFlap transformer with Solari-board mechanical feel (cycles, jitter, charset, hinge, spring, authentic timing, message transitions, flip-preview glyph, hinge-window flicker, per-column dispersion patterns)</DESC>
// <VERS>VERSION: 3.6.0</VERS>
// <WCTX>Packet 69-A: speed, cascade, cycles are now VfxBindableValue so hosts can drive flap rate / cascade-stagger / pool-cycles via runtime bindings.</WCTX>
// <CLOG>3.6.0: MINOR — speed, cascade, cycles fields + new/new_mechanical constructor signatures + solari_preset + Default impl all migrate from SignalOrFloat to VfxBindableValue. Three transform-time evaluate calls now pass ctx.runtime_params. estimated_flap_ms pattern-matches the Literal arm directly (no helper added per rule of three).</CLOG>

use crate::mechanical::{
    MechanicalSizing, MechanicalTile, grid_to_text, paired_grids, split_flap_tile_frame,
    validate_split_flap_tile,
};
use crate::traits::{TextTransformer, TransformContext};
use crate::utils::char_turn;
use mixed_signals::physics::DampedSpring;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use tui_vfx_core::bindable::{VfxBindable, VfxBindableValue};
use tui_vfx_types::Grid;

/// Character pool the flap cycles through.
///
/// `Alpha` is the classic Solari pool (space, A-Z, 0-9, punctuation).
/// `Digits` is the right choice for numeric-only displays like flight
/// numbers and clock times — you don't want "FLIGHT 772" flipping through
/// letters on the digit positions. `Uppercase` is A-Z for letter-only
/// displays like station names.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SplitFlapCharset {
    /// Space, A-Z, 0-9, and common punctuation (default — matches 2.1.0).
    #[default]
    Alpha,
    /// Digits only (0-9 + space).
    Digits,
    /// Letters only (A-Z + space).
    Uppercase,
}

/// Opening block-density sequence used by `leading_blocks`. Because `█`
/// fills the cell with the terminal fg color, this sequence flashes a
/// "full fg-color cursor" before any letters appear.
const BLOCK_CHARS: &[char] = &['█', '▓', '▒', '░'];

/// Landing-hinge rotation used by `settle_hinge` — simulates a physical
/// Solari flap rotating through its top hinge:
/// - `█` full block (flap face arriving)
/// - `▀` upper half block (top half of old card falling)
/// - `▔` upper one-eighth block (top edge, card thinning)
/// - `—` em dash (edge-on at the hinge)
/// - `▁` lower one-eighth block (bottom edge of new card emerging)
/// - `▄` lower half block (bottom half rising into place)
const HINGE_CHARS: &[char] = &['█', '▀', '▔', '—', '▁', '▄'];

const ALPHA_CHARS: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.',
    ',', '-', '!', '?',
];
const DIGIT_CHARS: &[char] = &[' ', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9'];
const UPPER_CHARS: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

impl SplitFlapCharset {
    fn chars(self) -> &'static [char] {
        match self {
            SplitFlapCharset::Alpha => ALPHA_CHARS,
            SplitFlapCharset::Digits => DIGIT_CHARS,
            SplitFlapCharset::Uppercase => UPPER_CHARS,
        }
    }
}

/// Dispersion pattern: how per-column start delays are distributed across
/// the message. Controls the visual shape of "when each flap begins
/// rotating" — the wave vs popcorn vs ripple distinction that reads as
/// the board's personality.
///
/// `Legacy` (default) preserves every pre-3.2.0 recipe byte-for-byte: it
/// honors the existing `cascade` and `authentic_timing` fields. All other
/// variants override those and produce their own delay curve.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SplitFlapDispersion {
    /// Honor the pre-3.2.0 `cascade` + `authentic_timing` fields. Default
    /// for backward compatibility — no existing recipe sees a change.
    #[default]
    Legacy,
    /// Explicit left-to-right wave: column i starts at time `i * cascade`.
    Cascade,
    /// Physical-Solari: all columns start simultaneously, landing at
    /// times proportional to their per-column flap distance. Equivalent
    /// to `authentic_timing: true` regardless of the field's value.
    Authentic,
    /// All columns start and land together (cascade forced to 0, no
    /// distance-proportional settle). Good for a snappy full-board swap.
    Simultaneous,
    /// Per-column FNV-hashed start delay — "popcorn" pattern. Each flap
    /// begins at a pseudo-random moment within the dispersion window;
    /// deterministic per column index.
    Random,
    /// Ripple from the midpoint outward: columns nearest the center
    /// start first, edges start last.
    CenterOut,
    /// Ripple from both edges inward: leftmost and rightmost columns
    /// start first, middle columns start last.
    EdgeIn,
    /// Cascade in hash-scrambled column order — every column gets a
    /// unique rank in [0, N), but the rank order is pseudo-random.
    /// Reads as a wave without a visible direction.
    Shuffled,
}

/// Split-flap (Solari) text transformer.
#[derive(Debug, Clone)]
pub struct SplitFlap {
    /// Flip animation speed. Bindable: literal, runtime binding, or signal.
    pub speed: VfxBindableValue,
    /// Cascade delay between characters (Legacy dispersion only). Bindable.
    pub cascade: VfxBindableValue,
    /// Minimum full character-pool cycles each char walks before landing.
    /// Bindable.
    pub cycles: VfxBindableValue,
    pub jitter: f32,
    pub charset: SplitFlapCharset,
    pub settle_overshoot: bool,
    pub leading_blocks: f32,
    pub settle_hinge: bool,
    pub spring_settle: bool,
    pub authentic_timing: bool,
    pub from_message: Option<String>,
    /// When true, replace the intermediate-letter walk with a continuous
    /// rolling-card animation — each walk step plays through the 6-frame
    /// HINGE rotation sequence (█→▀→▔→—→▁→▄) so the viewer sees cards
    /// physically tumbling over each other instead of watching the
    /// alphabet flicker past. The settled target letter is revealed only
    /// at progress=1.0 via the transformer's own early return.
    ///
    /// This captures the dominant visual character of a real Solari
    /// board — the letters in rapid rotation are too fast to read
    /// individually, so what the eye perceives is the rotation itself,
    /// not text. Pairs best with low `cycles` values (0.2–0.5) and
    /// longer animation durations so each rotation step has enough
    /// frames to read as a full flip rather than a blur.
    ///
    /// Default: false (traditional walk showing intermediate letters).
    pub rolling_flip: bool,
    /// Substitute the `▔` frame (index 2 of the HINGE rotation) with the
    /// Unicode 180°-turned target glyph, so viewers catch a brief
    /// upside-down preview of the arriving letter mid-flip. Models the
    /// back-of-the-flap glimpse in Vestaboard-style displays. Falls back
    /// to the original `▔` when the target has no honest turned form
    /// (Q, J, digits 1/2/4/5/7, non-ASCII). Requires `settle_hinge: true`.
    ///
    /// Has no effect when `flip_flicker` is true — flicker owns the
    /// entire hinge window.
    pub flip_preview: bool,
    /// Replace the ordered HINGE sequence with a per-(column, time-bucket)
    /// FNV-hashed pool draw across `{█, ▀, ▔, —, ▁, ▄, target, turn(target)}`.
    /// Reads as chaotic mechanical flicker — closer to the actual
    /// viewer-perceived look of a real Solari flap rotating in ~35ms
    /// than any ordered rotation. Per-column phase is automatic via the
    /// hash, so the whole board doesn't pulse in lockstep. Requires
    /// `settle_hinge: true` (the flicker lives inside the hinge window).
    ///
    /// Accessibility note: this is perceptible flicker. Opt-in.
    pub flip_flicker: bool,
    /// Dispersion pattern controlling per-column start delays — the
    /// visual "shape" of which flaps begin rotating first. Default
    /// `Legacy` preserves existing `cascade`/`authentic_timing` behavior.
    pub dispersion: SplitFlapDispersion,
    pub tile_width: u16,
    pub tile_height: u16,
}

impl SplitFlap {
    /// Target per-flap rotation time in milliseconds on a real Solari board.
    /// Physical flaps rotate in ~30-50ms under gravity + spring assist.
    /// Alitalia Milan Linate ~40ms, Frankfurt Hauptbahnhof ~35ms,
    /// Philadelphia 30th Street ~45-50ms.
    pub const AUTHENTIC_FLAP_MS: f32 = 35.0;
    pub const ALPHA_POOL_SIZE: f32 = 42.0;

    /// 2.1.0-compat constructor — speed + cascade only.
    pub fn new(speed: VfxBindableValue, cascade: VfxBindableValue) -> Self {
        Self {
            speed,
            cascade,
            cycles: VfxBindableValue::default(),
            jitter: 0.0,
            charset: SplitFlapCharset::Alpha,
            settle_overshoot: false,
            leading_blocks: 0.0,
            settle_hinge: false,
            spring_settle: false,
            authentic_timing: false,
            from_message: None,
            rolling_flip: false,
            flip_preview: false,
            flip_flicker: false,
            dispersion: SplitFlapDispersion::Legacy,
            tile_width: 1,
            tile_height: 1,
        }
    }

    /// Full 3.0.0 constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new_mechanical(
        speed: VfxBindableValue,
        cascade: VfxBindableValue,
        cycles: VfxBindableValue,
        jitter: f32,
        charset: SplitFlapCharset,
        settle_overshoot: bool,
        leading_blocks: f32,
        settle_hinge: bool,
        spring_settle: bool,
        authentic_timing: bool,
    ) -> Self {
        Self {
            speed,
            cascade,
            cycles,
            jitter: jitter.clamp(0.0, 1.0),
            charset,
            settle_overshoot,
            leading_blocks: leading_blocks.clamp(0.0, 1.0),
            settle_hinge,
            spring_settle,
            authentic_timing,
            from_message: None,
            rolling_flip: false,
            flip_preview: false,
            flip_flicker: false,
            dispersion: SplitFlapDispersion::Legacy,
            tile_width: 1,
            tile_height: 1,
        }
    }

    /// Enable `rolling_flip` mode on this transformer.
    pub fn with_rolling_flip(mut self, rolling: bool) -> Self {
        self.rolling_flip = rolling;
        self
    }

    /// Enable `flip_preview`: substitute the `▔` hinge frame with
    /// `char_turn(target)` so viewers catch an upside-down glimpse of
    /// the target letter mid-rotation.
    pub fn with_flip_preview(mut self, enabled: bool) -> Self {
        self.flip_preview = enabled;
        self
    }

    /// Enable `flip_flicker`: replace the ordered hinge sequence with
    /// per-(column, bucket)-hashed variant draws — chaotic mechanical
    /// flicker instead of a clean rotation.
    pub fn with_flip_flicker(mut self, enabled: bool) -> Self {
        self.flip_flicker = enabled;
        self
    }

    /// Set the dispersion pattern that controls per-column start delays.
    /// `Legacy` (default) preserves existing `cascade`/`authentic_timing`
    /// behavior; other variants override them.
    pub fn with_dispersion(mut self, dispersion: SplitFlapDispersion) -> Self {
        self.dispersion = dispersion;
        self
    }

    /// Message-to-message transition: each column rotates from the
    /// character at `from[i]` to the character at `target[i]`.
    pub fn with_from_message(mut self, from: impl Into<String>) -> Self {
        self.from_message = Some(from.into());
        self
    }

    /// Set SplitFlap tile geometry. `1x1` preserves legacy character behavior.
    pub fn with_tile_size(mut self, width: u16, height: u16) -> Self {
        self.tile_width = width;
        self.tile_height = height;
        self
    }

    /// Build a Solari-authentic SplitFlap scaled to `animation_ms`.
    /// Targets ~35ms per flap and enables hinge, spring, and authentic
    /// timing. See AUTHENTIC_FLAP_MS doc for real-board references.
    pub fn solari_preset(animation_ms: f32) -> Self {
        let animation_ms = animation_ms.max(100.0);
        let available_flaps = animation_ms / Self::AUTHENTIC_FLAP_MS;
        let cycles = (available_flaps / Self::ALPHA_POOL_SIZE).clamp(0.5, 3.0);
        Self::new_mechanical(
            VfxBindableValue::Literal(1.0),
            VfxBindableValue::Literal(0.0), // cascade=0: real boards dispatch all columns simultaneously
            VfxBindableValue::Literal(cycles),
            0.15, // jitter: mechanical imperfection
            SplitFlapCharset::Alpha,
            false, // settle_overshoot
            0.05,  // leading_blocks
            true,  // settle_hinge
            true,  // spring_settle
            true,  // authentic_timing
        )
    }

    /// Estimated per-flap wall time for diagnostics. Aim for 30-50ms.
    /// Returns `None` when cycles is bound to a runtime value or driven by a
    /// signal — without evaluation context the static estimate is undefined.
    pub fn estimated_flap_ms(&self, animation_ms: f32) -> Option<f32> {
        let VfxBindable::Literal(cycles) = &self.cycles else {
            return None;
        };
        let pool = self.charset.chars().len() as f32;
        let flaps_per_char = pool * cycles.max(0.0) + pool * 0.5;
        if flaps_per_char < 1.0 {
            return Some(animation_ms);
        }
        Some(animation_ms / flaps_per_char)
    }

    /// Stable FNV-1a hashed jitter factor in [1-jitter, 1+jitter].
    fn jitter_factor(&self, char_index: usize) -> f64 {
        if self.jitter <= 0.0 {
            return 1.0;
        }
        let normalized = (Self::fnv_hash(char_index as u32) as f64 / u32::MAX as f64) * 2.0 - 1.0;
        1.0 + (normalized * self.jitter as f64)
    }

    /// FNV-1a 32-bit hash of a single u32 — deterministic, no dependencies.
    /// Used for jitter, dispersion, and flicker-variant selection.
    fn fnv_hash(input: u32) -> u32 {
        let mut hash: u32 = 0x811c9dc5;
        for b in input.to_le_bytes().iter() {
            hash ^= u32::from(*b);
            hash = hash.wrapping_mul(0x01000193);
        }
        hash
    }

    /// Combined hash of two inputs — used to seed per-(col, bucket)
    /// flicker-variant picks so each column has its own flicker phase.
    fn fnv_hash2(a: u32, b: u32) -> u32 {
        Self::fnv_hash(a.wrapping_mul(0x01000193).wrapping_add(b))
    }

    /// Per-column start-delay normalized to `[0, 1]` — the fraction of
    /// total animation time before this column begins rotating. The
    /// caller multiplies the result by the recipe's `cascade` value, so
    /// `cascade` is interpretable as "max delay as fraction of total
    /// animation time" regardless of message length.
    ///
    /// `col_in_row` resets per row in the transform loop, so multi-line
    /// messages cascade independently per row instead of accumulating
    /// delay across rows (the pre-3.2.1 bug that made cascade > 0
    /// unusable on any board taller than one line).
    ///
    /// Legacy and Authentic return 0 from here; their timing is handled
    /// in the main transform() branch selection.
    fn column_start_delay(&self, col_in_row: usize, max_row_width: usize) -> f64 {
        let n = max_row_width.max(1) as f64;
        let last = (n - 1.0).max(1.0);
        match self.dispersion {
            SplitFlapDispersion::Legacy | SplitFlapDispersion::Authentic => 0.0,
            SplitFlapDispersion::Simultaneous => 0.0,
            SplitFlapDispersion::Cascade => (col_in_row as f64) / last,
            SplitFlapDispersion::Random => {
                Self::fnv_hash(col_in_row as u32) as f64 / u32::MAX as f64
            }
            SplitFlapDispersion::CenterOut => {
                let center = (n - 1.0) / 2.0;
                let max_dist = center.max(1.0);
                (col_in_row as f64 - center).abs() / max_dist
            }
            SplitFlapDispersion::EdgeIn => {
                let center = (n - 1.0) / 2.0;
                let max_dist = center.max(1.0);
                (max_dist - (col_in_row as f64 - center).abs()) / max_dist
            }
            SplitFlapDispersion::Shuffled => {
                let rank = (Self::fnv_hash(col_in_row as u32) as usize) % max_row_width.max(1);
                rank as f64 / last
            }
        }
    }

    /// True when dispersion requests physical-Solari distance-proportional
    /// landing (either via explicit `Authentic` variant or via legacy
    /// `authentic_timing: true`).
    fn uses_authentic_timing(&self) -> bool {
        matches!(self.dispersion, SplitFlapDispersion::Authentic)
            || (matches!(self.dispersion, SplitFlapDispersion::Legacy) && self.authentic_timing)
    }

    fn tile_cycle_progress(local_progress: f64, cycles: f64) -> f64 {
        let local = local_progress.clamp(0.0, 1.0);
        if local >= 1.0 {
            return 1.0;
        }
        let scaled = local * (cycles.max(0.0) + 1.0);
        if scaled >= cycles.max(0.0) {
            (scaled - cycles.max(0.0)).clamp(0.0, 1.0)
        } else {
            scaled.fract()
        }
    }

    fn hinge_spring() -> DampedSpring {
        DampedSpring::new(1.0, 180.0, 14.0, 0.0, 1.0)
    }

    fn spring_settle_phase(linear_phase: f64) -> f64 {
        let t = (linear_phase * 0.3).clamp(0.0, 1.0);
        let spring = Self::hinge_spring();
        let pos = spring.position_at(t) as f64;
        (1.0 - pos).clamp(0.0, 1.0)
    }
}

impl TextTransformer for SplitFlap {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        // Evaluate speed / cascade / cycles per-frame; resolves literal /
        // runtime binding / signal expression. ctx.runtime_params lets hosts
        // drive {"binding": "..."} fields from app state.
        let speed = self
            .speed
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
            .unwrap_or(0.0)
            .max(0.0);
        let cascade = self
            .cascade
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
            .unwrap_or(0.0)
            .max(0.0);
        let cycles = self
            .cycles
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
            .unwrap_or(0.0)
            .max(0.0) as f64;

        if self.tile_width != 1 || self.tile_height != 1 {
            let tile = MechanicalTile {
                width: self.tile_width,
                height: self.tile_height,
            };
            if validate_split_flap_tile(tile).is_err() {
                return Cow::Borrowed(target);
            }
            let source = paired_grids(
                self.from_message.as_deref(),
                target,
                MechanicalSizing::PadToMax,
            );
            let tile_cols = source
                .to
                .width()
                .max(source.from.width())
                .div_ceil(tile.width as usize)
                .max(1);
            let use_authentic = self.uses_authentic_timing();
            let grid = split_flap_tile_frame(&source, tile, |tile_col, tile_row| {
                let tile_index = tile_row.saturating_mul(tile_cols).saturating_add(tile_col);
                let jitter_factor = self.jitter_factor(tile_index);
                let local = if use_authentic {
                    progress * f64::from(speed) * jitter_factor
                } else {
                    let effective_cascade = f64::from(cascade) * jitter_factor;
                    let delay_units = if matches!(self.dispersion, SplitFlapDispersion::Legacy) {
                        tile_index as f64
                    } else {
                        self.column_start_delay(tile_col, tile_cols)
                    };
                    progress * f64::from(speed) - delay_units * effective_cascade
                };
                Self::tile_cycle_progress(local, cycles)
            });
            return Cow::Owned(grid_to_text(&grid));
        }

        let pool = self.charset.chars();
        let pool_len = pool.len().max(1);

        // Forward-only drum rotation distance — physical Solari drums only
        // rotate one direction, so Z→A goes forward through space, not
        // backward one step.
        let flap_distance = |from_idx: usize, to_idx: usize| -> f64 {
            let raw = (to_idx + pool_len - from_idx) % pool_len;
            raw as f64 + pool_len as f64 * cycles
        };

        // Resolve per-column "from" index from self.from_message (or fall
        // back to space).
        let from_chars: Vec<char> = self
            .from_message
            .as_deref()
            .map(|s| s.chars().collect())
            .unwrap_or_default();
        let from_idx_for = |col: usize| -> usize {
            from_chars
                .get(col)
                .copied()
                .and_then(|c| pool.iter().position(|&p| p == c.to_ascii_uppercase()))
                .unwrap_or(0)
        };

        // Pre-compute the longest row's width — non-Legacy dispersion
        // variants normalize their delay curves against this so cascade
        // semantics are "max delay as fraction of total animation time"
        // regardless of message length. Without this, multi-line boards
        // would never settle on lower rows.
        let max_row_width = target
            .lines()
            .map(|line| line.chars().count())
            .max()
            .unwrap_or(1)
            .max(1);

        // authentic_timing: pre-scan the message to find the maximum
        // per-column flap count. Every column rotates at a constant
        // per-flap rate starting from progress=0; the longest rotation
        // lands at progress=1.0, shorter rotations land earlier.
        let use_authentic = self.uses_authentic_timing();
        let max_flaps = if use_authentic {
            target
                .chars()
                .enumerate()
                .filter_map(|(col, c)| {
                    pool.iter()
                        .position(|&p| p == c.to_ascii_uppercase())
                        .map(|to_idx| flap_distance(from_idx_for(col), to_idx))
                })
                .fold(1.0f64, f64::max)
        } else {
            0.0
        };

        let mut out = String::with_capacity(target.len());
        // Per-row column position — resets after every newline so each
        // row of a multi-line board independently cascades from col 0.
        // Without this reset, dispersion delay would accumulate across
        // rows and lower rows would never settle for any cascade > 0.
        let mut col_in_row: usize = 0;
        for (i, target_char) in target.chars().enumerate() {
            // Structural characters (newlines, tabs, carriage returns)
            // pass through unchanged at every frame. This is load-bearing
            // for multi-line messages like arrivals boards — the flap
            // mechanism doesn't exist between rows, so the row separator
            // must remain a row separator throughout the animation
            // rather than flipping through the pool like content.
            if target_char == '\n' || target_char == '\r' || target_char == '\t' {
                out.push(target_char);
                if target_char == '\n' || target_char == '\r' {
                    col_in_row = 0;
                }
                col_in_row += 1;
                continue;
            }

            let jitter_factor = self.jitter_factor(i);

            // Resolve target_idx. If not in pool, emit the target char
            // directly when progress > 0.9, else emit the pool's space.
            let target_idx = match pool
                .iter()
                .position(|&p| p == target_char.to_ascii_uppercase())
            {
                Some(idx) => idx,
                None => {
                    if progress > 0.9 {
                        out.push(target_char);
                    } else {
                        out.push(pool[0]);
                    }
                    col_in_row += 1;
                    continue;
                }
            };

            let col_from_idx = from_idx_for(i);
            let this_flap_distance = flap_distance(col_from_idx, target_idx);

            // Unchanged column (from_message[i] == target[i] AND cycles=0)
            // renders the target instantly — matches real boards where
            // unchanged flaps never rotate.
            if this_flap_distance < 0.0001 {
                out.push(target_char);
                col_in_row += 1;
                continue;
            }

            // Compute per-column char_progress.
            let char_progress = if use_authentic {
                // Physical Solari: all columns start simultaneously, each
                // rotates at the same per-flap rate, so landing time is
                // proportional to flap distance. Jitter adds mechanical
                // imperfection across columns.
                let completion_ratio = (this_flap_distance / max_flaps).max(0.001);
                let base = progress * f64::from(speed) / completion_ratio;
                (base * jitter_factor).clamp(0.0, 1.0)
            } else {
                // Cascade model — each column starts at a delay determined
                // by the dispersion pattern, then walks to completion at
                // progress=1.0. Legacy dispersion uses the linear char
                // index (preserves pre-3.2.0 behavior, including its
                // multi-line bug where lower rows never settle for
                // cascade > 0). Non-Legacy variants use col_in_row +
                // max_row_width to produce a normalized [0, 1] delay
                // that resets per row, so cascade is interpretable as
                // "max delay as fraction of total animation time."
                let effective_cascade = f64::from(cascade) * jitter_factor;
                let delay_units = if matches!(self.dispersion, SplitFlapDispersion::Legacy) {
                    i as f64
                } else {
                    self.column_start_delay(col_in_row, max_row_width)
                };
                (progress * f64::from(speed) - (delay_units * effective_cascade)).clamp(0.0, 1.0)
            };

            if char_progress >= 1.0 {
                out.push(target_char);
                col_in_row += 1;
                continue;
            }

            // Opening block-flash phase.
            let leading = self.leading_blocks.clamp(0.0, 0.95) as f64;
            if leading > 0.0 && char_progress < leading {
                let sub = char_progress / leading;
                let cycle_rate = 6.0;
                let block_idx =
                    (sub * cycle_rate * BLOCK_CHARS.len() as f64) as usize % BLOCK_CHARS.len();
                out.push(BLOCK_CHARS[block_idx]);
                col_in_row += 1;
                continue;
            }

            // Closing hinge rotation.
            const HINGE_WINDOW: f64 = 0.18;
            if self.settle_hinge && char_progress > (1.0 - HINGE_WINDOW) {
                let linear_phase = (char_progress - (1.0 - HINGE_WINDOW)) / HINGE_WINDOW;
                let settle_phase = if self.spring_settle {
                    Self::spring_settle_phase(linear_phase)
                } else {
                    linear_phase
                };

                // flip_flicker: per-(col, bucket)-hashed variant draw —
                // each bucket lasts ~1/FLICKER_BUCKETS of the hinge window
                // (~34ms at 60fps over a 270ms hinge), producing ~30Hz
                // flicker. The pool mixes block, hinge, and letter
                // variants so the viewer reads chaotic mechanical flipping.
                // Per-column hash seed means columns flicker out of phase.
                if self.flip_flicker {
                    const FLICKER_BUCKETS: u32 = 8;
                    let bucket =
                        ((linear_phase * FLICKER_BUCKETS as f64) as u32).min(FLICKER_BUCKETS - 1);
                    let turned = char_turn(target_char);
                    // Variant pool — 8 slots when char_turn is available
                    // (turned duplicated for higher weight), else 6 slots.
                    let pool: [Option<char>; 8] = [
                        Some('█'),
                        Some('▀'),
                        Some('▔'),
                        Some('—'),
                        Some('▁'),
                        Some('▄'),
                        turned.or(Some(target_char)),
                        turned.or(Some('█')),
                    ];
                    let h = Self::fnv_hash2(i as u32, bucket) as usize;
                    out.push(pool[h % pool.len()].unwrap_or(target_char));
                    col_in_row += 1;
                    continue;
                }

                let glyph_idx = (settle_phase * HINGE_CHARS.len() as f64)
                    .min(HINGE_CHARS.len() as f64 - 1.0) as usize;

                // flip_preview: substitute the `▔` frame (index 2) with
                // the 180°-turned target glyph so viewers catch an
                // upside-down preview mid-rotation. Falls back to the
                // original frame for unmapped targets (Q, J, digits
                // 1/2/4/5/7, non-ASCII).
                let glyph = if self.flip_preview && glyph_idx == 2 {
                    char_turn(target_char).unwrap_or(HINGE_CHARS[glyph_idx])
                } else {
                    HINGE_CHARS[glyph_idx]
                };
                out.push(glyph);
                col_in_row += 1;
                continue;
            }

            // Walk phase — rescale char_progress into the walk window.
            let walk_end = if self.settle_hinge {
                1.0 - HINGE_WINDOW
            } else {
                1.0
            };
            let walk_span = (walk_end - leading).max(0.01);
            let walk_progress = ((char_progress - leading) / walk_span).clamp(0.0, 1.0);
            let walk_pos = this_flap_distance * walk_progress;

            // rolling_flip: instead of showing the intermediate letter at
            // each walk step, play the 6-frame HINGE rotation continuously
            // across every step. Each step = one full hinge rotation. The
            // final target is revealed only at char_progress >= 1.0 via
            // the transformer's early return; the last hinge step ending
            // in ▄ visually hands off to the settled letter cleanly.
            if self.rolling_flip {
                // Sub-progress within the current step (0.0..1.0 per step).
                let step_frac = walk_pos.fract();
                let glyph_idx = (step_frac * HINGE_CHARS.len() as f64)
                    .min(HINGE_CHARS.len() as f64 - 1.0) as usize;
                out.push(HINGE_CHARS[glyph_idx]);
                col_in_row += 1;
                continue;
            }

            // settle_overshoot (only when settle_hinge is off — hinge owns
            // the settle window).
            let current_idx = if !self.settle_hinge && self.settle_overshoot && char_progress > 0.9
            {
                let settle_phase = (char_progress - 0.9) / 0.1;
                if settle_phase < 0.5 {
                    (target_idx + 1) % pool_len
                } else {
                    target_idx
                }
            } else {
                (col_from_idx + walk_pos as usize) % pool_len
            };

            out.push(pool.get(current_idx).copied().unwrap_or(target_char));
            col_in_row += 1;
        }
        Cow::Owned(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mixed_signals::prelude::SignalContext;
    use std::sync::OnceLock;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    static CTX_PARTS: OnceLock<(SignalContext, ShaderRuntimeParams)> = OnceLock::new();

    fn ctx_parts() -> &'static (SignalContext, ShaderRuntimeParams) {
        CTX_PARTS.get_or_init(|| (SignalContext::new(0, 0), ShaderRuntimeParams::new()))
    }

    fn tctx() -> TransformContext<'static> {
        let p = ctx_parts();
        TransformContext::new(&p.0, &p.1)
    }

    /// Test helper: common 6-arg form (pre-spring/pre-authentic/pre-from).
    fn sf(
        cycles: f64,
        jitter: f32,
        charset: SplitFlapCharset,
        settle_overshoot: bool,
        leading_blocks: f32,
        settle_hinge: bool,
    ) -> SplitFlap {
        SplitFlap::new_mechanical(
            VfxBindableValue::Literal(1.0),
            VfxBindableValue::Literal(0.0_f32),
            VfxBindableValue::Literal(cycles as f32),
            jitter,
            charset,
            settle_overshoot,
            leading_blocks,
            settle_hinge,
            false,
            false,
        )
    }

    /// Test helper: full form including 3.0.0 authenticity knobs.
    #[allow(clippy::too_many_arguments)]
    fn sf_full(
        cycles: f64,
        jitter: f32,
        charset: SplitFlapCharset,
        settle_overshoot: bool,
        leading_blocks: f32,
        settle_hinge: bool,
        spring_settle: bool,
        authentic_timing: bool,
    ) -> SplitFlap {
        SplitFlap::new_mechanical(
            VfxBindableValue::Literal(1.0),
            VfxBindableValue::Literal(0.0_f32),
            VfxBindableValue::Literal(cycles as f32),
            jitter,
            charset,
            settle_overshoot,
            leading_blocks,
            settle_hinge,
            spring_settle,
            authentic_timing,
        )
    }

    // ---------- Backward compat ----------

    #[test]
    fn bare_constructor_preserves_defaults() {
        let x = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.2));
        assert_eq!(x.jitter, 0.0);
        assert_eq!(x.charset, SplitFlapCharset::Alpha);
        assert!(!x.settle_overshoot);
        assert!(!x.settle_hinge);
        assert!(!x.spring_settle);
        assert!(!x.authentic_timing);
        assert!(x.from_message.is_none());
    }

    #[test]
    fn complete_progress_returns_target_unchanged() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert_eq!(x.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    #[test]
    fn default_behavior_matches_v2_linear_walk() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("Z", 0.5, &tctx()).chars().next().unwrap();
        assert_ne!(c, 'Z');
    }

    // ---------- cycles ----------

    #[test]
    fn cycles_make_low_index_targets_flip_through_pool() {
        let x = sf(2.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("A", 0.25, &tctx()).chars().next().unwrap();
        assert_ne!(c, 'A');
    }

    #[test]
    fn cycles_still_land_at_progress_1() {
        let x = sf(3.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert_eq!(x.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    // ---------- jitter ----------

    #[test]
    fn jitter_produces_stable_hash() {
        let x = sf(0.0, 0.5, SplitFlapCharset::Alpha, false, 0.0, false);
        assert_eq!(x.jitter_factor(5), x.jitter_factor(5));
        assert!((0..10).any(|i| (x.jitter_factor(i) - 1.0).abs() > 0.01));
    }

    #[test]
    fn jitter_zero_is_unit() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        for i in 0..10 {
            assert_eq!(x.jitter_factor(i), 1.0);
        }
    }

    #[test]
    fn jitter_clamps_to_unit_range() {
        let x = sf(0.0, 5.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert_eq!(x.jitter, 1.0);
    }

    // ---------- charset ----------

    #[test]
    fn charset_digits_cycles_digits_only() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Digits, false, 0.0, false);
        let c = x.transform("7", 0.5, &tctx()).chars().next().unwrap();
        assert!(c.is_ascii_digit() || c == ' ');
    }

    #[test]
    fn charset_digits_lands_on_digits() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Digits, false, 0.0, false);
        assert_eq!(x.transform("2024", 1.0, &tctx()), "2024");
    }

    #[test]
    fn charset_uppercase_cycles_letters_only() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Uppercase, false, 0.0, false);
        let r = x.transform("LISBON", 0.5, &tctx());
        for c in r.chars() {
            assert!(c.is_ascii_uppercase() || c == ' ');
        }
    }

    // ---------- settle_overshoot ----------

    #[test]
    fn settle_overshoot_lands_on_target_at_progress_1() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, true, 0.0, false);
        assert_eq!(x.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    // ---------- leading_blocks ----------

    #[test]
    fn leading_blocks_shows_block_glyph_at_opening() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.3, false);
        let c = x.transform("A", 0.1, &tctx()).chars().next().unwrap();
        assert!(BLOCK_CHARS.contains(&c));
    }

    #[test]
    fn leading_blocks_zero_preserves_v2_behavior() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("A", 0.05, &tctx()).chars().next().unwrap();
        assert!(!BLOCK_CHARS.contains(&c));
    }

    // ---------- settle_hinge ----------

    #[test]
    fn settle_hinge_plays_rotation_in_settle_window() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        let c = x.transform("A", 0.9, &tctx()).chars().next().unwrap();
        assert!(HINGE_CHARS.contains(&c));
    }

    #[test]
    fn settle_hinge_progresses_through_rotation() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        let early = x.transform("A", 0.83, &tctx()).chars().next().unwrap();
        let late = x.transform("A", 0.99, &tctx()).chars().next().unwrap();
        assert_ne!(early, late);
    }

    #[test]
    fn settle_hinge_lands_on_target_at_progress_1() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        assert_eq!(x.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    // ---------- spring_settle ----------

    #[test]
    fn spring_settle_retimes_hinge_frames() {
        let linear = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        );
        let spring = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            true,
            false,
        );
        let samples = [0.835, 0.86, 0.9, 0.94, 0.97];
        let any_diff = samples.iter().any(|&t| {
            linear.transform("A", t, &tctx()).chars().next()
                != spring.transform("A", t, &tctx()).chars().next()
        });
        assert!(any_diff);
    }

    #[test]
    fn spring_settle_still_lands_on_target() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            true,
            false,
        );
        assert_eq!(x.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    // ---------- authentic_timing ----------

    #[test]
    fn authentic_timing_short_distance_lands_early() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        );
        let r = x.transform("AZ", 0.1, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars[0], 'A', "short-distance char must land early");
        assert_ne!(chars[1], 'Z', "long-distance char must still be flipping");
    }

    #[test]
    fn authentic_timing_all_land_at_progress_1() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        );
        assert_eq!(x.transform("FLIGHT 721", 1.0, &tctx()), "FLIGHT 721");
    }

    #[test]
    fn authentic_timing_identical_chars_land_together() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        );
        let r = x.transform("AA", 0.5, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars[0], chars[1]);
    }

    // ---------- solari_preset ----------

    #[test]
    fn solari_preset_enables_authenticity_flags() {
        let p = SplitFlap::solari_preset(1500.0);
        assert!(p.settle_hinge);
        assert!(p.spring_settle);
        assert!(p.authentic_timing);
    }

    #[test]
    fn solari_preset_cycles_near_one_for_1500ms() {
        let p = SplitFlap::solari_preset(1500.0);
        let VfxBindable::Literal(cycles) = p.cycles else {
            panic!("solari_preset must produce a Literal cycles, got {:?}", p.cycles);
        };
        assert!((cycles - 1.02).abs() < 0.2, "got cycles={cycles}");
    }

    #[test]
    fn estimated_flap_ms_is_close_to_authentic() {
        let p = SplitFlap::solari_preset(1500.0);
        let per_flap = p.estimated_flap_ms(1500.0).unwrap();
        assert!((20.0..60.0).contains(&per_flap), "got {per_flap}");
    }

    // ---------- from_message ----------

    #[test]
    fn from_message_unchanged_columns_land_immediately() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        )
        .with_from_message("LL");
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let r = x.transform("LX", t, &tctx());
            assert_eq!(r.chars().next().unwrap(), 'L', "at t={t}");
        }
    }

    #[test]
    fn from_message_forward_only_drum_rotation() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        )
        .with_from_message("Z");
        assert_eq!(x.transform("A", 1.0, &tctx()), "A");
        let mid = x.transform("A", 0.5, &tctx()).chars().next().unwrap();
        assert_ne!(mid, 'A');
        assert_ne!(mid, 'Z');
    }

    #[test]
    fn from_message_lands_all_chars_at_progress_1() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        )
        .with_from_message("LONDON");
        assert_eq!(x.transform("PARIS ", 1.0, &tctx()), "PARIS ");
    }

    #[test]
    fn from_message_shorter_than_target_pads_with_space() {
        let x = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        )
        .with_from_message("AB");
        assert_eq!(x.transform("ABCDE", 1.0, &tctx()), "ABCDE");
        let r = x.transform("ABCDE", 0.05, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars[0], 'A');
        assert_eq!(chars[1], 'B');
    }

    // ---------- rolling_flip: continuous card rotation ----------

    #[test]
    fn rolling_flip_shows_hinge_glyphs_during_walk() {
        // With rolling_flip enabled, every position during the walk
        // phase must be a HINGE rotation glyph, never a pool letter.
        let shader = sf_full(
            1.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            false,
        )
        .with_rolling_flip(true);
        for t in [0.1, 0.3, 0.5, 0.7] {
            let c = shader.transform("Z", t, &tctx()).chars().next().unwrap();
            assert!(
                HINGE_CHARS.contains(&c),
                "rolling_flip at t={t} must show a rotation glyph, got '{c}'"
            );
        }
    }

    #[test]
    fn rolling_flip_lands_on_target_at_progress_1() {
        let shader = sf_full(
            1.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            false,
        )
        .with_rolling_flip(true);
        assert_eq!(shader.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    #[test]
    fn rolling_flip_default_off_preserves_letter_walk() {
        let shader = sf(0.5, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert!(!shader.rolling_flip);
        // Without rolling_flip, mid-walk should show a pool letter
        // (not a hinge glyph), confirming the old walk behavior remains
        // the default.
        let c = shader.transform("Z", 0.3, &tctx()).chars().next().unwrap();
        assert!(
            !HINGE_CHARS.contains(&c),
            "without rolling_flip, mid-walk must show a pool letter, got '{c}'"
        );
    }

    // ---------- multi-line: newline passthrough ----------

    #[test]
    fn newlines_pass_through_unchanged_during_flip() {
        // A multi-line message (arrivals board style) must preserve its
        // newlines at every progress value so the layout doesn't collapse
        // into a single long row mid-animation.
        let shader = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        );
        for t in [0.0, 0.25, 0.5, 0.75, 0.99, 1.0] {
            let r = shader.transform("AB\nCD", t, &tctx());
            assert!(
                r.contains('\n'),
                "multi-line message must preserve newline at t={t}, got {r:?}"
            );
            // Also: the newline should be at position 2 (not shifted).
            assert_eq!(
                r.chars().position(|c| c == '\n'),
                Some(2),
                "newline must stay at its column at t={t}, got {r:?}"
            );
        }
    }

    #[test]
    fn multiline_authentic_timing_lands_all_rows() {
        let shader = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        );
        let target = "ROW 1\nROW 2\nROW 3";
        assert_eq!(shader.transform(target, 1.0, &tctx()), target);
    }

    #[test]
    fn multiline_from_message_preserves_structure() {
        let shader = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            false,
            false,
            true,
        )
        .with_from_message("A1\nB2");
        for t in [0.0, 0.3, 0.6, 1.0] {
            let r = shader.transform("X9\nY8", t, &tctx());
            assert_eq!(
                r.chars().filter(|&c| c == '\n').count(),
                1,
                "multi-line from_message must preserve newline count at t={t}"
            );
        }
    }

    // ---------- combined ----------

    #[test]
    fn full_solari_arc_produces_correct_glyph_family_per_phase() {
        let x = sf_full(
            1.0,
            0.1,
            SplitFlapCharset::Uppercase,
            false,
            0.15,
            true,
            false,
            false,
        );
        let opening = x.transform("S", 0.05, &tctx()).chars().next().unwrap();
        assert!(BLOCK_CHARS.contains(&opening));
        let middle = x.transform("S", 0.5, &tctx()).chars().next().unwrap();
        assert!(!BLOCK_CHARS.contains(&middle) && !HINGE_CHARS.contains(&middle));
        let ending = x.transform("S", 0.9, &tctx()).chars().next().unwrap();
        assert!(HINGE_CHARS.contains(&ending));
        assert_eq!(x.transform("S", 1.0, &tctx()), "S");
    }

    // ---------- flip_preview: inverted-glyph frame inside hinge window ----------

    #[test]
    fn flip_preview_substitutes_turned_target_in_hinge_window() {
        // Target `A` has turned form `Ɐ`. With flip_preview on and
        // settle_hinge active, there must be at least one progress value
        // in the hinge window where the emitted char is `Ɐ`.
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_preview(true);
        let mut saw_turned = false;
        // Hinge window is 0.82..1.0; sweep at 30Hz density.
        for i in 0..=100 {
            let t = 0.82 + (i as f64) * 0.0018;
            if sf.transform("A", t, &tctx()).starts_with('Ɐ') {
                saw_turned = true;
                break;
            }
        }
        assert!(
            saw_turned,
            "flip_preview must show turned target 'Ɐ' somewhere in the hinge window"
        );
    }

    #[test]
    fn flip_preview_falls_back_for_unmapped_target() {
        // Target `Q` has no turned form. flip_preview must fall back to
        // the original `▔` frame at that position — no Q or random char.
        // Sweep starts at 0.83 (strictly inside the 0.82..1.0 hinge
        // window, since the code condition is `char_progress > 0.82`).
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_preview(true);
        for i in 0..50 {
            let t = 0.83 + (i as f64) * 0.003;
            if t >= 1.0 {
                break;
            }
            let c = sf.transform("Q", t, &tctx()).chars().next().unwrap();
            assert!(
                HINGE_CHARS.contains(&c),
                "flip_preview with unmapped 'Q' must still emit a hinge glyph at t={t}, got '{c}'"
            );
        }
    }

    #[test]
    fn flip_preview_lands_on_target_at_progress_1() {
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_preview(true);
        assert_eq!(sf.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    #[test]
    fn flip_preview_default_off_preserves_hinge_sequence() {
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        );
        assert!(!sf.flip_preview);
        // Without flip_preview, hinge window never emits a turned glyph.
        for i in 0..=100 {
            let t = 0.82 + (i as f64) * 0.0018;
            let c = sf.transform("A", t, &tctx()).chars().next().unwrap();
            assert_ne!(c, 'Ɐ', "default off must not emit turned glyph at t={t}");
        }
    }

    // ---------- flip_flicker: per-column-hashed variant pool ----------

    #[test]
    fn flip_flicker_emits_variety_within_hinge_window() {
        // Across the hinge window, flicker must emit a mix of glyphs
        // (blocks, hinges, target, turned-target) — not a single stable
        // glyph like the ordered sequence would produce.
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_flicker(true);
        let mut distinct = std::collections::HashSet::new();
        for i in 0..=20 {
            let t = 0.82 + (i as f64) * 0.009;
            let c = sf.transform("A", t, &tctx()).chars().next().unwrap();
            distinct.insert(c);
        }
        assert!(
            distinct.len() >= 3,
            "flicker must produce at least 3 distinct glyphs across the window, got {distinct:?}"
        );
    }

    #[test]
    fn flip_flicker_lands_on_target_at_progress_1() {
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_flicker(true);
        assert_eq!(sf.transform("HELLO", 1.0, &tctx()), "HELLO");
    }

    #[test]
    fn flip_flicker_is_deterministic_per_column() {
        // Same (col, progress) pair must always yield the same glyph —
        // flicker is hash-driven, not time-of-day random.
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_flicker(true);
        let a = sf.transform("BOARDING", 0.87, &tctx());
        let b = sf.transform("BOARDING", 0.87, &tctx());
        assert_eq!(a, b);
    }

    #[test]
    fn flip_flicker_columns_are_out_of_phase() {
        // Different columns at the same progress should (almost always)
        // emit different glyphs — per-column hash means the board
        // doesn't pulse in lockstep. Test empirically across a range.
        let sf = sf_full(
            0.0,
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_flip_flicker(true);
        let mut saw_divergence = false;
        for i in 0..20 {
            let t = 0.82 + (i as f64) * 0.009;
            let rendered = sf.transform("ABCDEFGH", t, &tctx());
            let chars: Vec<char> = rendered.chars().collect();
            if chars.windows(2).any(|w| w[0] != w[1]) {
                saw_divergence = true;
                break;
            }
        }
        assert!(
            saw_divergence,
            "adjacent columns should flicker out of phase somewhere in the window"
        );
    }

    // ---------- SplitFlapDispersion: per-column start-delay patterns ----------

    #[test]
    fn dispersion_legacy_matches_cascade_behavior() {
        // Legacy dispersion + cascade > 0 must produce the same output
        // as pre-3.2.0 code with the same cascade value.
        let legacy = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.05));
        let explicit = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.05))
            .with_dispersion(SplitFlapDispersion::Legacy);
        for t in [0.1, 0.3, 0.5, 0.7, 1.0] {
            assert_eq!(
                legacy.transform("HELLO", t, &tctx()),
                explicit.transform("HELLO", t, &tctx()),
                "Legacy dispersion must match default cascade behavior at t={t}"
            );
        }
    }

    #[test]
    fn dispersion_simultaneous_lands_all_columns_together() {
        // Simultaneous => all columns have delay=0 => all at same
        // char_progress => all settle simultaneously.
        let sf = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.5))
            .with_dispersion(SplitFlapDispersion::Simultaneous);
        assert_eq!(sf.transform("HELLO", 1.0, &tctx()), "HELLO");
        // At t close to 1.0 but not quite, all chars should be "in progress"
        // (not yet settled) at the same rate — their glyphs should be
        // chosen from the same point in the walk.
        let r = sf.transform("AAAA", 0.3, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert!(
            chars.iter().all(|&c| c == chars[0]),
            "simultaneous dispersion must produce identical chars across identical targets, got {r:?}"
        );
    }

    #[test]
    fn dispersion_authentic_ignores_authentic_timing_field() {
        // dispersion: Authentic must enable Solari timing regardless of
        // the authentic_timing field's value.
        let sf = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.0))
            .with_dispersion(SplitFlapDispersion::Authentic);
        // At progress=0.1, a short-distance char should have settled but
        // a long-distance char should still be rotating.
        let r = sf.transform("AZ", 0.1, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(
            chars[0], 'A',
            "short-distance char must land early under Authentic dispersion"
        );
        assert_ne!(chars[1], 'Z', "long-distance char must still be flipping");
    }

    #[test]
    fn dispersion_random_is_deterministic() {
        // Random dispersion uses FNV hash, so same target must produce
        // same output across runs.
        let sf = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.02))
            .with_dispersion(SplitFlapDispersion::Random);
        assert_eq!(
            sf.transform("BOARDING", 0.5, &tctx()),
            sf.transform("BOARDING", 0.5, &tctx())
        );
    }

    #[test]
    fn dispersion_center_out_starts_middle_first() {
        // CenterOut: middle column (idx 2) has delay=0, edges (idx 0, 4)
        // have delay=2. With cascade=0.3 and speed=1:
        //   middle char_progress(t=0.99) = 0.99 → in hinge window
        //   edge char_progress(t=0.99)   = 0.99 - 2*0.3 = 0.39 → still walking
        // So middle emits a HINGE glyph while edges emit walk letters.
        let sf = SplitFlap::new_mechanical(
            VfxBindableValue::Literal(1.0),
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.0),
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_dispersion(SplitFlapDispersion::CenterOut);
        let r = sf.transform("AAAAA", 0.99, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert!(
            HINGE_CHARS.contains(&chars[2]),
            "middle column must be in hinge window, got {:?}",
            chars[2]
        );
        assert!(
            !HINGE_CHARS.contains(&chars[0]),
            "edge column must still be walking, got {:?}",
            chars[0]
        );
    }

    #[test]
    fn dispersion_edge_in_starts_edges_first() {
        // EdgeIn: edges have delay=0, middle has max delay — inverse of
        // CenterOut. At t=0.99: edges in hinge window, middle still walking.
        let sf = SplitFlap::new_mechanical(
            VfxBindableValue::Literal(1.0),
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.0),
            0.0,
            SplitFlapCharset::Alpha,
            false,
            0.0,
            true,
            false,
            false,
        )
        .with_dispersion(SplitFlapDispersion::EdgeIn);
        let r = sf.transform("AAAAA", 0.99, &tctx());
        let chars: Vec<char> = r.chars().collect();
        assert!(
            HINGE_CHARS.contains(&chars[0]),
            "edge column must be in hinge window, got {:?}",
            chars[0]
        );
        assert!(
            !HINGE_CHARS.contains(&chars[2]),
            "middle column must still be walking, got {:?}",
            chars[2]
        );
    }

    #[test]
    fn dispersion_all_variants_land_on_target_at_progress_1() {
        // Invariant across every dispersion variant: at progress=1.0 the
        // message must match the target exactly.
        for disp in [
            SplitFlapDispersion::Legacy,
            SplitFlapDispersion::Cascade,
            SplitFlapDispersion::Authentic,
            SplitFlapDispersion::Simultaneous,
            SplitFlapDispersion::Random,
            SplitFlapDispersion::CenterOut,
            SplitFlapDispersion::EdgeIn,
            SplitFlapDispersion::Shuffled,
        ] {
            let sf = SplitFlap::new(VfxBindableValue::Literal(1.0), VfxBindableValue::Literal(0.05))
                .with_dispersion(disp);
            assert_eq!(
                sf.transform("FLIGHT 721", 1.0, &tctx()),
                "FLIGHT 721",
                "dispersion {disp:?} must land on target at progress=1.0"
            );
        }
    }

    #[test]
    fn dispersion_enum_is_default_legacy() {
        assert_eq!(SplitFlapDispersion::default(), SplitFlapDispersion::Legacy);
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE>
// <VERS>END OF VERSION: 3.6.0</VERS>
