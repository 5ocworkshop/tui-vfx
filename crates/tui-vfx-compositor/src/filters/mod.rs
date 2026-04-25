// <FILE>tui-vfx-compositor/src/filters/mod.rs</FILE> - <DESC>Filter implementations (internal)</DESC>
// <VERS>VERSION: 2.9.0</VERS>
// <WCTX>Audit recommendation 1.6 — register the new cls_animated_glyph_ramp filter that synchronises glyph + colour cycling from one shared phase signal, closing the synthesis gap that prevents faithful TTE Waves / Sweep / SynthGrid recreation.</WCTX>
// <CLOG>2.9.0: MINOR — register cls_animated_glyph_ramp alongside the existing 31 filters.
// 2.8.0: register cls_glyph_style alongside the existing 30 filters</CLOG>

pub(crate) mod cls_animated_glyph_ramp;
pub(crate) mod cls_bracket_emphasis;
pub(crate) mod cls_braille_dust;
pub(crate) mod cls_charset_noise;
pub(crate) mod cls_color_bridged_shade;
pub(crate) mod cls_crt;
pub(crate) mod cls_dim;
pub(crate) mod cls_dot_indicator;
pub(crate) mod cls_edge_grow;
pub(crate) mod cls_fade_to_canvas;
pub(crate) mod cls_glisten_sweep;
pub(crate) mod cls_glyph_style;
pub(crate) mod cls_greyscale;
pub(crate) mod cls_hover_bar;
pub(crate) mod cls_interlace_curtain;
pub(crate) mod cls_invert;
pub(crate) mod cls_kitt_scanner;
pub(crate) mod cls_matrix_rain;
pub(crate) mod cls_motion_blur;
pub(crate) mod cls_pattern_fill;
pub(crate) mod cls_pill_button;
pub(crate) mod cls_rigid_shake;
pub(crate) mod cls_shade_scanner;
pub(crate) mod cls_sub_cell_shake;
pub(crate) mod cls_sub_pixel_bar;
pub(crate) mod cls_subcell_light;
pub(crate) mod cls_tint;
pub(crate) mod cls_underline_wipe;
pub(crate) mod cls_vignette;

#[cfg(test)]
pub(crate) mod test_support;

// <FILE>tui-vfx-compositor/src/filters/mod.rs</FILE> - <DESC>Filter implementations (internal)</DESC>
// <VERS>END OF VERSION: 2.7.0</VERS>
