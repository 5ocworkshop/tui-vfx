// <FILE>crates/tui-vfx-shadow/src/renderers/cls_half_block.rs</FILE> - <DESC>Half-block shadow renderer for sub-cell precision</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Honor explicit shared shadow inset controls so GTD can keep single-cell shadow spans while starting horizontal and vertical edges at different insets</WCTX>
// <CLOG>Add trailing inset support for centered bottom/top and side shadow runs.</CLOG>

//! Half-block shadow renderer using Unicode block characters.
//!
//! Uses characters like ▐ (right half), ▄ (lower half), ▌ (left half), ▀ (upper half)
//! for sub-cell precision shadow rendering.

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Half-block characters for sub-cell shadow rendering.
pub struct HalfBlockRenderer;

/// Right half block: ▐ (U+2590)
const RIGHT_HALF: char = '▐';
/// Lower half block: ▄ (U+2584)
const LOWER_HALF: char = '▄';
/// Left half block: ▌ (U+258C)
const LEFT_HALF: char = '▌';
/// Upper half block: ▀ (U+2580)
const UPPER_HALF: char = '▀';
/// Three-quarter block: ▙ (U+2599) - upper-left + lower-left + lower-right quadrants
/// Used for corner cell (dx=1, dy=0) to join left-shadow (upper-left) with bottom edge (lower)
#[allow(dead_code)]
const THREE_QUARTER_BLOCK: char = '▙';
/// Quadrant lower-left + lower-right + upper-right: ▟ (U+259F)
/// Used for corner cell (dx=0, dy=0) to continue vertical stripe (UR) into bottom edge (LL+LR)
const QUADRANT_LL_LR_UR: char = '▟';

impl HalfBlockRenderer {
    /// Render a half-block shadow for the given element rect.
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

        // Use TRANSPARENT for surface portions to enable compositor alpha blending.
        // The compositor will blend these transparent portions with the actual
        // underlying content, allowing text and colors to show through softly.
        let surface = Color::TRANSPARENT;

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let horizontal_span = config.horizontal_shadow_span(rect_x, rect_w, ox);
        let vertical_span = config.vertical_shadow_span(rect_y, rect_h, oy);

        let edges = config.edges;

        // Render right edge shadow
        if edges.has_right() && ox > 0 {
            Self::render_right_edge(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                ox,
                oy,
                vertical_span,
                shadow_color,
                surface,
                config.soft_edges,
            );
        }

