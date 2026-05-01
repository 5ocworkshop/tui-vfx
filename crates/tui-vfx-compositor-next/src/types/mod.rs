// <FILE>tui-vfx-compositor-next/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>VERSION: 2.12.0</VERS>
// <WCTX>Compositor-first v3.1 lowering: expose mask payload types for backend adapters.</WCTX>
// <CLOG>2.12.0: MINOR — re-export RevealPathType and SpiralDirection so player backend lowerers can construct MaskSpec::PathReveal.
// 2.11.0: re-export CellularPattern so player backend lowerers can construct MaskSpec::Cellular without touching mask internals.
// 2.10.0: re-export GlyphEncoderSpec and SamplerRef from cls_filter_spec.</CLOG>

pub mod cls_bindable_value;
pub mod cls_filter_spec;
pub mod cls_hover_bar_position;
pub mod cls_mask_spec;
pub mod cls_sampler_spec;
pub mod cls_shadow_spec;
pub mod mask_combine_mode;

pub use crate::masks::cls_cellular::CellularPattern;
pub use crate::masks::cls_path_reveal::{RevealPathType, SpiralDirection};
pub use crate::masks::cls_radial::RadialOrigin;
pub use cls_bindable_value::BindableValue;
pub use cls_filter_spec::{
    AnimatedGlyphRampAffect, AnimatedGlyphRampApplyTo, ApplyTo, BraillePatternType, FilterSpec,
    GlyphEncoderSpec, GlyphRecolorSpec, MatrixRainAffect, MatrixRainCharsetPreset, MatrixRainMode,
    MotionBlurDirection, PatternType, SamplerRef, SubPixelBarDirection,
};
pub use cls_hover_bar_position::HoverBarPosition;
pub use cls_mask_spec::{
    DitherMatrix, IrisShape, MaskSpec, Materialize, Orientation, ResolvedWipe, WipeDirection,
};
pub use cls_sampler_spec::{Axis, RippleCenter, SamplerSpec};
pub use cls_shadow_spec::ShadowSpec;
pub use mask_combine_mode::MaskCombineMode;

// <FILE>tui-vfx-compositor-next/src/types/mod.rs</FILE> - <DESC>Types module</DESC>
// <VERS>END OF VERSION: 2.12.0</VERS>
