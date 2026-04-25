// <FILE>crates/tui-vfx/tests/test_foundation_end_to_end.rs</FILE> - <DESC>End-to-end integration test proving the Sub-plan A foundation (A.1 primitives + A.2 pipeline + A.3 shadow role write-back + A.4 trace sink) composes cleanly</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>Audit Phase 7 prep — restore the cargo-test build after Phase 2 added `apply_to` and `intensity` to LinearGradientShader without updating this test fixture's struct literal.</WCTX>
// <CLOG>Pass back-compat `apply_to: Foreground` and `intensity: 1.0` to LinearGradientShader so the test compiles after Phase 2 extended the struct.</CLOG>

//! End-to-end integration test for Sub-plan A (Foundation).
//!
//! This single test exercises every foundation primitive shipped by
//! Phases A.1 through A.4:
//!
//! | Phase | What is exercised |
//! |---|---|
//! | A.1 | `SemanticScene::new`, `RoleMap::set`, 4 different `RoleTag` variants |
//! | A.2 | `render_pipeline` consumes `&RoleMap` + `&mut SemanticScene`; `StyleRegion::Role(Border)` targets only border-tagged cells |
//! | A.3 | `ShadowConfig::with_source_region(Border)` filters extrusion; destination role map gains `RoleTag::Shadow` where shadow cells land |
//! | A.4 | `InspectionSinkBridge` forwards pipeline callbacks as `TraceEvent`s into a `TraceSink`; `TraceReport::to_ndjson` / `from_ndjson` round-trip preserves ordering |
//!
//! A regression in ANY phase should surface here; this is the
//! cross-phase safety net before Sub-plan B lands emit sites on the
//! recipes side.

use std::io::Cursor;
use std::sync::Arc;

use tui_vfx_compositor::pipeline::{CompositionOptions, render_pipeline};
use tui_vfx_compositor::traits::cls_inspection_sink_bridge::{
    InspectionSinkBridge, TraceFrameContext,
};
use tui_vfx_debug::inspection::{
    StageMask, TraceEvent, TraceFilter, TraceReport, TraceSelector, TraceSink,
};
use tui_vfx_style::models::{
    ColorSpace, Gradient, LinearGradientApplyTo, LinearGradientShader, StyleRegion,
};
use tui_vfx_types::{Cell, Color, Grid, OwnedGrid, RecipeId, RoleMap, RoleTag, SemanticScene};

/// Build a 10×6 source grid and paired role map used by the test.
///
/// Layout:
///
/// ```text
/// row 0 (Border):     ############    ← all 'X', role=Border
/// row 1 (Text):       ############    ← all 'X', role=Text
/// row 2 (Image):      ############    ← all 'X', role=Image
/// row 3 (Text):       ############    ← all 'X', role=Text
/// row 4 (Text):       ############    ← all 'X', role=Text
/// row 5 (Border):     ############    ← all 'X', role=Border
/// ```
///
/// Non-trivial: four distinct role variants (Background is the default
/// in `RoleMap::empty`, and Border/Text/Image are explicitly set).
fn build_scene() -> (OwnedGrid, RoleMap) {
    const W: u16 = 10;
    const H: u16 = 6;
    let mut grid = OwnedGrid::new(W as usize, H as usize);
    for y in 0..H as usize {
        for x in 0..W as usize {
            grid.set(
                x,
                y,
                Cell {
                    ch: 'X',
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                    ..Default::default()
                },
            );
        }
    }
    let mut roles = RoleMap::all_background(W, H);
    for x in 0..W {
        roles.set((x, 0), RoleTag::Border);
        roles.set((x, 1), RoleTag::Text);
        roles.set((x, 2), RoleTag::Image);
        roles.set((x, 3), RoleTag::Text);
        roles.set((x, 4), RoleTag::Text);
        roles.set((x, 5), RoleTag::Border);
    }
    (grid, roles)
}

fn shader_blue() -> LinearGradientShader {
    // Solid blue over the full range so "painted by shader" is the
    // simple "fg changed from white" predicate.
    LinearGradientShader {
        gradient: Gradient {
            stops: vec![
                (0.0, Color::rgb(20, 120, 220)),
                (1.0, Color::rgb(20, 120, 220)),
            ],
            space: ColorSpace::Rgb,
        },
        angle_deg: 0.0,
        apply_to: LinearGradientApplyTo::Foreground,
        intensity: 1.0,
    }
}

