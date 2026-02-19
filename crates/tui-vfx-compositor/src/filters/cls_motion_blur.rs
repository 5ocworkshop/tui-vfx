// <FILE>tui-vfx-compositor/src/filters/cls_motion_blur.rs</FILE>
// <DESC>Motion blur trail effect with directional dimming</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Retain local test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Direction of motion blur trail.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum MotionDirection {
    /// Trail extends to the left (motion toward right)
    #[default]
    Left,
    /// Trail extends to the right (motion toward left)
    Right,
    /// Trail extends upward (motion toward bottom)
    Up,
    /// Trail extends downward (motion toward top)
    Down,
}

/// Motion blur filter that creates movement trail effects.
///
/// Applies graduated dimming across the widget in a specified direction,
/// simulating the appearance of motion blur or trailing effects.
/// Cells "ahead" of the motion are brighter, cells "behind" are dimmer.
pub struct MotionBlur {
    /// Length of the blur trail as a fraction of widget dimension (0.0 - 1.0)
    /// 0.5 means the trail covers half the widget
    pub trail_length: f32,

    /// Opacity decay curve exponent (1.0 = linear, 2.0 = quadratic)
    /// Higher values create sharper falloff near the "head" of motion
    pub opacity_decay: f32,

    /// Direction of the motion blur trail
    pub direction: MotionDirection,
}

impl Default for MotionBlur {
    fn default() -> Self {
        Self {
            trail_length: 0.5,
            opacity_decay: 1.5,
            direction: MotionDirection::Left,
        }
    }
}

impl MotionBlur {
    /// Create a new MotionBlur filter.
    pub fn new(trail_length: f32, opacity_decay: f32, direction: MotionDirection) -> Self {
        Self {
            trail_length: trail_length.clamp(0.0, 1.0),
            opacity_decay: opacity_decay.max(0.1),
            direction,
        }
    }

    /// Create a motion blur with default decay, moving in the specified direction.
    #[allow(dead_code)]
    pub fn towards(direction: MotionDirection) -> Self {
        Self {
            direction,
            ..Default::default()
        }
    }

    /// Set the trail length.
    #[allow(dead_code)]
    pub fn with_trail_length(mut self, trail_length: f32) -> Self {
        self.trail_length = trail_length.clamp(0.0, 1.0);
        self
    }

    /// Set the opacity decay exponent.
    #[allow(dead_code)]
    pub fn with_decay(mut self, decay: f32) -> Self {
        self.opacity_decay = decay.max(0.1);
        self
    }

    /// Calculate the normalized position along the motion axis (0.0 = start, 1.0 = end).
    fn motion_position(&self, x: u16, y: u16, width: u16, height: u16) -> f32 {
        match self.direction {
            MotionDirection::Left => {
                // Motion toward right, trail to left
                // Left edge (x=0) is the trailing edge, right edge is leading
                if width == 0 {
                    0.0
                } else {
                    x as f32 / (width - 1).max(1) as f32
                }
            }
            MotionDirection::Right => {
                // Motion toward left, trail to right
                // Right edge is trailing, left edge is leading
                if width == 0 {
                    0.0
                } else {
                    1.0 - (x as f32 / (width - 1).max(1) as f32)
                }
            }
            MotionDirection::Up => {
                // Motion toward bottom, trail upward
                // Top edge is trailing, bottom edge is leading
                if height == 0 {
                    0.0
                } else {
                    y as f32 / (height - 1).max(1) as f32
                }
            }
            MotionDirection::Down => {
                // Motion toward top, trail downward
                // Bottom edge is trailing, top edge is leading
                if height == 0 {
                    0.0
                } else {
                    1.0 - (y as f32 / (height - 1).max(1) as f32)
                }
            }
        }
    }

