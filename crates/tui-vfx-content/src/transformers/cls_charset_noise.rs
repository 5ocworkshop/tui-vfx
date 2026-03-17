// <FILE>tui-vfx-content/src/transformers/cls_charset_noise.rs</FILE> - <DESC>Non-converging time-varying character replacement with vertical gradient</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>New CharsetNoise transformer for living textures (fire, rain, static, smoke)</WCTX>
// <CLOG>Initial creation: hash-based character replacement at configurable hz with position-aware charset gradient and per-cell jitter</CLOG>

use crate::traits::TextTransformer;
use crate::types::cls_charset_noise_config::{AffectMode, GradientStop};
use mixed_signals::prelude::SignalContext;
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

/// Non-converging content transformer that replaces characters from a charset,
/// varying over time. Unlike Scramble (which resolves toward target), CharsetNoise
/// cycles indefinitely — each 1/hz seconds, every affected cell gets a fresh
/// deterministic pick from its position-appropriate charset pool.
///
/// With a vertical gradient of charsets (sparse at top, dense at bottom) and
/// jitter for per-cell variation, this produces living textures like fire,
/// rain, smoke, or static noise from a single recipe.
///
/// Including empty characters (like ⠀) in sparse pools causes cells to flicker
/// between visible and invisible, making the shape boundary itself fluctuate.
#[derive(Debug, Clone)]
pub struct CharsetNoise {
    seed: u64,
    hz: f32,
    jitter: f32,
    affect: AffectMode,
    /// Gradient stops sorted by `at` position. Must have at least one stop.
    gradient: Vec<GradientStop>,
}

impl CharsetNoise {
    pub fn new(
        seed: u64,
        hz: f32,
        jitter: f32,
        affect: AffectMode,
        gradient: Vec<GradientStop>,
    ) -> Self {
        let mut gradient = gradient;
        gradient.sort_by(|a, b| a.at.partial_cmp(&b.at).unwrap_or(std::cmp::Ordering::Equal));
        Self {
            seed,
            hz: hz.max(0.1),
            jitter: jitter.clamp(0.0, 1.0),
            affect,
            gradient,
        }
    }

    /// Construct from a flat (non-gradient) charset.
    pub fn flat(seed: u64, hz: f32, chars: String) -> Self {
        Self::new(
            seed,
            hz,
            0.0,
            AffectMode::NonEmpty,
            vec![GradientStop { at: 0.0, chars }],
        )
    }

    /// Cheap deterministic hash for per-cell variation. Same family as the
    /// grimoire's cell_hash and braille_dust's noise function.
    fn cell_hash(seed: u64, row: u32, col: u32, time_step: u64) -> u64 {
        let mut h = seed
            .wrapping_mul(2654435761)
            .wrapping_add(row as u64 * 131)
            .wrapping_add(col as u64 * 997)
            .wrapping_add(time_step.wrapping_mul(7919));
        h ^= h >> 16;
        h = h.wrapping_mul(0x45d9f3b);
        h ^= h >> 16;
        h
    }

    /// Select the charset for a given effective vertical position.
    /// Returns the chars string of the nearest gradient stop.
    fn charset_at(&self, effective_pos: f32) -> &str {
        if self.gradient.len() == 1 {
            return &self.gradient[0].chars;
        }

        let pos = effective_pos.clamp(0.0, 1.0);

        // Find the nearest stop
        let mut best_idx = 0;
        let mut best_dist = f32::MAX;
        for (i, stop) in self.gradient.iter().enumerate() {
            let dist = (stop.at - pos).abs();
            if dist < best_dist {
                best_dist = dist;
                best_idx = i;
            }
        }
        &self.gradient[best_idx].chars
    }

    /// Check whether a grapheme should be affected based on the AffectMode.
    fn should_affect(&self, grapheme: &str) -> bool {
        match self.affect {
            AffectMode::All => true,
            AffectMode::NonEmpty => {
                // Skip ASCII space, empty braille (⠀ = U+2800), and other whitespace
                !grapheme.chars().all(|c| c.is_whitespace() || c == '\u{2800}')
            }
        }
    }
}

