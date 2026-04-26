// <FILE>tui-vfx-content/src/transformers/cls_marquee.rs</FILE> - <DESC>Marquee transformer</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>2.1.0: TextTransformer signature now takes &TransformContext<'_>; reads ctx.signal_ctx for speed-signal evaluation.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use mixed_signals::prelude::SignalOrFloat;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Marquee {
    width: u16,
    /// Controls scrolling speed (evaluated per-frame)
    /// Higher values = faster scrolling
    speed: SignalOrFloat,
}

impl Marquee {
    pub fn new(width: u16, speed: SignalOrFloat) -> Self {
        Self { width, speed }
    }
}

impl Default for Marquee {
    fn default() -> Self {
        Self {
            width: 10,
            speed: SignalOrFloat::Static(1.0),
        }
    }
}
impl TextTransformer for Marquee {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        let total_len = len_graphemes(target);
        if total_len == 0 {
            return Cow::Owned(" ".repeat(self.width as usize));
        }

        // Evaluate speed signal per-frame (unwrap with fallback to 1.0 on error)
        let speed = f64::from(
            self.speed
                .evaluate(progress, ctx.signal_ctx)
                .unwrap_or(1.0)
                .max(0.0),
        );

        // Calculate offset based on progress * speed
        // Higher speed = faster scrolling
        let effective_progress = progress * speed;
        let offset = ((total_len as f64) * effective_progress) as usize % total_len;
        let width = self.width as usize;
        let end = offset + width;
        if end <= total_len {
            return Cow::Borrowed(slice_graphemes(target, offset, end));
        }
        let mut result = String::with_capacity(width);
        // We need to construct the window [offset .. offset + width]
        // handling wrapping.
        let first_chunk_len = (total_len - offset).min(width);
        result.push_str(slice_graphemes(target, offset, offset + first_chunk_len));
        if first_chunk_len < width {
            // We wrapped around
            let mut filled = first_chunk_len;
            while filled < width {
                let needed = width - filled;
                let take = needed.min(total_len);
                result.push_str(slice_graphemes(target, 0, take));
                filled += take;
            }
        }
        Cow::Owned(result)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_marquee.rs</FILE> - <DESC>Marquee transformer</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
