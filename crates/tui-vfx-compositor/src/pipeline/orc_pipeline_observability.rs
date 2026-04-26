// <FILE>crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs</FILE> - <DESC>Per-stage observability emit helpers for render_loop_inspected and the shadow path's element pass</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Pipeline observability Unit A — architect-flagged B1+B2 fixes: zero-cell skip suppresses StageEntered (PRD US-006 strict reading); pre-loop/post-loop emit blocks extracted so the shadow path's element pass can install per-stage observability.</WCTX>
// <CLOG>0.2.0: when a shader's scope matches zero cells, suppress on_stage_entered (still emit on_scope_evaluated for diagnosis, then on_stage_skipped); extract emit_per_stage_entered_block + emit_per_stage_finished_block so the shadow element pass shares the same per-stage block as render_loop_inspected.</CLOG>

//! Per-stage observability helpers used by `render_loop_inspected`.
//!
//! These helpers carry the new TraceEvent emission for Pipeline
//! observability Unit A: `RoleMapMaterialized`, `StageEntered`,
//! `ScopeEvaluated`, `StageSkipped`, `StageFinished`. They keep the
//! event-emit logic out of `orc_render_pipeline::render_loop_inspected`
//! so the cell loop stays readable.
//!
//! # Scope tally for shaders
//!
//! Shaders are the only stage kind today with a first-class per-stage
//! scope predicate (`StyleRegion`). For each shader layer, this module
//! evaluates the resolved region against the source `RoleMap` once at
//! pipeline entry, tallies (matched, skipped, role histogram), and
//! either:
//!
//! - when `matched == 0`: emits `ScopeEvaluated { matched: 0, .. }`
//!   followed by `StageSkipped { ScopeMatchedZeroCells }`, marks the
//!   shader for skipping in the cell loop, and never emits `StageEntered`
//!   or `StageFinished` (PRD US-006 strict skip semantics — skip events
//!   replace the entered/finished pair, they don't accompany it), or
//! - when `matched > 0`: emits `StageEntered` followed by `ScopeEvaluated`
//!   and lets the cell loop apply the shader normally; the matching
//!   `StageFinished` is emitted after the loop.
//!
//! Sampler / Mask / Filter / Shadow stages do not have a first-class
//! per-stage scope predicate today (they apply to every cell in the
//! area). They get a degenerate `ScopeEvaluated { matched=area, skipped=0 }`
//! and a straight `StageEntered` / `StageFinished` pair around the cell loop.

use std::time::Instant;

use tui_vfx_debug::inspection::{
    PipelineSkipReason, PipelineStageKind, RoleHistogram, RoleMapSource,
};
use tui_vfx_style::models::StyleRegion;
use tui_vfx_types::{Rect, RoleMap, RoleTag};

use crate::pipeline::cls_composition_options::ShaderWithRegion;
use crate::traits::pipeline_inspector::CompositorInspector;

/// Iterate the area cells of `roles` and bucket them into a [`RoleHistogram`].
///
/// Roles outside the histogram's five buckets (Title, Caption, Image,
/// Icon, Shadow, Decoration, Procedural, Custom) are not counted today.
/// Sum across buckets ≤ `width * height`.
pub(crate) fn tally_role_histogram_for_area(
    roles: &RoleMap,
    width: u16,
    height: u16,
) -> RoleHistogram {
    let mut hist = RoleHistogram::EMPTY;
    for y in 0..height {
        for x in 0..width {
            match roles.get((x, y)) {
                Some(RoleTag::Background) => hist.background += 1,
                Some(RoleTag::Text) => hist.text += 1,
                Some(RoleTag::Border) => hist.border += 1,
                Some(RoleTag::Indicator) => hist.indicator += 1,
                Some(RoleTag::Highlight) => hist.highlight += 1,
                _ => {}
            }
        }
    }
    hist
}

/// Closed-vocabulary summary string for a [`StyleRegion`].
///
/// Stable across the lifetime of the variant — consumers may filter on
/// the literal text (`--filter scope_summary='Role(Text)'`).
pub(crate) fn style_region_summary(region: &StyleRegion) -> String {
    match region {
        StyleRegion::All => "All".to_string(),
        StyleRegion::Role(tag) => format!("Role({tag:?})"),
        StyleRegion::Rows(rows) => format!("Rows({} entries)", rows.len()),
        StyleRegion::RowRange { .. } => "RowRange".to_string(),
        StyleRegion::Cell { .. } => "Cell".to_string(),
        StyleRegion::Cells(cells) => format!("Cells({} entries)", cells.len()),
        StyleRegion::Column(_) => "Column".to_string(),
        StyleRegion::Columns(cols) => format!("Columns({} entries)", cols.len()),
        StyleRegion::ColumnRange { .. } => "ColumnRange".to_string(),
        StyleRegion::Modulo { .. } => "Modulo".to_string(),
    }
}

