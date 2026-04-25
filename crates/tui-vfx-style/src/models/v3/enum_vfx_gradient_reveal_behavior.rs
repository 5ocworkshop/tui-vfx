// <FILE>tui-vfx-style/src/models/v3/enum_vfx_gradient_reveal_behavior.rs</FILE> - <DESC>V3 gradient-reveal family behavior surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Audit recommendation 1.2 + 1.3 — the V3 grouped reveal family previously inherited the four-cardinal RevealDirection vocabulary, which propagated the wipe-direction regression out of the legacy shader and into the V3 surface. Re-export tui_vfx_geometry::WipeDirection as VfxRevealDirection so the V3 surface, the legacy shader, and the Wipe mask all share one direction enum, and so the new corner-out / corner-in variants land at every layer simultaneously.</WCTX>
// <CLOG>0.2.0: VfxRevealDirection becomes a re-export of tui_vfx_geometry::WipeDirection (full 20-variant vocabulary). All existing V3 recipes that say `direction: "left_to_right"` parse identically. The wider set is now visible at the V3 grouped family layer for free.
// 0.1.0: Decision 2 migration slice — initial four-cardinal V3 RevealDirection</CLOG>

//! V3 behavior surface for gradient-reveal family shaders.
//!
//! This family groups the remaining primitive/substrate-aligned directional
//! fill/reveal treatments currently exposed as `LinearGradient` and
//! `RevealWipe`. Reveal direction comes from the canonical
//! [`tui_vfx_geometry::WipeDirection`] vocabulary, exposed here as the
//! legacy alias [`VfxRevealDirection`] for back-compat.

use crate::models::Gradient;
use serde::{Deserialize, Serialize};
use tui_vfx_geometry::WipeDirection;

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
