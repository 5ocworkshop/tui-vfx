// <FILE>crates/tui-vfx-compositor-next/tests/test_scope_mismatch_emits_zero_cell_skip.rs</FILE> - <DESC>Regression test for the focused_row_btop bug class — a Role(Text) scope on an all-Background RoleMap must emit StageSkipped { ScopeMatchedZeroCells } with an honest role histogram</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — mechanical regression guard for the 2026-04-26 case study where a recipe with Role(Text) scope on a producer that tagged every cell as Background produced silent zero changes; the new per-stage events must surface this scope mismatch in one assertion.</WCTX>
// <CLOG>0.1.0: render a Role(Text) shader against an all-Background RoleMap, install an InspectionSinkBridge, assert StageSkipped/ScopeMatchedZeroCells fires with histogram { background > 0, text == 0 } and ScopeEvaluated reports matched=0.</CLOG>

//! Regression test for the focused_row_btop bug class.
//!
//! The case study (2026-04-26): a `focused_row_gradient` shader recipe
//! with `Scope::Role(Text)` was applied to a `ContentShell::card`
//! producer that tagged every cell as `RoleTag::Background`. The
//! shader's scope predicate matched zero cells and produced zero changes
//! to the surface. The trace was silent — no event named the
//! mismatch — and the investigation took 30+ minutes.
//!
//! After Pipeline observability Unit A, the same scenario must emit:
//! - one `RoleMapMaterialized` event with the area histogram,
//! - one `StageEntered { kind: Shader, scope_summary: "Role(Text)" }`,
//! - one `ScopeEvaluated { matched: 0, skipped: <area>, role_histogram: { background: <area>, text: 0 } }`,
//! - one `StageSkipped { reason: ScopeMatchedZeroCells { predicate: "Role(Text)", role_histogram: { background: <area>, text: 0 } } }`.
//!
//! This test asserts those events are present and carry the expected payload.

use std::borrow::Cow;
use std::sync::Arc;

use smallvec::SmallVec;
use tui_vfx_compositor_next::pipeline::cls_composition_options::{
    CompositionOptions, ShaderWithRegion,
};
use tui_vfx_compositor_next::pipeline::orc_render_pipeline::render_pipeline;
use tui_vfx_compositor_next::traits::cls_inspection_sink_bridge::{
    InspectionSinkBridge, TraceFrameContext,
};
use tui_vfx_compositor_next::types::cls_filter_spec::FilterSpec;
use tui_vfx_compositor_next::types::cls_mask_spec::MaskSpec;
use tui_vfx_compositor_next::types::cls_sampler_spec::SamplerSpec;
use tui_vfx_debug::inspection::{
    PipelineSkipReason, PipelineStageKind, RoleMapSource, TraceEvent, TraceFilter, TraceSink,
};
use tui_vfx_style::models::{
    BorderSweepShader, ColorConfig, StyleRegion, VfxSpatialComposedPrimitive,
    VfxSpatialShaderFamily, VfxTravelingBandShader,
};
use tui_vfx_types::{Cell, Color, Modifiers, OwnedGrid, RecipeId, RoleMap, RoleTag, SemanticScene};

const WIDTH: usize = 8;
const HEIGHT: usize = 4;

fn styled_cell(ch: char) -> Cell {
    Cell::styled(
        ch,
        Color::new(255, 255, 255, 255),
        Color::new(0, 0, 0, 255),
        Modifiers::NONE,
    )
}

fn small_source() -> OwnedGrid {
    let mut grid = OwnedGrid::new(WIDTH, HEIGHT);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            tui_vfx_types::Grid::set(&mut grid, x, y, styled_cell('#'));
        }
    }
    grid
}

fn minimal_options_with_text_scoped_shader<'a>(
    shader: &'a BorderSweepShader,
) -> CompositionOptions<'a> {
    CompositionOptions {
        sampler_spec: None,
        samplers: Cow::Borrowed(&[] as &[SamplerSpec]),
        masks: Cow::Borrowed(&[] as &[MaskSpec]),
        mask_combine_mode: Default::default(),
        filters: Cow::Borrowed(&[] as &[FilterSpec]),
        shader_layers: SmallVec::from_vec(vec![ShaderWithRegion {
            shader,
            // The case study scope: target text cells only. The producer (below)
            // tags every cell as Background, so the predicate matches zero cells.
            region: StyleRegion::Role(RoleTag::Text),
            v3_family: Some(VfxSpatialShaderFamily::ComposedPrimitive(
                VfxSpatialComposedPrimitive::TravelingBand(VfxTravelingBandShader::from(shader)),
            )),
            shader_label: Some("FocusedRowGradient".to_string()),
        }]),
        shadow: None,
        shadow_element_rect: None,
        preserve_unfilled: true,
        t: 0.0,
        loop_t: None,
        phase: None,
        runtime_params: Arc::default(),
    }
}

