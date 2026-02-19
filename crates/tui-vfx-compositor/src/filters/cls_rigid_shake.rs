// <FILE>tui-vfx-compositor/src/filters/cls_rigid_shake.rs</FILE>
// <DESC>Rigid body shake filter with ketchup bottle damped oscillation pattern</DESC>
// <VERS>VERSION: 1.2.4</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color, RigidShakeTiming};

/// Rigid body shake filter using partial vertical blocks.
///
/// Creates a "ketchup bottle" shake effect where the entire element appears to
/// shift left and right as a rigid body. Uses partial block characters to draw
/// extensions and gaps in the margin cells outside the widget area.
///
/// The effect consists of multiple damped oscillations followed by a pause:
/// - Each oscillation is a full sine wave (right → center → left → center)
/// - Amplitude decreases with each successive shake (damping)
/// - A base extension is always visible so the effect doesn't appear from nothing
///
/// # Synchronization with Style Effects
///
/// Use [`RigidShakeTiming`] to synchronize text styling (italic, shift) with
/// the margin animation. Both this filter and `StyleEffect::RigidShakeStyle`
/// use the same timing calculation.
///
/// ```ignore
/// use tui_vfx_types::RigidShakeTiming;
///
/// let timing = RigidShakeTiming::default();
/// let state = timing.calculate(elapsed_secs);
///
/// // Use state.is_shifting_right() to sync text styling
/// ```
///
/// IMPORTANT: Apply this filter to an area that includes 2-cell margins on each side.
/// The filter treats the outermost 2 columns on each side as margin areas.
pub struct RigidShake {
    /// Shared timing configuration
    pub timing: RigidShakeTiming,
    /// Color of the element being shaken
    pub element_color: Color,
    /// Background color (shows in gaps)
    pub bg_color: Color,
    /// Width of the inner content area (excluding margins)
    /// Set this to the actual widget width; margins are outside this
    pub inner_width: u16,
    /// Margin width on each side (default 2)
    pub margin_width: u8,
}

// Legacy field accessors for backwards compatibility
#[allow(dead_code)]
impl RigidShake {
    /// Duration of one back-and-forth shake in seconds
    pub fn shake_period(&self) -> f32 {
        self.timing.shake_period
    }

    /// Number of shakes before pause
    pub fn num_shakes(&self) -> u8 {
        self.timing.num_shakes
    }

    /// Duration of pause between shake cycles in seconds
    pub fn pause_duration(&self) -> f32 {
        self.timing.pause_duration
    }

    /// Maximum extension in eighths of a cell
    pub fn max_eighths(&self) -> u8 {
        self.timing.max_eighths
    }

    /// Base extension always visible at rest
    pub fn base_eighths(&self) -> u8 {
        self.timing.base_eighths
    }

    /// Amplitude multipliers for each shake (damping curve)
    pub fn damping(&self) -> &[f32; 8] {
        &self.timing.damping
    }
}

impl Default for RigidShake {
    fn default() -> Self {
        Self {
            timing: RigidShakeTiming::default(),
            element_color: Color::rgb(100, 100, 100),
            bg_color: Color::rgb(30, 30, 30),
            inner_width: 10,
            margin_width: 2,
        }
    }
}

impl RigidShake {
    /// Create a new RigidShake with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the shake period (duration of one back-and-forth).
    pub fn with_shake_period(mut self, period: f32) -> Self {
        self.timing.shake_period = period;
        self
    }

    /// Set the number of shakes before pause.
    pub fn with_num_shakes(mut self, num: u8) -> Self {
        self.timing.num_shakes = num.min(8);
        self
    }

    /// Set the pause duration between shake cycles.
    pub fn with_pause_duration(mut self, duration: f32) -> Self {
        self.timing.pause_duration = duration;
        self
    }

    /// Set the maximum extension in eighths.
    pub fn with_max_eighths(mut self, eighths: u8) -> Self {
        self.timing.max_eighths = eighths.min(16);
        self
    }

    /// Set the base extension (always visible at rest).
    pub fn with_base_eighths(mut self, eighths: u8) -> Self {
        self.timing.base_eighths = eighths.min(self.timing.max_eighths);
        self
    }

    /// Set the damping curve.
    pub fn with_damping(mut self, damping: [f32; 8]) -> Self {
        self.timing.damping = damping;
        self
    }

    /// Set the timing configuration directly.
    #[allow(dead_code)]
    pub fn with_timing(mut self, timing: RigidShakeTiming) -> Self {
        self.timing = timing;
        self
    }

    /// Set the element color.
    pub fn with_element_color(mut self, color: Color) -> Self {
        self.element_color = color;
        self
    }

    /// Set the background color.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the inner content width.
    pub fn with_inner_width(mut self, width: u16) -> Self {
        self.inner_width = width;
        self
    }

    /// Set the margin width on each side.
    pub fn with_margin_width(mut self, width: u8) -> Self {
        self.margin_width = width.min(4);
        self
    }

    /// Calculate the current offset in eighths based on time.
    /// Delegates to the shared RigidShakeTiming utility.
    fn calculate_offset(&self, t: f64) -> i16 {
        self.timing.calculate(t).offset_eighths
    }

