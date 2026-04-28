// <FILE>tui-vfx-content/src/transformers/cls_scramble.rs</FILE> - <DESC>Scramble transformer</DESC>
// <VERS>VERSION: 3.2.0</VERS>
// <WCTX>Packet 69-A: resolve_pace is now VfxBindableValue so hosts can drive scramble pacing via runtime bindings.</WCTX>
// <CLOG>3.2.0: MINOR — resolve_pace field, Scramble::new arg, Default impl, and the transform-time evaluate call all migrate from SignalOrFloat to VfxBindableValue. The evaluate call now passes ctx.runtime_params so {"binding": "key"} resolves against the host's ShaderRuntimeParams.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::types::ScrambleCharset;
use crate::utils::fnc_graphemes::len_graphemes;
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;
use tui_vfx_core::bindable::VfxBindableValue;

/// Scramble transformer that progressively reveals text with scrambled characters.
///
/// Supports resolve_pace for controlling reveal speed (per-frame signal evaluation).
#[derive(Debug, Clone)]
pub struct Scramble {
    seed: u64,
    charset: ScrambleCharset,
    /// Controls how quickly scrambled text resolves (0.5 = faster, 1.0 = normal,
    /// 2.0 = slower). Bindable: literal, host-supplied runtime binding, or signal.
    resolve_pace: VfxBindableValue,
}

impl Scramble {
    pub fn new(seed: u64, charset: ScrambleCharset, resolve_pace: VfxBindableValue) -> Self {
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
            resolve_pace: VfxBindableValue::Literal(1.0),
        }
    }
}
impl TextTransformer for Scramble {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let total = len_graphemes(target);
        if total == 0 {
            return Cow::Borrowed("");
        }

        // Evaluate resolve_pace per-frame; resolves literal / runtime binding /
        // signal expression. Fallback to 1.0 on missing bindings or signal-build errors.
        let pace = self
            .resolve_pace
            .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
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
// <VERS>END OF VERSION: 3.2.0</VERS>
