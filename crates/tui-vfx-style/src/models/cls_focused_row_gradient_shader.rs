// <FILE>tui-vfx-style/src/models/cls_focused_row_gradient_shader.rs</FILE> - <DESC>btop-inspired focused row gradient shader</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Adding screen coordinate context to shaders</WCTX>
// <CLOG>Updated to use ShaderContext for screen-space effects</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::traits::{ShaderContext, StyleShader};
use crate::utils::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, Style};

/// Vertical gradient centered on a selected row with symmetrical falloff.
///
/// Creates a btop-style process list highlighting effect where a "selected" row
/// is brightest and rows above/below progressively dim based on distance.
///
/// # Example (Recipe JSON)
///
/// ```json
/// {
///   "spatial_shader": {
///     "type": "focused_row_gradient",
///     "selected_row_ratio": 0.5,
///     "falloff_distance": 5,
///     "bright_color": {"r": 220, "g": 220, "b": 220},
///     "dim_color": {"r": 64, "g": 64, "b": 64},
///     "apply_to": "Foreground"
///   }
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields, rename_all = "snake_case")]
pub struct FocusedRowGradientShader {
    /// Absolute row index to highlight (0-indexed). Takes precedence over selected_row_ratio.
    /// If not set, selected_row_ratio is used instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub selected_row: Option<u16>,

    /// Position of the selected/focused row (0.0 = top, 1.0 = bottom).
    /// Only used if selected_row is not set. Default: 0.5 (middle)
    #[serde(default = "default_selected_row_ratio")]
    pub selected_row_ratio: f32,

    /// Number of rows until full dim (controls gradient spread).
    /// Default: 5
    #[serde(default = "default_falloff_distance")]
    pub falloff_distance: u16,

    /// Color at the selected row (brightest point).
    /// Default: light gray (220, 220, 220)
    #[serde(default = "default_bright_color")]
    pub bright_color: ColorConfig,

    /// Color at maximum distance (dimmest point).
    /// Default: dark gray (64, 64, 64)
    #[serde(default = "default_dim_color")]
    pub dim_color: ColorConfig,

    /// Which style component to apply the gradient to.
    /// Default: Foreground
    #[serde(default)]
    pub apply_to: ApplyToColor,
}

fn default_selected_row_ratio() -> f32 {
    0.5
}

fn default_falloff_distance() -> u16 {
    5
}

fn default_bright_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 220,
        g: 220,
        b: 220,
    }
}

fn default_dim_color() -> ColorConfig {
    ColorConfig::Rgb {
        r: 64,
        g: 64,
        b: 64,
    }
}

/// Which color component of the style to apply the gradient to.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "PascalCase")]
pub enum ApplyToColor {
    /// Apply gradient to foreground color only
    #[default]
    Foreground,
    /// Apply gradient to background color only
    Background,
    /// Apply gradient to both foreground and background
    Both,
}

impl Default for FocusedRowGradientShader {
    fn default() -> Self {
        Self {
            selected_row: None,
            selected_row_ratio: default_selected_row_ratio(),
            falloff_distance: default_falloff_distance(),
            bright_color: default_bright_color(),
            dim_color: default_dim_color(),
            apply_to: ApplyToColor::default(),
        }
    }
}

impl StyleShader for FocusedRowGradientShader {
    fn style_at(&self, ctx: &ShaderContext, base: Style) -> Style {
        // Handle edge case of zero height
        let height = ctx.height.max(1);
        let y = ctx.local_y;

        // Calculate Y position of selected row
        // If selected_row is set, use it directly (clamped to valid range)
        // Otherwise, use selected_row_ratio to calculate position
        let selected_y = if let Some(row) = self.selected_row {
            row.min(height.saturating_sub(1)) as i32
        } else {
            let selected_row_ratio = self.selected_row_ratio.clamp(0.0, 1.0);
            // Use (height - 1) so ratio 1.0 maps to the last row, not beyond
            ((height.saturating_sub(1)) as f32 * selected_row_ratio).round() as i32
        };

        // Calculate distance from current row to selected row
        let distance = (y as i32 - selected_y).unsigned_abs() as u16;

        // Calculate blend factor (0.0 = at selected row, 1.0 = at or beyond falloff)
        let blend_factor = if self.falloff_distance == 0 {
            if distance == 0 { 0.0 } else { 1.0 }
        } else {
            (distance as f32 / self.falloff_distance as f32).min(1.0)
        };

        // Get colors
        let bright: Color = self.bright_color.into();
        let dim: Color = self.dim_color.into();

        // Interpolate: blend_factor 0 = bright, blend_factor 1 = dim
        let color = blend_colors(bright, dim, blend_factor, ColorSpace::Rgb);

        // Apply to appropriate component
        let mut result = base;
        match self.apply_to {
            ApplyToColor::Foreground => {
                result.fg = color;
            }
            ApplyToColor::Background => {
                result.bg = color;
            }
            ApplyToColor::Both => {
                result.fg = color;
                result.bg = color;
            }
        }
        result
    }
}

// <FILE>tui-vfx-style/src/models/cls_focused_row_gradient_shader.rs</FILE> - <DESC>btop-inspired focused row gradient shader</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
