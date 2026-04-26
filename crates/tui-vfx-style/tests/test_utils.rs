// <FILE>tui-vfx-style/tests/test_utils.rs</FILE> - <DESC>Linker file for utils tests</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>TTE effects port — wire test_fnc_brighten_hct into the utils linker so HCT brightness coverage runs with the standard suite.</WCTX>
// <CLOG>0.4.0: link test_fnc_brighten_hct.</CLOG>

#[path = "utils/test_fnc_blend_colors.rs"]
mod test_fnc_blend_colors;
#[path = "utils/test_fnc_brighten_hct.rs"]
mod test_fnc_brighten_hct;
#[path = "utils/test_fnc_color_ops.rs"]
mod test_fnc_color_ops;
#[path = "utils/test_fnc_easing.rs"]
mod test_fnc_easing;

// <FILE>tui-vfx-style/tests/test_utils.rs</FILE> - <DESC>Linker file for utils tests</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
