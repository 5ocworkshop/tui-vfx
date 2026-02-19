// <FILE>tui-vfx-geometry/src/paths/cls_spiral_path.rs</FILE> - <DESC>Spiral path implementation</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-18T22:15:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Adding radius_cells parameter</WCTX>
// <CLOG>Added radius_cells parameter for explicit radius control</CLOG>

use crate::traits::MotionPath;
use crate::types::Position;
use serde::{Deserialize, Serialize};

/// A path that spirals from start to end.
///
/// The path follows a linear trajectory from start to end while also
/// rotating around that trajectory. The rotation radius decays from
/// its initial value to 0 as t approaches 1.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct SpiralPath {
    /// Number of full rotations during the spiral
    pub rotations: f32,
    /// Optional fixed radius in cells. If None, uses half the distance between points.
    pub radius_cells: Option<u16>,
}

impl Default for SpiralPath {
    fn default() -> Self {
        Self {
            rotations: 1.0,
            radius_cells: None,
        }
    }
}

impl MotionPath for SpiralPath {
    fn calculate(&self, t: f64, start: Position, end: Position) -> (f32, f32) {
        let t = t.clamp(0.0, 1.0) as f32;
        let sx = start.x as f32;
        let sy = start.y as f32;
        let ex = end.x as f32;
        let ey = end.y as f32;

        // Base linear position (the center of the spiral moves linearly)
        let lx = sx + (ex - sx) * t;
        let ly = sy + (ey - sy) * t;

        // Calculate the spiral offset
        let angle = t * self.rotations * std::f32::consts::TAU;

        // Radius: either from radius_cells or derived from distance
        let max_radius = match self.radius_cells {
            Some(r) => r as f32,
            None => {
                let dist = ((ex - sx).powi(2) + (ey - sy).powi(2)).sqrt();
                dist * 0.5
            }
        };

        // Radius decays to 0 as t -> 1 (spiral IN to target)
        let radius = max_radius * (1.0 - t);

        let rx = radius * angle.cos();
        let ry = radius * angle.sin();

        (lx + rx, ly + ry)
    }
}

// <FILE>tui-vfx-geometry/src/paths/cls_spiral_path.rs</FILE> - <DESC>Spiral path implementation</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-18T22:15:00Z</VERS>
