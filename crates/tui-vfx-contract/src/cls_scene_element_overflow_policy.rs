// <FILE>crates/tui-vfx-contract/src/cls_scene_element_overflow_policy.rs</FILE> - <DESC>Scene element overflow policy DTO</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 scene parity: carry per-layer overflow semantics beyond simple clipping diagnostics.</WCTX>
// <CLOG>0.1.0: INIT — add clip, hide, and wrap overflow policies.</CLOG>

/// Policy for element-local cells that overflow the element's scene placement.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum SceneElementOverflowPolicy {
    /// Clip overflowing cells to the scene bounds.
    Clip,
    /// Hide the entire element when any painted cell would overflow.
    Hide,
    /// Wrap overflowing cells back inside the scene bounds.
    Wrap,
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_element_overflow_policy.rs</FILE> - <DESC>SceneElementOverflowPolicy</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
