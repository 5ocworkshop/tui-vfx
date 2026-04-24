// <FILE>crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs</FILE> - <DESC>Criterion bench: a representative 80×24, 4-layer, full-pipeline frame fits in ≤2ms at 60fps</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — acceptance bench for all-stages-on capture. Simulates one frame of: lifecycle phase entered, 1 asset + 1 procedural + 1 token + 1 recipe binding resolution, 4 layers (LayerStarted + ~80 cells painted + LayerCompleted) across an 80×24 surface, and a pipeline stream (SamplerApplied + MaskChecked + ShaderApplied + FilterApplied + CellRendered per cell, plus ShadowCellApplied for a band). Bench reports per-frame time so 2ms budget is observable at a glance.</WCTX>
// <CLOG>0.1.0: initial full-trace bench — generates ~11k events per frame; unbounded sink; accepts-all filter.</CLOG>

use std::sync::Arc;

use criterion::{Criterion, Throughput, criterion_group, criterion_main};

use tui_vfx_debug::inspection::{
    InspectionSink, TraceEnvelope, TraceEvent, TraceFilter, TraceSink,
};
use tui_vfx_types::{Cell, Color, LayerId, Modifiers, RecipeId, Rect, RoleTag, Style};

const WIDTH: u16 = 80;
const HEIGHT: u16 = 24;
const LAYERS: u16 = 4;

fn push_lifecycle(sink: &dyn InspectionSink, frame_no: u64, t_ms: u64, seq: &mut u32) {
    sink.report(TraceEnvelope {
        event: TraceEvent::LifecyclePhaseEntered {
            id: RecipeId::from("bench.splash"),
            phase: "enter".into(),
            t_ms,
        },
        frame_no,
        t_ms,
        recipe_id: Some(RecipeId::from("bench.splash")),
        seq_in_frame: *seq,
    });
    *seq += 1;
}

fn push_resolution(sink: &dyn InspectionSink, frame_no: u64, t_ms: u64, seq: &mut u32) {
    sink.report(TraceEnvelope {
        event: TraceEvent::AssetResolved {
            name: "logo.rs".into(),
            found: true,
            fallback_reason: None,
        },
        frame_no,
        t_ms,
        recipe_id: Some(RecipeId::from("bench.splash")),
        seq_in_frame: *seq,
    });
    *seq += 1;
    sink.report(TraceEnvelope {
        event: TraceEvent::ProceduralResolved {
            source_id: "spinner.braille".into(),
            resolved: true,
            fallback_id: None,
        },
        frame_no,
        t_ms,
        recipe_id: Some(RecipeId::from("bench.splash")),
        seq_in_frame: *seq,
    });
    *seq += 1;
    sink.report(TraceEnvelope {
        event: TraceEvent::TokenResolved {
            input: "{theme}".into(),
            output: "harbor".into(),
            missing_keys: Vec::new(),
        },
        frame_no,
        t_ms,
        recipe_id: Some(RecipeId::from("bench.splash")),
        seq_in_frame: *seq,
    });
    *seq += 1;
    sink.report(TraceEnvelope {
        event: TraceEvent::RecipeBindingResolved {
            selector: "splash".into(),
            recipe_id: RecipeId::from("bench.splash"),
            theme: "harbor".into(),
        },
        frame_no,
        t_ms,
        recipe_id: Some(RecipeId::from("bench.splash")),
        seq_in_frame: *seq,
    });
    *seq += 1;
}

