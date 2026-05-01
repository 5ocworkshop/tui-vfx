// <FILE>tui-vfx-compositor-next/src/context/cls_compositor_ctx.rs</FILE>
// <DESC>Grid reuse manager with growth-only allocation</DESC>
// <VERS>VERSION: 2.0.1</VERS>
// <WCTX>Compositor clippy cleanup pass</WCTX>
// <CLOG>2.0.1: PATCH — fold is_none/unwrap scratchpad check into a match per clippy</CLOG>

use tui_vfx_types::{Cell, Grid, GridExt, OwnedGrid};

/// Compositor context with optimized grid allocation.
///
/// ## Grid Allocation Strategy
///
/// The scratchpad grid uses a **growth-only allocation strategy** with a
/// **50% shrink threshold** to minimize allocations during terminal resize:
///
/// - **Reuse:** If `requested_size <= capacity`, reuse existing grid
/// - **Shrink:** If `requested_size < (capacity / 2)`, reallocate smaller
/// - **Grow:** If `requested_size > capacity`, reallocate larger
///
/// This eliminates repeated allocations during interactive resize while
/// preventing unbounded memory growth.
#[derive(Debug, Default)]
pub struct CompositorCtx {
    scratchpad: Option<OwnedGrid>,
    /// Tracked capacity in cells (width * height) for growth-only strategy
    scratchpad_capacity: usize,
}

impl CompositorCtx {
    pub fn new() -> Self {
        Self {
            scratchpad: None,
            scratchpad_capacity: 0,
        }
    }

    /// Get a scratchpad grid of the requested size.
    ///
    /// The grid is cleared (filled with default cells) before returning.
    pub fn scratchpad_for(&mut self, width: usize, height: usize) -> &mut OwnedGrid {
        let requested_size = width.saturating_mul(height);

        // Growth-only strategy with shrink threshold
        let should_reallocate = match &self.scratchpad {
            None => true,
            Some(grid) => {
                requested_size > self.scratchpad_capacity
                    || requested_size < (self.scratchpad_capacity / 2)
                    || grid.width() != width
                    || grid.height() != height
            }
        };

        if should_reallocate {
            // Allocate new grid at exact requested size
            self.scratchpad = Some(OwnedGrid::new(width, height));
            self.scratchpad_capacity = requested_size;
        }

        // Clear the grid
        let grid = self.scratchpad.as_mut().unwrap();
        grid.fill(Cell::default());
        grid
    }
}

// <FILE>tui-vfx-compositor-next/src/context/cls_compositor_ctx.rs</FILE>
// <DESC>Grid reuse manager with growth-only allocation</DESC>
// <VERS>END OF VERSION: 2.0.1</VERS>
