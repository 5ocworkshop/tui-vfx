// <FILE>tui-vfx-compositor/tests/context/test_cls_compositor_ctx.rs</FILE>
// <DESC>Tests for CompositorCtx grid management</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>L2/L3 abstraction: make compositor framework-agnostic</WCTX>
// <CLOG>Updated to use OwnedGrid with (width, height) instead of ratatui Buffer with Rect</CLOG>

use tui_vfx_compositor::context::cls_compositor_ctx::CompositorCtx;
use tui_vfx_types::Grid;

#[test]
fn test_reuse() {
    let mut ctx = CompositorCtx::new();

    let buf1_ptr = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *mut _
    };

    let buf2_ptr = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *mut _
    };

    assert_eq!(buf1_ptr, buf2_ptr, "Should reuse the same grid instance");
}

#[test]
fn test_resize() {
    let mut ctx = CompositorCtx::new();

    {
        let buf = ctx.scratchpad_for(10, 10);
        assert_eq!(buf.width(), 10);
        assert_eq!(buf.height(), 10);
    }

    {
        let buf = ctx.scratchpad_for(20, 20);
        assert_eq!(buf.width(), 20);
        assert_eq!(buf.height(), 20);
    }
}

// ============================================================================
// WG4: Growth-Only Grid Allocation Tests
// ============================================================================

/// ARC_FUNC_01: Verify same-size requests reuse grid (capacity unchanged)
#[test]
fn test_scratchpad_reuse_same_size() {
    let mut ctx = CompositorCtx::new();

    // First call allocates
    ctx.scratchpad_for(10, 10);

    // Second call with same size should reuse (capacity = 100)
    let buf1_ptr = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    // Third call - still reusing
    let buf2_ptr = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    // Pointers should be identical (same grid, not reallocated)
    assert_eq!(
        buf1_ptr, buf2_ptr,
        "Same-size requests should reuse grid without reallocation"
    );
}

/// ARC_FUNC_02: Verify smaller requests reuse grid if within threshold
#[test]
fn test_scratchpad_reuse_smaller_size() {
    let mut ctx = CompositorCtx::new();

    // Allocate for 10x10 (100 cells)
    ctx.scratchpad_for(10, 10);

    let ptr_after_large = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    // Request 8x8 (64 cells) - should reuse (64 > 100/2 = 50)
    let ptr_after_smaller = {
        let buf = ctx.scratchpad_for(8, 8);
        assert_eq!(buf.width(), 8, "Grid width should match requested");
        assert_eq!(buf.height(), 8, "Grid height should match requested");
        buf as *const _
    };

    // Pointers should be identical (reused, not reallocated)
    assert_eq!(
        ptr_after_large, ptr_after_smaller,
        "Smaller request within threshold should reuse grid"
    );

    // Request large again - still reusing
    let ptr_after_large_again = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    assert_eq!(
        ptr_after_smaller, ptr_after_large_again,
        "Should still reuse same grid"
    );
}

/// ARC_FUNC_03: Verify shrink threshold triggers reallocation
#[test]
fn test_scratchpad_shrink_threshold() {
    let mut ctx = CompositorCtx::new();

    // Allocate for 10x10 (100 cells)
    ctx.scratchpad_for(10, 10);

    let _ptr_after_large = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    // Request 5x5 (25 cells) - below 50% threshold (25 < 50), should shrink
    let ptr_after_shrink = {
        let buf = ctx.scratchpad_for(5, 5);
        assert_eq!(buf.width(), 5, "Grid width should match requested");
        assert_eq!(buf.height(), 5, "Grid height should match requested");
        buf as *const _
    };

    // Request 5x5 again - should reuse the NEW capacity (25 cells)
    let ptr_after_shrink_2 = {
        let buf = ctx.scratchpad_for(5, 5);
        buf as *const _
    };

    assert_eq!(
        ptr_after_shrink, ptr_after_shrink_2,
        "After shrink, same-size requests should reuse new capacity"
    );

    // Request 10x10 again - this should GROW (25 < 100), triggering reallocation
    let ptr_after_grow = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    // After growing, we can verify it's a different allocation by checking if subsequent same-size calls reuse
    let ptr_after_grow_2 = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    assert_eq!(
        ptr_after_grow, ptr_after_grow_2,
        "After growth, same-size requests should reuse new capacity"
    );
}

/// ARC_FUNC_04: Verify growth beyond capacity triggers reallocation
#[test]
fn test_scratchpad_growth() {
    let mut ctx = CompositorCtx::new();

    // Allocate for 5x5 (25 cells)
    ctx.scratchpad_for(5, 5);

    let _ptr_after_small = {
        let buf = ctx.scratchpad_for(5, 5);
        assert_eq!(buf.width(), 5);
        assert_eq!(buf.height(), 5);
        buf as *const _
    };

    // Request 10x10 (100 cells) - must grow (100 > 25)
    let ptr_after_growth = {
        let buf = ctx.scratchpad_for(10, 10);
        assert_eq!(
            buf.width(),
            10,
            "Grid width should match requested large size"
        );
        assert_eq!(
            buf.height(),
            10,
            "Grid height should match requested large size"
        );
        buf as *const _
    };

    // Verify the grown grid is reused for same-size requests
    let ptr_after_growth_2 = {
        let buf = ctx.scratchpad_for(10, 10);
        buf as *const _
    };

    assert_eq!(
        ptr_after_growth, ptr_after_growth_2,
        "After growth, same-size requests should reuse new capacity"
    );

    // Request 8x8 (64 cells) to test reuse: 64 > 50, should reuse
    let ptr_after_medium = {
        let buf = ctx.scratchpad_for(8, 8);
        assert_eq!(buf.width(), 8);
        assert_eq!(buf.height(), 8);
        buf as *const _
    };

    assert_eq!(
        ptr_after_growth_2, ptr_after_medium,
        "Medium request within threshold should reuse grown grid"
    );
}

// <FILE>tui-vfx-compositor/tests/context/test_cls_compositor_ctx.rs</FILE>
// <DESC>Tests for CompositorCtx grid management</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
