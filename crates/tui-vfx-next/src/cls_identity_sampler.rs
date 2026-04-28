// <FILE>crates/tui-vfx-next/src/cls_identity_sampler.rs</FILE> - <DESC>Identity coordinate sampler</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>New kernel Phase D0 schema/reference backfill after Phase C preflight OFPF split.</WCTX>
// <CLOG>0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.3.0: REFACTOR — extract IdentitySampler class.</CLOG>

use crate::CoordinateSampler;

/// Identity sampler: destination `(x, y)` samples source `(x, y)`.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    Eq,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
pub struct IdentitySampler;

impl CoordinateSampler for IdentitySampler {
    fn sample(
        &self,
        dest_x: usize,
        dest_y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        if dest_x < width && dest_y < height {
            Some((dest_x, dest_y))
        } else {
            None
        }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_identity_sampler.rs</FILE> - <DESC>Identity coordinate sampler</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
