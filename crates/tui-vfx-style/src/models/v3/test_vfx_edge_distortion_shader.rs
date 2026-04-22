// <FILE>tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs</FILE> - <DESC>Focused tests for the V3 edge-distortion family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 edge-distortion surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxEdgeDistortionShader into a dedicated sibling file.</CLOG>

use super::{VfxEdgeDistortionAxis, VfxEdgeDistortionBehavior, VfxEdgeDistortionShader};
use crate::models::{
    ChromaticEdgeShader, ColorConfig, GlitchLinesShader, NoiseType, SpatialShaderType,
    SubCellShakeShader,
};

#[test]
fn converts_glitch_lines_into_v3_edge_distortion_surface() {
    let legacy = GlitchLinesShader {
        seed: 7,
        intensity: 0.8,
        max_lines: 9,
        speed: 1.5,
        flash_chance: 0.2,
        pulse_color: Some(ColorConfig::White),
        pulse_speed: 0.75,
        italic_on_flash: true,
        flash_hold: 3,
        noise_type: NoiseType::Gaussian,
    };

    let converted = VfxEdgeDistortionShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxEdgeDistortionBehavior::GlitchLines {
            seed: 7,
            intensity: 0.8,
            max_lines: 9,
            speed: 1.5,
            flash_chance: 0.2,
            pulse_color: Some(ColorConfig::White),
            pulse_speed: 0.75,
            italic_on_flash: true,
            flash_hold: 3,
            noise_type: NoiseType::Gaussian,
        }
    );
}

#[test]
fn converts_chromatic_edge_into_v3_edge_distortion_surface() {
    let legacy = ChromaticEdgeShader {
        intensity: 0.4,
        edge_width: 0.2,
        horizontal: false,
    };

    let converted = VfxEdgeDistortionShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxEdgeDistortionBehavior::ChromaticEdge {
            intensity: 0.4,
            edge_width: 0.2,
            horizontal: false,
        }
    );
}

#[test]
fn converts_sub_cell_shake_into_v3_edge_distortion_surface() {
    let legacy = SubCellShakeShader {
        amplitude: 0.25,
        frequency: 18.0,
        axis: crate::models::ShakeAxis::Vertical,
        chromatic: true,
        seed: 11,
        edge_only: true,
        edge_width: 2,
    };

    let converted = VfxEdgeDistortionShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxEdgeDistortionBehavior::SubCellShake {
            amplitude: 0.25,
            frequency: 18.0,
            axis: VfxEdgeDistortionAxis::Vertical,
            chromatic: true,
            seed: 11,
            edge_only: true,
            edge_width: 2,
        }
    );
}

#[test]
fn returns_none_for_non_edge_distortion_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxEdgeDistortionShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_edge_distortion_shader.rs</FILE> - <DESC>Focused tests for the V3 edge-distortion family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
