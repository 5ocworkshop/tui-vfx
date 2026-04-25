// <FILE>tui-vfx-style/src/models/v3/test_vfx_gradient_reveal_shader.rs</FILE> - <DESC>Focused tests for the V3 gradient-reveal family surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Audit recommendation 2.1 — extend the round-trip test to cover the new apply_to and intensity fields on the LinearGradient behaviour, and add an explicit non-default test so a future drift gets caught.</WCTX>
// <CLOG>0.2.0: extend the legacy→V3 LinearGradient conversion test to fill the new apply_to / intensity fields on LinearGradientShader, and assert that they round-trip into the grouped behaviour. Add a second test exercising explicit non-default values.
// 0.1.0: extract focused conversion tests for VfxGradientRevealShader into a dedicated sibling file</CLOG>

use super::{VfxGradientRevealBehavior, VfxGradientRevealShader, VfxRevealDirection};
use crate::models::{
    Gradient, LinearGradientApplyTo, LinearGradientShader, RevealDirection, RevealWipeShader,
    SpatialShaderType,
};
use tui_vfx_types::Color;

#[test]
fn converts_linear_gradient_into_v3_gradient_reveal_surface() {
    let legacy = LinearGradientShader {
        gradient: Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]),
        angle_deg: 90.0,
        apply_to: LinearGradientApplyTo::Foreground,
        intensity: 1.0,
    };

    let converted = VfxGradientRevealShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGradientRevealBehavior::LinearGradient {
            gradient: legacy.gradient.clone(),
            angle_deg: 90.0,
            apply_to: LinearGradientApplyTo::Foreground,
            intensity: 1.0,
        }
    );
}

#[test]
fn round_trips_explicit_apply_to_and_intensity() {
    let legacy = LinearGradientShader {
        gradient: Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]),
        angle_deg: 45.0,
        apply_to: LinearGradientApplyTo::Background,
        intensity: 0.6,
    };

    let converted = VfxGradientRevealShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGradientRevealBehavior::LinearGradient {
            gradient: legacy.gradient.clone(),
            angle_deg: 45.0,
            apply_to: LinearGradientApplyTo::Background,
            intensity: 0.6,
        }
    );
}

#[test]
fn converts_reveal_wipe_into_v3_gradient_reveal_surface() {
    let legacy = RevealWipeShader {
        direction: RevealDirection::BottomToTop,
    };

    let converted = VfxGradientRevealShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGradientRevealBehavior::RevealWipe {
            direction: VfxRevealDirection::BottomToTop,
        }
    );
}

#[test]
fn returns_none_for_non_gradient_reveal_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxGradientRevealShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_gradient_reveal_shader.rs</FILE> - <DESC>Focused tests for the V3 gradient-reveal family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
