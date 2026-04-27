// <FILE>tui-vfx-compositor/src/pipeline/fnc_check_masks.rs</FILE> - <DESC>Mask visibility checking with optional inspector</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>2026-04-26 packet Phase 4 — orchestrator owns ctx construction so the sampler-accumulated resolved-coord delta reaches every mask check.</WCTX>
// <CLOG>2.0.0: BREAKING — check_prepared_masks now takes &VfxCellContext instead of (local_x, local_y, width, height, t); call sites in orc_render_pipeline.rs build ctx once per cell with .with_sampler_resolution applied.</CLOG>

use super::cls_prepared_mask::{PreparedMask, prepare_masks};
use crate::traits::pipeline_inspector::CompositorInspector;
use crate::types::MaskCombineMode;
use crate::types::cls_mask_spec::MaskSpec;
use smallvec::SmallVec;
use tui_vfx_types::VfxCellContext;

/// Check visibility against multiple prepared masks with composition mode.
///
/// Optionally reports to an inspector for debugging.
///
/// `ctx` is built once per cell by the caller and is expected to carry the
/// sampler-accumulated resolved-coord delta (via
/// [`VfxCellContext::with_sampler_resolution`]). Mask impls that read
/// `ctx.resolved_x` / `ctx.resolved_y` will see per-cell sampler
/// displacement when a sampler is in flight.
#[inline]
pub(crate) fn check_prepared_masks(
    ctx: &VfxCellContext,
    masks: &SmallVec<[PreparedMask; 2]>,
    combine_mode: MaskCombineMode,
    inspector: Option<&mut dyn CompositorInspector>,
) -> bool {
    if masks.is_empty() {
        return true;
    }

    // Inspector path: must evaluate every mask so every one is reported,
    // regardless of combine_mode. Collect and delegate to combine_results.
    if let Some(inspector) = inspector {
        let results: SmallVec<[bool; 2]> = masks
            .iter()
            .enumerate()
            .map(|(index, mask)| {
                let visible = mask.is_visible(ctx);
                inspector.on_mask_checked(
                    ctx.local_x,
                    ctx.local_y,
                    visible,
                    &format!("{}#{}", mask.name(), index + 1),
                );
                visible
            })
            .collect();
        return combine_results(&results, combine_mode);
    }

    // Non-inspector path: short-circuit mask evaluation when the outcome
    // is already decided.
    match combine_mode {
        MaskCombineMode::All => masks.iter().all(|mask| mask.is_visible(ctx)),
        MaskCombineMode::Any => masks.iter().any(|mask| mask.is_visible(ctx)),
        MaskCombineMode::Blend { .. } => {
            let results: SmallVec<[bool; 2]> = masks
                .iter()
                .map(|mask| mask.is_visible(ctx))
                .collect();
            combine_results(&results, combine_mode)
        }
    }
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
/// Prepares masks from specs and delegates to check_prepared_masks. The
/// constructed ctx defaults `screen_x`/`screen_y` to 0 and starts with
/// `resolved_x = local_x` / `resolved_y = local_y` (no sampler in flight)
/// — callers that need a sampler-displaced resolved coord should drop to
/// the lower-level wrapper that takes `&VfxCellContext` directly.
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
    let ctx = VfxCellContext::new(local_x, local_y, width, height, 0, 0, t);
    check_prepared_masks(&ctx, &prepared, combine_mode, None)
}

// <FILE>tui-vfx-compositor/src/pipeline/fnc_check_masks.rs</FILE> - <DESC>Mask visibility checking with optional inspector</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
