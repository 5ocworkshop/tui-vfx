// <FILE>tui-vfx-style/src/utils/fnc_style_hsl.rs</FILE> - <DESC>HSL manipulation utilities for styles and colors</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract HSL helpers from cls_style_effect.rs</WCTX>
// <CLOG>Initial extraction of shift_style_hsl and shift_color_hsl</CLOG>

use crate::utils::{hsl_to_rgb, rgb_to_hsl, to_rgb_tuple};
use tui_vfx_types::{Color, Style};

/// Shift the HSL values of a style's foreground and background colors.
///
/// # Arguments
/// * `style` - The source style to modify
/// * `hue_shift` - Hue shift in degrees (-360 to 360)
/// * `sat_shift` - Saturation adjustment (-1.0 to 1.0)
/// * `light_shift` - Lightness adjustment (-1.0 to 1.0)
pub fn shift_style_hsl(style: Style, hue_shift: f32, sat_shift: f32, light_shift: f32) -> Style {
    let mut result = style;
    if style.fg != Color::TRANSPARENT {
        result.fg = shift_color_hsl(style.fg, hue_shift, sat_shift, light_shift);
    }
    if style.bg != Color::TRANSPARENT {
        result.bg = shift_color_hsl(style.bg, hue_shift, sat_shift, light_shift);
    }
    result
}

/// Shift the HSL values of a single color.
///
/// # Arguments
/// * `c` - The source color
/// * `hue_shift` - Hue shift in degrees (wraps around 360)
/// * `sat_shift` - Saturation adjustment (clamped to 0.0-1.0)
/// * `light_shift` - Lightness adjustment (clamped to 0.0-1.0)
pub fn shift_color_hsl(c: Color, hue_shift: f32, sat_shift: f32, light_shift: f32) -> Color {
    if let Some((r, g, b)) = to_rgb_tuple(c) {
        let (h, s, l) = rgb_to_hsl(r, g, b);
        let new_h = (h + hue_shift).rem_euclid(360.0);
        let new_s = (s + sat_shift).clamp(0.0, 1.0);
        let new_l = (l + light_shift).clamp(0.0, 1.0);
        let (nr, ng, nb) = hsl_to_rgb(new_h, new_s, new_l);
        Color::rgb(nr, ng, nb)
    } else {
        c
    }
}

// <FILE>tui-vfx-style/src/utils/fnc_style_hsl.rs</FILE> - <DESC>HSL manipulation utilities for styles and colors</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
