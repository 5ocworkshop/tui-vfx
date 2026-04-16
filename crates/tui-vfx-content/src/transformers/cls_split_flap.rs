// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE> - <DESC>SplitFlap transformer with Solari-board mechanical feel (cycles, jitter, charset, hinge, spring, authentic timing, message transitions)</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>Upgrade SplitFlap from a linear-progression text walk into a physically-accurate mechanical Solari departure-board transformer. Add nine new fields — cycles (every char flips through satisfying pool distance regardless of target index), jitter (per-column mechanical timing imperfection), charset (Alpha/Digits/Uppercase pool selection), settle_overshoot (brief bounce past target), leading_blocks (opening █▓▒░ flash), settle_hinge (closing █→▀→▔→—→▁→▄→letter rotation using upper/lower half blocks plus em-dash hinge), spring_settle (remap hinge timing through DampedSpring for authentic gravity-fall-plus-bounce), authentic_timing (all columns start simultaneously from blank, landing time proportional to flap distance — not sequential cascade), from_message (message-to-message transitions where each column rotates from its previous char to its new char through shortest forward-only drum path). Add solari_preset(animation_ms) factory that computes Solari-authentic values (35ms/flap target, matching real Alitalia Linate and Frankfurt Hbf boards). All new fields default to values that preserve 2.1.0 linear-walk behavior.</WCTX>
// <CLOG>v3.0.0: complete physical Solari board implementation. Add SplitFlapCharset enum (Alpha/Digits/Uppercase), nine new struct fields all with #[serde(default)] for backward compat, solari_preset + with_from_message builders, estimated_flap_ms helper, AUTHENTIC_FLAP_MS + ALPHA_POOL_SIZE constants, BLOCK_CHARS + HINGE_CHARS rotation sequences, DampedSpring hinge remapping. ContentEffect::SplitFlap grows seven optional fields; existing `{ "type": "split_flap", "speed": X, "cascade": Y }` recipes deserialize unchanged.</CLOG>

use crate::traits::TextTransformer;
use mixed_signals::physics::DampedSpring;
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

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

/// Split-flap (Solari) text transformer.
#[derive(Debug, Clone)]
pub struct SplitFlap {
    pub speed: SignalOrFloat,
    pub cascade: SignalOrFloat,
    pub cycles: SignalOrFloat,
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
}

impl SplitFlap {
    /// Target per-flap rotation time in milliseconds on a real Solari board.
    /// Physical flaps rotate in ~30-50ms under gravity + spring assist.
    /// Alitalia Milan Linate ~40ms, Frankfurt Hauptbahnhof ~35ms,
    /// Philadelphia 30th Street ~45-50ms.
    pub const AUTHENTIC_FLAP_MS: f32 = 35.0;
    pub const ALPHA_POOL_SIZE: f32 = 42.0;

    /// 2.1.0-compat constructor — speed + cascade only.
    pub fn new(speed: SignalOrFloat, cascade: SignalOrFloat) -> Self {
        Self {
            speed,
            cascade,
            cycles: SignalOrFloat::default(),
            jitter: 0.0,
            charset: SplitFlapCharset::Alpha,
            settle_overshoot: false,
            leading_blocks: 0.0,
            settle_hinge: false,
            spring_settle: false,
            authentic_timing: false,
            from_message: None,
            rolling_flip: false,
        }
    }

