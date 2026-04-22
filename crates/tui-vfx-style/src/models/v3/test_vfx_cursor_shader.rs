// <FILE>tui-vfx-style/src/models/v3/test_vfx_cursor_shader.rs</FILE> - <DESC>Focused tests for the V3 cursor family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the grouped V3 cursor surface regression-covered while the legacy cursor variant remains operational for current playback.</WCTX>
// <CLOG>Extract focused conversion tests for VfxCursorShader into a dedicated sibling file.</CLOG>

use super::{VfxCursorMode, VfxCursorPrimary, VfxCursorShader, VfxCursorTrail};
use crate::models::{
    ColorConfig, CursorShader, CursorShaderMode, CursorShaderPrimary, CursorShaderTrail,
    SpatialShaderType,
};

#[test]
fn converts_cursor_into_v3_cursor_surface() {
    let legacy = CursorShader {
        mode: CursorShaderMode::Ghost,
        tint: ColorConfig::Cyan,
        primary: Some(CursorShaderPrimary {
            position: (2, 4),
            alpha: 0.7,
        }),
        trail: vec![CursorShaderTrail {
            position: (2, 3),
            alpha: 0.4,
            glyph: Some("·".to_string()),
        }],
    };

    let converted = VfxCursorShader::from(&legacy);
    assert_eq!(converted.mode, VfxCursorMode::Ghost);
    assert_eq!(converted.tint, ColorConfig::Cyan);
    assert_eq!(
        converted.primary,
        Some(VfxCursorPrimary {
            position: (2, 4),
            alpha: 0.7,
        })
    );
    assert_eq!(
        converted.trail,
        vec![VfxCursorTrail {
            position: (2, 3),
            alpha: 0.4,
            glyph: Some("·".to_string()),
        }]
    );
}

#[test]
fn returns_none_for_non_cursor_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxCursorShader::from_legacy_spatial_shader(&shader).is_none());
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_cursor_shader.rs</FILE> - <DESC>Focused tests for the V3 cursor family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
