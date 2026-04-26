// <FILE>tui-vfx-style/src/models/test_cls_water_field_signal.rs</FILE> - <DESC>Tests for WaterFieldSignal — equivalence to shader's internal sampling, output range, slope shortcut</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 5: TDD coverage for the Signal/SignalWithSlope wrapper around TerminalWaterShader.</WCTX>
// <CLOG>0.1.0: initial test coverage — same-(x,y,t) equivalence, unit-range output, analytic-slope correctness, subcell offset threading, ms-to-seconds time translation, Send + Sync.</CLOG>

use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalWithSlope};

use super::WaterFieldSignal;
use crate::models::cls_terminal_water_shader::TerminalWaterShader;

/// Floating-point equality with a small absolute tolerance for
/// equivalence checks against the shader's internal field sampling.
fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

/// Build a deterministic shader (default Ocean mode + default parameters)
/// and the matching signal wrapper for use across multiple tests.
fn make_signal() -> WaterFieldSignal {
    WaterFieldSignal::new(TerminalWaterShader::default())
}

#[test]
fn output_range_is_unit() {
    let signal = make_signal();
    assert_eq!(signal.output_range(), SignalRange::UNIT);
}

#[test]
fn sample_with_context_matches_shader_internal_sample_field_at() {
    // Anchor: the wrapper must produce the same light_scalar the shader
    // computes internally for the equivalent coordinate inputs.
    let signal = make_signal();
    let cell_x = 12_u16;
    let cell_y = 7_u16;
    let width = 80_u16;
    let height = 24_u16;
    let t_ms = 1234.0_f64;
    let t_seconds = (t_ms / 1000.0) as f32;

    let ctx = SignalContext::default()
        .with_dimensions(width, height)
        .with_cell_position(cell_x, cell_y)
        .with_absolute_time(t_ms);

    let got = signal.sample_with_context(0.0, &ctx);

    let expected_x = cell_x as f32;
    let expected_y = cell_y as f32 * 2.0;
    let expected = signal
        .shader()
        .sample_field_at(expected_x, expected_y, width, height, t_seconds)
        .light_scalar;

    assert!(
        approx_eq(got, expected, 1e-6),
        "wrapper light_scalar {got} != shader light_scalar {expected}"
    );
}

#[test]
fn sample_with_context_threads_subcell_offset() {
    // The subcell offset must be applied to (x, y) before sampling.
    // Check by comparing against the shader called with the exact
    // post-offset coordinates.
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 10)
        .with_cell_position(5, 3)
        .with_absolute_time(0.0)
        .with_subcell_offset(0.75, 0.125);

    let got = signal.sample_with_context(0.0, &ctx);

    let expected_x = 5.0 + 0.75;
    let expected_y = (3.0 + 0.125) * 2.0;
    let expected = signal
        .shader()
        .sample_field_at(expected_x, expected_y, 40, 10, 0.0)
        .light_scalar;

    assert!(
        approx_eq(got, expected, 1e-6),
        "subcell-offset wrapper {got} != shader-with-offset {expected}"
    );
}

#[test]
fn sample_with_context_converts_absolute_t_from_ms_to_seconds() {
    // tui-vfx writes elapsed milliseconds into SignalContext::absolute_t;
    // the wrapper must divide by 1000 before handing to the shader.
    let signal = make_signal();
    let t_ms = 5500.0_f64;
    let ctx = SignalContext::default()
        .with_dimensions(40, 10)
        .with_cell_position(0, 0)
        .with_absolute_time(t_ms);

    let got = signal.sample_with_context(0.0, &ctx);
    let expected = signal
        .shader()
        .sample_field_at(0.0, 0.0, 40, 10, 5.5)
        .light_scalar;

    assert!(
        approx_eq(got, expected, 1e-6),
        "ms-to-s conversion failed: got {got}, expected {expected}"
    );
}

#[test]
fn sample_with_context_falls_back_to_signal_time_when_absolute_t_absent() {
    // When ctx.absolute_t is None, the wrapper uses the t argument directly
    // (interpreted as seconds — caller's choice of unit).
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 10)
        .with_cell_position(0, 0);

    let got = signal.sample_with_context(2.5, &ctx);
    let expected = signal
        .shader()
        .sample_field_at(0.0, 0.0, 40, 10, 2.5)
        .light_scalar;

    assert!(
        approx_eq(got, expected, 1e-6),
        "fallback time path failed: got {got}, expected {expected}"
    );
}

#[test]
fn sample_with_context_clamps_zero_dimensions_to_one() {
    // The shader's sample_field_at expects width/height >= 1; the wrapper
    // clamps to avoid division-by-zero or empty-grid surprises.
    let signal = make_signal();
    let ctx = SignalContext::default(); // width=0, height=0
    // Should not panic; should return a finite value.
    let got = signal.sample_with_context(0.0, &ctx);
    assert!(got.is_finite());
    assert!((0.0..=1.0).contains(&got));
}

#[test]
fn output_stays_in_unit_range_across_sampled_grid() {
    // Property check: scan a small grid and confirm every sample is in [0,1].
    let signal = make_signal();
    for cell_x in 0..16_u16 {
        for cell_y in 0..8_u16 {
            for t_step in 0..4 {
                let ctx = SignalContext::default()
                    .with_dimensions(16, 8)
                    .with_cell_position(cell_x, cell_y)
                    .with_absolute_time(t_step as f64 * 250.0);
                let v = signal.sample_with_context(0.0, &ctx);
                assert!(
                    v.is_finite() && (0.0..=1.0).contains(&v),
                    "out-of-range sample at ({cell_x},{cell_y},t={t_step}): {v}"
                );
            }
        }
    }
}

#[test]
fn sample_no_ctx_returns_finite_unit_range_value() {
    let signal = make_signal();
    let v = signal.sample(1.0);
    assert!(v.is_finite());
    assert!((0.0..=1.0).contains(&v));
}

#[test]
fn sample_with_slope_returns_cached_shader_slopes_not_numeric_diff() {
    // The override must call the shader once and return the cached
    // slope_x/slope_y, not the default trait's three-call numeric diff.
    // We check this by asserting the SlopeSample fields exactly match
    // a fresh sample_field_at call; numeric differencing would produce
    // values dependent on the ±1 cell shift.
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 10)
        .with_cell_position(8, 4)
        .with_absolute_time(750.0);

    let slope = signal.sample_with_slope(0.0, &ctx);

    let expected = signal.shader().sample_field_at(8.0, 8.0, 40, 10, 0.75);
    assert!(approx_eq(slope.value, expected.light_scalar, 1e-6));
    assert!(approx_eq(slope.dx, expected.slope_x, 1e-6));
    assert!(approx_eq(slope.dy, expected.slope_y, 1e-6));
}

#[test]
fn sample_with_slope_returns_finite_values_at_origin() {
    let signal = make_signal();
    let ctx = SignalContext::default().with_dimensions(40, 10);
    let slope = signal.sample_with_slope(0.0, &ctx);
    assert!(slope.value.is_finite());
    assert!(slope.dx.is_finite());
    assert!(slope.dy.is_finite());
}

#[test]
fn signal_is_send_sync() {
    // Compile-time check: Signal trait requires Send + Sync.
    fn require_send_sync<T: Send + Sync>(_: &T) {}
    let signal = make_signal();
    require_send_sync(&signal);
}

#[test]
fn signal_is_clone() {
    let signal = make_signal();
    let _cloned = signal.clone();
}

// <FILE>tui-vfx-style/src/models/test_cls_water_field_signal.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
