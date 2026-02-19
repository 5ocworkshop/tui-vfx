// <FILE>tui-vfx-style/src/models/cls_gradient_lut.rs</FILE> - <DESC>Pre-computed gradient lookup table for O(1) color access</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Implementing btop-inspired features</WCTX>
// <CLOG>Initial implementation of GradientLUT</CLOG>

use crate::models::ColorSpace;
use crate::utils::fnc_blend_colors::blend_colors;
use tui_vfx_types::Color;

/// Pre-computed gradient lookup table with 101 entries (0-100%).
///
/// Provides O(1) color retrieval for percentage-based gradients,
/// inspired by btop's 101-point color gradient array technique.
///
/// This is a runtime utility, not a configuration type. For serializable
/// gradient configurations, use `Gradient` or `ColorRamp` instead.
///
/// # Example
///
/// ```
/// use tui_vfx_types::Color;
/// use tui_vfx_style::models::GradientLUT;
///
/// let gradient = GradientLUT::new_3_point(
///     Color::rgb(0, 255, 0),    // Green (low)
///     Color::rgb(255, 255, 0),  // Yellow (mid)
///     Color::rgb(255, 0, 0),    // Red (high)
/// );
///
/// let color_at_75_percent = gradient.get(75);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct GradientLUT {
    colors: Vec<Color>,
}

impl GradientLUT {
    /// Create a 2-point gradient (linear interpolation from start to end).
    pub fn new_2_point(start: Color, end: Color) -> Self {
        let mut colors = Vec::with_capacity(101);

        for i in 0..=100 {
            let t = i as f32 / 100.0;
            let color = Self::interpolate_color(start, end, t);
            colors.push(color);
        }

        Self { colors }
    }

    /// Create a 3-point gradient (start → mid at 50% → end at 100%).
    ///
    /// This creates the classic btop-style gradient with a midpoint color.
    pub fn new_3_point(start: Color, mid: Color, end: Color) -> Self {
        let mut colors = Vec::with_capacity(101);

        for i in 0..=100 {
            let color = if i <= 50 {
                // First half: interpolate start → mid
                let t = i as f32 / 50.0;
                Self::interpolate_color(start, mid, t)
            } else {
                // Second half: interpolate mid → end
                let t = (i - 50) as f32 / 50.0;
                Self::interpolate_color(mid, end, t)
            };
            colors.push(color);
        }

        Self { colors }
    }

    /// Get the color at a given percentage (0-100).
    ///
    /// Values above 100 are clamped to 100.
    #[inline]
    pub fn get(&self, percent: u8) -> Color {
        let index = percent.min(100) as usize;
        self.colors[index]
    }

    /// Interpolate between two colors in RGB space.
    fn interpolate_color(c1: Color, c2: Color, t: f32) -> Color {
        // Use the existing blend_colors utility
        blend_colors(c1, c2, t, ColorSpace::Rgb)
    }
}

impl Default for GradientLUT {
    fn default() -> Self {
        // Default to white-to-black gradient
        Self::new_2_point(Color::WHITE, Color::BLACK)
    }
}

// <FILE>tui-vfx-style/src/models/cls_gradient_lut.rs</FILE> - <DESC>Pre-computed gradient lookup table for O(1) color access</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
