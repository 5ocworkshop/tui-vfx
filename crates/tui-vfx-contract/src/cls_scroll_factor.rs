// <FILE>crates/tui-vfx-contract/src/cls_scroll_factor.rs</FILE> - <DESC>Per-element scene scroll response factor</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 pre-release schema reservation: preserve per-element parallax/depth scroll intent without backend execution.</WCTX>
// <CLOG>0.1.0: INIT — add optional typed scroll response factor for recipe scene elements.</CLOG>

/// Per-element motion rate relative to a future scene-level scroll or camera signal.
///
/// When present on a recipe scene element, `x` and `y` are axis-local multipliers
/// for future camera/scroll deltas. Absence means the element makes no explicit
/// parallax commitment and existing backends keep current lockstep behavior.
#[derive(
    Clone, Copy, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ScrollFactor {
    /// Horizontal multiplier relative to future scene/camera scroll deltas.
    pub x: f32,
    /// Vertical multiplier relative to future scene/camera scroll deltas.
    pub y: f32,
}

// <FILE>crates/tui-vfx-contract/src/cls_scroll_factor.rs</FILE> - <DESC>Per-element scene scroll response factor</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
