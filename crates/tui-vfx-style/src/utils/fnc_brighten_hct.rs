// <FILE>tui-vfx-style/src/utils/fnc_brighten_hct.rs</FILE> - <DESC>Perceptually-uniform brightness scaling via HCT tone (CAM16-based)</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TTE effects port phase 1 — replace the planned HSL-space brighten with HCT-space, using mcu-hct so the result is perceptually uniform and aligned with gt-design's color stack (which already builds on mcu-terminal-color).</WCTX>
// <CLOG>0.1.0: initial brighten_hct — converts to HCT, scales tone by factor (clamped to [0, 100]), converts back. Hue and chroma preserved.</CLOG>

//! Perceptually-uniform brightness scaling.
//!
//! [`brighten_hct`] scales a color's lightness in HCT (Hue, Chroma, Tone)
//! space — the perceptually-uniform color space at the heart of Material
//! Color Utilities. Tone is CIE L*-based, so multiplying it by a factor
//! produces an output whose perceived brightness is proportional to the
//! input.
//!
//! Compared to [`Color::brighten`] (RGB scale), `brighten_hct` preserves
//! hue and saturation through the entire darkening curve. A saturated red
//! at factor 0.3 fades through dark red, not mid-gray.
//!
//! Compared to HSL lightness scaling, `brighten_hct` is perceptually
//! uniform — saturated yellow and saturated blue at the same `factor`
//! end up at the same perceived brightness, which they don't in HSL.
//!
//! Cost: one round-trip through CAM16 per call (~2-5× the cost of an RGB
//! scale). Negligible at one-color-per-frame rates; if used per-cell on a
//! hot path, profile first.

use mcu_hct::Hct;
use mcu_utils::color::{argb_from_rgb, blue_from_argb, green_from_argb, red_from_argb};
use tui_vfx_types::Color;

/// Scale a color's perceptual lightness by `factor`, preserving hue and chroma.
///
/// `factor`:
/// - `1.0` is identity (modulo HCT round-trip drift, typically ≤2 LSB per channel)
/// - `< 1.0` darkens (e.g. `0.3` is the canonical TTE-style faded-text shade)
/// - `> 1.0` brightens; clamped at the L*=100 ceiling
///
/// Alpha is preserved.
///
/// # Examples
///
/// ```
/// use tui_vfx_style::utils::brighten_hct;
/// use tui_vfx_types::Color;
///
/// let red = Color::rgb(255, 0, 0);
/// let dim_red = brighten_hct(red, 0.3);
/// // Red dominance is preserved — dim_red is still red, not gray.
/// assert!(dim_red.r > dim_red.g);
/// assert!(dim_red.r > dim_red.b);
/// ```
pub fn brighten_hct(color: Color, factor: f64) -> Color {
    let argb = argb_from_rgb(color.r, color.g, color.b);
    let hct = Hct::from_int(argb);
    let new_tone = (hct.tone() * factor).clamp(0.0, 100.0);
    let scaled = hct.with_tone(new_tone);
    let new_argb = scaled.to_int();
    Color {
        r: red_from_argb(new_argb),
        g: green_from_argb(new_argb),
        b: blue_from_argb(new_argb),
        a: color.a,
    }
}

// <FILE>tui-vfx-style/src/utils/fnc_brighten_hct.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
