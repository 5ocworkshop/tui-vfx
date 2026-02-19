// <FILE>tui-vfx-content/src/transformers/cls_dissolve.rs</FILE> - <DESC>Text dissolve transformer</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Effect parity: Content transformers</WCTX>
// <CLOG>Added Clustered, ByWord, ByLine patterns</CLOG>

use crate::traits::TextTransformer;
use crate::types::{DissolveDirection, DissolvePattern, DissolveReplacement};
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::prelude::SignalContext;
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

/// Dissolve transformer that progressively replaces characters with a replacement character.
///
/// Unlike mask dissolve (which controls pixel visibility), this transformer operates
/// at the character level, replacing visible characters with space, dot, or custom chars.
#[derive(Debug, Clone, Default)]
pub struct Dissolve {
    replacement: DissolveReplacement,
    pattern: DissolvePattern,
    direction: DissolveDirection,
    seed: u64,
}

impl Dissolve {
    pub fn new(
        replacement: DissolveReplacement,
        pattern: DissolvePattern,
        direction: DissolveDirection,
        seed: u64,
    ) -> Self {
        Self {
            replacement,
            pattern,
            direction,
            seed,
        }
    }

    /// Calculate the dissolve threshold for a character at index i.
    /// Returns a value 0.0-1.0 representing when this character dissolves.
    fn threshold(&self, i: usize, total: usize, text: &str) -> f32 {
        if total == 0 {
            return 0.0;
        }

        match &self.pattern {
            DissolvePattern::Sequential => self.sequential_threshold(i, total),
            DissolvePattern::Random => self.random_threshold(i, total),
            DissolvePattern::EdgeIn => self.edge_in_threshold(i, total),
            DissolvePattern::EdgeOut => self.edge_out_threshold(i, total),
            DissolvePattern::Clustered { cluster_size } => {
                self.clustered_threshold(i, total, *cluster_size)
            }
            DissolvePattern::ByWord => self.by_word_threshold(i, text),
            DissolvePattern::ByLine => self.by_line_threshold(i, text),
        }
    }

    fn sequential_threshold(&self, i: usize, total: usize) -> f32 {
        let pos = match self.direction {
            DissolveDirection::LeftToRight => i as f32 / total as f32,
            DissolveDirection::RightToLeft => (total - 1 - i) as f32 / total as f32,
            DissolveDirection::CenterOut => {
                let center = total as f32 / 2.0;
                let dist = (i as f32 - center).abs();
                dist / center
            }
            DissolveDirection::CenterIn => {
                let center = total as f32 / 2.0;
                let dist = (i as f32 - center).abs();
                1.0 - (dist / center)
            }
        };
        pos.clamp(0.0, 1.0)
    }

    fn random_threshold(&self, i: usize, total: usize) -> f32 {
        // Use hash to generate deterministic "random" threshold
        let idx = hash_to_index(self.seed, i as u64, total);
        idx as f32 / total as f32
    }

    fn edge_in_threshold(&self, i: usize, total: usize) -> f32 {
        // Characters at edges dissolve first, center last
        let center = total as f32 / 2.0;
        let dist = (i as f32 - center).abs();
        let max_dist = center;
        // Invert: edges (high dist) get low threshold (dissolve first)
        1.0 - (dist / max_dist).clamp(0.0, 1.0)
    }

    fn edge_out_threshold(&self, i: usize, total: usize) -> f32 {
        // Characters at center dissolve first, edges last
        let center = total as f32 / 2.0;
        let dist = (i as f32 - center).abs();
        let max_dist = center;
        // Center (low dist) gets low threshold (dissolves first)
        (dist / max_dist).clamp(0.0, 1.0)
    }

    fn clustered_threshold(&self, i: usize, total: usize, cluster_size: u8) -> f32 {
        // Group characters into clusters, each cluster dissolves together
        let cluster_size = (cluster_size as usize).max(1);
        let cluster_idx = i / cluster_size;
        let num_clusters = total.div_ceil(cluster_size);

        // Use hash to assign random order to clusters
        let cluster_order = hash_to_index(self.seed, cluster_idx as u64, num_clusters);
        cluster_order as f32 / num_clusters as f32
    }