    /// Calculate the dimming factor for a position.
    /// Returns 1.0 for no dimming (leading edge), 0.0 for full dimming (far trailing edge).
    fn dim_factor(&self, position: f32) -> f32 {
        // Position: 0.0 = trailing edge, 1.0 = leading edge
        // Leading edge should be bright (factor = 1.0)
        // Trailing edge should be dim (factor depends on trail_length)

        if position >= 1.0 - self.trail_length {
            // Within the "bright" zone
            1.0
        } else {
            // In the trailing zone - apply decay
            let trail_start = 1.0 - self.trail_length;
            let normalized_trail_pos = position / trail_start;

            // Apply decay curve
            normalized_trail_pos.powf(self.opacity_decay)
        }
    }

    /// Apply dimming to a color.
    fn dim_color(&self, color: Color, factor: f32) -> Color {
        let factor = factor.clamp(0.0, 1.0);
        Color::rgb(
            (color.r as f32 * factor).round() as u8,
            (color.g as f32 * factor).round() as u8,
            (color.b as f32 * factor).round() as u8,
        )
    }
}

impl Filter for MotionBlur {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, _t: f64) {
        if self.trail_length <= 0.0 {
            return;
        }

        let position = self.motion_position(x, y, width, height);
        let factor = self.dim_factor(position);

