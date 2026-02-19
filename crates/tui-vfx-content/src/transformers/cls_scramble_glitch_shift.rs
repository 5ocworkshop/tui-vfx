// <FILE>tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs</FILE> - <DESC>Combined Scramble + GlitchShift transformer</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>Per-frame signal evaluation for glitch window</WCTX>
// <CLOG>Store glitch bounds as SignalOrFloat and evaluate per frame</CLOG>

use crate::traits::TextTransformer;
use crate::types::ScrambleCharset;
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use unicode_segmentation::UnicodeSegmentation;

/// Combined transformer that scrambles text while adding a brief horizontal shift glitch.
///
/// The scramble reveals text progressively (like the Scramble transformer).
/// During the glitch window, spaces are prepended to shift content right.
/// Text will naturally clip at the right border.
#[derive(Debug, Clone)]
pub struct ScrambleGlitchShift {
    scramble_seed: u64,
    charset: ScrambleCharset,
    shift_amount: u8,
    glitch_start: SignalOrFloat,
    glitch_end: SignalOrFloat,
    /// Controls reveal pacing (per-frame signal evaluation)
    resolve_pace: SignalOrFloat,
}

impl ScrambleGlitchShift {
    pub fn new(
        scramble_seed: u64,
        charset: ScrambleCharset,
        shift_amount: u8,
        glitch_start: SignalOrFloat,
        glitch_end: SignalOrFloat,
        resolve_pace: SignalOrFloat,
    ) -> Self {
        Self {
            scramble_seed,
            charset,
            shift_amount,
            glitch_start,
            glitch_end,
            resolve_pace,
        }
    }
}

impl Default for ScrambleGlitchShift {
    fn default() -> Self {
        Self {
            scramble_seed: 0,
            charset: ScrambleCharset::Binary,
            shift_amount: 5,
            glitch_start: SignalOrFloat::Static(0.3),
            glitch_end: SignalOrFloat::Static(0.4),
            resolve_pace: SignalOrFloat::Static(1.0),
        }
    }
}

impl TextTransformer for ScrambleGlitchShift {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        // First apply scramble logic
        let scrambled = if progress >= 1.0 {
            Cow::Borrowed(target)
        } else {
            let total = len_graphemes(target);
            if total == 0 {
                Cow::Borrowed("")
            } else {
                // Evaluate resolve_pace signal per-frame (unwrap with fallback to 1.0 on error)
                let pace = self
                    .resolve_pace
                    .evaluate(progress, signal_ctx)
                    .unwrap_or(1.0)
                    .max(0.1);

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
                        let progress_seed = (progress * 1000.0) as u64;
                        let mix_seed = self.scramble_seed.wrapping_add(progress_seed);
                        let char_idx = hash_to_index(mix_seed, i as u64, available_chars.len());
                        result.push(available_chars[char_idx]);
                    }
                }
                Cow::Owned(result)
            }
        };

        // Then apply glitch shift if in the window
        let progress_f32 = progress as f32;
        let glitch_start = self
            .glitch_start
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        let glitch_end = self
            .glitch_end
            .evaluate(progress, signal_ctx)
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        if progress_f32 >= glitch_start && progress_f32 < glitch_end {
            let spaces: String = " ".repeat(self.shift_amount as usize);
            Cow::Owned(format!("{}{}", spaces, scrambled))
        } else {
            scrambled
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scramble_without_glitch() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            SignalOrFloat::Static(1.0),
        );
        // At progress 0.1, should be mostly scrambled, no shift
        let result = effect.transform("hello", 0.1, &SignalContext::default());
        assert!(!result.starts_with("     ")); // Not in glitch window
    }

    #[test]
    fn test_scramble_with_glitch_shift() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            SignalOrFloat::Static(1.0),
        );
        // At progress 0.35, should be partially scrambled with shift
        let result = effect.transform("hello", 0.35, &SignalContext::default());
        assert!(result.starts_with("     ")); // In glitch window, prepended spaces
    }

    #[test]
    fn test_fully_resolved_no_shift() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            SignalOrFloat::Static(1.0),
        );
        // At progress 1.0, fully resolved, no shift
        let result = effect.transform("hello", 1.0, &SignalContext::default());
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_shift_amount() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Alphanumeric,
            6,
            SignalOrFloat::Static(0.5),
            SignalOrFloat::Static(0.6),
            SignalOrFloat::Static(1.0),
        );
        let result = effect.transform("test", 0.55, &SignalContext::default());
        assert!(result.starts_with("      ")); // 6 spaces
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs</FILE> - <DESC>Combined Scramble + GlitchShift transformer</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
