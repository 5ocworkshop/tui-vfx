// <FILE>tui-vfx-compositor/src/pipeline/fnc_check_masks.rs</FILE> - <DESC>Mask visibility checking with optional inspector</DESC>
// <VERS>VERSION: 1.1.0</VERS>
// <WCTX>Audit fixes - address OAI peer review findings</WCTX>
// <CLOG>Clamp blend ratio to 0.0-1.0 to prevent inverted thresholds</CLOG>

use super::cls_prepared_mask::{PreparedMask, prepare_masks};
use crate::traits::pipeline_inspector::CompositorInspector;
use crate::types::MaskCombineMode;
use crate::types::cls_mask_spec::MaskSpec;
use smallvec::SmallVec;

/// Check visibility against multiple prepared masks with composition mode.
///
/// Optionally reports to an inspector for debugging.
///
/// # Arguments
/// * `local_x`, `local_y` - Position in local coordinates
/// * `width`, `height` - Dimensions of the area
/// * `t` - Progress value for mask evaluation
/// * `masks` - Prepared masks to check against
/// * `combine_mode` - How to combine multiple mask results
/// * `inspector` - Optional inspector for debugging callbacks
#[allow(clippy::too_many_arguments)]
#[inline]
pub(crate) fn check_prepared_masks(
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    t: f64,
    masks: &SmallVec<[PreparedMask; 2]>,
    combine_mode: MaskCombineMode,
    inspector: Option<&mut dyn CompositorInspector>,
) -> bool {
    if masks.is_empty() {
        return true;
    }

    // Collect results, optionally reporting to inspector
    let results: SmallVec<[bool; 2]> = if let Some(inspector) = inspector {
        masks
            .iter()
            .map(|mask| {
                let visible = mask.is_visible(local_x, local_y, width, height, t);
                inspector.on_mask_checked(local_x, local_y, visible, mask.name());
                visible
            })
            .collect()
    } else {
        masks
            .iter()
            .map(|mask| mask.is_visible(local_x, local_y, width, height, t))
            .collect()
    };

    combine_results(&results, combine_mode)
}

/// Combine mask results according to the combine mode.
#[inline]
fn combine_results(results: &SmallVec<[bool; 2]>, combine_mode: MaskCombineMode) -> bool {
    match combine_mode {
        MaskCombineMode::All => results.iter().all(|&v| v),
        MaskCombineMode::Any => results.iter().any(|&v| v),
        MaskCombineMode::Blend { ratio } => {
            let pass_count = results.iter().filter(|&&v| v).count();
            let total = results.len();
            if total == 0 {
                return true;
            }
            // Clamp ratio to valid range to prevent inverted thresholds
            let ratio = ratio.clamp(0.0, 1.0);
            let min_ratio = 1.0 / total as f32;
            let required_ratio = 1.0 - ratio * (1.0 - min_ratio);
            let actual_ratio = pass_count as f32 / total as f32;
            actual_ratio >= required_ratio
        }
    }
}

/// Public wrapper for mask checking using MaskSpec.
///
/// Prepares masks from specs and delegates to check_prepared_masks.
pub fn check_masks(
    local_x: u16,
    local_y: u16,
    width: u16,
    height: u16,
    t: f64,
    masks: &[MaskSpec],
    combine_mode: MaskCombineMode,
) -> bool {
    let prepared = prepare_masks(masks);
    check_prepared_masks(
        local_x,
        local_y,
        width,
        height,
        t,
        &prepared,
        combine_mode,
        None,
    )
}

// <FILE>tui-vfx-compositor/src/pipeline/fnc_check_masks.rs</FILE> - <DESC>Mask visibility checking with optional inspector</DESC>
// <VERS>END OF VERSION: 1.1.0</VERS>
