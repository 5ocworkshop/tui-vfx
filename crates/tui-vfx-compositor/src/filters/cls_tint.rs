// <FILE>tui-vfx-compositor/src/filters/cls_tint.rs</FILE> - <DESC>Tint filter with spatial context support</DESC>
// <VERS>VERSION: 3.2.0</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in blend to prevent off-by-one color errors</CLOG>

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use tui_vfx_types::{Cell, Color};

/// Tint filter that blends a color into the cell's fg/bg.
///
/// Used to apply color overlays like warming/cooling effects,
/// damage indicators, or thematic coloring.
pub struct Tint {
    /// The tint color
    pub color: Color,
    /// Strength of the tint (0.0 = no effect, 1.0 = full replacement)
    pub strength: f32,
    /// Which color component to tint
    pub apply_to: ApplyTo,
}

impl Default for Tint {
    fn default() -> Self {
        Self {
            color: Color::RED,
            strength: 0.3,
            apply_to: ApplyTo::Both,
        }
    }
}

impl Tint {
    /// Create a new tint with the given color and strength.
    #[allow(dead_code)]
    pub fn new(color: Color, strength: f32) -> Self {
        Self {
            color,
            strength,
            apply_to: ApplyTo::Both,
        }
    }

    /// Apply tint to foreground only.
    #[allow(dead_code)]
    pub fn fg_only(mut self) -> Self {
        self.apply_to = ApplyTo::Foreground;
        self
    }

    /// Apply tint to background only.
    #[allow(dead_code)]
    pub fn bg_only(mut self) -> Self {
        self.apply_to = ApplyTo::Background;
        self
    }

    /// Blend the tint color into the given color.
    ///
    /// Uses self.strength directly - the caller controls strength animation
    /// via SignalOrFloat, so we don't multiply by t here (that would cause
    /// double-multiplication when strength is already computed from t).
    fn blend(&self, base: Color) -> Color {
        // tui_vfx_types::Color always has RGB components
        let tr = self.color.r;
        let tg = self.color.g;
        let tb = self.color.b;

        let br = base.r;
        let bg = base.g;
        let bb = base.b;

        // Linear blend using self.strength directly
        // Use round() instead of truncation to prevent off-by-one errors
        // at boundary values (e.g., strength ≈ 0.0 causing -1 color shift)
        let s = self.strength;
        let r = (br as f32 * (1.0 - s) + tr as f32 * s).round() as u8;
        let g = (bg as f32 * (1.0 - s) + tg as f32 * s).round() as u8;
        let b = (bb as f32 * (1.0 - s) + tb as f32 * s).round() as u8;

        Color::rgb(r, g, b)
    }
}

impl Filter for Tint {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, _t: f64) {
        // Note: we ignore t here - strength is caller-controlled via SignalOrFloat
        match self.apply_to {
            ApplyTo::Foreground => {
                cell.fg = self.blend(cell.fg);
            }
            ApplyTo::Background => {
                cell.bg = self.blend(cell.bg);
            }
            ApplyTo::Both => {
                cell.fg = self.blend(cell.fg);
                cell.bg = self.blend(cell.bg);
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;

    #[test]
    fn test_tint_default() {
        let tint = Tint::default();
        assert_eq!(tint.strength, 0.3);
        assert_eq!(tint.apply_to, ApplyTo::Both);
    }

    #[test]
    fn test_tint_applies_to_fg() {
        let tint = Tint {
            color: Color::rgb(255, 0, 0),
            strength: 1.0,
            apply_to: ApplyTo::Foreground,
        };

        let mut cell = Cell::default();
        cell.fg = Color::rgb(0, 0, 0);
        cell.bg = Color::rgb(255, 255, 255);

        tint.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // FG should be fully tinted to red
        assert_eq!(cell.fg, Color::rgb(255, 0, 0));
        // BG should be unchanged
        assert_eq!(cell.bg, Color::rgb(255, 255, 255));
    }

    #[test]
    fn test_tint_applies_to_bg() {
        let tint = Tint {
            color: Color::rgb(0, 255, 0),
            strength: 1.0,
            apply_to: ApplyTo::Background,
        };

        let mut cell = Cell::default();
        cell.fg = Color::rgb(255, 255, 255);
        cell.bg = Color::rgb(0, 0, 0);

        tint.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // FG should be unchanged
        assert_eq!(cell.fg, Color::rgb(255, 255, 255));
        // BG should be fully tinted to green
        assert_eq!(cell.bg, Color::rgb(0, 255, 0));
    }

    #[test]
    fn test_tint_partial_strength() {
        let tint = Tint {
            color: Color::rgb(255, 0, 0),
            strength: 0.5,
            apply_to: ApplyTo::Both,
        };

        let mut cell = Cell::default();
        cell.fg = Color::rgb(0, 0, 0);
        cell.bg = Color::rgb(0, 0, 0);

        tint.apply(&mut cell, 0, 0, 10, 10, 1.0);

        // Should be 50% blend (127 or 128 due to rounding)
        assert!(
            cell.fg.r >= 127 && cell.fg.r <= 128,
            "Expected r ~127-128, got {}",
            cell.fg.r
        );
    }

    #[test]
    fn test_tint_new_constructor() {
        let tint = Tint::new(Color::rgb(0, 0, 255), 0.75);
        assert_eq!(tint.color, Color::rgb(0, 0, 255));
        assert_eq!(tint.strength, 0.75);
    }

    #[test]
    fn test_tint_builder_methods() {
        let tint = Tint::default().fg_only();
        assert_eq!(tint.apply_to, ApplyTo::Foreground);

        let tint = Tint::default().bg_only();
        assert_eq!(tint.apply_to, ApplyTo::Background);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_tint.rs</FILE> - <DESC>Tint filter with spatial context support</DESC>
// <VERS>END OF VERSION: 3.2.0</VERS>
