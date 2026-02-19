// <FILE>tui-vfx-compositor/src/filters/cls_hover_bar.rs</FILE>
// <DESC>Progress-driven partial bar indicator for hover/focus states</DESC>
// <VERS>VERSION: 1.2.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use crate::types::HoverBarPosition;
use tui_vfx_types::{Cell, Color};

/// Progress-driven partial bar indicator for hover/focus states.
///
/// Supports both vertical (Left/Right) and horizontal (Top/Bottom) orientations:
/// - Vertical: Uses left-aligned partial blocks (▏▎▍▌▋▊▉█)
/// - Horizontal: Uses lower partial blocks (▁▂▃▄▅▆▇█)
///
/// The bar expands from `base_eighths` to `max_eighths` based on animation progress.
///
/// # Contiguous Bar Technique
///
/// To achieve seamless 2-cell bars, this filter uses fg/bg inversion:
/// - Cell adjacent to content: uses partial block with normal colors
/// - Cell further out: uses inverse partial with swapped colors
///
/// # Usage
///
/// Apply to an area that includes margin cells for the indicator.
/// The filter draws in the margin cells based on position.
pub struct HoverBar {
    /// Base width at rest (0.0 progress), in eighths (0-8)
    pub base_eighths: u8,
    /// Maximum width when fully active (1.0 progress), in eighths (0-16)
    pub max_eighths: u8,
    /// Position relative to content
    pub position: HoverBarPosition,
    /// Bar color
    pub bar_color: Color,
    /// Background color (for inversion)
    pub bg_color: Color,
    /// Animation progress (0.0 = rest, 1.0 = fully active)
    pub progress: f32,
    /// Width of margin area on the active side (1-2 cells)
    pub margin_width: u8,
}

impl Default for HoverBar {
    fn default() -> Self {
        Self {
            base_eighths: 4,
            max_eighths: 12,
            position: HoverBarPosition::Left,
            bar_color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
            progress: 0.0,
            margin_width: 2,
        }
    }
}

impl HoverBar {
    /// Create a new HoverBar with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the base width at rest (in eighths).
    pub fn with_base_eighths(mut self, eighths: u8) -> Self {
        self.base_eighths = eighths.min(8);
        self
    }

    /// Set the maximum width when fully active (in eighths).
    pub fn with_max_eighths(mut self, eighths: u8) -> Self {
        self.max_eighths = eighths.min(16);
        self
    }

    /// Set the position relative to content.
    pub fn with_position(mut self, position: HoverBarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the bar color.
    pub fn with_bar_color(mut self, color: Color) -> Self {
        self.bar_color = color;
        self
    }

    /// Set the background color.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the animation progress.
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Set the margin width (1-2 cells).
    pub fn with_margin_width(mut self, width: u8) -> Self {
        self.margin_width = width.clamp(1, 2);
        self
    }

    /// Calculate current eighths based on progress.
    fn current_eighths(&self) -> u8 {
        let range = self.max_eighths.saturating_sub(self.base_eighths) as f32;
        let delta = (range * self.progress) as u8;
        self.base_eighths + delta
    }

    /// Get the left-aligned partial block character for a given eighths value.
    fn left_block(eighths: usize) -> char {
        const BLOCKS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        BLOCKS[eighths.min(8)]
    }

    /// Get the lower partial block character for a given eighths value.
    fn lower_block(eighths: usize) -> char {
        const BLOCKS: [char; 9] = [' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'];
        BLOCKS[eighths.min(8)]
    }
}

impl Filter for HoverBar {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, _t: f64) {
        let margin = self.margin_width as u16;

