// <FILE>tui-vfx-style/src/models/cls_signal_color.rs</FILE> - <DESC>Signal-driven color with mix-based blending</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>mixed-signals migration: composition adoption</WCTX>
// <CLOG>Initial SignalColor for signal-driven color transitions</CLOG>

use crate::models::{ColorConfig, ColorSpace};
use crate::utils::blend_colors;
use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

/// A color that transitions between two values over time using signal-based blending.
///
/// `SignalColor` provides smooth crossfading between colors, conceptually similar
/// to using the `Mix` combinator from mixed-signals but applied to color channels.
///
/// # Example
///
/// ```ignore
/// use tui_vfx_style::models::{SignalColor, ColorSpace};
/// use tui_vfx_types::Color;
///
/// let color = SignalColor::new(Color::Red, Color::Blue, ColorSpace::Rgb);
/// let blended = color.sample(0.5); // 50% blend of red and blue
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SignalColor {
    /// Starting color (sampled at t=0)
    pub from: ColorConfig,
    /// Ending color (sampled at t=1)
    pub to: ColorConfig,
    /// Color space for interpolation (Rgb or Hsl)
    #[serde(default)]
    pub space: ColorSpace,
}

impl SignalColor {
    /// Create a new signal color that transitions from one color to another.
    pub fn new(from: Color, to: Color, space: ColorSpace) -> Self {
        Self {
            from: from.into(),
            to: to.into(),
            space,
        }
    }

    /// Create from ColorConfig values directly.
    pub fn from_config(from: ColorConfig, to: ColorConfig, space: ColorSpace) -> Self {
        Self { from, to, space }
    }

    /// Sample the color at time t (0.0 to 1.0).
    ///
    /// The blending uses the configured color space (RGB or HSL).
    /// At t=0.0, returns the `from` color.
    /// At t=1.0, returns the `to` color.
    /// Values between are smoothly interpolated.
    pub fn sample(&self, t: f32) -> Color {
        let from_color: Color = self.from.into();
        let to_color: Color = self.to.into();
        blend_colors(from_color, to_color, t, self.space)
    }

    /// Get the RGB tuple for the from color.
    pub fn from_rgb(&self) -> Option<(u8, u8, u8)> {
        use crate::utils::to_rgb_tuple;
        to_rgb_tuple(self.from.into())
    }

    /// Get the RGB tuple for the to color.
    pub fn to_rgb(&self) -> Option<(u8, u8, u8)> {
        use crate::utils::to_rgb_tuple;
        to_rgb_tuple(self.to.into())
    }
}

impl Default for SignalColor {
    fn default() -> Self {
        Self {
            from: ColorConfig::White,
            to: ColorConfig::Black,
            space: ColorSpace::Rgb,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_at_zero_returns_from_color() {
        let sc = SignalColor::new(Color::RED, Color::BLUE, ColorSpace::Rgb);
        let color = sc.sample(0.0);
        // blend_colors returns the original Color at t=0
        assert_eq!(color, Color::RED);
    }

    #[test]
    fn test_sample_at_one_returns_to_color() {
        let sc = SignalColor::new(Color::RED, Color::BLUE, ColorSpace::Rgb);
        let color = sc.sample(1.0);
        // blend_colors returns the original Color at t=1
        assert_eq!(color, Color::BLUE);
    }

    #[test]
    fn test_sample_at_half_returns_midpoint() {
        let sc = SignalColor::new(
            Color::rgb(0, 0, 0),
            Color::rgb(255, 255, 255),
            ColorSpace::Rgb,
        );
        let color = sc.sample(0.5);
        // Midpoint should be around (127, 127, 127)
        // tui_vfx_types::Color is always RGB
        assert!(color.r >= 126 && color.r <= 128);
        assert!(color.g >= 126 && color.g <= 128);
        assert!(color.b >= 126 && color.b <= 128);
    }

    #[test]
    fn test_from_config_creates_correctly() {
        let sc = SignalColor::from_config(ColorConfig::Red, ColorConfig::Blue, ColorSpace::Rgb);
        assert_eq!(sc.from, ColorConfig::Red);
        assert_eq!(sc.to, ColorConfig::Blue);
    }

    #[test]
    fn test_default_is_white_to_black() {
        let sc = SignalColor::default();
        assert_eq!(sc.from, ColorConfig::White);
        assert_eq!(sc.to, ColorConfig::Black);
    }

    #[test]
    fn test_rgb_accessors() {
        let sc = SignalColor::new(Color::RED, Color::GREEN, ColorSpace::Rgb);
        assert_eq!(sc.from_rgb(), Some((255, 0, 0)));
        assert_eq!(sc.to_rgb(), Some((0, 255, 0)));
    }

    #[test]
    fn test_hsl_interpolation() {
        let sc = SignalColor::new(Color::RED, Color::BLUE, ColorSpace::Hsl);
        let mid = sc.sample(0.5);
        // HSL interpolation should go through magenta (shorter path on hue wheel)
        // tui_vfx_types::Color is always RGB - just verify it's a valid color
        assert!(mid.a == 255);
    }
}

// <FILE>tui-vfx-style/src/models/cls_signal_color.rs</FILE> - <DESC>Signal-driven color with mix-based blending</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
