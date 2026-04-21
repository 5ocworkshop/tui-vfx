// <FILE>src/recipe_schema/scene/cls_ra_text_source.rs</FILE> - <DESC>Static text content source for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema type for authored text layers with optional content effects and alignment.</WCTX>
// <CLOG>0.1.0: add RaTextSource plus RaTextAlignment.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_content::types::ContentEffect;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaTextAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaTextSource {
    pub string: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub content_effect: Option<ContentEffect>,
    #[serde(default)]
    pub alignment: RaTextAlignment,
}

impl Default for RaTextSource {
    fn default() -> Self {
        Self {
            string: String::new(),
            content_effect: None,
            alignment: RaTextAlignment::Left,
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_text_source.rs</FILE> - <DESC>RaTextSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
