// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_apply_cell_motion.rs</FILE> - <DESC>Apply pure cell-motion scheduler</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>V3 Packet 1: deterministic content-local per-cell motion scheduler.</WCTX>
// <CLOG>0.1.0: add actor extraction, stagger, clipping, collision, reduced-motion, and stats.</CLOG>

use std::collections::BTreeMap;

use super::{
    CellCollisionMode, CellMotionOptions, CellMotionPhase, CellMotionResult, CellMotionSample,
    CellMotionSpec, CellMotionStats, CellMotionTiming, CellPlacementContext, CellVisibilityMode,
    cls_cell_motion_candidate::CellMotionCandidate, collect_cell_actors, resolve_actor_offset_ms,
    resolve_cell_placement,
};
use crate::cell_motion::fnc_cell_motion_visibility_position::{
    cell_motion_visibility_position, reduced_cell_motion_position,
};
use crate::cell_motion::fnc_cell_motion_winner_index::cell_motion_winner_index;
use crate::cell_motion::fnc_clip_cell_motion_position::clip_cell_motion_position;
use crate::cell_motion::fnc_lower_cell_motion_path::lower_cell_motion_path;
use crate::cell_motion::fnc_sample_cell_motion_position::sample_cell_motion_position;
use crate::cell_motion::fnc_selected_cell_actor_bounds::selected_cell_actor_bounds;
use crate::cell_motion::fnc_update_cell_motion_t_range::update_cell_motion_t_range;
use tui_vfx_geometry::types::Position;
use tui_vfx_types::{Grid, Rect, SemanticScene};

/// Apply a resolved cell-motion spec to a source scene for one sampled frame.
pub fn apply_cell_motion(
    scene: &SemanticScene,
    spec: &CellMotionSpec,
    timing: &CellMotionTiming,
    local_frame: Rect,
    options: &CellMotionOptions,
) -> CellMotionResult {
    let Some(phase_spec) = spec.phase_spec(timing.phase) else {
        return unchanged(scene);
    };
    if phase_spec.validate().is_err() {
        return unchanged(scene);
    }
    let (actors, mut output) = collect_cell_actors(scene, &phase_spec);
    if actors.is_empty() {
        return unchanged(scene);
    }

    let ctx = CellPlacementContext {
        local_frame,
        selected_bounds: selected_cell_actor_bounds(&actors, local_frame),
    };
    let mut stats = CellMotionStats {
        selected_actor_count: actors.len() as u32,
        ..Default::default()
    };
    let mut buckets: BTreeMap<(u16, u16), Vec<CellMotionCandidate>> = BTreeMap::new();
    let path = lower_cell_motion_path(&phase_spec.route, &phase_spec.dynamics);

    for actor in &actors {
        let offset = resolve_actor_offset_ms(
            actor,
            &phase_spec.stagger,
            &actors,
            &ctx,
            options.recipe_or_layer_seed ^ timing.seed,
        );
        stats.max_stagger_offset_ms = stats.max_stagger_offset_ms.max(offset);
        let from = resolve_cell_placement(actor, &phase_spec.from, &ctx);
        let to = resolve_cell_placement(actor, &phase_spec.to, &ctx);
        let before_start = timing.phase_elapsed_ms < offset;
        let local_elapsed = timing.phase_elapsed_ms.saturating_sub(offset);
        let mut local_t = actor_local_t(before_start, local_elapsed, phase_spec.duration_ms);
        local_t = quantize_t(local_t, phase_spec.quantize_steps);
        let after_complete = actor_after_complete(
            before_start,
            local_elapsed,
            phase_spec.duration_ms,
            timing.phase,
        );
        let rendered = if timing.reduced_motion {
            reduced_cell_motion_position(
                timing.phase,
                before_start,
                actor,
                &phase_spec.visibility,
                from,
                to,
                &mut stats,
            )
        } else if before_start {
            cell_motion_visibility_position(
                phase_spec.visibility.before_start,
                actor,
                from,
                to,
                &mut stats,
                true,
            )
        } else if after_complete
            && !matches!(
                phase_spec.visibility.after_complete,
                CellVisibilityMode::Hold
            )
        {
            cell_motion_visibility_position(
                phase_spec.visibility.after_complete,
                actor,
                from,
                to,
                &mut stats,
                false,
            )
        } else {
            let eased_t = phase_spec.easing.ease(local_t) as f64;
            Some(sample_cell_motion_position(
                from,
                phase_spec
                    .via
                    .as_ref()
                    .map(|v| resolve_cell_placement(actor, v, &ctx)),
                to,
                eased_t,
                &path,
                &phase_spec.snap,
            ))
        };
        update_cell_motion_t_range(&mut stats, local_t as f32, !before_start);
        let rendered = rendered.and_then(|p| clip_cell_motion_position(p, local_frame));
        record_candidate(
            actor.clone(),
            rendered,
            local_t as f32,
            &mut buckets,
            &mut stats,
        );
        record_sample(
            actor.authored_index,
            from,
            to,
            rendered,
            local_t as f32,
            options,
            &mut stats,
        );
    }

    write_collision_winners(buckets, phase_spec.collision, &mut output, &mut stats);
    CellMotionResult {
        scene: output,
        stats,
    }
}

