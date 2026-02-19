// <FILE>tui-vfx-compositor/src/filters/cls_greyscale.rs</FILE> - <DESC>Greyscale filter using BT.601 luminance formula</DESC>
// <VERS>VERSION: 1.1.1</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in color math to prevent off-by-one errors</CLOG>

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use tui_vfx_types::{Cell, Color};

/// Greyscale filter that desaturates colors using the BT.601 luminance formula.
///
/// The BT.601 standard weights color channels according to human perception:
/// - Red: 29.9%
/// - Green: 58.7%
/// - Blue: 11.4%
///
/// This is commonly used for modal backdrop "ghost" effects where the
/// background content is desaturated to draw focus to the modal.
pub struct Greyscale {
    /// Strength of the greyscale effect (0.0 = no effect, 1.0 = full greyscale)
    pub strength: f32,
    /// Which color component to affect
    pub apply_to: ApplyTo,
}

impl Default for Greyscale {
    fn default() -> Self {
        Self {
            strength: 1.0,
            apply_to: ApplyTo::Both,
        }
    }
}

impl Greyscale {
    /// Create a new greyscale filter with the given strength.
    #[allow(dead_code)]
    pub fn new(strength: f32) -> Self {
        Self {
            strength,
            apply_to: ApplyTo::Both,
        }
    }

    /// Apply greyscale to foreground only.
    #[allow(dead_code)]
    pub fn fg_only(mut self) -> Self {
        self.apply_to = ApplyTo::Foreground;
        self
    }

    /// Apply greyscale to background only.
    #[allow(dead_code)]
    pub fn bg_only(mut self) -> Self {
        self.apply_to = ApplyTo::Background;
        self
    }

    /// Convert RGB to greyscale using BT.601 luminance coefficients.
    ///
    /// Formula: Y = 0.299*R + 0.587*G + 0.114*B
    #[inline]
    fn to_greyscale(r: u8, g: u8, b: u8) -> u8 {
        (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32).round() as u8
    }

    /// Apply greyscale conversion to a color with strength blending.
    ///
    /// Uses self.strength directly - the caller controls strength animation
    /// via SignalOrFloat, so we don't multiply by t here (that would cause
    /// double-multiplication when strength is already computed from t).
    fn apply_to_color(&self, color: Color) -> Color {
        // Get luminance value
        let grey = Self::to_greyscale(color.r, color.g, color.b);

        // Blend between original and greyscale based on strength
        // Use round() to prevent off-by-one errors at boundary values
        let s = self.strength;
        let r = (color.r as f32 * (1.0 - s) + grey as f32 * s).round() as u8;
        let g = (color.g as f32 * (1.0 - s) + grey as f32 * s).round() as u8;
        let b = (color.b as f32 * (1.0 - s) + grey as f32 * s).round() as u8;

        Color::rgb(r, g, b)
    }
}

