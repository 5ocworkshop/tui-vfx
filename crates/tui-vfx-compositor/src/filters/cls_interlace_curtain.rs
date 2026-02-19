// <FILE>tui-vfx-compositor/src/filters/cls_interlace_curtain.rs</FILE>
// <DESC>Scanline/interlace effect that dims alternating rows for backdrop effects</DESC>
// <VERS>VERSION: 1.1.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Retain local test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Scanline/interlace effect filter for backdrop dimming.
///
/// Creates horizontal stripe patterns by dimming rows at regular intervals.
/// Useful for CRT-style effects, subtle texture, and visual depth separation.
pub struct InterlaceCurtain {
    /// Controls row spacing. 1.0 = every other row, 0.5 = every 2nd row, 2.0 = every row
    /// Range: 0.1 to 2.0 (clamped)
    pub density: f32,

    /// Dimming factor applied to affected rows (0.0 = no dimming, 1.0 = black)
    pub dim_factor: f32,

    /// Scroll offset in fractional rows (wraps around). Animated via `t` parameter.
    /// At scroll_speed=1.0, one full row scrolls per second.
    pub scroll_speed: f32,
}

impl Default for InterlaceCurtain {
    fn default() -> Self {
        Self {
            density: 1.0,      // Every other row
            dim_factor: 0.3,   // 30% dimming
            scroll_speed: 0.0, // Static by default
        }
    }
}

impl InterlaceCurtain {
    /// Create a new InterlaceCurtain with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the row density (how frequently rows are dimmed).
    pub fn with_density(mut self, density: f32) -> Self {
        self.density = density.clamp(0.1, 2.0);
        self
    }

    /// Set the dimming factor for affected rows.
    pub fn with_dim_factor(mut self, dim_factor: f32) -> Self {
        self.dim_factor = dim_factor.clamp(0.0, 1.0);
        self
    }

    /// Set the scroll speed for animation.
    pub fn with_scroll_speed(mut self, scroll_speed: f32) -> Self {
        self.scroll_speed = scroll_speed;
        self
    }

    /// Determine if a row should be dimmed based on position and animation time.
    fn should_dim_row(&self, y: u16, t: f64) -> bool {
        // Calculate period based on density first
        // density=1.0 -> period=2 (every other row)
        // density=0.5 -> period=4 (every 4th row)
        // density=2.0 -> period=1 (every row)
        let period = (2.0 / self.density.clamp(0.1, 2.0)).max(1.0);

        // Calculate scroll offset that cycles through the full period
        // This ensures the pattern visibly shifts - using .fract() only cycled 0-1
        // which never crossed row boundaries when period > 1
        let scroll_offset = (t as f32 * self.scroll_speed) % period;
        let effective_y = y as f32 + scroll_offset;

        // Dim when position modulo period is in the first half
        let position_in_period = effective_y % period;
        position_in_period < (period / 2.0)
    }

    /// Apply dimming to a color.
    fn dim_color(&self, c: Color) -> Color {
        let factor = 1.0 - self.dim_factor;
        Color::rgb(
            (c.r as f32 * factor).round() as u8,
            (c.g as f32 * factor).round() as u8,
            (c.b as f32 * factor).round() as u8,
        )
    }
}

impl Filter for InterlaceCurtain {
    fn apply(&self, cell: &mut Cell, _x: u16, y: u16, _width: u16, _height: u16, t: f64) {
        if self.should_dim_row(y, t) {
            cell.fg = self.dim_color(cell.fg);
            cell.bg = self.dim_color(cell.bg);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell() -> Cell {
        Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 50, 50),
            Modifiers::NONE,
        )
    }

    #[test]
    fn default_values() {
        let filter = InterlaceCurtain::default();
        assert_eq!(filter.density, 1.0);
        assert_eq!(filter.dim_factor, 0.3);
        assert_eq!(filter.scroll_speed, 0.0);
    }

    #[test]
    fn builder_pattern() {
        let filter = InterlaceCurtain::new()
            .with_density(0.5)
            .with_dim_factor(0.5)
            .with_scroll_speed(2.0);
        assert_eq!(filter.density, 0.5);
        assert_eq!(filter.dim_factor, 0.5);
        assert_eq!(filter.scroll_speed, 2.0);
    }

    #[test]
    fn density_clamped() {
        let filter = InterlaceCurtain::new().with_density(5.0);
        assert_eq!(filter.density, 2.0);

        let filter = InterlaceCurtain::new().with_density(0.01);
        assert_eq!(filter.density, 0.1);
    }

