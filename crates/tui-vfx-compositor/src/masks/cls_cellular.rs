// <FILE>tui-vfx-compositor/src/masks/cls_cellular.rs</FILE>
// <DESC>Cellular/organic pattern mask</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>RNG performance optimization</WCTX>
// <CLOG>Switched to fast_random for ~25x faster per-cell noise generation</CLOG>

use crate::traits::mask::Mask;
use serde::{Deserialize, Serialize};

/// Pattern type for cellular masks.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum CellularPattern {
    /// Voronoi-like organic cells
    #[default]
    Voronoi,
    /// Hexagonal grid pattern
    Hexagonal,
    /// Organic blob pattern
    Organic,
}

/// Cellular mask that reveals in organic, cell-like patterns.
///
/// Creates natural-looking reveal patterns inspired by cellular structures,
/// voronoi diagrams, or organic growth patterns.
pub struct Cellular {
    /// The cellular pattern type
    pub pattern: CellularPattern,
    /// Seed for deterministic randomness
    pub seed: u64,
    /// Number of cells (affects granularity)
    pub cell_count: u16,
}

impl Default for Cellular {
    fn default() -> Self {
        Self::new(CellularPattern::Voronoi, 0, 16)
    }
}

impl Cellular {
    /// Create a new Cellular mask.
    ///
    /// # Arguments
    /// * `pattern` - The cellular pattern type
    /// * `seed` - Seed for deterministic randomness
    /// * `cell_count` - Number of cells (affects granularity)
    pub fn new(pattern: CellularPattern, seed: u64, cell_count: u16) -> Self {
        Self {
            pattern,
            seed,
            cell_count: cell_count.max(1),
        }
    }

    /// Create a Voronoi-pattern cellular mask.
    #[allow(dead_code)]
    pub fn voronoi(seed: u64, cell_count: u16) -> Self {
        Self::new(CellularPattern::Voronoi, seed, cell_count)
    }

    /// Create a hexagonal-pattern cellular mask.
    #[allow(dead_code)]
    pub fn hexagonal(seed: u64, cell_count: u16) -> Self {
        Self::new(CellularPattern::Hexagonal, seed, cell_count)
    }

    /// Create an organic-pattern cellular mask.
    #[allow(dead_code)]
    pub fn organic(seed: u64, cell_count: u16) -> Self {
        Self::new(CellularPattern::Organic, seed, cell_count)
    }

    /// Generate cell centers based on seed and dimensions.
    fn generate_cell_centers(&self, w: u16, h: u16) -> Vec<(f32, f32, f32)> {
        let mut centers = Vec::with_capacity(self.cell_count as usize);

        for i in 0..self.cell_count {
            // Deterministic pseudo-random position for each cell
            let x_hash = hash_value(self.seed, i as u64, 0);
            let y_hash = hash_value(self.seed, i as u64, 1);
            let order_hash = hash_value(self.seed, i as u64, 2);

            let x = (x_hash as f32 / u64::MAX as f32) * w as f32;
            let y = (y_hash as f32 / u64::MAX as f32) * h as f32;
            let order = order_hash as f32 / u64::MAX as f32;

            centers.push((x, y, order));
        }

        centers
    }
}

impl Mask for Cellular {
    fn is_visible(&self, x: u16, y: u16, w: u16, h: u16, progress: f64) -> bool {
        let progress = progress as f32;

        if progress <= 0.0 {
            return false;
        }
        if progress >= 1.0 {
            return true;
        }

        match self.pattern {
            CellularPattern::Voronoi => {
                // Find the closest cell center and use its reveal order
                let centers = self.generate_cell_centers(w, h);

                let mut min_dist = f32::MAX;
                let mut cell_order = 0.0_f32;

                for (cx, cy, order) in &centers {
                    let dx = x as f32 - cx;
                    let dy = y as f32 - cy;
                    let dist = dx * dx + dy * dy; // Squared distance is fine for comparison

                    if dist < min_dist {
                        min_dist = dist;
                        cell_order = *order;
                    }
                }

                // Pixel is visible if its cell's order is less than progress
                cell_order < progress
            }
            CellularPattern::Hexagonal => {
                // Hexagonal grid pattern
                let cell_size = ((w.max(h) as f32) / (self.cell_count as f32).sqrt()).max(1.0);

                // Calculate hex grid coordinates
                let hex_x = (x as f32 / cell_size) as i32;
                let hex_y = (y as f32 / (cell_size * 0.866)) as i32; // 0.866 ≈ sqrt(3)/2

                // Offset every other row
                let effective_x = if hex_y % 2 == 0 { hex_x } else { hex_x + 1 };

                // Use hash of hex coordinates for reveal order
                let order_hash = hash_value(self.seed, (effective_x as u64) << 16, hex_y as u64);
                let order = order_hash as f32 / u64::MAX as f32;

                order < progress
            }
            CellularPattern::Organic => {
                // Organic blob pattern using simplex-like noise approximation
                let scale = 1.0 / ((self.cell_count as f32).sqrt() * 2.0);
                let nx = x as f32 * scale;
                let ny = y as f32 * scale;

                // Simple pseudo-noise based on position and seed
                let noise = pseudo_noise(nx, ny, self.seed);

                // Map noise to reveal order (adjusted so it's roughly uniform)
                let order = (noise + 1.0) / 2.0; // Map -1..1 to 0..1

                order < progress
            }
        }
    }
}

