// <FILE>tui-vfx-content/src/transformers/cls_morph.rs</FILE> - <DESC>Text morph transformer</DESC>
// <VERS>VERSION: 1.8.0</VERS>
// <WCTX>OFPF refactoring: extract character generators</WCTX>
// <CLOG>Move character generators to fnc_morph_chars.rs</CLOG>

use crate::traits::TextTransformer;
use crate::types::{MorphDirection, MorphProgression};
use mixed_signals::prelude::SignalContext;
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

use super::fnc_morph_chars::{
    BRAILLE_LEFT_COL, BRAILLE_RIGHT_COL, binary_char, braille_char_down, braille_char_up,
    density_char, density_char_reverse,
};

/// Morph transformer that transitions between source and target text.
///
/// Characters change from source to target based on progression pattern.
#[derive(Debug, Clone, Default)]
pub struct Morph {
    source: String,
    progression: MorphProgression,
    direction: MorphDirection,
    seed: u64,
}

impl Morph {
    pub fn new(
        source: String,
        progression: MorphProgression,
        direction: MorphDirection,
        seed: u64,
    ) -> Self {
        Self {
            source,
            progression,
            direction,
            seed,
        }
    }

    /// Calculate the morph threshold for a character at index i.
    /// Returns a value 0.0-1.0 representing when this character morphs.
    fn threshold(&self, i: usize, total: usize, target: &str) -> f32 {
        if total == 0 {
            return 0.0;
        }

        match self.progression {
            MorphProgression::Linear => self.linear_threshold(i, total),
            MorphProgression::Scatter => self.scatter_threshold(i, total),
            MorphProgression::Wave => self.wave_threshold(i, total),
            // Density, Binary, and Braille use linear threshold but show intermediate chars
            MorphProgression::Density | MorphProgression::Binary | MorphProgression::Braille => {
                self.linear_threshold(i, total)
            }
            // Cell-level reveals don't use threshold - all chars animate together
            MorphProgression::DensityReveal
            | MorphProgression::DensityConceal
            | MorphProgression::BrailleReveal
            | MorphProgression::BrailleRevealDown => 0.0,
            // Braille wave patterns (linear sweep)
            MorphProgression::BrailleWaveUp | MorphProgression::BrailleWaveDown => {
                self.linear_threshold(i, total)
            }
            // Braille random patterns
            MorphProgression::BrailleRandomUp | MorphProgression::BrailleRandomDown => {
                self.scatter_threshold(i, total)
            }
            // Braille by word patterns
            MorphProgression::BrailleByWordUp | MorphProgression::BrailleByWordDown => {
                self.by_word_threshold(i, target)
            }
            // Braille by line patterns
            MorphProgression::BrailleByLineUp | MorphProgression::BrailleByLineDown => {
                self.by_line_threshold(i, target)
            }
            // Half-cell wipe uses special handling in transform, threshold not used
            MorphProgression::BrailleHalfCellWipe => self.linear_threshold(i, total),
            MorphProgression::BrailleHalfCellWipeByWord => self.by_word_threshold(i, target),
        }
    }

    fn linear_threshold(&self, i: usize, total: usize) -> f32 {
        let pos = match self.direction {
            MorphDirection::LeftToRight => i as f32 / total as f32,
            MorphDirection::RightToLeft => (total - 1 - i) as f32 / total as f32,
            MorphDirection::Simultaneous => 0.5, // All change at midpoint
        };
        pos.clamp(0.0, 1.0)
    }

    fn scatter_threshold(&self, i: usize, total: usize) -> f32 {
        // Use hash to generate deterministic "random" threshold
        let idx = hash_to_index(self.seed, i as u64, total);
        idx as f32 / total as f32
    }

    fn wave_threshold(&self, i: usize, total: usize) -> f32 {
        // Create a wave pattern based on direction
        let base_pos = match self.direction {
            MorphDirection::LeftToRight => i as f32 / total as f32,
            MorphDirection::RightToLeft => (total - 1 - i) as f32 / total as f32,
            MorphDirection::Simultaneous => 0.5,
        };
        // Add sinusoidal variation
        let wave = (base_pos * std::f32::consts::PI * 2.0).sin() * 0.1;
        (base_pos + wave).clamp(0.0, 1.0)
    }

    /// Generate a deterministic permutation of indices using mixed-signals Fisher-Yates shuffle
    fn permute_indices(seed: u64, count: usize) -> Vec<usize> {
        use mixed_signals::rng::Rng;
        let mut indices: Vec<usize> = (0..count).collect();
        let mut rng = Rng::with_seed(seed);
        rng.shuffle(&mut indices);
        indices
    }

