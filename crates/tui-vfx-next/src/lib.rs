// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 proof engine over contract DTOs</DESC>
// <VERS>VERSION: 0.10.0</VERS>
// <WCTX>New kernel Phase G3: expose proof topology execution and channel delta helpers.</WCTX>
// <CLOG>0.10.0: MINOR — add topology execution and proof-only channel-aware deltas.
// 0.9.0: MINOR — add proof GraphSpec executor, value snapshot, adapters, and execution errors.
// 0.8.0: PATCH — stop exporting the old proof EffectDescriptor so contract owns the durable descriptor name.
// 0.7.0: MINOR — depend on tui-vfx-contract, re-export contract vocabulary, and keep proof-only pipeline/effect helpers local.
// 0.6.0: MINOR — add logical contract, proof, and schema-root re-export modules without changing runtime behavior.</CLOG>

//! Clean-room v3.1 proof engine for tui-vfx.
//!
//! Stable contract DTOs live in `tui-vfx-contract`. This incubator crate proves
//! those contracts with a tiny surface engine, toy pipeline stages, proof
//! effects, and semantic tests. It intentionally avoids legacy compositor,
//! style, content, and shadow implementation crates.

pub mod cls_cell_channel_write;
pub mod cls_cell_delta;
pub mod cls_dim_effect;
pub mod cls_explicit_role_write_effect;
pub mod cls_graph_execution_context;
pub mod cls_graph_execution_error;
pub mod cls_graph_execution_outcome;
pub mod cls_graph_executor;
pub mod cls_identity_sampler;
pub mod cls_pipeline_outcome;
pub mod cls_pipeline_sampler;
pub mod cls_pipeline_stage;
pub mod cls_proof_effect_adapter;
pub mod cls_surface_delta;
pub mod cls_surface_engine;
pub mod cls_surface_pipeline;
pub mod fnc_annotate_node_diagnostics;
pub mod fnc_annotate_stage_diagnostic;
pub mod fnc_annotate_stage_diagnostics;
pub mod fnc_apply_from_source_with_sampler;
pub mod fnc_apply_surface_delta;
pub mod fnc_merge_surface_delta;
pub mod fnc_read_proof_input;
pub mod fnc_resolve_value_source;
pub mod fnc_rewrite_glyph_cell;
pub mod fnc_surface_delta_between;
pub mod orc_apply_proof_node;
pub mod orc_execute_graph_step;

pub use tui_vfx_contract::*;

pub use cls_cell_channel_write::CellChannelWrite;
pub use cls_cell_delta::CellDelta;
pub use cls_dim_effect::DimEffect;
pub use cls_explicit_role_write_effect::ExplicitRoleWriteEffect;
pub use cls_graph_execution_context::GraphExecutionContext;
pub use cls_graph_execution_error::GraphExecutionError;
pub use cls_graph_execution_outcome::GraphExecutionOutcome;
pub use cls_graph_executor::GraphExecutor;
pub use cls_identity_sampler::IdentitySampler;
pub use cls_pipeline_outcome::PipelineOutcome;
pub use cls_pipeline_sampler::PipelineSampler;
pub use cls_pipeline_stage::PipelineStage;
pub use cls_proof_effect_adapter::ProofEffectAdapter;
pub use cls_surface_delta::SurfaceDelta;
pub use cls_surface_engine::SurfaceEngine;
pub use cls_surface_pipeline::SurfacePipeline;

/// Stable v3.1 contract vocabulary imported from `tui-vfx-contract`.
pub mod contract {
    pub use tui_vfx_contract::*;
}

/// Clean-room proof harness and toy implementation pieces.
///
/// These exports are public so tests and downstream experiments can prove the
/// contract, but future descriptor or recipe schemas must not copy their toy
/// effect/stage shapes as the production descriptor model.
pub mod proof {
    pub use crate::{
        CellChannelWrite, CellDelta, DimEffect, ExplicitRoleWriteEffect, GraphExecutionContext,
        GraphExecutionError, GraphExecutionOutcome, GraphExecutor, IdentitySampler, PipelineStage,
        ProofEffectAdapter, SurfaceDelta, SurfaceEngine, SurfacePipeline,
    };
}

/// Checked proof schema roots owned by `tui-vfx-next`.
///
/// Stable contract schema roots moved to `tui-vfx-contract` in Phase E0. These
/// roots remain proof-pipeline artifacts under `schemas/v3.1/next/`.
pub mod schema_roots {
    pub use crate::{PipelineSampler, SurfacePipeline};
}

// <FILE>crates/tui-vfx-next/src/lib.rs</FILE> - <DESC>Clean-room v3.1 proof engine over contract DTOs</DESC>
// <VERS>END OF VERSION: 0.10.0</VERS>
