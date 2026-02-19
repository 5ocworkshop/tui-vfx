// <FILE>tui-vfx-geometry/benches/easing.rs</FILE> - <DESC>Criterion benchmarks for easing functions</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-24</VERS>
// <WCTX>feat-20251224-183752: Easing curves expansion - Phase 1</WCTX>
// <CLOG>Initial benchmark suite for all 25 easing variants (19 existing + 6 new Expo/Circ)</CLOG>

use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use tui_vfx_geometry::easing::{EasingType, ease};

/// Benchmark a single easing function across multiple t values
fn bench_easing_type(c: &mut Criterion, easing_type: EasingType, name: &str) {
    let mut group = c.benchmark_group(name);

    // Benchmark at different t values to ensure realistic usage patterns
    for t in [0.0, 0.25, 0.5, 0.75, 1.0] {
        group.bench_with_input(BenchmarkId::new("t", t), &t, |b, &t| {
            b.iter(|| ease(black_box(t), black_box(easing_type)));
        });
    }

    group.finish();
}

/// Benchmark all existing easing types (baseline - should remain fast)
fn bench_existing_easing(c: &mut Criterion) {
    bench_easing_type(c, EasingType::Linear, "Linear");
    bench_easing_type(c, EasingType::QuadIn, "QuadIn");
    bench_easing_type(c, EasingType::QuadOut, "QuadOut");
    bench_easing_type(c, EasingType::QuadInOut, "QuadInOut");
    bench_easing_type(c, EasingType::CubicIn, "CubicIn");
    bench_easing_type(c, EasingType::CubicOut, "CubicOut");
    bench_easing_type(c, EasingType::CubicInOut, "CubicInOut");
    bench_easing_type(c, EasingType::SineIn, "SineIn");
    bench_easing_type(c, EasingType::SineOut, "SineOut");
    bench_easing_type(c, EasingType::SineInOut, "SineInOut");
    bench_easing_type(c, EasingType::BackIn, "BackIn");
    bench_easing_type(c, EasingType::BackOut, "BackOut");
    bench_easing_type(c, EasingType::BackInOut, "BackInOut");
    bench_easing_type(c, EasingType::ElasticIn, "ElasticIn");
    bench_easing_type(c, EasingType::ElasticOut, "ElasticOut");
    bench_easing_type(c, EasingType::ElasticInOut, "ElasticInOut");
    bench_easing_type(c, EasingType::BounceIn, "BounceIn");
    bench_easing_type(c, EasingType::BounceOut, "BounceOut");
    bench_easing_type(c, EasingType::BounceInOut, "BounceInOut");
}

/// Benchmark new Exponential easing types (target: <15ns per call)
fn bench_exponential_easing(c: &mut Criterion) {
    bench_easing_type(c, EasingType::ExpoIn, "ExpoIn");
    bench_easing_type(c, EasingType::ExpoOut, "ExpoOut");
    bench_easing_type(c, EasingType::ExpoInOut, "ExpoInOut");
}

/// Benchmark new Circular easing types (target: <15ns per call)
fn bench_circular_easing(c: &mut Criterion) {
    bench_easing_type(c, EasingType::CircIn, "CircIn");
    bench_easing_type(c, EasingType::CircOut, "CircOut");
    bench_easing_type(c, EasingType::CircInOut, "CircInOut");
}

/// Benchmark worst-case scenario: rapid sequential calls (frame-by-frame animation)
fn bench_animation_loop(c: &mut Criterion) {
    c.bench_function("animation_loop_60fps", |b| {
        b.iter(|| {
            // Simulate 60fps animation: 100 frames at 1/60th increments
            for i in 0..100 {
                let t = (i as f64) / 100.0;
                black_box(ease(black_box(t), black_box(EasingType::ExpoIn)));
            }
        });
    });
}

criterion_group!(
    benches,
    bench_existing_easing,
    bench_exponential_easing,
    bench_circular_easing,
    bench_animation_loop
);
criterion_main!(benches);

// <FILE>tui-vfx-geometry/benches/easing.rs</FILE> - <DESC>Criterion benchmarks for easing functions</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
