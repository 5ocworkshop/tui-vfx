// <FILE>crates/tui-vfx-shadow/src/renderers/cls_medium_shade.rs</FILE> - <DESC>Medium shade character shadow renderer</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Add +1 inset to right-edge shadow start_y for grade-underlying visual weight</WCTX>
// <CLOG>+1 inset on both right-edge start_y and bottom-edge start_x for grade-underlying visual weight</CLOG>

//! Medium shade character shadow renderer.
//!
//! Renders shadows using Unicode medium shade (`▒`) glyphs, which gives
//! a dense but still textured appearance compared to solid background fills.

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Unicode medium shade character (U+2592).
const MEDIUM_SHADE_CHAR: char = '▒';

/// Medium shade shadow renderer.
pub struct MediumShadeRenderer;

impl MediumShadeRenderer {
    /// Render a medium-shade shadow for the given element rect.
    ///
    /// # Arguments
    /// * `grid` - The grid to render into
    /// * `element_rect` - The rect of the element casting the shadow
    /// * `config` - Shadow configuration
    /// * `progress` - Animation progress 0.0-1.0
    pub fn render<G: Grid>(grid: &mut G, element_rect: Rect, config: &ShadowConfig, progress: f64) {
        let shadow_color = config.color_at_progress(progress);
        if shadow_color.a == 0 {
            return;
        }

        let surface = config.surface_color.unwrap_or(Color::TRANSPARENT);

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let edges = config.edges;

        // Right edge shadow
        if edges.has_right() && ox > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            // +1 inset: start shadow 1 row below element top for grade-underlying visual weight
            // TODO: plumb inset_x/inset_y through ShadowConfig when tunability is needed
            let start_y = (rect_y + oy.max(0) + 1).max(0) as usize;
            let w = ox as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color, surface);
        }

        // Bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            // +1 inset: start shadow 1 col right of element left for grade-underlying visual weight
            // TODO: plumb inset_x/inset_y through ShadowConfig when tunability is needed
            let start_x = (rect_x + ox.max(0) + 1).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = oy as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color, surface);
        }

        // Left edge shadow
        if edges.has_left() && ox < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let w = (-ox) as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color, surface);
        }

        // Top edge shadow
        if edges.has_top() && oy < 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = (-oy) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color, surface);
        }

        // Corner regions
        if edges.has_right() && edges.has_bottom() && ox > 0 && oy > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            Self::fill_region(
                grid,
                start_x,
                start_y,
                ox as usize,
                oy as usize,
                shadow_color,
                surface,
            );
        }

        if edges.has_left() && edges.has_top() && ox < 0 && oy < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;
            Self::fill_region(
                grid,
                start_x,
                start_y,
                (-ox) as usize,
                (-oy) as usize,
                shadow_color,
                surface,
            );
        }
    }

    /// Fill a rectangular region with medium-shade character cells.
    fn fill_region<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        fg: Color,
        bg: Color,
    ) {
        let cell = Cell::new(MEDIUM_SHADE_CHAR)
            .with_fg(fg)
            .with_bg(bg)
            .with_mod_alpha(Some(255));
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
    fn test_render_basic_shadow() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_offset(2, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        MediumShadeRenderer::render(&mut grid, rect, &config, 1.0);

        let right_edge_cell = grid.get(13, 4).unwrap();
        assert_eq!(right_edge_cell.ch, MEDIUM_SHADE_CHAR);
        assert_ne!(right_edge_cell.fg, Color::TRANSPARENT);

        let bottom_edge_cell = grid.get(8, 6).unwrap();
        assert_eq!(bottom_edge_cell.ch, MEDIUM_SHADE_CHAR);
        assert_ne!(bottom_edge_cell.fg, Color::TRANSPARENT);
    }

    #[test]
    fn test_zero_alpha_renders_nothing() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(0));

        MediumShadeRenderer::render(&mut grid, rect, &config, 1.0);

        for y in 0..10 {
            for x in 0..20 {
                let cell = grid.get(x, y).unwrap();
                assert_eq!(cell.bg, Color::TRANSPARENT);
                assert_eq!(cell.fg, Color::TRANSPARENT);
            }
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/renderers/cls_medium_shade.rs</FILE> - <DESC>Medium shade character shadow renderer</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
