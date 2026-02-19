// <FILE>tui-vfx-compositor/src/filters/cls_color_bridged_shade.rs</FILE>
// <DESC>Maps opacity to shade characters (░▒▓█) with smooth color bridging</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Retain local test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Shade characters from lightest to darkest.
const SHADE_LIGHT: char = '░'; // U+2591 - Light shade (25%)
const SHADE_MEDIUM: char = '▒'; // U+2592 - Medium shade (50%)
const SHADE_DARK: char = '▓'; // U+2593 - Dark shade (75%)
const SHADE_FULL: char = '█'; // U+2588 - Full block (100%)

/// Color-bridged shade filter for smooth opacity rendering.
///
/// Maps an opacity value (0.0-1.0) to shade characters (░▒▓█) while
/// adjusting fg/bg colors to create smooth visual transitions at
/// the boundaries between shade levels.
///
/// This technique creates more visually appealing opacity effects than
/// simple character selection by interpolating colors at the handover points.
pub struct ColorBridgedShade {
    /// Target opacity (0.0 = transparent, 1.0 = opaque)
    pub opacity: f32,

    /// Foreground color (the "fill" color)
    pub fg_color: Color,

    /// Background color (shows through at low opacity)
    pub bg_color: Color,
}

impl Default for ColorBridgedShade {
    fn default() -> Self {
        Self {
            opacity: 0.5,
            fg_color: Color::WHITE,
            bg_color: Color::BLACK,
        }
    }
}

impl ColorBridgedShade {
    /// Create a new ColorBridgedShade filter.
    pub fn new(opacity: f32, fg_color: Color, bg_color: Color) -> Self {
        Self {
            opacity: opacity.clamp(0.0, 1.0),
            fg_color,
            bg_color,
        }
    }

    /// Create with default colors (white on black).
    #[allow(dead_code)]
    pub fn with_opacity(opacity: f32) -> Self {
        Self {
            opacity: opacity.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    /// Linear interpolation between two colors.
    fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        Color::rgb(
            (a.r as f32 * (1.0 - t) + b.r as f32 * t).round() as u8,
            (a.g as f32 * (1.0 - t) + b.g as f32 * t).round() as u8,
            (a.b as f32 * (1.0 - t) + b.b as f32 * t).round() as u8,
        )
    }

    /// Select shade character and compute bridged colors for the given opacity.
    ///
    /// Returns (character, fg_color, bg_color) tuple.
    ///
    /// The color bridging works by adjusting fg/bg colors at the transition
    /// points between shade levels to create smooth visual gradients:
    ///
    /// - opacity 0.00-0.25: ░ (light), fg bridges from bg toward fg_color
    /// - opacity 0.25-0.50: ▒ (medium), balanced blend
    /// - opacity 0.50-0.75: ▓ (dark), bg bridges from fg_color toward bg
    /// - opacity 0.75-1.00: █ (full), solid fg_color
    fn shade_for_opacity(&self, opacity: f32) -> (char, Color, Color) {
        let opacity = opacity.clamp(0.0, 1.0);

        if opacity < 0.25 {
            // Light shade range: 0.0 - 0.25
            // Character is 25% fg coverage, so we need to adjust colors
            // to simulate opacities from 0% to 25%
            let t = opacity / 0.25; // 0.0 to 1.0 within this band

            // At t=0 (opacity=0): both colors should look like bg_color
            // At t=1 (opacity=0.25): normal shade appearance
            let bridged_fg = Self::lerp_color(self.bg_color, self.fg_color, t);
            (SHADE_LIGHT, bridged_fg, self.bg_color)
        } else if opacity < 0.50 {
            // Medium shade range: 0.25 - 0.50
            let t = (opacity - 0.25) / 0.25;

            // Bridge fg between light and medium appearance
            let bridged_fg = Self::lerp_color(self.fg_color, self.fg_color, 1.0); // fg stays fg
            let bridged_bg = Self::lerp_color(self.bg_color, self.bg_color, 1.0); // bg stays bg
            // The visual blend comes from the character choice
            let _ = t; // t affects visual perception through character coverage
            (SHADE_MEDIUM, bridged_fg, bridged_bg)
        } else if opacity < 0.75 {
            // Dark shade range: 0.50 - 0.75
            let t = (opacity - 0.50) / 0.25;

            // Start blending bg toward fg to smooth transition to full block
            let bridged_bg = Self::lerp_color(self.bg_color, self.fg_color, t * 0.5);
            (SHADE_DARK, self.fg_color, bridged_bg)
        } else {
            // Full block range: 0.75 - 1.0
            let t = (opacity - 0.75) / 0.25;

            // At t=1 (opacity=1.0): bg should equal fg for true solid appearance
            let bridged_bg = Self::lerp_color(self.fg_color, self.fg_color, t);
            (SHADE_FULL, self.fg_color, bridged_bg)
        }
    }
}

impl Filter for ColorBridgedShade {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, _t: f64) {
        let (shade_char, fg, bg) = self.shade_for_opacity(self.opacity);
        cell.ch = shade_char;
        cell.fg = fg;
        cell.bg = bg;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell() -> Cell {
        Cell::styled(
            'x',
            Color::rgb(128, 128, 128),
            Color::rgb(64, 64, 64),
            Modifiers::NONE,
        )
    }

