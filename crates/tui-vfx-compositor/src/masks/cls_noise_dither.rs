// <FILE>tui-vfx-compositor/src/masks/cls_noise_dither.rs</FILE> - <DESC>NoiseDither mask implementation</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.2.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/t replace positional params (width/height not read by this impl).</CLOG>

use crate::traits::mask::Mask;
use crate::types::cls_mask_spec::DitherMatrix;
use std::hash::{Hash, Hasher};
use tui_vfx_types::VfxCellContext;

/// Dithered noise pattern mask for halftone-style reveal.
///
/// Uses Bayer ordered-dithering matrices to produce a spatially structured
/// reveal pattern. The seed offsets the dither pattern horizontally for
/// visual variety across multiple simultaneous effects.
pub struct NoiseDither {
    /// Seed for deterministic randomness
    pub seed: u64,
    /// Dither matrix size
    pub matrix: DitherMatrix,
}

impl Default for NoiseDither {
    fn default() -> Self {
        Self::new(0, DitherMatrix::Bayer4)
    }
}

impl NoiseDither {
    /// Create a new NoiseDither mask.
    pub fn new(seed: u64, matrix: DitherMatrix) -> Self {
        Self { seed, matrix }
    }

    /// Get the dither threshold based on position and matrix
    fn dither_threshold(&self, x: u16, y: u16) -> f32 {
        // Bayer dither matrices for ordered dithering
        const BAYER4: [[f32; 4]; 4] = [
            [0.0 / 16.0, 8.0 / 16.0, 2.0 / 16.0, 10.0 / 16.0],
            [12.0 / 16.0, 4.0 / 16.0, 14.0 / 16.0, 6.0 / 16.0],
            [3.0 / 16.0, 11.0 / 16.0, 1.0 / 16.0, 9.0 / 16.0],
            [15.0 / 16.0, 7.0 / 16.0, 13.0 / 16.0, 5.0 / 16.0],
        ];

        const BAYER8: [[f32; 8]; 8] = [
            [
                0.0 / 64.0,
                32.0 / 64.0,
                8.0 / 64.0,
                40.0 / 64.0,
                2.0 / 64.0,
                34.0 / 64.0,
                10.0 / 64.0,
                42.0 / 64.0,
            ],
            [
                48.0 / 64.0,
                16.0 / 64.0,
                56.0 / 64.0,
                24.0 / 64.0,
                50.0 / 64.0,
                18.0 / 64.0,
                58.0 / 64.0,
                26.0 / 64.0,
            ],
            [
                12.0 / 64.0,
                44.0 / 64.0,
                4.0 / 64.0,
                36.0 / 64.0,
                14.0 / 64.0,
                46.0 / 64.0,
                6.0 / 64.0,
                38.0 / 64.0,
            ],
            [
                60.0 / 64.0,
                28.0 / 64.0,
                52.0 / 64.0,
                20.0 / 64.0,
                62.0 / 64.0,
                30.0 / 64.0,
                54.0 / 64.0,
                22.0 / 64.0,
            ],
            [
                3.0 / 64.0,
                35.0 / 64.0,
                11.0 / 64.0,
                43.0 / 64.0,
                1.0 / 64.0,
                33.0 / 64.0,
                9.0 / 64.0,
                41.0 / 64.0,
            ],
            [
                51.0 / 64.0,
                19.0 / 64.0,
                59.0 / 64.0,
                27.0 / 64.0,
                49.0 / 64.0,
                17.0 / 64.0,
                57.0 / 64.0,
                25.0 / 64.0,
            ],
            [
                15.0 / 64.0,
                47.0 / 64.0,
                7.0 / 64.0,
                39.0 / 64.0,
                13.0 / 64.0,
                45.0 / 64.0,
                5.0 / 64.0,
                37.0 / 64.0,
            ],
            [
                63.0 / 64.0,
                31.0 / 64.0,
                55.0 / 64.0,
                23.0 / 64.0,
                61.0 / 64.0,
                29.0 / 64.0,
                53.0 / 64.0,
                21.0 / 64.0,
            ],
        ];

        // Add seed-based offset
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        self.seed.hash(&mut hasher);
        let seed_offset = (hasher.finish() % 100) as u16;

        match self.matrix {
            DitherMatrix::Bayer4 => {
                let mx = ((x + seed_offset) % 4) as usize;
                let my = (y % 4) as usize;
                BAYER4[my][mx]
            }
            DitherMatrix::Bayer8 => {
                let mx = ((x + seed_offset) % 8) as usize;
                let my = (y % 8) as usize;
                BAYER8[my][mx]
            }
        }
    }
}

impl Mask for NoiseDither {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let progress = ctx.t as f32;
        let threshold = self.dither_threshold(ctx.local_x, ctx.local_y);
        progress > threshold
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn test_noise_dither_progress_zero_not_visible() {
        let mask = NoiseDither::new(0, DitherMatrix::Bayer4);
        // All thresholds are >= 0, so progress 0 should be invisible everywhere
        assert!(!mask.is_visible(&ctx_at(1, 1, 10, 10, 0.0)));
    }

    #[test]
    fn test_noise_dither_progress_one_visible() {
        let mask = NoiseDither::new(0, DitherMatrix::Bayer4);
        // All thresholds are < 1.0, so progress 1.0 should be visible everywhere
        assert!(mask.is_visible(&ctx_at(1, 1, 10, 10, 1.0)));
    }

    #[test]
    fn test_noise_dither_bayer4_deterministic() {
        let mask = NoiseDither::new(0, DitherMatrix::Bayer4);
        // Same position with same seed should give consistent results
        let v1 = mask.is_visible(&ctx_at(3, 7, 10, 10, 0.5));
        let v2 = mask.is_visible(&ctx_at(3, 7, 10, 10, 0.5));
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_noise_dither_bayer8_deterministic() {
        let mask = NoiseDither::new(0, DitherMatrix::Bayer8);
        let v1 = mask.is_visible(&ctx_at(5, 3, 10, 10, 0.5));
        let v2 = mask.is_visible(&ctx_at(5, 3, 10, 10, 0.5));
        assert_eq!(v1, v2);
    }

    #[test]
    fn test_noise_dither_partial_progress() {
        let mask = NoiseDither::new(0, DitherMatrix::Bayer4);
        // At progress 0.5, roughly half the pixels should be visible
        let mut visible_count = 0;
        for x in 0..4 {
            for y in 0..4 {
                if mask.is_visible(&ctx_at(x, y, 10, 10, 0.5)) {
                    visible_count += 1;
                }
            }
        }
        // Should be approximately 8 out of 16 (half)
        assert!(visible_count > 4 && visible_count < 12);
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_noise_dither.rs</FILE> - <DESC>NoiseDither mask implementation</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
