// <FILE>crates/tui-vfx-shadow/src/renderers/cls_gradient.rs</FILE> - <DESC>Multi-layer gradient shadow renderer</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Remove modifier-alpha feature flag</WCTX>
// <CLOG>mod_alpha always available - remove cfg guards</CLOG>

//! Multi-layer gradient shadow renderer.
//!
//! Creates softer shadows by rendering multiple layers with different colors
//! from a theme's surface ladder, creating a visible gradient falloff effect.
//!
//! Since terminals don't support alpha blending, gradients must use distinct
//! RGB colors (e.g., surface_container → surface_container_low → surface_container_lowest).

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Multi-layer gradient shadow renderer.
///
/// Renders shadows as multiple concentric layers with different colors,
/// creating a visible falloff effect. Each layer is rendered slightly further
/// from the element with a different color from the provided gradient.
pub struct GradientRenderer;

impl GradientRenderer {
    /// Render a gradient shadow using an array of colors.
    ///
    /// # Arguments
    /// * `grid` - The grid to render into
    /// * `element_rect` - The rect of the element casting the shadow
    /// * `config` - Shadow configuration (offset and edges used, color ignored)
    /// * `colors` - Gradient colors from lightest (outer) to darkest (inner)
    /// * `progress` - Animation progress 0.0-1.0
    ///
    /// # Example
    /// ```ignore
    /// // Use theme surface ladder for visible gradient
    /// let colors = [
    ///     theme.surface.surface_container,      // outer (lightest)
    ///     theme.surface.surface_container_low,  // middle
    ///     theme.surface.surface_container_lowest, // inner (darkest)
    /// ];
    /// GradientRenderer::render_with_colors(&mut grid, rect, &config, &colors, 1.0);
    /// ```
    pub fn render_with_colors<G: Grid>(
        grid: &mut G,
        element_rect: Rect,
        config: &ShadowConfig,
        colors: &[Color],
        progress: f64,
    ) {
        if colors.is_empty() || progress <= 0.0 {
            return;
        }

        let layers = colors.len();

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let edges = config.edges;

        // Render layers from outermost to innermost (so inner layers overwrite outer)
        // colors[0] = outermost/lightest, colors[n-1] = innermost/darkest
        for (layer_idx, color) in colors.iter().enumerate().rev() {
            // Layer 0 is outermost (furthest), layer n-1 is innermost (closest)
            let layer_mult = (layers - layer_idx) as i32;
            let layer_ox = ox * layer_mult;
            let layer_oy = oy * layer_mult;

            // Apply progress to alpha if needed
            let layer_color = if progress < 1.0 {
                let alpha = (color.a as f64 * progress).round() as u8;
                color.with_alpha(alpha)
            } else {
                *color
            };

            // Render this layer's shadow regions
            Self::render_layer(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                layer_ox,
                layer_oy,
                edges,
                layer_color,
            );
        }
    }

    /// Render a gradient shadow for the given element rect (legacy alpha-based).
    ///
    /// Note: This uses alpha variation which may not be visible in terminals.
    /// Prefer `render_with_colors` with theme colors for visible gradients.
    ///
    /// # Arguments
    /// * `grid` - The grid to render into
    /// * `element_rect` - The rect of the element casting the shadow
    /// * `config` - Shadow configuration
    /// * `layers` - Number of gradient layers (1-4)
    /// * `progress` - Animation progress 0.0-1.0
    pub fn render<G: Grid>(
        grid: &mut G,
        element_rect: Rect,
        config: &ShadowConfig,
        layers: u8,
        progress: f64,
    ) {
        let base_color = config.color_at_progress(progress);
        if base_color.a == 0 {
            return;
        }

        let layers = layers.clamp(1, 4) as usize;

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let edges = config.edges;

        // Render layers from outermost to innermost (so inner layers overwrite outer)
        for layer in (0..layers).rev() {
            // Calculate layer offset multiplier (outer layers are further)
            let layer_mult = (layer + 1) as i32;
            let layer_ox = ox * layer_mult;
            let layer_oy = oy * layer_mult;

            // Calculate layer color (outer layers are lighter/more transparent)
            let intensity = 1.0 - (layer as f32 / layers as f32);
            let layer_alpha = (base_color.a as f32 * intensity).round() as u8;
            let layer_color = base_color.with_alpha(layer_alpha);

            // Render this layer's shadow regions
            Self::render_layer(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                layer_ox,
                layer_oy,
                edges,
                layer_color,
            );
        }
    }

