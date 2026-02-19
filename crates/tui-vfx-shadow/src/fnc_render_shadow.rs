// <FILE>crates/tui-vfx-shadow/src/fnc_render_shadow.rs</FILE> - <DESC>Main entry point for shadow rendering</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New crate for theme-aware shadow rendering with multiple styles</WCTX>
// <CLOG>Add render_shadow_gradient_colors for visible gradients with theme colors</CLOG>

//! Main entry point function for shadow rendering.
//!
//! Provides a unified API that dispatches to the appropriate renderer
//! based on the configured shadow style.

use tui_vfx_types::{Grid, Rect};

use crate::renderers::{BrailleRenderer, GradientRenderer, HalfBlockRenderer, SolidRenderer};
use crate::types::{ShadowConfig, ShadowStyle};

/// Render a shadow for an element at the given rect.
///
/// This is the main entry point for shadow rendering. It dispatches to the
/// appropriate renderer based on the style configured in `ShadowConfig`.
///
/// # Arguments
/// * `grid` - The grid to render into
/// * `element_rect` - The rect of the element casting the shadow
/// * `config` - Shadow configuration (style, offset, color, edges)
/// * `progress` - Animation progress 0.0-1.0 (for animated shadows)
///
/// # Example
///
/// ```
/// use tui_vfx_shadow::{render_shadow, ShadowConfig, ShadowEdges};
/// use tui_vfx_types::{Color, OwnedGrid, Rect};
///
/// let mut grid = OwnedGrid::new(40, 20);
/// let element_rect = Rect::new(10, 5, 15, 8);
/// let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
///     .with_offset(2, 1)
///     .with_edges(ShadowEdges::BOTTOM_RIGHT);
///
/// render_shadow(&mut grid, element_rect, &config, 1.0);
/// ```
pub fn render_shadow<G: Grid>(
    grid: &mut G,
    element_rect: Rect,
    config: &ShadowConfig,
    progress: f64,
) {
    match config.style {
        ShadowStyle::HalfBlock => {
            HalfBlockRenderer::render(grid, element_rect, config, progress);
        }
        ShadowStyle::Braille { density } => {
            BrailleRenderer::render(grid, element_rect, config, density, progress);
        }
        ShadowStyle::Solid => {
            SolidRenderer::render(grid, element_rect, config, progress);
        }
        ShadowStyle::Gradient { layers } => {
            GradientRenderer::render(grid, element_rect, config, layers, progress);
        }
    }
}

/// Render a shadow with default configuration.
///
/// Convenience function that creates a shadow with the given color and
/// default settings (HalfBlock style, offset (1,1), BOTTOM_RIGHT edges).
///
/// # Arguments
/// * `grid` - The grid to render into
/// * `element_rect` - The rect of the element casting the shadow
/// * `shadow_color` - The shadow color
/// * `surface_color` - Optional surface color for half-block blending
/// * `progress` - Animation progress 0.0-1.0
pub fn render_shadow_simple<G: Grid>(
    grid: &mut G,
    element_rect: Rect,
    shadow_color: tui_vfx_types::Color,
    surface_color: Option<tui_vfx_types::Color>,
    progress: f64,
) {
    let mut config = ShadowConfig::new(shadow_color);
    if let Some(surface) = surface_color {
        config = config.with_surface_color(surface);
    }
    render_shadow(grid, element_rect, &config, progress);
}

/// Render a gradient shadow using an array of distinct colors.
///
/// This function renders visible gradients in terminals that don't support
/// alpha blending by using distinct RGB colors from the theme's surface ladder.
///
/// # Arguments
/// * `grid` - The grid to render into
/// * `element_rect` - The rect of the element casting the shadow
/// * `config` - Shadow configuration (style field ignored, uses Gradient internally)
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
/// render_shadow_gradient_colors(&mut grid, rect, &config, &colors, 1.0);
/// ```
pub fn render_shadow_gradient_colors<G: Grid>(
    grid: &mut G,
    element_rect: Rect,
    config: &ShadowConfig,
    colors: &[tui_vfx_types::Color],
    progress: f64,
) {
    GradientRenderer::render_with_colors(grid, element_rect, config, colors, progress);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShadowEdges;
    use tui_vfx_types::{Color, OwnedGrid};

    #[test]
    fn test_render_shadow_half_block() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::HalfBlock)
            .with_offset(2, 1) // Use offset 2 to have both soft columns
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        // Verify shadow was rendered at right edge (x=15, x=16)
        // Col 1 (x=15): 25% shadow using ▊ with fg=surface, bg=shadow
        // Col 2 (x=16): 50% shadow using ▐ with fg=shadow, bg=surface
        let cell = grid.get(15, 3).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT); // First col: bg=shadow
        let cell = grid.get(16, 3).unwrap();
        assert_ne!(cell.fg, Color::TRANSPARENT); // Second col: fg=shadow
    }

    #[test]
    fn test_render_shadow_solid() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::Solid)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        let cell = grid.get(15, 3).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_render_shadow_braille() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::braille(0.7))
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        let cell = grid.get(15, 3).unwrap();
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn test_render_shadow_gradient() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::gradient(3))
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        let cell = grid.get(15, 3).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_render_shadow_simple() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);

        // render_shadow_simple uses HalfBlock with default offset (1,1) and soft edges
        // With offset=1, only first column exists: ▊ with fg=surface, bg=shadow
        render_shadow_simple(&mut grid, rect, Color::BLACK.with_alpha(128), None, 1.0);

        let cell = grid.get(15, 3).unwrap();
        // First column uses bg=shadow (▊ with inverted colors)
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }
}

// <FILE>crates/tui-vfx-shadow/src/fnc_render_shadow.rs</FILE> - <DESC>Main entry point for shadow rendering</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
