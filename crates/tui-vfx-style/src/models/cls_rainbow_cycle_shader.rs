// <FILE>crates/tui-vfx-style/src/models/cls_rainbow_cycle_shader.rs</FILE> - <DESC>Timeline-driven rainbow foreground shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-owned v3.1 style.rainbow lowering needs hue rotation as a style shader instead of backend renderer emulation.</WCTX>
// <CLOG>0.1.0: INIT — add rainbow-cycle shader for V2-compatible foreground hue rotation.</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use crate::utils::rainbow_style;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Style;

/// Applies V2-compatible rainbow hue rotation to the foreground channel.
///
/// The shader preserves background color for readability and derives hue from
/// normalized timeline `phase_t` (`ctx.t`) multiplied by `rotation_speed`,
/// matching `StyleEffect::Rainbow` semantics without player/backend stages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct RainbowCycleShader {
    /// Hue rotations per normalized timeline cycle.
    #[config(default = 1.0)]
    pub rotation_speed: f32,
}

impl Default for RainbowCycleShader {
    fn default() -> Self {
        Self {
            rotation_speed: 1.0,
        }
    }
}

impl StyleShader for RainbowCycleShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let hue = ((ctx.t as f32) * self.rotation_speed * 360.0).rem_euclid(360.0);
        rainbow_style(base, hue)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Color;

    fn ctx(t: f64) -> ShaderContext {
        ShaderContext::new(0, 0, 8, 4, 0, 0, t, None, None)
    }

    #[test]
    fn rainbow_cycles_foreground_and_preserves_background() {
        let shader = RainbowCycleShader {
            rotation_speed: 1.0,
        };
        let base = Style {
            fg: Color::WHITE,
            bg: Color::rgb(30, 30, 30),
            ..Style::default()
        };

        let start = shader.style_at(&ctx(0.0), base);
        let halfway = shader.style_at(&ctx(0.5), base);

        assert_eq!(start.bg, base.bg);
        assert_eq!(halfway.bg, base.bg);
        assert_ne!(start.fg, base.fg);
        assert_ne!(halfway.fg, base.fg);
        assert_ne!(start.fg, halfway.fg);
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_rainbow_cycle_shader.rs</FILE> - <DESC>Timeline-driven rainbow foreground shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
