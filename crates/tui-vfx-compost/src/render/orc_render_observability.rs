// <FILE>crates/tui-vfx-compost/src/render/orc_render_observability.rs</FILE> - <DESC>Native render observability helpers</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Render observability aggregates stage lifecycle evidence emitted by the actual render path.</WCTX>
// <CLOG>0.4.0: MINOR — fold partial scope misses into finished stage evidence and reserve zero-cell skip for whole-stage misses.
// 0.3.0: MINOR — aggregate actual execution-stage evidence instead of precomputing duplicate scope results.
// 0.2.0: MINOR — lift role/scope tally and stage skipped/finished concepts for compost trace events.
// 0.1.0: INIT — add trace event builders for applied element stages.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{RecipeScene, RecipeSceneElement, ScopeEvalInput};
use tui_vfx_types::{Grid, RoleTag, SemanticScene};

use crate::render::{RenderSkipReason, RenderStageKind, RenderTraceEvent};

const ELEMENT_EFFECT: &str = "scene.element";
const PARALLEL_EFFECT: &str = "graph.parallel";
const SHADOW_EFFECT: &str = "surface.shadow";

#[derive(Clone, Debug, Default)]
pub(crate) struct RenderStageTrace {
    pub(crate) applied_effect_kinds: Vec<String>,
    pub(crate) trace_events: Vec<RenderTraceEvent>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum CellStageOutcome {
    Finished,
    Skipped(RenderSkipReason),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct CellStageTrace {
    pub(crate) stage_index: usize,
    pub(crate) stage_kind: RenderStageKind,
    pub(crate) effect: String,
    pub(crate) outcome: CellStageOutcome,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ScopeDestination {
    pub(crate) x: usize,
    pub(crate) y: usize,
    pub(crate) width: usize,
    pub(crate) height: usize,
}

impl CellStageTrace {
    pub(crate) fn finished(
        stage_index: usize,
        stage_kind: RenderStageKind,
        effect: impl Into<String>,
    ) -> Self {
        Self {
            stage_index,
            stage_kind,
            effect: effect.into(),
            outcome: CellStageOutcome::Finished,
        }
    }

    pub(crate) fn skipped(
        stage_index: usize,
        stage_kind: RenderStageKind,
        effect: impl Into<String>,
        reason: RenderSkipReason,
    ) -> Self {
        Self {
            stage_index,
            stage_kind,
            effect: effect.into(),
            outcome: CellStageOutcome::Skipped(reason),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
struct StageTraceKey {
    stage_index: usize,
    stage_kind: RenderStageKind,
    effect: String,
    skip_reason: Option<RenderSkipReason>,
}

#[derive(Clone, Copy, Debug, Default)]
struct StageTraceCounts {
    cells_matched: u32,
    cells_skipped: u32,
}

#[derive(Clone, Debug, Default)]
pub(crate) struct RenderStageAccumulator {
    stages: BTreeMap<StageTraceKey, StageTraceCounts>,
}

impl RenderStageAccumulator {
    pub(crate) fn record_cell_trace(&mut self, trace: CellStageTrace) {
        let (skip_reason, matched_delta, skipped_delta) = match trace.outcome {
            CellStageOutcome::Finished => (None, 1, 0),
            CellStageOutcome::Skipped(reason) => (Some(reason), 0, 1),
        };
        self.record(
            trace.stage_index,
            trace.stage_kind,
            trace.effect,
            skip_reason,
            matched_delta,
            skipped_delta,
        );
    }

    pub(crate) fn record_shadow_cells(&mut self, stage_index: usize, cells_matched: u32) {
        self.record(
            stage_index,
            RenderStageKind::Shadow,
            SHADOW_EFFECT.to_string(),
            None,
            cells_matched,
            0,
        );
    }

    pub(crate) fn extend(&mut self, other: Self) {
        for (key, counts) in other.stages {
            self.record(
                key.stage_index,
                key.stage_kind,
                key.effect,
                key.skip_reason,
                counts.cells_matched,
                counts.cells_skipped,
            );
        }
    }

    fn record(
        &mut self,
        stage_index: usize,
        stage_kind: RenderStageKind,
        effect: String,
        skip_reason: Option<RenderSkipReason>,
        matched_delta: u32,
        skipped_delta: u32,
    ) {
        let counts = self
            .stages
            .entry(StageTraceKey {
                stage_index,
                stage_kind,
                effect,
                skip_reason,
            })
            .or_default();
        counts.cells_matched += matched_delta;
        counts.cells_skipped += skipped_delta;
    }

    fn fold_partial_scope_misses(&mut self) {
        let scope_skip_keys: Vec<StageTraceKey> = self
            .stages
            .keys()
            .filter(|key| key.skip_reason == Some(RenderSkipReason::ScopeMatchedZeroCells))
            .cloned()
            .collect();
        for skip_key in scope_skip_keys {
            let finished_key = StageTraceKey {
                stage_index: skip_key.stage_index,
                stage_kind: skip_key.stage_kind,
                effect: skip_key.effect.clone(),
                skip_reason: None,
            };
            if !self.stages.contains_key(&finished_key) {
                continue;
            }
            let Some(skip_counts) = self.stages.remove(&skip_key) else {
                continue;
            };
            if let Some(finished_counts) = self.stages.get_mut(&finished_key) {
                finished_counts.cells_skipped += skip_counts.cells_skipped;
            }
        }
    }

    pub(crate) fn finish(
        mut self,
        scene: &RecipeScene,
        element: &RecipeSceneElement,
    ) -> RenderStageTrace {
        self.fold_partial_scope_misses();
        let mut trace = RenderStageTrace::default();
        for (key, counts) in self.stages {
            match key.skip_reason {
                Some(reason) => trace.trace_events.push(RenderTraceEvent::skipped(
                    scene.id.as_str(),
                    element.id.as_str(),
                    key.stage_index,
                    key.stage_kind,
                    key.effect,
                    reason,
                    counts.cells_matched,
                    counts.cells_skipped,
                )),
                None => {
                    if key.stage_kind != RenderStageKind::Parallel
                        && key.stage_kind != RenderStageKind::Shadow
                    {
                        trace.applied_effect_kinds.push(key.effect.clone());
                    }
                    trace.trace_events.push(RenderTraceEvent::finished(
                        scene.id.as_str(),
                        element.id.as_str(),
                        key.stage_index,
                        key.stage_kind,
                        key.effect,
                        counts.cells_matched,
                        counts.cells_skipped,
                    ));
                }
            }
        }
        trace
    }
}

pub(crate) fn trace_element_skipped(
    scene: &RecipeScene,
    element: &RecipeSceneElement,
    reason: RenderSkipReason,
) -> RenderTraceEvent {
    RenderTraceEvent::skipped(
        scene.id.as_str(),
        element.id.as_str(),
        0,
        RenderStageKind::Element,
        ELEMENT_EFFECT,
        reason,
        0,
        0,
    )
}

pub(crate) fn parallel_cell_trace(stage_index: usize) -> CellStageTrace {
    CellStageTrace::finished(stage_index, RenderStageKind::Parallel, PARALLEL_EFFECT)
}

pub(crate) fn scope_eval_input(
    source: &SemanticScene,
    destination: &SemanticScene,
    scope_destination: ScopeDestination,
    local_x: usize,
    local_y: usize,
    dest_x: usize,
    dest_y: usize,
) -> ScopeEvalInput {
    let sampled_source_role = source
        .role((local_x as u16, local_y as u16))
        .unwrap_or(RoleTag::Background);
    let destination_role = destination
        .role((dest_x as u16, dest_y as u16))
        .unwrap_or(RoleTag::Background);
    let sampled_source_glyph = source
        .grid()
        .get(local_x, local_y)
        .map(|cell| cell.ch.to_string());
    let destination_glyph = destination
        .grid()
        .get(dest_x, dest_y)
        .map(|cell| cell.ch.to_string());

    ScopeEvalInput {
        destination_x: scope_destination.x,
        destination_y: scope_destination.y,
        sampled_source_x: local_x,
        sampled_source_y: local_y,
        sampled_source_role,
        destination_role,
        destination_width: Some(scope_destination.width),
        destination_height: Some(scope_destination.height),
        sampled_source_width: Some(source.grid().width()),
        sampled_source_height: Some(source.grid().height()),
        destination_glyph,
        sampled_source_glyph,
    }
}

// <FILE>crates/tui-vfx-compost/src/render/orc_render_observability.rs</FILE> - <DESC>Native render observability helpers</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