/// Deterministic hash function for generating cell properties using fast_random.
/// ~25x faster than ChaCha8-based Rng.
fn hash_value(seed: u64, a: u64, b: u64) -> u64 {
    use mixed_signals::math::fast_random;
    // Combine seed and a into seed, use b as input
    let combined_seed = seed.wrapping_mul(31).wrapping_add(a);
    // Generate a u64-range value by combining two fast_random samples
    let high = (fast_random(combined_seed, b) * u32::MAX as f32) as u64;
    let low = (fast_random(combined_seed, b.wrapping_add(1)) * u32::MAX as f32) as u64;
    (high << 32) | low
}

/// Simple pseudo-noise function for organic patterns.
/// Uses grid-based interpolation for spatially coherent noise.
fn pseudo_noise(x: f32, y: f32, seed: u64) -> f32 {
    // Grid-based noise approximation
    let ix = x.floor() as i32;
    let iy = y.floor() as i32;
    let fx = x - x.floor();
    let fy = y - y.floor();

    // Get corner values
    let n00 = corner_noise(ix, iy, seed);
    let n10 = corner_noise(ix + 1, iy, seed);
    let n01 = corner_noise(ix, iy + 1, seed);
    let n11 = corner_noise(ix + 1, iy + 1, seed);

    // Smooth interpolation (smoothstep)
    let sx = fx * fx * (3.0 - 2.0 * fx);
    let sy = fy * fy * (3.0 - 2.0 * fy);

    let nx0 = n00 + sx * (n10 - n00);
    let nx1 = n01 + sx * (n11 - n01);

    nx0 + sy * (nx1 - nx0)
}

/// Get noise value for a grid corner using fast_random.
/// ~25x faster than ChaCha8-based Rng.
fn corner_noise(x: i32, y: i32, seed: u64) -> f32 {
    use mixed_signals::math::fast_random;
    // Combine x and seed, use y as input
    let combined_seed = seed.wrapping_mul(31).wrapping_add(x as u64);
    // fast_random returns 0.0..1.0, scale to [-1, 1]
    fast_random(combined_seed, y as u64) * 2.0 - 1.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cellular_at_zero_progress() {
        let mask = Cellular::default();
        assert!(!mask.is_visible(5, 5, 10, 10, 0.0));
    }

    #[test]
    fn test_cellular_at_full_progress() {
        let mask = Cellular::default();
        assert!(mask.is_visible(0, 0, 10, 10, 1.0));
        assert!(mask.is_visible(9, 9, 10, 10, 1.0));
    }

    #[test]
    fn test_cellular_deterministic() {
        let mask = Cellular::voronoi(42, 8);
        let result1 = mask.is_visible(5, 5, 20, 20, 0.5);
        let result2 = mask.is_visible(5, 5, 20, 20, 0.5);
        assert_eq!(result1, result2);
    }

    #[test]
    fn test_different_patterns() {
        let voronoi = Cellular::voronoi(0, 16);
        let hex = Cellular::hexagonal(0, 16);
        let organic = Cellular::organic(0, 16);

        // All should work without panicking
        let _ = voronoi.is_visible(5, 5, 20, 20, 0.5);
        let _ = hex.is_visible(5, 5, 20, 20, 0.5);
        let _ = organic.is_visible(5, 5, 20, 20, 0.5);
    }

    #[test]
    fn test_partial_reveal() {
        let mask = Cellular::voronoi(123, 4);
        // At 50% progress, some cells should be visible, some not
        let mut visible_count = 0;
        let mut hidden_count = 0;

        for y in 0..10 {
            for x in 0..10 {
                if mask.is_visible(x, y, 10, 10, 0.5) {
                    visible_count += 1;
                } else {
                    hidden_count += 1;
                }
            }
        }

        // Should have a mix of visible and hidden
        assert!(visible_count > 0);
        assert!(hidden_count > 0);
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_cellular.rs</FILE>
// <DESC>Cellular/organic pattern mask</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