    /// Render a single shadow layer.
    #[allow(clippy::too_many_arguments)]
    fn render_layer<G: Grid>(
        grid: &mut G,
        rect_x: i32,
        rect_y: i32,
        rect_w: i32,
        rect_h: i32,
        ox: i32,
        oy: i32,
        edges: crate::types::ShadowEdges,
        color: Color,
    ) {
        if color.a == 0 {
            return;
        }

        let cell = Cell::new(' ').with_bg(color).with_mod_alpha(Some(255));

        // Right edge shadow
        if edges.has_right() && ox > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            let end_x = (rect_x + rect_w + ox).max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let end_y = (rect_y + rect_h + oy.min(0)).max(0) as usize;

            Self::fill_region(
                grid,
                start_x,
                start_y,
                end_x.saturating_sub(start_x),
                end_y.saturating_sub(start_y),
                cell,
            );
        }

        // Bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let end_x = (rect_x + rect_w + ox.min(0)).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            let end_y = (rect_y + rect_h + oy).max(0) as usize;

            Self::fill_region(
                grid,
                start_x,
                start_y,
                end_x.saturating_sub(start_x),
                end_y.saturating_sub(start_y),
                cell,
            );
        }

        // Left edge shadow
        if edges.has_left() && ox < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let end_x = rect_x.max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let end_y = (rect_y + rect_h + oy.min(0)).max(0) as usize;

            Self::fill_region(
                grid,
                start_x,
                start_y,
                end_x.saturating_sub(start_x),
                end_y.saturating_sub(start_y),
                cell,
            );
        }

        // Top edge shadow
        if edges.has_top() && oy < 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let end_x = (rect_x + rect_w + ox.min(0)).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;
            let end_y = rect_y.max(0) as usize;

            Self::fill_region(
                grid,
                start_x,
                start_y,
                end_x.saturating_sub(start_x),
                end_y.saturating_sub(start_y),
                cell,
            );
        }

        // Corner region (bottom-right for positive offset)
        if edges.has_right() && edges.has_bottom() && ox > 0 && oy > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;

            Self::fill_region(grid, start_x, start_y, ox as usize, oy as usize, cell);
        }

        // Corner region (top-left for negative offset)
        if edges.has_left() && edges.has_top() && ox < 0 && oy < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;

            Self::fill_region(grid, start_x, start_y, (-ox) as usize, (-oy) as usize, cell);
        }
    }

    /// Fill a rectangular region with a cell.
    fn fill_region<G: Grid>(grid: &mut G, x: usize, y: usize, w: usize, h: usize, cell: Cell) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if grid.in_bounds(px, py) {
                    grid.set(px, py, cell);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShadowEdges;
    use tui_vfx_types::OwnedGrid;

    #[test]
    fn test_render_single_layer() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_offset(1, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        GradientRenderer::render(&mut grid, rect, &config, 1, 1.0);

        // Check that shadow exists
        let cell = grid.get(15, 3).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_render_multiple_layers() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_offset(1, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        GradientRenderer::render(&mut grid, rect, &config, 3, 1.0);

        // Outer layer should be lighter (lower alpha)
        let outer_cell = grid.get(17, 9).unwrap(); // Further out
        let inner_cell = grid.get(15, 8).unwrap(); // Closer in

        // Both should have some shadow
        assert_ne!(outer_cell.bg, Color::TRANSPARENT);
        assert_ne!(inner_cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_zero_progress_renders_nothing() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK);

        GradientRenderer::render(&mut grid, rect, &config, 3, 0.0);

        // All cells should be default
        for y in 0..15 {
            for x in 0..30 {
                let cell = grid.get(x, y).unwrap();
                assert_eq!(cell.bg, Color::TRANSPARENT);
            }
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/renderers/cls_gradient.rs</FILE> - <DESC>Multi-layer gradient shadow renderer</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
