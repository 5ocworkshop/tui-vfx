// <FILE>tui-vfx-style/src/models/v3/test_vfx_gradient_reveal_shader.rs</FILE> - <DESC>Focused tests for the V3 gradient-reveal family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 gradient-reveal surface regression-covered while the legacy flat variants remain operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxGradientRevealShader into a dedicated sibling file.</CLOG>

use super::{VfxGradientRevealBehavior, VfxGradientRevealShader, VfxRevealDirection};
use crate::models::{
    Gradient, LinearGradientShader, RevealDirection, RevealWipeShader, SpatialShaderType,
};
use tui_vfx_types::Color;

#[test]
fn converts_linear_gradient_into_v3_gradient_reveal_surface() {
    let legacy = LinearGradientShader {
        gradient: Gradient::new(vec![(0.0, Color::BLACK), (1.0, Color::WHITE)]),
        angle_deg: 90.0,
    };

    let converted = VfxGradientRevealShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxGradientRevealBehavior::LinearGradient {
            gradient: legacy.gradient.clone(),
            angle_deg: 90.0,
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
