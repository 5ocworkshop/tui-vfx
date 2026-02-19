// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE> - <DESC>Odometer transformer implementation</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>feat-20251224-155211: Signal-driven content effects</WCTX>
// <CLOG>BREAKING: Updated transform() signature to accept SignalContext parameter</CLOG>

use crate::traits::TextTransformer;
use mixed_signals::prelude::SignalContext;
use std::borrow::Cow;
#[derive(Debug, Clone, Default)]
pub struct Odometer;
impl TextTransformer for Odometer {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        _signal_ctx: &SignalContext,
    ) -> Cow<'a, str> {
        if progress >= 1.0 {
            return Cow::Borrowed(target);
        }
        let mut out = String::with_capacity(target.len());
        for c in target.chars() {
            if let Some(d) = c.to_digit(10) {
                // Determine displayed digit based on progress.
                // Concept: we scroll from 0 to digit `d`.
                // Map progress 0..1 to 0..d (float).
                let val = (d as f64) * progress;
                let current_digit = val.floor() as u32;
                let _fraction = val.fract();
                // If fraction is high (> 0.5), we might show a transition character
                // or just the next digit. For TUI, simple floor is safest,
                // or we could use '0' -> '1' cycling.
                // Let's implement strict counting: 0..target.
                let display = std::char::from_digit(current_digit, 10).unwrap_or('0');
                out.push(display);
            } else {
                out.push(c);
            }
        }
        Cow::Owned(out)
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_odometer.rs</FILE> - <DESC>Odometer transformer implementation</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