    fn by_word_threshold(&self, char_idx: usize, text: &str) -> f32 {
        // Find which word this character belongs to
        let mut word_idx = 0;
        let mut word_count = 0;
        let mut in_word = false;

        for (current_idx, g) in text.graphemes(true).enumerate() {
            let is_space = g.chars().all(|c| c.is_whitespace());

            if !is_space && !in_word {
                in_word = true;
                word_count += 1;
            } else if is_space && in_word {
                in_word = false;
            }

            if current_idx == char_idx {
                word_idx = if in_word { word_count - 1 } else { word_count };
                break;
            }
        }

        let total_words = text.split_whitespace().count().max(1);

        // Create a proper permutation to ensure each word gets a unique order
        let permutation = Self::permute_indices(self.seed, total_words);
        // Find where this word_idx appears in the permutation
        let word_order = permutation.iter().position(|&x| x == word_idx).unwrap_or(0);

        word_order as f32 / total_words as f32
    }

    fn by_line_threshold(&self, char_idx: usize, text: &str) -> f32 {
        // Find which line this character belongs to
        let mut line_idx = 0;

        for (current_idx, g) in text.graphemes(true).enumerate() {
            if current_idx == char_idx {
                break;
            }
            if g == "\n" {
                line_idx += 1;
            }
        }

        let total_lines = text.lines().count().max(1);

        // Create a proper permutation to ensure each line gets a unique order
        let permutation = Self::permute_indices(self.seed, total_lines);
        let line_order = permutation.iter().position(|&x| x == line_idx).unwrap_or(0);

        line_order as f32 / total_lines as f32
    }

    fn build_word_thresholds(&self, graphemes: &[&str], text: &str) -> Vec<f32> {
        let mut word_indices = Vec::with_capacity(graphemes.len());
        let mut word_count = 0;
        let mut in_word = false;

        for g in graphemes {
            let is_space = g.chars().all(|c| c.is_whitespace());

            if !is_space && !in_word {
                in_word = true;
                word_count += 1;
            } else if is_space && in_word {
                in_word = false;
            }

            let word_idx = if in_word { word_count - 1 } else { word_count };
            word_indices.push(word_idx);
        }

        let total_words = text.split_whitespace().count().max(1);
        let permutation = Self::permute_indices(self.seed, total_words);
        let mut order_by_word = vec![0_usize; total_words];
        for (order, word_idx) in permutation.into_iter().enumerate() {
            if word_idx < total_words {
                order_by_word[word_idx] = order;
            }
        }

        word_indices
            .into_iter()
            .map(|word_idx| {
                if word_idx < total_words {
                    order_by_word[word_idx] as f32 / total_words as f32
                } else {
                    0.0
                }
            })
            .collect()
    }

    fn build_line_thresholds(&self, graphemes: &[&str], text: &str) -> Vec<f32> {
        let mut line_indices = Vec::with_capacity(graphemes.len());
        let mut line_idx = 0;

        for g in graphemes {
            line_indices.push(line_idx);
            if *g == "\n" {
                line_idx += 1;
            }
        }

        let total_lines = text.lines().count().max(1);
        let permutation = Self::permute_indices(self.seed, total_lines);
        let mut order_by_line = vec![0_usize; total_lines];
        for (order, idx) in permutation.into_iter().enumerate() {
            if idx < total_lines {
                order_by_line[idx] = order;
            }
        }

        line_indices
            .into_iter()
            .map(|idx| {
                if idx < total_lines {
                    order_by_line[idx] as f32 / total_lines as f32
                } else {
                    0.0
                }
            })
            .collect()
    }
}

impl TextTransformer for Morph {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress <= 0.0 {
            // Return source text
            return Cow::Owned(self.source.clone());
        }
        if progress >= 1.0 {
            // Return target text
            return Cow::Borrowed(target);
        }

        let source_graphemes: Vec<&str> = self.source.graphemes(true).collect();
        let target_graphemes: Vec<&str> = target.graphemes(true).collect();

        let word_thresholds = if matches!(
            self.progression,
            MorphProgression::BrailleByWordUp
                | MorphProgression::BrailleByWordDown
                | MorphProgression::BrailleHalfCellWipeByWord
        ) {
            Some(self.build_word_thresholds(&target_graphemes, target))
        } else {
            None
        };

        let line_thresholds = if matches!(
            self.progression,
            MorphProgression::BrailleByLineUp | MorphProgression::BrailleByLineDown
        ) {
            Some(self.build_line_thresholds(&target_graphemes, target))
        } else {
            None
        };

