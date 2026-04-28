// <FILE>crates/tui-vfx-next/src/cls_element_placement.rs</FILE> - <DESC>Element-local to scene-coordinate placement</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase D1: lock signed scene placement for partially visible elements.</WCTX>
// <CLOG>0.1.0: ADD — introduce schema-visible element placement in scene coordinates.</CLOG>

/// Placement of an element-local surface into scene coordinates.
#[derive(
    Clone, Copy, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize, schemars::JsonSchema,
)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct ElementPlacement {
    /// Scene x coordinate where element-local x=0 lands.
    pub x: i32,
    /// Scene y coordinate where element-local y=0 lands.
    pub y: i32,
}

impl ElementPlacement {
    /// Create a placement from a scene x/y coordinate.
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

// <FILE>crates/tui-vfx-next/src/cls_element_placement.rs</FILE> - <DESC>Element-local to scene-coordinate placement</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
