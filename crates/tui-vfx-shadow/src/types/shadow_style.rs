// <FILE>crates/tui-vfx-shadow/src/types/shadow_style.rs</FILE> - <DESC>Shadow rendering style variants</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Add medium-shade character shadow style</WCTX>
// <CLOG>Add MediumShade enum variant and metadata reporting for docs/schema</CLOG>

//! # Shadow Styles
//!
//! Different rendering techniques for shadows, each with tradeoffs between
//! visual quality, terminal compatibility, and performance.
//!
//! ## Style Comparison
//!
//! | Style | Characters | Sub-cell | Compatibility | Best For |
//! |-------|------------|----------|---------------|----------|
//! | [`HalfBlock`] | `▐▄▌▀` | Yes | High | Default, most uses |
//! | [`Braille`] | `⣿` | Yes (2x4) | Medium | Density effects |
//! | [`MediumShade`] | `▒` | No | High | Textured full-cell shade |
//! | [`Solid`] | Space+BG | No | Maximum | Simple terminals |
//! | [`Gradient`] | Multiple | Layers | High | Soft shadows |
//!
//! [`HalfBlock`]: ShadowStyle::HalfBlock
//! [`Braille`]: ShadowStyle::Braille
//! [`MediumShade`]: ShadowStyle::MediumShade
//! [`Solid`]: ShadowStyle::Solid
//! [`Gradient`]: ShadowStyle::Gradient

/// The rendering style for shadows.
///
/// Different styles offer tradeoffs between visual quality and terminal compatibility.
#[derive(
    Clone,
    Copy,
    Debug,
    Default,
    PartialEq,
    serde::Serialize,
    serde::Deserialize,
    tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum ShadowStyle {
    /// Half-block characters (▐▄▌▀) for soft sub-cell shadows.
    ///
    /// This is the default style and provides the best visual quality on most terminals.
    /// Uses foreground/background color blending for sub-cell precision.
    #[default]
    HalfBlock,

    /// Braille patterns (⣿) for dithered/density-based shadows.
    ///
    /// Provides a 2x4 subpixel grid per cell for fine-grained density control.
    /// May not render correctly on all terminal fonts.
    Braille {
        /// Fill density from 0.0 (empty) to 1.0 (fully filled).
        density: f32,
    },

    /// Medium shade character cells (`▒`) for textured full-cell shadows.
    ///
    /// Uses foreground color with a fixed medium-density shade glyph.
    /// More visually pronounced than braille while preserving texture.
    MediumShade,

    /// Solid color cells (space with background color).
    ///
    /// The simplest shadow style - fills cells with solid background color.
    /// Maximum compatibility but no sub-cell precision.
    Solid,

    /// Multi-layer gradient shadow with decreasing intensity.
    ///
    /// Creates a softer shadow effect by rendering multiple layers
    /// with progressively lighter colors.
    Gradient {
        /// Number of gradient layers (1-4).
        /// More layers = softer shadow, but uses more screen space.
        layers: u8,
    },
}

impl ShadowStyle {
    /// Create a braille shadow with the specified density.
    ///
    /// # Arguments
    /// * `density` - Fill density from 0.0 (empty) to 1.0 (fully filled).
    ///   Values are clamped to this range.
    #[inline]
    pub fn braille(density: f32) -> Self {
        Self::Braille {
            density: density.clamp(0.0, 1.0),
        }
    }

    /// Create a gradient shadow with the specified number of layers.
    ///
    /// # Arguments
    /// * `layers` - Number of gradient layers (clamped to 1-4).
    #[inline]
    pub fn gradient(layers: u8) -> Self {
        Self::Gradient {
            layers: layers.clamp(1, 4),
        }
    }

    /// Returns the shadow style name as a string.
    pub fn name(&self) -> &'static str {
        match self {
            ShadowStyle::HalfBlock => "HalfBlock",
            ShadowStyle::Braille { .. } => "Braille",
            ShadowStyle::MediumShade => "MediumShade",
            ShadowStyle::Solid => "Solid",
            ShadowStyle::Gradient { .. } => "Gradient",
        }
    }

    /// Returns a brief human-readable description of what this style does.
    pub fn terse_description(&self) -> &'static str {
        match self {
            ShadowStyle::HalfBlock => "Half-block characters for soft sub-cell shadows",
            ShadowStyle::Braille { .. } => "Braille patterns for dithered/density-based shadows",
            ShadowStyle::MediumShade => {
                "Medium-shade character cells for textured full-cell shadows"
            }
            ShadowStyle::Solid => "Solid color cells (space with background color)",
            ShadowStyle::Gradient { .. } => "Multi-layer gradient shadow with decreasing intensity",
        }
    }

    /// Returns key parameters of this style for documentation purposes.
    pub fn key_parameters(&self) -> Vec<(&'static str, String)> {
        match self {
            ShadowStyle::HalfBlock => vec![],
            ShadowStyle::Braille { density } => vec![("density", format!("{}", density))],
            ShadowStyle::MediumShade => vec![],
            ShadowStyle::Solid => vec![],
            ShadowStyle::Gradient { layers } => vec![("layers", format!("{}", layers))],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_is_half_block() {
        assert_eq!(ShadowStyle::default(), ShadowStyle::HalfBlock);
    }

    #[test]
    fn test_braille_density_clamped() {
        let style = ShadowStyle::braille(2.0);
        match style {
            ShadowStyle::Braille { density } => assert_eq!(density, 1.0),
            _ => panic!("Expected Braille variant"),
        }

        let style = ShadowStyle::braille(-1.0);
        match style {
            ShadowStyle::Braille { density } => assert_eq!(density, 0.0),
            _ => panic!("Expected Braille variant"),
        }
    }

    #[test]
    fn test_gradient_layers_clamped() {
        let style = ShadowStyle::gradient(10);
        match style {
            ShadowStyle::Gradient { layers } => assert_eq!(layers, 4),
            _ => panic!("Expected Gradient variant"),
        }

        let style = ShadowStyle::gradient(0);
        match style {
            ShadowStyle::Gradient { layers } => assert_eq!(layers, 1),
            _ => panic!("Expected Gradient variant"),
        }
    }

    #[test]
    fn test_medium_shade_metadata() {
        let style = ShadowStyle::MediumShade;
        assert_eq!(style.name(), "MediumShade");
        assert_eq!(
            style.terse_description(),
            "Medium-shade character cells for textured full-cell shadows"
        );
        assert_eq!(style.key_parameters(), Vec::new());
    }
}

// <FILE>crates/tui-vfx-shadow/src/types/shadow_style.rs</FILE> - <DESC>Shadow rendering style variants</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
