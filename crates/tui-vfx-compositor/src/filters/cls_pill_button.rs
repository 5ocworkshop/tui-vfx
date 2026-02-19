// <FILE>tui-vfx-compositor/src/filters/cls_pill_button.rs</FILE>
// <DESC>Pill-shaped button with gradient edges and optional glisten</DESC>
// <VERS>VERSION: 1.0.2</VERS>
// <WCTX>Consolidate filter test helpers</WCTX>
// <CLOG>Use shared test cell helper</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Pill-shaped button effect with gradient edges.
///
/// Creates a button appearance with:
/// - No hard borders
/// - Horizontal gradients on left and right edges (bg → button → bg)
/// - Text row in the middle (for 3-row buttons)
/// - Optional glisten effect for hover states
///
/// # Usage
///
/// Apply to a 3-row area containing centered text:
/// ```text
/// ░░░▓▓▓███ Button ███▓▓▓░░░
/// ```
pub struct PillButton {
    /// Button/fill color
    pub button_color: Color,
    /// Background color (for gradient edges)
    pub bg_color: Color,
    /// Width of gradient edge in cells
    pub edge_width: u16,
    /// Enable glisten/shimmer effect
    pub glisten: bool,
    /// Hover progress (0.0 = not hovered, 1.0 = fully hovered)
    pub progress: f32,
}

impl Default for PillButton {
    fn default() -> Self {
        Self {
            button_color: Color::rgb(80, 120, 180),
            bg_color: Color::rgb(30, 30, 35),
            edge_width: 3,
            glisten: true,
            progress: 0.0,
        }
    }
}

impl PillButton {
    /// Create a new PillButton with default settings.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the button color.
    pub fn with_button_color(mut self, color: Color) -> Self {
        self.button_color = color;
        self
    }

    /// Set the background color.
    pub fn with_bg_color(mut self, color: Color) -> Self {
        self.bg_color = color;
        self
    }

    /// Set the gradient edge width.
    pub fn with_edge_width(mut self, width: u16) -> Self {
        self.edge_width = width;
        self
    }

    /// Enable or disable glisten effect.
    pub fn with_glisten(mut self, glisten: bool) -> Self {
        self.glisten = glisten;
        self
    }