/// Tally a shader scope predicate against the source role map.
///
/// Iterates the area, calls `region.should_style(x, y, role, area)` for
/// each cell, and returns (matched, skipped, role_histogram). The
/// histogram counts role tags only for cells the predicate visited
/// (i.e. all cells in the area).
pub(crate) fn tally_shader_scope_match(
    region: &StyleRegion,
    roles: &RoleMap,
    width: u16,
    height: u16,
) -> (u32, u32, RoleHistogram) {
    let area = Rect::new(0, 0, width, height);
    let mut matched = 0u32;
    let mut skipped = 0u32;
    let mut hist = RoleHistogram::EMPTY;
    for y in 0..height {
        for x in 0..width {
            let role = roles.get((x, y));
            match role {
                Some(RoleTag::Background) => hist.background += 1,
                Some(RoleTag::Text) => hist.text += 1,
                Some(RoleTag::Border) => hist.border += 1,
                Some(RoleTag::Indicator) => hist.indicator += 1,
                Some(RoleTag::Highlight) => hist.highlight += 1,
                _ => {}
            }
            if region.should_style(x, y, role, area) {
                matched += 1;
            } else {
                skipped += 1;
            }
        }
    }
    (matched, skipped, hist)
}

/// Per-shader observability state carried across the cell loop.
///
/// Built in [`emit_shader_entered_or_skipped`] and consumed by
/// [`emit_shader_finished`] after the cell loop runs.
pub(crate) struct ShaderObservabilityState {
    /// step_id assigned to this shader.
    pub step_id: u32,
    /// Start time for elapsed_ns measurement.
    pub started_at: Instant,
    /// Whether the cell loop should skip applying this shader (set when
    /// the scope tally returned matched=0).
    pub skipped: bool,
    /// `matched` count from the scope tally (used as `cells_modified` in
    /// the post-loop `StageFinished` for now; future enrichment can
    /// thread the actual mutation count from the cell loop).
    pub matched: u32,
}

/// Emit the one-per-render `RoleMapMaterialized` event with the area histogram.
///
/// Source is `Injected` in Unit A — gt-design upgrades this to
/// `ExplicitFromProducer { producer: "<widget>" }` in Unit B by
/// installing its own `CompositorInspector` that overrides
/// `on_role_map_materialized` upstream of the pipeline call.
pub(crate) fn emit_role_map_materialized(
    inspector: &mut dyn CompositorInspector,
    roles: &RoleMap,
    width: u16,
    height: u16,
) {
    let histogram = tally_role_histogram_for_area(roles, width, height);
    inspector.on_role_map_materialized(RoleMapSource::Injected, histogram);
}

/// Emit `StageEntered` for a shader, or `StageSkipped` when the scope
/// matched zero cells. Returns observability state to thread through
/// the cell loop.
///
/// Skip semantics follow the strict PRD US-006 reading: when
/// `matched == 0`, the order is `ScopeEvaluated` then `StageSkipped`,
/// and **no** `StageEntered` fires. The matching `emit_shader_finished`
/// short-circuits when the returned state is `skipped`. Consumers that
/// want to count "stage instances seen" without conflating skipped and
/// applied stages can rely on `StageEntered` being absent for skipped
/// shaders.
pub(crate) fn emit_shader_entered_or_skipped(
    inspector: &mut dyn CompositorInspector,
    step_id: u32,
    layer: &ShaderWithRegion<'_>,
    resolved_region: &StyleRegion,
    roles: &RoleMap,
    width: u16,
    height: u16,
    shader_index: usize,
) -> ShaderObservabilityState {
    let scope_summary = style_region_summary(resolved_region);
    let (matched, skipped, hist) = tally_shader_scope_match(resolved_region, roles, width, height);

    if matched == 0 {
        // ScopeEvaluated still fires (load-bearing diagnosis evidence —
        // matched=0 is exactly the bug class US-008 detects), then
        // StageSkipped names the cause. StageEntered is suppressed.
        inspector.on_scope_evaluated(step_id, matched, skipped, hist);
        inspector.on_stage_skipped(
            PipelineStageKind::Shader,
            step_id,
            PipelineSkipReason::ScopeMatchedZeroCells {
                predicate: scope_summary,
                role_histogram: hist,
            },
        );
        return ShaderObservabilityState {
            step_id,
            started_at: Instant::now(),
            skipped: true,
            matched: 0,
        };
    }

    let name = layer.inspector_shader_label(shader_index);
    inspector.on_stage_entered(PipelineStageKind::Shader, step_id, &name, &scope_summary);
    inspector.on_scope_evaluated(step_id, matched, skipped, hist);

    ShaderObservabilityState {
        step_id,
        started_at: Instant::now(),
        skipped: false,
        matched,
    }
}

