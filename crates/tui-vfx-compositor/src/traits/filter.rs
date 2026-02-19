// <FILE>tui-vfx-compositor/src/traits/filter.rs</FILE>
// <DESC>Trait for cell mutation with spatial context</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>L2/L3 abstraction: make compositor framework-agnostic</WCTX>
// <CLOG>Changed Cell type from ratatui to mixed_types for framework independence</CLOG>

use tui_vfx_types::Cell;

/// Trait for filters that mutate cells with full spatial awareness.
///
/// Filters apply per-cell transformations (color manipulation, effects) with access
/// to cell position and rendering area dimensions, enabling position-dependent effects
/// like vignettes, radial gradients, and scanline detection.
///
/// # Design Rationale
///
/// The spatial parameters align this trait with the `StyleShader` pattern, allowing
/// filters to implement position-aware effects that were previously impossible or
/// required inline workarounds in the pipeline.
///
/// # Breaking Change (v2.0.0)
///
/// This version adds spatial context parameters to the `apply()` method:
/// - `x`, `y`: Cell coordinates within the rendered area
/// - `width`, `height`: Total area dimensions for normalization
///
/// All filter implementations must update their signatures to match.
///
/// # Examples
///
/// ## Non-Spatial Filter (ignores spatial params)
///
/// ```ignore
/// impl Filter for Dim {
///     fn apply(&self, cell: &mut Cell, _x: u16, _y: u16, _width: u16, _height: u16, t: f64) {
///         // Uniform dimming - spatial params unused
///         if let Color::Rgb(r, g, b) = cell.fg {
///             let factor = 1.0 - t;
///             cell.fg = Color::Rgb(
///                 (r as f32 * factor) as u8,
///                 (g as f32 * factor) as u8,
///                 (b as f32 * factor) as u8,
///             );
///         }
///     }
/// }
/// ```
///
/// ## Spatial Filter (uses position for radial effect)
///
/// ```ignore
/// impl Filter for Vignette {
///     fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, _t: f64) {
///         // Calculate distance from center
///         let cx = width as f32 / 2.0;
///         let cy = height as f32 / 2.0;
///         let dist = ((x as f32 - cx).powi(2) + (y as f32 - cy).powi(2)).sqrt();
///         let max_dist = (cx.powi(2) + cy.powi(2)).sqrt();
///         let norm_dist = dist / max_dist;
///
///         // Apply dimming based on distance from center
///         if norm_dist > self.radius {
///             let dim_factor = self.strength * (norm_dist - self.radius) / (1.0 - self.radius);
///             // Apply dimming...
///         }
///     }
/// }
/// ```
pub trait Filter {
    /// Apply filter transformation to a cell with full spatial awareness.
    ///
    /// # Parameters
    ///
    /// - `cell`: Mutable reference to the cell being filtered
    /// - `x`: Horizontal coordinate of the cell (0 = leftmost)
    /// - `y`: Vertical coordinate of the cell (0 = topmost)
    /// - `width`: Total width of the rendering area (for normalization)
    /// - `height`: Total height of the rendering area (for normalization)
    /// - `t`: Animation progress (0.0 = start, 1.0 = end)
    ///
    /// # Coordinate System
    ///
    /// - Coordinates are relative to the rendering area (0-indexed)
    /// - `x < width` and `y < height` (callers ensure validity)
    /// - For radial effects: center is typically `(width/2, height/2)`
    ///
    /// # Implementation Notes
    ///
    /// - Filters that don't need spatial context should prefix unused params with `_`
    /// - Filters must handle edge cases (zero dimensions, corner cells) gracefully
    /// - Color calculations should saturate to 0..=255 range (no overflow/underflow)
    /// - Filters are infallible transformations (no Result/Option returns)
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64);
}

// <FILE>tui-vfx-compositor/src/traits/filter.rs</FILE>
// <DESC>Trait for cell mutation with spatial context</DESC>
// <VERS>END OF VERSION: 3.0.0</VERS>
