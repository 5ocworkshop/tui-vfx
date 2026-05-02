// <FILE>crates/tui-vfx-compost/src/primitive/cls_coordinate_sample.rs</FILE> - <DESC>Coordinate sampler primitive output</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Coordinate samplers produce source coordinates/displacements rather than cell writes.</WCTX>
// <CLOG>0.1.0: INIT — add simple coordinate sample output for sampler runtimes.</CLOG>

/// Source coordinate selected by a coordinate sampler for one destination cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CoordinateSample {
    /// Sampled source x coordinate.
    pub source_x: i32,
    /// Sampled source y coordinate.
    pub source_y: i32,
}

impl CoordinateSample {
    /// Build a coordinate sample.
    pub const fn new(source_x: i32, source_y: i32) -> Self {
        Self { source_x, source_y }
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_coordinate_sample.rs</FILE> - <DESC>Coordinate sampler primitive output</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
