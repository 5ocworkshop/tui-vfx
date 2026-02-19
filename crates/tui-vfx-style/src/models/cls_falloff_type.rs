// <FILE>tui-vfx-style/src/models/cls_falloff_type.rs</FILE> - <DESC>Distance falloff curves for glow, AO, and bevel effects</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Add 7 new visual effects to tui-vfx per IDEAS.md</WCTX>
// <CLOG>Initial implementation of FalloffType enum with Linear/Quadratic/Exponential curves</CLOG>

use serde::{Deserialize, Serialize};

/// Distance falloff curves for spatial effects like glow, ambient occlusion, and bevel.
///
/// Controls how effect intensity decreases with distance from an edge or center point.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum FalloffType {
    /// Linear falloff: intensity = 1 - (distance / radius)
    /// Produces a uniform gradient.
    Linear,

    /// Quadratic falloff: intensity = 1 - (distance / radius)²
    /// Faster initial falloff, slower at edges. Natural-looking for most effects.
    #[default]
    Quadratic,

    /// Exponential falloff: intensity = e^(-2 * distance / radius)
    /// Very sharp initial falloff, long gradual tail. Good for concentrated effects.
    Exponential,
}

impl FalloffType {
    /// Apply the falloff curve to a distance value.
    ///
    /// # Arguments
    /// * `distance` - Current distance from edge/center (0.0 = at edge, positive = away)
    /// * `radius` - Maximum effect radius (where intensity reaches ~0)
    ///
    /// # Returns
    /// Intensity value in range 0.0..=1.0 (1.0 at distance=0, decreasing with distance)
    pub fn apply(&self, distance: f32, radius: f32) -> f32 {
        if radius <= 0.0 {
            return 0.0;
        }
        let normalized = (distance / radius).clamp(0.0, 1.0);
        match self {
            Self::Linear => 1.0 - normalized,
            Self::Quadratic => 1.0 - normalized * normalized,
            Self::Exponential => (-2.0 * normalized).exp() * (1.0 - normalized), // Smooth to 0 at edge
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_is_quadratic() {
        assert_eq!(FalloffType::default(), FalloffType::Quadratic);
    }

    #[test]
    fn linear_at_zero_distance_is_one() {
        let result = FalloffType::Linear.apply(0.0, 10.0);
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn linear_at_full_distance_is_zero() {
        let result = FalloffType::Linear.apply(10.0, 10.0);
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn linear_at_half_distance_is_half() {
        let result = FalloffType::Linear.apply(5.0, 10.0);
        assert!((result - 0.5).abs() < 0.001);
    }

    #[test]
    fn quadratic_at_zero_distance_is_one() {
        let result = FalloffType::Quadratic.apply(0.0, 10.0);
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn quadratic_at_full_distance_is_zero() {
        let result = FalloffType::Quadratic.apply(10.0, 10.0);
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn quadratic_slower_than_linear_at_half() {
        // Quadratic: 1 - 0.5² = 0.75
        let quadratic = FalloffType::Quadratic.apply(5.0, 10.0);
        let linear = FalloffType::Linear.apply(5.0, 10.0);
        assert!(
            quadratic > linear,
            "quadratic should be higher than linear at midpoint"
        );
        assert!((quadratic - 0.75).abs() < 0.001);
    }

    #[test]
    fn exponential_at_zero_distance_is_one() {
        let result = FalloffType::Exponential.apply(0.0, 10.0);
        assert!((result - 1.0).abs() < 0.001);
    }

    #[test]
    fn exponential_at_full_distance_approaches_zero() {
        let result = FalloffType::Exponential.apply(10.0, 10.0);
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn exponential_faster_initial_decay_than_linear() {
        // At 10% distance, exponential should have dropped more than linear
        let exp_10 = FalloffType::Exponential.apply(1.0, 10.0);
        let lin_10 = FalloffType::Linear.apply(1.0, 10.0);
        // Linear at 10%: 0.9
        // Exponential at 10%: e^(-0.2) * 0.9 ≈ 0.737
        assert!(exp_10 < lin_10, "exponential should fall faster initially");
    }

    #[test]
    fn zero_radius_returns_zero() {
        assert_eq!(FalloffType::Linear.apply(5.0, 0.0), 0.0);
        assert_eq!(FalloffType::Quadratic.apply(5.0, 0.0), 0.0);
        assert_eq!(FalloffType::Exponential.apply(5.0, 0.0), 0.0);
    }

    #[test]
    fn negative_radius_returns_zero() {
        assert_eq!(FalloffType::Linear.apply(5.0, -1.0), 0.0);
    }

    #[test]
    fn beyond_radius_clamps_to_zero() {
        let result = FalloffType::Linear.apply(15.0, 10.0);
        assert!(result.abs() < 0.001);
    }

    #[test]
    fn serde_roundtrip() {
        let original = FalloffType::Exponential;
        let json = serde_json::to_string(&original).unwrap();
        let parsed: FalloffType = serde_json::from_str(&json).unwrap();
        assert_eq!(original, parsed);
    }

    #[test]
    fn serde_snake_case() {
        let json = r#""linear""#;
        let parsed: FalloffType = serde_json::from_str(json).unwrap();
        assert_eq!(parsed, FalloffType::Linear);
    }
}

// <FILE>tui-vfx-style/src/models/cls_falloff_type.rs</FILE> - <DESC>Distance falloff curves for glow, AO, and bevel effects</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
