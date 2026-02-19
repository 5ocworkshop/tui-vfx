// <FILE>tui-vfx-compositor/src/filters/cls_invert.rs</FILE> - <DESC>Invert filter with spatial context support</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>L2/L3 abstraction: make compositor framework-agnostic</WCTX>
// <CLOG>Changed to use tui_vfx_types::Cell and Color for framework independence</CLOG>

use crate::traits::filter::Filter;
use crate::types::cls_filter_spec::ApplyTo;
use tui_vfx_types::{Cell, Color};

/// Invert filter that swaps foreground and background colors.
pub struct Invert {
    /// Which color component(s) to invert
    pub apply_to: ApplyTo,
}

impl Default for Invert {
    fn default() -> Self {
        Self::new(ApplyTo::Both)
    }
}

impl Invert {
    /// Create a new Invert filter with given apply_to setting.
    pub fn new(apply_to: ApplyTo) -> Self {
        Self { apply_to }
    }
}

impl Filter for Invert {
    fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, _t: f64) {
        let old_fg = cell.fg;
        let old_bg = cell.bg;

        match self.apply_to {
            ApplyTo::Foreground => {
                // Invert foreground only (use bg color)
                cell.fg = if old_bg == Color::TRANSPARENT {
                    Color::BLACK
                } else {
                    old_bg
                };
            }
            ApplyTo::Background => {
                // Invert background only (use fg color)
                cell.bg = if old_fg == Color::TRANSPARENT {
                    Color::WHITE
                } else {
                    old_fg
                };
            }
            ApplyTo::Both => {
                // Swap FG and BG
                cell.fg = old_bg;
                cell.bg = old_fg;
                if cell.fg == Color::TRANSPARENT {
                    cell.fg = Color::BLACK;
                }
                if cell.bg == Color::TRANSPARENT {
                    cell.bg = Color::WHITE;
                }
            }
        }
    }
}

#[cfg(test)]
#[allow(clippy::field_reassign_with_default)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    #[test]
    fn test_invert_foreground_uses_black_on_transparent_bg() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(1, 2, 3),
            Color::TRANSPARENT,
            Modifiers::NONE,
        );
        Invert::new(ApplyTo::Foreground).apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::BLACK);
        assert_eq!(cell.bg, Color::TRANSPARENT); // Unchanged
    }

    #[test]
    fn test_invert_foreground_uses_bg_color() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 60, 70),
            Modifiers::NONE,
        );
        Invert::new(ApplyTo::Foreground).apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(50, 60, 70));
        assert_eq!(cell.bg, Color::rgb(50, 60, 70)); // Unchanged
    }

    #[test]
    fn test_invert_background_uses_white_on_transparent_fg() {
        let mut cell = Cell::styled(
            'x',
            Color::TRANSPARENT,
            Color::rgb(1, 2, 3),
            Modifiers::NONE,
        );
        Invert::new(ApplyTo::Background).apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::TRANSPARENT); // Unchanged
        assert_eq!(cell.bg, Color::WHITE);
    }

    #[test]
    fn test_invert_both_swaps_colors() {
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 50, 50),
            Modifiers::NONE,
        );
        Invert::new(ApplyTo::Both).apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(50, 50, 50));
        assert_eq!(cell.bg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_invert_both_handles_transparent() {
        let mut cell = Cell::styled('x', Color::TRANSPARENT, Color::TRANSPARENT, Modifiers::NONE);
        Invert::new(ApplyTo::Both).apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::BLACK);
        assert_eq!(cell.bg, Color::WHITE);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_invert.rs</FILE> - <DESC>Invert filter with spatial context support</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