        // Determine margin boundaries based on orientation
        let (in_target_margin, margin_idx) = match self.position {
            HoverBarPosition::Left => {
                let left_margin_end = margin;
                let in_margin = x < left_margin_end;
                let idx = if in_margin {
                    (left_margin_end - 1 - x) as usize
                } else {
                    0
                };
                (in_margin, idx)
            }
            HoverBarPosition::Right => {
                let right_margin_start = width.saturating_sub(margin);
                let in_margin = x >= right_margin_start;
                let idx = if in_margin {
                    (x - right_margin_start) as usize
                } else {
                    0
                };
                (in_margin, idx)
            }
            HoverBarPosition::Top => {
                let top_margin_end = margin;
                let in_margin = y < top_margin_end;
                let idx = if in_margin {
                    (top_margin_end - 1 - y) as usize
                } else {
                    0
                };
                (in_margin, idx)
            }
            HoverBarPosition::Bottom => {
                let bottom_margin_start = height.saturating_sub(margin);
                let in_margin = y >= bottom_margin_start;
                let idx = if in_margin {
                    (y - bottom_margin_start) as usize
                } else {
                    0
                };
                (in_margin, idx)
            }
        };

        if !in_target_margin {
            return;
        }

        let eighths = self.current_eighths() as usize;
        if eighths == 0 {
            cell.ch = ' ';
            cell.bg = self.bg_color;
            return;
        }

        let first_cell_eighths = eighths.min(8);
        let second_cell_eighths = eighths.saturating_sub(8);

