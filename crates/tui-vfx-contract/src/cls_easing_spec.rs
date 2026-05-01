// <FILE>crates/tui-vfx-contract/src/cls_easing_spec.rs</FILE> - <DESC>Canonical transition easing DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition timing: express named, bezier, and spring easing structurally.</WCTX>
// <CLOG>0.1.0: INIT — add typed easing spec.</CLOG>

use crate::NamedEasing;

/// Canonical easing specification for transition timing.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(
    tag = "kind",
    rename_all = "camelCase",
    rename_all_fields = "camelCase",
    deny_unknown_fields
)]
pub enum EasingSpec {
    /// Closed named easing curve.
    Named {
        /// Named easing value.
        value: NamedEasing,
    },
    /// CSS-compatible cubic-bezier control points.
    CubicBezier {
        /// First control point x coordinate.
        x1: f64,
        /// First control point y coordinate.
        y1: f64,
        /// Second control point x coordinate.
        x2: f64,
        /// Second control point y coordinate.
        y2: f64,
    },
    /// Discrete stepped easing for intentionally quantized grid motion.
    Steps {
        /// Number of steps in the easing curve.
        count: u32,
        /// Step positioning, such as `start`, `end`, or `both`.
        position: String,
    },
    /// Spring timing described with response and damping ratio.
    Spring {
        /// Approximate response time in seconds.
        response: f64,
        /// Damping ratio, where values near 1.0 approach critical damping.
        damping_ratio: f64,
    },
}

// <FILE>crates/tui-vfx-contract/src/cls_easing_spec.rs</FILE> - <DESC>Canonical transition easing DTO</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
