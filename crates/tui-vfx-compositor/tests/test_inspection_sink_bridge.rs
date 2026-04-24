// <FILE>crates/tui-vfx-compositor/tests/test_inspection_sink_bridge.rs</FILE> - <DESC>Integration test: installing an InspectionSinkBridge on render_pipeline produces TraceEvents; existing CompositorInspector impls keep working</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — bridge regression test. Drives render_pipeline once with InspectionSinkBridge and once with a hand-rolled CompositorInspector; asserts both paths reach the same per-cell stages and the bridge produces a non-empty TraceReport with the expected variants.</WCTX>
// <CLOG>0.1.0: initial red-phase bridge integration test.</CLOG>

use std::sync::Arc;

use smallvec::SmallVec;
use std::borrow::Cow;
use tui_vfx_compositor::pipeline::cls_composition_options::CompositionOptions;
use tui_vfx_compositor::pipeline::orc_render_pipeline::render_pipeline;
use tui_vfx_compositor::traits::cls_inspection_sink_bridge::{
    InspectionSinkBridge, TraceFrameContext,
};
use tui_vfx_compositor::traits::pipeline_inspector::CompositorInspector;
use tui_vfx_compositor::types::cls_filter_spec::FilterSpec;
use tui_vfx_compositor::types::cls_mask_spec::MaskSpec;
use tui_vfx_compositor::types::cls_sampler_spec::SamplerSpec;
use tui_vfx_debug::inspection::{StageMask, TraceEvent, TraceFilter, TraceSelector, TraceSink};
use tui_vfx_types::{
    Cell, Color, Grid, Modifiers, OwnedGrid, RecipeId, RoleMap, RoleTag, SemanticScene, Style,
};

/// Minimal hand-rolled inspector that counts each kind of callback.
#[derive(Default)]
struct CountingInspector {
    sampler: u32,
    mask: u32,
    shader: u32,
    filter: u32,
    shadow: u32,
    cells: u32,
}

impl CompositorInspector for CountingInspector {
    fn on_sampler_applied(
        &mut self,
        _dest_x: u16,
        _dest_y: u16,
        _src_x: Option<u16>,
        _src_y: Option<u16>,
        _sampler_name: &str,
    ) {
        self.sampler += 1;
    }
    fn on_mask_checked(&mut self, _x: u16, _y: u16, _visible: bool, _mask_name: &str) {
        self.mask += 1;
    }
    fn on_shader_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: Style,
        _after: Style,
        _shader_name: &str,
    ) {
        self.shader += 1;
    }
    fn on_filter_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: &Cell,
        _after: &Cell,
        _filter_name: &str,
    ) {
        self.filter += 1;
    }
    fn on_shadow_cell_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _shadow_cell: &Cell,
        _source_empty: bool,
    ) {
        self.shadow += 1;
    }
    fn on_cell_rendered(&mut self, _x: u16, _y: u16, _final_cell: &Cell) {
        self.cells += 1;
    }
}

fn styled_cell(ch: char) -> Cell {
    Cell::styled(
        ch,
        Color::new(255, 255, 255, 255),
        Color::new(0, 0, 0, 255),
        Modifiers::NONE,
    )
}

fn small_source(width: usize, height: usize) -> OwnedGrid {
    let mut grid = OwnedGrid::new(width, height);
    for y in 0..height {
        for x in 0..width {
            grid.set(x, y, styled_cell('#'));
        }
    }
    grid
}

fn minimal_options<'a>() -> CompositionOptions<'a> {
    // No sampler, no mask, no filter, no shader, no shadow — this forces
    // the inspected render_loop path (which fires per-cell callbacks).
    CompositionOptions {
        sampler_spec: None,
        samplers: Cow::Borrowed(&[] as &[SamplerSpec]),
        masks: Cow::Borrowed(&[] as &[MaskSpec]),
        mask_combine_mode: Default::default(),
        filters: Cow::Borrowed(&[] as &[FilterSpec]),
        shader_layers: SmallVec::new(),
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
fn bridge_produces_trace_events_for_every_rendered_cell() {
    let source = small_source(4, 3);
    let source_roles = RoleMap::all_background(4, 3);
    let mut dest =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(8, 4), RoleTag::Background);

    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx = TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("bridge.test"));
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);

    render_pipeline(
        &source,
        &source_roles,
        &mut dest,
        4,
        3,
        1,
        0,
        minimal_options(),
        Some(&mut bridge),
    );

    let report = sink.snapshot();
    assert!(
        !report.envelopes.is_empty(),
        "bridge should have forwarded at least one envelope"
    );

    // Every pipeline envelope must belong to the pipeline stage.
    for envelope in &report.envelopes {
        assert!(
            envelope.event.stage().contains(StageMask::PIPELINE),
            "bridge-produced event must be PIPELINE stage: {:?}",
            envelope.event,
        );
    }

    // Confirm we at least saw CellRendered events — the pipeline always
    // writes a cell for this minimal configuration.
    let cell_rendered_count = report
        .envelopes
        .iter()
        .filter(|e| matches!(e.event, TraceEvent::CellRendered { .. }))
        .count();
    assert_eq!(
        cell_rendered_count,
        4 * 3,
        "expected 12 CellRendered events (4x3 source)"
    );

    // Sequence numbers must be monotonically increasing within the frame.
    let mut prev = None;
    for envelope in &report.envelopes {
        assert_eq!(envelope.frame_no, 0);
        if let Some(prev_seq) = prev {
            assert!(
                envelope.seq_in_frame > prev_seq,
                "seq_in_frame must be monotonic: {} > {}",
                envelope.seq_in_frame,
                prev_seq,
            );
        }
        prev = Some(envelope.seq_in_frame);
    }
}

