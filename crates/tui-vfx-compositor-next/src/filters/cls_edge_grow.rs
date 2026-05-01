// <FILE>tui-vfx-compositor-next/src/filters/cls_edge_grow.rs</FILE>
// <DESC>Generalized edge-growth indicator filter using sub-cell partial blocks</DESC>
// <VERS>VERSION: 1.0.1</VERS>
// <WCTX>Slice 6.6 §F.5 — migrate Filter trait to VfxCellContext bundle</WCTX>
// <CLOG>1.0.1: migrate apply signature to &VfxCellContext.</CLOG>

use crate::traits::filter::Filter;
use crate::types::HoverBarPosition;
use tui_vfx_types::{Cell, Color, VfxCellContext};

/// Generalized progress-driven edge growth effect.
///
/// `EdgeGrow` expands a bar or fill region away from a content edge using sub-cell block
/// glyphs. It generalizes left/right/top/bottom hover bars into one filter that can drive
/// button hovers, focus rails, bottom tabs, and expanding gutters.
pub struct EdgeGrow {
    /// Width at rest in eighths.
    pub rest_eighths: u8,
    /// Width at full progress in eighths.
    pub peak_eighths: u8,
    /// Which edge the growth originates from.
    pub edge: HoverBarPosition,
    /// Fill color.
    pub fill_color: Color,
    /// Background color behind the fill.
    pub bg_color: Color,
    /// Progress (0.0-1.0).
    pub progress: f32,
    /// Margin width available on the active side.
    pub margin_width: u8,
}

impl Default for EdgeGrow {
    fn default() -> Self {
        Self {
            rest_eighths: 2,
            peak_eighths: 12,
            edge: HoverBarPosition::Left,
            fill_color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
            progress: 0.0,
            margin_width: 2,
        }
    }
}

impl EdgeGrow {
    pub fn from_hover_bar(
        base_eighths: u8,
        max_eighths: u8,
        position: HoverBarPosition,
        fill_color: Color,
        bg_color: Color,
        progress: f32,
        margin_width: u8,
    ) -> Self {
        Self {
            rest_eighths: base_eighths,
            peak_eighths: max_eighths,
            edge: position,
            fill_color,
            bg_color,
            progress,
            margin_width,
        }
    }

    fn current_eighths(&self) -> usize {
        let range = self.peak_eighths.saturating_sub(self.rest_eighths) as f32;
        let delta = (range * self.progress.clamp(0.0, 1.0)).round() as u8;
        self.rest_eighths.saturating_add(delta) as usize
    }

    fn left_block(eighths: usize) -> char {
        const BLOCKS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        BLOCKS[eighths.min(8)]
    }

    fn lower_block(eighths: usize) -> char {
        const BLOCKS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        BLOCKS[eighths.min(8)]
    }

    fn margin_index(&self, x: u16, y: u16, width: u16, height: u16) -> Option<usize> {
        let margin = self.margin_width as u16;
        match self.edge {
            HoverBarPosition::Left => {
                if x < margin {
                    Some((margin - 1 - x) as usize)
                } else {
                    None
                }
            }
            HoverBarPosition::Right => {
                let start = width.saturating_sub(margin);
                if x >= start {
                    Some((x - start) as usize)
                } else {
                    None
                }
            }
            HoverBarPosition::Top => {
                if y < margin {
                    Some((margin - 1 - y) as usize)
                } else {
                    None
                }
            }
            HoverBarPosition::Bottom => {
                let start = height.saturating_sub(margin);
                if y >= start {
                    Some((y - start) as usize)
                } else {
                    None
                }
            }
        }
    }

    fn cell_fill_eighths(&self, margin_idx: usize) -> usize {
        let total = self.current_eighths();
        total.saturating_sub(margin_idx * 8).min(8)
    }

    fn apply_left_or_bottom(&self, cell: &mut Cell, fill: usize, horizontal: bool) {
        if fill >= 8 {
            cell.ch = '█';
            cell.fg = self.fill_color;
            cell.bg = self.fill_color;
        } else if fill > 0 {
            let inverse = 8 - fill;
            cell.ch = if horizontal {
                Self::lower_block(inverse)
            } else {
                Self::left_block(inverse)
            };
            cell.fg = self.bg_color;
            cell.bg = self.fill_color;
        } else {
            cell.ch = ' ';
            cell.bg = self.bg_color;
        }
    }

    fn apply_right_or_top(&self, cell: &mut Cell, fill: usize, horizontal: bool) {
        if fill >= 8 {
            cell.ch = '█';
            cell.fg = self.fill_color;
            cell.bg = self.fill_color;
        } else if fill > 0 {
            cell.ch = if horizontal {
                Self::lower_block(fill)
            } else {
                Self::left_block(fill)
            };
            cell.fg = self.fill_color;
            cell.bg = self.bg_color;
        } else {
            cell.ch = ' ';
            cell.bg = self.bg_color;
        }
    }
}

impl Filter for EdgeGrow {
    fn apply(&self, cell: &mut Cell, ctx: &VfxCellContext) {
        let x = ctx.local_x;
        let y = ctx.local_y;
        let width = ctx.width;
        let height = ctx.height;
        let Some(margin_idx) = self.margin_index(x, y, width, height) else {
            return;
        };
        let fill = self.cell_fill_eighths(margin_idx);
        match self.edge {
            HoverBarPosition::Left => self.apply_left_or_bottom(cell, fill, false),
            HoverBarPosition::Right => self.apply_right_or_top(cell, fill, false),
            HoverBarPosition::Top => self.apply_right_or_top(cell, fill, true),
            HoverBarPosition::Bottom => self.apply_left_or_bottom(cell, fill, true),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn default_cell() -> Cell {
        Cell::default()
    }

    #[test]
    fn right_edge_grows_from_content_outward() {
        let filter = EdgeGrow::from_hover_bar(
            0,
            12,
            HoverBarPosition::Right,
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 0),
            1.0,
            2,
        );
        let mut near = default_cell();
        filter.apply(&mut near, &VfxCellContext::new(8, 0, 10, 1, 0, 0, 0.0));
        assert_eq!(near.ch, '█');
    }

    #[test]
    fn left_edge_uses_inverse_partial_blocks() {
        let filter = EdgeGrow::from_hover_bar(
            0,
            4,
            HoverBarPosition::Left,
            Color::rgb(255, 0, 0),
            Color::rgb(0, 0, 0),
            1.0,
            1,
        );
        let mut cell = default_cell();
        filter.apply(&mut cell, &VfxCellContext::new(0, 0, 10, 1, 0, 0, 0.0));
        assert_eq!(cell.bg, Color::rgb(255, 0, 0));
        assert_ne!(cell.ch, ' ');
    }
}

// <FILE>tui-vfx-compositor-next/src/filters/cls_edge_grow.rs</FILE>
// <DESC>Generalized edge-growth indicator filter using sub-cell partial blocks</DESC>
// <VERS>END OF VERSION: 1.0.1</VERS>
