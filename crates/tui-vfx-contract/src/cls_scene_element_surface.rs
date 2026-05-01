// <FILE>crates/tui-vfx-contract/src/cls_scene_element_surface.rs</FILE> - <DESC>Scene element surface envelope DTO</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>v3.1 scene parity: preserve layer-local base style and attached shadow payloads.</WCTX>
// <CLOG>0.2.0: MINOR — promote shadow from structured payload to typed surface attachment.
// 0.1.0: INIT — add structured base-style and shadow preservation fields.</CLOG>

use crate::{ShadowSpec, StructuredValue};

/// Element-local surface styling envelope.
///
/// This is the canonical v3.1 home for authored layer base style and attached
/// shadow details.
#[derive(Clone, Debug, PartialEq, serde::Serialize, serde::Deserialize, schemars::JsonSchema)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SceneElementSurface {
    /// Optional base style applied before the element-local pipeline.
    pub base_style: Option<StructuredValue>,
    /// Optional attached shadow payload owned by this element surface.
    pub shadow: Option<ShadowSpec>,
}

// <FILE>crates/tui-vfx-contract/src/cls_scene_element_surface.rs</FILE> - <DESC>SceneElementSurface</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
