// <FILE>tui-vfx-content/src/transformers/cls_glitch_shift.rs</FILE> - <DESC>GlitchShift transformer for brief horizontal offset</DESC>
// <VERS>VERSION: 2.3.0</VERS>
// <WCTX>Packet 69-A: glitch_start/glitch_end are now VfxBindableValue so hosts can drive the glitch window via runtime bindings.</WCTX>
// <CLOG>2.3.0: MINOR — glitch_start, glitch_end fields + GlitchShift::new args + Default impl migrate from SignalOrFloat to VfxBindableValue. Both transform-time evaluate calls now pass ctx.runtime_params. Tests updated to use VfxBindableValue::Literal.</CLOG>

use crate::traits::{TextTransformer, TransformContext};
use std::borrow::Cow;
use tui_vfx_core::bindable::VfxBindableValue;

/// Transformer that briefly shifts text right by prepending spaces.
///
/// During the glitch window (glitch_start to glitch_end progress),
/// prepends `shift_amount` spaces to create a horizontal offset effect.
/// Text will naturally clip at the right border. The glitch window
/// boundaries are bindable so hosts can drive the glitch from app state.
#[derive(Debug, Clone)]
pub struct GlitchShift {
    shift_amount: u8,
    glitch_start: VfxBindableValue,
    glitch_end: VfxBindableValue,
    #[allow(dead_code)]
    seed: u64,
}

impl GlitchShift {
    pub fn new(
        shift_amount: u8,
        glitch_start: VfxBindableValue,
        glitch_end: VfxBindableValue,
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
            glitch_start: VfxBindableValue::Literal(0.3),
            glitch_end: VfxBindableValue::Literal(0.4),
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
        // Check if we're in the glitch window. Window boundaries resolve through
        // the bindable's three-arg evaluate so {"binding": "key"} reaches the
        // host's ShaderRuntimeParams.
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
    use mixed_signals::prelude::SignalContext;
    use tui_vfx_style::traits::ShaderRuntimeParams;

    fn empty_ctx() -> (SignalContext, ShaderRuntimeParams) {
        (SignalContext::default(), ShaderRuntimeParams::new())
    }

    #[test]
    fn test_no_shift_before_window() {
        let glitch = GlitchShift::new(
            5,
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
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
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
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
            VfxBindableValue::Literal(0.3),
            VfxBindableValue::Literal(0.4),
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
            VfxBindableValue::Literal(0.2),
            VfxBindableValue::Literal(0.3),
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
            VfxBindableValue::Literal(0.5),
            VfxBindableValue::Literal(0.6),
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
            VfxBindableValue::Literal(0.5),
            VfxBindableValue::Literal(0.6),
            42,
        );
        // At exactly glitch_end, should NOT shift (condition is < glitch_end)
        let (sig, params) = empty_ctx();
        let result = glitch.transform("text", 0.6, &TransformContext::new(&sig, &params));
        assert_eq!(result, "text");
    }

    #[test]
    fn glitch_window_boundaries_resolve_from_runtime_binding() {
        let glitch = GlitchShift::new(
            5,
            VfxBindableValue::Binding("glitch_start".to_string()),
            VfxBindableValue::Binding("glitch_end".to_string()),
            42,
        );
        let mut params = ShaderRuntimeParams::new();
        params.insert("glitch_start", 0.2_f32);
        params.insert("glitch_end", 0.4_f32);
        let sig = SignalContext::default();
        let result = glitch.transform("hello", 0.3, &TransformContext::new(&sig, &params));
        assert_eq!(result, "     hello", "host-supplied window must apply");
    }
}

// <FILE>tui-vfx-content/src/transformers/cls_glitch_shift.rs</FILE> - <DESC>GlitchShift transformer for brief horizontal offset</DESC>
// <VERS>END OF VERSION: 2.3.0</VERS>
