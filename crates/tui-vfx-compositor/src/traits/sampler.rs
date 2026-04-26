// <FILE>tui-vfx-compositor/src/traits/sampler.rs</FILE>
// <DESC>Trait for coordinate remapping (Distortion)</DESC>
// <VERS>VERSION: 3.0.0</VERS>
// <WCTX>Slice 6.6 §F.4 — migrate Sampler trait to take &VfxCellContext</WCTX>
// <CLOG>3.0.0: BREAKING — sample(dest_x, dest_y, width, height, t) → sample(&VfxCellContext). ctx.local_x/local_y carry the destination coords; ctx.width/height carry area dims; ctx.t carries the animation clock.</CLOG>

use tui_vfx_types::VfxCellContext;

pub trait Sampler {
    /// Remaps destination coordinates to source coordinates for sampling/distortion effects.
    ///
    /// # Parameters
    /// - `ctx.local_x`, `ctx.local_y`: Destination coordinates to sample/remap.
    /// - `ctx.width`, `ctx.height`: Render area dimensions for spatial awareness.
    /// - `ctx.t`: Animation time (0.0–1.0 for phase-based, seconds for continuous).
    ///
    /// # Returns
    /// - `Some((src_x, src_y))`: Remapped source coordinates to sample from.
    /// - `None`: Pixel should be transparent/skipped.
    ///
    /// # Spatial Context
    /// The bundle enables area-aware effects:
    /// - Dynamic centering (e.g., ripple at `(ctx.width/2, ctx.height/2)`)
    /// - Normalized coordinates for position-independent distortion
    /// - Boundary-aware displacement calculations
    ///
    /// Impls that ignore some fields simply do not read them from `ctx`.
    fn sample(&self, ctx: &VfxCellContext) -> Option<(u16, u16)>;
}
