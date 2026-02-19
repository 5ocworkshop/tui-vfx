// <FILE>tui-vfx-compositor/src/types/mask_combine_mode.rs</FILE> - <DESC>Mask composition mode for multiple masks</DESC>
// <VERS>VERSION: 1.3.0</VERS>
// <WCTX>Audit fixes - address OAI peer review findings</WCTX>
// <CLOG>Document that Blend ratio is clamped to 0.0-1.0</CLOG>

use serde::{Deserialize, Serialize};

/// Composition mode for combining multiple masks.
///
/// When multiple masks are applied to a stage, this determines how their
/// visibility results are combined.
///
/// # Examples
///
/// ```
/// use tui_vfx_compositor::types::MaskCombineMode;
///
/// // Default is All (AND logic)
/// assert_eq!(MaskCombineMode::default(), MaskCombineMode::All);
///
/// // Blend mode for smooth alpha composition
/// let blend = MaskCombineMode::Blend { ratio: 0.5 };
/// ```
#[derive(
    Debug, Clone, Copy, PartialEq, Default, tui_vfx_core::ConfigSchema, Serialize, Deserialize,
)]
#[serde(rename_all = "snake_case")]
pub enum MaskCombineMode {
    /// AND logic: cell is visible only if ALL masks pass.
    ///
    /// Use this for layered reveals where each mask must pass.
    /// Example: Wipe + Dissolve = dissolving wipe edge.
    #[default]
    All,

    /// OR logic: cell is visible if ANY mask passes.
    ///
    /// Use this for multiple reveal sources.
    /// Example: Iris + Wipe = content reveals from center OR from edge.
    Any,

    /// Smooth alpha blending between masks using multiplication.
    ///
    /// The `ratio` determines the blend weight. Values outside 0.0-1.0 are clamped.
    /// Uses the Multiply combinator concept for smooth transitions.
    /// Example: Two wipes with Blend { ratio: 0.5 } = soft intersection.
    Blend {
        /// Blend ratio (0.0 = first mask dominates, 1.0 = second mask dominates).
        /// Clamped to 0.0-1.0 range.
        ratio: f32,
    },
}

impl MaskCombineMode {
    /// Check if this mode is the default (All/AND).
    pub fn is_default(&self) -> bool {
        matches!(self, MaskCombineMode::All)
    }
}

// <FILE>tui-vfx-compositor/src/types/mask_combine_mode.rs</FILE> - <DESC>Mask composition mode for multiple masks</DESC>
// <VERS>END OF VERSION: 1.3.0</VERS>
