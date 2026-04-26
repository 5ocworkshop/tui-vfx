// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_event.rs</FILE> - <DESC>TraceEvent taxonomy</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Pipeline observability Unit A — extend the Pipeline section with five new variants (StageEntered/StageFinished/StageSkipped/ScopeEvaluated/RoleMapMaterialized) so the focused_row_btop scope-mismatch class is mechanically detectable on the event stream.</WCTX>
// <CLOG>0.2.0: add five Pipeline variants for per-stage entry/exit/skip + scope-evaluation evidence + role-map source discriminant; stage() accessor extended to map them to StageMask::PIPELINE.</CLOG>

//! The canonical [`TraceEvent`] taxonomy.
//!
//! Emitted by every stage of the unified recipe-scene-composer pipeline:
//! lifecycle / resolution / composition / pipeline. Each variant carries
//! the minimum information needed for post-hoc inspection (replay, AI
//! analysis, selector-based filtering).
//!
//! Variants are organised into four stages — see [`TraceEvent::stage`] —
//! matching the [`StageMask`] bits. The enum is `#[non_exhaustive]`
//! because the catalog grows when new engine stages are added; external
//! consumers must match with a wildcard arm.

use serde::{Deserialize, Serialize};
use tui_vfx_types::{Cell, LayerId, RecipeId, Rect, RoleTag, Style};

use super::cls_pipeline_skip_reason::PipelineSkipReason;
use super::cls_pipeline_stage_kind::PipelineStageKind;
use super::cls_role_histogram::RoleHistogram;
use super::cls_role_map_source::RoleMapSource;
use super::cls_stage_mask::StageMask;

