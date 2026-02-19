// <FILE>crates/tui-vfx-shadow/src/renderers/cls_braille.rs</FILE> - <DESC>Braille pattern shadow renderer for dithered effects</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Remove modifier-alpha feature flag</WCTX>
// <CLOG>mod_alpha always available - remove cfg guards</CLOG>

//! Braille pattern shadow renderer.
//!
//! Uses Unicode braille patterns (U+2800-U+28FF) for density-based shadow rendering.
//! Each braille character provides a 2x4 subpixel grid for fine-grained control.

use tui_vfx_types::{Cell, Color, Grid, Rect};

use crate::types::ShadowConfig;

/// Braille pattern shadow renderer.
///
/// Creates dithered shadows using braille patterns for variable density.
/// Note: Requires terminal font with good braille support for best results.
pub struct BrailleRenderer;

/// Braille pattern base character (empty: ⠀)
const BRAILLE_BASE: u32 = 0x2800;

/// Braille dot positions (each bit represents one dot in the 2x4 grid):
/// ```text
/// 0x01  0x08
/// 0x02  0x10
/// 0x04  0x20
/// 0x40  0x80
/// ```
const BRAILLE_DOTS: [u8; 8] = [0x01, 0x02, 0x04, 0x40, 0x08, 0x10, 0x20, 0x80];

/// Right column dots only (for aspect-corrected right edge shadow)
const BRAILLE_RIGHT_DOTS: [u8; 4] = [0x08, 0x10, 0x20, 0x80];

impl BrailleRenderer {
    /// Render a braille-pattern shadow for the given element rect.
    ///
    /// # Arguments
    /// * `grid` - The grid to render into
    /// * `element_rect` - The rect of the element casting the shadow
    /// * `config` - Shadow configuration
    /// * `density` - Fill density from 0.0 (empty) to 1.0 (full)
    /// * `progress` - Animation progress 0.0-1.0
    pub fn render<G: Grid>(
        grid: &mut G,
        element_rect: Rect,
        config: &ShadowConfig,
        density: f32,
        progress: f64,
    ) {
        let shadow_color = config.color_at_progress(progress);
        if shadow_color.a == 0 {
            return;
        }

        let surface = config.surface_color.unwrap_or(Color::TRANSPARENT);

        // Convert rect fields to i32 for arithmetic with signed offsets
        let rect_x = element_rect.x as i32;
        let rect_y = element_rect.y as i32;
        let rect_w = element_rect.width as i32;
        let rect_h = element_rect.height as i32;

        let ox = config.offset_x as i32;
        let oy = config.offset_y as i32;
        let edges = config.edges;

        // Calculate the effective density based on animation progress
        let effective_density = density * progress as f32;
        let braille_char = Self::density_to_braille(effective_density);

        // Right edge shadow (aspect-corrected: first col = light, second col = right dots)
        if edges.has_right() && ox > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let w = ox as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;

            // First column: light braille (25% density for soft edge)
            let light_char = Self::density_to_braille_right(effective_density * 0.5);
            Self::fill_column(grid, start_x, start_y, h, light_char, shadow_color, surface);

            // Second column: right-half braille (fills right side of cell)
            if w > 1 {
                let right_char = Self::density_to_braille_right(effective_density);
                Self::fill_column(
                    grid,
                    start_x + 1,
                    start_y,
                    h,
                    right_char,
                    shadow_color,
                    surface,
                );
            }

            // Additional columns (if any): full braille
            for dx in 2..w {
                Self::fill_column(
                    grid,
                    start_x + dx,
                    start_y,
                    h,
                    braille_char,
                    shadow_color,
                    surface,
                );
            }
        }