#[test]
fn role_text_scope_on_all_background_role_map_emits_zero_cell_skip() {
    // ── Arrange ────────────────────────────────────────────────────────
    let source = small_source();
    // The producer tags every cell as Background — the focused_row_btop
    // case study's central condition.
    let source_roles = RoleMap::all_background(WIDTH as u16, HEIGHT as u16);
    let mut dest = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(WIDTH, HEIGHT),
        RoleTag::Background,
    );

    let shader = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        head: None,
        tail: None,
        position_binding: None,
    };
    let options = minimal_options_with_text_scoped_shader(&shader);

    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx = TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("focused_row_btop"));
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);

    // ── Act ────────────────────────────────────────────────────────────
    render_pipeline(
        &source,
        &source_roles,
        &mut dest,
        WIDTH,
        HEIGHT,
        0,
        0,
        options,
        Some(&mut bridge),
    );

    // ── Assert ─────────────────────────────────────────────────────────
    let report = sink.snapshot();

    // 1. RoleMapMaterialized fired with an honest histogram.
    let role_map_event = report
        .envelopes
        .iter()
        .map(|e| &e.event)
        .find(|e| matches!(e, TraceEvent::RoleMapMaterialized { .. }))
        .unwrap_or_else(|| {
            panic!(
                "expected one RoleMapMaterialized event, got: {:#?}",
                report.envelopes
            )
        });
    match role_map_event {
        TraceEvent::RoleMapMaterialized { source, histogram } => {
            assert!(
                matches!(source, RoleMapSource::Injected),
                "Unit A always reports RoleMapSource::Injected (gt-design upgrades to ExplicitFromProducer in Unit B); got {source:?}"
            );
            assert_eq!(
                histogram.background,
                (WIDTH * HEIGHT) as u32,
                "all-background role map should histogram every cell as background",
            );
            assert_eq!(histogram.text, 0, "no text cells in this scenario");
        }
        _ => unreachable!(),
    }

    // 2. ScopeEvaluated with matched == 0 fired for the shader stage.
    let scope_evaluated = report
        .envelopes
        .iter()
        .map(|e| &e.event)
        .find(|e| matches!(e, TraceEvent::ScopeEvaluated { matched: 0, .. }))
        .unwrap_or_else(|| {
            panic!(
                "expected ScopeEvaluated with matched=0 (Role(Text) on all-Background), got: {:#?}",
                report.envelopes
            )
        });
    match scope_evaluated {
        TraceEvent::ScopeEvaluated {
            matched,
            skipped,
            role_histogram,
            ..
        } => {
            assert_eq!(*matched, 0);
            assert_eq!(
                *matched + *skipped,
                (WIDTH * HEIGHT) as u32,
                "matched + skipped should equal area cell count",
            );
            assert_eq!(role_histogram.background, (WIDTH * HEIGHT) as u32);
            assert_eq!(role_histogram.text, 0);
        }
        _ => unreachable!(),
    }

    // 3. The load-bearing event: StageSkipped with ScopeMatchedZeroCells,
    //    naming the predicate and showing the role histogram.
    let stage_skipped = report
        .envelopes
        .iter()
        .map(|e| &e.event)
        .find(|e| {
            matches!(
                e,
                TraceEvent::StageSkipped {
                    kind: PipelineStageKind::Shader,
                    reason: PipelineSkipReason::ScopeMatchedZeroCells { .. },
                    ..
                }
            )
        })
        .unwrap_or_else(|| {
            panic!(
                "expected Shader StageSkipped with ScopeMatchedZeroCells, got: {:#?}",
                report.envelopes
            )
        });
    match stage_skipped {
        TraceEvent::StageSkipped {
            kind,
            reason:
                PipelineSkipReason::ScopeMatchedZeroCells {
                    predicate,
                    role_histogram,
                },
            ..
        } => {
            assert_eq!(*kind, PipelineStageKind::Shader);
            assert!(
                predicate.contains("Role") && predicate.contains("Text"),
                "predicate summary should mention Role and Text, got {predicate:?}",
            );
            assert!(
                role_histogram.background > 0,
                "histogram should show background cells > 0 (got {})",
                role_histogram.background
            );
            assert_eq!(
                role_histogram.text, 0,
                "histogram must show text == 0 (the bug condition)",
            );
        }
        _ => unreachable!(),
    }
}

// <FILE>crates/tui-vfx-compositor-next/tests/test_scope_mismatch_emits_zero_cell_skip.rs</FILE> - <DESC>Regression test for focused_row_btop scope mismatch</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
