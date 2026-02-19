// <FILE>tui-vfx-geometry/src/types/cls_time_warp_curve.rs</FILE> - <DESC>Time warp curves for progress remapping</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Effect parity: Timeline modifiers</WCTX>
// <CLOG>Initial implementation of TimeWarpCurve enum</CLOG>

use serde::{Deserialize, Serialize};

/// Curves for remapping animation progress.
///
/// Used by `ProgressModulator::TimeWarp` to transform the progress value.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum TimeWarpCurve {
    /// Linear remap from 0.0-1.0 to start..end range.
    /// Useful for playing only a portion of an animation.
    Linear {
        /// Output value when input is 0.0
        start: f32,
        /// Output value when input is 1.0
        end: f32,
    },

    /// Speed multiplier.
    /// Values > 1.0 speed up, < 1.0 slow down.
    /// Note: This effectively scales the progress, so multiplier=2.0
    /// means the animation completes in half the time.
    Speed {
        /// Speed multiplier (1.0 = normal speed)
        multiplier: f32,
    },

    /// Sigmoid curve for slow-fast-slow timing.
    /// Creates a dramatic acceleration in the middle.
    Sigmoid {
        /// Steepness of the curve (higher = sharper transition)
        #[serde(default = "default_steepness")]
        steepness: f32,
    },
}

fn default_steepness() -> f32 {
    6.0
}

impl Default for TimeWarpCurve {
    fn default() -> Self {
        Self::Linear {
            start: 0.0,
            end: 1.0,
        }
    }
}

impl TimeWarpCurve {
    /// Apply the time warp to a progress value.
    /// Input should be 0.0-1.0, output is the transformed progress.
    pub fn apply(&self, progress: f32) -> f32 {
        match self {
            TimeWarpCurve::Linear { start, end } => {
                // Linear interpolation from start to end
                start + (end - start) * progress
            }
            TimeWarpCurve::Speed { multiplier } => {
                // Scale progress by multiplier
                (progress * multiplier).clamp(0.0, 1.0)
            }
            TimeWarpCurve::Sigmoid { steepness } => {
                // Sigmoid function: 1 / (1 + e^(-steepness * (x - 0.5)))
                // This creates an S-curve centered at 0.5
                let x = progress * 2.0 - 1.0; // Map to -1..1
                let sigmoid = 1.0 / (1.0 + (-steepness * x).exp());
                // Normalize to ensure 0→0 and 1→1
                let sigmoid_0 = 1.0 / (1.0 + steepness.exp());
                let sigmoid_1 = 1.0 / (1.0 + (-steepness).exp());
                (sigmoid - sigmoid_0) / (sigmoid_1 - sigmoid_0)
            }
        }
    }

    /// Create a linear remap curve.
    pub fn linear(start: f32, end: f32) -> Self {
        Self::Linear { start, end }
    }

    /// Create a speed multiplier curve.
    pub fn speed(multiplier: f32) -> Self {
        Self::Speed { multiplier }
    }

    /// Create a sigmoid curve with default steepness.
    pub fn sigmoid() -> Self {
        Self::Sigmoid {
            steepness: default_steepness(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_identity() {
        let curve = TimeWarpCurve::Linear {
            start: 0.0,
            end: 1.0,
        };
        assert!((curve.apply(0.0) - 0.0).abs() < 0.001);
        assert!((curve.apply(0.5) - 0.5).abs() < 0.001);
        assert!((curve.apply(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_linear_remap() {
        // Remap to 0.2..0.8 range
        let curve = TimeWarpCurve::Linear {
            start: 0.2,
            end: 0.8,
        };
        assert!((curve.apply(0.0) - 0.2).abs() < 0.001);
        assert!((curve.apply(0.5) - 0.5).abs() < 0.001);
        assert!((curve.apply(1.0) - 0.8).abs() < 0.001);
    }

    #[test]
    fn test_linear_reverse() {
        // Reverse playback
        let curve = TimeWarpCurve::Linear {
            start: 1.0,
            end: 0.0,
        };
        assert!((curve.apply(0.0) - 1.0).abs() < 0.001);
        assert!((curve.apply(1.0) - 0.0).abs() < 0.001);
    }

    #[test]
    fn test_speed_normal() {
        let curve = TimeWarpCurve::Speed { multiplier: 1.0 };
        assert!((curve.apply(0.0) - 0.0).abs() < 0.001);
        assert!((curve.apply(0.5) - 0.5).abs() < 0.001);
        assert!((curve.apply(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_speed_double() {
        let curve = TimeWarpCurve::Speed { multiplier: 2.0 };
        assert!((curve.apply(0.0) - 0.0).abs() < 0.001);
        assert!((curve.apply(0.25) - 0.5).abs() < 0.001);
        // Clamps at 1.0
        assert!((curve.apply(0.5) - 1.0).abs() < 0.001);
        assert!((curve.apply(1.0) - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_speed_half() {
        let curve = TimeWarpCurve::Speed { multiplier: 0.5 };
        assert!((curve.apply(0.0) - 0.0).abs() < 0.001);
        assert!((curve.apply(0.5) - 0.25).abs() < 0.001);
        assert!((curve.apply(1.0) - 0.5).abs() < 0.001);
    }

    #[test]
    fn test_sigmoid_endpoints() {
        let curve = TimeWarpCurve::Sigmoid { steepness: 6.0 };
        // Should map 0→0 and 1→1
        assert!((curve.apply(0.0) - 0.0).abs() < 0.01);
        assert!((curve.apply(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_sigmoid_midpoint() {
        let curve = TimeWarpCurve::Sigmoid { steepness: 6.0 };
        // Midpoint should be around 0.5
        assert!((curve.apply(0.5) - 0.5).abs() < 0.01);
    }

    #[test]
    fn test_sigmoid_shape() {
        let curve = TimeWarpCurve::Sigmoid { steepness: 6.0 };
        // Early progress should be slow (output < input)
        assert!(curve.apply(0.2) < 0.2);
        // Late progress should be fast (output > input at this point due to S-curve)
        assert!(curve.apply(0.8) > 0.8);
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_time_warp_curve.rs</FILE> - <DESC>Time warp curves for progress remapping</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
