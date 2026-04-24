// <FILE>crates/tui-vfx-shadow/src/fnc_render_shadow.rs</FILE> - <DESC>Main entry point for shadow rendering (rect-based + role-aware SemanticScene entrypoints)</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Sub-plan A Phase A.3.5 — add `render_shadow_into_scene` role-aware wrapper that writes RoleTag::Shadow into the destination SemanticScene's RoleMap for every cell the shadow stage produces. Back-compat: source_region=None delegates directly to render_shadow on the destination grid. Source_region=Some(role) computes the role-filtered bounding rect via fnc_extract_shadow_envelope and uses it as the effective element rect.</WCTX>
// <CLOG>0.4.0: MINOR — add `render_shadow_into_scene` entrypoint for role-aware shadow stage. Writes `RoleTag::Shadow` into destination.roles_mut() for cells produced by the shadow render (measured by "was the destination cell empty before, and non-empty after"). When `config.source_region.is_some()` but no source cells match, no shadow is rendered.
// 0.3.0: Wire ShadowStyle::MediumShade to MediumShadeRenderer and add integration test.</CLOG>

//! Main entry point function for shadow rendering.
//!
//! Provides a unified API that dispatches to the appropriate renderer
//! based on the configured shadow style.

use tui_vfx_types::{Grid, Rect, RoleMap, RoleTag, SemanticScene};

