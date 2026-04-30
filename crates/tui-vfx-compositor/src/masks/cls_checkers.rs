// <FILE>tui-vfx-compositor/src/masks/cls_checkers.rs</FILE> - <DESC>Checkers mask</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.2.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/t replace positional params (width/height not read by this impl).</CLOG>

use crate::traits::mask::Mask;
use tui_vfx_types::VfxCellContext;

/// Checkerboard pattern mask for staggered reveal.
///
/// Reveals cells in a checkerboard order: even-parity cells appear first
/// (at `t > 0.25`), odd-parity cells appear second (at `t > 0.75`).
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
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let bx = ctx.local_x / self.cell_size;
        let by = ctx.local_y / self.cell_size;
        let is_even = (bx + by).is_multiple_of(2);
        if is_even { ctx.t > 0.25 } else { ctx.t > 0.75 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_checkers_alternating_pattern() {
        let mask = Checkers::default();
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.2)));
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.3)));
        assert!(!mask.is_visible(&ctx_at(2, 0, 10, 10, 0.7)));
        assert!(mask.is_visible(&ctx_at(2, 0, 10, 10, 0.8)));
        assert!(!mask.is_visible(&ctx_at(0, 2, 10, 10, 0.7)));
        assert!(mask.is_visible(&ctx_at(2, 2, 10, 10, 0.3)));
    }

    #[test]
    fn test_checkers_custom_cell_size() {
        let mask = Checkers::new(4);
        assert!(mask.is_visible(&ctx_at(0, 0, 20, 20, 0.3)));
        assert!(!mask.is_visible(&ctx_at(4, 0, 20, 20, 0.7)));
        assert!(mask.is_visible(&ctx_at(4, 0, 20, 20, 0.8)));
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_checkers.rs</FILE> - <DESC>Checkers mask</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
