// <FILE>crates/tui-vfx-shadow/src/types/shadow_composite_mode.rs</FILE> - <DESC>Shadow compositing mode selection</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Transparent shadows need a destination-preserving alpha blend mode for dark surfaces where GradeUnderlying is visually too subtle.</WCTX>
// <CLOG>0.2.0: add BlendUnderlying for glyph-preserving alpha-aware background blending.</CLOG>

//! Shadow compositing mode.
//!
//! Controls how the compositor applies shadow data onto destination cells.
//! The default mode ([`GlyphOverlay`](ShadowCompositeMode::GlyphOverlay))
//! preserves backward-compatible glyph-based shadow rendering. The
//! [`GradeUnderlying`](ShadowCompositeMode::GradeUnderlying) mode leaves
//! destination glyphs in place and applies color grading to the shadow region.
//! [`BlendUnderlying`](ShadowCompositeMode::BlendUnderlying) also preserves
//! destination glyphs, but alpha-blends the rendered shadow color onto the
//! destination background.

use serde::{Deserialize, Serialize};

/// Determines how the compositor applies shadow data onto destination cells.
///
/// Shadow geometry, masks, offsets, and progress timing are shared across all
/// compositing modes. The mode controls only the final blending step.
///
/// # Backward Compatibility
///
/// The default is [`GlyphOverlay`](Self::GlyphOverlay), which preserves the
/// existing shadow rendering behavior. Switching to
/// [`GradeUnderlying`](Self::GradeUnderlying) or
/// [`BlendUnderlying`](Self::BlendUnderlying) changes only the blend step, not
/// the shadow geometry pipeline.
#[derive(
    Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ShadowCompositeMode {
    /// Classic shadow rendering: shadow cells overwrite destination cells
    /// with shadow glyphs and colors.
    ///
    /// This is the original behavior and remains the default for backward
    /// compatibility.
    #[default]
    GlyphOverlay,

    /// Destination-preserving shadow: leaves destination glyphs and modifiers
    /// intact while applying color grading (dim, desaturate, tint) to the
    /// shadow region.
    ///
    /// Use this mode with [`ShadowGradeConfig`](super::ShadowGradeConfig)
    /// to control grading intensity. The
    /// [`ShadowConfig::with_dramatic_grade`](super::ShadowConfig::with_dramatic_grade)
    /// builder provides a recommended visible preset.
    GradeUnderlying,

    /// Destination-preserving alpha blend: leaves destination glyphs,
    /// foreground colors, and modifiers intact while alpha-blending the
    /// rendered shadow color onto the destination background.
    ///
    /// Use this for translucent shadows over dark or textured surfaces where
    /// [`GradeUnderlying`](Self::GradeUnderlying) can be too subtle but
    /// [`GlyphOverlay`](Self::GlyphOverlay) would destroy the underlying glyph.
    BlendUnderlying,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_composite_mode_default_is_glyph_overlay() {
        assert_eq!(
            ShadowCompositeMode::default(),
            ShadowCompositeMode::GlyphOverlay
        );
    }

    #[test]
    fn shadow_composite_mode_serde_round_trip() {
        let modes = [
            ShadowCompositeMode::GlyphOverlay,
            ShadowCompositeMode::GradeUnderlying,
            ShadowCompositeMode::BlendUnderlying,
        ];
        for mode in &modes {
            let json = serde_json::to_string(mode).unwrap();
            let restored: ShadowCompositeMode = serde_json::from_str(&json).unwrap();
            assert_eq!(*mode, restored, "round-trip failed for {:?}", mode);
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_composite_mode.rs</FILE> - <DESC>Shadow compositing mode selection</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