use crate::fnc_extract_shadow_envelope::extract_shadow_envelope;
use crate::renderers::{
    BrailleRenderer, GradientRenderer, HalfBlockRenderer, MediumShadeRenderer, SolidRenderer,
};
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
        ShadowStyle::MediumShade => {
            MediumShadeRenderer::render(grid, element_rect, config, progress);
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

/// Render a shadow into a destination [`SemanticScene`], honouring
/// `config.source_region` for role-filtered extrusion and writing
/// `RoleTag::Shadow` into the destination's role map for every cell the
/// shadow stage produced.
///
/// This is the role-aware entrypoint added in Sub-plan A Phase A.3.5.
/// It wraps [`render_shadow`] with two additional behaviours:
///
/// 1. **Role-filtered extrusion.** When `config.source_region` is
///    `Some(role)`, [`crate::extract_shadow_envelope`] computes the tight
///    bounding rectangle of source cells whose role matches; that
///    rectangle is used as the effective `element_rect` instead of the
///    caller-supplied one. If no cells match, the shadow stage is a
///    no-op. When `config.source_region` is `None`, the caller-supplied
///    `element_rect` is used unchanged (back-compat).
/// 2. **Destination role-tag write-back.** Before rendering, the
///    destination grid is snapshotted per cell. After rendering, cells
///    whose content transitioned from "empty" (`' '` + transparent fg/bg)
///    to "non-empty" are tagged `RoleTag::Shadow` in
///    [`SemanticScene::roles_mut`]. This lets downstream trace consumers
///    and subsequent pipeline passes target shadow output by role.
///
/// # Stage ordering note
///
/// The per-cell compositor pipeline runs filters BEFORE shadow today.
/// That means a filter in the same frame cannot see the shadow tags this
/// function writes (spec §8.3 "downstream filter can target shadow cells"
/// refers to a subsequent frame or pipeline pass, or future reordering).
/// The tags are still immediately observable via
/// `destination.roles().get((x, y))` and in `tui-vfx-trace`.
///
/// # Example
///
/// ```
/// use tui_vfx_shadow::{render_shadow_into_scene, ShadowConfig, ShadowEdges};
/// use tui_vfx_types::{
///     Cell, Color, Grid, OwnedGrid, Rect, RoleMap, RoleTag, SemanticScene,
/// };
///
/// // Source: a 20×10 grid with an 8×4 card.
/// let element_rect = Rect::new(5, 2, 8, 4);
/// let mut source_grid = OwnedGrid::new(20, 10);
/// for y in element_rect.y..element_rect.y + element_rect.height {
///     for x in element_rect.x..element_rect.x + element_rect.width {
///         source_grid.set(x as usize, y as usize, Cell::new('X'));
///     }
/// }
/// let source_roles = RoleMap::empty(20, 10);
///
/// // Destination scene.
/// let mut scene = SemanticScene::from_grid_with_default_role(
///     OwnedGrid::new(20, 10),
///     RoleTag::Background,
/// );
///
/// // Shadow config.
/// let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
///     .with_offset(2, 1)
///     .with_edges(ShadowEdges::BOTTOM_RIGHT);
///
/// render_shadow_into_scene(
///     &source_grid, &source_roles, &mut scene, element_rect, &config, 1.0,
/// );
/// ```
pub fn render_shadow_into_scene<G: Grid + ?Sized>(
    source_grid: &G,
    source_roles: &RoleMap,
    destination: &mut SemanticScene,
    element_rect: Rect,
    config: &ShadowConfig,
    progress: f64,
) {
    let effective_rect = match &config.source_region {
        None => element_rect,
        Some(role) => {
            let envelope = extract_shadow_envelope(source_grid, source_roles, Some(role.clone()));
            match envelope.bounding_rect() {
                Some(rect) => rect,
                None => {
                    // No source cells matched — no shadow to render.
                    return;
                }
            }
        }
    };

    // Snapshot destination emptiness so we can identify cells the shadow
    // stage produced. A cell is "empty" iff glyph is ' ' AND both fg/bg
    // alpha are zero; anything else we treat as pre-existing content.
    let dest_w = destination.grid().width();
    let dest_h = destination.grid().height();
    let mut was_empty = vec![false; dest_w * dest_h];
    for y in 0..dest_h {
        for x in 0..dest_w {
            let empty = destination
                .grid()
                .get(x, y)
                .is_none_or(|c| c.ch == ' ' && c.bg.a == 0 && c.fg.a == 0);
            was_empty[y * dest_w + x] = empty;
        }
    }

    // Render the shadow into the destination's grid.
    render_shadow(destination.grid_mut(), effective_rect, config, progress);

    // Write-back RoleTag::Shadow for every cell that went empty → non-empty.
    for y in 0..dest_h {
        for x in 0..dest_w {
            if !was_empty[y * dest_w + x] {
                continue;
            }
            let produced = destination
                .grid()
                .get(x, y)
                .is_some_and(|c| c.ch != ' ' || c.bg.a != 0 || c.fg.a != 0);
            if produced {
                destination
                    .roles_mut()
                    .set((x as u16, y as u16), RoleTag::Shadow);
            }
        }
    }
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
        // start_y = rect_y + oy + 1 = 2 + 1 + 1 = 4 (inset)
        // Col 1 (x=15): 50% shadow using ▐ with fg=shadow, bg=surface
        // Col 2 (x=16): 50% shadow using ▌ with fg=shadow, bg=surface
        let cell = grid.get(15, 4).unwrap();
        assert_ne!(cell.fg, Color::TRANSPARENT); // First col: fg=shadow
        let cell = grid.get(16, 4).unwrap();
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

        let cell = grid.get(15, 4).unwrap();
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

        let cell = grid.get(15, 4).unwrap();
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn test_render_shadow_medium_shade() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::MediumShade)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        let cell = grid.get(15, 4).unwrap();
        assert_eq!(cell.ch, '▒');
        assert_ne!(cell.fg, Color::TRANSPARENT);
    }

    #[test]
    fn test_render_shadow_gradient() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_style(ShadowStyle::gradient(3))
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        render_shadow(&mut grid, rect, &config, 1.0);

        let cell = grid.get(15, 4).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_render_shadow_simple() {
        let mut grid = OwnedGrid::new(30, 15);
        let rect = Rect::new(5, 2, 10, 6);

        // render_shadow_simple uses HalfBlock with default offset (1,1) and soft edges
        // With offset=1, only first column exists: ▐ with fg=shadow, bg=surface
        render_shadow_simple(&mut grid, rect, Color::BLACK.with_alpha(128), None, 1.0);

        let cell = grid.get(15, 4).unwrap();
        // First column uses fg=shadow (▐ with standard fg=shadow,bg=surface)
        assert_ne!(cell.fg, Color::TRANSPARENT);
    }
}

// <FILE>crates/tui-vfx-shadow/src/fnc_render_shadow.rs</FILE> - <DESC>Main entry point for shadow rendering</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
