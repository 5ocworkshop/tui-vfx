// <FILE>crates/tui-vfx-shadow/src/renderers/cls_solid.rs</FILE> - <DESC>Solid color shadow renderer</DESC>
// <VERS>VERSION: 0.7.0</VERS>
// <WCTX>Honor explicit shared shadow inset controls so GTD can keep single-cell shadow spans while starting horizontal and vertical edges at different insets</WCTX>
// <CLOG>Add optional 3/4-style foreground coverage for side shadow columns.</CLOG>

//! Solid color shadow renderer.
//!
//! The simplest shadow style - fills cells with solid background color by
//! default. Optional side coverage can render left/right columns with block
//! fraction foreground glyphs over transparent backgrounds for optical tuning.

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Solid color shadow renderer.
///
/// Renders shadows as simple background-colored cells with space characters.
/// This is the most compatible approach but offers no sub-cell precision.
pub struct SolidRenderer;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SideEdge {
    Left,
    Right,
}

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
            let (start_y, end_y) = config.vertical_shadow_span(rect_y, rect_h, oy);
            let w = ox as usize;
            let h = end_y.saturating_sub(start_y);
            Self::fill_side_region(
                grid,
                start_x,
                start_y,
                w,
                h,
                shadow_color,
                config.falloff_y.unwrap_or(0) as usize,
                SideEdge::Right,
                config.side_coverage_eighths,
            );
        }

        // Bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            let (start_x, end_x) = config.horizontal_shadow_span(rect_x, rect_w, ox);
            let start_y = (rect_y + rect_h).max(0) as usize;
            let w = end_x.saturating_sub(start_x);
            let h = oy as usize;
            Self::fill_region_horizontal_falloff(
                grid,
                start_x,
                start_y,
                w,
                h,
                shadow_color,
                config.falloff_x.unwrap_or(0) as usize,
            );
        }

        // Left edge shadow
        if edges.has_left() && ox < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let (start_y, end_y) = config.vertical_shadow_span(rect_y, rect_h, oy);
            let w = (-ox) as usize;
            let h = end_y.saturating_sub(start_y);
            Self::fill_side_region(
                grid,
                start_x,
                start_y,
                w,
                h,
                shadow_color,
                config.falloff_y.unwrap_or(0) as usize,
                SideEdge::Left,
                config.side_coverage_eighths,
            );
        }

        // Top edge shadow
        if edges.has_top() && oy < 0 {
            let (start_x, end_x) = config.horizontal_shadow_span(rect_x, rect_w, ox);
            let start_y = (rect_y + oy).max(0) as usize;
            let w = end_x.saturating_sub(start_x);
            let h = (-oy) as usize;
            Self::fill_region_horizontal_falloff(
                grid,
                start_x,
                start_y,
                w,
                h,
                shadow_color,
                config.falloff_x.unwrap_or(0) as usize,
            );
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
        Self::fill_region_with_alpha_at(grid, x, y, w, h, |_, _| color);
    }

    /// Fill a horizontal run with transparent alpha falloff at its left/right ends.
    fn fill_region_horizontal_falloff<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
        falloff: usize,
    ) {
        Self::fill_region_with_alpha_at(grid, x, y, w, h, |dx, _| {
            color.with_alpha(falloff_alpha(color.a, dx, w, falloff))
        });
    }

    /// Fill a vertical run with transparent alpha falloff at its top/bottom ends.
    fn fill_region_vertical_falloff<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
        falloff: usize,
    ) {
        Self::fill_region_with_alpha_at(grid, x, y, w, h, |_, dy| {
            color.with_alpha(falloff_alpha(color.a, dy, h, falloff))
        });
    }

    /// Fill a vertical side run, optionally using sub-cell foreground coverage.
    fn fill_side_region<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        color: Color,
        falloff: usize,
        side: SideEdge,
        coverage_eighths: Option<u8>,
    ) {
        let Some(coverage) = coverage_eighths.map(|value| value.clamp(0, 8)) else {
            Self::fill_region_vertical_falloff(grid, x, y, w, h, color, falloff);
            return;
        };

        Self::fill_region_with_cell_at(grid, x, y, w, h, |_, dy| {
            let color = color.with_alpha(falloff_alpha(color.a, dy, h, falloff));
            side_coverage_cell(side, coverage, color)
        });
    }

    fn fill_region_with_alpha_at<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        mut color_at: impl FnMut(usize, usize) -> Color,
    ) {
        Self::fill_region_with_cell_at(grid, x, y, w, h, |dx, dy| {
            let color = color_at(dx, dy);
            Cell::new(' ').with_bg(color).with_mod_alpha(Some(255))
        });
    }

    fn fill_region_with_cell_at<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        mut cell_at: impl FnMut(usize, usize) -> Cell,
    ) {
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if grid.in_bounds(px, py) {
                    grid.set(px, py, cell_at(dx, dy));
                }
            }
        }
    }
}

const LEFT_BLOCKS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];

#[inline]
fn side_coverage_cell(side: SideEdge, coverage: u8, color: Color) -> Cell {
    if color.a == 0 || coverage == 0 {
        return Cell::default();
    }

    if coverage >= 8 {
        return Cell::new(' ').with_bg(color).with_mod_alpha(Some(255));
    }

    let ch = match side {
        SideEdge::Right => LEFT_BLOCKS[coverage as usize],
        SideEdge::Left => right_block(coverage),
    };

    Cell::new(ch)
        .with_fg(color)
        .with_bg(Color::TRANSPARENT)
        .with_mod_alpha(Some(255))
}

