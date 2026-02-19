// <FILE>tui-vfx-compositor/src/filters/cls_sub_cell_shake.rs</FILE>
// <DESC>Sub-cell shake filter using partial blocks for physical vibration effect</DESC>
// <VERS>VERSION: 1.0.5</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Retain local test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Sub-cell shake filter using partial vertical blocks.
///
/// Creates physical-feeling vibration by oscillating edges using partial blocks
/// (▏▎▍▌▋▊▉) to shift the visual center of mass without changing grid coordinates.
///
/// This implements the "incorrect password" or "tactile click" effect from IDEAS.md:
/// - State 0 (Rest): Normal rendering
/// - State 1 (Shift Right +0.25): Left edge indents, right edge extends
/// - State 2 (Shift Left -0.25): Left edge extends, right edge indents
///
/// The animation cycles between offsets creating a rapid vibration that feels
/// physical rather than digital.
pub struct SubCellShake {
    /// Maximum offset in eighths of a cell (1-4 recommended)
    pub amplitude: u8,
    /// Shake frequency (cycles per second)
    pub frequency: f32,
    /// Random seed for pattern variation
    pub seed: u64,
    /// If true, only shake edge cells (left and right borders)
    pub edge_only: bool,
    /// Filled color (foreground of the element)
    pub filled_color: Color,
    /// Background color (shows through partial blocks)
    pub bg_color: Color,
}

impl Default for SubCellShake {
    fn default() -> Self {
        Self {
            amplitude: 2,
            frequency: 8.0,
            seed: 42,
            edge_only: true,
            filled_color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
        }
    }
}

impl SubCellShake {
    /// Create a new SubCellShake with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the shake amplitude in eighths of a cell.
    pub fn with_amplitude(mut self, amplitude: u8) -> Self {
        self.amplitude = amplitude.min(7);
        self
    }

    /// Set the shake frequency in cycles per second.
    pub fn with_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Set the random seed.
    pub fn with_seed(mut self, seed: u64) -> Self {
        self.seed = seed;
        self
    }

    /// Set whether to only shake edge cells.
    pub fn edge_only(mut self, edge_only: bool) -> Self {
        self.edge_only = edge_only;
        self
    }

    /// Set the filled color.
    pub fn with_filled_color(mut self, color: Color) -> Self {
        self.filled_color = color;
        self
    }

