// <FILE>tui-vfx-style/src/models/v3/test_vfx_progress_emphasis_shader.rs</FILE> - <DESC>Focused tests for the V3 progress/emphasis family surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — keep the new V3 progress/emphasis surface covered while Highlighter remains the live legacy source during cutover.</WCTX>
// <CLOG>Extract focused conversion tests for VfxProgressEmphasisShader into a dedicated sibling file.</CLOG>

use super::{
    VfxProgressEmphasisApplyTo, VfxProgressEmphasisDirection, VfxProgressEmphasisMode,
    VfxProgressEmphasisRowMask, VfxProgressEmphasisShader, VfxProgressEmphasisTextContrast,
};
use crate::models::{
    ColorConfig, HighlighterApplyTo, HighlighterDirection, HighlighterMode, HighlighterRowMask,
    HighlighterShader, SpatialShaderType, TextContrast,
};

#[test]
fn converts_highlighter_into_v3_progress_emphasis_surface() {
    let legacy = HighlighterShader {
        color: ColorConfig::Yellow,
        apply_to: HighlighterApplyTo::Both,
        text_contrast: TextContrast::Preserve,
        mode: HighlighterMode::Band,
        band_width: 9,
        soft_edge: 0.5,
        blend_strength: 0.6,
        blend_strength_binding: Some("blend".to_string()),
        speed: 2.0,
        speed_binding: Some("speed".to_string()),
        direction: HighlighterDirection::EdgesIn,
        direction_binding: Some("direction".to_string()),
        row_mask: HighlighterRowMask::LastRow,
    };

    let converted = VfxProgressEmphasisShader::from(&legacy);
    assert_eq!(converted.color, ColorConfig::Yellow);
    assert_eq!(converted.apply_to, VfxProgressEmphasisApplyTo::Both);
    assert_eq!(
        converted.text_contrast,
        VfxProgressEmphasisTextContrast::Preserve
    );
    assert_eq!(converted.mode, VfxProgressEmphasisMode::Band);
    assert_eq!(converted.band_width, 9);
    assert_eq!(converted.soft_edge, 0.5);
    assert_eq!(converted.blend_strength, 0.6);
    assert_eq!(converted.blend_strength_binding.as_deref(), Some("blend"));
    assert_eq!(converted.speed, 2.0);
    assert_eq!(converted.speed_binding.as_deref(), Some("speed"));
    assert_eq!(converted.direction, VfxProgressEmphasisDirection::EdgesIn);
    assert_eq!(converted.direction_binding.as_deref(), Some("direction"));
    assert_eq!(converted.row_mask, VfxProgressEmphasisRowMask::LastRow);
}

#[test]
fn returns_none_for_non_progress_emphasis_legacy_variant() {
    let shader = SpatialShaderType::BorderSweep(crate::models::BorderSweepShader::default());
    assert!(VfxProgressEmphasisShader::from_legacy_spatial_shader(&shader).is_none());
}

#[test]
fn preserves_explicit_text_contrast_when_converting() {
    let legacy = HighlighterShader {
        text_contrast: TextContrast::Explicit {
            color: ColorConfig::Black,
        },
        row_mask: HighlighterRowMask::Range { start: 1, end: 3 },
        ..HighlighterShader::default()
    };

    let converted = VfxProgressEmphasisShader::from(&legacy);
    assert_eq!(
        converted.text_contrast,
        VfxProgressEmphasisTextContrast::Explicit {
            color: ColorConfig::Black,
        }
    );
    assert_eq!(
        converted.row_mask,
        VfxProgressEmphasisRowMask::Range { start: 1, end: 3 }
    );
}

// <FILE>tui-vfx-style/src/models/v3/test_vfx_progress_emphasis_shader.rs</FILE> - <DESC>Focused tests for the V3 progress/emphasis family surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
