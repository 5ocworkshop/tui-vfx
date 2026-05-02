// <FILE>crates/tui-vfx-compost/src/primitive/cls_coordinate_sample.rs</FILE> - <DESC>Coordinate sampler primitive output</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Coordinate samplers produce source coordinates plus resolved-coordinate deltas rather than cell writes, matching the hardened legacy SamplerOutput contract.</WCTX>
// <CLOG>0.2.0: MINOR — carry optional source and delta_x/delta_y so sampler ports preserve legacy displacement semantics.
// 0.1.0: INIT — add simple coordinate sample output for sampler runtimes.</CLOG>

/// Source coordinate selected by a coordinate sampler for one destination cell.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Default)]
pub struct CoordinateSample {
    /// Source coordinate to sample from; `None` skips the destination cell.
    pub source: Option<(u16, u16)>,
    /// Sampler contribution to the resolved x-coordinate chain.
    pub delta_x: i32,
    /// Sampler contribution to the resolved y-coordinate chain.
    pub delta_y: i32,
}

impl CoordinateSample {
    /// Transparent/skipped output with no displacement.
    pub const fn no_displacement() -> Self {
        Self {
            source: None,
            delta_x: 0,
            delta_y: 0,
        }
    }

    /// Pass-through output that samples the local coordinate with zero displacement.
    pub const fn passthrough(local_x: u16, local_y: u16) -> Self {
        Self {
            source: Some((local_x, local_y)),
            delta_x: 0,
            delta_y: 0,
        }
    }

    /// Displaced output that samples a source coordinate and contributes deltas downstream.
    pub const fn displaced(source_x: u16, source_y: u16, delta_x: i32, delta_y: i32) -> Self {
        Self {
            source: Some((source_x, source_y)),
            delta_x,
            delta_y,
        }
    }

    /// Build a coordinate sample from signed source coordinates when both are in bounds.
    pub fn new(source_x: i32, source_y: i32) -> Self {
        match (u16::try_from(source_x), u16::try_from(source_y)) {
            (Ok(source_x), Ok(source_y)) => Self::passthrough(source_x, source_y),
            _ => Self::no_displacement(),
        }
    }
}

// <FILE>crates/tui-vfx-compost/src/primitive/cls_coordinate_sample.rs</FILE> - <DESC>Coordinate sampler primitive output</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
