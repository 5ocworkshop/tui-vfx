// <FILE>tui-vfx-compositor/src/filters/cls_dot_indicator.rs</FILE>
// <DESC>Simple dot/bullet that appears adjacent to content</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use crate::types::HoverBarPosition;
use tui_vfx_types::{Cell, Color};

/// Simple dot/bullet indicator that appears adjacent to content.
///
/// Draws a single indicator character (default: •) at the edge of the content,
/// fading in based on animation progress.
///
/// # Usage
///
/// Apply to content where you want a simple selection indicator:
/// - List item selection: `• Selected item`
/// - Navigation markers: `→ Current page`
/// - Bullet points: `◆ Important item`
pub struct DotIndicator {
    /// Indicator character (default: •)
    pub indicator_char: char,
    /// Position (Left or Right)
    pub position: HoverBarPosition,
    /// Indicator color
    pub color: Color,
    /// Background color for blending
    pub bg_color: Color,
    /// Progress (0.0 = invisible, 1.0 = fully visible)
    pub progress: f32,
}

impl Default for DotIndicator {
    fn default() -> Self {
        Self {
            indicator_char: '•',
            position: HoverBarPosition::Left,
            color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
            progress: 0.0,
        }
    }
}

impl DotIndicator {
    /// Create a new DotIndicator with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the indicator character.
    pub fn with_char(mut self, ch: char) -> Self {
        self.indicator_char = ch;
        self
    }

    /// Set the position.
    pub fn with_position(mut self, position: HoverBarPosition) -> Self {
        self.position = position;
        self
    }

    /// Set the indicator color.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the background color for blending.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the animation progress.
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Blend two colors based on factor (0.0 = a, 1.0 = b)
    fn blend(a: Color, b: Color, factor: f32) -> Color {
        a.lerp(b, factor)
    }
}

impl Filter for DotIndicator {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, _t: f64) {
        if self.progress <= 0.0 {
            return;
        }

        // Check if this is the target cell based on position
        let is_target = match self.position {
            HoverBarPosition::Left => x == 0,
            HoverBarPosition::Right => x == width.saturating_sub(1),
            HoverBarPosition::Top => y == 0 && x == width / 2,
            HoverBarPosition::Bottom => y == height.saturating_sub(1) && x == width / 2,
        };

        if !is_target {
            return;
        }

        // Apply indicator with faded color based on progress
        let blended_color = Self::blend(self.bg_color, self.color, self.progress);

        cell.ch = self.indicator_char;
        cell.fg = blended_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::test_support::make_cell;

    #[test]
    fn default_values() {
        let filter = DotIndicator::default();
        assert_eq!(filter.indicator_char, '•');
        assert_eq!(filter.position, HoverBarPosition::Left);
        assert_eq!(filter.progress, 0.0);
    }

    #[test]
    fn builder_pattern() {
        let filter = DotIndicator::new()
            .with_char('→')
            .with_position(HoverBarPosition::Right)
            .with_color(Color::rgb(255, 0, 0))
            .with_progress(0.75);

        assert_eq!(filter.indicator_char, '→');
        assert_eq!(filter.position, HoverBarPosition::Right);
        assert_eq!(filter.progress, 0.75);
    }

    #[test]
    fn zero_progress_no_change() {
        let filter = DotIndicator::new().with_progress(0.0);

        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn full_progress_draws_left_dot() {
        let filter = DotIndicator::new()
            .with_position(HoverBarPosition::Left)
            .with_progress(1.0)
            .with_color(Color::rgb(100, 150, 200));

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, '•');
        assert_eq!(cell.fg, Color::rgb(100, 150, 200));
    }

    #[test]
    fn full_progress_draws_right_dot() {
        let filter = DotIndicator::new()
            .with_position(HoverBarPosition::Right)
            .with_progress(1.0);

        let mut cell = make_cell();
        // Right edge (x=9 for width=10)
        filter.apply(&mut cell, 9, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, '•');
    }

    #[test]
    fn only_affects_target_cell() {
        let filter = DotIndicator::new()
            .with_position(HoverBarPosition::Left)
            .with_progress(1.0);

        // Interior cell should not be modified
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn partial_progress_blends_color() {
        let filter = DotIndicator::new()
            .with_bg_color(Color::rgb(0, 0, 0))
            .with_color(Color::rgb(100, 200, 100))
            .with_progress(0.5);

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        // At 50% progress, color should be blended
        assert_eq!(cell.fg, Color::rgb(50, 100, 50));
    }

    #[test]
    fn custom_indicator_char() {
        let filter = DotIndicator::new().with_char('◆').with_progress(1.0);

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, '◆');
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_dot_indicator.rs</FILE>
// <DESC>Simple dot/bullet that appears adjacent to content</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
