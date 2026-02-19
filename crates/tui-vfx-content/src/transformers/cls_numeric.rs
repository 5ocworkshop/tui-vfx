// <FILE>tui-vfx-content/src/transformers/cls_numeric.rs</FILE> - <DESC>Numeric transformer</DESC>
// <VERS>VERSION: 1.0.1 - 2025-12-16T20:40:24Z</VERS>
// <WCTX>feat-20251224-155211: Signal-driven content effects</WCTX>
// <CLOG>BREAKING: Updated transform() signature to accept SignalContext parameter</CLOG>

use crate::traits::TextTransformer;
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;
#[derive(Debug, Clone)]
pub struct Numeric {
    format_str: String,
}
impl Numeric {
    pub fn new(format: &str) -> Self {
        Self {
            format_str: format.to_string(),
        }
    }
}
impl Default for Numeric {
    fn default() -> Self {
        Self {
            format_str: "{}".to_string(),
        }
    }
}
impl TextTransformer for Numeric {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        // Try parsing as f64 first (covers ints too)
        if let Ok(val) = target.parse::<f64>() {
            let current = val * progress;
            // Simple heuristic: if target parses as i64, treat current as i64 for default format
            if self.format_str == "{}" && target.parse::<i64>().is_ok() {
                return Cow::Owned(format!("{}", current as i64));
            }
            if self.format_str.contains("{:.1}") {
                return Cow::Owned(format!("{:.1}", current));
            } else if self.format_str.contains("{:.2}") {
                return Cow::Owned(format!("{:.2}", current));
            }
            // Fallback to default display
            return Cow::Owned(format!("{}", current));
        }
        // Non-numeric, return as is
        Cow::Borrowed(target)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_numeric.rs</FILE> - <DESC>Numeric transformer</DESC>
// <VERS>END OF VERSION: 1.0.1 - 2025-12-16T20:40:24Z</VERS>
