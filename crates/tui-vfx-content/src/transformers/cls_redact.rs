// <FILE>tui-vfx-content/src/transformers/cls_redact.rs</FILE> - <DESC>Redact transformer</DESC>
// <VERS>VERSION: 2.1.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>2.1.0: TextTransformer signature now takes &TransformContext<'_>; this transformer ignores the context and underscores the parameter.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use crate::utils::fnc_graphemes::{len_graphemes, slice_graphemes};
use std::borrow::Cow;
#[derive(Debug, Clone)]
pub struct Redact {
    symbol: char,
}
impl Redact {
    pub fn new(symbol: char) -> Self {
        Self { symbol }
    }
}
impl Default for Redact {
    fn default() -> Self {
        Self { symbol: '█' }
    }
}
impl TextTransformer for Redact {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let total = len_graphemes(target);
        let visible = if progress <= 0.0 {
            0
        } else {
            (total as f64 * progress) as usize
        };
        let revealed = slice_graphemes(target, 0, visible);
        let redacted_count = total.saturating_sub(visible);
        let mut result = String::with_capacity(target.len()); // Approx
        result.push_str(revealed);
        for _ in 0..redacted_count {
            result.push(self.symbol);
        }
        Cow::Owned(result)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_redact.rs</FILE> - <DESC>Redact transformer</DESC>
// <VERS>END OF VERSION: 2.1.0</VERS>
