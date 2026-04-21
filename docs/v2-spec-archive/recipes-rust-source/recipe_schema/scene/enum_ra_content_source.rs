// <FILE>src/recipe_schema/scene/enum_ra_content_source.rs</FILE> - <DESC>Tagged content-source enum for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema enum for text/image/procedural/card layer sources.</WCTX>
// <CLOG>0.1.0: add adjacently-tagged RaContentSource enum.</CLOG>

use serde::{Deserialize, Serialize};

use super::{RaCardSource, RaImageSource, RaProceduralSource, RaTextSource};

#[non_exhaustive]
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(tag = "type", content = "spec", rename_all = "snake_case")]
pub enum RaContentSource {
    Text(Box<RaTextSource>),
    Image(RaImageSource),
    Procedural(RaProceduralSource),
    Card(Box<RaCardSource>),
}

// <FILE>src/recipe_schema/scene/enum_ra_content_source.rs</FILE> - <DESC>RaContentSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
