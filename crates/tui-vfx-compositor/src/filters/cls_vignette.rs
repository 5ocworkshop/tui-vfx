// <FILE>tui-vfx-compositor/src/filters/cls_vignette.rs</FILE> - <DESC>Vignette filter with proper spatial radial gradient</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>Adopt the new mixed-signals surface-space basis for optical falloff consumers where the mapping is now direct and verified.</WCTX>
// <CLOG>3.1.0: use mixed-signals sample_surface_radius for classic all-sides vignette evaluation instead of open-coding frame-center distance math.
// 3.0.1: Use round() instead of truncation in color math to prevent off-by-one errors</CLOG>

use crate::traits::filter::Filter;
use mixed_signals::prelude::{Signal, SignalContext, SpatialCoordinateSignal};
use tui_vfx_types::{Cell, Color};

/// Which edge a directional vignette can originate from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum VignetteEdge {
    #[default]
    Top,
    Bottom,
    Left,
    Right,
}

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
    /// Which edges the directional darkening originates from.
    ///
    /// Empty means the classic all-sides radial vignette.
    pub sides: Vec<VignetteEdge>,
    /// Optional low-amplitude spatial dither applied to the normalized
    /// distance before radius/falloff evaluation. Helps reduce obvious square
    /// contouring on large flat fields. 0.0 = disabled.
    pub dither_amount: f32,
    /// Optional temporal rate for the dither pattern in Hz. 0.0 = static.
    pub temporal_dither_hz: f32,
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
            sides: Vec::new(),
            dither_amount: 0.0,
            temporal_dither_hz: 0.0,
        }
    }

    /// Set the edge set for directional vignette behavior.
    pub fn with_sides(mut self, sides: impl Into<Vec<VignetteEdge>>) -> Self {
        self.sides = sides.into();
        self
    }

    /// Add subtle contour-softening dither.
    pub fn with_dither(mut self, amount: f32, hz: f32) -> Self {
        self.dither_amount = amount.clamp(0.0, 0.25);
        self.temporal_dither_hz = hz.max(0.0);
        self
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

    fn dither_offset(&self, x: u16, y: u16, t: f64) -> f32 {
        if self.dither_amount <= 0.0 {
            return 0.0;
        }

        let time_step = if self.temporal_dither_hz > 0.0 {
            (t * self.temporal_dither_hz as f64).floor() as u32
        } else {
            0
        };

        let hash = (x as u32).wrapping_mul(73856093)
            ^ (y as u32).wrapping_mul(19349663)
            ^ time_step.wrapping_mul(83492791);
        let unit = (hash % 1024) as f32 / 1023.0;
        (unit - 0.5) * 2.0 * self.dither_amount
    }
}

impl Filter for Vignette {
    fn apply(&self, cell: &mut Cell, x: u16, y: u16, width: u16, height: u16, t: f64) {
        // Handle zero dimensions gracefully
        if width == 0 || height == 0 {
            return;
        }

        let dim_factor = if self.sides.is_empty() {
            let signal_ctx = SignalContext::new(0, 0)
                .with_dimensions(width, height)
                .with_cell_position(x, y);
            // Normalize distance (0.0 at center, 1.0 at corner)
            let norm_dist = SpatialCoordinateSignal::sample_surface_radius()
                .sample_with_context(0.0, &signal_ctx);
            let norm_dist = (norm_dist + self.dither_offset(x, y, t)).clamp(0.0, 1.0);

            if norm_dist > self.radius {
                let beyond_radius = norm_dist - self.radius;
                let radius_range = 1.0 - self.radius;
                if radius_range > 0.0 {
                    self.strength * (beyond_radius / radius_range)
                } else {
                    self.strength
                }
            } else {
                0.0
            }
        } else {
            let max_x = width.saturating_sub(1).max(1) as f32;
            let max_y = height.saturating_sub(1).max(1) as f32;
            let edge_dist = self
                .sides
                .iter()
                .map(|side| match side {
                    VignetteEdge::Top => y as f32 / max_y,
                    VignetteEdge::Bottom => (height.saturating_sub(y + 1)) as f32 / max_y,
                    VignetteEdge::Left => x as f32 / max_x,
                    VignetteEdge::Right => (width.saturating_sub(x + 1)) as f32 / max_x,
                })
                .fold(f32::INFINITY, f32::min);
            let edge_dist = (edge_dist + self.dither_offset(x, y, t)).clamp(0.0, 1.0);
            if edge_dist < self.radius {
                self.strength * (1.0 - edge_dist / self.radius.max(f32::EPSILON))
            } else {
                0.0
            }
        };

        if dim_factor > 0.0 {
            cell.fg = Self::dim_color(cell.fg, dim_factor);
            cell.bg = Self::dim_color(cell.bg, dim_factor);
        }
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

    #[test]
    fn left_side_only_does_not_dim_right_side() {
        let vignette = Vignette::new(1.0, 0.3).with_sides(vec![VignetteEdge::Left]);
        let mut left = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        let mut right = left;
        vignette.apply(&mut left, 0, 5, 10, 10, 0.0);
        vignette.apply(&mut right, 9, 5, 10, 10, 0.0);
        assert_ne!(left.fg, Color::rgb(100, 100, 100));
        assert_eq!(right.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn two_side_mode_hits_top_and_left() {
        let vignette =
            Vignette::new(1.0, 0.3).with_sides(vec![VignetteEdge::Top, VignetteEdge::Left]);
        let mut top_left = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        let mut bottom_right = top_left;
        vignette.apply(&mut top_left, 0, 0, 10, 10, 0.0);
        vignette.apply(&mut bottom_right, 9, 9, 10, 10, 0.0);
        assert_ne!(top_left.fg, Color::rgb(100, 100, 100));
        assert_eq!(bottom_right.fg, Color::rgb(100, 100, 100));
    }

    #[test]
    fn static_dither_changes_adjacent_cells() {
        let vignette = Vignette::new(0.7, 0.4).with_dither(0.08, 0.0);
        let mut a = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        let mut b = a;
        vignette.apply(&mut a, 0, 1, 20, 10, 0.0);
        vignette.apply(&mut b, 1, 1, 20, 10, 0.0);
        assert_ne!(a.fg, b.fg);
    }

    #[test]
    fn temporal_dither_changes_over_time() {
        let vignette = Vignette::new(0.7, 0.4).with_dither(0.08, 2.0);
        let mut a = Cell::styled(
            'x',
            Color::rgb(100, 100, 100),
            Color::rgb(100, 100, 100),
            Modifiers::NONE,
        );
        let mut b = a;
        vignette.apply(&mut a, 0, 1, 20, 10, 0.0);
        vignette.apply(&mut b, 0, 1, 20, 10, 1.0);
        assert_ne!(a.fg, b.fg);
    }
}

// <FILE>tui-vfx-compositor/src/filters/cls_vignette.rs</FILE> - <DESC>Vignette filter with proper spatial radial gradient</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
