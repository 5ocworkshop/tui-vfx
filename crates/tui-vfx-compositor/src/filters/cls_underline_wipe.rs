// <FILE>tui-vfx-compositor/src/filters/cls_underline_wipe.rs</FILE>
// <DESC>Horizontal underline that wipes in based on progress</DESC>
// <VERS>VERSION: 1.3.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use crate::types::WipeDirection;
use tui_vfx_types::{Cell, Color};

/// Horizontal underline that wipes in based on progress.
///
/// Draws a line character (default: —) progressively revealed based on
/// animation progress and wipe direction. Supports gradient coloring and
/// glisten effects.
///
/// The default `—` (emdash) positions the line at the vertical center of
/// the cell, providing balanced visual separation from content above.
///
/// # Usage
///
/// Apply to content where you want an animated underline effect:
/// - Menu items gaining focus
/// - Links being hovered
/// - Selection indicators
pub struct UnderlineWipe {
    /// Wipe direction (LeftToRight or RightToLeft)
    pub direction: WipeDirection,
    /// Line color (target color for gradient)
    pub color: Color,
    /// Background color (start color for gradient)
    pub bg_color: Color,
    /// Character for the line (default: — emdash)
    pub line_char: char,
    /// Row offset from bottom (0 = last row)
    pub row_offset: u16,
    /// Progress (0.0 = none, 1.0 = full width)
    pub progress: f32,
    /// Enable gradient from bg_color to color
    pub gradient: bool,
    /// Enable glisten/shimmer effect
    pub glisten: bool,
}

impl Default for UnderlineWipe {
    fn default() -> Self {
        Self {
            direction: WipeDirection::LeftToRight,
            color: Color::rgb(100, 150, 200),
            bg_color: Color::rgb(30, 30, 30),
            line_char: '—',
            row_offset: 0,
            progress: 0.0,
            gradient: true,
            glisten: true,
        }
    }
}

impl UnderlineWipe {
    /// Create a new UnderlineWipe with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the wipe direction.
    pub fn with_direction(mut self, direction: WipeDirection) -> Self {
        self.direction = direction;
        self
    }

    /// Set the line color.
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the background color for gradient.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the line character.
    pub fn with_line_char(mut self, ch: char) -> Self {
        self.line_char = ch;
        self
    }

    /// Set the row offset from bottom.
    pub fn with_row_offset(mut self, offset: u16) -> Self {
        self.row_offset = offset;
        self
    }

    /// Set the animation progress.
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
        self
    }

    /// Enable or disable gradient effect.
    pub fn with_gradient(mut self, gradient: bool) -> Self {
        self.gradient = gradient;
        self
    }

    /// Enable or disable glisten effect.
    pub fn with_glisten(mut self, glisten: bool) -> Self {
        self.glisten = glisten;
        self
    }

    /// Interpolate between two colors.
    fn lerp_color(a: Color, b: Color, t: f32) -> Color {
        let t = t.clamp(0.0, 1.0);
        let (r1, g1, b1) = a.to_rgb();
        let (r2, g2, b2) = b.to_rgb();
        Color::rgb(
            (r1 as f32 + (r2 as f32 - r1 as f32) * t) as u8,
            (g1 as f32 + (g2 as f32 - g1 as f32) * t) as u8,
            (b1 as f32 + (b2 as f32 - b1 as f32) * t) as u8,
        )
    }
}

impl Filter for UnderlineWipe {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        // Check if this is the target row (bottom - offset)
        let target_row = height.saturating_sub(1 + self.row_offset);
        if y != target_row {
            return;
        }

        if self.progress <= 0.0 {
            return;
        }

        // Calculate which cells should have the underline based on progress
        let revealed_width = (width as f32 * self.progress).ceil() as u16;

        let (should_draw, position_ratio) = match self.direction {
            WipeDirection::LeftToRight => {
                let draw = x < revealed_width;
                let ratio = if revealed_width > 0 {
                    x as f32 / revealed_width as f32
                } else {
                    0.0
                };
                (draw, ratio)
            }
            WipeDirection::RightToLeft => {
                let start = width.saturating_sub(revealed_width);
                let draw = x >= start;
                let ratio = if revealed_width > 0 {
                    (width.saturating_sub(1).saturating_sub(x)) as f32 / revealed_width as f32
                } else {
                    0.0
                };
                (draw, ratio)
            }
            WipeDirection::TopToBottom | WipeDirection::BottomToTop => {
                // For vertical directions, treat as full width wipe at progress
                (self.progress > 0.5, x as f32 / width.max(1) as f32)
            }
            _ => (x < revealed_width, x as f32 / revealed_width.max(1) as f32),
        };