fn push_composition(sink: &dyn InspectionSink, frame_no: u64, t_ms: u64, seq: &mut u32) {
    for l in 0..LAYERS {
        let layer_id = LayerId::from(format!("layer.{}", l));
        sink.report(TraceEnvelope {
            event: TraceEvent::LayerStarted {
                layer_id: layer_id.clone(),
                z: l as i32,
                source_kind: "scene".into(),
                target_rect: Rect::new(0, 0, WIDTH, HEIGHT),
            },
            frame_no,
            t_ms,
            recipe_id: Some(RecipeId::from("bench.splash")),
            seq_in_frame: *seq,
        });
        *seq += 1;
        for x in 0..80u16 {
            sink.report(TraceEnvelope {
                event: TraceEvent::LayerCellPainted {
                    layer_id: layer_id.clone(),
                    x,
                    y: l,
                    glyph: '#',
                    role: RoleTag::Text,
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
        }
        sink.report(TraceEnvelope {
            event: TraceEvent::LayerCompleted {
                layer_id,
                cells_painted: 80,
                cells_skipped: 0,
                fallback: false,
            },
            frame_no,
            t_ms,
            recipe_id: Some(RecipeId::from("bench.splash")),
            seq_in_frame: *seq,
        });
        *seq += 1;
    }
}

fn push_pipeline(sink: &dyn InspectionSink, frame_no: u64, t_ms: u64, seq: &mut u32) {
    let style = Style {
        fg: Color::new(200, 180, 160, 255),
        bg: Color::new(10, 10, 20, 255),
        mods: Modifiers::NONE,
    };
    let cell = Cell::default();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            sink.report(TraceEnvelope {
                event: TraceEvent::SamplerApplied {
                    dest_x: x,
                    dest_y: y,
                    src_x: Some(x),
                    src_y: Some(y),
                    sampler: "None#1".into(),
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
            sink.report(TraceEnvelope {
                event: TraceEvent::MaskChecked {
                    x,
                    y,
                    visible: true,
                    mask: "none#1".into(),
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
            sink.report(TraceEnvelope {
                event: TraceEvent::ShaderApplied {
                    x,
                    y,
                    before: style,
                    after: style,
                    shader: "pulse#1".into(),
                    region: Some("Full".into()),
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
            sink.report(TraceEnvelope {
                event: TraceEvent::FilterApplied {
                    x,
                    y,
                    before: cell,
                    after: cell,
                    filter: "tint#1".into(),
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
            if y < 2 {
                sink.report(TraceEnvelope {
                    event: TraceEvent::ShadowCellApplied {
                        x,
                        y,
                        shadow_cell: cell,
                        source_role: Some(RoleTag::Border),
                        source_empty: false,
                    },
                    frame_no,
                    t_ms,
                    recipe_id: Some(RecipeId::from("bench.splash")),
                    seq_in_frame: *seq,
                });
                *seq += 1;
            }
            sink.report(TraceEnvelope {
                event: TraceEvent::CellRendered {
                    x,
                    y,
                    final_cell: cell,
                },
                frame_no,
                t_ms,
                recipe_id: Some(RecipeId::from("bench.splash")),
                seq_in_frame: *seq,
            });
            *seq += 1;
        }
    }
}

fn emit_one_full_frame(sink: &dyn InspectionSink, frame_no: u64) {
    let mut seq: u32 = 0;
    let t_ms = frame_no * 16;
    push_lifecycle(sink, frame_no, t_ms, &mut seq);
    push_resolution(sink, frame_no, t_ms, &mut seq);
    push_composition(sink, frame_no, t_ms, &mut seq);
    push_pipeline(sink, frame_no, t_ms, &mut seq);
}

fn bench_full_trace_60fps(c: &mut Criterion) {
    let mut group = c.benchmark_group("full_trace_60fps");
    // One iteration == one frame.
    group.throughput(Throughput::Elements(1));
    group.bench_function("emit_one_full_frame", |b| {
        let sink: Arc<dyn InspectionSink> = Arc::new(TraceSink::new(TraceFilter::accept_all()));
        let mut frame: u64 = 0;
        b.iter(|| {
            emit_one_full_frame(sink.as_ref(), frame);
            frame = frame.wrapping_add(1);
        });
    });
    group.finish();
}

criterion_group!(benches, bench_full_trace_60fps);
criterion_main!(benches);

// <FILE>crates/tui-vfx-debug/benches/bench_full_trace_60fps.rs</FILE> - <DESC>Full-trace 60fps bench</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
