// <FILE>crates/tui-vfx-shadow/src/types/shadow_grade_config.rs</FILE> - <DESC>Color grading parameters for grade-underlying shadow mode</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 0 dramatic color-shadow rollout: add grade config struct with dramatic preset</WCTX>
// <CLOG>Initial creation with locked dramatic numeric defaults</CLOG>

//! Shadow grade configuration.
//!
//! [`ShadowGradeConfig`] controls the strength of dim, desaturate, and tint
//! effects applied to destination cells when using
//! [`ShadowCompositeMode::GradeUnderlying`](super::ShadowCompositeMode::GradeUnderlying).
//!
//! All strength values range from `0.0` (no effect) to `1.0` (full effect)
//! and are further scaled by shadow coverage at each cell.

use serde::{Deserialize, Serialize};

/// Color grading parameters for the grade-underlying shadow compositing mode.
///
/// Each strength field ranges from `0.0` (no effect) to `1.0` (maximum).
/// Effective per-cell strength is `field_value * shadow_coverage`, where
/// coverage is derived from the rendered shadow cell's alpha channels.
///
/// Background grading is intentionally stronger than foreground grading in the
/// [`dramatic`](Self::dramatic) preset to preserve text readability while
/// making the shadow region clearly visible.
///
/// # Example
///
/// ```
/// use tui_vfx_shadow::ShadowGradeConfig;
///
/// // Use the recommended dramatic preset
/// let grade = ShadowGradeConfig::dramatic();
/// assert!(grade.bg_dim_strength > grade.fg_dim_strength);
///
/// // Or configure manually
/// let custom = ShadowGradeConfig {
///     fg_dim_strength: 0.15,
///     bg_dim_strength: 0.40,
///     ..Default::default()
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShadowGradeConfig {
    /// Foreground dimming strength (0.0–1.0).
    pub fg_dim_strength: f32,

    /// Background dimming strength (0.0–1.0).
    pub bg_dim_strength: f32,

    /// Foreground desaturation strength (0.0–1.0).
    pub fg_desaturate_strength: f32,

    /// Background desaturation strength (0.0–1.0).
    pub bg_desaturate_strength: f32,

    /// Foreground tint-toward-shadow-color strength (0.0–1.0).
    pub fg_tint_strength: f32,

    /// Background tint-toward-shadow-color strength (0.0–1.0).
    pub bg_tint_strength: f32,

    /// When `true`, preserve the destination foreground alpha channel.
    pub preserve_fg_alpha: bool,

    /// When `true`, preserve the destination background alpha channel.
    pub preserve_bg_alpha: bool,
}

impl Default for ShadowGradeConfig {
    /// Returns a neutral (zero-strength) grade configuration.
    fn default() -> Self {
        Self {
            fg_dim_strength: 0.0,
            bg_dim_strength: 0.0,
            fg_desaturate_strength: 0.0,
            bg_desaturate_strength: 0.0,
            fg_tint_strength: 0.0,
            bg_tint_strength: 0.0,
            preserve_fg_alpha: true,
            preserve_bg_alpha: true,
        }
    }
}

impl ShadowGradeConfig {
    /// Returns the recommended dramatic grade preset.
    ///
    /// This preset is intentionally moderate-to-strong so the shadow region
    /// is clearly visible on ordinary RGB terminal cells. Background grading
    /// is stronger than foreground grading to maintain text readability.
    ///
    /// Numeric values are locked by plan and must not change without a plan
    /// update.
    pub const fn dramatic() -> Self {
        Self {
            fg_dim_strength: 0.28,
            bg_dim_strength: 0.58,
            fg_desaturate_strength: 0.22,
            bg_desaturate_strength: 0.42,
            fg_tint_strength: 0.10,
            bg_tint_strength: 0.18,
            preserve_fg_alpha: true,
            preserve_bg_alpha: true,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shadow_grade_config_default_is_not_dramatic() {
        let def = ShadowGradeConfig::default();
        let dra = ShadowGradeConfig::dramatic();
        assert_ne!(def, dra, "default must differ from dramatic");
        assert_eq!(def.fg_dim_strength, 0.0);
        assert_eq!(def.bg_dim_strength, 0.0);
    }

    #[test]
    fn shadow_grade_config_dramatic_defaults_are_visible() {
        let d = ShadowGradeConfig::dramatic();

        // Locked numeric values from plan
        assert_eq!(d.fg_dim_strength, 0.28);
        assert_eq!(d.bg_dim_strength, 0.58);
        assert_eq!(d.fg_desaturate_strength, 0.22);
        assert_eq!(d.bg_desaturate_strength, 0.42);
        assert_eq!(d.fg_tint_strength, 0.10);
        assert_eq!(d.bg_tint_strength, 0.18);
        assert!(d.preserve_fg_alpha);
        assert!(d.preserve_bg_alpha);

        // Background grading must be stronger than foreground
        assert!(d.bg_dim_strength > d.fg_dim_strength);
        assert!(d.bg_desaturate_strength > d.fg_desaturate_strength);
        assert!(d.bg_tint_strength > d.fg_tint_strength);
    }

    #[test]
    fn shadow_grade_config_serde_round_trip() {
        let original = ShadowGradeConfig::dramatic();
        let json = serde_json::to_string(&original).unwrap();
        let restored: ShadowGradeConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(original, restored);
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_grade_config.rs</FILE> - <DESC>Color grading parameters for grade-underlying shadow mode</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
