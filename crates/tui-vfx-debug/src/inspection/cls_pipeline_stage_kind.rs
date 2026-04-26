// <FILE>crates/tui-vfx-debug/src/inspection/cls_pipeline_stage_kind.rs</FILE> - <DESC>PipelineStageKind — discriminator for per-stage TraceEvent variants</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — discriminator for per-stage TraceEvent variants (StageEntered/StageFinished/StageSkipped) so a sink can filter or join across the five pipeline stage kinds without parsing strings.</WCTX>
// <CLOG>0.1.0: initial enum with five stage kinds (Sampler, Mask, Shader, Filter, Shadow) covering every CompositorInspector stage callback today.</CLOG>

//! Discriminator for the per-stage [`TraceEvent`] variants.
//!
//! Each pipeline stage kind corresponds to one family of
//! [`crate::traits::pipeline_inspector::CompositorInspector`]-equivalent
//! callbacks in the compositor: Sampler, Mask, Shader, Filter, Shadow.
//! The kind appears on `StageEntered`, `StageFinished`, and
//! `StageSkipped` so consumers can filter or join across stages without
//! parsing free-form name strings.
//!
//! # Why an enum, not a string?
//!
//! Inspection sinks frequently filter on stage kind (`SQL WHERE kind =
//! 'Sampler'`, `--filter kind=Filter`). A closed-vocabulary enum gives
//! the filter machinery a stable join key and lets the compiler verify
//! exhaustive `match` at every consumer site.

use serde::{Deserialize, Serialize};

/// Discriminator for the per-stage [`crate::inspection::TraceEvent`]
/// variants emitted by the compositor pipeline.
///
/// One variant per stage kind in `render_loop_inspected` /
/// `apply_shaders_inspected` / `render_pipeline_with_shadow`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PipelineStageKind {
    /// Coordinate sampler — transforms destination cell coordinates to
    /// source-cell coordinates before downstream stages run.
    Sampler,
    /// Visibility mask — gates whether downstream stages affect a cell.
    Mask,
    /// Style shader — produces a new style for cells in scope.
    Shader,
    /// Cell filter — mutates cell content/style in place.
    Filter,
    /// Shadow stage — produces shadow-region cells before final blend.
    Shadow,
}

#[cfg(test)]
mod tests {
    use super::PipelineStageKind;

    #[test]
    fn round_trips_through_json() {
        for kind in [
            PipelineStageKind::Sampler,
            PipelineStageKind::Mask,
            PipelineStageKind::Shader,
            PipelineStageKind::Filter,
            PipelineStageKind::Shadow,
        ] {
            let json = serde_json::to_string(&kind).expect("serialize");
            let back: PipelineStageKind = serde_json::from_str(&json).expect("deserialize");
            assert_eq!(kind, back, "round-trip mismatch for {kind:?}");
        }
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_pipeline_stage_kind.rs</FILE> - <DESC>PipelineStageKind</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
