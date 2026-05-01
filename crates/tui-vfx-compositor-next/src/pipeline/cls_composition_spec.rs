// <FILE>tui-vfx-compositor-next/src/pipeline/cls_composition_spec.rs</FILE>
// <DESC>Serializable composition spec for render pipeline</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Add shadow and preserve_unfilled to CompositionSpec</WCTX>
// <CLOG>0.4.0: add ordered sampler chains while preserving the legacy single-sampler field.</CLOG>

use crate::pipeline::{
    cls_composition_playback_timing::CompositionPlaybackTiming,
    cls_shader_layer_spec::ShaderLayerSpec,
};
use crate::types::{FilterSpec, MaskCombineMode, MaskSpec, SamplerSpec, ShadowSpec};
use mixed_signals::traits::Phase;
use serde::{Deserialize, Serialize};
use tui_vfx_style::models::{TryLowerV3SpatialShaderError, VfxSpatialShaderFamily};
use tui_vfx_style::traits::ShaderRuntimeParams;

/// Serializable composition specification for render pipeline bindings.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct CompositionSpec {
    /// Legacy single-sampler specification.
    ///
    /// Kept for backwards compatibility with older callers. New callers should
    /// prefer [`CompositionSpec::samplers`] when sampler ordering matters. If
    /// `samplers` is non-empty, it is the authoritative ordered chain and this
    /// field is treated as a compatibility mirror.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sampler_spec: Option<SamplerSpec>,

    /// Ordered sampler chain applied before masks, shaders, and filters.
    ///
    /// Each sampler consumes the coordinates produced by the previous sampler.
    /// If any sampler rejects a cell, the cell is skipped. When this vector is
    /// empty, the legacy `sampler_spec` field remains the effective single
    /// sampler for backwards compatibility.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub samplers: Vec<SamplerSpec>,

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

    /// Render-time runtime parameter map exposed to spatial shaders.
    #[serde(default, skip_serializing, skip_deserializing)]
    #[config(opaque)]
    pub runtime_params: ShaderRuntimeParams,
}

impl Default for CompositionSpec {
    fn default() -> Self {
        Self {
            sampler_spec: None,
            samplers: Vec::new(),
            masks: Vec::new(),
            mask_combine_mode: MaskCombineMode::All,
            filters: Vec::new(),
            shader_layers: Vec::new(),
            shadow: None,
            preserve_unfilled: true,
            t: 0.0,
            loop_t: None,
            phase: None,
            runtime_params: ShaderRuntimeParams::default(),
        }
    }
}

impl CompositionSpec {
    /// Return the effective ordered sampler chain.
    ///
    /// `samplers` is authoritative when populated. Otherwise the legacy
    /// `sampler_spec` is exposed as a one-item chain.
    pub fn effective_samplers(&self) -> Vec<SamplerSpec> {
        if !self.samplers.is_empty() {
            return self.samplers.clone();
        }
        self.sampler_spec.iter().cloned().collect()
    }

    /// True when the effective sampler chain contains at least one active
    /// sampler.
    pub fn has_active_sampler(&self) -> bool {
        self.effective_samplers()
            .iter()
            .any(|sampler| !matches!(sampler, SamplerSpec::None))
    }

    /// Push a sampler into the ordered sampler chain.
    ///
    /// The first sampler is mirrored into `sampler_spec` so older readers still
    /// see that a sampler is configured while newer readers use `samplers` for
    /// the full ordered chain.
    pub fn push_sampler(&mut self, sampler: SamplerSpec) {
        if self.sampler_spec.is_none() {
            self.sampler_spec = Some(sampler.clone());
        }
        self.samplers.push(sampler);
    }

    /// Apply one shared playback timing bundle to this composition spec.
    pub fn apply_playback_timing(&mut self, timing: CompositionPlaybackTiming) {
        self.t = timing.t;
        self.loop_t = timing.loop_t;
        self.phase = timing.phase;
    }

    /// Convenience builder for attaching shared playback timing.
    pub fn with_playback_timing(mut self, timing: CompositionPlaybackTiming) -> Self {
        self.apply_playback_timing(timing);
        self
    }

    /// Push one grouped V3 spatial family into this composition spec by
    /// lowering it through the executable legacy shader surface.
    pub fn try_push_v3_shader_family(
        &mut self,
        family: &VfxSpatialShaderFamily,
        region: tui_vfx_style::models::StyleRegion,
    ) -> Result<(), TryLowerV3SpatialShaderError> {
        self.shader_layers
            .push(ShaderLayerSpec::try_from_v3_shader_family(family, region)?);
        Ok(())
    }

    /// Convenience builder for appending one grouped V3 spatial family.
    pub fn try_with_v3_shader_family(
        mut self,
        family: &VfxSpatialShaderFamily,
        region: tui_vfx_style::models::StyleRegion,
    ) -> Result<Self, TryLowerV3SpatialShaderError> {
        self.try_push_v3_shader_family(family, region)?;
        Ok(self)
    }

    /// Returns the grouped V3 family form of every spatial shader layer in this
    /// composition spec.
    pub fn v3_shader_families(&self) -> Vec<VfxSpatialShaderFamily> {
        self.shader_layers
            .iter()
            .map(ShaderLayerSpec::v3_shader_family)
            .collect()
    }
}

fn default_preserve_unfilled() -> bool {
    true
}

// <FILE>tui-vfx-compositor-next/src/pipeline/cls_composition_spec.rs</FILE>
// <DESC>Serializable composition spec for render pipeline</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
