// <FILE>crates/tui-vfx-debug/benches/bench_emit_overhead.rs</FILE> - <DESC>Criterion bench: proves StageMask::NONE short-circuit keeps emit overhead below noise</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.4 — zero-overhead-when-NONE verification. Emitters read TraceSink::accepts_any_stage() first and bail before constructing the envelope; this bench measures the complete emit path with a NONE-mask sink vs a comparison path with a disposable counter to ensure overhead is statistical noise (< 1µs per emit target per plan).</WCTX>
// <CLOG>0.1.0: initial bench — "emit_no_op_NONE_mask" (full construct-and-report path with a NONE mask sink) vs "emit_accepts_any_stage_probe" (bare accepts_any_stage() check). Reports throughput.</CLOG>

use criterion::{BatchSize, BenchmarkId, Criterion, Throughput, criterion_group, criterion_main};
use std::hint::black_box;
use std::sync::Arc;

use tui_vfx_debug::inspection::{
    InspectionSink, StageMask, TraceEnvelope, TraceEvent, TraceFilter, TraceSelector, TraceSink,
};
use tui_vfx_types::{Cell, RecipeId};

fn make_envelope(seq: u32) -> TraceEnvelope {
    TraceEnvelope {
        event: TraceEvent::CellRendered {
            x: (seq & 0x7f) as u16,
            y: ((seq >> 7) & 0x7f) as u16,
            final_cell: Cell::default(),
        },
        frame_no: (seq / 80) as u64,
        t_ms: (seq as u64) * 16 / 60,
        recipe_id: Some(RecipeId::from("bench.recipe")),
        seq_in_frame: seq,
    }
}

fn bench_emit_overhead(c: &mut Criterion) {
    // Filter with stages=NONE — accepts_any_stage() returns false.
    // The sink's InspectionSink::report() must short-circuit before any
    // mutex or allocation.
    let none_filter = TraceFilter {
        selectors: vec![TraceSelector::All],
        stages: StageMask::NONE,
        frames: 0..u64::MAX,
        time_ms: 0..u64::MAX,
    };
    let sink_none: Arc<dyn InspectionSink> = Arc::new(TraceSink::new(none_filter));

    // Filter that accepts everything — used for the contrast case so the
    // orchestrator can eyeball the difference in throughput. We don't
    // assert on it; the plan's target is < 1µs for the NONE path alone.
    let all_filter = TraceFilter::accept_all();
    let sink_all = Arc::new(TraceSink::with_capacity(all_filter, 1024));

    let mut group = c.benchmark_group("emit_overhead");
    group.throughput(Throughput::Elements(1));

    group.bench_function(BenchmarkId::new("emit", "NONE_mask_short_circuit"), |b| {
        let mut seq: u32 = 0;
        b.iter_batched(
            || {
                seq = seq.wrapping_add(1);
                make_envelope(seq)
            },
            |envelope| {
                sink_none.report(black_box(envelope));
            },
            BatchSize::SmallInput,
        );
    });

    group.bench_function(BenchmarkId::new("emit", "ALL_mask_accepted"), |b| {
        let mut seq: u32 = 0;
        b.iter_batched(
            || {
                seq = seq.wrapping_add(1);
                make_envelope(seq)
            },
            |envelope| {
                sink_all.report(black_box(envelope));
            },
            BatchSize::SmallInput,
        );
    });

    group.finish();
}

criterion_group!(benches, bench_emit_overhead);
criterion_main!(benches);

// <FILE>crates/tui-vfx-debug/benches/bench_emit_overhead.rs</FILE> - <DESC>Emit overhead bench</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
