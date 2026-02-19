// <FILE>crates/tui-vfx-shadow/src/types/shadow_config.rs</FILE> - <DESC>Shadow configuration with builder pattern</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Phase 1 rustdoc enrichment for documentation pipeline</WCTX>
// <CLOG>Enhance module documentation with configuration overview</CLOG>

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
//! | `offset_x/y` | `i8` | Shadow position relative to element |
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
//!     .with_offset(2, 1)
//!     .with_edges(ShadowEdges::BOTTOM_RIGHT);
//! ```
//!
//! [`Color`]: tui_vfx_types::Color

use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

use super::{ShadowEdges, ShadowStyle};

/// Configuration for rendering a shadow effect.
///
/// Use the builder pattern to construct a configuration:
///
/// ```
/// use tui_vfx_shadow::{ShadowConfig, ShadowStyle, ShadowEdges};
/// use tui_vfx_types::Color;
///
/// let config = ShadowConfig::new(Color::BLACK.with_alpha(128))
///     .with_offset(2, 1)
///     .with_style(ShadowStyle::HalfBlock)
///     .with_edges(ShadowEdges::BOTTOM_RIGHT)
///     .with_soft_edges(true);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ShadowConfig {
    /// Shadow rendering style.
    pub style: ShadowStyle,

    /// X offset from element (positive = right, negative = left).
    pub offset_x: i8,

    /// Y offset from element (positive = down, negative = up).
    pub offset_y: i8,

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
    pub edges: ShadowEdges,

    /// Whether to use soft edges (half-blocks at shadow boundaries).
    ///
    /// Only applies to `ShadowStyle::HalfBlock`. When true, the shadow
    /// edge uses half-block characters for a softer transition.
    pub soft_edges: bool,
}

impl Default for ShadowConfig {
    fn default() -> Self {
        Self {
            style: ShadowStyle::HalfBlock,
            offset_x: 1,
            offset_y: 1,
            color: Color::BLACK.with_alpha(128),
            surface_color: None,
            edges: ShadowEdges::BOTTOM_RIGHT,
            soft_edges: true,
        }
    }
}

impl ShadowConfig {
    /// Create a new shadow configuration with the specified color.
    ///
    /// Uses defaults for other settings:
    /// - Style: HalfBlock
    /// - Offset: (1, 1)
    /// - Edges: BOTTOM_RIGHT
    /// - Soft edges: enabled
    #[inline]
    pub fn new(color: Color) -> Self {
        Self {
            color,
            ..Default::default()
        }
    }

    /// Set the shadow offset (x, y).
    ///
    /// Positive x = shadow to the right, negative = left.
    /// Positive y = shadow below, negative = above.
    #[inline]
    pub fn with_offset(mut self, x: i8, y: i8) -> Self {
        self.offset_x = x;
        self.offset_y = y;
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
        assert_eq!(config.edges, ShadowEdges::BOTTOM_RIGHT);
        assert!(config.soft_edges);
    }

    #[test]
    fn test_builder_pattern() {
        let config = ShadowConfig::new(Color::RED)
            .with_offset(2, 3)
            .with_style(ShadowStyle::Solid)
            .with_edges(ShadowEdges::ALL)
            .with_soft_edges(false);

        assert_eq!(config.color, Color::RED);
        assert_eq!(config.offset_x, 2);
        assert_eq!(config.offset_y, 3);
        assert_eq!(config.style, ShadowStyle::Solid);
        assert_eq!(config.edges, ShadowEdges::ALL);
        assert!(!config.soft_edges);
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
// <VERS>END OF VERSION: 0.3.0</VERS>
