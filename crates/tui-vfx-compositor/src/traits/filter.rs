// <FILE>tui-vfx-compositor/src/traits/filter.rs</FILE>
// <DESC>Trait for cell mutation with spatial context</DESC>
// <VERS>VERSION: 4.0.0</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>4.0.0: BREAKING — apply signature changes from positional (x, y, width, height, t) to &VfxCellContext.</CLOG>

use tui_vfx_types::{Cell, VfxCellContext};

/// Trait for filters that mutate cells with full spatial awareness.
///
/// Filters apply per-cell transformations (color manipulation, effects) with access
/// to cell position and rendering area dimensions, enabling position-dependent effects
/// like vignettes, radial gradients, and scanline detection.
///
/// # Design Rationale
///
/// Spatial context is bundled into [`VfxCellContext`] so that future field
/// additions (screen offsets, display scale, etc.) extend the struct rather
/// than churning this trait signature again.
///
/// # Breaking Change (v4.0.0)
///
/// The `apply()` method now accepts `&VfxCellContext` instead of five
/// positional scalars (`x`, `y`, `width`, `height`, `t`). Read the fields
/// as `ctx.local_x`, `ctx.local_y`, `ctx.width`, `ctx.height`, `ctx.t`.
///
/// # Examples
///
/// ## Non-Spatial Filter (ignores spatial params)
///
/// ```ignore
/// impl Filter for Dim {
///     fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
///         // Uniform dimming — position fields unused
///         let t = ctx.t as f32;
///         cell.fg = Color::rgb(
///             (cell.fg.r as f32 * (1.0 - t)).round() as u8,
///             (cell.fg.g as f32 * (1.0 - t)).round() as u8,
///             (cell.fg.b as f32 * (1.0 - t)).round() as u8,
///         );
///     }
/// }
/// ```
///
/// ## Spatial Filter (uses position for radial effect)
///
/// ```ignore
/// impl Filter for Vignette {
///     fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
///         let (x, y, width, height) = (ctx.local_x, ctx.local_y, ctx.width, ctx.height);
///         if width == 0 || height == 0 {
///             return;
///         }
///         let cx = width as f32 / 2.0;
///         let cy = height as f32 / 2.0;
///         let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
///         let max_dist = (cx.powi(2) + cy.powi(2)).sqrt();
///         let norm_dist = dist / max_dist;
///         if norm_dist > self.radius {
///             let dim_factor =
///                 self.strength * (norm_dist - self.radius) / (1.0 - self.radius);
///             // apply dimming...
///             drop(dim_factor);
///         }
///     }
/// }
/// ```
pub trait Filter {
    /// Apply filter transformation to a cell with full spatial awareness.
    ///
    /// # Parameters
    ///
    /// - `cell`: Mutable reference to the cell being filtered.
    /// - `ctx`: Per-cell spatial context bundle. Key fields:
    ///   - `ctx.local_x` / `ctx.local_y`: cell position within the layer's local rect (0-indexed).
    ///   - `ctx.width` / `ctx.height`: layer dimensions (for normalization).
    ///   - `ctx.t`: animation progress (0.0 = start, 1.0 = end of the clock period).
    ///
    /// # Coordinate System
    ///
    /// - Coordinates are relative to the rendering area (0-indexed).
    /// - `local_x < width` and `local_y < height` (callers ensure validity).
    /// - For radial effects: center is typically `(width/2, height/2)`.
    ///
    /// # Implementation Notes
    ///
    /// - Filters that don't need spatial context should name the param `ctx` and
    ///   read only the fields they need; prefix with `_ctx` only if zero fields
    ///   are accessed.
    /// - Filters must handle edge cases (zero dimensions, corner cells) gracefully.
    /// - Color calculations should saturate to 0..=255 range (no overflow/underflow).
    /// - Filters are infallible transformations (no `Result`/`Option` returns).
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext);
}

// <FILE>tui-vfx-compositor/src/traits/filter.rs</FILE>
// <DESC>Trait for cell mutation with spatial context</DESC>
// <VERS>END OF VERSION: 4.0.0</VERS>
