// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE> - <DESC>SplitFlap transformer implementation</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Per-frame signal evaluation for split-flap timing</WCTX>
// <CLOG>Store speed/cascade as SignalOrFloat and evaluate per frame</CLOG>

use crate::traits::TextTransformer;
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use std::borrow::Cow;
const FLAP_CHARS: &[char] = &[
    ' ', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', '.',
    ',', '-', '!', '?',
];
#[derive(Debug, Clone)]
pub struct SplitFlap {
    pub speed: SignalOrFloat,
    pub cascade: SignalOrFloat,
}
impl SplitFlap {
    pub fn new(speed: SignalOrFloat, cascade: SignalOrFloat) -> Self {
        Self { speed, cascade }
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
        let mut out = String::with_capacity(target.len());
        for (i, target_char) in target.chars().enumerate() {
            // Per-character progress: delays characters further to the right based on cascade.
            let char_progress =
                (progress * f64::from(speed) - (i as f64 * f64::from(cascade))).clamp(0.0, 1.0);
            if char_progress >= 1.0 {
                out.push(target_char);
                continue;
            }
            let Some(target_idx) = FLAP_CHARS
                .iter()
                .position(|&c| c == target_char.to_ascii_uppercase())
            else {
                out.push(target_char);
                continue;
            };
            // Current index is based on progress towards target index
            let current_idx = (target_idx as f64 * char_progress) as usize;
            // Simple visual sugar: if we are very close to start (0.0), showing ' ' looks like a blank board waiting.
            // If we are moving, we cycle through FLAP_CHARS.
            let display_char = FLAP_CHARS.get(current_idx).unwrap_or(&target_char);
            out.push(*display_char);
        }
        Cow::Owned(out)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_split_flap.rs</FILE> - <DESC>SplitFlap transformer implementation</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