impl TextTransformer for CharsetNoise {
    fn transform<'a>(
        &self,
        target: &'a str,
        _progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if self.gradient.is_empty() {
            return Cow::Borrowed(target);
        }

        // Compute the time step from absolute_t (milliseconds) and hz.
        // This determines which "frame" of the noise we're on.
        let absolute_t_ms = signal_ctx.absolute_t.unwrap_or(0.0);
        let time_step = (absolute_t_ms * self.hz as f64 / 1000.0).floor() as u64;

        // Count total lines for vertical positioning
        let lines: Vec<&str> = target.split('\n').collect();
        let total_lines = lines.len().max(1) as f32;

        let mut result = String::with_capacity(target.len());
        let mut any_changed = false;

        for (line_idx, line) in lines.iter().enumerate() {
            if line_idx > 0 {
                result.push('\n');
            }

            let v_pos = line_idx as f32 / (total_lines - 1.0).max(1.0);

            for (col_idx, grapheme) in line.graphemes(true).enumerate() {
                if !self.should_affect(grapheme) {
                    result.push_str(grapheme);
                    continue;
                }

                let h = Self::cell_hash(self.seed, line_idx as u32, col_idx as u32, time_step);

                // Apply per-cell jitter to vertical position
                let jitter_offset = if self.jitter > 0.0 {
                    let jitter_raw = ((h % 101) as f32 / 100.0 - 0.5) * 2.0;
                    jitter_raw * self.jitter
                } else {
                    0.0
                };
                let effective_pos = (v_pos + jitter_offset).clamp(0.0, 1.0);

                let charset = self.charset_at(effective_pos);
                let chars: Vec<char> = charset.chars().collect();
                if chars.is_empty() {
                    result.push_str(grapheme);
                    continue;
                }

                // Pick a character from the charset using the hash
                let char_idx = hash_to_index(self.seed.wrapping_add(time_step), col_idx as u64 + line_idx as u64 * 1000, chars.len());
                result.push(chars[char_idx]);
                any_changed = true;
            }
        }

