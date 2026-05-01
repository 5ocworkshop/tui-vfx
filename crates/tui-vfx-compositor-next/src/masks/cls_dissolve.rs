// <FILE>tui-vfx-compositor-next/src/masks/cls_dissolve.rs</FILE>
// <DESC>Dissolve mask using deterministic noise</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.2.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/t replace positional params (width/height not read by this impl).</CLOG>

use crate::traits::mask::Mask;
use std::hash::{Hash, Hasher};
use tui_vfx_types::VfxCellContext;

/// Dissolve mask that reveals/hides pixels based on deterministic noise.
///
/// The seed ensures the same pattern is produced each time, and chunk_size
/// allows grouping pixels together for a chunkier dissolve effect.
pub struct Dissolve {
    /// Seed for deterministic randomness
    pub seed: u64,
    /// Size of dissolve chunks (1 = single pixels, 2+ = grouped)
    pub chunk_size: u8,
}

impl Default for Dissolve {
    fn default() -> Self {
        Self::new(0, 1)
    }
}

impl Dissolve {
    /// Create a new Dissolve mask.
    ///
    /// # Arguments
    /// * `seed` - Seed for deterministic randomness
    /// * `chunk_size` - Size of pixel groups (1 = individual pixels)
    pub fn new(seed: u64, chunk_size: u8) -> Self {
        Self {
            seed,
            chunk_size: chunk_size.max(1), // Ensure at least 1
        }
    }
}

impl Mask for Dissolve {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        if ctx.t <= 0.0 {
            return false;
        }
        if ctx.t >= 1.0 {
            return true;
        }

        // Group pixels by chunk_size for chunkier dissolve
        let chunk_x = ctx.local_x / self.chunk_size as u16;
        let chunk_y = ctx.local_y / self.chunk_size as u16;

        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        chunk_x.hash(&mut hasher);
        chunk_y.hash(&mut hasher);
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();

        // Normalize to [0, 1]
        let val = (hash as f64) / (u64::MAX as f64);

        val < ctx.t
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_dissolve_progress_zero_not_visible() {
        let mask = Dissolve::new(42, 1);
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.0)));
    }

    #[test]
    fn test_dissolve_progress_one_visible() {
        let mask = Dissolve::new(42, 1);
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
    }

    #[test]
    fn test_dissolve_deterministic() {
        let mask = Dissolve::new(42, 1);
        let v1 = mask.is_visible(&ctx_at(3, 7, 10, 10, 0.5));
        let v2 = mask.is_visible(&ctx_at(3, 7, 10, 10, 0.5));
        assert_eq!(v1, v2); // Same seed + position = same result
    }

    #[test]
    fn test_dissolve_chunk_grouping() {
        let mask = Dissolve::new(42, 2);
        // Pixels (0,0) and (1,1) both map to chunk (0,0)
        let a = mask.is_visible(&ctx_at(0, 0, 10, 10, 0.5));
        let b = mask.is_visible(&ctx_at(1, 1, 10, 10, 0.5));
        assert_eq!(a, b);
    }

    #[test]
    fn test_dissolve_different_seeds_different_patterns() {
        let mask1 = Dissolve::new(1, 1);
        let mask2 = Dissolve::new(2, 1);
        // Test multiple positions - at least one should differ
        let mut differ = false;
        for x in 0..5 {
            for y in 0..5 {
                if mask1.is_visible(&ctx_at(x, y, 10, 10, 0.5))
                    != mask2.is_visible(&ctx_at(x, y, 10, 10, 0.5))
                {
                    differ = true;
                    break;
                }
            }
        }
        assert!(differ, "Different seeds should produce different patterns");
    }
}

// <FILE>tui-vfx-compositor-next/src/masks/cls_dissolve.rs</FILE>
// <DESC>Dissolve mask using deterministic noise</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
