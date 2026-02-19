// <FILE>tui-vfx-geometry/src/transitions/col_lerp.rs</FILE>
// <DESC>Small math helper for transitions</DESC>
// <VERS>VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
// <WCTX>Leaf-node helper</WCTX>
// <CLOG>Extracted lerp helper</CLOG>

#[inline]
pub(super) fn lerp(a: f32, b: f32, t: f32) -> f32 {
    a + (b - a) * t
}

// <FILE>tui-vfx-geometry/src/transitions/col_lerp.rs</FILE>
// <DESC>Small math helper for transitions</DESC>
// <VERS>END OF VERSION: 0.1.1 - 2025-12-17T00:00:00Z</VERS>
