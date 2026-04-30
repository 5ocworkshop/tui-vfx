// <FILE>crates/tui-vfx-contract/src/cls_scene_anchor.rs</FILE> - <DESC>Scene anchor placement vocabulary</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 scene parity: carry authored anchor placement semantics in canonical recipe scenes.</WCTX>
// <CLOG>0.1.0: INIT — add nine-position scene anchor enum.</CLOG>

/// Named anchor point used to place an element inside a scene or sibling layer.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum SceneAnchor {
    /// Top-left corner.
    TopLeft,
    /// Top edge centered horizontally.
    TopCenter,
    /// Top-right corner.
    TopRight,
    /// Left edge centered vertically.
    CenterLeft,
    /// Center point.
    Center,
    /// Right edge centered vertically.
    CenterRight,
    /// Bottom-left corner.
    BottomLeft,
    /// Bottom edge centered horizontally.
    BottomCenter,
    /// Bottom-right corner.
    BottomRight,
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_anchor.rs</FILE> - <DESC>SceneAnchor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