/// Emit `StageFinished` for a shader unless it was skipped.
pub(crate) fn emit_shader_finished(
    inspector: &mut dyn CompositorInspector,
    state: &ShaderObservabilityState,
) {
    if state.skipped {
        return;
    }
    let elapsed_ns = state
        .started_at
        .elapsed()
        .as_nanos()
        .min(u128::from(u64::MAX)) as u64;
    inspector.on_stage_finished(
        PipelineStageKind::Shader,
        state.step_id,
        state.matched,
        elapsed_ns,
    );
}

/// Per-instance observability state for the simple stage kinds (Sampler,
/// Mask, Filter, Shadow) that have no first-class per-stage scope
/// predicate today.
pub(crate) struct SimpleStageState {
    /// step_id assigned to this stage.
    pub step_id: u32,
    /// Stage kind (for the matching StageFinished).
    pub kind: PipelineStageKind,
    /// Start time for elapsed_ns measurement.
    pub started_at: Instant,
    /// Total cells in the area (used as `cells_modified` placeholder).
    pub area_total: u32,
}

/// Emit `StageEntered` + a degenerate `ScopeEvaluated { matched=area, skipped=0 }`
/// for a simple stage kind (Sampler / Mask / Filter / Shadow). Returns
/// observability state for the matching post-loop emit.
pub(crate) fn emit_simple_stage_entered(
    inspector: &mut dyn CompositorInspector,
    kind: PipelineStageKind,
    step_id: u32,
    name: &str,
    roles: &RoleMap,
    width: u16,
    height: u16,
) -> SimpleStageState {
    let area_total = u32::from(width) * u32::from(height);
    let hist = tally_role_histogram_for_area(roles, width, height);

    inspector.on_stage_entered(kind, step_id, name, "");
    inspector.on_scope_evaluated(step_id, area_total, 0, hist);

    SimpleStageState {
        step_id,
        kind,
        started_at: Instant::now(),
        area_total,
    }
}

/// Emit the matching `StageFinished` for a simple stage.
pub(crate) fn emit_simple_stage_finished(
    inspector: &mut dyn CompositorInspector,
    state: &SimpleStageState,
) {
    let elapsed_ns = state
        .started_at
        .elapsed()
        .as_nanos()
        .min(u128::from(u64::MAX)) as u64;
    inspector.on_stage_finished(state.kind, state.step_id, state.area_total, elapsed_ns);
}

/// Bundled state from a pre-loop per-stage emit. Threaded into the
/// matching post-loop [`emit_per_stage_finished`] call after the cell
/// loop completes.
pub(crate) struct PerStageBlock {
    /// Sampler / Mask / Filter / Shadow stages that emitted a
    /// StageEntered/ScopeEvaluated pair and need a matching StageFinished.
    pub simple_states: Vec<SimpleStageState>,
    /// Shader stage states. Skipped shaders carry `skipped: true` so the
    /// matching [`emit_shader_finished`] call short-circuits.
    pub shader_states: Vec<ShaderObservabilityState>,
    /// Indices into the shader-layers slice for shaders whose scope
    /// matched zero cells. The cell loop must skip applying these.
    pub skipped_shader_indices: Vec<usize>,
}

/// Inputs to [`emit_per_stage_entered`]. Bundling the per-stage labels
/// and area dimensions into one borrow keeps the call site readable and
/// avoids piling positional arguments on the helper.
pub(crate) struct PerStageInputs<'a> {
    /// Optional sampler label — `Some(label)` emits one Sampler stage,
    /// `None` skips the Sampler emit entirely.
    pub sampler_label: Option<&'a str>,
    /// One label per prepared mask, in pipeline order. Each label emits
    /// one Mask stage.
    pub mask_labels: &'a [String],
    /// One `(layer, resolved-region)` pair per shader, in pipeline
    /// order. Each pair emits one Shader stage (or a StageSkipped if its
    /// scope matches zero cells).
    pub shader_layers_with_regions: &'a [(&'a ShaderWithRegion<'a>, &'a StyleRegion)],
    /// One label per prepared filter, in pipeline order. Each label
    /// emits one Filter stage.
    pub filter_labels: &'a [String],
    /// Source role map; consulted by every per-stage emit for the area
    /// histogram and by the shader scope tally.
    pub source_roles: &'a RoleMap,
    pub width: u16,
    pub height: u16,
}

