// <FILE>tui-vfx-style/src/models/cls_color_space.rs</FILE> - <DESC>Enum for color interpolation modes — RGB, HSL, and HCT (perceptually uniform)</DESC>
// <VERS>VERSION: 1.1.0 - 2026-04-26</VERS>
// <WCTX>TTE effects port — extend the existing ColorSpace enum with an Hct variant so gradient interpolation and brightness scaling can use Material Color's perceptually-uniform tone space without changing the existing Rgb/Hsl call sites.</WCTX>
// <CLOG>1.1.0: add ColorSpace::Hct (additive); existing Rgb/Hsl call sites unchanged. Hct routes through mcu-hct (CAM16-based perceptual color space) so mid-stops in gradient interpolation and brightness scaling stay perceptually evenly spaced.</CLOG>

use serde::{Deserialize, Serialize};

/// Color space used for interpolation and brightness operations.
///
/// `Rgb` is the historical default — fast per-channel arithmetic,
/// matches CSS gradients, but mid-stops desaturate when interpolating
/// between distinct hues (e.g. red→blue passes through gray).
///
/// `Hsl` interpolates lightness, saturation, and hue independently;
/// hue takes the shortest path around the wheel. Better hue identity
/// than RGB but mathematical lightness is not perceptually linear
/// (saturated yellow at L=0.5 looks brighter than saturated blue at
/// L=0.5).
///
/// `Hct` (Hue, Chroma, Tone) is perceptually uniform — built on CAM16
/// and CIE L*. Mid-stops are evenly spaced in perceived brightness,
/// hue is a true perceptual angle, and "scale brightness by 0.3"
/// produces a result that *looks* 30% as bright. Recommended for
/// design-system color work; ~2-5× the conversion cost of RGB but
/// negligible at typical gradient/interpolation rates. Routes through
/// the `mcu-hct` crate (Material Color Utilities port).
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ColorSpace {
    #[default]
    Rgb,
    Hsl,
    /// Perceptually-uniform Hue/Chroma/Tone (CAM16-based).
    /// See module-level docs for trade-offs.
    Hct,
}

// <FILE>tui-vfx-style/src/models/cls_color_space.rs</FILE>
// <VERS>END OF VERSION: 1.1.0 - 2026-04-26</VERS>
