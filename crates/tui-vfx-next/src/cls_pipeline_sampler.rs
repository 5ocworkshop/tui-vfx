// <FILE>crates/tui-vfx-next/src/cls_pipeline_sampler.rs</FILE> - <DESC>Small sampler enum for pipeline toy stages</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>New kernel Phase D0 verifier fix: make sampler enum wire shape strict and fully described.</WCTX>
// <CLOG>0.4.1: PATCH — switch shifted sampler payload to a named field for strict schema descriptions.
// 0.4.0: PATCH — add Serde/Schemars schema-reference readiness while preserving runtime behavior.
// 0.1.0: ADD — support identity and shifted sampling inside ordered stages.</CLOG>

use crate::{CoordinateSampler, IdentitySampler, ShiftSampler};

/// Samplers supported by the Phase C toy pipeline stages.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
pub enum PipelineSampler {
    /// Identity sampling, preserving Phase A behavior.
    Identity,
    /// Shift sampling, preserving Phase B behavior inside a pipeline stage.
    Shift {
        /// Shift sampler configuration.
        sampler: ShiftSampler,
    },
}

impl CoordinateSampler for PipelineSampler {
    fn sample(
        &self,
        dest_x: usize,
        dest_y: usize,
        width: usize,
        height: usize,
    ) -> Option<(usize, usize)> {
        match self {
            Self::Identity => IdentitySampler.sample(dest_x, dest_y, width, height),
            Self::Shift { sampler } => sampler.sample(dest_x, dest_y, width, height),
        }
    }
}

impl From<ShiftSampler> for PipelineSampler {
    fn from(value: ShiftSampler) -> Self {
        Self::Shift { sampler: value }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_pipeline_sampler.rs</FILE> - <DESC>Small sampler enum for pipeline toy stages</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
