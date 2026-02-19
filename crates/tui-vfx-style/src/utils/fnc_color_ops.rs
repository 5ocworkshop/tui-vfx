// <FILE>tui-vfx-style/src/utils/fnc_color_ops.rs</FILE> - <DESC>Color manipulation utilities</DESC>
// <VERS>VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
// <WCTX>Turn 7 Audit Resolution</WCTX>
// <CLOG>Implemented darken and degradation logic</CLOG>

use super::fnc_blend_colors::blend_colors;
use crate::models::ColorSpace;
use tui_vfx_types::Color;
/// Darkens a color by a percentage (0.0 to 1.0).
///
/// * `amount`: 0.0 = no change, 1.0 = fully black.
pub fn darken(color: Color, amount: f32) -> Color {
    // We can reuse blend_colors to blend towards Black
    blend_colors(color, Color::BLACK, amount.clamp(0.0, 1.0), ColorSpace::Rgb)
}
/// degraded an RGB color to the nearest 8-bit (256 color) ANSI index.
///
/// This implements the "Low Fidelity" mode requirement from the PRD.
pub fn rgb_to_indexed(r: u8, g: u8, b: u8) -> u8 {
    // 1. Check standard 16 colors (0-15) - simplified mapping
    // 2. Check grayscale ramp (232-255)
    // 3. Check 6x6x6 color cube (16-231)
    // Simple Euclidean distance approach over the 6x6x6 cube for v1
    // Formula: 16 + 36*r + 6*g + b where r,g,b are 0..5
    let r_idx = ((r as u16 * 5 + 127) / 255) as u8;
    let g_idx = ((g as u16 * 5 + 127) / 255) as u8;
    let b_idx = ((b as u16 * 5 + 127) / 255) as u8;
    16 + 36 * r_idx + 6 * g_idx + b_idx
}
/// Helper to force a color to its indexed approximation.
/// Note: tui_vfx_types::Color doesn't have an Indexed variant, so this
/// returns a color from the 6x6x6 color cube as an approximation.
pub fn degrade_color(color: Color) -> Color {
    if color.a == 0 {
        color // Keep transparent as-is
    } else {
        // Approximate to 6x6x6 color cube values
        let idx = rgb_to_indexed(color.r, color.g, color.b);
        // Convert index back to RGB (6x6x6 cube: 16-231)
        let idx = idx.saturating_sub(16);
        let r_idx = idx / 36;
        let g_idx = (idx % 36) / 6;
        let b_idx = idx % 6;
        // Map 0-5 back to 0,95,135,175,215,255
        let map_val = |v: u8| -> u8 {
            match v {
                0 => 0,
                1 => 95,
                2 => 135,
                3 => 175,
                4 => 215,
                _ => 255,
            }
        };
        Color::rgb(map_val(r_idx), map_val(g_idx), map_val(b_idx))
    }
}

// <FILE>tui-vfx-style/src/utils/fnc_color_ops.rs</FILE> - <DESC>Color manipulation utilities</DESC>
// <VERS>END OF VERSION: 0.1.0 - 2025-12-16T21:01:47Z</VERS>
