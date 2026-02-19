// <FILE>tui-vfx-style/src/utils/fnc_style_blend.rs</FILE> - <DESC>Style color blending utilities</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract style blend helpers from cls_style_effect.rs</WCTX>
// <CLOG>Initial extraction of blend_style_to_color and blend_style_to_color_in_space</CLOG>

use crate::models::ColorSpace;
use crate::utils::blend_colors;
use tui_vfx_types::{Color, Style};

/// Blend a style's foreground and background colors toward a target color.
///
/// Uses RGB color space for blending. Only blends non-transparent colors.
///
/// # Arguments
/// * `style` - The source style to blend from
/// * `target` - The target color to blend toward
/// * `t` - Blend factor (0.0 = original, 1.0 = target)
pub fn blend_style_to_color(style: Style, target: Color, t: f32) -> Style {
    let mut result = style;
    if style.fg != Color::TRANSPARENT {
        result.fg = blend_colors(style.fg, target, t, ColorSpace::Rgb);
    }
    if style.bg != Color::TRANSPARENT {
        result.bg = blend_colors(style.bg, target, t, ColorSpace::Rgb);
    }
    result
}

/// Blend a style's foreground and background colors toward a target color
/// using a specified color space.
///
/// # Arguments
/// * `style` - The source style to blend from
/// * `target` - The target color to blend toward
/// * `t` - Blend factor (0.0 = original, 1.0 = target)
/// * `color_space` - The color space to use for interpolation
pub fn blend_style_to_color_in_space(
    style: Style,
    target: Color,
    t: f32,
    color_space: ColorSpace,
) -> Style {
    let mut result = style;
    if style.fg != Color::TRANSPARENT {
        result.fg = blend_colors(style.fg, target, t, color_space);
    }
    if style.bg != Color::TRANSPARENT {
        result.bg = blend_colors(style.bg, target, t, color_space);
    }
    result
}

// <FILE>tui-vfx-style/src/utils/fnc_style_blend.rs</FILE> - <DESC>Style color blending utilities</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