        // Use the longer of the two for iteration
        let max_len = source_graphemes.len().max(target_graphemes.len());
        if max_len == 0 {
            return Cow::Borrowed("");
        }

        let mut result = String::with_capacity(target.len().max(self.source.len()));

        for i in 0..max_len {
            let threshold = match self.progression {
                MorphProgression::BrailleByWordUp
                | MorphProgression::BrailleByWordDown
                | MorphProgression::BrailleHalfCellWipeByWord => word_thresholds
                    .as_ref()
                    .and_then(|thresholds| thresholds.get(i).copied())
                    .unwrap_or(0.0),
                MorphProgression::BrailleByLineUp | MorphProgression::BrailleByLineDown => {
                    line_thresholds
                        .as_ref()
                        .and_then(|thresholds| thresholds.get(i).copied())
                        .unwrap_or(0.0)
                }
                _ => self.threshold(i, max_len, target),
            };
            let threshold = f64::from(threshold);
            let source_char = source_graphemes.get(i).copied().unwrap_or(" ");
            let target_char = target_graphemes.get(i).copied().unwrap_or(" ");

            match self.progression {
                MorphProgression::Density => {
                    // Calculate local progress for this character (0 = source, 1 = target)
                    // Spread each char's transition over a window
                    let window = 0.3_f64; // Each char transitions over 30% of total time
                    let char_start = threshold * (1.0 - window);
                    let local_progress = ((progress - char_start) / window).clamp(0.0, 1.0);

                    if local_progress <= 0.0 {
                        result.push_str(source_char);
                    } else if local_progress >= 1.0 {
                        result.push_str(target_char);
                    } else {
                        // Show density block character
                        result.push_str(density_char(local_progress as f32));
                    }
                }
                MorphProgression::Binary => {
                    let window = 0.4_f64;
                    let char_start = threshold * (1.0 - window);
                    let local_progress = ((progress - char_start) / window).clamp(0.0, 1.0);

                    if local_progress <= 0.0 {
                        result.push_str(source_char);
                    } else if local_progress >= 1.0 {
                        result.push_str(target_char);
                    } else {
                        // Show binary digit
                        result.push(binary_char(local_progress as f32, self.seed, i));
                    }
                }
                MorphProgression::Braille => {
                    let window = 0.35_f64; // Each char transitions over 35% of total time
                    let char_start = threshold * (1.0 - window);
                    let local_progress = ((progress - char_start) / window).clamp(0.0, 1.0);

                    if local_progress <= 0.0 {
                        result.push_str(source_char);
                    } else if local_progress >= 1.0 {
                        result.push_str(target_char);
                    } else {
                        // Show braille character with increasing dots
                        result.push_str(braille_char_up(local_progress as f32));
                    }
                }
                MorphProgression::DensityReveal => {
                    // Cell-level reveal: all chars animate simultaneously
                    // 0.0-0.8: cycle through density blocks, 0.8-1.0: reveal target
                    if progress < 0.8 {
                        // Scale progress to 0.0-1.0 for density cycling
                        let density_progress = progress / 0.8;
                        result.push_str(density_char(density_progress as f32));
                    } else {
                        result.push_str(target_char);
                    }
                }
                MorphProgression::DensityConceal => {
                    // Cell-level conceal: all chars animate simultaneously (reverse order)
                    // 0.0-0.8: cycle through density blocks solid→sparse, 0.8-1.0: reveal target
                    if progress < 0.8 {
                        // Scale progress to 0.0-1.0 for density cycling (reverse)
                        let density_progress = progress / 0.8;
                        result.push_str(density_char_reverse(density_progress as f32));
                    } else {
                        result.push_str(target_char);
                    }
                }
                MorphProgression::BrailleReveal => {
                    // Cell-level reveal: all chars animate simultaneously (fill up)
                    // 0.0-0.85: cycle through braille dots, 0.85-1.0: reveal target
                    if progress < 0.85 {
                        let braille_progress = progress / 0.85;
                        result.push_str(braille_char_up(braille_progress as f32));
                    } else {
                        result.push_str(target_char);
                    }
                }
                MorphProgression::BrailleRevealDown => {
                    // Cell-level reveal: all chars animate simultaneously (fill down)
                    // 0.0-0.85: cycle through braille dots, 0.85-1.0: reveal target
                    if progress < 0.85 {
                        let braille_progress = progress / 0.85;
                        result.push_str(braille_char_down(braille_progress as f32));
                    } else {
                        result.push_str(target_char);
                    }
                }
                MorphProgression::BrailleWaveUp
                | MorphProgression::BrailleRandomUp
                | MorphProgression::BrailleByWordUp
                | MorphProgression::BrailleByLineUp => {
                    // Braille wave patterns (fill up): each char/word/line cycles through braille
                    let window = 0.4_f64; // Each unit transitions over 40% of total time
                    let char_start = threshold * (1.0 - window);
                    let local_progress = ((progress - char_start) / window).clamp(0.0, 1.0);

                    if local_progress <= 0.0 {
                        result.push_str(source_char);
                    } else if local_progress >= 1.0 {
                        result.push_str(target_char);
                    } else {
                        // Show braille character with increasing dots (fill up)
                        result.push_str(braille_char_up(local_progress as f32));
                    }
                }
                MorphProgression::BrailleWaveDown
                | MorphProgression::BrailleRandomDown
                | MorphProgression::BrailleByWordDown
                | MorphProgression::BrailleByLineDown => {
                    // Braille wave patterns (fill down): each char/word/line cycles through braille
                    let window = 0.4_f64; // Each unit transitions over 40% of total time
                    let char_start = threshold * (1.0 - window);
                    let local_progress = ((progress - char_start) / window).clamp(0.0, 1.0);

                    if local_progress <= 0.0 {
                        result.push_str(source_char);
                    } else if local_progress >= 1.0 {
                        result.push_str(target_char);
                    } else {
                        // Show braille character with decreasing dots (fill down)
                        result.push_str(braille_char_down(local_progress as f32));
                    }
                }
                MorphProgression::BrailleHalfCellWipe
                | MorphProgression::BrailleHalfCellWipeByWord => {
                    // Half-cell wipe: uses braille columns for sub-character resolution
                    // The wipe has a "half step" leading edge before fully revealing
                    //
                    // For left-to-right: left column (⡇) leads, then target reveals
                    // For right-to-left: right column (⣸) leads, then target reveals
                    //
                    // We use half-cell positions: each char has positions i and i+0.5
                    // - Position < i: source (wipe hasn't reached)
                    // - Position in [i, i+0.5): half column braille (leading edge)
                    // - Position >= i+0.5: target (revealed)

                    // Map progress to position with half-cell resolution
                    // threshold already gives us normalized position 0.0-1.0
                    // Scale to make room for the half-step leading edge

                    let wipe_pos = progress * 1.3; // Extend slightly for leading edge
                    let char_thresh = threshold;

                    match self.direction {
                        MorphDirection::LeftToRight => {
                            if wipe_pos < char_thresh {
                                result.push_str(source_char);
                            } else if wipe_pos < char_thresh + 0.08 {
                                // Leading edge: left column only
                                result.push_str(BRAILLE_LEFT_COL);
                            } else {
                                result.push_str(target_char);
                            }
                        }
                        MorphDirection::RightToLeft => {
                            if wipe_pos < char_thresh {
                                result.push_str(source_char);
                            } else if wipe_pos < char_thresh + 0.08 {
                                // Leading edge: right column only
                                result.push_str(BRAILLE_RIGHT_COL);
                            } else {
                                result.push_str(target_char);
                            }
                        }
                        MorphDirection::Simultaneous => {
                            // For simultaneous, show half-step at midpoint
                            if progress < 0.45 {
                                result.push_str(source_char);
                            } else if progress < 0.55 {
                                result.push_str(BRAILLE_LEFT_COL);
                            } else {
                                result.push_str(target_char);
                            }
                        }
                    }
                }
                _ => {
                    // Linear, Scatter, Wave: simple switch
                    if progress > threshold {
                        result.push_str(target_char);
                    } else {
                        result.push_str(source_char);
                    }
                }
            }
        }