    /// Get the left-aligned partial block character for a given eighths value.
    fn left_block(eighths: usize) -> char {
        const BLOCKS: [char; 9] = [' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'];
        BLOCKS[eighths.min(8)]
    }
}

impl Filter for RigidShake {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        let margin = self.margin_width as u16;

        // Determine if this cell is in the margin area
        let left_margin_end = margin;
        let right_margin_start = width.saturating_sub(margin);

        let in_left_margin = x < left_margin_end;
        let in_right_margin = x >= right_margin_start;

        // If not in margin, leave cell unchanged
        if !in_left_margin && !in_right_margin {
            return;
        }

        let _ = (y, height); // Suppress unused warnings

        let offset_eighths = self.calculate_offset(t);

        if offset_eighths == 0 {
            // At exact center, clear margins
            cell.ch = ' ';
            cell.bg = self.bg_color;
            return;
        }

        let abs_eighths = offset_eighths.unsigned_abs() as usize;
        let first_cell_eighths = abs_eighths.min(8);
        let second_cell_eighths = abs_eighths.saturating_sub(8);

        // Determine which margin cell this is (0 = adjacent to content, 1 = further out)
        let left_margin_idx = if in_left_margin {
            (margin - 1 - x) as usize
        } else {
            0
        };
        let right_margin_idx = if in_right_margin {
            (x - right_margin_start) as usize
        } else {
            0
        };

        if offset_eighths > 0 {
            // Card shifted RIGHT
            if in_right_margin {
                // Right margin: card color extends in
                let eighths = if right_margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };
                if eighths > 0 {
                    cell.ch = Self::left_block(eighths);
                    cell.fg = self.element_color;
                    cell.bg = self.bg_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            } else if in_left_margin {
                // Left margin: gap appears (background)
                let eighths = if left_margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };
                cell.ch = Self::left_block(eighths);
                cell.fg = self.bg_color;
                cell.bg = self.bg_color;
            }
        } else {
            // Card shifted LEFT (offset_eighths < 0)
            if in_left_margin {
                // Left margin: card color extends in from right
                let eighths = if left_margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };
                if eighths >= 8 {
                    cell.ch = '█';
                    cell.fg = self.element_color;
                    cell.bg = self.element_color;
                } else if eighths > 0 {
                    // Card on right side of cell
                    let inverse = 8 - eighths;
                    cell.ch = Self::left_block(inverse.clamp(1, 7));
                    cell.fg = self.bg_color;
                    cell.bg = self.element_color;
                } else {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                }
            } else if in_right_margin {
                // Right margin: gap appears
                let eighths = if right_margin_idx == 0 {
                    first_cell_eighths
                } else {
                    second_cell_eighths
                };
                if eighths >= 8 {
                    cell.ch = ' ';
                    cell.bg = self.bg_color;
                } else if eighths > 0 {
                    let inverse = 8 - eighths;
                    cell.ch = Self::left_block(inverse.clamp(1, 7));
                    cell.fg = self.bg_color;
                    cell.bg = self.bg_color;
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
        let filter = RigidShake::default();
        assert_eq!(filter.shake_period(), 0.29);
        assert_eq!(filter.num_shakes(), 4);
        assert_eq!(filter.pause_duration(), 0.52);
        assert_eq!(filter.max_eighths(), 12);
        assert_eq!(filter.base_eighths(), 3);
    }

    #[test]
    fn builder_pattern() {
        let filter = RigidShake::new()
            .with_shake_period(0.5)
            .with_num_shakes(3)
            .with_pause_duration(1.0)
            .with_max_eighths(8)
            .with_base_eighths(2)
            .with_element_color(Color::rgb(255, 0, 0))
            .with_bg_color(Color::rgb(0, 0, 0));

        assert_eq!(filter.shake_period(), 0.5);
        assert_eq!(filter.num_shakes(), 3);
        assert_eq!(filter.pause_duration(), 1.0);
        assert_eq!(filter.max_eighths(), 8);
        assert_eq!(filter.base_eighths(), 2);
    }

    #[test]
    fn offset_during_pause() {
        let filter = RigidShake::default();
        // During pause (after active duration), offset should be base_eighths
        let active = filter.shake_period() * filter.num_shakes() as f32;
        let offset = filter.calculate_offset((active + 0.1) as f64);
        // During pause, raw_offset = 0, so offset = base_eighths = 3
        assert_eq!(offset, 3);
    }

    #[test]
    fn leaves_interior_unchanged() {
        let filter = RigidShake::default()
            .with_inner_width(10)
            .with_margin_width(2);

        // Cell in interior (x=5, total width=14 with margins)
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 5, 0, 14, 1, 0.0);

        // Interior cell should be unchanged
        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn modifies_margin_cells() {
        let filter = RigidShake::default()
            .with_inner_width(10)
            .with_margin_width(2)
            .with_element_color(Color::rgb(100, 100, 100));

        // Right margin cell (x=12 in width=14)
        let mut cell = make_cell();
        filter.apply(&mut cell, 12, 0, 14, 1, 0.0);

        // Should be modified (base extension visible)
        assert!(cell.ch != ' ' || cell.fg != Color::WHITE);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_rigid_shake.rs</FILE>
// <DESC>Rigid body shake filter with ketchup bottle damped oscillation pattern</DESC>
// <VERS>END OF VERSION: 1.2.4</VERS>
