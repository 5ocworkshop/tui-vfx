// <FILE>tui-vfx-compositor/src/filters/cls_crt.rs</FILE> - <DESC>CRT filter with scanlines and phosphor glow</DESC>
// <VERS>VERSION: 4.0.1</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in color math to prevent off-by-one errors</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// CRT monitor effect with scanlines and phosphor glow.
///
/// Creates alternating scanlines by dimming even rows and optionally
/// boosting odd rows with a phosphor glow effect.
pub struct Crt {
    /// Strength of the scanline dimming (0.0 = no effect, 1.0 = max darkness on scanlines)
    pub scanline_strength: f32,
    /// Phosphor glow amount on lit rows (0.0 = no glow, 1.0 = max brightness boost)
    pub glow: f32,
}

impl Default for Crt {
    fn default() -> Self {
        Self::new(0.3, 0.0)
    }
}

impl Crt {
    /// Create a new CRT filter with given scanline strength and glow.
    pub fn new(scanline_strength: f32, glow: f32) -> Self {
        Self {
            scanline_strength: scanline_strength.clamp(0.0, 1.0),
            glow: glow.clamp(0.0, 1.0),
        }
    }

    /// Dim a color by a factor.
    fn dim_color(c: Color, factor: f32) -> Color {
        Color::rgb(
            (c.r as f32 * factor).round() as u8,
            (c.g as f32 * factor).round() as u8,
            (c.b as f32 * factor).round() as u8,
        )
    }

    /// Boost a color's brightness.
    fn boost_color(c: Color, boost: f32) -> Color {
        Color::rgb(
            (c.r as f32 * boost).round().min(255.0) as u8,
            (c.g as f32 * boost).round().min(255.0) as u8,
            (c.b as f32 * boost).round().min(255.0) as u8,
        )
    }
}

impl Filter for Crt {
    fn apply(&self, cell: &mut Cell, _x: u16, y: u16, _width: u16, _height: u16, _t: f64) {
        if y % 2 == 0 {
            // Even rows: Apply scanline dimming
            let factor = 1.0 - self.scanline_strength;
            cell.fg = Self::dim_color(cell.fg, factor);
            cell.bg = Self::dim_color(cell.bg, factor);
        } else if self.glow > 0.0 {
            // Odd rows: Apply phosphor glow (brightness boost)
            let boost = 1.0 + self.glow * 0.5; // Up to 50% brightness boost
            cell.fg = Self::boost_color(cell.fg, boost);
            cell.bg = Self::boost_color(cell.bg, boost);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    #[test]
    fn test_crt_scanline_dims_even_rows() {
        let crt = Crt::new(1.0, 0.0);
        let mut even = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        crt.apply(&mut even, 0, 0, 10, 10, 0.0); // y=0 is even
        // scanline_strength=1.0 means factor=0.0, full darkness
        assert_eq!(even.fg, Color::rgb(0, 0, 0));
        assert_eq!(even.bg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_crt_glow_boosts_odd_rows() {
        let crt = Crt::new(0.0, 1.0);
        let mut odd = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        crt.apply(&mut odd, 0, 1, 10, 10, 0.0); // y=1 is odd
        // glow=1.0 means boost=1.5, so 100*1.5=150
        assert_eq!(odd.fg, Color::rgb(150, 150, 150));
        assert_eq!(odd.bg, Color::rgb(150, 150, 150));
    }

    #[test]
    fn test_crt_no_glow_odd_row_unchanged() {
        let crt = Crt::new(0.5, 0.0);
        let mut odd = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        crt.apply(&mut odd, 0, 1, 10, 10, 0.0); // y=1 is odd, glow=0
        // No glow applied to odd rows when glow is 0
        assert_eq!(odd.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_crt_partial_scanline() {
        let crt = Crt::new(0.5, 0.0);
        let mut even = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        crt.apply(&mut even, 0, 0, 10, 10, 0.0);
        // factor = 1.0 - 0.5 = 0.5, so 100 * 0.5 = 50
        assert_eq!(even.fg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn test_crt_glow_clamps_to_255() {
        let crt = Crt::new(0.0, 1.0);
        let mut bright = Cell::styled(
            'x',
            Color::rgb(200, 200, 200),
            Color::rgb(200, 200, 200),
            Modifiers::NONE,
        );
        crt.apply(&mut bright, 0, 1, 10, 10, 0.0);
        // boost=1.5, 200*1.5=300, clamped to 255
        assert_eq!(bright.fg, Color::rgb(255, 255, 255));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_crt.rs</FILE> - <DESC>CRT filter with scanlines and phosphor glow</DESC>
// <VERS>END OF VERSION: 4.0.1</VERS>
