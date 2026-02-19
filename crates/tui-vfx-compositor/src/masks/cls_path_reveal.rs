// <FILE>tui-vfx-compositor/src/masks/cls_path_reveal.rs</FILE> - <DESC>Path-based reveal mask</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-23</VERS>
// <WCTX>Gun barrel spiral reveal implementation</WCTX>
// <CLOG>Initial implementation with Spiral path support</CLOG>

use crate::traits::mask::Mask;
use std::f32::consts::{PI, TAU};

/// Direction for spiral reveals.
#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    Default,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum SpiralDirection {
    #[default]
    #[serde(alias = "Clockwise")]
    Clockwise,
    #[serde(alias = "CounterClockwise")]
    CounterClockwise,
}

/// Path type for reveal masks.
/// Similar to motion PathType but optimized for cell-by-cell reveals.
#[derive(
    Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum RevealPathType {
    /// Spiral from center outward
    #[serde(alias = "Spiral")]
    Spiral {
        /// Number of full rotations from center to edge
        #[serde(default = "default_rotations")]
        rotations: f32,
        /// Direction of spiral
        #[serde(default)]
        direction: SpiralDirection,
    },
    /// Radial sweep (like radar)
    #[serde(alias = "Radial")]
    Radial {
        /// Starting angle in degrees (0 = up/north)
        #[serde(default)]
        start_angle: f32,
        /// Direction of sweep
        #[serde(default)]
        direction: SpiralDirection,
    },
}

fn default_rotations() -> f32 {
    2.5
}

impl Default for RevealPathType {
    fn default() -> Self {
        Self::Spiral {
            rotations: 2.5,
            direction: SpiralDirection::Clockwise,
        }
    }
}

/// Path-based reveal mask.
/// Reveals cells based on their position along a path trajectory.
#[derive(Default)]
pub struct PathReveal {
    /// The path type defining the reveal pattern
    pub path: RevealPathType,
    /// Whether to apply soft edge blending
    pub soft_edge: bool,
}

impl PathReveal {
    /// Create a new PathReveal mask.
    pub fn new(path: RevealPathType, soft_edge: bool) -> Self {
        Self { path, soft_edge }
    }

    /// Calculate the reveal threshold for a cell.
    /// Returns a value 0.0-1.0 indicating when this cell should be revealed.
    fn reveal_threshold(&self, x: u16, y: u16, w: u16, h: u16) -> f32 {
        // Center of the widget
        let cx = w as f32 / 2.0;
        let cy = h as f32 / 2.0;

        // Cell position relative to center
        let dx = x as f32 - cx;
        let dy = y as f32 - cy;

        // Polar coordinates from center
        // Note: dy is negated so that "up" (negative y) is angle 0
        let r = (dx * dx + dy * dy).sqrt();
        let theta = (-dy).atan2(dx); // Angle from center, 0 = right, PI/2 = up

        // Maximum radius to cover the widget
        let max_r = ((cx * cx) + (cy * cy)).sqrt();

        match &self.path {
            RevealPathType::Spiral {
                rotations,
                direction,
            } => {
                // Archimedean spiral: r = a * θ
                // We want the spiral to reach max_r after `rotations` full turns
                // So: max_r = a * (rotations * TAU)
                // Therefore: a = max_r / (rotations * TAU)

                if max_r < 0.001 || *rotations < 0.001 {
                    return 0.0;
                }

                let pitch = max_r / rotations; // Radial distance per full rotation

                // Normalize angle to 0..TAU range
                // For clockwise starting "up": we want up (theta=PI/2) to be 0
                // and going clockwise (decreasing theta) to increase reveal time
                let normalized_angle = match direction {
                    SpiralDirection::Clockwise => {
                        // Clockwise from up: up=0, right=0.25, down=0.5, left=0.75
                        let angle = PI / 2.0 - theta; // Rotate so up is 0
                        if angle < 0.0 { angle + TAU } else { angle }
                    }
                    SpiralDirection::CounterClockwise => {
                        // Counter-clockwise from up
                        let angle = theta - PI / 2.0;
                        if angle < 0.0 { angle + TAU } else { angle }
                    }
                };

                // Calculate which "arm" of the spiral this cell is closest to
                // The spiral passes through angle θ at radii: r = pitch * (θ/TAU + k) for k = 0, 1, 2...
                // Find the k that gives the closest radius to our cell's radius

                let angle_fraction = normalized_angle / TAU; // 0..1 for one rotation

                // For each spiral arm k, the radius at our angle is: pitch * (angle_fraction + k)
                // We want to find k where this equals our r
                // k = (r / pitch) - angle_fraction
                let k_float = (r / pitch) - angle_fraction;
                let k = k_float.round().max(0.0) as u32;

                // The reveal time for this cell is when the spiral reaches it
                // t = (k + angle_fraction) / rotations
                let reveal_t = (k as f32 + angle_fraction) / rotations;

                reveal_t.clamp(0.0, 1.0)
            }
            RevealPathType::Radial {
                start_angle,
                direction,
            } => {
                // Radial sweep like a radar
                // All cells at the same angle get revealed together
                // start_angle: 0 = up/north, 90 = right/east, etc.
                let start_rad = (90.0 - start_angle).to_radians(); // Convert to math coords

                let normalized_angle = match direction {
                    SpiralDirection::Clockwise => {
                        // Clockwise: angle decreases
                        let angle = theta - start_rad;

                        if angle > 0.0 { TAU - angle } else { -angle }
                    }
                    SpiralDirection::CounterClockwise => {
                        // Counter-clockwise: angle increases
                        let angle = theta - start_rad;
                        if angle < 0.0 { angle + TAU } else { angle }
                    }
                };

                (normalized_angle / TAU).clamp(0.0, 1.0)
            }
        }
    }
}

