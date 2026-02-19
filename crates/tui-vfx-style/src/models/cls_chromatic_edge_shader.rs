// <FILE>tui-vfx-style/src/models/cls_chromatic_edge_shader.rs</FILE> - <DESC>Edge-based chromatic aberration approximation</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Integrate ChromaticEdgeShader with spatial shader runtime</WCTX>
// <CLOG>Add StyleShader implementation for ChromaticEdgeShader</CLOG>

use crate::traits::{ShaderContext, StyleShader};
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Chromatic edge shader - approximates chromatic aberration in terminal.
///
/// Since terminals can't display sub-pixel color shifts, this shader
/// simulates chromatic aberration by tinting left edges red and right
/// edges cyan/blue, creating a similar visual impression at text boundaries.
///
/// # Terminal Limitations
/// True chromatic aberration requires offsetting R, G, B channels by
/// fractional pixels - impossible in character-cell terminals where each
/// cell has exactly one color. This shader provides the best approximation
/// by leveraging spatial position to create color gradients at edges.
///
/// # Usage
/// Apply to notifications that need a "glitchy" or "damaged display" aesthetic.
/// Works best with larger text and borders where the edge effect is visible.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct ChromaticEdgeShader {
    /// Red/cyan separation intensity (0.0 = none, 1.0 = full)
    pub intensity: f32,
    /// Width of the edge effect in cells (0.0 = none, 1.0 = half width)
    pub edge_width: f32,
    /// Direction of effect: true = horizontal (left-right), false = vertical (top-bottom)
    #[serde(default = "default_horizontal")]
    pub horizontal: bool,
}

fn default_horizontal() -> bool {
    true
}

impl Default for ChromaticEdgeShader {
    fn default() -> Self {
        Self {
            intensity: 0.3,
            edge_width: 0.15,
            horizontal: true,
        }
    }
}

impl ChromaticEdgeShader {
    /// Create a subtle chromatic edge effect.
    pub fn subtle() -> Self {
        Self {
            intensity: 0.2,
            edge_width: 0.1,
            horizontal: true,
        }
    }

    /// Create a strong chromatic edge effect.
    pub fn strong() -> Self {
        Self {
            intensity: 0.6,
            edge_width: 0.25,
            horizontal: true,
        }
    }

    /// Create a glitchy chromatic effect.
    pub fn glitch() -> Self {
        Self {
            intensity: 0.8,
            edge_width: 0.3,
            horizontal: true,
        }
    }

    /// Calculate the color shift factor at a given position.
    ///
    /// Returns (red_shift, blue_shift) where:
    /// - red_shift > 0 means add red, < 0 means subtract red
    /// - blue_shift > 0 means add blue, < 0 means subtract blue
    ///
    /// # Arguments
    /// * `x` - Horizontal position (0 = left edge, 1 = right edge)
    /// * `y` - Vertical position (0 = top, 1 = bottom)
    /// * `width` - Total width in cells
    /// * `height` - Total height in cells
    pub fn color_shift(&self, x: u16, y: u16, width: u16, height: u16) -> (f32, f32) {
        if self.intensity <= 0.0 || self.edge_width <= 0.0 {
            return (0.0, 0.0);
        }

        let normalized_pos = if self.horizontal {
            if width == 0 {
                0.5
            } else {
                x as f32 / width as f32
            }
        } else if height == 0 {
            0.5
        } else {
            y as f32 / height as f32
        };

        // Calculate distance from edges (0 at edges, 0.5 at center)
        let left_edge_dist = normalized_pos;
        let right_edge_dist = 1.0 - normalized_pos;

        // Calculate effect strength based on distance from edge
        let left_effect = if left_edge_dist < self.edge_width {
            (1.0 - left_edge_dist / self.edge_width) * self.intensity
        } else {
            0.0
        };

        let right_effect = if right_edge_dist < self.edge_width {
            (1.0 - right_edge_dist / self.edge_width) * self.intensity
        } else {
            0.0
        };

        // Left edge gets red, right edge gets cyan (subtract red)
        let red_shift = left_effect - right_effect * 0.5;
        // Right edge gets blue boost
        let blue_shift = right_effect - left_effect * 0.3;

        (red_shift, blue_shift)
    }