        Cow::Owned(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_source_at_zero() {
        let morph = Morph::new(
            "Hello".to_string(),
            MorphProgression::Linear,
            MorphDirection::LeftToRight,
            0,
        );
        let result = morph.transform("World", 0.0, &SignalContext::default());
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_target_at_one() {
        let morph = Morph::new(
            "Hello".to_string(),
            MorphProgression::Linear,
            MorphDirection::LeftToRight,
            0,
        );
        let result = morph.transform("World", 1.0, &SignalContext::default());
        assert_eq!(result, "World");
    }

    #[test]
    fn test_partial_morph() {
        let morph = Morph::new(
            "AAAA".to_string(),
            MorphProgression::Linear,
            MorphDirection::LeftToRight,
            0,
        );
        let result = morph.transform("BBBB", 0.5, &SignalContext::default());
        // First half should be morphed to B, second half still A
        assert!(result.starts_with("BB"));
    }

    #[test]
    fn test_simultaneous_morph() {
        let morph = Morph::new(
            "AAA".to_string(),
            MorphProgression::Linear,
            MorphDirection::Simultaneous,
            0,
        );
        // At 0.4, threshold is 0.5 for all, so all are still source
        let result = morph.transform("BBB", 0.4, &SignalContext::default());
        assert_eq!(result, "AAA");
        // At 0.6, threshold is 0.5 for all, so all are now target
        let result = morph.transform("BBB", 0.6, &SignalContext::default());
        assert_eq!(result, "BBB");
    }

    #[test]
    fn test_different_lengths() {
        let morph = Morph::new(
            "AB".to_string(),
            MorphProgression::Linear,
            MorphDirection::LeftToRight,
            0,
        );
        // Target is longer - at progress 1.0, return target as-is
        let result = morph.transform("WXYZ", 1.0, &SignalContext::default());
        assert_eq!(result, "WXYZ");

        // At 0, return source as-is (no padding at endpoints)
        let result = morph.transform("WXYZ", 0.0, &SignalContext::default());
        assert_eq!(result, "AB");

        // During morph, padding applies for length matching
        let result = morph.transform("WXYZ", 0.5, &SignalContext::default());
        // First 2 chars morphed to target, last 2 still from source (padded with spaces)
        assert_eq!(result.len(), 4);
    }

    #[test]
    fn test_scatter_deterministic() {
        let morph = Morph::new(
            "Hello".to_string(),
            MorphProgression::Scatter,
            MorphDirection::LeftToRight,
            42,
        );
        let result1 = morph.transform("World", 0.5, &SignalContext::default());
        let result2 = morph.transform("World", 0.5, &SignalContext::default());
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_permute_indices_unique() {
        // Test that permute_indices produces a proper permutation with all unique values
        let perm = Morph::permute_indices(42, 5);
        assert_eq!(perm.len(), 5);
        // Check all values 0-4 are present exactly once
        let mut sorted = perm.clone();
        sorted.sort();
        assert_eq!(sorted, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_permute_indices_deterministic() {
        // Same seed should produce same permutation
        let perm1 = Morph::permute_indices(123, 10);
        let perm2 = Morph::permute_indices(123, 10);
        assert_eq!(perm1, perm2);
    }

    #[test]
    fn test_permute_indices_different_seeds() {
        // Different seeds should produce different permutations (with high probability)
        let perm1 = Morph::permute_indices(1, 10);
        let perm2 = Morph::permute_indices(2, 10);
        assert_ne!(perm1, perm2);
    }

    #[test]
    fn test_by_word_threshold_unique() {
        // Each word should get a unique threshold
        let morph = Morph::new(
            String::new(),
            MorphProgression::BrailleByWordUp,
            MorphDirection::LeftToRight,
            42,
        );
        let text = "ONE TWO THREE";
        // Get thresholds for the first char of each word (index 0, 4, 8)
        let thresh0 = morph.by_word_threshold(0, text); // "ONE"
        let thresh1 = morph.by_word_threshold(4, text); // "TWO"
        let thresh2 = morph.by_word_threshold(8, text); // "THREE"

        // All should be different
        assert_ne!(thresh0, thresh1);
        assert_ne!(thresh1, thresh2);
        assert_ne!(thresh0, thresh2);
    }

    #[test]
    fn test_half_cell_wipe_shows_leading_edge() {
        // Test that half-cell wipe shows braille column at leading edge
        let morph = Morph::new(
            String::new(),
            MorphProgression::BrailleHalfCellWipe,
            MorphDirection::LeftToRight,
            0,
        );
        // At very beginning, first char should show left column braille
        let result = morph.transform("ABCD", 0.05, &SignalContext::default());
        // Should contain the left column braille character ⡇
        assert!(
            result.contains("⡇"),
            "Expected left column braille at leading edge"
        );
    }

    #[test]
    fn test_half_cell_wipe_rtl_uses_right_column() {
        // Test that right-to-left wipe uses right column braille
        let morph = Morph::new(
            String::new(),
            MorphProgression::BrailleHalfCellWipe,
            MorphDirection::RightToLeft,
            0,
        );
        let result = morph.transform("ABCD", 0.05, &SignalContext::default());
        // Should contain the right column braille character ⣸
        assert!(
            result.contains("⣸"),
            "Expected right column braille for RTL wipe"
        );
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_morph.rs</FILE> - <DESC>Text morph transformer</DESC>
// <VERS>END OF VERSION: 1.8.0</VERS>
