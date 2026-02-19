// <FILE>tui-vfx-style/src/utils/mod.rs</FILE> - <DESC>Utils module definition</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>OFPF refactoring: extract style helpers from cls_style_effect.rs</WCTX>
// <CLOG>Add fnc_style_blend, fnc_style_hsl, fnc_style_rainbow modules</CLOG>

pub mod fnc_blend_colors;
pub mod fnc_color_ops;
pub mod fnc_easing;
pub mod fnc_style_blend;
pub mod fnc_style_hsl;
pub mod fnc_style_rainbow;

pub use fnc_blend_colors::blend_colors;
pub use fnc_blend_colors::{hsl_to_rgb, rgb_to_hsl, to_rgb_tuple};
pub use fnc_color_ops::{darken, degrade_color, rgb_to_indexed};
pub use fnc_easing::apply_easing;
pub use fnc_style_blend::{blend_style_to_color, blend_style_to_color_in_space};
pub use fnc_style_hsl::{shift_color_hsl, shift_style_hsl};
pub use fnc_style_rainbow::{rainbow_color, rainbow_style};

// <FILE>tui-vfx-style/src/utils/mod.rs</FILE> - <DESC>Utils module definition</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
