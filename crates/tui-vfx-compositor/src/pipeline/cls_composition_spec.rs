// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_spec.rs</FILE>
// <DESC>Serializable composition spec for render pipeline</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add shadow and preserve_unfilled to CompositionSpec</WCTX>
// <CLOG>Include shadow/preserve_unfilled fields for spec parity</CLOG>

use crate::pipeline::cls_shader_layer_spec::ShaderLayerSpec;
use crate::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec, ShadowSpec};
use mixed_signals::traits::Phase;
use serde::{Deserialize, Serialize};

/// Serializable composition specification for render pipeline bindings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CompositionSpec {
    /// Sampler specification (single - chaining deferred to future PRD).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampler_spec: Option<SamplerSpec>,

    /// Mask specifications - combined via mask_combine_mode.
    #[serde(default)]
    pub masks: Vec<MaskSpec>,

    /// How to combine multiple masks (default: All/AND).
    #[serde(default)]
    pub mask_combine_mode: MaskCombineMode,

    /// Filter specifications - applied in order (left to right).
    #[serde(default)]
    pub filters: Vec<FilterSpec>,

    /// Style shader layers with per-shader region targeting.
    #[serde(default)]
    pub shader_layers: Vec<ShaderLayerSpec>,

    /// Optional shadow spec (same as runtime CompositionOptions).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[config(opaque)]
    pub shadow: Option<ShadowSpec>,

    /// Preserve unfilled cells when applying masks.
    #[serde(default = "default_preserve_unfilled")]
    pub preserve_unfilled: bool,

    /// Animation progress (0.0 to 1.0) - phase-based time.
    #[serde(default)]
    pub t: f64,

    /// Cyclical loop time (0.0-1.0, repeating) for continuous effects.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub loop_t: Option<f64>,

    /// Current animation phase (Entering/Dwelling/Exiting/Finished).
    /// Runtime-only for now (not part of JSON spec).
    #[serde(default, skip_serializing, skip_deserializing)]
    #[config(opaque)]
    pub phase: Option<Phase>,
}

impl Default for CompositionSpec {
    fn default() -> Self {
        Self {
            sampler_spec: None,
            masks: Vec::new(),
            mask_combine_mode: MaskCombineMode::All,
            filters: Vec::new(),
            shader_layers: Vec::new(),
            shadow: None,
            preserve_unfilled: true,
            t: 0.0,
            loop_t: None,
            phase: None,
        }
    }
}

fn default_preserve_unfilled() -> bool {
    true
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_spec.rs</FILE>
// <DESC>Serializable composition spec for render pipeline</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
