// <FILE>tui-vfx-style/src/models/v3/enum_vfx_style_effect_family.rs</FILE> - <DESC>Central V3 family representation for style effects</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 follow-on — extend the grouped V3 family seam beyond spatial shaders so runtime wiring can reason about all style-effect categories through one family surface.</WCTX>
// <CLOG>0.1.0: add a canonical V3 family enum for style effects covering fades, modulation, typography windows, instability, paired capabilities, and spatial families.</CLOG>

//! Canonical grouped V3 family representation for style effects.
//!
//! This keeps non-spatial style effects aligned with the grouped spatial shader
//! seam. During cutover, the legacy [`crate::models::StyleEffect`] enum remains
//! the executable surface; this enum provides stable grouped identity for docs,
//! debug output, and runtime wiring.

use serde::{Deserialize, Serialize};

use crate::models::VfxSpatialShaderFamily;

/// Canonical grouped V3 family identity for the current `StyleEffect` surface.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "family", content = "spatial", rename_all = "snake_case")]
pub enum VfxStyleEffectFamily {
    /// Fade/target-interpolation effects such as fade-in, fade-out, and color-fade.
    StyleFade,
    /// Continuous dwell-time style modulation such as pulse, rainbow, neon flicker,
    /// and color-shift.
    StyleModulation,
    /// Windowed typography treatment such as explicit italic windows.
    TypographyWindow,
    /// Instability/noise-driven style disruption such as glitch.
    StyleInstability,
    /// Cross-lane paired capabilities that share timing semantics with another lane.
    PairedCapability,
    /// Spatial style effects lowered through the grouped V3 spatial shader family seam.
    Spatial(VfxSpatialShaderFamily),
}

impl VfxStyleEffectFamily {
    /// Returns a stable family label for docs/debug/reporting surfaces.
    pub fn family_label(&self) -> &'static str {
        match self {
            VfxStyleEffectFamily::StyleFade => "style_fade",
            VfxStyleEffectFamily::StyleModulation => "style_modulation",
            VfxStyleEffectFamily::TypographyWindow => "typography_window",
            VfxStyleEffectFamily::StyleInstability => "style_instability",
            VfxStyleEffectFamily::PairedCapability => "paired_capability",
            VfxStyleEffectFamily::Spatial(family) => family.family_label(),
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_style_effect_family.rs</FILE> - <DESC>Central V3 family representation for style effects</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
