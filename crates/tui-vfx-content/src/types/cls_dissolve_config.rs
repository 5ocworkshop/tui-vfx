// <FILE>tui-vfx-content/src/types/cls_dissolve_config.rs</FILE> - <DESC>Configuration types for text dissolve effect</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Effect parity: Content transformers</WCTX>
// <CLOG>Added Clustered, ByWord, ByLine patterns</CLOG>

use serde::{Deserialize, Serialize};

/// What to replace dissolved characters with.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DissolveReplacement {
    /// Replace with space character (text fades to empty)
    #[default]
    Space,
    /// Replace with dot character
    Dot,
    /// Replace with a specific custom character
    Custom(char),
}

impl DissolveReplacement {
    /// Get the replacement character.
    pub fn char(&self) -> char {
        match self {
            DissolveReplacement::Space => ' ',
            DissolveReplacement::Dot => '.',
            DissolveReplacement::Custom(c) => *c,
        }
    }
}

/// Pattern for how characters dissolve.
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum DissolvePattern {
    /// Characters dissolve in order based on direction
    #[default]
    Sequential,
    /// Characters dissolve in random order (deterministic based on seed)
    Random,
    /// Characters dissolve from edges toward center
    EdgeIn,
    /// Characters dissolve from center toward edges
    EdgeOut,
    /// Characters dissolve in organic clusters
    Clustered {
        /// Average size of each cluster (1-10 chars)
        #[serde(default = "default_cluster_size")]
        cluster_size: u8,
    },
    /// Word-by-word dissolve (spaces act as boundaries)
    ByWord,
    /// Line-by-line dissolve (newlines act as boundaries)
    ByLine,
}

fn default_cluster_size() -> u8 {
    3
}

/// Direction for sequential dissolve pattern.
#[derive(
    Debug, Clone, Copy, PartialEq, Default, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum DissolveDirection {
    /// Dissolve from left to right
    #[default]
    LeftToRight,
    /// Dissolve from right to left
    RightToLeft,
    /// Dissolve from both ends toward center
    CenterIn,
    /// Dissolve from center toward both ends
    CenterOut,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_replacement_char() {
        assert_eq!(DissolveReplacement::Space.char(), ' ');
        assert_eq!(DissolveReplacement::Dot.char(), '.');
        assert_eq!(DissolveReplacement::Custom('█').char(), '█');
    }

    #[test]
    fn test_defaults() {
        assert_eq!(DissolveReplacement::default(), DissolveReplacement::Space);
        assert_eq!(DissolvePattern::default(), DissolvePattern::Sequential);
        assert_eq!(DissolveDirection::default(), DissolveDirection::LeftToRight);
    }
}

// <FILE>tui-vfx-content/src/types/cls_dissolve_config.rs</FILE> - <DESC>Configuration types for text dissolve effect</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
