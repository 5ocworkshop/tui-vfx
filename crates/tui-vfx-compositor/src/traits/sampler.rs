// <FILE>tui-vfx-compositor/src/traits/sampler.rs</FILE>
// <DESC>Trait for coordinate remapping (Distortion)</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>WG5: Sampler Spatial Context Enhancement</WCTX>
// <CLOG>BREAKING CHANGE: Added width and height spatial parameters to sample() method signature</CLOG>

pub trait Sampler {
    /// Remaps destination coordinates to source coordinates for sampling/distortion effects.
    ///
    /// # Parameters
    /// - `dest_x`, `dest_y`: Destination coordinates to sample/remap
    /// - `width`, `height`: Render area dimensions for spatial awareness
    /// - `t`: Animation time parameter (0.0-1.0 for phase-based, or seconds for continuous)
    ///
    /// # Returns
    /// - `Some((src_x, src_y))`: Remapped source coordinates to sample from
    /// - `None`: Pixel should be transparent/skipped
    ///
    /// # Spatial Context
    /// The `width` and `height` parameters enable area-aware effects:
    /// - Dynamic centering (e.g., ripple at `(width/2, height/2)`)
    /// - Normalized coordinates for position-independent distortion
    /// - Boundary-aware displacement calculations
    ///
    /// Samplers that don't use spatial context may prefix parameters with `_` (e.g., `_width: u16`).
    fn sample(
        &self,
        dest_x: u16,
        dest_y: u16,
        width: u16,
        height: u16,
        t: f64,
    ) -> Option<(u16, u16)>;
}
