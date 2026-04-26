// <FILE>tui-vfx-style/src/models/test_cls_fire_field_signal.rs</FILE> - <DESC>Tests for FireFieldSignal — equivalence to shader's internal sampling, output range, default-slope behavior</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Glyph rendering framework Phase 6: TDD coverage for the Signal/SignalWithSlope wrapper around TerminalFireShader.</WCTX>
// <CLOG>0.1.0: initial test coverage — same-(x,y,t) equivalence, unit-range output, subcell offset threading, ms-to-seconds time translation, default central-differencing slope behavior, Send + Sync, Clone.</CLOG>

use mixed_signals::traits::{Signal, SignalContext, SignalRange, SignalWithSlope};

use super::FireFieldSignal;
use crate::models::cls_terminal_fire_shader::TerminalFireShader;

fn approx_eq(a: f32, b: f32, eps: f32) -> bool {
    (a - b).abs() <= eps
}

fn make_signal() -> FireFieldSignal {
    FireFieldSignal::new(TerminalFireShader::default())
}

#[test]
fn output_range_is_unit() {
    let signal = make_signal();
    assert_eq!(signal.output_range(), SignalRange::UNIT);
}

#[test]
fn sample_with_context_matches_shader_internal_sample_field_at() {
    let signal = make_signal();
    let cell_x = 12_u16;
    let cell_y = 7_u16;
    let width = 40_u16;
    let height = 12_u16;
    let t_ms = 1234.0_f64;
    let t_seconds = (t_ms / 1000.0) as f32;

    let ctx = SignalContext::default()
        .with_dimensions(width, height)
        .with_cell_position(cell_x, cell_y)
        .with_absolute_time(t_ms);

    let got = signal.sample_with_context(0.0, &ctx);

    let expected = signal
        .shader()
        .sample_field_at(cell_x as f32, cell_y as f32, width, height, t_seconds)
        .intensity;

    assert!(
        approx_eq(got, expected, 1e-6),
        "wrapper intensity {got} != shader intensity {expected}"
    );
}

#[test]
fn sample_with_context_threads_subcell_offset() {
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 12)
        .with_cell_position(5, 3)
        .with_absolute_time(0.0)
        .with_subcell_offset(0.75, 0.125);

    let got = signal.sample_with_context(0.0, &ctx);

    let expected = signal
        .shader()
        .sample_field_at(5.75, 3.125, 40, 12, 0.0)
        .intensity;

    assert!(
        approx_eq(got, expected, 1e-6),
        "subcell-offset wrapper {got} != shader-with-offset {expected}"
    );
}

#[test]
fn sample_with_context_converts_absolute_t_from_ms_to_seconds() {
    let signal = make_signal();
    let t_ms = 5500.0_f64;
    let ctx = SignalContext::default()
        .with_dimensions(40, 12)
        .with_cell_position(0, 0)
        .with_absolute_time(t_ms);

    let got = signal.sample_with_context(0.0, &ctx);
    let expected = signal
        .shader()
        .sample_field_at(0.0, 0.0, 40, 12, 5.5)
        .intensity;

    assert!(
        approx_eq(got, expected, 1e-6),
        "ms-to-s conversion failed: got {got}, expected {expected}"
    );
}

#[test]
fn sample_with_context_falls_back_to_signal_time_when_absolute_t_absent() {
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 12)
        .with_cell_position(0, 0);

    let got = signal.sample_with_context(2.5, &ctx);
    let expected = signal
        .shader()
        .sample_field_at(0.0, 0.0, 40, 12, 2.5)
        .intensity;

    assert!(
        approx_eq(got, expected, 1e-6),
        "fallback time path failed: got {got}, expected {expected}"
    );
}

#[test]
fn sample_with_context_clamps_zero_dimensions_to_one() {
    let signal = make_signal();
    let ctx = SignalContext::default(); // width=0, height=0
    let got = signal.sample_with_context(0.0, &ctx);
    assert!(got.is_finite());
    assert!((0.0..=1.0).contains(&got));
}

#[test]
fn output_stays_in_unit_range_across_sampled_grid() {
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
fn sample_with_slope_uses_default_central_differencing() {
    // Fire intentionally does NOT cache analytic slopes (see module docs).
    // The default SignalWithSlope impl uses central differencing with a
    // ±1 cell shift. Verify it returns a finite SlopeSample without
    // panicking — exact gradient values are not contracted.
    let signal = make_signal();
    let ctx = SignalContext::default()
        .with_dimensions(40, 12)
        .with_cell_position(8, 4)
        .with_absolute_time(750.0);

    let slope = signal.sample_with_slope(0.0, &ctx);
    assert!(slope.value.is_finite());
    assert!(slope.dx.is_finite());
    assert!(slope.dy.is_finite());

    // Value must equal the direct sample_with_context at the same ctx.
    let direct = signal.sample_with_context(0.0, &ctx);
    assert!(
        approx_eq(slope.value, direct, 1e-6),
        "slope.value {} should match sample_with_context {}",
        slope.value,
        direct
    );
}

#[test]
fn sample_with_slope_returns_finite_values_at_origin() {
    let signal = make_signal();
    let ctx = SignalContext::default().with_dimensions(40, 12);
    let slope = signal.sample_with_slope(0.0, &ctx);
    assert!(slope.value.is_finite());
    assert!(slope.dx.is_finite());
    assert!(slope.dy.is_finite());
}

#[test]
fn signal_is_send_sync() {
    fn require_send_sync<T: Send + Sync>(_: &T) {}
    let signal = make_signal();
    require_send_sync(&signal);
}

#[test]
fn signal_is_clone() {
    let signal = make_signal();
    let _cloned = signal.clone();
}

// <FILE>tui-vfx-style/src/models/test_cls_fire_field_signal.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
