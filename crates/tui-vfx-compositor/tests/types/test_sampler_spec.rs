// <FILE>tui-vfx-compositor/tests/types/test_sampler_spec.rs</FILE> - <DESC>Tests for SamplerSpec</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T22:40:00Z</VERS>
// <WCTX>V2 Recipe Gap Analysis: Adding SamplerSpec with parameters</WCTX>
// <CLOG>Initial tests for SamplerSpec serialization and defaults</CLOG>

use mixed_signals::prelude::SignalOrFloat;
use tui_vfx_compositor::types::{Axis, RippleCenter, SamplerSpec};

#[test]
fn test_sampler_spec_default_is_none() {
    let spec = SamplerSpec::default();
    assert_eq!(spec, SamplerSpec::None);
}

#[test]
fn test_axis_serde_roundtrip() {
    for axis in [Axis::X, Axis::Y] {
        let json = serde_json::to_string(&axis).unwrap();
        let parsed: Axis = serde_json::from_str(&json).unwrap();
        assert_eq!(axis, parsed);
    }
}

#[test]
fn test_ripple_center_serde_roundtrip() {
    let centers = vec![RippleCenter::Center, RippleCenter::Point { x: 10, y: 20 }];
    for center in centers {
        let json = serde_json::to_string(&center).unwrap();
        let parsed: RippleCenter = serde_json::from_str(&json).unwrap();
        assert_eq!(center, parsed);
    }
}

#[test]
fn test_sampler_spec_sine_wave_serde_roundtrip() {
    let spec = SamplerSpec::SineWave {
        axis: Axis::Y,
        amplitude: SignalOrFloat::Static(2.5),
        frequency: SignalOrFloat::Static(3.0),
        speed: SignalOrFloat::Static(1.0),
        phase: SignalOrFloat::Static(0.5),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_sampler_spec_ripple_serde_roundtrip() {
    let spec = SamplerSpec::Ripple {
        amplitude: SignalOrFloat::Static(1.5),
        wavelength: SignalOrFloat::Static(4.0),
        speed: SignalOrFloat::Static(2.0),
        center: RippleCenter::Point { x: 5, y: 5 },
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_sampler_spec_shredder_serde_roundtrip() {
    let spec = SamplerSpec::Shredder {
        stripe_width: 3,
        odd_speed: SignalOrFloat::Static(1.5),
        even_speed: SignalOrFloat::Static(-1.5),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_sampler_spec_fault_line_serde_roundtrip() {
    let spec = SamplerSpec::FaultLine {
        seed: 42,
        intensity: SignalOrFloat::Static(0.8),
        split_bias: 0.3,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_sampler_spec_crt_serde_roundtrip() {
    let spec = SamplerSpec::Crt {
        scanline_strength: SignalOrFloat::Static(0.5),
        jitter: SignalOrFloat::Static(0.1),
        curvature: SignalOrFloat::Static(0.2),
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

#[test]
fn test_sampler_spec_crt_jitter_serde_roundtrip() {
    let spec = SamplerSpec::CrtJitter {
        intensity: SignalOrFloat::Static(0.7),
        speed_hz: SignalOrFloat::Static(30.0),
        decay_ms: 500,
    };
    let json = serde_json::to_string(&spec).unwrap();
    let parsed: SamplerSpec = serde_json::from_str(&json).unwrap();
    assert_eq!(spec, parsed);
}

// <FILE>tui-vfx-compositor/tests/types/test_sampler_spec.rs</FILE> - <DESC>Tests for SamplerSpec</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T22:40:00Z</VERS>
