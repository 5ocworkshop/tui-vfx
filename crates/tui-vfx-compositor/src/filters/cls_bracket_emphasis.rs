// <FILE>tui-vfx-compositor/src/filters/cls_bracket_emphasis.rs</FILE>
// <DESC>Brackets that appear around content based on progress</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Brackets that appear around content based on progress.
///
/// Draws bracket characters at the left and right edges of the content area,
/// with color fading in based on animation progress.
///
/// # Usage
///
/// Apply to content where you want emphasis brackets:
/// - Selected menu items: `[ Option ]`
/// - Highlighted choices: `» Item «`
/// - Focus indicators: `⟦ Content ⟧`
pub struct BracketEmphasis {
    /// Left bracket character
    pub left: char,
    /// Right bracket character
    pub right: char,
    /// Bracket color
    pub color: Color,
    /// Background color for blending
    pub bg_color: Color,
    /// Progress (0.0 = invisible, 1.0 = fully visible)
    pub progress: f32,
}

impl Default for BracketEmphasis {
    fn default() -> Self {
        Self {
            left: '[',
            right: ']',
            color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
            progress: 0.0,
        }
    }
}

impl BracketEmphasis {
    /// Create a new BracketEmphasis with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the left bracket character.
    pub fn with_left(mut self, ch: char) -> Self {
        self.left = ch;
        self
    }

    /// Set the right bracket character.
    pub fn with_right(mut self, ch: char) -> Self {
        self.right = ch;
        self
    }

    /// Set both bracket characters at once.
    #[allow(dead_code)]
    pub fn with_brackets(mut self, left: char, right: char) -> Self {
        self.left = left;
        self.right = right;
        self
    }

    /// Set the bracket color.
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

impl Filter for BracketEmphasis {
    fn apply(&self, cell: &mut Cell, x: u16, _y: u16, width: u16, _height: u16, _t: f64) {
        if self.progress <= 0.0 {
            return;
        }

        let is_left_edge = x == 0;
        let is_right_edge = x == width.saturating_sub(1);

        if !is_left_edge && !is_right_edge {
            return;
        }

        // Determine which bracket to draw
        let bracket_char = if is_left_edge { self.left } else { self.right };

        // Apply bracket with faded color based on progress
        let blended_color = Self::blend(self.bg_color, self.color, self.progress);

        cell.ch = bracket_char;
        cell.fg = blended_color;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::test_support::make_cell;

    #[test]
    fn default_values() {
        let filter = BracketEmphasis::default();
        assert_eq!(filter.left, '[');
        assert_eq!(filter.right, ']');
        assert_eq!(filter.progress, 0.0);
    }

    #[test]
    fn builder_pattern() {
        let filter = BracketEmphasis::new()
            .with_left('«')
            .with_right('»')
            .with_color(Color::rgb(255, 0, 0))
            .with_progress(0.5);

        assert_eq!(filter.left, '«');
        assert_eq!(filter.right, '»');
        assert_eq!(filter.progress, 0.5);
    }

    #[test]
    fn with_brackets_sets_both() {
        let filter = BracketEmphasis::new().with_brackets('⟦', '⟧');

        assert_eq!(filter.left, '⟦');
        assert_eq!(filter.right, '⟧');
    }

    #[test]
    fn zero_progress_no_change() {
        let filter = BracketEmphasis::new().with_progress(0.0);

        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn full_progress_draws_left_bracket() {
        let filter = BracketEmphasis::new()
            .with_progress(1.0)
            .with_color(Color::rgb(100, 150, 200));

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, '[');
        assert_eq!(cell.fg, Color::rgb(100, 150, 200));
    }

    #[test]
    fn full_progress_draws_right_bracket() {
        let filter = BracketEmphasis::new().with_progress(1.0);

        let mut cell = make_cell();
        // Right edge (x=9 for width=10)
        filter.apply(&mut cell, 9, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, ']');
    }

    #[test]
    fn only_affects_edges() {
        let filter = BracketEmphasis::new().with_progress(1.0);

        // Interior cell should not be modified
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 5, 0, 10, 1, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn partial_progress_blends_color() {
        let filter = BracketEmphasis::new()
            .with_bg_color(Color::rgb(0, 0, 0))
            .with_color(Color::rgb(100, 200, 100))
            .with_progress(0.5);

        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 0, 10, 1, 0.0);

        // At 50% progress, color should be blended
        assert_eq!(cell.fg, Color::rgb(50, 100, 50));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_bracket_emphasis.rs</FILE>
// <DESC>Brackets that appear around content based on progress</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
