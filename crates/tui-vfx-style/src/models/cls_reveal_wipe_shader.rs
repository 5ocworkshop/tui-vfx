// <FILE>tui-vfx-style/src/models/cls_reveal_wipe_shader.rs</FILE> - <DESC>RevealWipe shader: progressively reveals text from one direction</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Region-constrained wipe effect for enter animations</WCTX>
// <CLOG>Initial implementation with direction support</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;

/// Direction for reveal wipe animation.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum RevealDirection {
    #[default]
    LeftToRight,
    RightToLeft,
    TopToBottom,
    BottomToTop,
}

/// RevealWipe shader: progressively reveals text by hiding unrevealed cells.
///
/// Unlike Highlighter which changes colors in the revealed area, RevealWipe
/// hides unrevealed cells by setting their foreground to match the background,
/// making text invisible until the wipe passes.
///
/// This preserves the original styling of revealed content - perfect for
/// "draw-in" effects where text appears progressively.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct RevealWipeShader {
    /// Direction of the reveal animation
    #[serde(default)]
    pub direction: RevealDirection,
}

impl Default for RevealWipeShader {
    fn default() -> Self {
        Self {
            direction: RevealDirection::LeftToRight,
        }
    }
}

impl StyleShader for RevealWipeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let (position, size) = match self.direction {
            RevealDirection::LeftToRight => (ctx.local_x as f64, ctx.width as f64),
            RevealDirection::RightToLeft => {
                let pos = ctx.width.saturating_sub(ctx.local_x + 1) as f64;
                (pos, ctx.width as f64)
            }
            RevealDirection::TopToBottom => (ctx.local_y as f64, ctx.height as f64),
            RevealDirection::BottomToTop => {
                let pos = ctx.height.saturating_sub(ctx.local_y + 1) as f64;
                (pos, ctx.height as f64)
            }
        };

        let limit = size * ctx.t;

        if position < limit {
            // Revealed: keep original styling
            base
        } else {
            // Unrevealed: hide text by matching fg to bg
            let mut style = base;
            style.fg = base.bg;
            style
        }
    }

    fn name(&self) -> &'static str {
        "RevealWipe"
    }
}

// <FILE>tui-vfx-style/src/models/cls_reveal_wipe_shader.rs</FILE> - <DESC>RevealWipe shader: progressively reveals text from one direction</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
