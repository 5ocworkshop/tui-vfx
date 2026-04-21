// <FILE>tui-vfx-content/src/transformers/mod.rs</FILE> - <DESC>Content transformers module</DESC>
// <VERS>VERSION: 1.14.0</VERS>
// <WCTX>Re-export SplitFlapDispersion alongside SplitFlap/SplitFlapCharset so recipe deserialization and downstream consumers can reference the dispersion enum by name</WCTX>
// <CLOG>MINOR: add SplitFlapDispersion to the re-export list from cls_split_flap</CLOG>

pub mod cls_dissolve;
pub mod cls_glitch_shift;
pub mod cls_glyph_cascade;
pub mod cls_marquee;
pub mod cls_mirror;
pub mod cls_morph;
pub mod cls_numeric;
pub mod cls_odometer;
pub mod cls_redact;
pub mod cls_scramble;
pub mod cls_scramble_glitch_shift;
pub mod cls_slide_shift;
pub mod cls_split_flap;
pub mod cls_typewriter;
pub mod cls_wrap_indicator;
pub mod fnc_get_transformer;
pub mod fnc_morph_chars;

pub use cls_dissolve::Dissolve;
pub use cls_glitch_shift::GlitchShift;
pub use cls_glyph_cascade::GlyphCascade;
pub use cls_marquee::Marquee;
pub use cls_mirror::Mirror;
pub use cls_morph::Morph;
pub use cls_numeric::Numeric;
pub use cls_odometer::Odometer;
pub use cls_redact::Redact;
pub use cls_scramble::Scramble;
pub use cls_scramble_glitch_shift::ScrambleGlitchShift;
pub use cls_slide_shift::SlideShift;
pub use cls_split_flap::{SplitFlap, SplitFlapCharset, SplitFlapDispersion};
pub use cls_typewriter::Typewriter;
pub use cls_wrap_indicator::WrapIndicator;
pub use fnc_get_transformer::get_transformer;

// <FILE>tui-vfx-content/src/transformers/mod.rs</FILE> - <DESC>Content transformers module</DESC>
// <VERS>END OF VERSION: 1.13.0</VERS>