    fn by_word_threshold(&self, char_idx: usize, text: &str) -> f32 {
        // Find which word this character belongs to
        let mut word_idx = 0;
        let mut word_count = 0;
        let mut in_word = false;

        for (current_idx, g) in text.graphemes(true).enumerate() {
            let is_space = g.chars().all(|c| c.is_whitespace());

            if !is_space && !in_word {
                // Starting a new word
                in_word = true;
                word_count += 1;
            } else if is_space && in_word {
                // Ending a word
                in_word = false;
            }

            if current_idx == char_idx {
                word_idx = if in_word { word_count - 1 } else { word_count };
                break;
            }
        }

        // Count total words
        let total_words = text.split_whitespace().count().max(1);

        // Use hash to randomize word order
        let word_order = hash_to_index(self.seed, word_idx as u64, total_words);
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

        // Count total lines
        let total_lines = text.lines().count().max(1);

        // Use hash to randomize line order
        let line_order = hash_to_index(self.seed, line_idx as u64, total_lines);
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
        word_indices
            .into_iter()
            .map(|word_idx| {
                hash_to_index(self.seed, word_idx as u64, total_words) as f32 / total_words as f32
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
        line_indices
            .into_iter()
            .map(|idx| {
                hash_to_index(self.seed, idx as u64, total_lines) as f32 / total_lines as f32
            })
            .collect()
    }
}

impl TextTransformer for Dissolve {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress <= 0.0 {
            return Cow::Borrowed(target);
        }
        if progress >= 1.0 {
            // Fully dissolved - replace all characters
            let total = len_graphemes(target);
            let replacement_char = self.replacement.char();
            return Cow::Owned(replacement_char.to_string().repeat(total));
        }

        let graphemes: Vec<&str> = target.graphemes(true).collect();
        let total = graphemes.len();
        if total == 0 {
            return Cow::Borrowed("");
        }

        let word_thresholds = if matches!(self.pattern, DissolvePattern::ByWord) {
            Some(self.build_word_thresholds(&graphemes, target))
        } else {
            None
        };

        let line_thresholds = if matches!(self.pattern, DissolvePattern::ByLine) {
            Some(self.build_line_thresholds(&graphemes, target))
        } else {
            None
        };

        let replacement_char = self.replacement.char();
        let mut result = String::with_capacity(target.len());

        for (i, g) in graphemes.iter().enumerate() {
            let threshold = match self.pattern {
                DissolvePattern::ByWord => word_thresholds
                    .as_ref()
                    .and_then(|thresholds| thresholds.get(i).copied())
                    .unwrap_or(0.0),
                DissolvePattern::ByLine => line_thresholds
                    .as_ref()
                    .and_then(|thresholds| thresholds.get(i).copied())
                    .unwrap_or(0.0),
                _ => self.threshold(i, total, target),
            };
            if progress > f64::from(threshold) {
                // Character is dissolved
                result.push(replacement_char);
            } else {
                // Character is still visible
                result.push_str(g);
            }
        }

        Cow::Owned(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_dissolve_at_zero() {
        let dissolve = Dissolve::default();
        let result = dissolve.transform("Hello", 0.0, &SignalContext::default());
        assert_eq!(result, "Hello");
    }

    #[test]
    fn test_full_dissolve_at_one() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Space,
            DissolvePattern::Sequential,
            DissolveDirection::LeftToRight,
            0,
        );
        let result = dissolve.transform("Hello", 1.0, &SignalContext::default());
        assert_eq!(result, "     ");
    }

    #[test]
    fn test_dot_replacement() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Dot,
            DissolvePattern::Sequential,
            DissolveDirection::LeftToRight,
            0,
        );
        let result = dissolve.transform("Hi", 1.0, &SignalContext::default());
        assert_eq!(result, "..");
    }

    #[test]
    fn test_custom_replacement() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Custom('█'),
            DissolvePattern::Sequential,
            DissolveDirection::LeftToRight,
            0,
        );
        let result = dissolve.transform("AB", 1.0, &SignalContext::default());
        assert_eq!(result, "██");
    }

    #[test]
    fn test_left_to_right_partial() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Space,
            DissolvePattern::Sequential,
            DissolveDirection::LeftToRight,
            0,
        );
        // At 50% progress, roughly half should be dissolved
        let result = dissolve.transform("ABCD", 0.5, &SignalContext::default());
        // First two characters should be dissolved
        assert!(result.starts_with("  "));
    }

    #[test]
    fn test_right_to_left_partial() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Space,
            DissolvePattern::Sequential,
            DissolveDirection::RightToLeft,
            0,
        );
        let result = dissolve.transform("ABCD", 0.5, &SignalContext::default());
        // Last two characters should be dissolved
        assert!(result.ends_with("  "));
    }

    #[test]
    fn test_random_deterministic() {
        let dissolve = Dissolve::new(
            DissolveReplacement::Space,
            DissolvePattern::Random,
            DissolveDirection::LeftToRight,
            42,
        );
        let result1 = dissolve.transform("Hello", 0.5, &SignalContext::default());
        let result2 = dissolve.transform("Hello", 0.5, &SignalContext::default());
        assert_eq!(result1, result2);
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_dissolve.rs</FILE> - <DESC>Text dissolve transformer</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