/// Canonical inspection event emitted by the unified trace pipeline.
///
/// Matches the taxonomy defined in the recipe-scene-composer spec §9.2.
///
/// Variant grouping (via [`Self::stage`]):
///
/// - **Lifecycle:** `LifecyclePhaseEntered`, `LifecyclePhaseTransition`,
///   `LifecycleDismissed`, `LifecycleHeld`.
/// - **Resolution:** `AssetResolved`, `ProceduralResolved`,
///   `TokenResolved`, `RecipeBindingResolved`.
/// - **Composition:** `LayerStarted`, `LayerCellPainted`,
///   `LayerCompleted`, `LayerSkipped`.
/// - **Pipeline:** `SamplerApplied`, `MaskChecked`, `ShaderApplied`,
///   `FilterApplied`, `ShadowCellApplied`, `CellRendered`, plus the
///   per-stage and scope-evidence variants `StageEntered`,
///   `StageFinished`, `StageSkipped`, `ScopeEvaluated`,
///   `RoleMapMaterialized`.
#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TraceEvent {
    // ── Lifecycle (tui-vfx-recipes::manager) ──────────────────────────
    /// A recipe lifecycle phase was entered (e.g. `"enter"`, `"dwell"`).
    LifecyclePhaseEntered {
        /// Recipe identity carrying this lifecycle.
        id: RecipeId,
        /// Name of the phase that was just entered.
        phase: String,
        /// Elapsed milliseconds since the manager started.
        t_ms: u64,
    },
    /// A phase transition with easing progress (0.0..1.0).
    LifecyclePhaseTransition {
        /// Recipe identity carrying this lifecycle.
        id: RecipeId,
        /// Previous phase name.
        from: String,
        /// New phase name.
        to: String,
        /// Elapsed milliseconds since the manager started.
        t_ms: u64,
        /// Eased progress through the transition (0.0..1.0).
        eased_progress: f64,
    },
    /// Recipe lifecycle was dismissed (dwelling ended).
    LifecycleDismissed {
        /// Recipe identity.
        id: RecipeId,
        /// Free-form dismissal reason (e.g. `"timeout"`, `"user"`).
        reason: String,
        /// Elapsed milliseconds since the manager started.
        t_ms: u64,
    },
    /// Recipe lifecycle is holding until a deadline.
    LifecycleHeld {
        /// Recipe identity.
        id: RecipeId,
        /// Milliseconds until the hold releases.
        until_ms: u64,
    },

    // ── Resolution (tui-vfx-recipes::scene) ────────────────────────────
    /// A recipe asset was resolved (or fell back).
    AssetResolved {
        /// Asset name (recipe-declared).
        name: String,
        /// Whether the asset was resolved successfully.
        found: bool,
        /// When `found == false`, the reason the fallback was used.
        fallback_reason: Option<String>,
    },
    /// A procedural source was resolved by id.
    ProceduralResolved {
        /// Source identifier (recipe-declared).
        source_id: String,
        /// Whether the source resolved successfully.
        resolved: bool,
        /// Fallback id used when primary was missing.
        fallback_id: Option<String>,
    },
    /// A token was expanded against the active token context.
    TokenResolved {
        /// Input token reference (e.g. `"{theme}"`).
        input: String,
        /// Resolved output value.
        output: String,
        /// Token keys that were absent from the context.
        missing_keys: Vec<String>,
    },
    /// A recipe binding was resolved.
    RecipeBindingResolved {
        /// Selector string that triggered the binding.
        selector: String,
        /// Recipe identity the binding resolved to.
        recipe_id: RecipeId,
        /// Theme name in scope for this binding.
        theme: String,
    },

    // ── Composition (tui-vfx-recipes::scene) ───────────────────────────
    /// A scene layer started painting.
    LayerStarted {
        /// Layer identity.
        layer_id: LayerId,
        /// Z-order for this layer (higher = on top).
        z: i32,
        /// Kind of content source (e.g. `"scene-fragment"`, `"procedural"`).
        source_kind: String,
        /// Layer's target rectangle in the destination surface.
        target_rect: Rect,
    },
    /// A single cell was painted into the composed surface.
    LayerCellPainted {
        /// Owning layer's identity.
        layer_id: LayerId,
        /// Destination x coordinate.
        x: u16,
        /// Destination y coordinate.
        y: u16,
        /// Glyph written.
        glyph: char,
        /// Semantic role tagged for this cell.
        role: RoleTag,
    },
    /// A layer finished painting — summary accounting.
    LayerCompleted {
        /// Owning layer's identity.
        layer_id: LayerId,
        /// Count of cells painted.
        cells_painted: u32,
        /// Count of cells skipped (out of bounds, masked, empty source).
        cells_skipped: u32,
        /// Whether a fallback source was used.
        fallback: bool,
    },
    /// A layer was skipped entirely (e.g. unresolved source).
    LayerSkipped {
        /// Layer identity.
        layer_id: LayerId,
        /// Free-form skip reason.
        reason: String,
    },

    // ── Pipeline (tui-vfx-compositor) ──────────────────────────────────
    /// Sampler transformed coordinates for a cell.
    SamplerApplied {
        /// Destination x.
        dest_x: u16,
        /// Destination y.
        dest_y: u16,
        /// Source x after sampling (None = cell skipped / gap).
        src_x: Option<u16>,
        /// Source y after sampling (None = cell skipped / gap).
        src_y: Option<u16>,
        /// Sampler name (e.g. `"ripple#1"`).
        sampler: String,
    },
    /// A mask visibility result was computed for a cell.
    MaskChecked {
        /// Cell x.
        x: u16,
        /// Cell y.
        y: u16,
        /// Whether the mask allows this cell through.
        visible: bool,
        /// Mask name (e.g. `"wipe#1"`).
        mask: String,
    },
    /// A shader produced a new style for a cell.
    ShaderApplied {
        /// Cell x.
        x: u16,
        /// Cell y.
        y: u16,
        /// Style before the shader ran.
        before: Style,
        /// Style after the shader ran.
        after: Style,
        /// Shader name (e.g. `"pulse#1"`).
        shader: String,
        /// Canonical region name the shader targeted
        /// (e.g. `"Border"`, `"Full"`). `None` if unknown.
        region: Option<String>,
    },
    /// A filter mutated a cell.
    FilterApplied {
        /// Cell x.
        x: u16,
        /// Cell y.
        y: u16,
        /// Cell before the filter ran.
        before: Cell,
        /// Cell after the filter ran.
        after: Cell,
        /// Filter name (e.g. `"tint#1"`).
        filter: String,
    },
    /// A shadow-region cell was prepared (before final blend/preserve).
    ShadowCellApplied {
        /// Cell x in the extended shadow area.
        x: u16,
        /// Cell y in the extended shadow area.
        y: u16,
        /// Shadow cell as produced by the shadow stage.
        shadow_cell: Cell,
        /// Role of the corresponding source cell (may be `None` when
        /// the shadow extrudes from an empty source cell).
        source_role: Option<RoleTag>,
        /// Whether the corresponding source cell is empty/missing.
        source_empty: bool,
    },
    /// The pipeline finalised a cell to the destination.
    CellRendered {
        /// Cell x.
        x: u16,
        /// Cell y.
        y: u16,
        /// Final cell written to the destination.
        final_cell: Cell,
    },

    // ── Pipeline — per-stage entry / exit / skip + scope evidence ─────
    /// A pipeline stage began applying.
    ///
    /// Emitted at the boundary of every Sampler / Mask / Shader / Filter /
    /// Shadow stage application in `render_loop_inspected` and
    /// `apply_shaders_inspected`. Paired with `StageFinished` (or
    /// replaced by `StageSkipped` when the stage skipped iteration).
    StageEntered {
        /// Which kind of stage just started.
        kind: PipelineStageKind,
        /// 1-based per-render stage counter; the join key for inspector queries.
        step_id: u32,
        /// Stage display name (e.g. `"FocusedRowGradient"`, `"FadeIn"`).
        name: String,
        /// Closed-vocabulary scope predicate summary (e.g. `"Role(Text)"`,
        /// `"And(RowRange,Channel(Background))"`). Empty string when the
        /// stage has no explicit scope (whole-area).
        scope_summary: String,
    },
    /// A pipeline stage finished applying.
    StageFinished {
        /// Which kind of stage just finished.
        kind: PipelineStageKind,
        /// 1-based per-render stage counter (matches the paired `StageEntered`).
        step_id: u32,
        /// Upper bound on cells the stage modified.
        ///
        /// In Pipeline observability Unit A this is the pre-loop scope-tally
        /// match count for shaders, or `area_total` for Sampler/Mask/Filter/
        /// Shadow stages that have no per-stage scope predicate today. It
        /// over-counts when a sampler returns `None` for some cells or a
        /// mask culls visibility — those cells are visited by the predicate
        /// but never written by the stage. A future enrichment will thread
        /// the actual per-cell mutation counter from the cell loop and
        /// tighten this to the exact written count.
        cells_modified: u32,
        /// Wall-clock duration of the stage application, in nanoseconds.
        elapsed_ns: u64,
    },
    /// A pipeline stage was skipped without iterating cells.
    ///
    /// The `reason` discriminator carries the data needed to reproduce
    /// the decision (e.g. `ScopeMatchedZeroCells` includes the predicate
    /// summary and the per-role histogram the predicate visited).
    StageSkipped {
        /// Which kind of stage was skipped.
        kind: PipelineStageKind,
        /// 1-based per-render stage counter.
        step_id: u32,
        /// Why the stage was skipped.
        reason: PipelineSkipReason,
    },
    /// Scope predicate evaluation summary for one stage application.
    ///
    /// Sum of `matched + skipped` equals the number of cells in the
    /// stage's effective area. The histogram shows what the predicate
    /// saw — load-bearing for diagnosing role-map mismatches like the
    /// focused_row_btop case study.
    ScopeEvaluated {
        /// 1-based per-render stage counter.
        step_id: u32,
        /// Number of cells the predicate matched.
        matched: u32,
        /// Number of cells the predicate skipped.
        skipped: u32,
        /// Per-role cell counts the predicate visited.
        role_histogram: RoleHistogram,
    },
    /// A role map became available to the pipeline; carries the source
    /// (geometric inference vs explicit producer-tagged vs externally-injected).
    RoleMapMaterialized {
        /// Where the role map came from.
        source: RoleMapSource,
        /// Per-role cell counts in the materialized role map.
        histogram: RoleHistogram,
    },
}

