// <FILE>crates/tui-vfx-compositor-next/tests/test_shadow_path_emits_shader_scope_skip.rs</FILE> - <DESC>Shadow path's element pass emits StageSkipped { ScopeMatchedZeroCells } for a Role(Text) shader on an all-Background role map — closes architect-flagged B2</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Pipeline observability Unit A — architect B2: render_pipeline_with_shadow's element pass was previously observability-blind for Sampler/Mask/Shader/Filter stages, so a shadowed recipe with a scope-mismatched shader rendered zero cells silently. This regression test installs a TraceSink over a shadowed render with a Role(Text) shader on an all-Background role map and asserts the same StageSkipped/ScopeEvaluated evidence the no-shadow path emits.</WCTX>
// <CLOG>0.1.0: configure a shadow + Role(Text) shader on all-Background, install InspectionSinkBridge, assert StageSkipped { kind: Shader, ScopeMatchedZeroCells } and ScopeEvaluated { matched: 0 } both fire even though the shadow path runs a different element-render loop than render_loop_inspected.</CLOG>

//! Regression test for the shadow path's per-stage observability gap.
//!
//! Before architect B2 was closed, `render_pipeline_with_shadow` ran its
//! own element-pass loop that called `apply_shaders(...)` and the filter
//! chain without emitting per-stage `StageEntered` / `StageFinished` /
//! `ScopeEvaluated` events. A shadowed recipe with a `Role(Text)` shader
//! on a producer that tagged every cell as `Background` therefore
//! rendered zero shader changes silently — the very bug class US-008
//! was supposed to make mechanically detectable, except the shadow path
//! re-introduced it.
//!
//! After B2: the shadow path's element pass calls
//! `emit_per_stage_entered` (start_step_id=2 because step_id=1 is the
//! Shadow stage) and the matching `emit_per_stage_finished` after the
//! mask + dest write loop completes. The Shader StageSkipped event
//! fires through the same surface the no-shadow path uses.

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
use tui_vfx_compositor_next::types::ShadowSpec;
use tui_vfx_compositor_next::types::cls_filter_spec::FilterSpec;
use tui_vfx_compositor_next::types::cls_mask_spec::MaskSpec;
use tui_vfx_compositor_next::types::cls_sampler_spec::SamplerSpec;
use tui_vfx_debug::inspection::{
    PipelineSkipReason, PipelineStageKind, TraceEvent, TraceFilter, TraceSink,
};
use tui_vfx_shadow::ShadowConfig;
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

fn shadow_options_with_text_scoped_shader<'a>(
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
            region: StyleRegion::Role(RoleTag::Text),
            v3_family: Some(VfxSpatialShaderFamily::ComposedPrimitive(
                VfxSpatialComposedPrimitive::TravelingBand(VfxTravelingBandShader::from(shader)),
            )),
            shader_label: Some("FocusedRowGradient".to_string()),
        }]),
        shadow: Some(ShadowSpec::new(ShadowConfig::new(Color::rgb(0, 0, 0)))),
        shadow_element_rect: None,
        preserve_unfilled: true,
        t: 0.0,
        loop_t: None,
        phase: None,
        runtime_params: Arc::default(),
    }
}

#[test]
fn shadow_path_role_text_on_all_background_emits_stage_skipped_for_shader() {
    // ── Arrange ────────────────────────────────────────────────────────
    let source = small_source();
    let source_roles = RoleMap::all_background(WIDTH as u16, HEIGHT as u16);
    let mut dest = SemanticScene::from_grid_with_default_role(
        OwnedGrid::new(WIDTH + 4, HEIGHT + 4),
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
    let options = shadow_options_with_text_scoped_shader(&shader);

    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx =
        TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("focused_row_btop_with_shadow"));
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

    // The shadow path emits exactly one RoleMapMaterialized at function
    // entry (no double-emit even though the element-pass per-stage block
    // also runs).
    let role_map_count = report
        .envelopes
        .iter()
        .filter(|env| matches!(env.event, TraceEvent::RoleMapMaterialized { .. }))
        .count();
    assert_eq!(
        role_map_count, 1,
        "shadow path with shader should emit exactly one RoleMapMaterialized; got {role_map_count}",
    );

    // ScopeEvaluated for the shader stage with matched=0 confirms the
    // pre-loop scope tally fired even though the element pass diverges
    // from render_loop_inspected.
    let shader_scope_eval = report
        .envelopes
        .iter()
        .map(|env| &env.event)
        .find(|e| matches!(e, TraceEvent::ScopeEvaluated { matched: 0, .. }))
        .unwrap_or_else(|| {
            panic!(
                "expected shadow-path ScopeEvaluated with matched=0; got: {:#?}",
                report.envelopes
            )
        });
    if let TraceEvent::ScopeEvaluated {
        matched,
        skipped,
        role_histogram,
        ..
    } = shader_scope_eval
    {
        assert_eq!(*matched, 0);
        assert_eq!(
            *matched + *skipped,
            (WIDTH * HEIGHT) as u32,
            "matched + skipped must equal area cell count",
        );
        assert_eq!(role_histogram.background, (WIDTH * HEIGHT) as u32);
        assert_eq!(role_histogram.text, 0);
    }

    // The load-bearing event: Shader StageSkipped with the predicate
    // summary and zero-text histogram. The shadow path now fires this
    // through the same surface render_loop_inspected uses.
    let shader_skipped = report
        .envelopes
        .iter()
        .map(|env| &env.event)
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
                "shadow path must emit Shader StageSkipped {{ ScopeMatchedZeroCells }}; got: {:#?}",
                report.envelopes
            )
        });
    if let TraceEvent::StageSkipped {
        reason:
            PipelineSkipReason::ScopeMatchedZeroCells {
                predicate,
                role_histogram,
            },
        ..
    } = shader_skipped
    {
        assert!(
            predicate.contains("Role") && predicate.contains("Text"),
            "predicate summary should mention Role and Text, got {predicate:?}",
        );
        assert!(role_histogram.background > 0);
        assert_eq!(role_histogram.text, 0);
    }

    // No StageEntered for the skipped shader stage (B1 strict-skip
    // semantics — applied by emit_shader_entered_or_skipped). The shadow
    // path re-uses the same helper and inherits the strict ordering.
    let shader_entered_count = report
        .envelopes
        .iter()
        .filter(|env| {
            matches!(
                env.event,
                TraceEvent::StageEntered {
                    kind: PipelineStageKind::Shader,
                    ..
                }
            )
        })
        .count();
    assert_eq!(
        shader_entered_count, 0,
        "skipped shaders must not emit StageEntered (PRD US-006 strict-skip semantics)",
    );
}

// <FILE>crates/tui-vfx-compositor-next/tests/test_shadow_path_emits_shader_scope_skip.rs</FILE> - <DESC>Shadow path shader scope-skip regression test</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
