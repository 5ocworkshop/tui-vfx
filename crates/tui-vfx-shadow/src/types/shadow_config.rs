// <FILE>crates/tui-vfx-shadow/src/types/shadow_config.rs</FILE> - <DESC>Shadow configuration with builder pattern</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Sub-plan A Phase A.3.3 — add `source_region: Option<RoleTag>` so consumers can restrict shadow extrusion to cells whose source-map role matches (e.g. only extrude from Border cells instead of the whole widget rect). Default `None` preserves today's rect-based extrusion.</WCTX>
// <CLOG>0.6.0: MINOR — add `source_region: Option<RoleTag>` field, `with_source_region(role)` builder, and `source_region()` accessor. Default is None (back-compat: extrude from the element_rect as before). Serde round-trips both None and all 12 first-class RoleTag variants plus Custom. `#[serde(skip_serializing_if = "Option::is_none")]` keeps legacy JSON unchanged when unset.
// 0.5.0: add inset_x/inset_y fields plus with_inset builder.</CLOG>

//! # Shadow Configuration
//!
//! The [`ShadowConfig`] struct provides builder-pattern configuration for
//! shadow rendering. Shadows add depth and visual hierarchy to UI elements.
//!
//! ## Configuration Options
//!
//! | Option | Type | Description |
//! |--------|------|-------------|
//! | `style` | [`ShadowStyle`] | Rendering technique (HalfBlock, Braille, Solid, Gradient) |
//! | `offset_x/y` | `i8` | Shadow span beyond the element on each axis |
//! | `inset_x/y` | `Option<u8>` | Optional orthogonal inset override before horizontal/vertical edges begin |
//! | `color` | [`Color`] | Shadow color (use alpha for transparency) |
//! | `edges` | [`ShadowEdges`] | Which edges receive shadows |
//! | `soft_edges` | `bool` | Enable half-block edge transitions |
//!
//! ## Quick Start
//!
//! ```
//! use tui_vfx_shadow::{ShadowConfig, ShadowEdges};
//! use tui_vfx_types::Color;
//!
//! // Typical drop shadow
//! let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
//!     .with_offset(1, 1)
//!     .with_inset(2, 1)
//!     .with_edges(ShadowEdges::BOTTOM_RIGHT);
//! ```
//!
//! [`Color`]: tui_vfx_types::Color

use serde::{Deserialize, Serialize};
use tui_vfx_types::{Color, RoleTag};

use super::{ShadowCompositeMode, ShadowEdges, ShadowGradeConfig, ShadowStyle};

/// Configuration for rendering a shadow effect.
///
/// Use the builder pattern to construct a configuration:
///
/// ```
/// use tui_vfx_shadow::{ShadowConfig, ShadowStyle, ShadowEdges};
/// use tui_vfx_types::Color;
///
/// let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
///     .with_offset(1, 1)
///     .with_inset(2, 1)
///     .with_style(ShadowStyle::HalfBlock)
///     .with_edges(ShadowEdges::BOTTOM_RIGHT)
///     .with_soft_edges(true);
/// ```
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(default)]
pub struct ShadowConfig {
    /// Shadow rendering style.
    pub style: ShadowStyle,

    /// X offset from element (positive = right, negative = left).
    pub offset_x: i8,

    /// Y offset from element (positive = down, negative = up).
    pub offset_y: i8,

    /// Horizontal inset before top/bottom shadow edges begin.
    pub inset_x: Option<u8>,

    /// Vertical inset before left/right shadow edges begin.
    pub inset_y: Option<u8>,

    /// Shadow color.
    pub color: Color,

    /// Background/surface color for half-block blending.
    ///
    /// When rendering half-block shadows, this color is used for the
    /// "empty" half of edge cells. If `None`, the shadow will use
    /// transparent background.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_color: Option<Color>,

    /// Which edges to render shadow on.
    #[config(opaque)]
    pub edges: ShadowEdges,

    /// Whether to use soft edges (half-blocks at shadow boundaries).
    ///
    /// Only applies to `ShadowStyle::HalfBlock`. When true, the shadow
    /// edge uses half-block characters for a softer transition.
    pub soft_edges: bool,

    /// Shadow compositing mode.
    ///
    /// Controls how rendered shadow data is applied onto destination cells.
    /// The default [`GlyphOverlay`](ShadowCompositeMode::GlyphOverlay)
    /// preserves backward-compatible glyph-based shadow rendering.
    pub composite_mode: ShadowCompositeMode,

