// <FILE>tui-vfx-style/src/utils/fnc_style_rainbow.rs</FILE> - <DESC>Rainbow color effect utilities</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>OFPF refactoring: extract rainbow helpers from cls_style_effect.rs</WCTX>
// <CLOG>Initial extraction of rainbow_style and rainbow_color</CLOG>

use crate::utils::{hsl_to_rgb, rgb_to_hsl, to_rgb_tuple};
use tui_vfx_types::{Color, Style};

/// Apply rainbow effect to a style - creates fully saturated colors at the given hue.
///
/// Unlike simple hue shifting, this works correctly with achromatic colors (white, gray, black)
/// by forcing full saturation. Only applies to foreground; background is kept stable for readability.
///
/// # Arguments
/// * `style` - The source style
/// * `hue` - The hue angle in degrees (0-360)
pub fn rainbow_style(style: Style, hue: f32) -> Style {
    let mut result = style;
    if style.fg != Color::TRANSPARENT {
        result.fg = rainbow_color(style.fg, hue);
    }
    // Don't apply rainbow to background - keep it stable for readability
    result
}

/// Create a rainbow color at the given hue.
///
/// For achromatic colors (white, gray, black), this creates a fully saturated color
/// at the specified hue. Extreme lightness values (pure white/black) are adjusted
/// to mid-range so the hue is visible.
///
/// # Arguments
/// * `c` - The source color (lightness is preserved unless extreme)
/// * `hue` - The hue angle in degrees (0-360)
pub fn rainbow_color(c: Color, hue: f32) -> Color {
    if let Some((r, g, b)) = to_rgb_tuple(c) {
        let (_h, s, l) = rgb_to_hsl(r, g, b);
        // Use full saturation for rainbow effect on achromatic colors
        let new_s = if s < 0.1 { 1.0 } else { s };
        // Lightness at extremes (near 0.0 or 1.0) makes hue invisible - adjust to mid-range
        let new_l = if !(0.05..=0.95).contains(&l) {
            0.5 // White/black -> mid-brightness colored
        } else {
            l
        };
        let (nr, ng, nb) = hsl_to_rgb(hue, new_s, new_l);
        Color::rgb(nr, ng, nb)
    } else {
        c
    }
}

// <FILE>tui-vfx-style/src/utils/fnc_style_rainbow.rs</FILE> - <DESC>Rainbow color effect utilities</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
