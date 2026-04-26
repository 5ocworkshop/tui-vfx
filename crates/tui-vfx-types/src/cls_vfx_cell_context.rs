// <FILE>crates/tui-vfx-types/src/cls_vfx_cell_context.rs</FILE> - <DESC>Per-cell spatial context bundle shared by Filter / Mask / Sampler / StyleShader</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Slice 6.6 §F.1 — bundle the seven per-cell spatial fields shared across Filter/Mask/Sampler/StyleShader so future field additions extend the struct rather than churning four trait signatures.</WCTX>
// <CLOG>1.0.0: introduce VfxCellContext { local_x, local_y, width, height, screen_x, screen_y, t } with Copy + new() + screen_cell_x/y + normalized_x/y accessors and inline tests.</CLOG>

//! Per-cell spatial context for compositor trait surfaces.
//!
//! See [`VfxCellContext`] for the struct itself.

/// Per-cell spatial context shared across `Filter`, `Mask`, `Sampler`, and
/// (composed into) `ShaderContext`.
///
/// All fields are scalar; the struct is `Copy` and zero-allocation. Pass by
/// reference (`&VfxCellContext`) at trait surfaces for consistency and to
/// keep the door open for future heap fields, even though `Copy` would
/// allow by-value.
///
/// # Coordinate systems
///
/// - **Local** (`local_x`, `local_y`): position within the layer's local
///   rect. `(0, 0)` is the layer's top-left.
/// - **Screen offset** (`screen_x`, `screen_y`): the layer's top-left
///   absolute screen position. Use [`Self::screen_cell_x`] /
///   [`Self::screen_cell_y`] to compute the cell's absolute screen
///   coordinate.
///
/// # Construction
///
/// Prefer [`Self::new`] over struct-literal construction. A struct literal
/// silently breaks when a future field is added; a constructor call fails
/// to compile, which is what we want.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VfxCellContext {
    /// Cell coordinate within the layer's local rect (0 = left edge).
    pub local_x: u16,
    /// Cell coordinate within the layer's local rect (0 = top edge).
    pub local_y: u16,
    /// Layer width in cells.
    pub width: u16,
    /// Layer height in cells.
    pub height: u16,
    /// Screen X offset — layer's left edge in absolute screen coordinates.
    pub screen_x: u16,
    /// Screen Y offset — layer's top edge in absolute screen coordinates.
    pub screen_y: u16,
    /// Animation progress / time clock. Same `f64` semantics as the
    /// legacy `t` / `progress` parameters across `Filter`, `Mask`, and
    /// `Sampler`.
    pub t: f64,
}

impl VfxCellContext {
    /// Construct a context bundle from the seven per-cell spatial fields.
    ///
    /// All call sites should use this constructor; struct-literal
    /// construction is allowed but discouraged because adding a future
    /// field would silently break literal sites.
    #[inline]
    pub fn new(
        local_x: u16,
        local_y: u16,
        width: u16,
        height: u16,
        screen_x: u16,
        screen_y: u16,
        t: f64,
    ) -> Self {
        Self {
            local_x,
            local_y,
            width,
            height,
            screen_x,
            screen_y,
            t,
        }
    }

    /// Absolute screen X for this cell (`screen_x + local_x`, saturating).
    #[inline]
    pub fn screen_cell_x(&self) -> u16 {
        self.screen_x.saturating_add(self.local_x)
    }

    /// Absolute screen Y for this cell (`screen_y + local_y`, saturating).
    #[inline]
    pub fn screen_cell_y(&self) -> u16 {
        self.screen_y.saturating_add(self.local_y)
    }

    /// Normalized local X position (0.0 = left edge, 1.0 = right edge).
    /// Returns `0.0` for degenerate `width == 0`.
    #[inline]
    pub fn normalized_x(&self) -> f32 {
        if self.width > 0 {
            self.local_x as f32 / self.width as f32
        } else {
            0.0
        }
    }

    /// Normalized local Y position (0.0 = top edge, 1.0 = bottom edge).
    /// Returns `0.0` for degenerate `height == 0`.
    #[inline]
    pub fn normalized_y(&self) -> f32 {
        if self.height > 0 {
            self.local_y as f32 / self.height as f32
        } else {
            0.0
        }
    }

    /// Test-only zero-filled context. Production code constructs via
    /// [`Self::new`].
    #[cfg(test)]
    pub fn test_default() -> Self {
        Self::new(0, 0, 0, 0, 0, 0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_cell_coords_are_local_plus_offset() {
        let ctx = VfxCellContext::new(3, 5, 10, 10, 100, 200, 0.5);
        assert_eq!(ctx.screen_cell_x(), 103);
        assert_eq!(ctx.screen_cell_y(), 205);
    }

    #[test]
    fn normalized_returns_zero_on_degenerate_dimensions() {
        let ctx = VfxCellContext::new(3, 5, 0, 0, 0, 0, 0.0);
        assert_eq!(ctx.normalized_x(), 0.0);
        assert_eq!(ctx.normalized_y(), 0.0);
    }

    #[test]
    fn normalized_returns_fraction_on_normal_dimensions() {
        let ctx = VfxCellContext::new(2, 4, 8, 8, 0, 0, 0.0);
        assert!((ctx.normalized_x() - 0.25).abs() < f32::EPSILON);
        assert!((ctx.normalized_y() - 0.5).abs() < f32::EPSILON);
    }

    #[test]
    fn screen_cell_coords_saturate_on_overflow() {
        let ctx = VfxCellContext::new(100, 100, 0, 0, u16::MAX, u16::MAX, 0.0);
        assert_eq!(ctx.screen_cell_x(), u16::MAX);
        assert_eq!(ctx.screen_cell_y(), u16::MAX);
    }

    #[test]
    fn copy_semantics_compile() {
        let ctx = VfxCellContext::test_default();
        let _a = ctx; // Copy
        let _b = ctx; // Still usable
        assert_eq!(ctx.local_x, 0);
    }
}

// <FILE>crates/tui-vfx-types/src/cls_vfx_cell_context.rs</FILE>
// <VERS>END OF VERSION: 1.0.0</VERS>
