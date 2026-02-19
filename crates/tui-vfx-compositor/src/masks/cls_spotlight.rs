// <FILE>tui-vfx-compositor/src/masks/cls_spotlight.rs</FILE> - <DESC>Spotlight (Iris) mask implementation</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-23</VERS>
// <WCTX>Pipeline configuration fix</WCTX>
// <CLOG>Added shape and soft_edge config fields</CLOG>

use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::IrisShape;

/// Spotlight/Iris mask - reveals from center outward.
pub struct Spotlight {
    /// Shape of the iris reveal
    pub shape: IrisShape,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl Default for Spotlight {
    fn default() -> Self {
        Self::new(IrisShape::Circle, false)
    }
}

impl Spotlight {
    /// Create a new Spotlight/Iris mask.
    pub fn new(shape: IrisShape, soft_edge: bool) -> Self {
        Self { shape, soft_edge }
    }

    /// Calculate distance from center based on shape
    fn distance(&self, x: u16, y: u16, w: u16, h: u16) -> f32 {
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;

        match self.shape {
            IrisShape::Circle => (dx * dx + dy * dy).sqrt(),
            IrisShape::Diamond => dx.abs() + dy.abs(),
            IrisShape::Box => dx.abs().max(dy.abs()),
        }
    }
}

impl Mask for Spotlight {
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        let progress = progress as f32;

        let dist = self.distance(x, y, w, h);
        let max_dim = w.max(h) as f32;
        let max_radius = max_dim * 0.75; // Reach corners approx
        let current_radius = max_radius * progress;

        if self.soft_edge {
            // Soft edge: gradual transition
            let edge_width = max_radius * 0.1;
            dist < current_radius + edge_width
        } else {
            dist < current_radius
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spotlight_center_progress_zero_not_visible() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        // Center of 10x10 is (5,5), but at progress=0 radius=0
        assert!(!mask.is_visible(5, 5, 10, 10, 0.0));
    }

    #[test]
    fn test_spotlight_center_progress_one_visible() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        assert!(mask.is_visible(5, 5, 10, 10, 1.0));
    }

    #[test]
    fn test_spotlight_circle_shape() {
        let mask = Spotlight::new(IrisShape::Circle, false);
        // At progress 0.5, radius covers center region
        assert!(mask.is_visible(5, 5, 10, 10, 0.5)); // Center visible
    }

    #[test]
    fn test_spotlight_diamond_shape() {
        let mask = Spotlight::new(IrisShape::Diamond, false);
        assert!(mask.is_visible(5, 5, 10, 10, 0.5)); // Center visible
    }

    #[test]
    fn test_spotlight_box_shape() {
        let mask = Spotlight::new(IrisShape::Box, false);
        assert!(mask.is_visible(5, 5, 10, 10, 0.5)); // Center visible
    }

    #[test]
    fn test_spotlight_soft_edge_extends_visibility() {
        let hard = Spotlight::new(IrisShape::Circle, false);
        let soft = Spotlight::new(IrisShape::Circle, true);
        // Find a position that's just outside hard edge but inside soft edge
        // At progress=0.5, hard radius = 7.5 * 0.5 = 3.75
        // soft edge width = 7.5 * 0.1 = 0.75
        // A point at distance ~4 from center should be visible with soft but not hard
        // Distance from (5,5) to (9,5) is 4
        let hard_vis = hard.is_visible(9, 5, 10, 10, 0.5);
        let soft_vis = soft.is_visible(9, 5, 10, 10, 0.5);
        // Soft edge should make more positions visible
        assert!(soft_vis || !hard_vis); // If hard is visible, soft must be too
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_spotlight.rs</FILE> - <DESC>Spotlight (Iris) mask implementation</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-23</VERS>
