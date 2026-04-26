// <FILE>tui-vfx-content/src/transformers/cls_glitch_shift.rs</FILE> - <DESC>GlitchShift transformer for brief horizontal offset</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>Slice 6.6 of mechanical circular content cycles plan: TextTransformer signature now takes &TransformContext<'_>.</WCTX>
// <CLOG>2.2.0: TextTransformer signature now takes &TransformContext<'_>; reads ctx.signal_ctx for glitch-window signal evaluation.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use mixed_signals::prelude::SignalOrFloat;
use std::borrow::Cow;

/// Transformer that briefly shifts text right by prepending spaces.
///
/// During the glitch window (glitch_start to glitch_end progress),
/// prepends `shift_amount` spaces to create a horizontal offset effect.
/// Text will naturally clip at the right border.
#[derive(Debug, Clone)]
pub struct GlitchShift {
    shift_amount: u8,
    glitch_start: SignalOrFloat,
    glitch_end: SignalOrFloat,
    #[allow(dead_code)]
    seed: u64,
}

impl GlitchShift {
    pub fn new(
        shift_amount: u8,
        glitch_start: SignalOrFloat,
        glitch_end: SignalOrFloat,
        seed: u64,
    ) -> Self {
        Self {
            shift_amount,
            glitch_start,
            glitch_end,
            seed,
        }
    }
}

impl Default for GlitchShift {
    fn default() -> Self {
        Self {
            shift_amount: 5,
            glitch_start: SignalOrFloat::Static(0.3),
            glitch_end: SignalOrFloat::Static(0.4),
            seed: 0,
        }
    }
}

impl TextTransformer for GlitchShift {
    fn transform<'a>(
        &self,
        target: &'a str,
        progress: f64,
        ctx: &TransformContext<'_>,
    ) -> Cow<'a, str> {
        // Check if we're in the glitch window
        let progress_f32 = progress as f32;
        let glitch_start = self
            .glitch_start
            .evaluate(progress, ctx.signal_ctx)
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        let glitch_end = self
            .glitch_end
            .evaluate(progress, ctx.signal_ctx)
            .unwrap_or(0.0)
            .clamp(0.0, 1.0);
        if progress_f32 >= glitch_start && progress_f32 < glitch_end {
            // Prepend spaces to shift text right
            let spaces: String = " ".repeat(self.shift_amount as usize);
            Cow::Owned(format!("{}{}", spaces, target))
        } else {
            // Outside glitch window - return text unchanged
            Cow::Borrowed(target)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mixed_signals::prelude::{SignalContext, SignalOrFloat};
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn empty_ctx() -> (SignalContext, ShaderRuntimeParams) {
        (SignalContext::default(), ShaderRuntimeParams::new())
    }

    #[test]
    fn test_no_shift_before_window() {
        let glitch = GlitchShift::new(
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            42,
        );
        let (sig, params) = empty_ctx();
        let result = glitch.transform("hello", 0.1, &TransformContext::new(&sig, &params));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_shift_during_window() {
        let glitch = GlitchShift::new(
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            42,
        );
        let (sig, params) = empty_ctx();
        let result = glitch.transform("hello", 0.35, &TransformContext::new(&sig, &params));
        assert_eq!(result, "     hello");
    }

    #[test]
    fn test_no_shift_after_window() {
        let glitch = GlitchShift::new(
            5,
            SignalOrFloat::Static(0.3),
            SignalOrFloat::Static(0.4),
            42,
        );
        let (sig, params) = empty_ctx();
        let result = glitch.transform("hello", 0.5, &TransformContext::new(&sig, &params));
        assert_eq!(result, "hello");
    }

    #[test]
    fn test_shift_amount_configurable() {
        let glitch = GlitchShift::new(
            3,
            SignalOrFloat::Static(0.2),
            SignalOrFloat::Static(0.3),
            42,
        );
        let (sig, params) = empty_ctx();
        let result = glitch.transform("test", 0.25, &TransformContext::new(&sig, &params));
        assert_eq!(result, "   test");
    }

    #[test]
    fn test_at_window_start() {
        let glitch = GlitchShift::new(
            4,
            SignalOrFloat::Static(0.5),
            SignalOrFloat::Static(0.6),
            42,
        );
        let (sig, params) = empty_ctx();
        let result = glitch.transform("text", 0.5, &TransformContext::new(&sig, &params));
        assert_eq!(result, "    text");
    }

    #[test]
    fn test_at_window_end_boundary() {
        let glitch = GlitchShift::new(
            4,
            SignalOrFloat::Static(0.5),
            SignalOrFloat::Static(0.6),
            42,
        );
        // At exactly glitch_end, should NOT shift (condition is < glitch_end)
        let (sig, params) = empty_ctx();
        let result = glitch.transform("text", 0.6, &TransformContext::new(&sig, &params));
        assert_eq!(result, "text");
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_glitch_shift.rs</FILE> - <DESC>GlitchShift transformer for brief horizontal offset</DESC>
// <VERS>END OF VERSION: 2.2.0</VERS>
