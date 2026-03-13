// <FILE>crates/tui-vfx-compositor/src/types/cls_shadow_spec.rs</FILE> - <DESC>Shadow specification for compositor pipeline</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 0 dramatic color-shadow rollout: add grade-underlying contract tests</WCTX>
// <CLOG>Add tests for grade-underlying config preservation and default glyph-overlay assertion</CLOG>

//! Shadow specification for the compositor pipeline.
//!
//! Wraps [`ShadowConfig`] from `tui-vfx-shadow` for use in composition options.
//! The compositor renders shadows with the same mask as the element, ensuring
//! shadows wipe/dissolve/fade in sync with their element.

use serde::{Deserialize, Serialize};
use tui_vfx_shadow::ShadowConfig;

/// Shadow specification for compositor-integrated shadow rendering.
///
/// When added to [`CompositionOptions`](crate::pipeline::CompositionOptions),
/// the compositor will:
///
/// 1. Calculate the extended render area (element + shadow extent)
/// 2. Apply the same mask to both shadow and element regions
/// 3. Render shadow first, then element on top
///
/// **Important:** The rendered area will be larger than the source dimensions
/// by the shadow offset. For a 30x12 element with offset (2, 1), the total
/// rendered area is 32x13.
///
/// # Example
///
/// ```ignore
/// use tui_vfx_compositor::pipeline::CompositionOptions;
/// use tui_vfx_compositor::types::ShadowSpec;
/// use tui_vfx_shadow::{ShadowConfig, ShadowEdges};
/// use tui_vfx_types::Color;
///
/// let shadow = ShadowSpec::new(
///     ShadowConfig::new(Color::BLACK.with_alpha(150))
///         .with_offset(2, 1)
///         .with_edges(ShadowEdges::BOTTOM_RIGHT)
/// );
///
/// let options = CompositionOptions {
///     t: 0.5,
///     shadow: Some(shadow),
///     ..Default::default()
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShadowSpec {
    /// The shadow configuration from tui-vfx-shadow.
    #[serde(flatten)]
    pub config: ShadowConfig,
}

impl ShadowSpec {
    /// Create a new shadow specification.
    pub fn new(config: ShadowConfig) -> Self {
        Self { config }
    }

    /// Create a shadow spec from individual parameters (convenience).
    ///
    /// # Arguments
    /// * `color` - Shadow color (use `with_alpha()` for transparency)
    /// * `offset_x` - Horizontal offset (positive = right, negative = left)
    /// * `offset_y` - Vertical offset (positive = down, negative = up)
    pub fn simple(color: tui_vfx_types::Color, offset_x: i8, offset_y: i8) -> Self {
        Self {
            config: ShadowConfig::new(color).with_offset(offset_x, offset_y),
        }
    }

    /// Calculate the additional width needed for the shadow.
    ///
    /// Returns the number of extra columns to the right (if offset_x > 0)
    /// or to the left (if offset_x < 0).
    #[inline]
    pub fn extra_width(&self) -> usize {
        self.config.offset_x.unsigned_abs() as usize
    }

    /// Calculate the additional height needed for the shadow.
    ///
    /// Returns the number of extra rows below (if offset_y > 0)
    /// or above (if offset_y < 0).
    #[inline]
    pub fn extra_height(&self) -> usize {
        self.config.offset_y.unsigned_abs() as usize
    }

    /// Calculate the x offset for the element within the extended area.
    ///
    /// When offset_x < 0 (shadow to the left), the element shifts right.
    #[inline]
    pub fn element_offset_x(&self) -> usize {
        if self.config.offset_x < 0 {
            self.config.offset_x.unsigned_abs() as usize
        } else {
            0
        }
    }

    /// Calculate the y offset for the element within the extended area.
    ///
    /// When offset_y < 0 (shadow above), the element shifts down.
    #[inline]
    pub fn element_offset_y(&self) -> usize {
        if self.config.offset_y < 0 {
            self.config.offset_y.unsigned_abs() as usize
        } else {
            0
        }
    }
}

impl From<ShadowConfig> for ShadowSpec {
    fn from(config: ShadowConfig) -> Self {
        Self::new(config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_vfx_types::Color;

    #[test]
    fn test_extra_dimensions_positive_offset() {
        let spec = ShadowSpec::simple(Color::BLACK, 2, 1);
        assert_eq!(spec.extra_width(), 2);
        assert_eq!(spec.extra_height(), 1);
        assert_eq!(spec.element_offset_x(), 0);
        assert_eq!(spec.element_offset_y(), 0);
    }

    #[test]
    fn test_extra_dimensions_negative_offset() {
        let spec = ShadowSpec::simple(Color::BLACK, -2, -1);
        assert_eq!(spec.extra_width(), 2);
        assert_eq!(spec.extra_height(), 1);
        assert_eq!(spec.element_offset_x(), 2);
        assert_eq!(spec.element_offset_y(), 1);
    }

    #[test]
    fn test_from_shadow_config() {
        let config = ShadowConfig::new(Color::BLACK.with_alpha(100)).with_offset(3, 2);
        let spec: ShadowSpec = config.into();
        assert_eq!(spec.extra_width(), 3);
        assert_eq!(spec.extra_height(), 2);
    }

    #[test]
    fn shadow_spec_preserves_grade_underlying_config() {
        use tui_vfx_shadow::{ShadowCompositeMode, ShadowGradeConfig};

        let config = ShadowConfig::new(Color::BLACK.with_alpha(180)).with_dramatic_grade();
        let spec = ShadowSpec::new(config);

        assert_eq!(
            spec.config.composite_mode,
            ShadowCompositeMode::GradeUnderlying
        );
        assert_eq!(spec.config.grade, Some(ShadowGradeConfig::dramatic()));

        // Serde round-trip through spec
        let json = serde_json::to_string(&spec).unwrap();
        let restored: ShadowSpec = serde_json::from_str(&json).unwrap();
        assert_eq!(spec, restored);
    }

    #[test]
    fn shadow_spec_simple_still_defaults_to_glyph_overlay() {
        use tui_vfx_shadow::ShadowCompositeMode;

        let spec = ShadowSpec::simple(Color::BLACK.with_alpha(128), 2, 1);
        assert_eq!(
            spec.config.composite_mode,
            ShadowCompositeMode::GlyphOverlay
        );
        assert!(spec.config.grade.is_none());
    }
}

// <FILE>crates/tui-vfx-compositor/src/types/cls_shadow_spec.rs</FILE> - <DESC>Shadow specification for compositor pipeline</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