impl TraceEvent {
    /// The stage bitmask value for this variant.
    ///
    /// Used by [`crate::inspection::TraceFilter`] to gate events at
    /// sink-time without re-dispatching on the event variant.
    pub fn stage(&self) -> StageMask {
        match self {
            TraceEvent::LifecyclePhaseEntered { .. }
            | TraceEvent::LifecyclePhaseTransition { .. }
            | TraceEvent::LifecycleDismissed { .. }
            | TraceEvent::LifecycleHeld { .. } => StageMask::LIFECYCLE,

            TraceEvent::AssetResolved { .. }
            | TraceEvent::ProceduralResolved { .. }
            | TraceEvent::TokenResolved { .. }
            | TraceEvent::RecipeBindingResolved { .. } => StageMask::RESOLUTION,

            TraceEvent::LayerStarted { .. }
            | TraceEvent::LayerCellPainted { .. }
            | TraceEvent::LayerCompleted { .. }
            | TraceEvent::LayerSkipped { .. } => StageMask::COMPOSITION,

            TraceEvent::SamplerApplied { .. }
            | TraceEvent::MaskChecked { .. }
            | TraceEvent::ShaderApplied { .. }
            | TraceEvent::FilterApplied { .. }
            | TraceEvent::ShadowCellApplied { .. }
            | TraceEvent::CellRendered { .. }
            | TraceEvent::StageEntered { .. }
            | TraceEvent::StageFinished { .. }
            | TraceEvent::StageSkipped { .. }
            | TraceEvent::ScopeEvaluated { .. }
            | TraceEvent::RoleMapMaterialized { .. } => StageMask::PIPELINE,
        }
    }
}