    /// Set the hover progress.
    pub fn with_progress(mut self, progress: f32) -> Self {
        self.progress = progress.clamp(0.0, 1.0);
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

impl Filter for PillButton {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        // Calculate gradient position
        let edge_w = self.edge_width.min(width / 2);

        // Determine base color based on x position
        let base_color = if x < edge_w {
            // Left gradient: bg → button
            let ratio = x as f32 / edge_w as f32;
            Self::lerp_color(self.bg_color, self.button_color, ratio)
        } else if x >= width.saturating_sub(edge_w) {
            // Right gradient: button → bg
            let from_right = width.saturating_sub(1).saturating_sub(x);
            let ratio = from_right as f32 / edge_w as f32;
            Self::lerp_color(self.bg_color, self.button_color, ratio)
        } else {
            // Center: solid button color
            self.button_color
        };

        // Apply hover brightening based on progress
        let hover_color = if self.progress > 0.0 {
            let boost = (self.progress * 25.0) as u8;
            let (r, g, b) = base_color.to_rgb();
            Color::rgb(
                r.saturating_add(boost),
                g.saturating_add(boost),
                b.saturating_add(boost),
            )
        } else {
            base_color
        };

        // Apply glisten effect when hovered
        let final_color = if self.glisten && self.progress > 0.5 {
            // Glisten only when mostly hovered
            let glisten_pos = ((t * 0.45) % 1.0) as f32; // ~2.2 second cycle
            let cell_pos = x as f32 / width.max(1) as f32;

            // Distance from glisten position (wrapping)
            let dist = (cell_pos - glisten_pos)
                .abs()
                .min((cell_pos - glisten_pos + 1.0).abs());
            let glisten_width = 0.2;

            if dist < glisten_width {
                let intensity = 1.0 - (dist / glisten_width);
                let boost = (intensity * 35.0 * self.progress) as u8;
                let (r, g, b) = hover_color.to_rgb();
                Color::rgb(
                    r.saturating_add(boost),
                    g.saturating_add(boost),
                    b.saturating_add(boost),
                )
            } else {
                hover_color
            }
        } else {
            hover_color
        };

        // Apply to background - preserve the character/text
        cell.bg = final_color;

        // For middle row, keep text visible; for top/bottom rows, use shade chars
        let middle_row = height / 2;
        if y != middle_row && cell.ch == ' ' {
            // Top and bottom rows: use subtle shade for depth
            if y == 0 || y == height.saturating_sub(1) {
                // Slightly darker for top/bottom edge rows
                let (r, g, b) = final_color.to_rgb();
                cell.bg = Color::rgb(
                    r.saturating_sub(10),
                    g.saturating_sub(10),
                    b.saturating_sub(10),
                );
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
        let filter = PillButton::default();
        assert_eq!(filter.edge_width, 3);
        assert!(filter.glisten);
        assert_eq!(filter.progress, 0.0);
    }

    #[test]
    fn builder_pattern() {
        let filter = PillButton::new()
            .with_button_color(Color::rgb(100, 150, 200))
            .with_bg_color(Color::rgb(20, 20, 25))
            .with_edge_width(4)
            .with_glisten(false)
            .with_progress(0.5);

        assert_eq!(filter.edge_width, 4);
        assert!(!filter.glisten);
        assert_eq!(filter.progress, 0.5);
    }

    #[test]
    fn center_gets_button_color() {
        let filter = PillButton::new()
            .with_button_color(Color::rgb(100, 100, 100))
            .with_bg_color(Color::rgb(0, 0, 0))
            .with_edge_width(2)
            .with_glisten(false);

        let mut cell = make_cell();
        // Center cell (x=5 in width=10, edge=2)
        filter.apply(&mut cell, 5, 1, 10, 3, 0.0);

        assert_eq!(cell.bg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn left_edge_is_gradient() {
        let filter = PillButton::new()
            .with_button_color(Color::rgb(100, 100, 100))
            .with_bg_color(Color::rgb(0, 0, 0))
            .with_edge_width(4)
            .with_glisten(false);

        let mut cell1 = make_cell();
        let mut cell2 = make_cell();

        // First cell should be closer to bg
        filter.apply(&mut cell1, 0, 1, 10, 3, 0.0);
        // Cell further in should be closer to button
        filter.apply(&mut cell2, 3, 1, 10, 3, 0.0);

        // cell2 should be brighter (closer to button color)
        let (r1, _, _) = cell1.bg.to_rgb();
        let (r2, _, _) = cell2.bg.to_rgb();
        assert!(r2 > r1);
    }

    #[test]
    fn hover_brightens() {
        let filter_no_hover = PillButton::new()
            .with_button_color(Color::rgb(100, 100, 100))
            .with_glisten(false)
            .with_progress(0.0);

        let filter_hover = PillButton::new()
            .with_button_color(Color::rgb(100, 100, 100))
            .with_glisten(false)
            .with_progress(1.0);

        let mut cell1 = make_cell();
        let mut cell2 = make_cell();

        filter_no_hover.apply(&mut cell1, 5, 1, 10, 3, 0.0);
        filter_hover.apply(&mut cell2, 5, 1, 10, 3, 0.0);

        let (r1, _, _) = cell1.bg.to_rgb();
        let (r2, _, _) = cell2.bg.to_rgb();
        assert!(r2 > r1);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_pill_button.rs</FILE>
// <DESC>Pill-shaped button with gradient edges and optional glisten</DESC>
// <VERS>END OF VERSION: 1.0.2</VERS>
