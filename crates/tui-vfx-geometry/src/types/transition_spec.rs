// <FILE>tui-vfx-geometry/src/types/transition_spec.rs</FILE> - <DESC>Main configuration struct</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>mixed-signals migration: processing adoption</WCTX>
// <CLOG>Added optional quantize_steps for stepped animation support</CLOG>

use crate::easing::EasingType;
use crate::types::{EasingCurve, PathType, SnappingStrategy};
use serde::{Deserialize, Serialize};

/// Configuration for a motion transition.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct TransitionSpec {
    pub duration_ms: u64,
    pub ease: EasingCurve,
    pub path: PathType,
    pub snap: SnappingStrategy,
    /// Optional step quantization for pixel-art/retro animation style.
    /// When set, progress is snapped to discrete steps (e.g., 8 = 8 discrete frames).
    /// Uses the Quantize processing concept from mixed-signals.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantize_steps: Option<u32>,
}

impl Default for TransitionSpec {
    fn default() -> Self {
        Self {
            duration_ms: 500,
            ease: EasingCurve::Type(EasingType::Linear),
            path: PathType::Linear,
            snap: SnappingStrategy::Round,
            quantize_steps: None,
        }
    }
}

impl TransitionSpec {
    /// Apply quantization to a progress value if quantize_steps is set.
    ///
    /// Returns the quantized progress (snapped to discrete steps).
    pub fn quantize(&self, t: f64) -> f64 {
        match self.quantize_steps {
            Some(steps) if steps >= 2 => {
                // Snap t to discrete steps: floor(t * steps) / steps
                // This creates a staircase function for stepped animation
                let step = (t * steps as f64).floor() / steps as f64;
                step.clamp(0.0, 1.0)
            }
            _ => t,
        }
    }
}

// <FILE>tui-vfx-geometry/src/types/transition_spec.rs</FILE> - <DESC>Main configuration struct</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
