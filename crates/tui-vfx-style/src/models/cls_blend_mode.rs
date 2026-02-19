// <FILE>tui-vfx-style/src/models/cls_blend_mode.rs</FILE> - <DESC>Color blend mode operations</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T19:20:00Z</VERS>
// <WCTX>Implementing color management gaps</WCTX>
// <CLOG>Initial implementation of BlendMode with standard blend operations</CLOG>

use crate::utils::fnc_blend_colors::to_rgb_tuple;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

/// Standard color blend modes for compositing effects.
///
/// These modes determine how overlay colors combine with base colors,
/// enabling effects like glow, shadows, and lighting.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum BlendMode {
    /// Replace base with overlay (standard behavior)
    #[default]
    Normal,
    /// Add overlay to base (glow, light sources)
    /// Formula: min(base + overlay, 255)
    Additive,
    /// Multiply base by overlay (shadows, darkening)
    /// Formula: base * overlay / 255
    Multiply,
    /// Inverse of multiply (highlights, lightening)
    /// Formula: 255 - (255 - base) * (255 - overlay) / 255
    Screen,
    /// Combines multiply and screen based on base luminance
    /// Dark areas get darker, light areas get lighter
    Overlay,
    /// Linear interpolation with strength
    /// Formula: base * (1 - strength) + overlay * strength
    Mix,
}

impl BlendMode {
    /// Blend two colors using this blend mode.
    ///
    /// * `base` - The underlying color
    /// * `overlay` - The color to blend on top
    /// * `strength` - Blend intensity (0.0 = no effect, 1.0 = full effect)
    pub fn blend(&self, base: Color, overlay: Color, strength: f32) -> Color {
        let strength = strength.clamp(0.0, 1.0);

        if strength <= 0.0 {
            return base;
        }

        let base_rgb = match to_rgb_tuple(base) {
            Some(rgb) => rgb,
            None => return base,
        };

        let overlay_rgb = match to_rgb_tuple(overlay) {
            Some(rgb) => rgb,
            None => return base,
        };

        let (br, bg, bb) = base_rgb;
        let (or, og, ob) = overlay_rgb;

        let (result_r, result_g, result_b) = match self {
            BlendMode::Normal => {
                // Simple replacement with strength
                (
                    lerp_u8(br, or, strength),
                    lerp_u8(bg, og, strength),
                    lerp_u8(bb, ob, strength),
                )
            }
            BlendMode::Additive => {
                let r = (br as u16 + (or as f32 * strength) as u16).min(255) as u8;
                let g = (bg as u16 + (og as f32 * strength) as u16).min(255) as u8;
                let b = (bb as u16 + (ob as f32 * strength) as u16).min(255) as u8;
                (r, g, b)
            }
            BlendMode::Multiply => {
                let r = multiply_channel(br, or, strength);
                let g = multiply_channel(bg, og, strength);
                let b = multiply_channel(bb, ob, strength);
                (r, g, b)
            }
            BlendMode::Screen => {
                let r = screen_channel(br, or, strength);
                let g = screen_channel(bg, og, strength);
                let b = screen_channel(bb, ob, strength);
                (r, g, b)
            }
            BlendMode::Overlay => {
                let r = overlay_channel(br, or, strength);
                let g = overlay_channel(bg, og, strength);
                let b = overlay_channel(bb, ob, strength);
                (r, g, b)
            }
            BlendMode::Mix => (
                lerp_u8(br, or, strength),
                lerp_u8(bg, og, strength),
                lerp_u8(bb, ob, strength),
            ),
        };

        Color::rgb(result_r, result_g, result_b)
    }
}

/// Linear interpolation for u8 values
fn lerp_u8(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 * (1.0 - t) + b as f32 * t) as u8
}

/// Multiply blend for a single channel
fn multiply_channel(base: u8, overlay: u8, strength: f32) -> u8 {
    let multiplied = (base as u16 * overlay as u16 / 255) as u8;
    lerp_u8(base, multiplied, strength)
}

/// Screen blend for a single channel
fn screen_channel(base: u8, overlay: u8, strength: f32) -> u8 {
    let screened = 255 - ((255 - base as u16) * (255 - overlay as u16) / 255) as u8;
    lerp_u8(base, screened, strength)
}

/// Overlay blend for a single channel (combines multiply and screen)
fn overlay_channel(base: u8, overlay: u8, strength: f32) -> u8 {
    let result = if base < 128 {
        // Dark base: use multiply formula
        let value = 2 * (base as u16 * overlay as u16 / 255);
        value.min(255) as u8
    } else {
        // Light base: use screen formula
        let value = 255_i32 - 2_i32 * ((255 - base as i32) * (255 - overlay as i32) / 255);
        value.clamp(0, 255) as u8
    };
    lerp_u8(base, result, strength)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_normal_blend() {
        let base = Color::rgb(100, 100, 100);
        let overlay = Color::rgb(200, 200, 200);

        // Full strength should be overlay
        let result = BlendMode::Normal.blend(base, overlay, 1.0);
        assert_eq!(result, Color::rgb(200, 200, 200));

        // Zero strength should be base
        let result = BlendMode::Normal.blend(base, overlay, 0.0);
        assert_eq!(result, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_additive_blend() {
        let base = Color::rgb(100, 100, 100);
        let overlay = Color::rgb(100, 100, 100);

        let result = BlendMode::Additive.blend(base, overlay, 1.0);
        assert_eq!(result, Color::rgb(200, 200, 200));

        // Test clamping to 255
        let bright_base = Color::rgb(200, 200, 200);
        let result = BlendMode::Additive.blend(bright_base, overlay, 1.0);
        assert_eq!(result, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_multiply_blend() {
        let base = Color::rgb(255, 255, 255);
        let overlay = Color::rgb(128, 128, 128);

        // White * gray = gray
        let result = BlendMode::Multiply.blend(base, overlay, 1.0);
        assert!(result.r >= 126 && result.r <= 130);

        // Black multiplied stays black
        let black = Color::rgb(0, 0, 0);
        let result = BlendMode::Multiply.blend(black, overlay, 1.0);
        assert_eq!(result, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_screen_blend() {
        let base = Color::rgb(0, 0, 0);
        let overlay = Color::rgb(128, 128, 128);

        // Black screened with gray = gray
        let result = BlendMode::Screen.blend(base, overlay, 1.0);
        assert!(result.r >= 126 && result.r <= 130);

        // White stays white
        let white = Color::rgb(255, 255, 255);
        let result = BlendMode::Screen.blend(white, overlay, 1.0);
        assert_eq!(result, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_strength_parameter() {
        let base = Color::rgb(0, 0, 0);
        let overlay = Color::rgb(200, 200, 200);

        // Half strength additive
        let result = BlendMode::Additive.blend(base, overlay, 0.5);
        assert!(result.r >= 98 && result.r <= 102);
    }
}

// <FILE>tui-vfx-style/src/models/cls_blend_mode.rs</FILE> - <DESC>Color blend mode operations</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T19:20:00Z</VERS>
