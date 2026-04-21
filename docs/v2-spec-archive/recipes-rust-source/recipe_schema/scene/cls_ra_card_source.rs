// <FILE>src/recipe_schema/scene/cls_ra_card_source.rs</FILE> - <DESC>Composite card content source for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema type for card layers composed from a fill color, border config, padding, and nested text source.</WCTX>
// <CLOG>0.1.0: add RaCardSource.</CLOG>

use serde::{Deserialize, Serialize};
use tui_vfx_types::Color;

use crate::recipe_schema::{RaBorderConfig, RaPaddingConfig};

use super::cls_ra_text_source::RaTextSource;

#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaCardSource {
    #[config(opaque)]
    pub bg_fill: Color,
    #[serde(default)]
    pub border_style: RaBorderConfig,
    #[serde(default)]
    pub padding: RaPaddingConfig,
    #[serde(default)]
    pub text: RaTextSource,
}

impl Default for RaCardSource {
    fn default() -> Self {
        Self {
            bg_fill: Color::BLACK,
            border_style: RaBorderConfig::default(),
            padding: RaPaddingConfig::default(),
            text: RaTextSource::default(),
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_card_source.rs</FILE> - <DESC>RaCardSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