#[inline]
fn right_block(coverage: u8) -> char {
    match coverage {
        0 => ' ',
        1 => '▕',
        2 => '\u{1FB87}',
        3 => '\u{1FB88}',
        4 => '▐',
        5 => '\u{1FB89}',
        6 => '\u{1FB8A}',
        7 => '\u{1FB8B}',
        _ => '█',
    }
}

#[inline]
fn falloff_alpha(alpha: u8, pos: usize, len: usize, falloff: usize) -> u8 {
    if alpha == 0 || falloff == 0 || len == 0 {
        return alpha;
    }
    let end_pos = len.saturating_sub(1).saturating_sub(pos);
    let distance = pos.min(end_pos);
    if distance >= falloff {
        return alpha;
    }
    let numerator = (distance + 1) as f32;
    let denominator = (falloff + 1) as f32;
    (alpha as f32 * (numerator / denominator)).round() as u8
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
            .with_style(crate::types::ShadowStyle::Solid)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        // Right edge shadow uses a single column and starts one row below the top edge.
        let cell = grid.get(13, 4).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
        assert_eq!(cell.ch, ' ');
        assert_eq!(grid.get(14, 4).unwrap().bg, Color::TRANSPARENT);

        // Bottom edge shadow begins two columns in from the left edge.
        let cell = grid.get(7, 6).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
        assert_eq!(grid.get(6, 6).unwrap().bg, Color::TRANSPARENT);

        // Corner shadow remains at the bottom-right corner.
        let cell = grid.get(13, 6).unwrap();
        assert_ne!(cell.bg, Color::TRANSPARENT);
    }

    #[test]
    fn bottom_only_shadow_can_be_centered_with_symmetric_horizontal_inset() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(4, 2, 10, 3);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_offset(0, 1)
            .with_symmetric_inset(2, 0)
            .with_edges(ShadowEdges::BOTTOM);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        assert_eq!(grid.get(4, 5).unwrap().bg, Color::TRANSPARENT);
        assert_eq!(grid.get(5, 5).unwrap().bg, Color::TRANSPARENT);
        assert_ne!(grid.get(6, 5).unwrap().bg, Color::TRANSPARENT);
        assert_ne!(grid.get(11, 5).unwrap().bg, Color::TRANSPARENT);
        assert_eq!(grid.get(12, 5).unwrap().bg, Color::TRANSPARENT);
        assert_eq!(grid.get(13, 5).unwrap().bg, Color::TRANSPARENT);
        assert_eq!(grid.get(14, 5).unwrap().bg, Color::TRANSPARENT);
    }

    #[test]
    fn bottom_shadow_falloff_reduces_alpha_at_horizontal_ends() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(4, 2, 10, 3);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(210))
            .with_offset(0, 1)
            .with_symmetric_inset(1, 0)
            .with_falloff(2, 0)
            .with_edges(ShadowEdges::BOTTOM);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        assert_eq!(grid.get(5, 5).unwrap().bg.a, 70);
        assert_eq!(grid.get(6, 5).unwrap().bg.a, 140);
        assert_eq!(grid.get(7, 5).unwrap().bg.a, 210);
        assert_eq!(grid.get(11, 5).unwrap().bg.a, 140);
        assert_eq!(grid.get(12, 5).unwrap().bg.a, 70);
    }

    #[test]
    fn side_shadow_falloff_reduces_alpha_at_vertical_ends() {
        let mut grid = OwnedGrid::new(20, 12);
        let rect = Rect::new(4, 2, 8, 7);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
            .with_offset(1, 0)
            .with_symmetric_inset(0, 1)
            .with_falloff(0, 2)
            .with_edges(ShadowEdges::RIGHT);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        assert_eq!(grid.get(12, 3).unwrap().bg.a, 60);
        assert_eq!(grid.get(12, 4).unwrap().bg.a, 120);
        assert_eq!(grid.get(12, 5).unwrap().bg.a, 180);
        assert_eq!(grid.get(12, 6).unwrap().bg.a, 120);
        assert_eq!(grid.get(12, 7).unwrap().bg.a, 60);
    }

    #[test]
    fn right_side_shadow_can_use_three_quarter_foreground_coverage() {
        let mut grid = OwnedGrid::new(20, 12);
        let rect = Rect::new(4, 2, 8, 7);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
            .with_offset(1, 0)
            .with_symmetric_inset(0, 1)
            .with_falloff(0, 2)
            .with_side_coverage(6)
            .with_edges(ShadowEdges::RIGHT);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        let edge = grid.get(12, 5).unwrap();
        assert_eq!(edge.ch, '▊');
        assert_eq!(edge.fg.a, 180);
        assert_eq!(edge.bg, Color::TRANSPARENT);

        let falloff = grid.get(12, 3).unwrap();
        assert_eq!(falloff.ch, '▊');
        assert_eq!(falloff.fg.a, 60);
        assert_eq!(falloff.bg, Color::TRANSPARENT);
    }

    #[test]
    fn left_side_shadow_can_use_three_quarter_foreground_coverage() {
        let mut grid = OwnedGrid::new(20, 12);
        let rect = Rect::new(7, 2, 8, 7);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(180))
            .with_offset(-1, 0)
            .with_symmetric_inset(0, 1)
            .with_side_coverage(6)
            .with_edges(ShadowEdges::LEFT);

        SolidRenderer::render(&mut grid, rect, &config, 1.0);

        let edge = grid.get(6, 5).unwrap();
        assert_eq!(edge.ch, '\u{1FB8A}');
        assert_eq!(edge.fg.a, 180);
        assert_eq!(edge.bg, Color::TRANSPARENT);
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
// <VERS>END OF VERSION: 0.7.0</VERS>