/// The one end-to-end test.
///
/// Runs the pipeline with:
/// - a Border-targeted shader (A.2 role-aware targeting),
/// - a shadow whose source_region is Border (A.3 role-filtered
///   extrusion + destination role write-back),
/// - a trace sink wired via the InspectionSinkBridge (A.4),
///
/// Then asserts:
/// (a) destination RoleMap has RoleTag::Shadow cells where expected,
/// (b) captured events carry the expected stage sequence,
/// (c) NDJSON round-trip preserves envelope ordering.
#[test]
fn foundation_end_to_end_proves_all_phases_compose() {
    // ---- Arrange ----------------------------------------------------
    let (source, source_roles) = build_scene();
    let src_w = source_roles.width();
    let src_h = source_roles.height();

    // Destination surface: larger than the source so the shadow has
    // somewhere to extrude into (shadow offset is (+2, +1)).
    let dest_grid = OwnedGrid::new(14, 9);
    let mut dest_scene = SemanticScene::from_grid_with_default_role(dest_grid, RoleTag::Background);

    // Border-targeted shader: should paint only rows 0 and 5.
    let shader = shader_blue();

    // Shadow configured to extrude from Border cells only. Border rows
    // are y=0 and y=5 of the SOURCE, so the role-filtered bounding rect
    // is the full source width by the y-span [0, 5].
    let shadow_config = tui_vfx_shadow::ShadowConfig::new(Color::BLACK.with_alpha(200))
        .with_style(tui_vfx_shadow::ShadowStyle::HalfBlock)
        .with_offset(2, 1)
        .with_edges(tui_vfx_shadow::ShadowEdges::BOTTOM_RIGHT)
        .with_source_region(RoleTag::Border);
    let shadow_spec = tui_vfx_compositor::pipeline::ShadowSpec::new(shadow_config);

    // Place the element at offset (1, 1) inside the destination so the
    // shadow's BOTTOM_RIGHT offset (+2, +1) has room to extrude.
    let offset_x = 1usize;
    let offset_y = 1usize;

    // t=1.0 drives both shader gradient and shadow progress to full —
    // required because HalfBlock shadow renders nothing at progress=0.
    let mut options = CompositionOptions::default()
        .with_shader_layer(&shader, StyleRegion::Role(RoleTag::Border))
        .with_shadow(shadow_spec);
    options.t = 1.0;

    // A.4 — wire an InspectionSinkBridge into the pipeline.
    let filter = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::PIPELINE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    let sink = Arc::new(TraceSink::new(filter));
    let frame_ctx =
        TraceFrameContext::new(0, 0).with_recipe_id(RecipeId::from("foundation.e2e.v1"));
    let mut bridge = InspectionSinkBridge::from_trace_sink(sink.clone(), frame_ctx);

    // ---- Act --------------------------------------------------------
    render_pipeline(
        &source,
        &source_roles,
        &mut dest_scene,
        src_w as usize,
        src_h as usize,
        offset_x,
        offset_y,
        options,
        Some(&mut bridge),
    );

    // ---- Assert (a): destination RoleMap has Shadow cells -----------
    //
    // The shadow stage writes RoleTag::Shadow into destination roles
    // for every cell it produced. Because we gave the shadow a
    // role-filtered bounding rect (Border rows 0 and 5 of the source),
    // the shadow does extrude, so at least one Shadow cell must appear
    // in the destination role map.
    let shadow_cell_count = count_shadow_cells_in_dest_roles(&dest_scene);
    assert!(
        shadow_cell_count > 0,
        "expected >= 1 cell tagged RoleTag::Shadow in destination roles; got {shadow_cell_count}",
    );

    // None of the Shadow cells must fall inside the element rect — the
    // shadow stage tags only cells the SHADOW wrote, not element cells.
    let elem_rect_x = offset_x as u16;
    let elem_rect_y = offset_y as u16;
    let elem_rect_w = src_w;
    let elem_rect_h = src_h;
    for y in 0..dest_scene.roles().height() {
        for x in 0..dest_scene.roles().width() {
            if dest_scene.role((x, y)) != Some(RoleTag::Shadow) {
                continue;
            }
            let inside_element = x >= elem_rect_x
                && x < elem_rect_x + elem_rect_w
                && y >= elem_rect_y
                && y < elem_rect_y + elem_rect_h;
            assert!(
                !inside_element,
                "Shadow role found inside element rect at ({x},{y})",
            );
        }
    }

    // ---- Assert (b): captured events match expected stage sequence --
    let report = sink.snapshot();
    assert!(
        !report.envelopes.is_empty(),
        "InspectionSinkBridge must have forwarded at least one envelope; got 0",
    );

    // Every envelope is PIPELINE stage (the filter admits only
    // PIPELINE; the bridge only forwards pipeline callbacks anyway).
    for env in &report.envelopes {
        assert!(
            env.event.stage().contains(StageMask::PIPELINE),
            "non-pipeline event leaked through the bridge: {:?}",
            env.event,
        );
        // Every envelope carries our recipe id and frame 0.
        assert_eq!(env.frame_no, 0);
        assert_eq!(
            env.recipe_id.as_ref().map(|r| r.as_str().to_string()),
            Some("foundation.e2e.v1".to_string()),
            "recipe_id must be stamped on every envelope",
        );
    }

    // We expect at least one CellRendered (final-stage cell write) and
    // at least one ShadowCellApplied in the shadow path.
    let cell_rendered = report
        .envelopes
        .iter()
        .filter(|e| matches!(e.event, TraceEvent::CellRendered { .. }))
        .count();
    let shadow_applied = report
        .envelopes
        .iter()
        .filter(|e| matches!(e.event, TraceEvent::ShadowCellApplied { .. }))
        .count();
    assert!(
        cell_rendered > 0,
        "expected >= 1 CellRendered event; got {cell_rendered}",
    );
    assert!(
        shadow_applied > 0,
        "expected >= 1 ShadowCellApplied event (shadow path exercised); got {shadow_applied}",
    );

    // seq_in_frame is strictly monotonic within the single frame.
    let mut prev: Option<u32> = None;
    for env in &report.envelopes {
        if let Some(prev_seq) = prev {
            assert!(
                env.seq_in_frame > prev_seq,
                "seq_in_frame must be strictly monotonic within a frame: prev={prev_seq}, this={}",
                env.seq_in_frame,
            );
        }
        prev = Some(env.seq_in_frame);
    }

    // ---- Assert (c): NDJSON round-trip preserves ordering -----------
    let mut buf = Vec::<u8>::new();
    report
        .to_ndjson(&mut buf)
        .expect("to_ndjson must succeed on a fresh TraceReport");
    let reader = Cursor::new(&buf);
    let round_tripped =
        TraceReport::from_ndjson(reader).expect("from_ndjson must succeed on to_ndjson output");

    assert_eq!(
        round_tripped.envelopes.len(),
        report.envelopes.len(),
        "NDJSON round-trip must preserve envelope count",
    );
    for (i, (orig, back)) in report
        .envelopes
        .iter()
        .zip(round_tripped.envelopes.iter())
        .enumerate()
    {
        assert_eq!(
            orig.frame_no, back.frame_no,
            "envelope[{i}] frame_no drift after round-trip",
        );
        assert_eq!(
            orig.seq_in_frame, back.seq_in_frame,
            "envelope[{i}] seq_in_frame drift after round-trip",
        );
        assert_eq!(
            orig.t_ms, back.t_ms,
            "envelope[{i}] t_ms drift after round-trip",
        );
        assert_eq!(
            orig.recipe_id, back.recipe_id,
            "envelope[{i}] recipe_id drift after round-trip",
        );
        assert_eq!(
            orig.event, back.event,
            "envelope[{i}] event drift after round-trip",
        );
    }
}

/// Count cells whose role tag is `RoleTag::Shadow` in the destination
/// scene's role map.
fn count_shadow_cells_in_dest_roles(scene: &SemanticScene) -> usize {
    let roles = scene.roles();
    let mut n = 0usize;
    for y in 0..roles.height() {
        for x in 0..roles.width() {
            if scene.role((x, y)) == Some(RoleTag::Shadow) {
                n += 1;
            }
        }
    }
    n
}

// <FILE>crates/tui-vfx/tests/test_foundation_end_to_end.rs</FILE> - <DESC>End-to-end foundation integration test</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
