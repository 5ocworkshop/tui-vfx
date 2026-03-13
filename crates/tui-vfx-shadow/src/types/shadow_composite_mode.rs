// <FILE>crates/tui-vfx-shadow/src/types/shadow_composite_mode.rs</FILE> - <DESC>Shadow compositing mode selection</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 dramatic color-shadow rollout: add compositing mode enum</WCTX>
// <CLOG>Initial creation with GlyphOverlay and GradeUnderlying variants</CLOG>

//! Shadow compositing mode.
//!
//! Controls how the compositor applies shadow data onto destination cells.
//! The default mode ([`GlyphOverlay`](ShadowCompositeMode::GlyphOverlay))
//! preserves backward-compatible glyph-based shadow rendering. The
//! [`GradeUnderlying`](ShadowCompositeMode::GradeUnderlying) mode leaves
//! destination glyphs in place and applies color grading to the shadow region.

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
/// [`GradeUnderlying`](Self::GradeUnderlying) changes only the blend step,
/// not the shadow geometry pipeline.
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
        ];
        for mode in &modes {
            let json = serde_json::to_string(mode).unwrap();
            let restored: ShadowCompositeMode = serde_json::from_str(&json).unwrap();
            assert_eq!(*mode, restored, "round-trip failed for {:?}", mode);
        }
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_composite_mode.rs</FILE> - <DESC>Shadow compositing mode selection</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
