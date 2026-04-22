// <FILE>tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs</FILE> - <DESC>Focused tests for the V3 motion-field family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 motion-field surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxMotionFieldShader into a dedicated sibling file.</CLOG>

use super::{VfxMotionFieldBehavior, VfxMotionFieldDirection, VfxMotionFieldShader};
use crate::models::{ColorConfig, OrbitShader, PulseWaveShader, RadarShader, SpatialShaderType};

#[test]
fn converts_pulse_wave_into_v3_motion_field_surface() {
    let legacy = PulseWaveShader {
        frequency: 3.0,
        frequency_binding: Some("freq".to_string()),
        speed: 1.25,
        color: ColorConfig::Magenta,
        direction: crate::models::WaveDirection::Radial,
        wavelength: 10.0,
    };

    let converted = VfxMotionFieldShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMotionFieldBehavior::PulseWave {
            frequency: 3.0,
            frequency_binding: Some("freq".to_string()),
            speed: 1.25,
            color: ColorConfig::Magenta,
            direction: VfxMotionFieldDirection::Radial,
            wavelength: 10.0,
        }
    );
}

#[test]
fn converts_radar_into_v3_motion_field_surface() {
    let legacy = RadarShader {
        speed: 0.8,
        tail_length: 1.4,
        color: ColorConfig::Cyan,
    };

    let converted = VfxMotionFieldShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMotionFieldBehavior::Radar {
            speed: 0.8,
            tail_length: 1.4,
            color: ColorConfig::Cyan,
        }
    );
}

#[test]
fn converts_orbit_into_v3_motion_field_surface() {
    let legacy = OrbitShader {
        speed: 1.5,
        dot_count: 5,
        color: ColorConfig::Yellow,
    };

    let converted = VfxMotionFieldShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxMotionFieldBehavior::Orbit {
            speed: 1.5,
            dot_count: 5,
            color: ColorConfig::Yellow,
        }
    );
}

#[test]
fn returns_none_for_non_motion_field_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxMotionFieldShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_motion_field_shader.rs</FILE> - <DESC>Focused tests for the V3 motion-field family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