        // Render based on position and cell index
        match self.position {
            HoverBarPosition::Left => {
                // Bar extends leftward from content edge
                let cell_eighths = if margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };

                if cell_eighths >= 8 {
                    cell.ch = '█';
                    cell.fg = self.bar_color;
                    cell.bg = self.bar_color;
                } else if cell_eighths > 0 {
                    // Use inverse: show gap on left, bar on right
                    let inverse = 8 - cell_eighths;
                    cell.ch = Self::left_block(inverse);
                    cell.fg = self.bg_color;
                    cell.bg = self.bar_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            }
            HoverBarPosition::Right => {
                // Bar extends rightward from content edge
                let cell_eighths = if margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };

                if cell_eighths >= 8 {
                    cell.ch = '█';
                    cell.fg = self.bar_color;
                    cell.bg = self.bar_color;
                } else if cell_eighths > 0 {
                    cell.ch = Self::left_block(cell_eighths);
                    cell.fg = self.bar_color;
                    cell.bg = self.bg_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            }
            HoverBarPosition::Top => {
                // Bar starts at bottom of top margin (adjacent to content) and expands upward
                let cell_eighths = if margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };

                if cell_eighths >= 8 {
                    cell.ch = '█';
                    cell.fg = self.bar_color;
                    cell.bg = self.bar_color;
                } else if cell_eighths > 0 {
                    // Lower block fills from bottom up, perfect for bar adjacent to content below
                    cell.ch = Self::lower_block(cell_eighths);
                    cell.fg = self.bar_color;
                    cell.bg = self.bg_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            }
            HoverBarPosition::Bottom => {
                // Bar starts at top of bottom margin (adjacent to content) and expands downward
                let cell_eighths = if margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };

                if cell_eighths >= 8 {
                    cell.ch = '█';
                    cell.fg = self.bar_color;
                    cell.bg = self.bar_color;
                } else if cell_eighths > 0 {
                    // Use inverse: show gap at bottom, bar at top (adjacent to content above)
                    let inverse = 8 - cell_eighths;
                    cell.ch = Self::lower_block(inverse);
                    cell.fg = self.bg_color;
                    cell.bg = self.bar_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::test_support::make_cell;

    #[test]
    fn default_values() {
        let filter = HoverBar::default();
        assert_eq!(filter.base_eighths, 4);
        assert_eq!(filter.max_eighths, 12);
        assert_eq!(filter.position, HoverBarPosition::Left);
        assert_eq!(filter.margin_width, 2);
    }

    #[test]
    fn builder_pattern() {
        let filter = HoverBar::new()
            .with_base_eighths(2)
            .with_max_eighths(16)
            .with_position(HoverBarPosition::Right)
            .with_progress(0.5)
            .with_bar_color(Color::rgb(255, 0, 0))
            .with_bg_color(Color::rgb(0, 0, 0))
            .with_margin_width(1);

        assert_eq!(filter.base_eighths, 2);
        assert_eq!(filter.max_eighths, 16);
        assert_eq!(filter.position, HoverBarPosition::Right);
        assert_eq!(filter.progress, 0.5);
        assert_eq!(filter.margin_width, 1);
    }

    #[test]
    fn zero_progress_shows_base() {
        let filter = HoverBar::new()
            .with_base_eighths(4)
            .with_max_eighths(12)
            .with_progress(0.0);

        assert_eq!(filter.current_eighths(), 4);
    }

    #[test]
    fn full_progress_shows_max() {
        let filter = HoverBar::new()
            .with_base_eighths(4)
            .with_max_eighths(12)
            .with_progress(1.0);

        assert_eq!(filter.current_eighths(), 12);
    }

    #[test]
    fn half_progress_interpolates() {
        let filter = HoverBar::new()
            .with_base_eighths(4)
            .with_max_eighths(12)
            .with_progress(0.5);

        // base=4, max=12, range=8, half=4, result=4+4=8
        assert_eq!(filter.current_eighths(), 8);
    }

    #[test]
    fn leaves_interior_unchanged() {
        let filter = HoverBar::new()
            .with_margin_width(2)
            .with_position(HoverBarPosition::Left);

        // Cell in interior (x=5, total width=14 with 2-cell margins on each side)
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 5, 0, 14, 1, 0.0);

        assert_eq!(cell.ch, original.ch);
        assert_eq!(cell.fg, original.fg);
        assert_eq!(cell.bg, original.bg);
    }

    #[test]
    fn left_position_modifies_left_margin() {
        let filter = HoverBar::new()
            .with_margin_width(2)
            .with_position(HoverBarPosition::Left)
            .with_base_eighths(4)
            .with_progress(0.0)
            .with_bar_color(Color::rgb(100, 150, 200))
            .with_bg_color(Color::rgb(30, 30, 30));

        // Left margin cell (x=0 or x=1 in width=14)
        let mut cell = make_cell();
        filter.apply(&mut cell, 1, 0, 14, 1, 0.0);

        // Should be modified (base extension visible)
        // With base_eighths=4, cell 1 (adjacent to content) shows 4/8
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn right_position_modifies_right_margin() {
        let filter = HoverBar::new()
            .with_margin_width(2)
            .with_position(HoverBarPosition::Right)
            .with_base_eighths(4)
            .with_progress(0.0)
            .with_bar_color(Color::rgb(100, 150, 200));

        // Right margin cell (x=12 in width=14)
        let mut cell = make_cell();
        filter.apply(&mut cell, 12, 0, 14, 1, 0.0);

        // Should be modified
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn left_block_chars() {
        assert_eq!(HoverBar::left_block(0), ' ');
        assert_eq!(HoverBar::left_block(1), '▏');
        assert_eq!(HoverBar::left_block(4), '▌');
        assert_eq!(HoverBar::left_block(8), '█');
        assert_eq!(HoverBar::left_block(10), '█'); // Clamped
    }

    #[test]
    fn lower_block_chars() {
        assert_eq!(HoverBar::lower_block(0), ' ');
        assert_eq!(HoverBar::lower_block(1), '▁');
        assert_eq!(HoverBar::lower_block(4), '▄');
        assert_eq!(HoverBar::lower_block(8), '█');
        assert_eq!(HoverBar::lower_block(10), '█'); // Clamped
    }

    #[test]
    fn bottom_position_modifies_bottom_margin() {
        let filter = HoverBar::new()
            .with_margin_width(1)
            .with_position(HoverBarPosition::Bottom)
            .with_base_eighths(4)
            .with_progress(0.0)
            .with_bar_color(Color::rgb(100, 150, 200));

        // Bottom margin cell (y=9 in height=10)
        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 9, 10, 10, 0.0);

        // Should be modified
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn top_position_modifies_top_margin() {
        let filter = HoverBar::new()
            .with_margin_width(1)
            .with_position(HoverBarPosition::Top)
            .with_base_eighths(4)
            .with_progress(0.0)
            .with_bar_color(Color::rgb(100, 150, 200));

        // Top margin cell (y=0 in height=10)
        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 0, 10, 10, 0.0);

        // Should be modified
        assert_ne!(cell.ch, ' ');
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_hover_bar.rs</FILE>
// <DESC>Progress-driven partial bar indicator for hover/focus states</DESC>
// <VERS>END OF VERSION: 1.2.2</VERS>