        // Render bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            Self::render_bottom_edge(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                ox,
                oy,
                horizontal_span,
                shadow_color,
                surface,
                config.soft_edges,
            );
        }

        // Render left edge shadow
        if edges.has_left() && ox < 0 {
            Self::render_left_edge(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                ox,
                oy,
                vertical_span,
                shadow_color,
                surface,
                config.soft_edges,
            );
        }

        // Render top edge shadow
        if edges.has_top() && oy < 0 {
            Self::render_top_edge(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                ox,
                oy,
                horizontal_span,
                shadow_color,
                surface,
                config.soft_edges,
            );
        }

        // Render corner if both adjacent edges are enabled
        if edges.has_right() && edges.has_bottom() && ox > 0 && oy > 0 {
            Self::render_corner(
                grid,
                rect_x,
                rect_y,
                rect_w,
                rect_h,
                ox,
                oy,
                shadow_color,
                surface,
                config.soft_edges,
            );
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_right_edge<G: Grid>(
        grid: &mut G,
        rect_x: i32,
        _rect_y: i32,
        rect_w: i32,
        _rect_h: i32,
        ox: i32,
        _oy: i32,
        span_y: (usize, usize),
        shadow: Color,
        surface: Color,
        soft: bool,
    ) {
        let start_x = (rect_x + rect_w).max(0) as usize;
        let end_x = (rect_x + rect_w + ox).max(0) as usize;
        let (start_y, end_y) = span_y;

        for y in start_y..end_y {
            for x in start_x..end_x {
                if grid.in_bounds(x, y) {
                    let cell = if soft && x == start_x {
                        // First column: 50% shadow using right half block
                        // ▐ with fg=shadow,bg=surface shows 50% surface (left) + 50% shadow (right)
                        // Maintains fg=shadow,bg=surface convention for compositor consistency
                        shadow_cell(Cell::new(RIGHT_HALF).with_fg(shadow).with_bg(surface))
                    } else if soft && x == start_x + 1 {
                        // Second column: left half block (50% shadow on LEFT half)
                        // Shadow connects with first column's 25% shadow
                        shadow_cell(Cell::new(LEFT_HALF).with_fg(shadow).with_bg(surface))
                    } else {
                        // Additional columns: solid shadow cell
                        shadow_cell(Cell::new(' ').with_bg(shadow))
                    };
                    grid.set(x, y, cell);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_bottom_edge<G: Grid>(
        grid: &mut G,
        _rect_x: i32,
        rect_y: i32,
        _rect_w: i32,
        rect_h: i32,
        _ox: i32,
        oy: i32,
        span_x: (usize, usize),
        shadow: Color,
        surface: Color,
        soft: bool,
    ) {
        let (start_x, end_x) = span_x;
        let start_y = (rect_y + rect_h).max(0) as usize;
        let end_y = (rect_y + rect_h + oy).max(0) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                if grid.in_bounds(x, y) {
                    let cell = if soft && y == start_y {
                        // Soft edge: use lower half block (shadow on bottom half of cell)
                        shadow_cell(Cell::new(LOWER_HALF).with_fg(shadow).with_bg(surface))
                    } else {
                        // Solid shadow cell
                        shadow_cell(Cell::new(' ').with_bg(shadow))
                    };
                    grid.set(x, y, cell);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_left_edge<G: Grid>(
        grid: &mut G,
        rect_x: i32,
        _rect_y: i32,
        _rect_w: i32,
        _rect_h: i32,
        ox: i32,
        _oy: i32,
        span_y: (usize, usize),
        shadow: Color,
        surface: Color,
        soft: bool,
    ) {
        let start_x = (rect_x + ox).max(0) as usize;
        let end_x = rect_x.max(0) as usize;
        let (start_y, end_y) = span_y;

        for y in start_y..end_y {
            for x in start_x..end_x {
                if grid.in_bounds(x, y) {
                    let cell = if soft && x == end_x.saturating_sub(1) {
                        // Soft edge: use left half block (shadow on left half of cell)
                        shadow_cell(Cell::new(LEFT_HALF).with_fg(shadow).with_bg(surface))
                    } else {
                        shadow_cell(Cell::new(' ').with_bg(shadow))
                    };
                    grid.set(x, y, cell);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_top_edge<G: Grid>(
        grid: &mut G,
        _rect_x: i32,
        rect_y: i32,
        _rect_w: i32,
        _rect_h: i32,
        _ox: i32,
        oy: i32,
        span_x: (usize, usize),
        shadow: Color,
        surface: Color,
        soft: bool,
    ) {
        let (start_x, end_x) = span_x;
        let start_y = (rect_y + oy).max(0) as usize;
        let end_y = rect_y.max(0) as usize;

        for y in start_y..end_y {
            for x in start_x..end_x {
                if grid.in_bounds(x, y) {
                    let cell = if soft && y == end_y.saturating_sub(1) {
                        // Soft edge: use upper half block (shadow on top half of cell)
                        shadow_cell(Cell::new(UPPER_HALF).with_fg(shadow).with_bg(surface))
                    } else {
                        shadow_cell(Cell::new(' ').with_bg(shadow))
                    };
                    grid.set(x, y, cell);
                }
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn render_corner<G: Grid>(
        grid: &mut G,
        rect_x: i32,
        rect_y: i32,
        rect_w: i32,
        rect_h: i32,
        ox: i32,
        oy: i32,
        shadow: Color,
        surface: Color,
        soft: bool,
    ) {
        // Bottom-right corner
        let corner_x = (rect_x + rect_w).max(0) as usize;
        let corner_y = (rect_y + rect_h).max(0) as usize;

        for dy in 0..oy as usize {
            for dx in 0..ox as usize {
                let x = corner_x + dx;
                let y = corner_y + dy;
                if grid.in_bounds(x, y) {
                    let cell = if soft && dx == 0 && dy == 0 {
                        // First column, first row: continue vertical stripe + bottom edge
                        // ▟ fills LL+LR+UR: shadow in bottom + upper-right (continues 25% stripe)
                        // This balances with ▙ at dx=1 to avoid visual notch/protrusion
                        shadow_cell(
                            Cell::new(QUADRANT_LL_LR_UR)
                                .with_fg(shadow)
                                .with_bg(surface),
                        )
                    } else if soft && dx == 0 {
                        // First column, other rows: 50% shadow (matches right edge)
                        shadow_cell(Cell::new(RIGHT_HALF).with_fg(shadow).with_bg(surface))
                    } else if soft && dx == 1 && dy == 0 {
                        // Second column, first row: continue vertical edge only (hard 90 corner)
                        // ▌ continues the left-half pattern from right edge col2, no horizontal extension
                        shadow_cell(Cell::new(LEFT_HALF).with_fg(shadow).with_bg(surface))
                    } else if soft && dx == 1 {
                        // Second column, other rows: left half to match right edge
                        shadow_cell(Cell::new(LEFT_HALF).with_fg(shadow).with_bg(surface))
                    } else {
                        shadow_cell(Cell::new(' ').with_bg(shadow))
                    };
                    grid.set(x, y, cell);
                }
            }
        }
    }
}

/// Apply mod_alpha=255 to shadow cells.
/// This prevents modifier bleed from underlying content through semi-transparent shadows.
#[inline]
fn shadow_cell(cell: Cell) -> Cell {
    cell.with_mod_alpha(Some(255))
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
            .with_offset(1, 1)
            .with_inset(2, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        HalfBlockRenderer::render(&mut grid, rect, &config, 1.0);

        // Right edge shadow uses a single column and starts one row below the top edge.
        let cell = grid.get(13, 4).unwrap();
        assert_eq!(cell.ch, RIGHT_HALF);
        assert_ne!(cell.fg, Color::TRANSPARENT);
        assert_eq!(grid.get(14, 4).unwrap().fg, Color::TRANSPARENT);

        // Bottom edge shadow begins two columns in from the left edge.
        let cell = grid.get(7, 6).unwrap();
        assert_eq!(cell.ch, LOWER_HALF);
        assert_ne!(cell.fg, Color::TRANSPARENT);
        assert_eq!(grid.get(6, 6).unwrap().fg, Color::TRANSPARENT);
    }

    #[test]
    fn test_zero_progress_renders_nothing() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK);

        HalfBlockRenderer::render(&mut grid, rect, &config, 0.0);

        // All cells should be default (transparent)
        for y in 0..10 {
            for x in 0..20 {
                let cell = grid.get(x, y).unwrap();
                assert_eq!(cell.bg, Color::TRANSPARENT);
            }
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/renderers/cls_half_block.rs</FILE> - <DESC>Half-block shadow renderer for sub-cell precision</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
