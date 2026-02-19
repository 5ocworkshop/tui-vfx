// <FILE>crates/tui-vfx-types/src/color.rs</FILE> - <DESC>RGBA color type with alpha compositing</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in lerp/blend_over/brighten to prevent off-by-one errors</CLOG>

//! RGBA color type with alpha channel for compositing.

/// RGBA color with alpha channel for compositing.
///
/// Unlike framework-specific color types, this includes an alpha channel
/// to enable proper effect layering and transparency.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Color {
    /// Red channel (0-255)
    pub r: u8,
    /// Green channel (0-255)
    pub g: u8,
    /// Blue channel (0-255)
    pub b: u8,
    /// Alpha channel (0=transparent, 255=opaque)
    pub a: u8,
}

impl Color {
    // Common color constants
    pub const TRANSPARENT: Color = Color::new(0, 0, 0, 0);
    pub const BLACK: Color = Color::new(0, 0, 0, 255);
    pub const WHITE: Color = Color::new(255, 255, 255, 255);
    pub const RED: Color = Color::new(255, 0, 0, 255);
    pub const GREEN: Color = Color::new(0, 255, 0, 255);
    pub const BLUE: Color = Color::new(0, 0, 255, 255);
    pub const YELLOW: Color = Color::new(255, 255, 0, 255);
    pub const CYAN: Color = Color::new(0, 255, 255, 255);
    pub const MAGENTA: Color = Color::new(255, 0, 255, 255);

    /// Create a new color with specified RGBA values.
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    /// Create an opaque color (alpha = 255).
    #[inline]
    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::new(r, g, b, 255)
    }

    /// Create a color with the same value for R, G, B (grayscale).
    #[inline]
    pub const fn gray(value: u8) -> Self {
        Self::rgb(value, value, value)
    }

    /// Create a color with specified alpha, keeping RGB.
    #[inline]
    pub const fn with_alpha(self, a: u8) -> Self {
        Self::new(self.r, self.g, self.b, a)
    }

    /// Convert to RGB tuple (for frameworks without alpha).
    #[inline]
    pub const fn to_rgb(self) -> (u8, u8, u8) {
        (self.r, self.g, self.b)
    }

    /// Linear interpolation between two colors.
    ///
    /// `t` is clamped to [0.0, 1.0].
    #[inline]
    pub fn lerp(self, other: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let inv_t = 1.0 - t;
        // Optimization: (x + 0.5) as u8 is faster than round() and equivalent for positive numbers
        Color::new(
            (self.r as f32 * inv_t + other.r as f32 * t + 0.5) as u8,
            (self.g as f32 * inv_t + other.g as f32 * t + 0.5) as u8,
            (self.b as f32 * inv_t + other.b as f32 * t + 0.5) as u8,
            (self.a as f32 * inv_t + other.a as f32 * t + 0.5) as u8,
        )
    }

    /// Blend this color over a background using alpha compositing.
    ///
    /// Uses the "over" operator: result = fg * fg.a + bg * (1 - fg.a)
    #[inline]
    pub fn blend_over(self, background: Color) -> Color {
        let fg_a = self.a as f32 / 255.0;
        let bg_a = background.a as f32 / 255.0;
        let out_a = fg_a + bg_a * (1.0 - fg_a);

        if out_a == 0.0 {
            return Color::TRANSPARENT;
        }

        let r = (self.r as f32 * fg_a + background.r as f32 * bg_a * (1.0 - fg_a)) / out_a;
        let g = (self.g as f32 * fg_a + background.g as f32 * bg_a * (1.0 - fg_a)) / out_a;
        let b = (self.b as f32 * fg_a + background.b as f32 * bg_a * (1.0 - fg_a)) / out_a;

        // Optimization: (x + 0.5) as u8 is faster than round() and equivalent for positive numbers
        Color::new(
            (r + 0.5) as u8,
            (g + 0.5) as u8,
            (b + 0.5) as u8,
            (out_a * 255.0 + 0.5) as u8,
        )
    }

    /// Brighten the color by a factor (1.0 = unchanged, 2.0 = twice as bright).
    #[inline]
    pub fn brighten(self, factor: f32) -> Color {
        // Optimization: (x + 0.5) as u8 is faster than round() and equivalent for positive numbers
        Color::new(
            ((self.r as f32 * factor).min(255.0) + 0.5) as u8,
            ((self.g as f32 * factor).min(255.0) + 0.5) as u8,
            ((self.b as f32 * factor).min(255.0) + 0.5) as u8,
            self.a,
        )
    }

    /// Dim the color by a factor (1.0 = unchanged, 0.5 = half brightness).
    #[inline]
    pub fn dim(self, factor: f32) -> Color {
        self.brighten(factor)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_constants() {
        assert_eq!(Color::WHITE.r, 255);
        assert_eq!(Color::WHITE.g, 255);
        assert_eq!(Color::WHITE.b, 255);
        assert_eq!(Color::WHITE.a, 255);
        assert_eq!(Color::TRANSPARENT.a, 0);
    }

    #[test]
    fn test_color_lerp() {
        let black = Color::BLACK;
        let white = Color::WHITE;
        // 0 * 0.5 + 255 * 0.5 = 127.5, rounds to 128
        let mid = black.lerp(white, 0.5);
        assert_eq!(mid.r, 128);
        assert_eq!(mid.g, 128);
        assert_eq!(mid.b, 128);
    }

    #[test]
    fn test_color_lerp_clamped() {
        let black = Color::BLACK;
        let white = Color::WHITE;
        assert_eq!(black.lerp(white, -1.0), black);
        assert_eq!(black.lerp(white, 2.0), white);
    }
}

// <FILE>crates/tui-vfx-types/src/color.rs</FILE> - <DESC>RGBA color type with alpha compositing</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
