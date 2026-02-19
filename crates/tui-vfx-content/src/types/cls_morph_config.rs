// <FILE>tui-vfx-content/src/types/cls_morph_config.rs</FILE> - <DESC>Configuration types for text morph effect</DESC>
// <VERS>VERSION: 1.6.0</VERS>
// <WCTX>Effect parity: Content transformers</WCTX>
// <CLOG>Added DensityConceal variant for reverse density reveal</CLOG>

use serde::{Deserialize, Serialize};

/// How characters transition during morph.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MorphProgression {
    /// Characters switch one at a time in order
    #[default]
    Linear,
    /// Characters switch in random order (deterministic based on seed)
    Scatter,
    /// Characters switch in a wave pattern
    Wave,
    /// Show density block characters (░▒▓█) during transition (wave pattern)
    Density,
    /// Show binary digits (0/1) during transition
    Binary,
    /// Show braille characters (⠀→⣿) during transition (wave pattern)
    Braille,
    /// Cell-level density reveal: each cell cycles ░▒▓█ → target simultaneously
    DensityReveal,
    /// Cell-level density conceal: each cell cycles █▓▒░ → target simultaneously (reverse)
    DensityConceal,
    /// Cell-level braille reveal: each cell cycles ⠀→⣿ → target simultaneously (fill up)
    BrailleReveal,
    /// Cell-level braille reveal: each cell cycles ⣿→⠀ → target simultaneously (fill down)
    BrailleRevealDown,
    /// Braille wave sweeps across text (linear, dots fill up ⠀→⣿)
    BrailleWaveUp,
    /// Braille wave sweeps across text (linear, dots empty ⣿→⠀)
    BrailleWaveDown,
    /// Braille wave with random per-character timing (dots fill up)
    BrailleRandomUp,
    /// Braille wave with random per-character timing (dots empty)
    BrailleRandomDown,
    /// Braille reveal word-by-word (dots fill up then reveal)
    BrailleByWordUp,
    /// Braille reveal word-by-word (dots empty then reveal)
    BrailleByWordDown,
    /// Braille reveal line-by-line (dots fill up then reveal)
    BrailleByLineUp,
    /// Braille reveal line-by-line (dots empty then reveal)
    BrailleByLineDown,
    /// Half-cell wipe using braille columns for sub-character resolution
    BrailleHalfCellWipe,
    /// Half-cell wipe by word using braille columns
    BrailleHalfCellWipeByWord,
}

/// Direction for the morph transition.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum MorphDirection {
    /// Morph from left to right
    #[default]
    LeftToRight,
    /// Morph from right to left
    RightToLeft,
    /// All characters morph simultaneously
    Simultaneous,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_defaults() {
        assert_eq!(MorphProgression::default(), MorphProgression::Linear);
        assert_eq!(MorphDirection::default(), MorphDirection::LeftToRight);
    }
}

// <FILE>tui-vfx-content/src/types/cls_morph_config.rs</FILE> - <DESC>Configuration types for text morph effect</DESC>
// <VERS>END OF VERSION: 1.6.0</VERS>
