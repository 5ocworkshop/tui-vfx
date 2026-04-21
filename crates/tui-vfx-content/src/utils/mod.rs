// <FILE>tui-vfx-content/src/utils/mod.rs</FILE> - <DESC>Utils module</DESC>
// <VERS>VERSION: 1.2.0</VERS>
// <WCTX>Register fnc_char_turn primitive for Unicode 180° glyph rotation; initial consumer is SplitFlap.flip_preview / flip_flicker but it's a reusable crate-level utility</WCTX>
// <CLOG>MINOR: add pub mod fnc_char_turn and re-export char_turn at the utils module root</CLOG>

pub mod fnc_char_turn;
pub mod fnc_graphemes;

pub use fnc_char_turn::char_turn;

// <FILE>tui-vfx-content/src/utils/mod.rs</FILE> - <DESC>Utils module</DESC>
// <VERS>END OF VERSION: 1.2.0</VERS>
