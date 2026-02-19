// <FILE>tui-vfx-compositor/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>VERSION: 2.8.0</VERS>
// <WCTX>Hover indicator effects implementation</WCTX>
// <CLOG>Add HoverBarPosition type for hover/focus indicator positioning</CLOG>

pub mod cls_filter_spec;
pub mod cls_hover_bar_position;
pub mod cls_mask_spec;
pub mod cls_sampler_spec;
pub mod cls_shadow_spec;
pub mod mask_combine_mode;

pub use cls_filter_spec::{
    ApplyTo, BraillePatternType, FilterSpec, MotionBlurDirection, PatternType, SubPixelBarDirection,
};
pub use cls_hover_bar_position::HoverBarPosition;
pub use cls_mask_spec::{
    DitherMatrix, IrisShape, MaskSpec, Orientation, ResolvedWipe, WipeDirection,
};
pub use cls_sampler_spec::{Axis, RippleCenter, SamplerSpec};
pub use cls_shadow_spec::ShadowSpec;
pub use mask_combine_mode::MaskCombineMode;

// <FILE>tui-vfx-compositor/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>END OF VERSION: 2.8.0</VERS>
