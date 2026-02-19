// <FILE>tui-vfx-style/src/utils/fnc_easing.rs</FILE> - <DESC>Thin wrapper for tui_geometry easing functions</DESC>
// <VERS>VERSION: 2.0.0 - 2025-12-24</VERS>
// <WCTX>Easing Curves Expansion: Full Bezier support in tui-style-fx</WCTX>
// <CLOG>BREAKING: Updated to use EasingCurve instead of EasingType for Bezier support</CLOG>

use tui_vfx_geometry::types::EasingCurve;

/// Applies the specified easing curve to the normalized time `t`.
///
/// Supports both standard easing types and custom cubic Bézier curves.
/// Input `t` is expected to be in [0.0, 1.0].
/// Output is usually in [0.0, 1.0] but elastic/bounce/back functions may overshoot.
///
/// # Examples
///
/// ```
/// use tui_vfx_style::utils::fnc_easing::apply_easing;
/// use tui_vfx_geometry::types::EasingCurve;
/// use tui_vfx_geometry::easing::EasingType;
///
/// let result = apply_easing(0.5, EasingCurve::Type(EasingType::QuadOut));
/// assert!(result > 0.5); // QuadOut accelerates early
/// ```
#[inline]
pub fn apply_easing(t: f64, easing_curve: EasingCurve) -> f32 {
    easing_curve.ease(t)
}

// <FILE>tui-vfx-style/src/utils/fnc_easing.rs</FILE> - <DESC>Thin wrapper for tui_geometry easing functions</DESC>
// <VERS>END OF VERSION: 1.0.0 - 2025-12-24</VERS>
