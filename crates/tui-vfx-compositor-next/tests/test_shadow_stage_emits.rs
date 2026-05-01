// <FILE>crates/tui-vfx-compositor-next/tests/test_shadow_stage_emits.rs</FILE> - <DESC>Shadow path emits a Shadow StageEntered/StageFinished pair around the shadow buffer render</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — closes architect-flagged C1: render_pipeline_with_shadow now emits Shadow stage events so the shadow path is observable through the same per-stage event surface as the no-shadow path.</WCTX>
// <CLOG>0.1.0: configure a recipe with shadow, install InspectionSinkBridge, assert StageEntered { kind: Shadow } and StageFinished { kind: Shadow } both fire and pair on step_id.</CLOG>

//! Verifies that the shadow render path emits per-stage Shadow events.
//!
//! Before Pipeline observability Unit A landed the Shadow stage was
//! observability-blind to the new per-stage taxonomy: the architect
//! review (C1) flagged that `render_pipeline_with_shadow` did not call
//! the new pre-stage / post-stage emit helpers. This test guards against
//! that regression — install a `TraceSink` over a shadow-configured
//! render and assert exactly one `Shadow` `StageEntered` / `StageFinished`
//! pair lands, plus one `RoleMapMaterialized`.

use std::sync::Arc;

use tui_vfx_compositor_next::pipeline::cls_composition_options::CompositionOptions;
use tui_vfx_compositor_next::pipeline::orc_render_pipeline::render_pipeline;
use tui_vfx_compositor_next::traits::cls_inspection_sink_bridge::{
    InspectionSinkBridge, TraceFrameContext,
};
use tui_vfx_compositor_next::types::ShadowSpec;
use tui_vfx_debug::inspection::{PipelineStageKind, TraceEvent, TraceFilter, TraceSink};
use tui_vfx_shadow::ShadowConfig;
use tui_vfx_types::{Color, OwnedGrid, RecipeId, RoleMap, RoleTag, SemanticScene};

#[test]
fn shadow_path_emits_stage_entered_finished_pair_and_role_map_materialized() {
    // ── Arrange ────────────────────────────────────────────────────────
    let source = OwnedGrid::new(10, 10);
    let source_roles = RoleMap::all_background(10, 10);
    let mut scene =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(14, 14), RoleTag::Background);
    let shadow_config = ShadowConfig::new(Color::rgb(0, 0, 0));
    let shadow_spec = ShadowSpec::new(shadow_config);
    let options = CompositionOptions::default().with_shadow(shadow_spec);

    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx = TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("shadow.smoke"));
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);

    // ── Act ────────────────────────────────────────────────────────────
    render_pipeline(
        &source,
        &source_roles,
        &mut scene,
        10,
        10,
        0,
        0,
        options,
        Some(&mut bridge),
    );

    // ── Assert ─────────────────────────────────────────────────────────
    let report = sink.snapshot();

    // Exactly one RoleMapMaterialized fires for the shadow path.
    let role_map_count = report
        .envelopes
        .iter()
        .filter(|env| matches!(env.event, TraceEvent::RoleMapMaterialized { .. }))
        .count();
    assert_eq!(
        role_map_count, 1,
        "shadow path should emit exactly one RoleMapMaterialized; got {role_map_count}",
    );

    // One Shadow StageEntered.
    let shadow_entered: Vec<&TraceEvent> = report
        .envelopes
        .iter()
        .map(|env| &env.event)
        .filter(|e| {
            matches!(
                e,
                TraceEvent::StageEntered {
                    kind: PipelineStageKind::Shadow,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(
        shadow_entered.len(),
        1,
        "expected exactly one Shadow StageEntered, got {}: {:#?}",
        shadow_entered.len(),
        shadow_entered
    );

    // One Shadow StageFinished, with matching step_id.
    let entered_step_id = match shadow_entered[0] {
        TraceEvent::StageEntered { step_id, .. } => *step_id,
        _ => unreachable!(),
    };
    let shadow_finished: Vec<&TraceEvent> = report
        .envelopes
        .iter()
        .map(|env| &env.event)
        .filter(|e| {
            matches!(
                e,
                TraceEvent::StageFinished {
                    kind: PipelineStageKind::Shadow,
                    ..
                }
            )
        })
        .collect();
    assert_eq!(
        shadow_finished.len(),
        1,
        "expected exactly one Shadow StageFinished",
    );
    match shadow_finished[0] {
        TraceEvent::StageFinished { step_id, .. } => {
            assert_eq!(
                *step_id, entered_step_id,
                "StageFinished must pair with StageEntered on the same step_id",
            );
        }
        _ => unreachable!(),
    }
}

// <FILE>crates/tui-vfx-compositor-next/tests/test_shadow_stage_emits.rs</FILE> - <DESC>Shadow path per-stage emit test</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
