// <FILE>tui-vfx-compositor/src/masks/cls_blinds.rs</FILE>
// <DESC>Blinds mask</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-23</VERS>
// <WCTX>Pipeline configuration fix</WCTX>
// <CLOG>Added orientation and count config fields</CLOG>

use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::Orientation;

/// Venetian blinds effect mask.
pub struct Blinds {
    /// Whether blinds are horizontal or vertical
    pub orientation: Orientation,
    /// Number of blinds
    pub count: u16,
}

impl Default for Blinds {
    fn default() -> Self {
        Self::new(Orientation::Horizontal, 10)
    }
}

impl Blinds {
    /// Create a new Blinds mask.
    pub fn new(orientation: Orientation, count: u16) -> Self {
        Self {
            orientation,
            count: count.max(1),
        }
    }
}

impl Mask for Blinds {
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        let progress = progress as f32;

        let (position, size) = match self.orientation {
            Orientation::Horizontal => (y as f32, h as f32),
            Orientation::Vertical => (x as f32, w as f32),
        };

        let blind_size = (size / self.count as f32).max(1.0);
        let blind_index = (position / blind_size).floor();
        let pos_in_blind = position - (blind_index * blind_size);

        let threshold = blind_size * progress;
        pos_in_blind < threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blinds_progress_zero_not_visible() {
        let mask = Blinds::new(Orientation::Horizontal, 2);
        assert!(!mask.is_visible(0, 0, 10, 10, 0.0));
    }

    #[test]
    fn test_blinds_progress_one_visible() {
        let mask = Blinds::new(Orientation::Horizontal, 2);
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
    }

    #[test]
    fn test_blinds_vertical_orientation() {
        let mask = Blinds::new(Orientation::Vertical, 2);
        // At progress 0.5, half of each blind should be visible
        assert!(mask.is_visible(0, 0, 10, 10, 0.5)); // x=0 is at start of blind
        assert!(!mask.is_visible(4, 0, 10, 10, 0.5)); // x=4 is past midpoint
    }

    #[test]
    fn test_blinds_horizontal_partial() {
        let mask = Blinds::new(Orientation::Horizontal, 2);
        // With 2 blinds in height 10, each blind is 5 rows
        // At progress 0.5, threshold = 2.5
        assert!(mask.is_visible(0, 0, 10, 10, 0.5)); // y=0, pos_in_blind=0 < 2.5
        assert!(mask.is_visible(0, 2, 10, 10, 0.5)); // y=2, pos_in_blind=2 < 2.5
        assert!(!mask.is_visible(0, 3, 10, 10, 0.5)); // y=3, pos_in_blind=3 >= 2.5
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_blinds.rs</FILE>
// <DESC>Blinds mask</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-23</VERS>
