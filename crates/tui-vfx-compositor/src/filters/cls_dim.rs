// <FILE>tui-vfx-compositor/src/filters/cls_dim.rs</FILE>
// <DESC>Dim filter with apply_to targeting and spatial context support</DESC>
// <VERS>VERSION: 4.0.1</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in color math to prevent off-by-one errors</CLOG>

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use tui_vfx_types::{Cell, Color};

/// Dim filter that darkens colors based on a factor.
///
/// Supports targeting foreground, background, or both via `apply_to`.
pub struct Dim {
    /// Which color component(s) to dim
    pub apply_to: ApplyTo,
}

impl Default for Dim {
    fn default() -> Self {
        Self::new(ApplyTo::Both)
    }
}

impl Dim {
    /// Create a new Dim filter with given apply_to setting.
    pub fn new(apply_to: ApplyTo) -> Self {
        Self { apply_to }
    }
}

impl Filter for Dim {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, t: f64) {
        let t = t as f32;

        fn dim_color(c: Color, factor: f32) -> Color {
            // tui_vfx_types::Color always has RGB components
            // Use round() to prevent off-by-one errors at boundary values
            Color::rgb(
                (c.r as f32 * (1.0 - factor)).round() as u8,
                (c.g as f32 * (1.0 - factor)).round() as u8,
                (c.b as f32 * (1.0 - factor)).round() as u8,
            )
        }

        match self.apply_to {
            ApplyTo::Foreground => {
                cell.fg = dim_color(cell.fg, t);
            }
            ApplyTo::Background => {
                cell.bg = dim_color(cell.bg, t);
            }
            ApplyTo::Both => {
                cell.fg = dim_color(cell.fg, t);
                cell.bg = dim_color(cell.bg, t);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    #[test]
    fn test_dim_foreground_only() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 50, 0),
            Color::rgb(10, 20, 30),
            Modifiers::NONE,
        );
        Dim::new(ApplyTo::Foreground).apply(&mut cell, 0, 0, 10, 10, 0.5);
        assert_eq!(cell.fg, Color::rgb(50, 25, 0));
        assert_eq!(cell.bg, Color::rgb(10, 20, 30)); // Unchanged
    }

    #[test]
    fn test_dim_background_only() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 50, 0),
            Color::rgb(10, 20, 30),
            Modifiers::NONE,
        );
        Dim::new(ApplyTo::Background).apply(&mut cell, 0, 0, 10, 10, 0.5);
        assert_eq!(cell.fg, Color::rgb(100, 50, 0)); // Unchanged
        assert_eq!(cell.bg, Color::rgb(5, 10, 15));
    }

    #[test]
    fn test_dim_both() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        Dim::new(ApplyTo::Both).apply(&mut cell, 0, 0, 10, 10, 0.5);
        assert_eq!(cell.fg, Color::rgb(50, 50, 50));
        assert_eq!(cell.bg, Color::rgb(50, 50, 50));
    }

    #[test]
    fn test_dim_t_zero_no_change() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        Dim::default().apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_dim_t_one_full_black() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        Dim::default().apply(&mut cell, 0, 0, 10, 10, 1.0);
        assert_eq!(cell.fg, Color::rgb(0, 0, 0));
        assert_eq!(cell.bg, Color::rgb(0, 0, 0));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_dim.rs</FILE>
// <DESC>Dim filter with apply_to targeting and spatial context support</DESC>
// <VERS>END OF VERSION: 4.0.1</VERS>