    #[test]
    fn dim_factor_clamped() {
        let filter = InterlaceCurtain::new().with_dim_factor(2.0);
        assert_eq!(filter.dim_factor, 1.0);

        let filter = InterlaceCurtain::new().with_dim_factor(-0.5);
        assert_eq!(filter.dim_factor, 0.0);
    }

    #[test]
    fn alternating_rows_with_density_1() {
        let filter = InterlaceCurtain::new()
            .with_density(1.0)
            .with_dim_factor(0.5);

        // Row 0 should be dimmed
        let mut cell0 = make_cell();
        filter.apply(&mut cell0, 0, 0, 10, 10, 0.0);
        assert_eq!(cell0.fg, Color::rgb(50, 50, 50));
        assert_eq!(cell0.bg, Color::rgb(25, 25, 25));

        // Row 1 should NOT be dimmed
        let mut cell1 = make_cell();
        filter.apply(&mut cell1, 0, 1, 10, 10, 0.0);
        assert_eq!(cell1.fg, Color::rgb(100, 100, 100));
        assert_eq!(cell1.bg, Color::rgb(50, 50, 50));

        // Row 2 should be dimmed
        let mut cell2 = make_cell();
        filter.apply(&mut cell2, 0, 2, 10, 10, 0.0);
        assert_eq!(cell2.fg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn scroll_shifts_pattern() {
        let filter = InterlaceCurtain::new()
            .with_density(1.0)
            .with_dim_factor(1.0)
            .with_scroll_speed(1.0);

        // At t=0, row 0 is dimmed (scroll_offset=0)
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(0, 0, 0)); // Fully dimmed

        // At t=1.0, pattern shifts by 1 row (scroll_offset = 1.0 % 2.0 = 1.0)
        // Row 0: effective_y = 0 + 1 = 1, 1 % 2 = 1, 1 < 1 = false → NOT dimmed
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 1.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100)); // Not dimmed

        // At t=1.0, row 1 should be dimmed (swapped with row 0)
        // Row 1: effective_y = 1 + 1 = 2, 2 % 2 = 0, 0 < 1 = true → dimmed
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 1, 10, 10, 1.0);
        assert_eq!(cell.fg, Color::rgb(0, 0, 0)); // Dimmed

        // At t=2.0, pattern cycles back (scroll_offset = 2.0 % 2.0 = 0)
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 2.0);
        assert_eq!(cell.fg, Color::rgb(0, 0, 0)); // Dimmed again (back to original)
    }

    #[test]
    fn zero_dim_factor_no_change() {
        let filter = InterlaceCurtain::new().with_dim_factor(0.0);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn full_dim_factor_blacks_out() {
        let filter = InterlaceCurtain::new().with_dim_factor(1.0);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(0, 0, 0));
        assert_eq!(cell.bg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn x_position_has_no_effect() {
        let filter = InterlaceCurtain::new()
            .with_density(1.0)
            .with_dim_factor(0.5);

        // Different x positions on the same row should have same result
        let mut cell1 = make_cell();
        let mut cell2 = make_cell();
        filter.apply(&mut cell1, 0, 0, 10, 10, 0.0);
        filter.apply(&mut cell2, 5, 0, 10, 10, 0.0);
        assert_eq!(cell1.fg, cell2.fg);
        assert_eq!(cell1.bg, cell2.bg);
    }

    #[test]
    fn lower_density_fewer_rows_affected() {
        let filter = InterlaceCurtain::new()
            .with_density(0.5)
            .with_dim_factor(1.0);

        // density=0.5 -> period=4, so rows 0-1 dimmed, rows 2-3 not dimmed
        let mut cell0 = make_cell();
        filter.apply(&mut cell0, 0, 0, 10, 10, 0.0);
        assert_eq!(cell0.fg, Color::rgb(0, 0, 0)); // Dimmed

        let mut cell1 = make_cell();
        filter.apply(&mut cell1, 0, 1, 10, 10, 0.0);
        assert_eq!(cell1.fg, Color::rgb(0, 0, 0)); // Dimmed

        let mut cell2 = make_cell();
        filter.apply(&mut cell2, 0, 2, 10, 10, 0.0);
        assert_eq!(cell2.fg, Color::rgb(100, 100, 100)); // Not dimmed

        let mut cell3 = make_cell();
        filter.apply(&mut cell3, 0, 3, 10, 10, 0.0);
        assert_eq!(cell3.fg, Color::rgb(100, 100, 100)); // Not dimmed
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_interlace_curtain.rs</FILE>
// <DESC>Scanline/interlace effect that dims alternating rows for backdrop effects</DESC>
// <VERS>END OF VERSION: 1.1.2</VERS>