    /// Optional color grading parameters for
    /// [`GradeUnderlying`](ShadowCompositeMode::GradeUnderlying) mode.
    ///
    /// Ignored when `composite_mode` is `GlyphOverlay`. When `None` with
    /// `GradeUnderlying`, the compositor uses `ShadowGradeConfig::default()`
    /// (zero-strength, effectively no grading).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grade: Option<ShadowGradeConfig>,

    /// Optional role filter for shadow extrusion source.
    ///
    /// When `None` (the default), shadow extrusion is rectangular:
    /// shadow cells are emitted outside the supplied `element_rect` with
    /// no dependence on per-cell source content.
    ///
    /// When `Some(role)`, the shadow stage first computes the tight
    /// bounding rectangle of source cells whose role matches `role` (via
    /// [`crate::extract_shadow_envelope`]) and extrudes from THAT
    /// bounding rectangle instead. This lets a card shadow be driven by
    /// its `RoleTag::Border` cells rather than the whole widget rect —
    /// fixing the "shadow on text rect" problem where a borderless text
    /// card would otherwise cast shadow from the text cells themselves.
    ///
    /// Serialization skips this field when `None`, keeping legacy JSON
    /// recipes unchanged on round-trip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_region: Option<RoleTag>,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            style: ShadowStyle::HalfBlock,
            offset_x: 1,
            offset_y: 1,
            inset_x: None,
            inset_y: None,
            color: Color::BLACK.with_alpha(128),
            surface_color: None,
            edges: ShadowEdges::BOTTOM_RIGHT,
            soft_edges: true,
            composite_mode: ShadowCompositeMode::GlyphOverlay,
            grade: None,
            source_region: None,
        }
    }
}

