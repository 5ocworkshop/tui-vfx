// <FILE>tui-vfx-compositor/src/masks/cls_checkers.rs</FILE> - <DESC>Checkers mask</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-23</VERS>
// <WCTX>Pipeline configuration fix</WCTX>
// <CLOG>Added cell_size config field</CLOG>

use crate::traits::mask::Mask;

/// Checkerboard pattern mask for staggered reveal.
pub struct Checkers {
    /// Size of each checker cell in pixels
    pub cell_size: u16,
}

impl Default for Checkers {
    fn default() -> Self {
        Self::new(2)
    }
}

impl Checkers {
    /// Create a new Checkers mask.
    pub fn new(cell_size: u16) -> Self {
        Self {
            cell_size: cell_size.max(1),
        }
    }
}

impl Mask for Checkers {
    fn is_visible(&self, x: u16, y: u16, _w: u16, _h: u16, progress: f64) -> bool {
        let bx = x / self.cell_size;
        let by = y / self.cell_size;
        let is_even = (bx + by) % 2 == 0;
        if is_even {
            progress > 0.25
        } else {
            progress > 0.75
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_checkers_alternating_pattern() {
        let mask = Checkers::default();
        assert!(!mask.is_visible(0, 0, 10, 10, 0.2));
        assert!(mask.is_visible(0, 0, 10, 10, 0.3));
        assert!(!mask.is_visible(2, 0, 10, 10, 0.7));
        assert!(mask.is_visible(2, 0, 10, 10, 0.8));
        assert!(!mask.is_visible(0, 2, 10, 10, 0.7));
        assert!(mask.is_visible(2, 2, 10, 10, 0.3));
    }

    #[test]
    fn test_checkers_custom_cell_size() {
        let mask = Checkers::new(4);
        assert!(mask.is_visible(0, 0, 20, 20, 0.3));
        assert!(!mask.is_visible(4, 0, 20, 20, 0.7));
        assert!(mask.is_visible(4, 0, 20, 20, 0.8));
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_checkers.rs</FILE> - <DESC>Checkers mask</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-23</VERS>
