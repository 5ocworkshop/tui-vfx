// <FILE>tui-vfx-style/src/models/v3/test_vfx_stripe_motion_shader.rs</FILE> - <DESC>Focused tests for the V3 stripe-motion family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 stripe-motion surface regression-covered while the legacy BarberPole variant remains operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxStripeMotionShader into a dedicated sibling file.</CLOG>

use super::{VfxStripeMotionBehavior, VfxStripeMotionShader};
use crate::models::{BarberPoleShader, ColorConfig, SpatialShaderType};

#[test]
fn converts_barber_pole_into_v3_stripe_motion_surface() {
    let legacy = BarberPoleShader {
        speed: 1.5,
        stripe_width: 3,
        gap_width: 2,
        angle_deg: 0.0,
        color: ColorConfig::Red,
        background_color: None,
        apply_to: Default::default(),
    };

    let converted = VfxStripeMotionShader::from(&legacy);
    assert_eq!(
        converted.behavior,
        VfxStripeMotionBehavior::BarberPole {
            speed: 1.5,
            stripe_width: 3,
            gap_width: 2,
            color: ColorConfig::Red,
        }
    );
}

#[test]
fn returns_none_for_non_stripe_motion_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxStripeMotionShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_stripe_motion_shader.rs</FILE> - <DESC>Focused tests for the V3 stripe-motion family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
