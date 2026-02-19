// <FILE>tui-vfx-compositor/src/filters/cls_sub_pixel_bar.rs</FILE>
// <DESC>Sub-pixel progress bar filter with 8x horizontal/vertical resolution</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Retain local test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Direction for sub-pixel progress bar rendering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BarDirection {
    /// Fill from left to right using vertical partial blocks (▏▎▍▌▋▊▉█)
    #[default]
    Horizontal,
    /// Fill from bottom to top using horizontal partial blocks (▁▂▃▄▅▆▇█)
    Vertical,
}

/// Sub-pixel progress bar filter with 8x resolution.
///
/// Uses partial block characters to render progress bars with 8 times the
/// resolution of standard cell-by-cell filling:
/// - Horizontal: `▏▎▍▌▋▊▉█` (left one-eighth to full block)
/// - Vertical: `▁▂▃▄▅▆▇█` (lower one-eighth to full block)
///
/// This creates smooth, high-resolution progress indicators that feel
/// modern and precise.
pub struct SubPixelBar {
    /// Progress value (0.0 = empty, 1.0 = full)
    pub progress: f32,
    /// Fill direction
    pub direction: BarDirection,
    /// Color of the filled portion
    pub filled_color: Color,
    /// Color of the unfilled portion (background)
    pub unfilled_color: Color,
    /// If true, animate the progress value using t parameter
    pub animated: bool,
}

impl Default for SubPixelBar {
    fn default() -> Self {
        Self {
            progress: 0.5,
            direction: BarDirection::Horizontal,
            filled_color: Color::rgb(100, 200, 100),
            unfilled_color: Color::rgb(50, 50, 50),
            animated: false,
        }
    }
}

impl SubPixelBar {
    /// Create a new SubPixelBar with given progress.
    pub fn new(progress: f32) -> Self {
        Self {
            progress: progress.clamp(0.0, 1.0),
            ..Default::default()
        }
    }

    /// Set the fill direction.
    pub fn with_direction(mut self, direction: BarDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the filled portion color.
    pub fn with_filled_color(mut self, color: Color) -> Self {
        self.filled_color = color;
        self
    }

    /// Set the unfilled portion color.
    pub fn with_unfilled_color(mut self, color: Color) -> Self {
        self.unfilled_color = color;
        self
    }

    /// Enable animation (progress follows t parameter).
    pub fn animated(mut self, animated: bool) -> Self {
        self.animated = animated;
        self
    }

    /// Get the horizontal partial block character for a given sub-cell index (0-7).
    /// 0 = empty, 1-7 = partial, 8 would be full (handled separately).
    fn horizontal_partial(sub_index: u8) -> char {
        match sub_index {
            0 => ' ',
            1 => '▏', // Left one-eighth block (U+258F)
            2 => '▎', // Left one-quarter block (U+258E)
            3 => '▍', // Left three-eighths block (U+258D)
            4 => '▌', // Left half block (U+258C)
            5 => '▋', // Left five-eighths block (U+258B)
            6 => '▊', // Left three-quarters block (U+258A)
            7 => '▉', // Left seven-eighths block (U+2589)
            _ => '█', // Full block (U+2588)
        }
    }

    /// Get the vertical partial block character for a given sub-cell index (0-7).
    /// 0 = empty, 1-7 = partial, 8 would be full (handled separately).
    fn vertical_partial(sub_index: u8) -> char {
        match sub_index {
            0 => ' ',
            1 => '▁', // Lower one-eighth block (U+2581)
            2 => '▂', // Lower one-quarter block (U+2582)
            3 => '▃', // Lower three-eighths block (U+2583)
            4 => '▄', // Lower half block (U+2584)
            5 => '▅', // Lower five-eighths block (U+2585)
            6 => '▆', // Lower three-quarters block (U+2586)
            7 => '▇', // Lower seven-eighths block (U+2587)
            _ => '█', // Full block (U+2588)
        }
    }
}

impl Filter for SubPixelBar {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        // Determine effective progress (animated or static)
        let progress = if self.animated {
            (t as f32).fract()
        } else {
            self.progress
        };

        match self.direction {
            BarDirection::Horizontal => {
                self.apply_horizontal(cell, x, width, progress);
            }
            BarDirection::Vertical => {
                self.apply_vertical(cell, y, height, progress);
            }
        }
    }
}

impl SubPixelBar {
    fn apply_horizontal(&self, cell: &mut Cell, x: u16, width: u16, progress: f32) {
        if width == 0 {
            return;
        }

        // Total sub-pixels across the width
        let total_sub_pixels = width as f32 * 8.0;
        // How many sub-pixels should be filled
        let filled_sub_pixels = (progress * total_sub_pixels).round() as u32;

        // Which cell is the transition cell (where partial block appears)
        let full_cells = filled_sub_pixels / 8;
        let partial_eighths = (filled_sub_pixels % 8) as u8;

        let cell_index = x as u32;

        if cell_index < full_cells {
            // Fully filled cell
            cell.ch = '█';
            cell.fg = self.filled_color;
            cell.bg = self.unfilled_color;
        } else if cell_index == full_cells && partial_eighths > 0 {
            // Partial cell - the transition point
            cell.ch = Self::horizontal_partial(partial_eighths);
            cell.fg = self.filled_color;
            cell.bg = self.unfilled_color;
        } else {
            // Unfilled cell
            cell.ch = ' ';
            cell.fg = self.unfilled_color;
            cell.bg = self.unfilled_color;
        }
    }

