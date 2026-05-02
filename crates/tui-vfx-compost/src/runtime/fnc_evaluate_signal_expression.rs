// <FILE>crates/tui-vfx-compost/src/runtime/fnc_evaluate_signal_expression.rs</FILE> - <DESC>Evaluate deterministic numeric signal expressions</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>SignalExpression value sources use normalized expression phase, not elapsed clocks.</WCTX>
// <CLOG>0.1.0: INIT — split signal-expression math from the value-source dispatch.</CLOG>

use std::f64::consts::TAU;

use tui_vfx_contract::SignalExpressionSpec;

pub(crate) fn evaluate_signal_expression(expression: &SignalExpressionSpec, t: f64) -> f64 {
    match expression {
        SignalExpressionSpec::Constant { value } => *value,
        SignalExpressionSpec::Ramp { start, end } => start + (end - start) * t,
        SignalExpressionSpec::Sine {
            frequency_hz,
            amplitude,
            offset,
            phase,
        } => offset + amplitude * ((t * frequency_hz + phase) * TAU).sin(),
        SignalExpressionSpec::Triangle {
            frequency_hz,
            amplitude,
            offset,
            phase,
        } => offset + amplitude * triangle_wave(t * frequency_hz + phase),
        SignalExpressionSpec::Mix { a, b, mix } => {
            evaluate_signal_expression(a, t) * (1.0 - mix) + evaluate_signal_expression(b, t) * mix
        }
        SignalExpressionSpec::Add { a, b } => {
            evaluate_signal_expression(a, t) + evaluate_signal_expression(b, t)
        }
        SignalExpressionSpec::Multiply { a, b } => {
            evaluate_signal_expression(a, t) * evaluate_signal_expression(b, t)
        }
        SignalExpressionSpec::Clamp { input, min, max } => {
            evaluate_signal_expression(input, t).clamp(*min, *max)
        }
    }
}

fn triangle_wave(cycles: f64) -> f64 {
    let cycle = cycles.rem_euclid(1.0);
    if cycle < 0.5 {
        cycle * 4.0 - 1.0
    } else {
        3.0 - cycle * 4.0
    }
}

// <FILE>crates/tui-vfx-compost/src/runtime/fnc_evaluate_signal_expression.rs</FILE> - <DESC>Evaluate deterministic numeric signal expressions</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
