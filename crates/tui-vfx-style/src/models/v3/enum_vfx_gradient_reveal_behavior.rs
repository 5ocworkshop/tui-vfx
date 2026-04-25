// <FILE>tui-vfx-style/src/models/v3/enum_vfx_gradient_reveal_behavior.rs</FILE> - <DESC>V3 gradient-reveal family behavior surface</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Audit recommendation 2.1 — the V3 grouped LinearGradient behaviour previously had only `gradient` and `angle_deg`, dropping the apply_to and intensity fields that authors set on `gradient_overlay`. Add both fields so the V3 surface carries them through to the executable LinearGradientShader without loss.</WCTX>
// <CLOG>0.3.0: VfxGradientRevealBehavior::LinearGradient grew two new authoring fields, `apply_to: LinearGradientApplyTo` and `intensity: f32`. Both serde-default to back-compat values (Foreground / 1.0) so existing V3 recipes that omit them are unchanged. The grouped family now matches the runtime LinearGradientShader 1:1.
// 0.2.0: VfxRevealDirection becomes a re-export of tui_vfx_geometry::WipeDirection (full 20-variant vocabulary).
// 0.1.0: Decision 2 migration slice — initial four-cardinal V3 RevealDirection</CLOG>

//! V3 behavior surface for gradient-reveal family shaders.
//!
//! This family groups the remaining primitive/substrate-aligned directional
//! fill/reveal treatments currently exposed as `LinearGradient` and
//! `RevealWipe`. Reveal direction comes from the canonical
//! [`tui_vfx_geometry::WipeDirection`] vocabulary, exposed here as the
//! legacy alias [`VfxRevealDirection`] for back-compat.

use crate::models::{Gradient, LinearGradientApplyTo};
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::WipeDirection;

fn default_intensity() -> f32 {
    1.0
}

/// Direction policy for reveal-style shaders.
///
/// As of 0.2.0 this is a re-export of the canonical
/// [`tui_vfx_geometry::WipeDirection`] enum. The full 20-variant
/// vocabulary (cardinal, diagonal, centre-out, edges-in, corner-out,
/// corner-in) is therefore available at the V3 grouped family layer.
/// Existing recipes are unaffected — the four pre-existing variants
/// (`LeftToRight`, `RightToLeft`, `TopToBottom`, `BottomToTop`) are
/// the canonical defaults of `WipeDirection`.
pub type VfxRevealDirection = WipeDirection;

/// Behavior surface for the V3 gradient-reveal family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxGradientRevealBehavior {
    /// Static directional gradient fill.
    LinearGradient {
        /// Gradient ramp to sample.
        gradient: Gradient,
        /// Gradient angle in degrees.
        angle_deg: f32,
        /// Which colour channel(s) the gradient writes to. Default
        /// `Foreground` for back-compat with pre-0.3.0 V3 recipes.
        #[serde(default)]
        apply_to: LinearGradientApplyTo,
        /// Blend strength (0.0–1.0). Default 1.0 (fully replace target
        /// channel) matches pre-0.3.0 behaviour.
        #[serde(default = "default_intensity")]
        intensity: f32,
    },
    /// Progressive directional reveal.
    RevealWipe {
        /// Reveal direction.
        #[serde(default)]
        direction: VfxRevealDirection,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_gradient_reveal_behavior.rs</FILE> - <DESC>V3 gradient-reveal family behavior surface</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