    /// Set the background color.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Get the left-aligned partial block character for a given eighths value (1-7).
    /// These are "left N-eighths block" characters.
    fn left_partial(eighths: u8) -> char {
        match eighths {
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

    /// Get the right-aligned partial block character for a given eighths value (1-7).
    /// We simulate right-aligned by using left blocks with inverted colors.
    #[allow(dead_code)]
    fn right_partial(eighths: u8) -> char {
        // Right partial is achieved by using left partial with swapped colors
        // So if we want 2/8 from the right, we use 6/8 from the left with inverted colors
        Self::left_partial(8 - eighths)
    }

    /// Calculate the current offset based on time.
    /// Returns a value from -amplitude to +amplitude in eighths.
    fn calculate_offset(&self, t: f64, x: u16, y: u16) -> i8 {
        // Use a simple hash to add per-cell variation
        let cell_hash = ((x as u64)
            .wrapping_mul(31)
            .wrapping_add(y as u64)
            .wrapping_mul(17)
            .wrapping_add(self.seed))
            % 1000;
        let phase_offset = cell_hash as f64 / 1000.0;

        // Calculate oscillation phase
        let phase = (t * self.frequency as f64 + phase_offset) * std::f64::consts::TAU;

        // Use sine wave for smooth oscillation
        let sine = phase.sin();

        // Scale to amplitude and convert to eighths
        (sine * self.amplitude as f64).round() as i8
    }
}

impl Filter for SubCellShake {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, _height: u16, t: f64) {
        if width == 0 {
            return;
        }

        let is_left_edge = x == 0;
        let is_right_edge = x == width.saturating_sub(1);
        let is_edge = is_left_edge || is_right_edge;

        // If edge_only mode, skip non-edge cells
        if self.edge_only && !is_edge {
            return;
        }

        let offset = self.calculate_offset(t, x, y);

        if offset == 0 {
            // No shake this frame - keep cell as is but ensure colors are set
            if is_edge {
                cell.ch = '█';
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            }
            return;
        }

        if is_left_edge {
            // Left edge cell
            if offset > 0 {
                // Shifting right: left edge "indents" - show partial block from right
                // Use left partial block with inverted colors to simulate indent
                let partial_eighths = 8 - offset.unsigned_abs();
                cell.ch = Self::left_partial(partial_eighths);
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            } else {
                // Shifting left: left edge "extends" - this would extend into previous cell
                // For leftmost edge, we can't extend left, so show full block
                cell.ch = '█';
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            }
        } else if is_right_edge {
            // Right edge cell
            if offset > 0 {
                // Shifting right: right edge "extends" into next cell area
                // Show full block (extension happens conceptually)
                cell.ch = '█';
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            } else {
                // Shifting left: right edge "contracts" - show partial from left
                let partial_eighths = 8 - offset.unsigned_abs();
                cell.ch = Self::left_partial(partial_eighths);
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            }
        } else if !self.edge_only {
            // Interior cell (only if not edge_only mode)
            // Interior cells shift their content slightly using partial blocks
            // This creates a subtle wave effect across the surface
            if offset > 0 {
                // Positive offset: show slightly less on left, implying rightward shift
                let show_eighths = 8 - (offset.unsigned_abs() / 2);
                cell.ch = Self::left_partial(show_eighths.max(6)); // Keep at least 6/8
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            } else {
                // Negative offset: show full or slightly extended
                cell.ch = '█';
                cell.fg = self.filled_color;
                cell.bg = self.bg_color;
            }
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
        let filter = SubCellShake::default();
        assert_eq!(filter.amplitude, 2);
        assert_eq!(filter.frequency, 8.0);
        assert!(filter.edge_only);
    }

    #[test]
    fn builder_pattern() {
        let filter = SubCellShake::new()
            .with_amplitude(4)
            .with_frequency(16.0)
            .with_seed(123)
            .edge_only(false)
            .with_filled_color(Color::rgb(255, 0, 0))
            .with_bg_color(Color::rgb(0, 0, 0));

        assert_eq!(filter.amplitude, 4);
        assert_eq!(filter.frequency, 16.0);
        assert_eq!(filter.seed, 123);
        assert!(!filter.edge_only);
        assert_eq!(filter.filled_color, Color::rgb(255, 0, 0));
    }

    #[test]
    fn amplitude_clamped() {
        let filter = SubCellShake::new().with_amplitude(10);
        assert_eq!(filter.amplitude, 7); // Clamped to max 7
    }

    #[test]
    fn left_partial_characters() {
        assert_eq!(SubCellShake::left_partial(0), ' ');
        assert_eq!(SubCellShake::left_partial(1), '▏');
        assert_eq!(SubCellShake::left_partial(2), '▎');
        assert_eq!(SubCellShake::left_partial(3), '▍');
        assert_eq!(SubCellShake::left_partial(4), '▌');
        assert_eq!(SubCellShake::left_partial(5), '▋');
        assert_eq!(SubCellShake::left_partial(6), '▊');
        assert_eq!(SubCellShake::left_partial(7), '▉');
        assert_eq!(SubCellShake::left_partial(8), '█');
    }

    #[test]
    fn edge_only_skips_interior() {
        let filter = SubCellShake::new().edge_only(true);

        // Interior cell (x=5 in width=10)
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);

        // Cell should be unchanged (edge_only skips interior)
        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn applies_to_edges() {
        let filter = SubCellShake::new()
            .with_amplitude(2)
            .with_filled_color(Color::rgb(100, 100, 100))
            .edge_only(true);

        // Left edge cell (x=0)
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        // Should have changed to a block character
        assert!(
            cell.ch == '█'
                || cell.ch == '▉'
                || cell.ch == '▊'
                || cell.ch == '▋'
                || cell.ch == '▌'
                || cell.ch == '▍'
                || cell.ch == '▎'
                || cell.ch == '▏'
        );
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn oscillates_over_time() {
        let filter = SubCellShake::new()
            .with_amplitude(3)
            .with_frequency(1.0) // 1 cycle per second
            .with_seed(0);

        let mut symbols_at_times = Vec::new();

        // Sample at different times through one cycle
        for i in 0..8 {
            let t = i as f64 / 8.0;
            let mut cell = make_cell();
            filter.apply(&mut cell, 0, 0, 10, 1, t);
            symbols_at_times.push(cell.ch);
        }

        // Should have some variation (not all same character)
        let unique: std::collections::HashSet<_> = symbols_at_times.iter().collect();
        assert!(
            unique.len() > 1,
            "Expected variation in shake symbols over time"
        );
    }

    #[test]
    fn non_edge_only_affects_interior() {
        let filter = SubCellShake::new().with_amplitude(2).edge_only(false);

        // Interior cell
        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 0, 10, 1, 0.25);

        // Should have been modified
        assert!(cell.ch != ' ' || cell.fg != Color::rgb(255, 255, 255));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_sub_cell_shake.rs</FILE>
// <DESC>Sub-cell shake filter using partial blocks for physical vibration effect</DESC>
// <VERS>END OF VERSION: 1.0.5</VERS>