    /// Apply the chromatic shift to an RGB color.
    #[allow(clippy::too_many_arguments)]
    pub fn apply_to_color(
        &self,
        r: u8,
        g: u8,
        b: u8,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
    ) -> (u8, u8, u8) {
        let (red_shift, blue_shift) = self.color_shift(x, y, width, height);

        // Apply shifts, clamping to valid range
        let new_r = (r as f32 + red_shift * 100.0).clamp(0.0, 255.0) as u8;
        let new_g =
            (g as f32 - (red_shift.abs() + blue_shift.abs()) * 20.0).clamp(0.0, 255.0) as u8;
        let new_b = (b as f32 + blue_shift * 100.0).clamp(0.0, 255.0) as u8;

        (new_r, new_g, new_b)
    }
}

impl StyleShader for ChromaticEdgeShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        if self.intensity <= 0.0 || self.edge_width <= 0.0 {
            return base;
        }

        let (fg_r, fg_g, fg_b) = self.apply_to_color(
            base.fg.r,
            base.fg.g,
            base.fg.b,
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
        );
        let (bg_r, bg_g, bg_b) = self.apply_to_color(
            base.bg.r,
            base.bg.g,
            base.bg.b,
            ctx.local_x,
            ctx.local_y,
            ctx.width,
            ctx.height,
        );

        let mut style = base;
        style.fg = Color::new(fg_r, fg_g, fg_b, base.fg.a);
        style.bg = Color::new(bg_r, bg_g, bg_b, base.bg.a);
        style
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_creates_valid_shader() {
        let shader = ChromaticEdgeShader::default();
        assert!(shader.intensity > 0.0);
        assert!(shader.edge_width > 0.0);
        assert!(shader.horizontal);
    }

    #[test]
    fn test_center_has_no_effect() {
        let shader = ChromaticEdgeShader::default();
        // At center, should have minimal effect
        let (red, blue) = shader.color_shift(10, 5, 20, 10);
        assert!(red.abs() < 0.01);
        assert!(blue.abs() < 0.01);
    }

    #[test]
    fn test_left_edge_is_red() {
        let shader = ChromaticEdgeShader::strong();
        let (red, _blue) = shader.color_shift(0, 5, 20, 10);
        assert!(red > 0.0, "Left edge should have positive red shift");
    }

    #[test]
    fn test_right_edge_is_blue() {
        let shader = ChromaticEdgeShader::strong();
        let (_red, blue) = shader.color_shift(19, 5, 20, 10);
        assert!(blue > 0.0, "Right edge should have positive blue shift");
    }

    #[test]
    fn test_zero_intensity_no_effect() {
        let shader = ChromaticEdgeShader {
            intensity: 0.0,
            edge_width: 0.3,
            horizontal: true,
        };
        let (red, blue) = shader.color_shift(0, 0, 20, 10);
        assert!((red).abs() < 0.001);
        assert!((blue).abs() < 0.001);
    }

    #[test]
    fn test_color_application() {
        let shader = ChromaticEdgeShader::strong();
        // At left edge, red should increase
        let (r, _g, _b) = shader.apply_to_color(128, 128, 128, 0, 5, 20, 10);
        assert!(r > 128, "Left edge should increase red");

        // At right edge, blue should increase
        let (_r, _g, b) = shader.apply_to_color(128, 128, 128, 19, 5, 20, 10);
        assert!(b > 128, "Right edge should increase blue");
    }
}

// <FILE>tui-vfx-style/src/models/cls_chromatic_edge_shader.rs</FILE> - <DESC>Edge-based chromatic aberration approximation</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
