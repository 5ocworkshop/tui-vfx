// <FILE>tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs</FILE> - <DESC>V3 motion-field family behavior surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — create a grouped V3 home for primitive motion-field shaders while preserving the legacy PulseWave, Radar, and Orbit variants for current playback.</WCTX>
// <CLOG>0.2.0: add radial_spiral motion-field behavior for procedural spiral density fields.
// Define the V3 motion-field family enums and payloads that lift the shared field/scan behavior out of the legacy flat shader catalog.</CLOG>

//! V3 behavior surface for motion-field family shaders.
//!
//! This family groups the primitive/substrate-aligned motion-field treatments
//! currently exposed as `PulseWave`, `Radar`, and `Orbit`.

use crate::models::ColorConfig;
use serde::{Deserialize, Serialize};

/// Direction policy for wave-style motion fields.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum VfxMotionFieldDirection {
    /// Horizontal field motion.
    #[default]
    Horizontal,
    /// Vertical field motion.
    Vertical,
    /// Radial field motion.
    Radial,
    /// Diagonal field motion.
    Diagonal,
}

/// Behavior surface for the V3 motion-field family.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum VfxMotionFieldBehavior {
    /// Rippling wave field over the widget surface.
    PulseWave {
        /// Wave frequency in waves per cycle.
        #[config(default = 2.0)]
        frequency: f32,
        /// Optional runtime binding overriding frequency.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        frequency_binding: Option<String>,
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Target pulse color.
        color: ColorConfig,
        /// Field direction.
        #[serde(default)]
        direction: VfxMotionFieldDirection,
        /// Wavelength in cells.
        #[config(default = 8.0)]
        wavelength: f32,
    },
    /// Rotating radar sweep around the center.
    Radar {
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Sweep tail length in radians.
        #[config(default = 1.0)]
        tail_length: f32,
        /// Sweep color.
        color: ColorConfig,
    },
    /// Orbiting points around the center.
    Orbit {
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Number of orbiting dots.
        #[config(default = 3)]
        dot_count: u8,
        /// Dot color.
        color: ColorConfig,
    },
    /// Procedural radial spiral density field.
    RadialSpiral {
        /// Angular arm count / repetition factor.
        #[config(default = 1.5)]
        arms: f32,
        /// Radial ring frequency.
        #[config(default = 12.0)]
        radial_frequency: f32,
        /// Radius falloff power.
        #[config(default = 0.6)]
        radial_power: f32,
        /// Speed multiplier.
        #[config(default = 1.0)]
        speed: f32,
        /// Maximum blend strength.
        #[config(default = 0.5)]
        blend_strength: f32,
        /// Blend color.
        color: ColorConfig,
    },
}

// <FILE>tui-vfx-style/src/models/v3/enum_vfx_motion_field_behavior.rs</FILE> - <DESC>V3 motion-field family behavior surface</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
