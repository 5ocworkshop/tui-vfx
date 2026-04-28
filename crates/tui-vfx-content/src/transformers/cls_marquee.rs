// <FILE>tui-vfx-content/src/transformers/cls_marquee.rs</FILE> - <DESC>Marquee transformer</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>Packet 69-A: speed is now VfxBindableValue so hosts can drive marquee scroll rate via runtime bindings.</WCTX>
// <CLOG>2.2.0: MINOR — speed field, Marquee::new arg, Default impl, and the transform-time evaluate call all migrate from SignalOrFloat to VfxBindableValue. The evaluate call now passes ctx.runtime_params.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use std::borrow::Cow;
use tui_vfx_core::bindable::VfxBindableValue;

#[derive(Debug, Clone)]
pub struct Marquee {
    width: u16,
    /// Controls scrolling speed (evaluated per-frame). Bindable: literal,
    /// runtime binding, or signal expression. Higher values = faster scrolling.
    speed: VfxBindableValue,
}

impl Marquee {
    pub fn new(width: u16, speed: VfxBindableValue) -> Self {
        Self { width, speed }
    }
}

impl Default for Marquee {
    fn default() -> Self {
        Self {
            width: 10,
            speed: VfxBindableValue::Literal(1.0),
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

        // Evaluate speed per-frame; resolves literal / runtime binding / signal.
        // Fallback to 1.0 on missing bindings or signal-build errors.
        let speed = f64::from(
            self.speed
                .evaluate(progress, ctx.signal_ctx, ctx.runtime_params)
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
// <VERS>END OF VERSION: 2.2.0</VERS>
