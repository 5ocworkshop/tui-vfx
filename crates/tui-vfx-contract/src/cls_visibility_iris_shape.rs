// <FILE>crates/tui-vfx-contract/src/cls_visibility_iris_shape.rs</FILE> - <DESC>Visibility iris aperture shape enum</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3.1 transition tracks: represent iris aperture shape without generic type fields.</WCTX>
// <CLOG>0.1.0: INIT — add iris shape enum.</CLOG>

/// Aperture shape used by a `visibility.iris` transition track.
#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    serde::Serialize,
    serde::Deserialize,
    schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum VisibilityIrisShape {
    /// Circular aperture.
    Circle,
    /// Diamond aperture.
    Diamond,
}

// <FILE>crates/tui-vfx-contract/src/cls_visibility_iris_shape.rs</FILE> - <DESC>Visibility iris aperture shape enum</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
