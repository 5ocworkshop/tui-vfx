// <FILE>tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs</FILE> - <DESC>Combined Scramble + GlitchShift transformer</DESC>
// <VERS>VERSION: 3.3.0</VERS>
// <WCTX>Packet 69-A: resolve_pace, glitch_start, glitch_end are now VfxBindableValue so hosts can drive every rate-bearing parameter via runtime bindings.</WCTX>
// <CLOG>3.3.0: MINOR — resolve_pace, glitch_start, glitch_end fields + ScrambleGlitchShift::new args + Default impl all migrate from SignalOrFloat to VfxBindableValue. All three transform-time evaluate calls now pass ctx.runtime_params. Tests updated to use VfxBindableValue::Literal.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::types::ScrambleCharset;
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use tui_vfx_core::bindable::VfxBindableValue;
use unicode_segmentation::UnicodeSegmentation;

/// Combined transformer that scrambles text while adding a brief horizontal shift glitch.
///
/// The scramble reveals text progressively (like the Scramble transformer).
/// During the glitch window, spaces are prepended to shift content right.
/// Text will naturally clip at the right border. Pacing and window boundaries
/// are bindable so hosts can drive them from app state.
#[derive(Debug, Clone)]
pub struct ScrambleGlitchShift {
    scramble_seed: u64,
    charset: ScrambleCharset,
    shift_amount: u8,
    /// Glitch-window start (0.0-1.0). Bindable.
    glitch_start: VfxBindableValue,
    /// Glitch-window end (0.0-1.0). Bindable.
    glitch_end: VfxBindableValue,
    /// Controls reveal pacing (per-frame). Bindable.
    resolve_pace: VfxBindableValue,
}

impl ScrambleGlitchShift {
    pub fn new(
        scramble_seed: u64,
        charset: ScrambleCharset,
        shift_amount: u8,
        glitch_start: VfxBindableValue,
        glitch_end: VfxBindableValue,
        resolve_pace: VfxBindableValue,
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
            glitch_start: VfxBindableValue::Literal(0.3),
            glitch_end: VfxBindableValue::Literal(0.4),
            resolve_pace: VfxBindableValue::Literal(1.0),
        }
    }
}

impl TextTransformer for ScrambleGlitchShift {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        // First apply scramble logic
        let scrambled = if progress >= 1.0 {
            Cow::Borrowed(target)
        } else {
            let total = len_graphemes(target);
            if total == 0 {
                Cow::Borrowed("")
            } else {
                // Evaluate resolve_pace per-frame; resolves literal / runtime
                // binding / signal expression. Fallback to 1.0 on missing
                // bindings or signal-build errors.
                let pace = self
                    .resolve_pace
                    .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
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

        // Then apply glitch shift if in the window. Window boundaries resolve
        // through ctx.runtime_params so {"binding": "..."} reaches the host.
        let progress_f32 = progress as f32;
        let glitch_start = self
            .glitch_start
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        let glitch_end = self
            .glitch_end
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
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
    use mixed_signals::prelude::SignalContext;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn empty_ctx() -> (SignalContext, ShaderRuntimeParams) {
        (SignalContext::default(), ShaderRuntimeParams::new())
    }

    #[test]
    fn test_scramble_without_glitch() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
            VfxBindableValue::Literal(1.0),
        );
        // At progress 0.1, should be mostly scrambled, no shift
        let (sig, params) = empty_ctx();
        let result = effect.transform("hello", 0.1, &TransformContext::new(&sig, &params));
        assert!(!result.starts_with("     ")); // Not in glitch window
    }

    #[test]
    fn test_scramble_with_glitch_shift() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
            VfxBindableValue::Literal(1.0),
        );
        // At progress 0.35, should be partially scrambled with shift
        let (sig, params) = empty_ctx();
        let result = effect.transform("hello", 0.35, &TransformContext::new(&sig, &params));
        assert!(result.starts_with("     ")); // In glitch window, prepended spaces
    }

    #[test]
    fn test_fully_resolved_no_shift() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Binary,
            5,
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
            VfxBindableValue::Literal(1.0),
        );
        // At progress 1.0, fully resolved, no shift
        let (sig, params) = empty_ctx();
        let result = effect.transform("hello", 1.0, &TransformContext::new(&sig, &params));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_shift_amount() {
        let effect = ScrambleGlitchShift::new(
            42,
            ScrambleCharset::Alphanumeric,
            6,
            VfxBindableValue::Literal(0.5),
            VfxBindableValue::Literal(0.6),
            VfxBindableValue::Literal(1.0),
        );
        let (sig, params) = empty_ctx();
        let result = effect.transform("test", 0.55, &TransformContext::new(&sig, &params));
        assert!(result.starts_with("      ")); // 6 spaces
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_scramble_glitch_shift.rs</FILE> - <DESC>Combined Scramble + GlitchShift transformer</DESC>
// <VERS>END OF VERSION: 3.3.0</VERS>
