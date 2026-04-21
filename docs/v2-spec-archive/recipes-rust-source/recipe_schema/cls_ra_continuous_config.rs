// <FILE>src/recipe_schema/cls_ra_continuous_config.rs</FILE> - <DESC>Cross-phase continuous effect block for recipes</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan B Phase B.1 — additive schema type mirroring the real RaPipelineConfig effect groups for cross-phase continuous effects with a unified clock selector.</WCTX>
// <CLOG>0.1.0: add RaContinuousConfig.</CLOG>

use serde::{Deserialize, Serialize};

use crate::recipe_schema::{RaFilterConfig, RaMaskConfig, RaSamplerConfig, RaStylePipelineConfig};

use super::enum_ra_clock::RaClock;

#[non_exhaustive]
#[derive(Debug, Clone, tui_vfx_core::ConfigSchema, Serialize, Deserialize)]
#[serde(default, deny_unknown_fields)]
pub struct RaContinuousConfig {
    #[serde(default)]
    pub mask: RaMaskConfig,
    #[serde(default)]
    pub sampler: RaSamplerConfig,
    #[serde(default)]
    pub filter: RaFilterConfig,
    #[serde(default)]
    pub styles: Vec<RaStylePipelineConfig>,
    #[serde(default)]
    pub clock: RaClock,
}

impl Default for RaContinuousConfig {
    fn default() -> Self {
        Self {
            mask: RaMaskConfig::default(),
            sampler: RaSamplerConfig::default(),
            filter: RaFilterConfig::default(),
            styles: Vec::new(),
            clock: RaClock::LoopT,
        }
    }
}

// <FILE>src/recipe_schema/cls_ra_continuous_config.rs</FILE> - <DESC>RaContinuousConfig</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
