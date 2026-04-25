// <FILE>crates/tui-vfx-shadow/src/types/shadow_style.rs</FILE> - <DESC>Shadow rendering style variants</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Default shadows should use the full-cell translucent style; half-block remains available but is no longer the ergonomic default.</WCTX>
// <CLOG>Switch ShadowStyle default from HalfBlock to Solid and refresh style guidance.</CLOG>

//! # Shadow Styles
//!
//! Different rendering techniques for shadows, each with tradeoffs between
//! visual quality, terminal compatibility, and performance.
//!
//! ## Style Comparison
//!
//! | Style | Characters | Sub-cell | Compatibility | Best For |
//! |-------|------------|----------|---------------|----------|
//! | [`Solid`] | Space+BG | No | Maximum | Default translucent drop shadow |
//! | [`Braille`] | `⣿` | Yes (2x4) | Medium | Density effects |
//! | [`MediumShade`] | `▒` | No | High | Textured full-cell shade |
//! | [`HalfBlock`] | `▐▄▌▀` | Yes | High | Legacy sub-cell texture |
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
    /// Use this when a recipe deliberately wants visible sub-cell texture. The
    /// default is [`Solid`](Self::Solid), which gives the cleaner translucent
    /// full-cell drop shadow preferred by the V3 recipe path.
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

    /// Solid translucent full-cell shadow (space with alpha-bearing background).
    ///
    /// This is the default V3 shadow style. It produces the cleanest drop
    /// shadow in modern true-color terminals: a transparent full-cell overlay
    /// offset from the host, without half-block glyph texture.
    #[default]
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
            ShadowStyle::HalfBlock => {
                "Half-block characters for deliberate sub-cell shadow texture"
            }
            ShadowStyle::Braille { .. } => "Braille patterns for dithered/density-based shadows",
            ShadowStyle::MediumShade => {
                "Medium-shade character cells for textured full-cell shadows"
            }
            ShadowStyle::Solid => "Translucent full-cell drop shadow using alpha background cells",
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
    fn test_default_is_solid_translucent_shadow() {
        assert_eq!(ShadowStyle::default(), ShadowStyle::Solid);
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
// <VERS>END OF VERSION: 0.6.0</VERS>
