// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_options.rs</FILE> - <DESC>Composition options for render pipeline</DESC>
// <VERS>VERSION: 3.1.0</VERS>
// <WCTX>Shadow corner transparency handling</WCTX>
// <CLOG>Add preserve_unfilled flag to control shadow corner bleed-through behavior</CLOG>

use crate::types::MaskCombineMode;
use crate::types::cls_filter_spec::FilterSpec;
use crate::types::cls_mask_spec::MaskSpec;
use crate::types::cls_sampler_spec::SamplerSpec;
use crate::types::cls_shadow_spec::ShadowSpec;
use mixed_signals::traits::Phase;
use smallvec::SmallVec;
use std::borrow::Cow;
use tui_vfx_style::models::StyleRegion;
use tui_vfx_style::traits::StyleShader;

/// A shader paired with its region constraint.
/// Each shader can target a different region (e.g., BorderOnly, TextOnly, Rows).
pub struct ShaderWithRegion<'a> {
    pub shader: &'a dyn StyleShader,
    pub region: StyleRegion,
}

/// Composition options with full spec support.
///
/// This version supports multiple masks, filters, shaders, and shadows per stage.
/// SmallVec is used for inline allocation to avoid heap allocation for common cases.
///
/// ## Shadow Support
///
/// When `shadow` is set, the compositor renders a shadow that:
/// - Extends beyond the element dimensions by the shadow offset
/// - Receives the same mask treatment as the element (wipes, dissolves, etc.)
/// - Is rendered before the element so it appears behind it
///
/// **Important:** The total rendered area is larger than the source dimensions:
/// - Total width = element width + |shadow.offset_x|
/// - Total height = element height + |shadow.offset_y|
///
/// For example, a 30x12 element with shadow offset (2, 1) renders to a 32x13 area.
pub struct CompositionOptions<'a> {
    /// Sampler specification (single - chaining deferred to future PRD)
    pub sampler_spec: Option<SamplerSpec>,

    /// Mask specifications - combined via mask_combine_mode.
    pub masks: Cow<'a, [MaskSpec]>,

    /// How to combine multiple masks (default: All/AND).
    pub mask_combine_mode: MaskCombineMode,

    /// Filter specifications - applied in order (left to right).
    pub filters: Cow<'a, [FilterSpec]>,

    /// Style shaders with per-shader region targeting.
    /// Each shader can target different regions (BorderOnly, TextOnly, Rows, etc.).
    /// SmallVec<[ShaderWithRegion; 2]> avoids heap allocation for ≤2 shaders.
    pub shader_layers: SmallVec<[ShaderWithRegion<'a>; 2]>,

    /// Shadow specification for compositor-integrated shadow rendering.
    ///
    /// When set, the compositor will:
    /// 1. Extend the render area to include the shadow
    /// 2. Apply masks to both shadow and element regions
    /// 3. Render shadow first, then element on top
    ///
    /// The shadow wipes/dissolves/fades in sync with the element.
    pub shadow: Option<ShadowSpec>,

    /// Whether to preserve destination content where the compositor didn't render.
    ///
    /// When `true` (default), unfilled cells in the shadow's extended area (such as
    /// the upper-right and lower-left corners for a bottom-right shadow) will NOT
    /// overwrite the destination, allowing underlying content to show through.
    ///
    /// When `false`, unfilled cells will be written as transparent, which may
    /// result in visual artifacts depending on how the adapter handles transparency.
    ///
    /// This is primarily relevant for shadow rendering where the shadow geometry
    /// doesn't fill the entire extended buffer area.
    pub preserve_unfilled: bool,

    /// Animation progress (0.0 to 1.0) - phase-based time
    pub t: f64,

    /// Cyclical loop time (0.0-1.0, repeating) for continuous effects
    pub loop_t: Option<f64>,

    /// Current animation phase (Entering/Dwelling/Exiting/Finished)
    pub phase: Option<Phase>,
}

impl Default for CompositionOptions<'_> {
    fn default() -> Self {
        Self {
            sampler_spec: None,
            masks: Cow::Borrowed(&[]),
            mask_combine_mode: MaskCombineMode::All,
            filters: Cow::Borrowed(&[]),
            shader_layers: SmallVec::new(),
            shadow: None,
            preserve_unfilled: true,
            t: 0.0,
            loop_t: None,
            phase: None,
        }
    }
}

impl<'a> CompositionOptions<'a> {
    /// Add a single mask (convenience method).
    pub fn with_mask(mut self, mask: MaskSpec) -> Self {
        match &mut self.masks {
            Cow::Borrowed([]) => {
                self.masks = Cow::Owned(vec![mask]);
            }
            Cow::Borrowed(existing) => {
                let mut owned = existing.to_vec();
                owned.push(mask);
                self.masks = Cow::Owned(owned);
            }
            Cow::Owned(existing) => {
                existing.push(mask);
            }
        }
        self
    }

    /// Add multiple masks.
    pub fn with_masks(mut self, masks: impl Into<Cow<'a, [MaskSpec]>>) -> Self {
        self.masks = masks.into();
        self
    }

    /// Set the mask combine mode.
    pub fn with_mask_combine_mode(mut self, mode: MaskCombineMode) -> Self {
        self.mask_combine_mode = mode;
        self
    }

    /// Add a single filter (convenience method).
    pub fn with_filter(mut self, filter: FilterSpec) -> Self {
        match &mut self.filters {
            Cow::Borrowed([]) => {
                self.filters = Cow::Owned(vec![filter]);
            }
            Cow::Borrowed(existing) => {
                let mut owned = existing.to_vec();
                owned.push(filter);
                self.filters = Cow::Owned(owned);
            }
            Cow::Owned(existing) => {
                existing.push(filter);
            }
        }
        self
    }

    /// Add multiple filters.
    pub fn with_filters(mut self, filters: impl Into<Cow<'a, [FilterSpec]>>) -> Self {
        self.filters = filters.into();
        self
    }

    /// Add a shader with its region constraint.
    pub fn with_shader_layer(mut self, shader: &'a dyn StyleShader, region: StyleRegion) -> Self {
        self.shader_layers.push(ShaderWithRegion { shader, region });
        self
    }

    /// Set the shadow specification.
    ///
    /// When set, the compositor will render a shadow that extends beyond the
    /// element dimensions and receives the same mask treatment as the element.
    ///
    /// **Note:** This increases the rendered area by the shadow offset.
    /// A 30x12 element with offset (2, 1) renders to a 32x13 area.
    pub fn with_shadow(mut self, shadow: impl Into<ShadowSpec>) -> Self {
        self.shadow = Some(shadow.into());
        self
    }

    /// Control whether unfilled cells preserve destination content.
    ///
    /// Default is `true`, which allows underlying content to show through in
    /// shadow corner regions that aren't covered by the shadow geometry.
    pub fn with_preserve_unfilled(mut self, preserve: bool) -> Self {
        self.preserve_unfilled = preserve;
        self
    }
}

// <FILE>tui-vfx-compositor/src/pipeline/cls_composition_options.rs</FILE> - <DESC>Composition options for render pipeline</DESC>
// <VERS>END OF VERSION: 3.1.0</VERS>
