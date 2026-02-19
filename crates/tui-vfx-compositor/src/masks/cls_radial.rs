// <FILE>tui-vfx-compositor/src/masks/cls_radial.rs</FILE>
// <DESC>Radial mask revealing from configurable origin</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Effect parity: Mask enhancements</WCTX>
// <CLOG>Initial implementation of Radial mask</CLOG>

use crate::traits::mask::Mask;
use serde::{Deserialize, Serialize};

/// Origin point for radial reveal.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RadialOrigin {
    /// Expand from center (default)
    #[default]
    Center,
    /// Expand from top-left corner
    TopLeft,
    /// Expand from top-right corner
    TopRight,
    /// Expand from bottom-left corner
    BottomLeft,
    /// Expand from bottom-right corner
    BottomRight,
    /// Custom origin as fraction (0.0-1.0 for x and y)
    Custom { x: f32, y: f32 },
}

impl RadialOrigin {
    /// Get the origin point as (x, y) fractions in 0.0-1.0 range.
    pub fn as_fraction(&self) -> (f32, f32) {
        match self {
            RadialOrigin::Center => (0.5, 0.5),
            RadialOrigin::TopLeft => (0.0, 0.0),
            RadialOrigin::TopRight => (1.0, 0.0),
            RadialOrigin::BottomLeft => (0.0, 1.0),
            RadialOrigin::BottomRight => (1.0, 1.0),
            RadialOrigin::Custom { x, y } => (*x, *y),
        }
    }
}

/// Radial mask that reveals in a circular pattern from a configurable origin.
///
/// The reveal expands outward from the origin point, creating a circular
/// or elliptical pattern depending on the widget's aspect ratio.
pub struct Radial {
    /// Origin point for the radial expansion
    pub origin: RadialOrigin,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl Default for Radial {
    fn default() -> Self {
        Self::new(RadialOrigin::Center, false)
    }
}

impl Radial {
    /// Create a new Radial mask.
    ///
    /// # Arguments
    /// * `origin` - The point from which the reveal expands
    /// * `soft_edge` - Whether to apply soft edge blending
    pub fn new(origin: RadialOrigin, soft_edge: bool) -> Self {
        Self { origin, soft_edge }
    }

    /// Create a radial mask expanding from center.
    #[allow(dead_code)]
    pub fn from_center() -> Self {
        Self::new(RadialOrigin::Center, false)
    }

    /// Create a radial mask expanding from a corner.
    #[allow(dead_code)]
    pub fn from_corner(origin: RadialOrigin) -> Self {
        Self::new(origin, false)
    }
}

impl Mask for Radial {
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        let progress = progress as f32;

        if progress <= 0.0 {
            return false;
        }
        if progress >= 1.0 {
            return true;
        }

        let (origin_x, origin_y) = self.origin.as_fraction();

        // Calculate the origin point in pixel coordinates
        let ox = origin_x * w as f32;
        let oy = origin_y * h as f32;

        // Calculate distance from origin (normalized by max possible distance)
        let dx = x as f32 - ox;
        let dy = y as f32 - oy;
        let distance = (dx * dx + dy * dy).sqrt();

        // Calculate max distance from origin to any corner
        let max_distance = calculate_max_distance(ox, oy, w as f32, h as f32);

        // Normalize distance
        let normalized_dist = if max_distance > 0.0 {
            distance / max_distance
        } else {
            0.0
        };

        if self.soft_edge {
            // Soft edge: use smooth transition
            let edge_width = 0.1;
            let threshold = progress;
            if normalized_dist < threshold - edge_width {
                true
            } else if normalized_dist > threshold {
                false
            } else {
                // Smooth transition
                let t = (threshold - normalized_dist) / edge_width;
                t > 0.5
            }
        } else {
            // Hard edge
            normalized_dist < progress
        }
    }
}

/// Calculate the maximum distance from origin to any corner of the rectangle.
fn calculate_max_distance(ox: f32, oy: f32, w: f32, h: f32) -> f32 {
    let corners = [(0.0, 0.0), (w, 0.0), (0.0, h), (w, h)];

    corners
        .iter()
        .map(|(cx, cy)| {
            let dx = cx - ox;
            let dy = cy - oy;
            (dx * dx + dy * dy).sqrt()
        })
        .fold(0.0_f32, f32::max)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_radial_origin_fractions() {
        assert_eq!(RadialOrigin::Center.as_fraction(), (0.5, 0.5));
        assert_eq!(RadialOrigin::TopLeft.as_fraction(), (0.0, 0.0));
        assert_eq!(RadialOrigin::TopRight.as_fraction(), (1.0, 0.0));
        assert_eq!(RadialOrigin::BottomLeft.as_fraction(), (0.0, 1.0));
        assert_eq!(RadialOrigin::BottomRight.as_fraction(), (1.0, 1.0));
        assert_eq!(
            RadialOrigin::Custom { x: 0.25, y: 0.75 }.as_fraction(),
            (0.25, 0.75)
        );
    }

    #[test]
    fn test_center_at_zero_progress() {
        let mask = Radial::from_center();
        // At 0% progress, nothing visible
        assert!(!mask.is_visible(5, 5, 10, 10, 0.0));
    }

    #[test]
    fn test_center_at_full_progress() {
        let mask = Radial::from_center();
        // At 100% progress, everything visible
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(9, 9, 10, 10, 1.0));
    }

    #[test]
    fn test_center_reveals_from_middle() {
        let mask = Radial::from_center();
        // At partial progress, center should be visible before corners
        // For a 10x10 grid, center is (5, 5)
        // At low progress, center visible but corners not
        assert!(mask.is_visible(5, 5, 10, 10, 0.1));
        assert!(!mask.is_visible(0, 0, 10, 10, 0.1));
    }

    #[test]
    fn test_corner_origin() {
        let mask = Radial::from_corner(RadialOrigin::TopLeft);
        // At partial progress, top-left should be visible first
        assert!(mask.is_visible(0, 0, 10, 10, 0.1));
        // Far corner should not be visible at low progress
        assert!(!mask.is_visible(9, 9, 10, 10, 0.1));
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_radial.rs</FILE>
// <DESC>Radial mask revealing from configurable origin</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