        if any_changed {
            Cow::Owned(result)
        } else {
            Cow::Borrowed(target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_ctx(absolute_t: f64) -> SignalContext {
        SignalContext {
            frame: 0,
            seed: 0,
            width: 16,
            height: 16,
            phase: None,
            phase_t: None,
            loop_t: None,
            absolute_t: Some(absolute_t),
            char_index: None,
        }
    }

    #[test]
    fn flat_charset_replaces_non_empty() {
        let tx = CharsetNoise::flat(42, 8.0, "AB".to_string());
        let result = tx.transform("⣿⠀⣿", 0.5, &test_ctx(0.0));
        let chars: Vec<char> = result.chars().collect();
        // First char replaced with A or B
        assert!(chars[0] == 'A' || chars[0] == 'B');
        // Second char is empty braille — skipped
        assert_eq!(chars[1], '⠀');
        // Third char replaced
        assert!(chars[2] == 'A' || chars[2] == 'B');
    }

    #[test]
    fn empty_cells_skipped_in_non_empty_mode() {
        let tx = CharsetNoise::flat(42, 8.0, "X".to_string());
        let result = tx.transform("⠀ ⠀", 0.5, &test_ctx(0.0));
        assert_eq!(result, "⠀ ⠀");
    }

    #[test]
    fn all_mode_replaces_everything() {
        let tx = CharsetNoise::new(
            42, 8.0, 0.0, AffectMode::All,
            vec![GradientStop { at: 0.0, chars: "X".to_string() }],
        );
        let result = tx.transform("⠀ ⠀", 0.5, &test_ctx(0.0));
        assert_eq!(result, "XXX");
    }

    #[test]
    fn different_time_produces_different_output() {
        let tx = CharsetNoise::flat(42, 8.0, "ABCDEFGHIJ".to_string());
        let input = "⣿⣿⣿⣿⣿";
        let out1 = tx.transform(input, 0.5, &test_ctx(0.0));
        let out2 = tx.transform(input, 0.5, &test_ctx(200.0)); // 200ms later, different time_step
        assert_ne!(out1, out2, "Different time steps should produce different characters");
    }

    #[test]
    fn same_time_is_deterministic() {
        let tx = CharsetNoise::flat(42, 8.0, "ABCDEFGHIJ".to_string());
        let input = "⣿⣿⣿⣿⣿";
        let out1 = tx.transform(input, 0.5, &test_ctx(500.0));
        let out2 = tx.transform(input, 0.5, &test_ctx(500.0));
        assert_eq!(out1, out2, "Same inputs must produce same output");
    }

    #[test]
    fn gradient_top_gets_sparse_pool() {
        // Two-stop gradient: top = "AB", bottom = "XY"
        let tx = CharsetNoise::new(
            42, 8.0, 0.0, AffectMode::NonEmpty,
            vec![
                GradientStop { at: 0.0, chars: "A".to_string() },
                GradientStop { at: 1.0, chars: "Z".to_string() },
            ],
        );
        // Two lines: line 0 = top (position 0.0), line 1 = bottom (position 1.0)
        let result = tx.transform("⣿\n⣿", 0.5, &test_ctx(0.0));
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines[0], "A", "Top line should use first gradient stop");
        assert_eq!(lines[1], "Z", "Bottom line should use last gradient stop");
    }

    #[test]
    fn jitter_creates_variation_at_same_row() {
        // High jitter on a multiline input — cells at the same row may get different pools
        let tx = CharsetNoise::new(
            42, 8.0, 0.5, AffectMode::NonEmpty,
            vec![
                GradientStop { at: 0.0, chars: "A".to_string() },
                GradientStop { at: 1.0, chars: "Z".to_string() },
            ],
        );
        // 4 lines, wide enough that jitter has room to push some cells to different stops
        let input = "⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿\n⣿⣿⣿⣿⣿⣿⣿⣿";
        let result = tx.transform(input, 0.5, &test_ctx(0.0));
        // Middle lines should have a mix of A and Z due to jitter
        let has_a = result.contains('A');
        let has_z = result.contains('Z');
        assert!(has_a && has_z, "Jitter should cause both gradient stops to appear");
    }

    #[test]
    fn empty_gradient_returns_original() {
        let tx = CharsetNoise::new(42, 8.0, 0.0, AffectMode::NonEmpty, vec![]);
        let result = tx.transform("⣿⣿⣿", 0.5, &test_ctx(0.0));
        assert_eq!(result, "⣿⣿⣿");
    }

    #[test]
    fn hz_controls_change_rate() {
        let tx = CharsetNoise::flat(42, 1.0, "ABCDEFGHIJ".to_string());
        let input = "⣿⣿⣿⣿⣿";
        // At 1hz, time_step changes every 1000ms
        let out_0ms = tx.transform(input, 0.5, &test_ctx(0.0));
        let out_500ms = tx.transform(input, 0.5, &test_ctx(500.0));
        let out_1100ms = tx.transform(input, 0.5, &test_ctx(1100.0));
        assert_eq!(out_0ms, out_500ms, "Same time_step within 1s window");
        assert_ne!(out_0ms, out_1100ms, "Different time_step after 1s");
    }

    #[test]
    fn multiline_preserves_newlines() {
        let tx = CharsetNoise::flat(42, 8.0, "X".to_string());
        let result = tx.transform("⣿⣿\n⣿⣿\n⣿⣿", 0.5, &test_ctx(0.0));
        let lines: Vec<&str> = result.split('\n').collect();
        assert_eq!(lines.len(), 3);
        assert!(lines.iter().all(|l| l.len() == "XX".len()));
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_charset_noise.rs</FILE> - <DESC>Non-converging time-varying character replacement with vertical gradient</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
