// <FILE>crates/tui-vfx-contract/src/cls_visibility_iris_shape.rs</FILE> - <DESC>Visibility iris aperture shape enum</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add box aperture for axis-aligned rectangular iris reveals.</WCTX>
// <CLOG>0.2.0: MINOR — add box aperture variant for rectangular iris reveals.</CLOG>

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
    /// Axis-aligned rectangular aperture.
    Box,
}

// <FILE>crates/tui-vfx-contract/src/cls_visibility_iris_shape.rs</FILE> - <DESC>Visibility iris aperture shape enum</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
