// <FILE>tui-vfx-style/src/models/v3/enum_vfx_gradient_reveal_behavior.rs</FILE> - <DESC>V3 gradient-reveal family behavior surface</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for the remaining primitive gradient/reveal shaders while preserving the legacy LinearGradient and RevealWipe variants for current playback.</WCTX>
// <CLOG>Define the V3 gradient-reveal family enums and payloads that lift the shared directional fill/reveal behavior out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for gradient-reveal family shaders.
//!
//! This family groups the remaining primitive/substrate-aligned directional
//! fill/reveal treatments currently exposed as `LinearGradient` and
//! `RevealWipe`.

use crate::models::Gradient;
use serde::{Deserialize, Serialize};

/// Direction policy for reveal-style shaders.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxRevealDirection {
    /// Left-to-right reveal.
    #[default]
    LeftToRight,
    /// Right-to-left reveal.
    RightToLeft,
    /// Top-to-bottom reveal.
    TopToBottom,
    /// Bottom-to-top reveal.
    BottomToTop,
}

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
