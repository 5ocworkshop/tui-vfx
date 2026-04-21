// <FILE>src/recipe_schema/scene/cls_ra_image_source.rs</FILE> - <DESC>Image content source for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema type for authored image layers with optional tinting.</WCTX>
// <CLOG>0.1.0: add RaImageSource plus RaImageAspect.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum RaImageAspect {
    #[default]
    Fit,
    Fill,
    Stretch,
}

#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaImageSource {
    pub image_name: String,
    #[config(opaque)]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tint: Option<Color>,
    #[serde(default)]
    pub aspect: RaImageAspect,
}

impl Default for RaImageSource {
    fn default() -> Self {
        Self {
            image_name: String::new(),
            tint: None,
            aspect: RaImageAspect::Fit,
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_image_source.rs</FILE> - <DESC>RaImageSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
