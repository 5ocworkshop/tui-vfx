// <FILE>tui-vfx-compositor/src/filters/cls_vignette.rs</FILE> - <DESC>Vignette filter with proper spatial radial gradient</DESC>
// <VERS>VERSION: 3.0.1</VERS>
// <WCTX>Fix brightness jump at animation completion</WCTX>
// <CLOG>Use round() instead of truncation in color math to prevent off-by-one errors</CLOG>

use crate::traits::filter::Filter;
use tui_vfx_types::{Cell, Color};

/// Vignette filter that darkens edges based on radial distance from center.
///
/// Creates a classic vignette effect where the image is bright in the center
/// and gradually darkens toward the edges, following a radial gradient pattern.
///
/// # Spatial Implementation
///
/// This filter REQUIRES spatial context to function correctly. It calculates
/// the distance from each cell to the center of the rendering area and applies
/// dimming proportional to that distance.
///
/// # Parameters
///
/// - `strength`: Maximum dimming factor at the edges (0.0-1.0)
/// - `radius`: Normalized distance threshold (0.0-1.0, where 0.0 = center, 1.0 = corner)
///
/// Cells within the radius are unaffected. Cells beyond the radius are dimmed
/// proportionally to their distance, reaching maximum dimming at the corners.
pub struct Vignette {
    /// Strength of the vignette effect (0.0 = no effect, 1.0 = full darkness at edges)
    pub strength: f32,
    /// Normalized radius where dimming begins (0.0 = center, 1.0 = corner)
    pub radius: f32,
}

impl Default for Vignette {
    fn default() -> Self {
        Self::new(0.6, 0.5)
    }
}

impl Vignette {
    /// Create a new Vignette filter with given strength and radius.
    ///
    /// # Parameters
    ///
    /// - `strength`: Dimming strength (0.0-1.0, clamped)
    /// - `radius`: Threshold radius (0.0-1.0, clamped)
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Subtle vignette starting halfway to edges
    /// let vignette = Vignette::new(0.3, 0.5);
    ///
    /// // Strong vignette with tight radius
    /// let strong = Vignette::new(0.8, 0.3);
    /// ```
    pub fn new(strength: f32, radius: f32) -> Self {
        Self {
            strength: strength.clamp(0.0, 1.0),
            radius: radius.clamp(0.0, 1.0),
        }
    }

    /// Dim a color by the given factor.
    fn dim_color(color: Color, factor: f32) -> Color {
        // tui_vfx_types::Color always has RGB components
        // Use round() to prevent off-by-one errors at boundary values
        let dim = 1.0 - factor;
        Color::rgb(
            (color.r as f32 * dim).round() as u8,
            (color.g as f32 * dim).round() as u8,
            (color.b as f32 * dim).round() as u8,
        )
    }
}

impl Filter for Vignette {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, _t: f64) {
        // Handle zero dimensions gracefully
        if width == 0 || height == 0 {
            return;
        }

        // Calculate center coordinates
        let cx = width as f32 / 2.0;
        let cy = height as f32 / 2.0;

        // Calculate distance from cell to center
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;
        let dist = (dx * dx + dy * dy).sqrt();

        // Calculate maximum possible distance (corner to center)
        let max_dist = (cx * cx + cy * cy).sqrt();

        // Normalize distance (0.0 at center, 1.0 at corner)
        let norm_dist = if max_dist > 0.0 { dist / max_dist } else { 0.0 };

        // Apply dimming if beyond radius threshold
        if norm_dist > self.radius {
            // Calculate dimming factor (0.0 at radius, strength at corner)
            let beyond_radius = norm_dist - self.radius;
            let radius_range = 1.0 - self.radius;
            let dim_factor = if radius_range > 0.0 {
                self.strength * (beyond_radius / radius_range)
            } else {
                self.strength
            };

            // Apply dimming to both foreground and background
            cell.fg = Self::dim_color(cell.fg, dim_factor);
            cell.bg = Self::dim_color(cell.bg, dim_factor);
        }
        // Cells within radius remain unaffected
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Modifiers;

    #[test]
    fn test_vignette_center_unchanged() {
        let vignette = Vignette::new(1.0, 0.0);
        let mut center = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        // Cell at exact center (5,5) in 10x10 grid
        vignette.apply(&mut center, 5, 5, 10, 10, 0.0);
        // Center has norm_dist ~0, within any radius, so no dimming
        assert_eq!(center.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_vignette_corner_fully_dimmed() {
        let vignette = Vignette::new(1.0, 0.0);
        let mut corner = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        // Cell at corner (0,0) in 10x10 grid - max distance from center
        vignette.apply(&mut corner, 0, 0, 10, 10, 0.0);
        // At corner with strength 1.0 and radius 0.0, should be fully dimmed
        assert_eq!(corner.fg, Color::rgb(0, 0, 0));
        assert_eq!(corner.bg, Color::rgb(0, 0, 0));
    }

    #[test]
    fn test_vignette_zero_dimensions_noop() {
        let vignette = Vignette::new(1.0, 0.0);
        let mut cell = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        vignette.apply(&mut cell, 0, 0, 0, 0, 0.0);
        // Should return early, no change
        assert_eq!(cell.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn test_vignette_partial_strength() {
        let vignette = Vignette::new(0.5, 0.0);
        let mut corner = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        vignette.apply(&mut corner, 0, 0, 10, 10, 0.0);
        // 50% dim at corner: 100 * 0.5 = 50
        assert_eq!(corner.fg, Color::rgb(50, 50, 50));
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_vignette.rs</FILE> - <DESC>Vignette filter with proper spatial radial gradient</DESC>
// <VERS>END OF VERSION: 3.0.1</VERS>
