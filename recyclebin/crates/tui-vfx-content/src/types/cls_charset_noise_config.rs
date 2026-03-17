// <FILE>tui-vfx-content/src/types/cls_charset_noise_config.rs</FILE> - <DESC>Configuration types for CharsetNoise content transformer</DESC>
// <VERS>VERSION: 1.0.0</VERS>
// <WCTX>New CharsetNoise transformer for time-varying character replacement with vertical gradient support</WCTX>
// <CLOG>Initial creation: GradientStop struct and AffectMode enum for charset_noise content effect</CLOG>

/// A single stop in a vertical charset gradient.
///
/// Maps a normalized vertical position (0.0 = top, 1.0 = bottom) to a pool
/// of characters. Between stops, the transformer selects from the nearest
/// stop based on position and per-cell jitter.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(deny_unknown_fields)]
pub struct GradientStop {
    /// Normalized vertical position (0.0 = top of widget, 1.0 = bottom).
    pub at: f32,
    /// Pool of characters available at this position.
    pub chars: String,
}

/// Controls which cells the CharsetNoise transformer affects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, serde::Serialize, serde::Deserialize, tui_vfx_core::ConfigSchema)]
#[serde(rename_all = "snake_case")]
pub enum AffectMode {
    /// Replace all cells (including whitespace).
    All,
    /// Replace only non-whitespace cells (spaces and empty braille ⠀ are skipped).
    #[default]
    NonEmpty,
}

// <FILE>tui-vfx-content/src/types/cls_charset_noise_config.rs</FILE> - <DESC>Configuration types for CharsetNoise content transformer</DESC>
// <VERS>END OF VERSION: 1.0.0</VERS>
