// <FILE>tui-vfx-content/src/transformers/cls_scramble.rs</FILE> - <DESC>Scramble transformer</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>feat-20251224-170136: Complete signal-driven content effects</WCTX>
// <CLOG>BREAKING: Added resolve_pace field with per-frame signal evaluation for dynamic reveal pacing</CLOG>

use crate::traits::TextTransformer;
use crate::types::ScrambleCharset;
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;

/// Scramble transformer that progressively reveals text with scrambled characters.
///
/// Supports resolve_pace for controlling reveal speed (per-frame signal evaluation).
#[derive(Debug, Clone)]
pub struct Scramble {
    seed: u64,
    charset: ScrambleCharset,
    /// Controls how quickly scrambled text resolves (0.5 = faster, 1.0 = normal, 2.0 = slower)
    /// Evaluated per-frame for dynamic pacing
    resolve_pace: SignalOrFloat,
}

impl Scramble {
    pub fn new(seed: u64, charset: ScrambleCharset, resolve_pace: SignalOrFloat) -> Self {
        Self {
            seed,
            charset,
            resolve_pace,
        }
    }
}
impl Default for Scramble {
    fn default() -> Self {
        Self {
            seed: 0,
            charset: ScrambleCharset::Alphanumeric,
            resolve_pace: SignalOrFloat::Static(1.0),
        }
    }
}
impl TextTransformer for Scramble {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let total = len_graphemes(target);
        if total == 0 {
            return Cow::Borrowed("");
        }

        // Evaluate resolve_pace signal per-frame (unwrap with fallback to 1.0 on error)
        let pace = self
            .resolve_pace
            .evaluate(progress, signal_ctx)
            .unwrap_or(1.0)
            .max(0.1);

        // Build result with scrambled/revealed characters
        let mut result = String::with_capacity(target.len());
        let available_chars = self.charset.get_chars();
        for (i, g) in target.graphemes(true).enumerate() {
            let threshold = i as f64 / total as f64;
            // Apply resolve_pace: higher pace = slower reveal (threshold gets larger relative to progress)
            let adjusted_threshold = threshold * f64::from(pace);

            if adjusted_threshold < progress {
                // Revealed
                result.push_str(g);
            } else {
                // Scrambled
                // Deterministic hash-based selection using mixed-signals
                // Mix progress into seed, then hash with character index
                let progress_seed = (progress * 1000.0) as u64;
                let mix_seed = self.seed.wrapping_add(progress_seed);
                let char_idx = hash_to_index(mix_seed, i as u64, available_chars.len());
                result.push(available_chars[char_idx]);
            }
        }
        Cow::Owned(result)
    }
}
// Helper import for the implementation
use unicode_segmentation::UnicodeSegmentation;

// <FILE>tui-vfx-content/src/transformers/cls_scramble.rs</FILE> - <DESC>Scramble transformer</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
