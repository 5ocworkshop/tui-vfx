// <FILE>tui-vfx-style/src/models/cls_style_layer.rs</FILE> - <DESC>Style layer with region and phase effects</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>Multi-layer style support for per-region effects</WCTX>
// <CLOG>Initial implementation of StyleLayer for multi-region styling</CLOG>

use crate::models::{StyleEffect, StyleRegion};
use serde::{Deserialize, Serialize};

/// A style layer combines a region constraint with phase-specific effects.
///
/// Multiple layers can be stacked to achieve different effects on different
/// regions of a widget. For example:
/// - Layer 1: region=TextOnly, base styling for text
/// - Layer 2: region=BorderOnly, animated shader for border
///
/// Each layer can also have per-effect region overrides, allowing an effect
/// to target a different region than its layer's default.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct StyleLayer {
    /// Region constraint for this layer (default: All)
    #[serde(default)]
    pub region: StyleRegion,

    /// Effect applied during enter phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_effect: Option<StyleEffect>,

    /// Optional region override for enter effect
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter_region: Option<StyleRegion>,

    /// Effect applied during dwell phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dwell_effect: Option<StyleEffect>,

    /// Optional region override for dwell effect
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dwell_region: Option<StyleRegion>,

    /// Effect applied during exit phase
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_effect: Option<StyleEffect>,

    /// Optional region override for exit effect
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_region: Option<StyleRegion>,
}

impl StyleLayer {
    /// Create a new style layer with the given region.
    pub fn new(region: StyleRegion) -> Self {
        Self {
            region,
            ..Default::default()
        }
    }

    /// Set the enter effect with optional region override.
    pub fn with_enter(mut self, effect: StyleEffect, region: Option<StyleRegion>) -> Self {
        self.enter_effect = Some(effect);
        self.enter_region = region;
        self
    }

    /// Set the dwell effect with optional region override.
    pub fn with_dwell(mut self, effect: StyleEffect, region: Option<StyleRegion>) -> Self {
        self.dwell_effect = Some(effect);
        self.dwell_region = region;
        self
    }

    /// Set the exit effect with optional region override.
    pub fn with_exit(mut self, effect: StyleEffect, region: Option<StyleRegion>) -> Self {
        self.exit_effect = Some(effect);
        self.exit_region = region;
        self
    }

    /// Get the effective region for the enter effect.
    /// Uses the effect's region override if set, otherwise the layer's region.
    pub fn effective_enter_region(&self) -> &StyleRegion {
        self.enter_region.as_ref().unwrap_or(&self.region)
    }

    /// Get the effective region for the dwell effect.
    /// Uses the effect's region override if set, otherwise the layer's region.
    pub fn effective_dwell_region(&self) -> &StyleRegion {
        self.dwell_region.as_ref().unwrap_or(&self.region)
    }

    /// Get the effective region for the exit effect.
    /// Uses the effect's region override if set, otherwise the layer's region.
    pub fn effective_exit_region(&self) -> &StyleRegion {
        self.exit_region.as_ref().unwrap_or(&self.region)
    }
}

// <FILE>tui-vfx-style/src/models/cls_style_layer.rs</FILE> - <DESC>Style layer with region and phase effects</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