impl ShadowConfig {
    /// Create a new shadow configuration with the specified color.
    ///
    /// Uses defaults for other settings:
    /// - Style: HalfBlock
    /// - Offset: (1, 1)
    /// - Inset: legacy renderer-derived behavior
    /// - Edges: BOTTOM_RIGHT
    /// - Soft edges: enabled
    #[inline]
    pub fn new(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    /// Set the shadow span beyond the element (x, y).
    ///
    /// Positive x = shadow to the right, negative = left.
    /// Positive y = shadow below, negative = above.
    ///
    /// The absolute values determine how many columns/rows of shadow extend
    /// beyond the element. Use [`with_inset`](Self::with_inset) to trim where
    /// those edge runs begin along the orthogonal axis.
    #[inline]
    pub fn with_offset(mut self, x: i8, y: i8) -> Self {
        self.offset_x = x;
        self.offset_y = y;
        self
    }

    /// Set the orthogonal shadow edge insets (x, y).
    ///
    /// `x` trims top/bottom shadow runs inward from the horizontal edge.
    /// `y` trims left/right shadow runs inward from the vertical edge.
    #[inline]
    pub fn with_inset(mut self, x: u8, y: u8) -> Self {
        self.inset_x = Some(x);
        self.inset_y = Some(y);
        self
    }

    /// Set the shadow rendering style.
    #[inline]
    pub fn with_style(mut self, style: ShadowStyle) -> Self {
        self.style = style;
        self
    }

    /// Set the shadow color.
    #[inline]
    pub fn with_color(mut self, color: Color) -> Self {
        self.color = color;
        self
    }

    /// Set the surface/background color for half-block blending.
    #[inline]
    pub fn with_surface_color(mut self, color: Color) -> Self {
        self.surface_color = Some(color);
        self
    }

    /// Set which edges should have shadows.
    #[inline]
    pub fn with_edges(mut self, edges: ShadowEdges) -> Self {
        self.edges = edges;
        self
    }

    /// Enable or disable soft edges (half-block transitions).
    #[inline]
    pub fn with_soft_edges(mut self, enabled: bool) -> Self {
        self.soft_edges = enabled;
        self
    }

    /// Set the shadow compositing mode.
    ///
    /// See [`ShadowCompositeMode`] for available modes.
    #[inline]
    pub fn with_composite_mode(mut self, mode: ShadowCompositeMode) -> Self {
        self.composite_mode = mode;
        self
    }

    /// Set custom grade parameters for grade-underlying mode.
    ///
    /// This also sets `composite_mode` to
    /// [`GradeUnderlying`](ShadowCompositeMode::GradeUnderlying).
    #[inline]
    pub fn with_grade(mut self, grade: ShadowGradeConfig) -> Self {
        self.composite_mode = ShadowCompositeMode::GradeUnderlying;
        self.grade = Some(grade);
        self
    }

    /// Enable dramatic grade-underlying mode with the recommended preset.
    ///
    /// Convenience builder that sets `composite_mode` to `GradeUnderlying`
    /// and `grade` to [`ShadowGradeConfig::dramatic()`].
    #[inline]
    pub fn with_dramatic_grade(self) -> Self {
        self.with_grade(ShadowGradeConfig::dramatic())
    }

    /// Set the role filter for shadow extrusion source.
    ///
    /// After this call, the shadow stage will extrude from the tight
    /// bounding rectangle of cells whose source role matches `role`
    /// instead of from the full caller-supplied `element_rect`. See the
    /// field docs on [`Self::source_region`] for the full contract.
    #[inline]
    pub fn with_source_region(mut self, role: RoleTag) -> Self {
        self.source_region = Some(role);
        self
    }

    /// Return the current source_region filter, if any.
    ///
    /// A plain accessor pair to the builder [`Self::with_source_region`]
    /// that avoids requiring downstream code to touch the field directly.
    #[inline]
    pub fn source_region(&self) -> Option<RoleTag> {
        self.source_region.clone()
    }

    /// Calculate the actual shadow color at a given progress value.
    ///
    /// This allows shadows to animate in/out by interpolating alpha.
    #[inline]
    pub fn color_at_progress(&self, progress: f64) -> Color {
        let alpha = (self.color.a as f64 * progress).round() as u8;
        self.color.with_alpha(alpha)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ShadowConfig::default();
        assert_eq!(config.style, ShadowStyle::HalfBlock);
        assert_eq!(config.offset_x, 1);
        assert_eq!(config.offset_y, 1);
        assert_eq!(config.inset_x, None);
        assert_eq!(config.inset_y, None);
        assert_eq!(config.edges, ShadowEdges::BOTTOM_RIGHT);
        assert!(config.soft_edges);
    }

    #[test]
    fn test_builder_pattern() {
        let config = ShadowConfig::new(Color::RED)
            .with_offset(2, 3)
            .with_inset(4, 5)
            .with_style(ShadowStyle::Solid)
            .with_edges(ShadowEdges::ALL)
            .with_soft_edges(false);

        assert_eq!(config.color, Color::RED);
        assert_eq!(config.offset_x, 2);
        assert_eq!(config.offset_y, 3);
        assert_eq!(config.inset_x, Some(4));
        assert_eq!(config.inset_y, Some(5));
        assert_eq!(config.style, ShadowStyle::Solid);
        assert_eq!(config.edges, ShadowEdges::ALL);
        assert!(!config.soft_edges);
    }

    #[test]
    fn shadow_config_defaults_to_glyph_overlay() {
        let config = ShadowConfig::default();
        assert_eq!(config.composite_mode, ShadowCompositeMode::GlyphOverlay);
        assert!(config.grade.is_none());
    }

    #[test]
    fn shadow_config_grade_underlying_serde_round_trip() {
        let config = ShadowConfig::new(Color::BLACK.with_alpha(180)).with_dramatic_grade();
        let json = serde_json::to_string(&config).unwrap();
        let restored: ShadowConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config, restored);
        assert_eq!(
            restored.composite_mode,
            ShadowCompositeMode::GradeUnderlying
        );
        assert!(restored.grade.is_some());
    }

    #[test]
    fn shadow_config_with_dramatic_grade_sets_mode_and_grade() {
        let config = ShadowConfig::new(Color::BLACK.with_alpha(128)).with_dramatic_grade();
        assert_eq!(config.composite_mode, ShadowCompositeMode::GradeUnderlying);
        assert_eq!(config.grade, Some(ShadowGradeConfig::dramatic()));
    }

    #[test]
    fn test_color_at_progress() {
        let config = ShadowConfig::new(Color::BLACK.with_alpha(200));

        // At progress 0.5, alpha should be ~100
        let color = config.color_at_progress(0.5);
        assert_eq!(color.a, 100);

        // At progress 0.0, alpha should be 0
        let color = config.color_at_progress(0.0);
        assert_eq!(color.a, 0);

        // At progress 1.0, alpha should be full
        let color = config.color_at_progress(1.0);
        assert_eq!(color.a, 200);
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_config.rs</FILE> - <DESC>Shadow configuration with builder pattern</DESC>
// <VERS>END OF VERSION: 0.6.0</VERS>