#[cfg(test)]
mod pipeline_observability_tests {
    use super::TraceEvent;
    use crate::inspection::{
        PipelineSkipReason, PipelineStageKind, RoleHistogram, RoleMapSource, StageMask,
    };

    fn round_trip(event: &TraceEvent) -> TraceEvent {
        let json = serde_json::to_string(event).expect("serialize");
        serde_json::from_str(&json).expect("deserialize")
    }

    #[test]
    fn stage_entered_round_trips_and_maps_to_pipeline_stage() {
        let e = TraceEvent::StageEntered {
            kind: PipelineStageKind::Shader,
            step_id: 1,
            name: "FocusedRowGradient".to_string(),
            scope_summary: "Role(Text)".to_string(),
        };
        assert_eq!(round_trip(&e), e);
        assert_eq!(e.stage(), StageMask::PIPELINE);
    }

    #[test]
    fn stage_finished_round_trips_and_maps_to_pipeline_stage() {
        let e = TraceEvent::StageFinished {
            kind: PipelineStageKind::Filter,
            step_id: 2,
            cells_modified: 17,
            elapsed_ns: 4_200,
        };
        assert_eq!(round_trip(&e), e);
        assert_eq!(e.stage(), StageMask::PIPELINE);
    }

    #[test]
    fn stage_skipped_round_trips_and_maps_to_pipeline_stage() {
        let e = TraceEvent::StageSkipped {
            kind: PipelineStageKind::Shader,
            step_id: 3,
            reason: PipelineSkipReason::ScopeMatchedZeroCells {
                predicate: "Role(Text)".to_string(),
                role_histogram: RoleHistogram {
                    background: 320,
                    text: 0,
                    border: 0,
                    indicator: 0,
                    highlight: 0,
                },
            },
        };
        assert_eq!(round_trip(&e), e);
        assert_eq!(e.stage(), StageMask::PIPELINE);
    }

    #[test]
    fn scope_evaluated_round_trips_and_maps_to_pipeline_stage() {
        let e = TraceEvent::ScopeEvaluated {
            step_id: 1,
            matched: 0,
            skipped: 320,
            role_histogram: RoleHistogram {
                background: 320,
                ..RoleHistogram::EMPTY
            },
        };
        assert_eq!(round_trip(&e), e);
        assert_eq!(e.stage(), StageMask::PIPELINE);
    }

    #[test]
    fn role_map_materialized_round_trips_and_maps_to_pipeline_stage() {
        let e = TraceEvent::RoleMapMaterialized {
            source: RoleMapSource::ExplicitFromProducer {
                producer: "ContentShell::card".to_string(),
            },
            histogram: RoleHistogram {
                background: 320,
                ..RoleHistogram::EMPTY
            },
        };
        assert_eq!(round_trip(&e), e);
        assert_eq!(e.stage(), StageMask::PIPELINE);
    }
}

// <FILE>crates/tui-vfx-debug/src/inspection/cls_trace_event.rs</FILE> - <DESC>TraceEvent taxonomy</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