    /// Full 3.0.0 constructor.
    #[allow(clippy::too_many_arguments)]
    pub fn new_mechanical(
        speed: SignalOrFloat,
        cascade: SignalOrFloat,
        cycles: SignalOrFloat,
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
        }
    }

    /// Enable `rolling_flip` mode on this transformer.
    pub fn with_rolling_flip(mut self, rolling: bool) -> Self {
        self.rolling_flip = rolling;
        self
    }

    /// Message-to-message transition: each column rotates from the
    /// character at `from[i]` to the character at `target[i]`.
    pub fn with_from_message(mut self, from: impl Into<String>) -> Self {
        self.from_message = Some(from.into());
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
            SignalOrFloat::from(1.0),
            SignalOrFloat::from(0.0_f32), // cascade=0: real boards dispatch all columns simultaneously
            SignalOrFloat::from(cycles),
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
    pub fn estimated_flap_ms(&self, animation_ms: f32) -> Option<f32> {
        let cycles = self.cycles.as_static()?;
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
        let mut hash: u32 = 0x811c9dc5;
        for b in (char_index as u32).to_le_bytes().iter() {
            hash ^= u32::from(*b);
            hash = hash.wrapping_mul(0x01000193);
        }
        let normalized = (hash as f64 / u32::MAX as f64) * 2.0 - 1.0;
        1.0 + (normalized * self.jitter as f64)
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
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let speed = self
            .speed
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .max(0.0);
        let cascade = self
            .cascade
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .max(0.0);
        let cycles = self
            .cycles
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .max(0.0) as f64;

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

        // authentic_timing: pre-scan the message to find the maximum
        // per-column flap count. Every column rotates at a constant
        // per-flap rate starting from progress=0; the longest rotation
        // lands at progress=1.0, shorter rotations land earlier.
        let max_flaps = if self.authentic_timing {
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
        for (i, target_char) in target.chars().enumerate() {
            // Structural characters (newlines, tabs, carriage returns)
            // pass through unchanged at every frame. This is load-bearing
            // for multi-line messages like arrivals boards — the flap
            // mechanism doesn't exist between rows, so the row separator
            // must remain a row separator throughout the animation
            // rather than flipping through the pool like content.
            if target_char == '\n' || target_char == '\r' || target_char == '\t' {
                out.push(target_char);
                continue;
            }

            let jitter_factor = self.jitter_factor(i);

            // Resolve target_idx. If not in pool, emit the target char
            // directly when progress > 0.9, else emit the pool's space.
            let target_idx = match pool.iter().position(|&p| p == target_char.to_ascii_uppercase())
            {
                Some(idx) => idx,
                None => {
                    if progress > 0.9 {
                        out.push(target_char);
                    } else {
                        out.push(pool[0]);
                    }
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
                continue;
            }

            // Compute per-column char_progress.
            let char_progress = if self.authentic_timing {
                // Physical Solari: all columns start simultaneously, each
                // rotates at the same per-flap rate, so landing time is
                // proportional to flap distance. Jitter adds mechanical
                // imperfection across columns.
                let completion_ratio = (this_flap_distance / max_flaps).max(0.001);
                let base = progress * f64::from(speed) / completion_ratio;
                (base * jitter_factor).clamp(0.0, 1.0)
            } else {
                // Legacy cascade model — each column starts at time
                // i*cascade and walks to completion at progress=1.0
                // regardless of target distance.
                let effective_cascade = f64::from(cascade) * jitter_factor;
                (progress * f64::from(speed) - (i as f64 * effective_cascade)).clamp(0.0, 1.0)
            };

            if char_progress >= 1.0 {
                out.push(target_char);
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
                let glyph_idx =
                    (settle_phase * HINGE_CHARS.len() as f64).min(HINGE_CHARS.len() as f64 - 1.0)
                        as usize;
                out.push(HINGE_CHARS[glyph_idx]);
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
                    .min(HINGE_CHARS.len() as f64 - 1.0)
                    as usize;
                out.push(HINGE_CHARS[glyph_idx]);
                continue;
            }

            // settle_overshoot (only when settle_hinge is off — hinge owns
            // the settle window).
            let current_idx = if !self.settle_hinge
                && self.settle_overshoot
                && char_progress > 0.9
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
        }
        Cow::Owned(out)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> SignalContext {
        SignalContext::new(0, 0)
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
            SignalOrFloat::from(1.0),
            SignalOrFloat::from(0.0_f32),
            SignalOrFloat::from(cycles as f32),
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
            SignalOrFloat::from(1.0),
            SignalOrFloat::from(0.0_f32),
            SignalOrFloat::from(cycles as f32),
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
        let x = SplitFlap::new(SignalOrFloat::from(1.0), SignalOrFloat::from(0.2));
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
        assert_eq!(x.transform("HELLO", 1.0, &ctx()), "HELLO");
    }

    #[test]
    fn default_behavior_matches_v2_linear_walk() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("Z", 0.5, &ctx()).chars().next().unwrap();
        assert_ne!(c, 'Z');
    }

    // ---------- cycles ----------

    #[test]
    fn cycles_make_low_index_targets_flip_through_pool() {
        let x = sf(2.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("A", 0.25, &ctx()).chars().next().unwrap();
        assert_ne!(c, 'A');
    }

    #[test]
    fn cycles_still_land_at_progress_1() {
        let x = sf(3.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert_eq!(x.transform("HELLO", 1.0, &ctx()), "HELLO");
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
        let c = x.transform("7", 0.5, &ctx()).chars().next().unwrap();
        assert!(c.is_ascii_digit() || c == ' ');
    }

    #[test]
    fn charset_digits_lands_on_digits() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Digits, false, 0.0, false);
        assert_eq!(x.transform("2024", 1.0, &ctx()), "2024");
    }

    #[test]
    fn charset_uppercase_cycles_letters_only() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Uppercase, false, 0.0, false);
        let r = x.transform("LISBON", 0.5, &ctx());
        for c in r.chars() {
            assert!(c.is_ascii_uppercase() || c == ' ');
        }
    }

    // ---------- settle_overshoot ----------

    #[test]
    fn settle_overshoot_lands_on_target_at_progress_1() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, true, 0.0, false);
        assert_eq!(x.transform("HELLO", 1.0, &ctx()), "HELLO");
    }

    // ---------- leading_blocks ----------

    #[test]
    fn leading_blocks_shows_block_glyph_at_opening() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.3, false);
        let c = x.transform("A", 0.1, &ctx()).chars().next().unwrap();
        assert!(BLOCK_CHARS.contains(&c));
    }

    #[test]
    fn leading_blocks_zero_preserves_v2_behavior() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        let c = x.transform("A", 0.05, &ctx()).chars().next().unwrap();
        assert!(!BLOCK_CHARS.contains(&c));
    }

    // ---------- settle_hinge ----------

    #[test]
    fn settle_hinge_plays_rotation_in_settle_window() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        let c = x.transform("A", 0.9, &ctx()).chars().next().unwrap();
        assert!(HINGE_CHARS.contains(&c));
    }

    #[test]
    fn settle_hinge_progresses_through_rotation() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        let early = x.transform("A", 0.83, &ctx()).chars().next().unwrap();
        let late = x.transform("A", 0.99, &ctx()).chars().next().unwrap();
        assert_ne!(early, late);
    }

    #[test]
    fn settle_hinge_lands_on_target_at_progress_1() {
        let x = sf(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true);
        assert_eq!(x.transform("HELLO", 1.0, &ctx()), "HELLO");
    }

    // ---------- spring_settle ----------

    #[test]
    fn spring_settle_retimes_hinge_frames() {
        let linear = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true, false, false);
        let spring = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true, true, false);
        let samples = [0.835, 0.86, 0.9, 0.94, 0.97];
        let any_diff = samples.iter().any(|&t| {
            linear.transform("A", t, &ctx()).chars().next()
                != spring.transform("A", t, &ctx()).chars().next()
        });
        assert!(any_diff);
    }

    #[test]
    fn spring_settle_still_lands_on_target() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, true, true, false);
        assert_eq!(x.transform("HELLO", 1.0, &ctx()), "HELLO");
    }

    // ---------- authentic_timing ----------

    #[test]
    fn authentic_timing_short_distance_lands_early() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true);
        let r = x.transform("AZ", 0.1, &ctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars[0], 'A', "short-distance char must land early");
        assert_ne!(chars[1], 'Z', "long-distance char must still be flipping");
    }

    #[test]
    fn authentic_timing_all_land_at_progress_1() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true);
        assert_eq!(x.transform("FLIGHT 721", 1.0, &ctx()), "FLIGHT 721");
    }

    #[test]
    fn authentic_timing_identical_chars_land_together() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true);
        let r = x.transform("AA", 0.5, &ctx());
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
        let cycles = p.cycles.as_static().unwrap();
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
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true)
            .with_from_message("LL");
        for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
            let r = x.transform("LX", t, &ctx());
            assert_eq!(r.chars().next().unwrap(), 'L', "at t={t}");
        }
    }

    #[test]
    fn from_message_forward_only_drum_rotation() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true)
            .with_from_message("Z");
        assert_eq!(x.transform("A", 1.0, &ctx()), "A");
        let mid = x.transform("A", 0.5, &ctx()).chars().next().unwrap();
        assert_ne!(mid, 'A');
        assert_ne!(mid, 'Z');
    }

    #[test]
    fn from_message_lands_all_chars_at_progress_1() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true)
            .with_from_message("LONDON");
        assert_eq!(x.transform("PARIS ", 1.0, &ctx()), "PARIS ");
    }

    #[test]
    fn from_message_shorter_than_target_pads_with_space() {
        let x = sf_full(0.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, true)
            .with_from_message("AB");
        assert_eq!(x.transform("ABCDE", 1.0, &ctx()), "ABCDE");
        let r = x.transform("ABCDE", 0.05, &ctx());
        let chars: Vec<char> = r.chars().collect();
        assert_eq!(chars[0], 'A');
        assert_eq!(chars[1], 'B');
    }

    // ---------- rolling_flip: continuous card rotation ----------

    #[test]
    fn rolling_flip_shows_hinge_glyphs_during_walk() {
        // With rolling_flip enabled, every position during the walk
        // phase must be a HINGE rotation glyph, never a pool letter.
        let shader = sf_full(1.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, false)
            .with_rolling_flip(true);
        for t in [0.1, 0.3, 0.5, 0.7] {
            let c = shader.transform("Z", t, &ctx()).chars().next().unwrap();
            assert!(
                HINGE_CHARS.contains(&c),
                "rolling_flip at t={t} must show a rotation glyph, got '{c}'"
            );
        }
    }

    #[test]
    fn rolling_flip_lands_on_target_at_progress_1() {
        let shader = sf_full(1.0, 0.0, SplitFlapCharset::Alpha, false, 0.0, false, false, false)
            .with_rolling_flip(true);
        assert_eq!(shader.transform("HELLO", 1.0, &ctx()), "HELLO");
    }

    #[test]
    fn rolling_flip_default_off_preserves_letter_walk() {
        let shader = sf(0.5, 0.0, SplitFlapCharset::Alpha, false, 0.0, false);
        assert!(!shader.rolling_flip);
        // Without rolling_flip, mid-walk should show a pool letter
        // (not a hinge glyph), confirming the old walk behavior remains
        // the default.
        let c = shader.transform("Z", 0.3, &ctx()).chars().next().unwrap();
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
            let r = shader.transform("AB\nCD", t, &ctx());
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
        assert_eq!(shader.transform(target, 1.0, &ctx()), target);
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
            let r = shader.transform("X9\nY8", t, &ctx());
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
        let x = sf_full(1.0, 0.1, SplitFlapCharset::Uppercase, false, 0.15, true, false, false);
        let opening = x.transform("S", 0.05, &ctx()).chars().next().unwrap();
        assert!(BLOCK_CHARS.contains(&opening));
        let middle = x.transform("S", 0.5, &ctx()).chars().next().unwrap();
        assert!(!BLOCK_CHARS.contains(&middle) && !HINGE_CHARS.contains(&middle));
        let ending = x.transform("S", 0.9, &ctx()).chars().next().unwrap();
        assert!(HINGE_CHARS.contains(&ending));
        assert_eq!(x.transform("S", 1.0, &ctx()), "S");
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE> - <DESC>SplitFlap transformer with Solari-board mechanical feel (cycles, jitter, charset, hinge, spring, authentic timing, message transitions)</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
