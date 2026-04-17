// <FILE>tui-vfx-content/src/types/cls_glyph_cascade.rs</FILE> - <DESC>Configuration types for glyph-cascade content effect</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>New richer evolve-like content transformer</WCTX>
// <CLOG>Add glyph alphabets, progression modes, and reveal ordering for GlyphCascade content effects</CLOG>

use serde::{Deserialize, Serialize};

/// Glyph alphabet used by [`ContentEffect::GlyphCascade`].
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum GlyphCascadeAlphabet {
    BlocksHorizontal,
    BlocksVertical,
    CircleFill,
    #[default]
    Circles,
    Quadrants,
    Shaded,
    Squares,
    BrailleRise,
    BrailleFall,
    Custom {
        glyphs: String,
    },
}

impl GlyphCascadeAlphabet {
    pub fn glyphs(&self) -> Vec<char> {
        match self {
            Self::BlocksHorizontal => vec![' ', '▏', '▎', '▍', '▌', '▋', '▊', '▉', '█'],
            Self::BlocksVertical => vec![' ', '▁', '▂', '▃', '▄', '▅', '▆', '▇', '█'],
            Self::CircleFill => vec![' ', '◌', '◎', '◍', '●'],
            Self::Circles => vec![' ', '·', '•', '◉', '●'],
            Self::Quadrants => vec![' ', '▖', '▘', '▗', '▝', '▚', '▞', '▙', '▛', '▜', '▟', '█'],
            Self::Shaded => vec![' ', '░', '▒', '▓', '█'],
            Self::Squares => vec![' ', '·', '▫', '▪', '◼', '█'],
            Self::BrailleRise => vec![' ', '⠁', '⠃', '⠇', '⠧', '⠷', '⠿'],
            Self::BrailleFall => vec![' ', '⢀', '⢄', '⢆', '⢇', '⢏', '⢟', '⢿'],
            Self::Custom { glyphs } => {
                let chars: Vec<char> = glyphs.chars().collect();
                if chars.is_empty() { vec![' '] } else { chars }
            }
        }
    }
}

/// Ordering pattern for glyph cascade activation.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum GlyphCascadePattern {
    #[default]
    Sequential,
    Random,
    EdgeIn,
    EdgeOut,
    ByWord,
    ByLine,
}

/// How glyph cascade interacts with the target text.
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum GlyphCascadeMode {
    /// Reveal from glyphs into the target text.
    #[default]
    IntoTarget,
    /// Start at the target text and destabilize outward into glyphs.
    FromTarget,
    /// Stay in the glyph alphabet for the full effect (final state is the last glyph).
    GlyphsOnly,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn custom_alphabet_falls_back_to_space_when_empty() {
        assert_eq!(
            GlyphCascadeAlphabet::Custom {
                glyphs: String::new()
            }
            .glyphs(),
            vec![' ']
        );
    }
}

// <FILE>tui-vfx-content/src/types/cls_glyph_cascade.rs</FILE> - <DESC>Configuration types for glyph-cascade content effect</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