impl Mask for PathReveal {
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        let progress = progress as f32;
        let threshold = self.reveal_threshold(x, y, w, h);

        if self.soft_edge {
            // Soft edge: cells near the current progress get partial visibility
            // For now, use a simple comparison with slight lookahead
            let edge_width = 0.05;
            threshold < progress + edge_width
        } else {
            threshold <= progress
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spiral_center_revealed_first() {
        let mask = PathReveal::new(
            RevealPathType::Spiral {
                rotations: 2.0,
                direction: SpiralDirection::Clockwise,
            },
            false,
        );

        // Center cell should have lowest threshold (revealed first)
        let center_threshold = mask.reveal_threshold(5, 5, 10, 10);
        let edge_threshold = mask.reveal_threshold(0, 0, 10, 10);

        assert!(
            center_threshold < edge_threshold,
            "Center ({}) should be revealed before edge ({})",
            center_threshold,
            edge_threshold
        );
    }

    #[test]
    fn test_spiral_clockwise_order() {
        let mask = PathReveal::new(
            RevealPathType::Spiral {
                rotations: 1.0,
                direction: SpiralDirection::Clockwise,
            },
            false,
        );

        // For a 10x10 grid, check that "up" from center is revealed before "right"
        // Center is (5, 5), "up" is (5, 4), "right" is (6, 5)
        let up_threshold = mask.reveal_threshold(5, 4, 10, 10);
        let right_threshold = mask.reveal_threshold(6, 5, 10, 10);
        let down_threshold = mask.reveal_threshold(5, 6, 10, 10);
        let left_threshold = mask.reveal_threshold(4, 5, 10, 10);

        // Clockwise from up: up < right < down < left
        assert!(
            up_threshold < right_threshold,
            "Up ({}) should be before right ({})",
            up_threshold,
            right_threshold
        );
        assert!(
            right_threshold < down_threshold,
            "Right ({}) should be before down ({})",
            right_threshold,
            down_threshold
        );
        assert!(
            down_threshold < left_threshold,
            "Down ({}) should be before left ({})",
            down_threshold,
            left_threshold
        );
    }

    #[test]
    fn test_radial_sweep() {
        let mask = PathReveal::new(
            RevealPathType::Radial {
                start_angle: 0.0, // Start from up
                direction: SpiralDirection::Clockwise,
            },
            false,
        );

        // Check that cells at the starting angle are revealed first
        let up_threshold = mask.reveal_threshold(5, 0, 10, 10);
        let right_threshold = mask.reveal_threshold(9, 5, 10, 10);

        assert!(
            up_threshold < right_threshold,
            "Up ({}) should be before right ({})",
            up_threshold,
            right_threshold
        );
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_path_reveal.rs</FILE> - <DESC>Path-based reveal mask</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-23</VERS>
