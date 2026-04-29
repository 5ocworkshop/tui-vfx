// <FILE>crates/tui-vfx-player/src/cls_resolved_color.rs</FILE> - <DESC>Resolved color helper for player primitive adapters</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Player primitive adapters: share RGB and HCT color interpolation.</WCTX>
// <CLOG>0.1.0: INIT — add resolved color helpers backed by mcu-hct.</CLOG>

use mcu_hct::Hct;
use mcu_utils::color::{argb_from_rgb, blue_from_argb, green_from_argb, red_from_argb};

/// RGBA color resolved from a canonical effect input.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct ResolvedColor {
    pub(crate) r: u8,
    pub(crate) g: u8,
    pub(crate) b: u8,
    pub(crate) a: u8,
}

impl ResolvedColor {
    pub(crate) const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Build an opaque RGB resolved color.
    pub(crate) const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Linear interpolation between two colors in RGB space.
    pub(crate) fn lerp(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        Self::new(
            lerp_channel(self.r, other.r, inv_t, t),
            lerp_channel(self.g, other.g, inv_t, t),
            lerp_channel(self.b, other.b, inv_t, t),
            lerp_channel(self.a, other.a, inv_t, t),
        )
    }

    /// Interpolate in the named descriptor color space.
    pub(crate) fn lerp_in_color_space(self, other: Self, t: f32, color_space: &str) -> Self {
        if color_space.eq_ignore_ascii_case("hct") {
            self.lerp_hct(other, t)
        } else {
            self.lerp(other, t)
        }
    }

    /// Format this color as the report-stable RGBA label.
    pub(crate) fn rgba_label(self) -> String {
        format!("rgba({},{},{},{})", self.r, self.g, self.b, self.a)
    }

    fn lerp_hct(self, other: Self, t: f32) -> Self {
        let t = t.clamp(0.0, 1.0) as f64;
        let start = Hct::from_int(argb_from_rgb(self.r, self.g, self.b));
        let end = Hct::from_int(argb_from_rgb(other.r, other.g, other.b));
        let hct = Hct::from(
            lerp_hue(start.hue(), end.hue(), t),
            lerp_f64(start.chroma(), end.chroma(), t),
            lerp_f64(start.tone(), end.tone(), t),
        );
        let argb = hct.to_int();
        Self::new(
            red_from_argb(argb),
            green_from_argb(argb),
            blue_from_argb(argb),
            lerp_channel(self.a, other.a, 1.0 - t as f32, t as f32),
        )
    }
}

fn lerp_hue(start: f64, end: f64, t: f64) -> f64 {
    let delta = ((end - start + 540.0) % 360.0) - 180.0;
    (start + delta * t).rem_euclid(360.0)
}

fn lerp_f64(start: f64, end: f64, t: f64) -> f64 {
    start + (end - start) * t
}

fn lerp_channel(start: u8, end: u8, inv_t: f32, t: f32) -> u8 {
    (start as f32 * inv_t + end as f32 * t + 0.5) as u8
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fnc_hct_color_interpolation_differs_from_rgb_midpoint() {
        let start = ResolvedColor::rgb(255, 0, 0);
        let end = ResolvedColor::rgb(0, 255, 255);

        let rgb_midpoint = start.lerp(end, 0.5);
        let hct_midpoint = start.lerp_in_color_space(end, 0.5, "hct");

        assert_ne!(hct_midpoint, rgb_midpoint);
    }
}

// <FILE>crates/tui-vfx-player/src/cls_resolved_color.rs</FILE> - <DESC>Resolved color helper for player primitive adapters</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
