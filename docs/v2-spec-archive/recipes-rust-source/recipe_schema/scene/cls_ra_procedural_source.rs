// <FILE>src/recipe_schema/scene/cls_ra_procedural_source.rs</FILE> - <DESC>Procedural content source for scene layers</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive scene schema type for procedural layer sources keyed by source_id + params.</WCTX>
// <CLOG>0.1.0: add RaProceduralSource.</CLOG>

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaProceduralSource {
    pub source_id: String,
    #[config(opaque)]
    #[serde(default)]
    pub params: Value,
}

impl Default for RaProceduralSource {
    fn default() -> Self {
        Self {
            source_id: String::new(),
            params: Value::Object(Default::default()),
        }
    }
}

// <FILE>src/recipe_schema/scene/cls_ra_procedural_source.rs</FILE> - <DESC>RaProceduralSource</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