impl Filter for Greyscale {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, _t: f64) {
        // Note: we ignore t here - strength is caller-controlled via SignalOrFloat
        match self.apply_to {
            ApplyTo::Foreground => {
                cell.fg = self.apply_to_color(cell.fg);
            }
            ApplyTo::Background => {
                cell.bg = self.apply_to_color(cell.bg);
            }
            ApplyTo::Both => {
                cell.fg = self.apply_to_color(cell.fg);
                cell.bg = self.apply_to_color(cell.bg);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_greyscale_default() {
        let filter = Greyscale::default();
        assert_eq!(filter.strength, 1.0);
        assert_eq!(filter.apply_to, ApplyTo::Both);
    }

    #[test]
    fn test_bt601_luminance_pure_red() {
        // Pure red (255, 0, 0) → ~76 (0.299 * 255 ≈ 76.245)
        let grey = Greyscale::to_greyscale(255, 0, 0);
        assert_eq!(grey, 76);
    }

    #[test]
    fn test_bt601_luminance_pure_green() {
        // Pure green (0, 255, 0) → ~150 (0.587 * 255 ≈ 149.685, rounds to 150)
        let grey = Greyscale::to_greyscale(0, 255, 0);
        assert_eq!(grey, 150);
    }

    #[test]
    fn test_bt601_luminance_pure_blue() {
        // Pure blue (0, 0, 255) → ~29 (0.114 * 255 ≈ 29.07)
        let grey = Greyscale::to_greyscale(0, 0, 255);
        assert_eq!(grey, 29);
    }

    #[test]
    fn test_bt601_luminance_white() {
        // White should remain ~255
        let grey = Greyscale::to_greyscale(255, 255, 255);
        // 0.299*255 + 0.587*255 + 0.114*255 = 254.745 → 254
        assert!(grey >= 254);
    }

    #[test]
    fn test_bt601_luminance_black() {
        // Black should remain 0
        let grey = Greyscale::to_greyscale(0, 0, 0);
        assert_eq!(grey, 0);
    }

    #[test]
    fn test_greyscale_full_strength_red() {
        let filter = Greyscale::new(1.0);
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);
        cell.bg = Color::rgb(255, 255, 255);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // FG should be greyscale (76, 76, 76)
        assert_eq!(cell.fg.r, 76);
        assert_eq!(cell.fg.g, 76);
        assert_eq!(cell.fg.b, 76);
    }

    #[test]
    fn test_greyscale_zero_strength() {
        let filter = Greyscale::new(0.0);
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // Color should be unchanged
        assert_eq!(cell.fg.r, 255);
        assert_eq!(cell.fg.g, 0);
        assert_eq!(cell.fg.b, 0);
    }

    #[test]
    fn test_greyscale_half_strength() {
        let filter = Greyscale::new(0.5);
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // Should be 50% blend: (255 * 0.5 + 76 * 0.5) ≈ 165
        assert!(cell.fg.r >= 164 && cell.fg.r <= 166);
        // g component: (0 * 0.5 + 76 * 0.5) ≈ 38
        assert!(cell.fg.g >= 37 && cell.fg.g <= 39);
    }

    #[test]
    fn test_greyscale_fg_only() {
        let filter = Greyscale::new(1.0).fg_only();
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);
        cell.bg = Color::rgb(0, 255, 0);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // FG should be greyscale
        assert_eq!(cell.fg.r, cell.fg.g);
        assert_eq!(cell.fg.g, cell.fg.b);

        // BG should be unchanged
        assert_eq!(cell.bg.r, 0);
        assert_eq!(cell.bg.g, 255);
        assert_eq!(cell.bg.b, 0);
    }

    #[test]
    fn test_greyscale_bg_only() {
        let filter = Greyscale::new(1.0).bg_only();
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);
        cell.bg = Color::rgb(0, 255, 0);

        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // FG should be unchanged
        assert_eq!(cell.fg.r, 255);
        assert_eq!(cell.fg.g, 0);
        assert_eq!(cell.fg.b, 0);

        // BG should be greyscale
        assert_eq!(cell.bg.r, cell.bg.g);
        assert_eq!(cell.bg.g, cell.bg.b);
    }

    #[test]
    fn test_greyscale_ignores_t_parameter() {
        // Filter strength is caller-controlled via SignalOrFloat,
        // so the t parameter is ignored to avoid double-multiplication
        let filter = Greyscale::new(1.0);
        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 0, 0);

        // Even at t=0.5, full strength should apply because strength is 1.0
        filter.apply(&mut cell, 0, 0, 10, 10, 0.5);

        // Should be fully greyscale (76, 76, 76)
        assert_eq!(cell.fg.r, 76);
        assert_eq!(cell.fg.g, 76);
        assert_eq!(cell.fg.b, 76);
    }

    #[test]
    fn test_greyscale_new_constructor() {
        let filter = Greyscale::new(0.75);
        assert_eq!(filter.strength, 0.75);
        assert_eq!(filter.apply_to, ApplyTo::Both);
    }

    #[test]
    fn test_greyscale_builder_methods() {
        let filter = Greyscale::default().fg_only();
        assert_eq!(filter.apply_to, ApplyTo::Foreground);

        let filter = Greyscale::default().bg_only();
        assert_eq!(filter.apply_to, ApplyTo::Background);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_greyscale.rs</FILE> - <DESC>Greyscale filter using BT.601 luminance formula</DESC>
// <VERS>END OF VERSION: 1.1.1</VERS>
