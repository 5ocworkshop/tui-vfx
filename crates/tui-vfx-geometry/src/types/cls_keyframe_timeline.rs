// <FILE>tui-vfx-geometry/src/types/cls_keyframe_timeline.rs</FILE> - <DESC>Multi-segment keyframe interpolation</DESC>
// <VERS>VERSION: 1.0.0 - 2025-12-18T19:35:00Z</VERS>
// <WCTX>Implementing timing/progress gaps</WCTX>
// <CLOG>Initial implementation of KeyframeTimeline for complex animations</CLOG>

use crate::easing::{EasingType, ease};
use serde::{Deserialize, Serialize};

/// A single keyframe in a timeline.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct Keyframe {
    /// Position in the timeline (0.0 to 1.0)
    pub time: f32,
    /// Value at this keyframe
    pub value: f32,
}

impl Keyframe {
    pub fn new(time: f32, value: f32) -> Self {
        Self {
            time: if time.is_finite() {
                time.clamp(0.0, 1.0)
            } else {
                0.0
            },
            value,
        }
    }
}

/// Multi-segment keyframe timeline for complex animations.
///
/// Maps input time t (0.0-1.0) to output value via piecewise interpolation
/// between keyframes. Useful for patterns like heartbeat (bump-bump-pause).
///
/// # Example: Heartbeat pattern
/// ```ignore
/// let heartbeat = KeyframeTimeline::new(vec![
///     Keyframe::new(0.0, 1.0),   // normal
///     Keyframe::new(0.1, 1.15),  // first bump
///     Keyframe::new(0.2, 1.0),   // back down
///     Keyframe::new(0.3, 1.25),  // second bump (bigger)
///     Keyframe::new(0.5, 1.0),   // settle
///     Keyframe::new(1.0, 1.0),   // hold
/// ], EasingType::CubicInOut);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
pub struct KeyframeTimeline {
    /// Keyframes sorted by time
    pub keyframes: Vec<Keyframe>,
    /// Easing to apply between keyframes
    #[serde(default)]
    pub easing: EasingType,
}

impl KeyframeTimeline {
    /// Create a new keyframe timeline. Keyframes are automatically sorted by time.
    pub fn new(mut keyframes: Vec<Keyframe>, easing: EasingType) -> Self {
        keyframes.sort_by(|a, b| a.time.total_cmp(&b.time));
        Self { keyframes, easing }
    }

    /// Create a simple linear timeline from start to end value.
    pub fn linear(start: f32, end: f32) -> Self {
        Self::new(
            vec![Keyframe::new(0.0, start), Keyframe::new(1.0, end)],
            EasingType::Linear,
        )
    }

    /// Create a heartbeat pattern (bump-bump-pause).
    pub fn heartbeat(base: f32, bump1: f32, bump2: f32) -> Self {
        Self::new(
            vec![
                Keyframe::new(0.0, base),
                Keyframe::new(0.1, base + bump1),
                Keyframe::new(0.2, base),
                Keyframe::new(0.3, base + bump2),
                Keyframe::new(0.5, base),
                Keyframe::new(1.0, base),
            ],
            EasingType::CubicInOut,
        )
    }

    /// Create a CRT power-on pattern (line → expand).
    /// Returns values for width/height scaling factor.
    pub fn crt_power_on() -> Self {
        Self::new(
            vec![
                Keyframe::new(0.0, 0.0),  // nothing
                Keyframe::new(0.3, 0.05), // thin line
                Keyframe::new(0.4, 0.1),  // slightly thicker
                Keyframe::new(1.0, 1.0),  // full size
            ],
            EasingType::CubicOut,
        )
    }

    /// Sample the timeline at time t (0.0 to 1.0).
    pub fn sample(&self, t: f64) -> f32 {
        let t = t.clamp(0.0, 1.0) as f32;

        if self.keyframes.is_empty() {
            return 0.0;
        }

        if self.keyframes.len() == 1 {
            return self.keyframes[0].value;
        }

        // Find the two keyframes we're between
        let mut lower_idx = 0;
        let mut upper_idx = self.keyframes.len() - 1;

        for (i, kf) in self.keyframes.iter().enumerate() {
            if kf.time <= t {
                lower_idx = i;
            }
            if kf.time >= t && i < upper_idx {
                upper_idx = i;
                break;
            }
        }

        // If at or beyond a keyframe
        if lower_idx == upper_idx {
            return self.keyframes[lower_idx].value;
        }

        let lower = &self.keyframes[lower_idx];
        let upper = &self.keyframes[upper_idx];

        // Calculate local t between keyframes
        let range = upper.time - lower.time;
        let local_t = if range > 0.0 {
            (t - lower.time) / range
        } else {
            0.0
        };

        // Apply easing to local t
        let eased_t = ease(local_t as f64, self.easing);

        // Interpolate value
        lower.value + (upper.value - lower.value) * eased_t
    }
}

impl Default for KeyframeTimeline {
    fn default() -> Self {
        Self::linear(0.0, 1.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linear_timeline() {
        let timeline = KeyframeTimeline::linear(0.0, 100.0);
        assert!((timeline.sample(0.0) - 0.0).abs() < 0.01);
        assert!((timeline.sample(0.5) - 50.0).abs() < 0.01);
        assert!((timeline.sample(1.0) - 100.0).abs() < 0.01);
    }

    #[test]
    fn test_multi_keyframe() {
        let timeline = KeyframeTimeline::new(
            vec![
                Keyframe::new(0.0, 0.0),
                Keyframe::new(0.5, 100.0),
                Keyframe::new(1.0, 50.0),
            ],
            EasingType::Linear,
        );

        assert!((timeline.sample(0.0) - 0.0).abs() < 0.01);
        assert!((timeline.sample(0.5) - 100.0).abs() < 0.01);
        assert!((timeline.sample(1.0) - 50.0).abs() < 0.01);
        // Midpoint between first two keyframes
        assert!((timeline.sample(0.25) - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_heartbeat_pattern() {
        let timeline = KeyframeTimeline::heartbeat(1.0, 0.15, 0.25);

        // Should start at base
        assert!((timeline.sample(0.0) - 1.0).abs() < 0.01);
        // Should be elevated at first bump
        assert!(timeline.sample(0.1) > 1.1);
        // Should return to base in middle
        assert!((timeline.sample(0.5) - 1.0).abs() < 0.1);
        // Should end at base
        assert!((timeline.sample(1.0) - 1.0).abs() < 0.01);
    }

    #[test]
    fn test_clamp_bounds() {
        let timeline = KeyframeTimeline::linear(10.0, 20.0);
        // Should clamp to start
        assert!((timeline.sample(-1.0) - 10.0).abs() < 0.01);
        // Should clamp to end
        assert!((timeline.sample(2.0) - 20.0).abs() < 0.01);
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_keyframe_timeline.rs</FILE> - <DESC>Multi-segment keyframe interpolation</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-18T19:35:00Z</VERS>