        // Bottom edge shadow
        if edges.has_bottom() && oy > 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = oy as usize;
            Self::fill_region(
                grid,
                start_x,
                start_y,
                w,
                h,
                braille_char,
                shadow_color,
                surface,
            );
        }

        // Left edge shadow
        if edges.has_left() && ox < 0 {
            let start_x = (rect_x + ox).max(0) as usize;
            let start_y = (rect_y + oy.max(0)).max(0) as usize;
            let w = (-ox) as usize;
            let h = (rect_h - oy.abs().min(rect_h)).max(0) as usize;
            Self::fill_region(
                grid,
                start_x,
                start_y,
                w,
                h,
                braille_char,
                shadow_color,
                surface,
            );
        }

        // Top edge shadow
        if edges.has_top() && oy < 0 {
            let start_x = (rect_x + ox.max(0)).max(0) as usize;
            let start_y = (rect_y + oy).max(0) as usize;
            let w = (rect_w - ox.abs().min(rect_w)).max(0) as usize;
            let h = (-oy) as usize;
            Self::fill_region(
                grid,
                start_x,
                start_y,
                w,
                h,
                braille_char,
                shadow_color,
                surface,
            );
        }

        // Corner regions (aspect-corrected to match right edge)
        if edges.has_right() && edges.has_bottom() && ox > 0 && oy > 0 {
            let start_x = (rect_x + rect_w).max(0) as usize;
            let start_y = (rect_y + rect_h).max(0) as usize;
            let w = ox as usize;
            let h = oy as usize;

            // First column: light braille (matches right edge)
            let light_char = Self::density_to_braille_right(effective_density * 0.5);
            Self::fill_column(grid, start_x, start_y, h, light_char, shadow_color, surface);

            // Second column: right-half braille
            if w > 1 {
                let right_char = Self::density_to_braille_right(effective_density);
                Self::fill_column(
                    grid,
                    start_x + 1,
                    start_y,
                    h,
                    right_char,
                    shadow_color,
                    surface,
                );
            }

            // Additional columns: full braille
            for dx in 2..w {
                Self::fill_column(
                    grid,
                    start_x + dx,
                    start_y,
                    h,
                    braille_char,
                    shadow_color,
                    surface,
                );
            }
        }
    }

    /// Convert density (0.0-1.0) to a braille character.
    ///
    /// Maps density to progressively filled braille patterns:
    /// ' ' → ⠁ → ⠃ → ⠇ → ⡇ → ⣇ → ⣧ → ⣷ → ⣿
    #[inline]
    fn density_to_braille(density: f32) -> char {
        let density = density.clamp(0.0, 1.0);
        let dots_to_fill = (density * 8.0).round() as usize;

        let mut pattern: u8 = 0;
        for &dot in BRAILLE_DOTS.iter().take(dots_to_fill) {
            pattern |= dot;
        }

        char::from_u32(BRAILLE_BASE + pattern as u32).unwrap_or(' ')
    }

    /// Convert density to braille using only RIGHT column dots.
    /// Used for aspect-corrected right edge shadow.
    #[inline]
    fn density_to_braille_right(density: f32) -> char {
        let density = density.clamp(0.0, 1.0);
        let dots_to_fill = (density * 4.0).round() as usize;

        let mut pattern: u8 = 0;
        for &dot in BRAILLE_RIGHT_DOTS.iter().take(dots_to_fill) {
            pattern |= dot;
        }

        char::from_u32(BRAILLE_BASE + pattern as u32).unwrap_or(' ')
    }

    /// Fill a rectangular region with braille pattern cells.
    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn fill_region<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        ch: char,
        fg: Color,
        bg: Color,
    ) {
        let cell = Cell::new(ch)
            .with_fg(fg)
            .with_bg(bg)
            .with_mod_alpha(Some(255));
        for dy in 0..h {
            for dx in 0..w {
                let px = x + dx;
                let py = y + dy;
                if grid.in_bounds(px, py) {
                    grid.set(px, py, cell);
                }
            }
        }
    }

    /// Fill a single column with braille pattern cells.
    #[inline]
    fn fill_column<G: Grid>(
        grid: &mut G,
        x: usize,
        y: usize,
        h: usize,
        ch: char,
        fg: Color,
        bg: Color,
    ) {
        let cell = Cell::new(ch)
            .with_fg(fg)
            .with_bg(bg)
            .with_mod_alpha(Some(255));
        for dy in 0..h {
            let py = y + dy;
            if grid.in_bounds(x, py) {
                grid.set(x, py, cell);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ShadowEdges;
    use tui_vfx_types::OwnedGrid;

    #[test]
    fn test_density_to_braille() {
        // Empty
        assert_eq!(BrailleRenderer::density_to_braille(0.0), '⠀');
        // Full
        assert_eq!(BrailleRenderer::density_to_braille(1.0), '⣿');
        // Approximately half
        let half = BrailleRenderer::density_to_braille(0.5);
        assert!(half != '⠀' && half != '⣿');
    }

    #[test]
    fn test_render_basic_shadow() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200))
            .with_offset(2, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        BrailleRenderer::render(&mut grid, rect, &config, 0.8, 1.0);

        // Check that shadow exists at expected positions
        let cell = grid.get(13, 3).unwrap();
        assert_ne!(cell.ch, ' ');
    }

    #[test]
    fn test_zero_density_renders_empty_braille() {
        let mut grid = OwnedGrid::new(20, 10);
        let rect = Rect::new(5, 2, 8, 4);
        let config = ShadowConfig::new(Color::BLACK)
            .with_offset(1, 1)
            .with_edges(ShadowEdges::BOTTOM_RIGHT);

        BrailleRenderer::render(&mut grid, rect, &config, 0.0, 1.0);

        // Shadow region should have empty braille
        let cell = grid.get(13, 3).unwrap();
        assert_eq!(cell.ch, '⠀');
    }
}

// <FILE>crates/tui-vfx-shadow/src/renderers/cls_braille.rs</FILE> - <DESC>Braille pattern shadow renderer for dithered effects</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