fn unchanged(scene: &SemanticScene) -> CellMotionResult {
    CellMotionResult {
        scene: scene.clone(),
        stats: CellMotionStats::default(),
    }
}

fn actor_local_t(before_start: bool, local_elapsed: u64, duration_ms: u64) -> f64 {
    if before_start {
        0.0
    } else if duration_ms == 0 {
        1.0
    } else {
        (local_elapsed as f64 / duration_ms as f64).clamp(0.0, 1.0)
    }
}

fn quantize_t(local_t: f64, quantize_steps: Option<u32>) -> f64 {
    if let Some(steps) = quantize_steps
        && steps >= 2
    {
        (local_t * (steps - 1) as f64).round() / (steps - 1) as f64
    } else {
        local_t
    }
}

fn actor_after_complete(
    before_start: bool,
    local_elapsed: u64,
    duration_ms: u64,
    phase: CellMotionPhase,
) -> bool {
    !before_start
        && (local_elapsed > duration_ms
            || (duration_ms == 0 && matches!(phase, CellMotionPhase::Exit)))
}

fn record_candidate(
    actor: super::CellActor,
    rendered: Option<(u16, u16)>,
    local_t: f32,
    buckets: &mut BTreeMap<(u16, u16), Vec<CellMotionCandidate>>,
    stats: &mut CellMotionStats,
) {
    match rendered {
        Some((x, y)) => {
            stats.moved_actor_count += 1;
            buckets
                .entry((x, y))
                .or_default()
                .push(CellMotionCandidate {
                    actor,
                    x,
                    y,
                    local_t,
                });
        }
        None => stats.clipped_actor_count += 1,
    }
}

fn record_sample(
    authored_index: u32,
    from: Position,
    to: Position,
    rendered: Option<(u16, u16)>,
    local_t: f32,
    options: &CellMotionOptions,
    stats: &mut CellMotionStats,
) {
    if stats.samples.len() >= options.sample_limit {
        return;
    }
    stats.samples.push(CellMotionSample {
        authored_index,
        from,
        to,
        rendered: rendered.map(|(x, y)| Position::new(x as i32, y as i32)),
        local_t,
    });
}

fn write_collision_winners(
    buckets: BTreeMap<(u16, u16), Vec<CellMotionCandidate>>,
    collision: CellCollisionMode,
    output: &mut SemanticScene,
    stats: &mut CellMotionStats,
) {
    for ((x, y), candidates) in buckets {
        let baseline_non_empty = output.cell((x, y)).is_some_and(|c| !c.is_empty());
        if baseline_non_empty && matches!(collision, CellCollisionMode::PreserveExisting) {
            stats.collision_count += candidates.len() as u32;
            continue;
        }
        let winner_idx = cell_motion_winner_index(&candidates, collision);
        stats.collision_count += candidates.len().saturating_sub(1) as u32;
        if baseline_non_empty {
            stats.baseline_overwrite_count += 1;
        }
        let winner = &candidates[winner_idx];
        output
            .grid_mut()
            .set(winner.x as usize, winner.y as usize, winner.actor.cell);
        output
            .roles_mut()
            .set((winner.x, winner.y), winner.actor.role.clone());
    }
}

// <FILE>crates/tui-vfx-content/src/cell_motion/fnc_apply_cell_motion.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