/// Emit the pre-loop per-stage block: one `StageEntered` (or
/// `StageSkipped` for zero-match shaders) plus matching `ScopeEvaluated`
/// for each Sampler / Mask / Shader / Filter stage instance, in
/// pipeline order. `RoleMapMaterialized` is the caller's responsibility
/// so the shadow path can emit it once for the whole render rather than
/// twice.
///
/// `start_step_id` is the first step_id assigned; the helper increments
/// internally and the resulting state vectors carry the assigned ids.
/// Pass `1` for the no-shadow path; the shadow path passes `2` because
/// step_id 1 is reserved for the Shadow stage emitted at the function
/// entry.
pub(crate) fn emit_per_stage_entered(
    inspector: &mut dyn CompositorInspector,
    start_step_id: u32,
    inputs: &PerStageInputs<'_>,
) -> PerStageBlock {
    let mut next_step_id = start_step_id;
    let mut simple_states: Vec<SimpleStageState> = Vec::new();
    let mut shader_states: Vec<ShaderObservabilityState> =
        Vec::with_capacity(inputs.shader_layers_with_regions.len());
    let mut skipped_shader_indices: Vec<usize> = Vec::new();

    if let Some(label) = inputs.sampler_label {
        simple_states.push(emit_simple_stage_entered(
            inspector,
            PipelineStageKind::Sampler,
            next_step_id,
            label,
            inputs.source_roles,
            inputs.width,
            inputs.height,
        ));
        next_step_id += 1;
    }
    for label in inputs.mask_labels {
        simple_states.push(emit_simple_stage_entered(
            inspector,
            PipelineStageKind::Mask,
            next_step_id,
            label,
            inputs.source_roles,
            inputs.width,
            inputs.height,
        ));
        next_step_id += 1;
    }
    for (shader_index, (layer, resolved)) in inputs.shader_layers_with_regions.iter().enumerate() {
        let state = emit_shader_entered_or_skipped(
            inspector,
            next_step_id,
            layer,
            resolved,
            inputs.source_roles,
            inputs.width,
            inputs.height,
            shader_index,
        );
        next_step_id += 1;
        if state.skipped {
            skipped_shader_indices.push(shader_index);
        }
        shader_states.push(state);
    }
    for label in inputs.filter_labels {
        simple_states.push(emit_simple_stage_entered(
            inspector,
            PipelineStageKind::Filter,
            next_step_id,
            label,
            inputs.source_roles,
            inputs.width,
            inputs.height,
        ));
        next_step_id += 1;
    }

    PerStageBlock {
        simple_states,
        shader_states,
        skipped_shader_indices,
    }
}

/// Emit the matching post-loop `StageFinished` for every entry in
/// [`PerStageBlock`]. Skipped shaders short-circuit; non-skipped stages
/// emit their `StageFinished` in the same order they were entered.
pub(crate) fn emit_per_stage_finished(
    inspector: &mut dyn CompositorInspector,
    block: &PerStageBlock,
) {
    for state in &block.simple_states {
        emit_simple_stage_finished(inspector, state);
    }
    for state in &block.shader_states {
        emit_shader_finished(inspector, state);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::RoleMap;

    #[test]
    fn area_histogram_counts_background() {
        let roles = RoleMap::all_background(4, 4);
        let hist = tally_role_histogram_for_area(&roles, 4, 4);
        assert_eq!(hist.background, 16);
        assert_eq!(hist.text, 0);
    }

    #[test]
    fn area_histogram_skips_unbucketed_roles() {
        let mut roles = RoleMap::all_background(2, 2);
        roles.set((0, 0), RoleTag::Procedural); // not in the 5-bucket histogram
        let hist = tally_role_histogram_for_area(&roles, 2, 2);
        assert_eq!(hist.background, 3);
        assert_eq!(hist.text, 0);
        assert_eq!(hist.total(), 3); // Procedural cell is not counted
    }

    #[test]
    fn role_text_scope_against_all_background_matches_zero() {
        let region = StyleRegion::Role(RoleTag::Text);
        let roles = RoleMap::all_background(8, 4);
        let (matched, skipped, hist) = tally_shader_scope_match(&region, &roles, 8, 4);
        assert_eq!(matched, 0);
        assert_eq!(skipped, 32);
        assert_eq!(hist.background, 32);
        assert_eq!(hist.text, 0);
    }

    #[test]
    fn all_scope_matches_every_cell() {
        let region = StyleRegion::All;
        let roles = RoleMap::all_background(3, 3);
        let (matched, skipped, _) = tally_shader_scope_match(&region, &roles, 3, 3);
        assert_eq!(matched, 9);
        assert_eq!(skipped, 0);
    }

    #[test]
    fn style_region_summary_for_role_is_grep_friendly() {
        let s = style_region_summary(&StyleRegion::Role(RoleTag::Text));
        assert!(s.contains("Role"));
        assert!(s.contains("Text"));
    }
}

// <FILE>crates/tui-vfx-compositor/src/pipeline/orc_pipeline_observability.rs</FILE> - <DESC>Per-stage observability emit helpers</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
