// <FILE>tui-vfx-compositor/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>VERSION: 2.9.0</VERS>
// <WCTX>Phase 0 P0.1 — progress_binding infrastructure</WCTX>
// <CLOG>Add cls_bindable_value module and BindableValue re-export</CLOG>

pub mod cls_bindable_value;
pub mod cls_filter_spec;
pub mod cls_hover_bar_position;
pub mod cls_mask_spec;
pub mod cls_sampler_spec;
pub mod cls_shadow_spec;
pub mod mask_combine_mode;

pub use crate::masks::cls_radial::RadialOrigin;
pub use cls_bindable_value::BindableValue;
pub use cls_filter_spec::{
    ApplyTo, BraillePatternType, FilterSpec, MatrixRainAffect, MatrixRainCharsetPreset,
    MatrixRainMode, MotionBlurDirection, PatternType, SubPixelBarDirection,
};
pub use cls_hover_bar_position::HoverBarPosition;
pub use cls_mask_spec::{
    DitherMatrix, IrisShape, MaskSpec, Materialize, Orientation, ResolvedWipe, WipeDirection,
};
pub use cls_sampler_spec::{Axis, RippleCenter, SamplerSpec};
pub use cls_shadow_spec::ShadowSpec;
pub use mask_combine_mode::MaskCombineMode;

// <FILE>tui-vfx-compositor/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>END OF VERSION: 2.9.0</VERS>
