// <FILE>tui-vfx-compositor/src/masks/cls_materialize.rs</FILE>
// <DESC>Organic materialization mask with radial bias and deterministic noise</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Slice 6.6 §F.3 — migrate Mask trait to &VfxCellContext</WCTX>
// <CLOG>1.2.0: MINOR — is_visible signature updated to &VfxCellContext; local_x/local_y/width/height/t replace positional params.</CLOG>

use crate::masks::cls_radial::RadialOrigin;
use crate::traits::mask::Mask;
use mixed_signals::prelude::{Signal, SignalContext, SurfaceDistanceSignal};
use std::hash::{Hash, Hasher};
use tui_vfx_types::VfxCellContext;

/// Organic reveal mask that materializes content from an origin with noisy breakup.
///
/// `Materialize` is designed as a richer pipeline-level alternative to a simple
/// reverse-dissolve. It blends a radial/directional reveal order with deterministic
/// per-cell noise so content can feel like it gathers, condenses, or stabilizes into
/// place rather than merely appearing.
pub struct Materialize {
    /// Origin point that biases where the reveal starts.
    pub origin: RadialOrigin,
    /// Seed for deterministic noise.
    pub seed: u64,
    /// Chunk size for grouped/cellular reveal behavior.
    pub chunk_size: u8,
    /// Noise amplitude in normalized threshold space (0.0-1.0).
    pub noise: f32,
    /// Whether to blend the edge softly rather than hard-clipping.
    pub soft_edge: bool,
}

impl Default for Materialize {
    fn default() -> Self {
        Self::new(RadialOrigin::Center, 0, 1, 0.18, true)
    }
}

impl Materialize {
    pub fn new(
        origin: RadialOrigin,
        seed: u64,
        chunk_size: u8,
        noise: f32,
        soft_edge: bool,
    ) -> Self {
        Self {
            origin,
            seed,
            chunk_size: chunk_size.max(1),
            noise: noise.clamp(0.0, 1.0),
            soft_edge,
        }
    }

    fn chunk_noise(&self, x: u16, y: u16) -> f32 {
        let chunk_x = x / self.chunk_size as u16;
        let chunk_y = y / self.chunk_size as u16;
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        chunk_x.hash(&mut hasher);
        chunk_y.hash(&mut hasher);
        self.seed.hash(&mut hasher);
        let hash = hasher.finish();
        (hash as f64 / u64::MAX as f64) as f32
    }

    fn normalized_distance(&self, x: u16, y: u16, w: u16, h: u16) -> f32 {
        let (origin_x, origin_y) = self.origin.as_fraction();
        let signal_ctx = SignalContext::new(0, 0)
            .with_dimensions(w, h)
            .with_cell_position(x, y);
        SurfaceDistanceSignal::radius_from(origin_x, origin_y).sample_with_context(0.0, &signal_ctx)
    }

    pub(crate) fn threshold(&self, x: u16, y: u16, w: u16, h: u16) -> f32 {
        let chunk = self.chunk_size as u16;
        let chunk_x = (x / chunk) * chunk + chunk / 2;
        let chunk_y = (y / chunk) * chunk + chunk / 2;
        let base = self.normalized_distance(chunk_x, chunk_y, w, h);
        let noise = (self.chunk_noise(x, y) - 0.5) * 2.0 * self.noise;
        (base + noise).clamp(0.0, 1.0)
    }
}

impl Mask for Materialize {
    fn is_visible(&self, ctx: &VfxCellContext) -> bool {
        let progress = ctx.t as f32;
        if progress <= 0.0 {
            return false;
        }
        if progress >= 1.0 {
            return true;
        }

        let threshold = self.threshold(ctx.local_x, ctx.local_y, ctx.width, ctx.height);
        if self.soft_edge {
            let edge = 0.08 + self.noise * 0.12;
            if threshold <= progress - edge {
                true
            } else if threshold >= progress + edge {
                false
            } else {
                let blend = ((progress + edge) - threshold) / (edge * 2.0);
                blend > 0.5
            }
        } else {
            threshold <= progress
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx_at(x: u16, y: u16, w: u16, h: u16, t: f64) -> VfxCellContext {
        VfxCellContext::new(x, y, w, h, 0, 0, t)
    }

    #[test]
    fn zero_progress_hides_everything() {
        let mask = Materialize::default();
        assert!(!mask.is_visible(&ctx_at(5, 5, 10, 10, 0.0)));
    }

    #[test]
    fn full_progress_shows_everything() {
        let mask = Materialize::default();
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 1.0)));
        assert!(mask.is_visible(&ctx_at(9, 9, 10, 10, 1.0)));
    }

    #[test]
    fn center_origin_biases_middle_first() {
        let mask = Materialize::new(RadialOrigin::Center, 3, 1, 0.0, false);
        assert!(mask.is_visible(&ctx_at(5, 5, 10, 10, 0.15)));
        assert!(!mask.is_visible(&ctx_at(0, 0, 10, 10, 0.15)));
    }

    #[test]
    fn corner_origin_biases_corner_first() {
        let mask = Materialize::new(RadialOrigin::TopLeft, 3, 1, 0.0, false);
        assert!(mask.is_visible(&ctx_at(0, 0, 10, 10, 0.15)));
        assert!(!mask.is_visible(&ctx_at(9, 9, 10, 10, 0.15)));
    }

    #[test]
    fn chunking_keeps_cells_together() {
        let mask = Materialize::new(RadialOrigin::Center, 42, 2, 0.3, false);
        let a = mask.threshold(2, 2, 12, 12);
        let b = mask.threshold(3, 3, 12, 12);
        assert!((a - b).abs() < f32::EPSILON);
    }
}

// <FILE>tui-vfx-compositor/src/masks/cls_materialize.rs</FILE>
// <DESC>Organic materialization mask with radial bias and deterministic noise</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
