// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>VERSION: 2.0.1</VERS>
// <WCTX>feat-20251224-170136: Complete signal-driven content effects</WCTX>
// <CLOG>Fixed off-by-one error in character reveal threshold calculation (i+1)/total instead of i/total</CLOG>

use crate::traits::TextTransformer;
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use mixed_signals::prelude::{SignalContext, SignalOrFloat};
use mixed_signals::random::hash_to_index;
use std::borrow::Cow;

/// Typewriter effect that reveals text character-by-character.
///
/// Supports optional speed variance for organic, human-like typing rhythm.
#[derive(Debug, Clone)]
pub struct Typewriter {
    /// Per-character timing variance (0.0 = uniform, higher = more variation)
    /// Can be static or signal-driven for time-varying effects.
    pub speed_variance: SignalOrFloat,
}

impl Typewriter {
    pub fn new(speed_variance: SignalOrFloat) -> Self {
        Self { speed_variance }
    }
}

impl Default for Typewriter {
    fn default() -> Self {
        Self {
            speed_variance: SignalOrFloat::Static(0.0),
        }
    }
}

impl TextTransformer for Typewriter {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress <= 0.0 {
            return Cow::Borrowed("");
        }
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }

        let total = len_graphemes(target);
        if total == 0 {
            return Cow::Borrowed(target);
        }

        // Evaluate speed_variance signal per-frame (unwrap with fallback to 0.0 on error)
        let variance = f64::from(
            self.speed_variance
                .evaluate(progress, signal_ctx)
                .unwrap_or(0.0),
        );

        // Calculate visible characters with per-character variance
        let mut visible = 0;
        for i in 0..total {
            let base_threshold = (i + 1) as f64 / total as f64;

            // Apply deterministic per-character variance using hash
            let char_variance = if variance.abs() > 0.0001 {
                // Use hash_to_index to map character index to a deterministic variance value
                // Map from [0, u64::MAX] to [-variance, variance]
                let hash_input = signal_ctx.seed.wrapping_add(i as u64);
                let hash_val = hash_to_index(hash_input, 0, 10000); // Map to 0-9999
                let normalized = (hash_val as f64 / 10000.0) * 2.0 - 1.0; // Map to -1.0 to 1.0
                normalized * variance
            } else {
                0.0
            };

            let threshold = (base_threshold + char_variance).clamp(0.0, 1.0);

            if progress >= threshold {
                visible = i + 1;
            } else {
                break;
            }
        }

        Cow::Borrowed(slice_graphemes(target, 0, visible))
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_typewriter.rs</FILE> - <DESC>Typewriter transformer</DESC>
// <VERS>END OF VERSION: 2.0.1</VERS>
