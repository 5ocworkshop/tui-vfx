// <FILE>crates/tui-vfx-types/src/cls_vfx_cell_context.rs</FILE> - <DESC>Per-cell spatial context bundle shared by Filter / Mask / Sampler / StyleShader</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>2026-04-26 packet — add resolved_x/resolved_y so downstream stages can react to per-cell sampler displacement.</WCTX>
// <CLOG>1.1.0: add resolved_x/resolved_y i32 fields (default to local at construction), with_sampler_resolution accumulator builder, displacement/displacement_magnitude accessors, and five peer tests.</CLOG>

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

    /// Sampler-resolved x coordinate after any in-flight sampler chain has
    /// run. `i32` to allow negative offsets (e.g., reads conceptually past
    /// the layer's left edge). Defaults to `local_x as i32` at construction;
    /// each sampler in a chain accumulates its delta via
    /// [`Self::with_sampler_resolution`]. Downstream stages (mask, shader,
    /// filter) read this to react to displacement — see [`Self::displacement`].
    /// Readers that need a `usize` source-grid index should clamp:
    /// `ctx.resolved_x.clamp(0, ctx.width as i32 - 1) as usize`.
    pub resolved_x: i32,

    /// Sampler-resolved y coordinate. Same accumulator semantics as
    /// [`Self::resolved_x`].
    pub resolved_y: i32,
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
            resolved_x: local_x as i32,
            resolved_y: local_y as i32,
        }
    }

    /// Build a downstream context by applying a sampler's resolved-coord
    /// delta to a prior context. All non-resolved fields are preserved
    /// verbatim. Uses `saturating_add` so an out-of-spec sampler delta
    /// cannot panic on `i32::MAX` / `i32::MIN`.
    ///
    /// Samplers are applied in pipeline order; the resolved coords are an
    /// accumulator across the chain (`resolved += delta` per sampler).
    /// Order matters only for non-commutative sampler compositions (a
    /// sampler that scales its delta by `prior_resolved_y`, for example);
    /// today's samplers compute deltas from `(local_x, local_y, t)` only,
    /// which is commutative.
    #[inline]
    pub fn with_sampler_resolution(self, delta_x: i32, delta_y: i32) -> Self {
        Self {
            resolved_x: self.resolved_x.saturating_add(delta_x),
            resolved_y: self.resolved_y.saturating_add(delta_y),
            ..self
        }
    }

    /// Displacement vector — `(resolved - local)`. Useful for
    /// distance-based shaders, masks, and filters. Saturating in both
    /// limbs to keep extreme inputs panic-free.
    #[inline]
    pub fn displacement(&self) -> (i32, i32) {
        (
            self.resolved_x.saturating_sub(self.local_x as i32),
            self.resolved_y.saturating_sub(self.local_y as i32),
        )
    }

    /// Euclidean magnitude of [`Self::displacement`]. Convenience for
    /// brightness/intensity fades that read off a sampler's displacement.
    #[inline]
    pub fn displacement_magnitude(&self) -> f32 {
        let (dx, dy) = self.displacement();
        ((dx * dx + dy * dy) as f32).sqrt()
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

    #[test]
    fn resolved_defaults_to_local_at_construction() {
        let ctx = VfxCellContext::new(3, 5, 10, 10, 0, 0, 0.0);
        assert_eq!(ctx.resolved_x, 3);
        assert_eq!(ctx.resolved_y, 5);
        assert_eq!(ctx.displacement(), (0, 0));
        assert_eq!(ctx.displacement_magnitude(), 0.0);
    }

    #[test]
    fn with_sampler_resolution_accumulates() {
        let ctx = VfxCellContext::new(3, 5, 10, 10, 0, 0, 0.0)
            .with_sampler_resolution(2, -1)
            .with_sampler_resolution(1, 4);
        assert_eq!(ctx.resolved_x, 6); // 3 + 2 + 1
        assert_eq!(ctx.resolved_y, 8); // 5 + (-1) + 4
        assert_eq!(ctx.displacement(), (3, 3));
    }

    #[test]
    fn displacement_magnitude_is_euclidean() {
        let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
            .with_sampler_resolution(3, 4);
        assert!((ctx.displacement_magnitude() - 5.0).abs() < f32::EPSILON);
    }

    #[test]
    fn with_sampler_resolution_saturates_at_i32_bounds() {
        let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
            .with_sampler_resolution(i32::MAX, i32::MAX)
            .with_sampler_resolution(1, 1);
        assert_eq!(ctx.resolved_x, i32::MAX);
        assert_eq!(ctx.resolved_y, i32::MAX);
    }

    #[test]
    fn negative_resolved_coords_supported() {
        let ctx = VfxCellContext::new(0, 0, 10, 10, 0, 0, 0.0)
            .with_sampler_resolution(-3, -7);
        assert_eq!(ctx.resolved_x, -3);
        assert_eq!(ctx.resolved_y, -7);
    }
}

// <FILE>crates/tui-vfx-types/src/cls_vfx_cell_context.rs</FILE>
// <VERS>END OF VERSION: 1.1.0</VERS>