#[test]
fn existing_compositor_inspector_impls_keep_working() {
    let source = small_source(4, 3);
    let source_roles = RoleMap::all_background(4, 3);
    let mut dest =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(8, 4), RoleTag::Background);

    let mut inspector = CountingInspector::default();
    render_pipeline(
        &source,
        &source_roles,
        &mut dest,
        4,
        3,
        1,
        0,
        minimal_options(),
        Some(&mut inspector),
    );

    // The minimal config has no mask, no shader, no filter, no shadow:
    // the inspector should only observe sampler + cell callbacks.
    assert_eq!(
        inspector.cells,
        4 * 3,
        "every cell reports on_cell_rendered"
    );
    // Sampler fires on every (x, y) pair even in the minimal config
    // (inspected path always calls on_sampler_applied).
    assert_eq!(
        inspector.sampler,
        4 * 3,
        "every cell reports on_sampler_applied"
    );
    assert_eq!(inspector.mask, 0);
    assert_eq!(inspector.shader, 0);
    assert_eq!(inspector.filter, 0);
    assert_eq!(inspector.shadow, 0);
}

#[test]
fn bridge_short_circuits_with_none_stage_filter() {
    let source = small_source(4, 3);
    let source_roles = RoleMap::all_background(4, 3);
    let mut dest =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(8, 4), RoleTag::Background);

    // Filter that only accepts LIFECYCLE events — PIPELINE is masked off.
    let filter = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::LIFECYCLE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    let sink = Arc::new(TraceSink::new(filter));
    let ctx = TraceFrameContext::new(0, 0);
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);

    render_pipeline(
        &source,
        &source_roles,
        &mut dest,
        4,
        3,
        1,
        0,
        minimal_options(),
        Some(&mut bridge),
    );

    let report = sink.snapshot();
    assert_eq!(
        report.envelopes.len(),
        0,
        "PIPELINE-masked-off sink must receive zero envelopes via the bridge"
    );
}

// <FILE>crates/tui-vfx-compositor/tests/test_inspection_sink_bridge.rs</FILE> - <DESC>Bridge integration test</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>

#[test]
fn bridge_reports_v3_family_in_shader_label_when_available() {
    use tui_vfx_compositor::pipeline::cls_composition_options::ShaderWithRegion;
    use tui_vfx_style::models::{
        BorderSweepShader, ColorConfig, StyleRegion, VfxSpatialComposedPrimitive,
        VfxSpatialShaderFamily, VfxTravelingBandShader,
    };

    let source = small_source(4, 3);
    let source_roles = RoleMap::all_background(4, 3);
    let mut dest =
        SemanticScene::from_grid_with_default_role(OwnedGrid::new(8, 4), RoleTag::Background);

    let sink = Arc::new(TraceSink::new(TraceFilter::accept_all()));
    let ctx = TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("bridge.shader.family"));
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), ctx);

    let shader = BorderSweepShader {
        speed: 1.0,
        length: 3,
        color: ColorConfig::Red,
        head: None,
        tail: None,
        position_binding: None,
    };
    let options = CompositionOptions {
        shader_layers: SmallVec::from_vec(vec![ShaderWithRegion {
            shader: &shader,
            region: StyleRegion::All,
            v3_family: Some(VfxSpatialShaderFamily::ComposedPrimitive(
                VfxSpatialComposedPrimitive::TravelingBand(VfxTravelingBandShader::from(&shader)),
            )),
            shader_label: Some("BorderSweep".to_string()),
        }]),
        ..minimal_options()
    };

    render_pipeline(
        &source,
        &source_roles,
        &mut dest,
        4,
        3,
        0,
        0,
        options,
        Some(&mut bridge),
    );

    let report = sink.snapshot();
    let shader_names: Vec<String> = report
        .envelopes
        .iter()
        .filter_map(|envelope| match &envelope.event {
            TraceEvent::ShaderApplied { shader, .. } => Some(shader.clone()),
            _ => None,
        })
        .collect();

    assert!(
        shader_names
            .iter()
            .any(|name| name.starts_with("traveling_band:BorderSweep#")),
        "expected at least one shader event label to include the grouped V3 family, got {shader_names:?}"
    );
}
