// <FILE>tui-vfx-style/src/models/v3/test_vfx_spatial_shader_family.rs</FILE> - <DESC>Focused tests for the central V3 spatial-shader family lowering seam</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — keep the central family-level lowering seam regression-covered so downstream runtime wiring can target one SSOT entry point with explicit primitive/composed layers.</WCTX>
// <CLOG>Update the central lowering tests to validate the primitive/composed split and cover the FocusField guidance case.</CLOG>

use super::{
    lower_legacy_spatial_shader, VfxSpatialComposedPrimitive, VfxSpatialPrimitive,
    VfxSpatialShaderFamily,
};
use crate::models::{
    BarberPoleShader, BorderSweepShader, ColorConfig, CursorShader, CursorShaderMode,
    FocusFieldApplyTo, FocusFieldShader, FocusFieldShape, HighlighterShader, SpatialShaderType,
};

#[test]
fn lowers_border_sweep_into_composed_traveling_band_family() {
    let shader = SpatialShaderType::BorderSweep(BorderSweepShader {
        speed: 1.0,
        length: 4,
        color: ColorConfig::Cyan,
        position_binding: None,
    });

    assert!(matches!(
        lower_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::TravelingBand(_))
    ));
}

#[test]
fn lowers_highlighter_into_composed_progress_emphasis_family() {
    let shader = SpatialShaderType::Highlighter(HighlighterShader::default());

    assert!(matches!(
        VfxSpatialShaderFamily::from_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::ProgressEmphasis(_))
    ));
}

#[test]
fn lowers_barber_pole_into_composed_stripe_motion_family() {
    let shader = SpatialShaderType::BarberPole(BarberPoleShader {
        speed: 1.0,
        stripe_width: 2,
        gap_width: 2,
        color: ColorConfig::Red,
    });

    assert!(matches!(
        lower_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::StripeMotion(_))
    ));
}

#[test]
fn lowers_cursor_into_composed_cursor_family() {
    let shader = SpatialShaderType::Cursor(CursorShader {
        mode: CursorShaderMode::Tint,
        tint: ColorConfig::Yellow,
        primary: None,
        trail: Vec::new(),
    });

    assert!(matches!(
        lower_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::Cursor(_))
    ));
}

#[test]
fn lowers_focus_field_into_composed_guidance_family() {
    let shader = SpatialShaderType::FocusField(FocusFieldShader {
        color: ColorConfig::Cyan,
        shape: FocusFieldShape::Ellipse,
        center_x: 4,
        center_y: 2,
        center_x_binding: None,
        center_y_binding: None,
        radius_x: 6,
        radius_y: 3,
        rect_x: 0,
        rect_y: 0,
        rect_width: 8,
        rect_height: 4,
        rect_x_binding: None,
        rect_y_binding: None,
        rect_width_binding: None,
        rect_height_binding: None,
        feather: 2,
        falloff: crate::models::FalloffType::Quadratic,
        intensity: 0.2,
        apply_to: FocusFieldApplyTo::Background,
        pulse_speed: 0.0,
    });

    assert!(matches!(
        lower_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::ComposedPrimitive(VfxSpatialComposedPrimitive::GuidanceCue(_))
    ));
}

#[test]
fn lowers_glow_into_primitive_surface_depth_family() {
    let shader = SpatialShaderType::Glow(crate::models::GlowShader::default());

    assert!(matches!(
        lower_legacy_spatial_shader(&shader),
        VfxSpatialShaderFamily::Primitive(VfxSpatialPrimitive::SurfaceDepth(_))
    ));
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_spatial_shader_family.rs</FILE> - <DESC>Focused tests for the central V3 spatial-shader family lowering seam</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