    fn apply_vertical(&self, cell: &mut Cell, y: u16, height: u16, progress: f32) {
        if height == 0 {
            return;
        }

        // Total sub-pixels across the height
        let total_sub_pixels = height as f32 * 8.0;
        // How many sub-pixels should be filled (from bottom)
        let filled_sub_pixels = (progress * total_sub_pixels).round() as u32;

        // Which cell is the transition cell
        let full_cells = filled_sub_pixels / 8;
        let partial_eighths = (filled_sub_pixels % 8) as u8;

        // For vertical, y=height-1 is the bottom, y=0 is the top
        // Cells fill from bottom up
        let cells_from_bottom = (height as u32).saturating_sub(1 + y as u32);

        if cells_from_bottom < full_cells {
            // Fully filled cell
            cell.ch = '█';
            cell.fg = self.filled_color;
            cell.bg = self.unfilled_color;
        } else if cells_from_bottom == full_cells && partial_eighths > 0 {
            // Partial cell - the transition point
            cell.ch = Self::vertical_partial(partial_eighths);
            cell.fg = self.filled_color;
            cell.bg = self.unfilled_color;
        } else {
            // Unfilled cell
            cell.ch = ' ';
            cell.fg = self.unfilled_color;
            cell.bg = self.unfilled_color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell() -> Cell {
        Cell::styled(
            ' ',
            Color::rgb(255, 255, 255),
            Color::rgb(0, 0, 0),
            Modifiers::NONE,
        )
    }

    #[test]
    fn default_values() {
        let filter = SubPixelBar::default();
        assert_eq!(filter.progress, 0.5);
        assert_eq!(filter.direction, BarDirection::Horizontal);
        assert!(!filter.animated);
    }

    #[test]
    fn builder_pattern() {
        let filter = SubPixelBar::new(0.75)
            .with_direction(BarDirection::Vertical)
            .with_filled_color(Color::rgb(0, 255, 0))
            .with_unfilled_color(Color::rgb(50, 50, 50))
            .animated(true);

        assert_eq!(filter.progress, 0.75);
        assert_eq!(filter.direction, BarDirection::Vertical);
        assert_eq!(filter.filled_color, Color::rgb(0, 255, 0));
        assert!(filter.animated);
    }

    #[test]
    fn progress_clamped() {
        let filter = SubPixelBar::new(1.5);
        assert_eq!(filter.progress, 1.0);

        let filter = SubPixelBar::new(-0.5);
        assert_eq!(filter.progress, 0.0);
    }

    #[test]
    fn horizontal_zero_progress() {
        let filter = SubPixelBar::new(0.0);

        // All cells should be empty
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, ' ');

        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, ' ');
    }

    #[test]
    fn horizontal_full_progress() {
        let filter = SubPixelBar::new(1.0);

        // All cells should be full blocks
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, '█');

        let mut cell = make_cell();
        filter.apply(&mut cell, 9, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, '█');
    }

    #[test]
    fn horizontal_half_progress() {
        let filter = SubPixelBar::new(0.5);

        // Width 10, 50% = 40 sub-pixels = 5 full cells, 0 partial
        let mut cell = make_cell();
        filter.apply(&mut cell, 4, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, '█'); // Cell 4 is full (cells 0-4 = 5 cells)

        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, ' '); // Cell 5 is empty
    }

    #[test]
    fn horizontal_partial_block() {
        // 10 cells = 80 sub-pixels
        // 0.125 progress = 10 sub-pixels = 1 full cell + 2/8 partial
        let filter = SubPixelBar::new(0.125);

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, '█'); // Cell 0 is full

        let mut cell = make_cell();
        filter.apply(&mut cell, 1, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, '▎'); // Cell 1 is 2/8 = ▎

        let mut cell = make_cell();
        filter.apply(&mut cell, 2, 0, 10, 1, 0.0);
        assert_eq!(cell.ch, ' '); // Cell 2 is empty
    }

    #[test]
    fn vertical_direction() {
        let filter = SubPixelBar::new(1.0).with_direction(BarDirection::Vertical);

        // All cells should be full blocks
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 1, 10, 0.0);
        assert_eq!(cell.ch, '█');
    }

    #[test]
    fn animated_uses_t() {
        let filter = SubPixelBar::new(0.0).animated(true);

        // With t=0.5, progress should be 0.5
        // Width 10, 50% = 40 sub-pixels = 5 full cells
        let mut cell = make_cell();
        filter.apply(&mut cell, 4, 0, 10, 1, 0.5);
        assert_eq!(cell.ch, '█'); // Cell 4 is full

        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 0, 10, 1, 0.5);
        assert_eq!(cell.ch, ' '); // Cell 5 is empty
    }

    #[test]
    fn colors_applied_correctly() {
        let filter = SubPixelBar::new(0.5)
            .with_filled_color(Color::rgb(0, 255, 0))
            .with_unfilled_color(Color::rgb(100, 100, 100));

        // Filled cell
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);
        assert_eq!(cell.fg, Color::rgb(0, 255, 0));
        assert_eq!(cell.bg, Color::rgb(100, 100, 100));

        // Unfilled cell
        let mut cell = make_cell();
        filter.apply(&mut cell, 9, 0, 10, 1, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
        assert_eq!(cell.bg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn horizontal_partial_characters_correct() {
        // Test each partial block appears at correct progress
        // Width 8 cells = 64 sub-pixels, so each sub-pixel = 1/64 progress
        for i in 1..8u8 {
            let progress = i as f32 / 64.0; // 1-7 sub-pixels in first cell
            let filter = SubPixelBar::new(progress);

            let mut cell = make_cell();
            filter.apply(&mut cell, 0, 0, 8, 1, 0.0);

            let expected = SubPixelBar::horizontal_partial(i);
            assert_eq!(
                cell.ch, expected,
                "At progress {}, expected '{}' but got '{}'",
                progress, expected, cell.ch
            );
        }
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_sub_pixel_bar.rs</FILE>
// <DESC>Sub-pixel progress bar filter with 8x horizontal/vertical resolution</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
