// <FILE>crates/tui-vfx-contract/src/cls_clip_policy.rs</FILE> - <DESC>Scene element out-of-bounds composition policy</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: make out-of-bounds element placement deterministic.</WCTX>
// <CLOG>0.1.0: ADD — introduce clip and warn policies for D1 scene composition.</CLOG>

/// Policy for element cells that land outside the final scene bounds.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase")]
pub enum ClipPolicy {
    /// Ignore out-of-bounds element cells and preserve the current composed surface.
    Clip,
    /// Ignore out-of-bounds element cells and emit one element-aware warning diagnostic.
    Warn,
}

// <FILE>crates/tui-vfx-contract/src/cls_clip_policy.rs</FILE> - <DESC>Scene element out-of-bounds composition policy</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
