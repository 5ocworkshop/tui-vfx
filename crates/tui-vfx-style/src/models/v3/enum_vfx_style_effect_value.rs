// <FILE>tui-vfx-style/src/models/v3/enum_vfx_style_effect_value.rs</FILE> - <DESC>Grouped V3 executable value surface for overall style effects</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 runtime follow-on — grouped V3 effect identity has been propagating through labels and runtime seams, and now needs an executable grouped value surface that can round-trip back into StyleEffect during cutover.</WCTX>
// <CLOG>0.1.0: define a grouped V3 value enum for overall style effects plus lowering helpers to and from the legacy StyleEffect surface.</CLOG>

//! Grouped V3 executable value surface for overall style effects.

use serde::{Deserialize, Serialize};

use crate::models::{StyleEffect, VfxSpatialShaderFamily};
use crate::models::v3::{
    TryLowerV3StyleEffectError, VfxStyleEffectFamily, try_lower_v3_spatial_shader_family,
};

/// Grouped V3 executable value surface for overall style effects.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "grouped_effect", rename_all = "snake_case")]
pub enum VfxStyleEffectValue {
    /// Fade/target-interpolation effects.
    StyleFade(StyleEffect),
    /// Continuous dwell-time style modulation effects.
    StyleModulation(StyleEffect),
    /// Windowed typography effects.
    TypographyWindow(StyleEffect),
    /// Instability/noise-driven style effects.
    StyleInstability(StyleEffect),
    /// Cross-lane paired capabilities.
    PairedCapability(StyleEffect),
    /// Spatial style effects lowered through the grouped V3 spatial family seam.
    Spatial(VfxSpatialShaderFamily),
}

impl VfxStyleEffectValue {
    /// Lower a legacy `StyleEffect` into the grouped V3 value surface.
    pub fn from_legacy_style_effect(effect: &StyleEffect) -> Self {
        match effect.v3_effect_family() {
            VfxStyleEffectFamily::StyleFade => Self::StyleFade(effect.clone()),
            VfxStyleEffectFamily::StyleModulation => Self::StyleModulation(effect.clone()),
            VfxStyleEffectFamily::TypographyWindow => Self::TypographyWindow(effect.clone()),
            VfxStyleEffectFamily::StyleInstability => Self::StyleInstability(effect.clone()),
            VfxStyleEffectFamily::PairedCapability => Self::PairedCapability(effect.clone()),
            VfxStyleEffectFamily::Spatial(family) => Self::Spatial(family),
        }
    }

    /// Return the grouped family classification for this value.
    pub fn family(&self) -> VfxStyleEffectFamily {
        match self {
            Self::StyleFade(_) => VfxStyleEffectFamily::StyleFade,
            Self::StyleModulation(_) => VfxStyleEffectFamily::StyleModulation,
            Self::TypographyWindow(_) => VfxStyleEffectFamily::TypographyWindow,
            Self::StyleInstability(_) => VfxStyleEffectFamily::StyleInstability,
            Self::PairedCapability(_) => VfxStyleEffectFamily::PairedCapability,
            Self::Spatial(family) => VfxStyleEffectFamily::Spatial(family.clone()),
        }
    }

    /// Attempt to lower this grouped V3 value back into the executable legacy
    /// `StyleEffect` surface.
    pub fn try_to_legacy_style_effect(&self) -> Result<StyleEffect, TryLowerV3StyleEffectError> {
        match self {
            Self::StyleFade(effect) => validate_non_spatial(effect, "style_fade"),
            Self::StyleModulation(effect) => validate_non_spatial(effect, "style_modulation"),
            Self::TypographyWindow(effect) => validate_non_spatial(effect, "typography_window"),
            Self::StyleInstability(effect) => validate_non_spatial(effect, "style_instability"),
            Self::PairedCapability(effect) => validate_non_spatial(effect, "paired_capability"),
            Self::Spatial(family) => Ok(StyleEffect::Spatial {
                shader: try_lower_v3_spatial_shader_family(family)?,
            }),
        }
    }
}

fn validate_non_spatial(
    effect: &StyleEffect,
    expected_family: &'static str,
) -> Result<StyleEffect, TryLowerV3StyleEffectError> {
    let actual_family = effect.v3_effect_family().family_label();
    if actual_family != expected_family {
        return Err(TryLowerV3StyleEffectError::MismatchedVariant {
            expected_family,
            actual_effect: effect.effect_type_name(),
        });
    }
    Ok(effect.clone())
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_style_effect_value.rs</FILE> - <DESC>Grouped V3 executable value surface for overall style effects</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