        if should_draw {
            cell.ch = self.line_char;

            // Calculate base color (with optional gradient)
            let base_color = if self.gradient {
                // Gradient from bg_color at edge to color at wipe front
                Self::lerp_color(self.bg_color, self.color, 1.0 - position_ratio)
            } else {
                self.color
            };

            // Apply glisten effect (subtle shimmer moving along the line)
            let final_color = if self.glisten {
                // Create a slow, subtle moving highlight based on time
                let glisten_pos = ((t * 0.45) % 1.0) as f32; // ~2.2 second cycle
                let cell_pos = x as f32 / width.max(1) as f32;

                // Distance from glisten position (wrapping)
                let dist = (cell_pos - glisten_pos)
                    .abs()
                    .min((cell_pos - glisten_pos + 1.0).abs());
                let glisten_width = 0.2; // Width of the glisten highlight

                if dist < glisten_width {
                    // Apply subtle highlight - gently brighten the color
                    let intensity = 1.0 - (dist / glisten_width);
                    let boost = (intensity * 30.0) as u8; // Subtle boost
                    let (r, g, b) = base_color.to_rgb();
                    Color::rgb(
                        r.saturating_add(boost),
                        g.saturating_add(boost),
                        b.saturating_add(boost),
                    )
                } else {
                    base_color
                }
            } else {
                base_color
            };

            cell.fg = final_color;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::filters::test_support::make_cell;

    #[test]
    fn default_values() {
        let filter = UnderlineWipe::default();
        assert_eq!(filter.line_char, '—');
        assert_eq!(filter.row_offset, 0);
        assert_eq!(filter.progress, 0.0);
        assert!(filter.gradient);
        assert!(filter.glisten);
    }

    #[test]
    fn builder_pattern() {
        let filter = UnderlineWipe::new()
            .with_direction(WipeDirection::RightToLeft)
            .with_color(Color::rgb(255, 0, 0))
            .with_bg_color(Color::rgb(50, 50, 50))
            .with_line_char('─')
            .with_row_offset(1)
            .with_progress(0.5)
            .with_gradient(false)
            .with_glisten(false);

        assert_eq!(filter.direction, WipeDirection::RightToLeft);
        assert_eq!(filter.line_char, '─');
        assert_eq!(filter.row_offset, 1);
        assert_eq!(filter.progress, 0.5);
        assert!(!filter.gradient);
        assert!(!filter.glisten);
    }

    #[test]
    fn zero_progress_no_change() {
        let filter = UnderlineWipe::new().with_progress(0.0);

        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 0, 9, 10, 10, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn full_progress_draws_underline() {
        let filter = UnderlineWipe::new()
            .with_progress(1.0)
            .with_color(Color::rgb(100, 150, 200))
            .with_gradient(false)
            .with_glisten(false);

        let mut cell = make_cell();
        // Bottom row (y=9 for height=10)
        filter.apply(&mut cell, 0, 9, 10, 10, 0.0);

        assert_eq!(cell.ch, '—');
        assert_eq!(cell.fg, Color::rgb(100, 150, 200));
    }

    #[test]
    fn only_affects_bottom_row() {
        let filter = UnderlineWipe::new().with_progress(1.0);

        // Not bottom row
        let mut cell = make_cell();
        let original = cell;
        filter.apply(&mut cell, 0, 5, 10, 10, 0.0);

        assert_eq!(cell.ch, original.ch);
    }

    #[test]
    fn row_offset_works() {
        let filter = UnderlineWipe::new().with_progress(1.0).with_row_offset(2);

        // Should affect row 7 (9 - 2) for height=10
        let mut cell = make_cell();
        filter.apply(&mut cell, 0, 7, 10, 10, 0.0);

        assert_eq!(cell.ch, '—');
    }

    #[test]
    fn partial_progress_left_to_right() {
        let filter = UnderlineWipe::new()
            .with_direction(WipeDirection::LeftToRight)
            .with_progress(0.5)
            .with_gradient(false)
            .with_glisten(false);

        // At 50% progress on width=10, cells 0-4 should be drawn
        let mut cell = make_cell();
        filter.apply(&mut cell, 4, 9, 10, 10, 0.0);
        assert_eq!(cell.ch, '—');

        let mut cell = make_cell();
        filter.apply(&mut cell, 5, 9, 10, 10, 0.0);
        assert_eq!(cell.ch, ' ');
    }

    #[test]
    fn gradient_produces_varying_colors() {
        let filter = UnderlineWipe::new()
            .with_progress(1.0)
            .with_color(Color::rgb(200, 200, 200))
            .with_bg_color(Color::rgb(50, 50, 50))
            .with_gradient(true)
            .with_glisten(false);

        // First cell should be closer to target color
        let mut cell1 = make_cell();
        filter.apply(&mut cell1, 0, 9, 10, 10, 0.0);

        // Last cell should be closer to bg color
        let mut cell2 = make_cell();
        filter.apply(&mut cell2, 9, 9, 10, 10, 0.0);

        // They should have different colors
        assert_ne!(cell1.fg, cell2.fg);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_underline_wipe.rs</FILE>
// <DESC>Horizontal underline that wipes in based on progress</DESC>
// <VERS>END OF VERSION: 1.3.2</VERS>
