// <FILE>crates/tui-vfx-next/src/tr_coordinate_sampler.rs</FILE> - <DESC>Coordinate sampler trait</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase C preflight OFPF split.</WCTX>
// <CLOG>0.3.0: REFACTOR — extract CoordinateSampler trait.</CLOG>

/// Maps destination coordinates to sampled-source coordinates.
pub trait CoordinateSampler {
    /// Return the sampled source coordinate for one destination coordinate.
    ///
    /// `width` and `height` are the source surface dimensions. Returning
    /// `None` skips the destination write and preserves destination state.
    fn sample(
        &self,
        dest_x: usize,
        dest_y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)>;
}

// <FILE>crates/tui-vfx-next/src/tr_coordinate_sampler.rs</FILE> - <DESC>Coordinate sampler trait</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
