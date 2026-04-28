// <FILE>crates/tui-vfx-next/src/cls_shift_sampler.rs</FILE> - <DESC>Shift coordinate sampler</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract ShiftSampler class.</CLOG>

use crate::CoordinateSampler;

/// Shift sampler: destination `(x, y)` samples source `(x + dx, y + dy)`.
///
/// Positive `dx` samples from a source cell to the right of the destination;
/// positive `dy` samples from a source cell below the destination. Coordinates
/// shifted outside the source surface return `None` and skip the write.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ShiftSampler {
    /// Horizontal source offset relative to the destination coordinate.
    pub dx: i16,
    /// Vertical source offset relative to the destination coordinate.
    pub dy: i16,
}

impl ShiftSampler {
    /// Create a shift sampler.
    pub const fn new(dx: i16, dy: i16) -> Self {
        Self { dx, dy }
    }
}

impl CoordinateSampler for ShiftSampler {
    fn sample(
        &self,
        dest_x: usize,
        dest_y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        let source_x = dest_x.checked_add_signed(isize::from(self.dx))?;
        let source_y = dest_y.checked_add_signed(isize::from(self.dy))?;
        if source_x < width && source_y < height {
            Some((source_x, source_y))
        } else {
            None
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_shift_sampler.rs</FILE> - <DESC>Shift coordinate sampler</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
