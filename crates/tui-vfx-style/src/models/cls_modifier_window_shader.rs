// <FILE>crates/tui-vfx-style/src/models/cls_modifier_window_shader.rs</FILE> - <DESC>Time-windowed text modifier shader</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Compositor-owned v3.1 style.italicWindow lowering needs a style shader that applies modifiers without backend renderer emulation.</WCTX>
// <CLOG>0.1.0: INIT — add modifier-window shader for normalized timeline modifier application.</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Modifiers, Style};

/// Applies text modifiers only inside a normalized timeline window.
///
/// `start` and `end` are inclusive normalized `phase_t` bounds. Active
/// modifiers are OR-combined with the incoming base style, so this shader never
/// removes modifiers supplied by source content or earlier shader layers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ModifierWindowShader {
    /// Inclusive normalized start time for the modifier window.
    #[config(default = 0.0)]
    pub start: f32,
    /// Inclusive normalized end time for the modifier window.
    #[config(default = 1.0)]
    pub end: f32,
    /// Whether italic styling is applied while the window is active.
    #[serde(default)]
    #[config(default = false)]
    pub italic: bool,
}

impl Default for ModifierWindowShader {
    fn default() -> Self {
        Self {
            start: 0.0,
            end: 1.0,
            italic: false,
        }
    }
}

impl StyleShader for ModifierWindowShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        let t = ctx.t as f32;
        if !(self.start..=self.end).contains(&t) {
            return base;
        }

        let mut modifiers = Modifiers::NONE;
        if self.italic {
            modifiers = modifiers.combine(Modifiers::italic());
        }
        if modifiers.is_empty() {
            return base;
        }
        base.with_mods(base.mods.combine(modifiers))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn italic_applies_inside_window_only() {
        let shader = ModifierWindowShader {
            start: 0.2,
            end: 0.8,
            italic: true,
        };
        let inside = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.5, None, None);
        let outside = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.9, None, None);

        let start_boundary = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.2, None, None);
        let end_boundary = ShaderContext::new(0, 0, 8, 4, 0, 0, 0.8, None, None);

        assert!(shader.style_at(&inside, Style::default()).mods.italic);
        assert!(
            shader
                .style_at(&start_boundary, Style::default())
                .mods
                .italic
        );
        assert!(shader.style_at(&end_boundary, Style::default()).mods.italic);
        assert!(!shader.style_at(&outside, Style::default()).mods.italic);
    }
}

// <FILE>crates/tui-vfx-style/src/models/cls_modifier_window_shader.rs</FILE> - <DESC>Time-windowed text modifier shader</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
