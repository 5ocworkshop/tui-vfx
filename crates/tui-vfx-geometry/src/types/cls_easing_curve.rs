// <FILE>tui-vfx-geometry/src/types/cls_easing_curve.rs</FILE> - <DESC>Wrapper type for easing curves with backward compatibility</DESC>
// <VERS>VERSION: 2.1.0 - 2025-12-25</VERS>
// <WCTX>Mixed-signals migration Phase 5 - WP3</WCTX>
// <CLOG>Migrated cubic Bezier solver to mixed_signals::math module</CLOG>

use crate::easing::{EasingType, ease};
use mixed_signals::math::solve_bezier;
use serde::{Deserialize, Serialize};

/// Wrapper enum for easing curves, supporting both simple types and cubic Bézier curves.
///
/// This type provides backward compatibility via serde's untagged deserialization:
/// - Old format: `"Linear"` (just the enum variant name)
/// - Bézier format: `{"x1": 0.25, "y1": 0.1, "x2": 0.25, "y2": 1.0}` (CSS cubic-bezier)
///
/// # Examples
///
/// ```
/// use tui_vfx_geometry::types::EasingCurve;
/// use tui_vfx_geometry::easing::EasingType;
///
/// // Create from EasingType
/// let curve: EasingCurve = EasingType::Linear.into();
/// assert_eq!(curve.ease(0.5), 0.5);
///
/// // Create cubic Bézier (CSS ease)
/// let curve = EasingCurve::bezier(0.25, 0.1, 0.25, 1.0);
/// assert!(curve.ease(0.5) > 0.5); // Accelerates
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(untagged)]
pub enum EasingCurve {
    /// Simple easing type (e.g., Linear, ExpoIn, CircOut)
    ///
    /// Serializes as the enum variant name (e.g., `"Linear"`)
    Type(EasingType),

    /// Cubic Bézier curve with control points
    ///
    /// Serializes as `{"x1": ..., "y1": ..., "x2": ..., "y2": ...}`
    /// Compatible with CSS cubic-bezier() format
    ///
    /// Note: This is an easing curve in normalized time/value space, not a
    /// spatial motion path. For spatial curves, use `PathType::Bezier` (quadratic).
    #[serde(rename_all = "camelCase")]
    Bezier { x1: f32, y1: f32, x2: f32, y2: f32 },
}

impl EasingCurve {
    /// Create a cubic Bézier curve with the given control points.
    ///
    /// # Panics
    ///
    /// Panics if x1 or x2 are outside the range [0, 1], as this would create
    /// a non-monotonic curve that cannot be uniquely inverted.
    ///
    /// # Examples
    ///
    /// ```
    /// use tui_vfx_geometry::types::EasingCurve;
    ///
    /// // CSS ease: cubic-bezier(0.25, 0.1, 0.25, 1.0)
    /// let curve = EasingCurve::bezier(0.25, 0.1, 0.25, 1.0);
    /// assert!(curve.ease(0.5) > 0.5);
    /// ```
    pub fn bezier(x1: f32, y1: f32, x2: f32, y2: f32) -> Self {
        assert!(
            (0.0..=1.0).contains(&x1) && (0.0..=1.0).contains(&x2),
            "x values must be in [0, 1] for monotonic Bézier curve"
        );
        EasingCurve::Bezier { x1, y1, x2, y2 }
    }

    /// Evaluate the easing curve at time `t` (usually 0.0 to 1.0).
    ///
    /// Returns the eased value, typically in [0.0, 1.0] but may overshoot
    /// for elastic/bounce/back/Bézier curves.
    pub fn ease(&self, t: f64) -> f32 {
        match self {
            EasingCurve::Type(easing_type) => ease(t, *easing_type),
            EasingCurve::Bezier { x1, y1, x2, y2 } => solve_bezier(t, *x1, *y1, *x2, *y2),
        }
    }
}

impl Default for EasingCurve {
    fn default() -> Self {
        EasingCurve::Type(EasingType::Linear)
    }
}

impl From<EasingType> for EasingCurve {
    fn from(easing_type: EasingType) -> Self {
        EasingCurve::Type(easing_type)
    }
}

// <FILE>tui-vfx-geometry/src/types/cls_easing_curve.rs</FILE> - <DESC>Wrapper type for easing curves with backward compatibility</DESC>
// <VERS>END OF VERSION: 2.1.0 - 2025-12-25</VERS>
