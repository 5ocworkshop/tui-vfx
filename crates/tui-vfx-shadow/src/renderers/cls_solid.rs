// <FILE>crates/tui-vfx-shadow/src/renderers/cls_solid.rs</FILE> - <DESC>Solid color shadow renderer</DESC>
// <VERS>VERSION: 0.3.1</VERS>
// <WCTX>Add +1 inset to right-edge shadow start_y for grade-underlying visual weight</WCTX>
// <CLOG>+1 inset on both right-edge start_y and bottom-edge start_x for grade-underlying visual weight</CLOG>

//! Solid color shadow renderer.
//!
//! The simplest shadow style - fills cells with solid background color.
//! Maximum compatibility across all terminal fonts.

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Solid color shadow renderer.
///
/// Renders shadows as simple background-colored cells with space characters.
/// This is the most compatible approach but offers no sub-cell precision.
pub struct SolidRenderer;

impl SolidRenderer {
    /// Render a solid shadow for the given element rect.
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

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let edges = config.edges;

        // Calculate shadow regions based on offset direction and enabled edges

        // Right edge shadow
        if edges.has_right() && ox > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            // +1 inset: start shadow 1 row below element top for grade-underlying visual weight
            // TODO: plumb inset_x/inset_y through ShadowConfig when tunability is needed
            let start_y = (rect_y + oy.max(0) + 1).max(0) as usize;
            let w = ox as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color);
        }

        // Bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            // +1 inset: start shadow 1 col right of element left for grade-underlying visual weight
            // TODO: plumb inset_x/inset_y through ShadowConfig when tunability is needed
            let start_x = (rect_x + ox.max(0) + 1).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = oy as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color);
        }

        // Left edge shadow
        if edges.has_left() && ox < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let w = (-ox) as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color);
        }

        // Top edge shadow
        if edges.has_top() && oy < 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = (-oy) as usize;
            Self::fill_region(grid, start_x, start_y, w, h, shadow_color);
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
            );
        }
    }

    /// Fill a rectangular region with the shadow color.
    fn fill_region<G: Grid>(grid: &mut G, x: usize, y: usize, w: usize, h: usize, color: Color) {
        let cell = Cell::new(' ').with_bg(color).with_mod_alpha(Some(255));
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
        let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
            .with_offset(2, 1)
            .with_style(crate::types::ShadowStyle::Solid)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        // Check that shadow exists at expected positions
        // Right edge shadow: x=13-14, y=4-5 (start_y = rect_y + oy + 1 = 2+1+1 = 4, inset)
        let cell = grid.get(13, 4).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
        assert_eq!(cell.ch, ' ');

        // Bottom edge shadow: x=8-12, y=6 (start_x = rect_x + ox + 1 = 5+2+1 = 8, inset)
        let cell = grid.get(8, 6).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);

        // Corner shadow at x=13-14, y=6
        let cell = grid.get(13, 6).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn test_zero_alpha_renders_nothing() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(0));

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        // All cells should be default (transparent)
        for y in 0..10 {
            for x in 0..20 {
                let cell = grid.get(x, y).unwrap();
                assert_eq!(cell.bg, Color::TRANSPARENT);
            }
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/renderers/cls_solid.rs</FILE> - <DESC>Solid color shadow renderer</DESC>
// <VERS>END OF VERSION: 0.3.1</VERS>