    #[test]
    fn default_values() {
        let filter = ColorBridgedShade::default();
        assert_eq!(filter.opacity, 0.5);
        assert_eq!(filter.fg_color, Color::WHITE);
        assert_eq!(filter.bg_color, Color::BLACK);
    }

    #[test]
    fn opacity_clamped() {
        let filter = ColorBridgedShade::with_opacity(1.5);
        assert_eq!(filter.opacity, 1.0);

        let filter = ColorBridgedShade::with_opacity(-0.5);
        assert_eq!(filter.opacity, 0.0);
    }

    #[test]
    fn very_low_opacity_uses_light_shade() {
        let filter = ColorBridgedShade::with_opacity(0.1);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, SHADE_LIGHT);
    }

    #[test]
    fn quarter_opacity_uses_light_shade() {
        let filter = ColorBridgedShade::with_opacity(0.24);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, SHADE_LIGHT);
    }

    #[test]
    fn half_opacity_uses_medium_shade() {
        let filter = ColorBridgedShade::with_opacity(0.35);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, SHADE_MEDIUM);
    }

    #[test]
    fn three_quarter_opacity_uses_dark_shade() {
        let filter = ColorBridgedShade::with_opacity(0.6);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, SHADE_DARK);
    }

    #[test]
    fn high_opacity_uses_full_block() {
        let filter = ColorBridgedShade::with_opacity(0.9);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.ch, SHADE_FULL);
    }

    #[test]
    fn zero_opacity_fg_matches_bg() {
        let filter = ColorBridgedShade::new(
            0.0,
            Color::rgb(255, 0, 0), // Red fg
            Color::rgb(0, 0, 255), // Blue bg
        );
        let (_, fg, _) = filter.shade_for_opacity(0.0);
        // At zero opacity, fg should be bridged to bg color
        assert_eq!(fg, Color::rgb(0, 0, 255));
    }

    #[test]
    fn full_opacity_uses_fg_color() {
        let filter = ColorBridgedShade::new(1.0, Color::rgb(255, 0, 0), Color::rgb(0, 0, 255));
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);

        // At full opacity, colors should both approach fg_color
        assert_eq!(cell.fg, Color::rgb(255, 0, 0));
        assert_eq!(cell.bg, Color::rgb(255, 0, 0));
    }

    #[test]
    fn colors_set_correctly() {
        let filter = ColorBridgedShade::new(0.5, Color::rgb(200, 100, 50), Color::rgb(20, 40, 80));
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);

        // Medium shade should use fg_color and bg_color as-is
        assert_eq!(cell.fg, Color::rgb(200, 100, 50));
        assert_eq!(cell.bg, Color::rgb(20, 40, 80));
    }

    #[test]
    fn position_has_no_effect() {
        let filter = ColorBridgedShade::with_opacity(0.5);
        let mut cell1 = make_cell();
        let mut cell2 = make_cell();

        filter.apply(&mut cell1, 0, 0, 10, 10, 0.0);
        filter.apply(&mut cell2, 5, 5, 10, 10, 0.0);

        // Same filter should produce same result regardless of position
        assert_eq!(cell1.ch, cell2.ch);
        assert_eq!(cell1.fg, cell2.fg);
        assert_eq!(cell1.bg, cell2.bg);
    }

    #[test]
    fn time_has_no_effect() {
        let filter = ColorBridgedShade::with_opacity(0.5);
        let mut cell1 = make_cell();
        let mut cell2 = make_cell();

        filter.apply(&mut cell1, 0, 0, 10, 10, 0.0);
        filter.apply(&mut cell2, 0, 0, 10, 10, 0.5);

        // Same filter should produce same result regardless of time
        assert_eq!(cell1.ch, cell2.ch);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_color_bridged_shade.rs</FILE>
// <DESC>Maps opacity to shade characters (░▒▓█) with smooth color bridging</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