        cell.fg = self.dim_color(cell.fg, factor);
        cell.bg = self.dim_color(cell.bg, factor);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    fn make_cell() -> Cell {
        Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(50, 50, 50),
            Modifiers::NONE,
        )
    }

    #[test]
    fn default_values() {
        let filter = MotionBlur::default();
        assert_eq!(filter.trail_length, 0.5);
        assert_eq!(filter.opacity_decay, 1.5);
        assert_eq!(filter.direction, MotionDirection::Left);
    }

    #[test]
    fn trail_length_clamped() {
        let filter = MotionBlur::new(1.5, 1.0, MotionDirection::Left);
        assert_eq!(filter.trail_length, 1.0);

        let filter = MotionBlur::new(-0.5, 1.0, MotionDirection::Left);
        assert_eq!(filter.trail_length, 0.0);
    }

    #[test]
    fn decay_minimum() {
        let filter = MotionBlur::new(0.5, 0.01, MotionDirection::Left);
        assert_eq!(filter.opacity_decay, 0.1);
    }

    #[test]
    fn zero_trail_length_no_change() {
        let filter = MotionBlur::new(0.0, 1.0, MotionDirection::Left);
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn left_direction_dims_left_edge() {
        let filter = MotionBlur::new(0.5, 1.0, MotionDirection::Left);

        // Left edge (x=0) should be dimmer
        let mut cell_left = make_cell();
        filter.apply(&mut cell_left, 0, 5, 10, 10, 0.0);

        // Right edge (x=9) should be brighter (within bright zone)
        let mut cell_right = make_cell();
        filter.apply(&mut cell_right, 9, 5, 10, 10, 0.0);

        assert!(
            cell_left.fg.r < cell_right.fg.r,
            "Left edge should be dimmer than right"
        );
    }

    #[test]
    fn right_direction_dims_right_edge() {
        let filter = MotionBlur::new(0.5, 1.0, MotionDirection::Right);

        // Right edge (x=9) should be dimmer
        let mut cell_right = make_cell();
        filter.apply(&mut cell_right, 9, 5, 10, 10, 0.0);

        // Left edge (x=0) should be brighter
        let mut cell_left = make_cell();
        filter.apply(&mut cell_left, 0, 5, 10, 10, 0.0);

        assert!(
            cell_right.fg.r < cell_left.fg.r,
            "Right edge should be dimmer than left"
        );
    }

    #[test]
    fn up_direction_dims_top_edge() {
        let filter = MotionBlur::new(0.5, 1.0, MotionDirection::Up);

        // Top edge (y=0) should be dimmer
        let mut cell_top = make_cell();
        filter.apply(&mut cell_top, 5, 0, 10, 10, 0.0);

        // Bottom edge (y=9) should be brighter
        let mut cell_bottom = make_cell();
        filter.apply(&mut cell_bottom, 5, 9, 10, 10, 0.0);

        assert!(
            cell_top.fg.r < cell_bottom.fg.r,
            "Top edge should be dimmer than bottom"
        );
    }

    #[test]
    fn down_direction_dims_bottom_edge() {
        let filter = MotionBlur::new(0.5, 1.0, MotionDirection::Down);

        // Bottom edge (y=9) should be dimmer
        let mut cell_bottom = make_cell();
        filter.apply(&mut cell_bottom, 5, 9, 10, 10, 0.0);

        // Top edge (y=0) should be brighter
        let mut cell_top = make_cell();
        filter.apply(&mut cell_top, 5, 0, 10, 10, 0.0);

        assert!(
            cell_bottom.fg.r < cell_top.fg.r,
            "Bottom edge should be dimmer than top"
        );
    }

    #[test]
    fn leading_edge_undimmed() {
        let filter = MotionBlur::new(0.5, 1.0, MotionDirection::Left);

        // With trail_length=0.5, the leading 50% should be at full brightness
        // Right edge is leading
        let mut cell = make_cell();
        filter.apply(&mut cell, 9, 5, 10, 10, 0.0);
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn higher_decay_sharper_falloff() {
        let low_decay = MotionBlur::new(0.9, 1.0, MotionDirection::Left);
        let high_decay = MotionBlur::new(0.9, 3.0, MotionDirection::Left);

        let mut cell_low = make_cell();
        let mut cell_high = make_cell();

        // With trail_length=0.9, trail zone is 0.0-0.1, bright zone is 0.1-1.0
        // x=1 in 10-wide: position = 1/9 ≈ 0.11, just barely in bright zone
        // x=0 in 10-wide: position = 0, fully in trail
        // Test at position 0 (fully in trail zone)
        low_decay.apply(&mut cell_low, 0, 5, 10, 10, 0.0);
        high_decay.apply(&mut cell_high, 0, 5, 10, 10, 0.0);

        // At position 0, normalized_trail_pos = 0, so 0^decay = 0 for any decay
        // Both should be completely black at position 0
        // Test at a position in the middle of the trail: x=0 has position=0, let's check
        // Actually at pos=0, both are black. Let me test at pos that's in trail but not 0.
        // With trail_length=0.9, trail_start=0.1
        // Need position in (0, 0.1) range. For 10-wide widget, only x=0 has position=0.
        // Let me use a larger widget: 20-wide, x=1 gives position=1/19≈0.053
        let mut cell_low = make_cell();
        let mut cell_high = make_cell();
        low_decay.apply(&mut cell_low, 1, 5, 20, 10, 0.0);
        high_decay.apply(&mut cell_high, 1, 5, 20, 10, 0.0);

        // At position ~0.053, trail_start=0.1, normalized=0.53
        // low_decay: 0.53^1.0 = 0.53
        // high_decay: 0.53^3.0 ≈ 0.15
        // Higher decay should be dimmer
        assert!(
            cell_high.fg.r < cell_low.fg.r,
            "Higher decay should produce dimmer result at mid-trail: low={}, high={}",
            cell_low.fg.r,
            cell_high.fg.r
        );
    }

    #[test]
    fn builder_pattern() {
        let filter = MotionBlur::towards(MotionDirection::Up)
            .with_trail_length(0.7)
            .with_decay(2.0);

        assert_eq!(filter.direction, MotionDirection::Up);
        assert_eq!(filter.trail_length, 0.7);
        assert_eq!(filter.opacity_decay, 2.0);
    }

    #[test]
    fn affects_both_fg_and_bg() {
        let filter = MotionBlur::new(0.8, 1.0, MotionDirection::Left);
        let mut cell = make_cell();

        // At trailing edge
        filter.apply(&mut cell, 0, 5, 10, 10, 0.0);

        // Both should be dimmed
        assert!(cell.fg.r < 100);
        assert!(cell.bg.r < 50);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_motion_blur.rs</FILE>
// <DESC>Motion blur trail effect with directional dimming</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
