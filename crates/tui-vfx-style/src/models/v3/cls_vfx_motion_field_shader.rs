// <FILE>tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs</FILE> - <DESC>V3 motion-field family shader surface</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Decision 2 migration slice — create a real grouped V3 surface for primitive motion-field shaders so PulseWave, Radar, and Orbit become migration inputs instead of the lasting conceptual model.</WCTX>
// <CLOG>0.2.0: include RadialSpiral in the grouped motion-field family.
// Introduce VfxMotionFieldShader plus conversion helpers from the legacy motion-field variants and SpatialShaderType.</CLOG>

//! V3 family surface for motion-field shaders.
//!
//! This grouped type provides a forward-looking V3 home for the primitive scan,
//! pulse, and orbit treatments that currently live as separate flat variants.

use crate::models::v3::enum_vfx_motion_field_behavior::{
    VfxMotionFieldBehavior, VfxMotionFieldDirection,
};
use crate::models::{
    OrbitShader, PulseWaveShader, RadarShader, RadialSpiralShader, SpatialShaderType,
    TerminalFireShader, TerminalWaterShader, WaveDirection,
};
use serde::{Deserialize, Serialize};

/// Canonical V3 family surface for motion-field shaders.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct VfxMotionFieldShader {
    /// Behavior/configuration surface for the chosen motion-field family member.
    pub behavior: VfxMotionFieldBehavior,
}

impl VfxMotionFieldShader {
    /// Convert a legacy flat `SpatialShaderType` variant into the V3
    /// motion-field family when that shader belongs to this family.
    pub fn from_legacy_spatial_shader(shader: &SpatialShaderType) -> Option<Self> {
        match shader {
            SpatialShaderType::PulseWave(shader) => Some(Self::from(shader)),
            SpatialShaderType::Radar(shader) => Some(Self::from(shader)),
            SpatialShaderType::Orbit(shader) => Some(Self::from(shader)),
            SpatialShaderType::RadialSpiral(shader) => Some(Self::from(shader)),
            SpatialShaderType::TerminalWater(shader) => Some(Self::from(shader)),
            _ => None,
        }
    }
}

impl From<&PulseWaveShader> for VfxMotionFieldShader {
    fn from(shader: &PulseWaveShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::PulseWave {
                frequency: shader.frequency,
                frequency_binding: shader.frequency_binding.clone(),
                speed: shader.speed,
                color: shader.color,
                direction: shader.direction.into(),
                wavelength: shader.wavelength,
            },
        }
    }
}

impl From<&RadarShader> for VfxMotionFieldShader {
    fn from(shader: &RadarShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::Radar {
                speed: shader.speed,
                tail_length: shader.tail_length,
                color: shader.color,
            },
        }
    }
}

impl From<&OrbitShader> for VfxMotionFieldShader {
    fn from(shader: &OrbitShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::Orbit {
                speed: shader.speed,
                dot_count: shader.dot_count,
                color: shader.color,
            },
        }
    }
}

impl From<&RadialSpiralShader> for VfxMotionFieldShader {
    fn from(shader: &RadialSpiralShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::RadialSpiral {
                arms: shader.arms,
                radial_frequency: shader.radial_frequency,
                radial_power: shader.radial_power,
                speed: shader.speed,
                blend_strength: shader.blend_strength,
                color: shader.color,
            },
        }
    }
}

impl From<&TerminalWaterShader> for VfxMotionFieldShader {
    fn from(shader: &TerminalWaterShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::TerminalWater {
                shader: shader.clone(),
            },
        }
    }
}

impl From<&TerminalFireShader> for VfxMotionFieldShader {
    fn from(shader: &TerminalFireShader) -> Self {
        Self {
            behavior: VfxMotionFieldBehavior::TerminalFire {
                shader: shader.clone(),
            },
        }
    }
}

impl From<WaveDirection> for VfxMotionFieldDirection {
    fn from(value: WaveDirection) -> Self {
        match value {
            WaveDirection::Horizontal => Self::Horizontal,
            WaveDirection::Vertical => Self::Vertical,
            WaveDirection::Radial => Self::Radial,
            WaveDirection::Diagonal => Self::Diagonal,
        }
    }
}

// <FILE>tui-vfx-style/src/models/v3/cls_vfx_motion_field_shader.rs</FILE> - <DESC>V3 motion-field family shader surface</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
