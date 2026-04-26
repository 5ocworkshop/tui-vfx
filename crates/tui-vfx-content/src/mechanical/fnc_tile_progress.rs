// <FILE>crates/tui-vfx-content/src/mechanical/fnc_tile_progress.rs</FILE> - <DESC>Derive per-tile local progress from frame progress and a MechanicalCascadePolicy</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 3 of mechanical circular content cycles plan: cascade scheduling that turns whole-frame progress into per-tile local progress so route sampling, settle, and visual flow compose.</WCTX>
// <CLOG>0.1.0: introduce tile_progress_for covering Simultaneous, Staggered, NumericCarry (with UnchangedCellPolicy), and Randomized scheduling.</CLOG>

use crate::types::{MechanicalCascadePolicy, UnchangedCellPolicy};

/// Per-tile schedule metadata that callers compute from the source/
/// target faces and the tile-rect iteration order. The cascade policy
/// alone is not enough because `NumericCarry` needs to know which
/// tiles changed and their place in the carry sequence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct TileScheduleMeta {
    /// Linear tile index in row-major iteration order.
    pub(crate) tile_index: usize,
    /// Total number of tiles in the mechanism.
    pub(crate) total_tiles: usize,
    /// Whether this tile's source and target faces differ.
    pub(crate) changed: bool,
    /// Position among changed tiles, sequenced from least-significant
    /// to most-significant. Only meaningful if `changed = true`.
    /// `0 = LSB, total_changed - 1 = MSB`.
    pub(crate) changed_lsb_index: usize,
    /// Total number of changed tiles.
    pub(crate) total_changed: usize,
}

/// Compute per-tile local progress in `[0.0, 1.0]` from frame progress
/// and cascade policy.
///
/// `Simultaneous` returns `frame_progress` for every tile.
/// `Staggered` distributes start times uniformly across all tiles.
/// `NumericCarry` sequences only changed tiles from LSB to MSB and
/// holds (or spins) unchanged tiles per `UnchangedCellPolicy`.
/// `Randomized` derives a per-tile start delay deterministically
/// from `seed` and `tile_index` via splitmix64.
pub(crate) fn tile_progress_for(
    policy: &MechanicalCascadePolicy,
    frame_progress: f64,
    meta: TileScheduleMeta,
) -> f64 {
    let p = frame_progress.clamp(0.0, 1.0);
    match policy {
        MechanicalCascadePolicy::Simultaneous => p,
        MechanicalCascadePolicy::Staggered { fraction } => staggered_progress(
            p,
            meta.tile_index,
            meta.total_tiles,
            (*fraction).clamp(0.0, 0.95) as f64,
        ),
        MechanicalCascadePolicy::NumericCarry {
            stagger_fraction,
            unchanged,
        } => {
            let frac = (*stagger_fraction).clamp(0.0, 0.95) as f64;
            if !meta.changed {
                return match unchanged {
                    UnchangedCellPolicy::Hold => 0.0,
                    UnchangedCellPolicy::SpinAndReturn => p,
                };
            }
            staggered_progress(p, meta.changed_lsb_index, meta.total_changed.max(1), frac)
        }
        MechanicalCascadePolicy::Randomized {
            seed,
            max_delay_fraction,
        } => {
            let frac = (*max_delay_fraction).clamp(0.0, 0.95) as f64;
            let delay = randomized_delay(*seed, meta.tile_index) * frac;
            local_progress_from_delay(p, delay, frac)
        }
    }
}

fn staggered_progress(
    frame_progress: f64,
    tile_idx: usize,
    total_tiles: usize,
    fraction: f64,
) -> f64 {
    if total_tiles <= 1 {
        return frame_progress;
    }
    let start = (tile_idx as f64) * fraction / ((total_tiles - 1) as f64);
    local_progress_from_delay(frame_progress, start, fraction)
}

fn local_progress_from_delay(frame_progress: f64, start: f64, fraction: f64) -> f64 {
    let active_window = (1.0 - fraction).max(f64::EPSILON);
    ((frame_progress - start) / active_window).clamp(0.0, 1.0)
}

fn randomized_delay(seed: u64, tile_index: usize) -> f64 {
    let mut state = seed
        .wrapping_add(0x9E3779B97F4A7C15)
        .wrapping_add(tile_index as u64);
    state = (state ^ (state >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    state = (state ^ (state >> 27)).wrapping_mul(0x94D049BB133111EB);
    state ^= state >> 31;
    (state >> 11) as f64 / (1u64 << 53) as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn meta(idx: usize, total: usize) -> TileScheduleMeta {
        TileScheduleMeta {
            tile_index: idx,
            total_tiles: total,
            changed: true,
            changed_lsb_index: idx,
            total_changed: total,
        }
    }

    fn assert_close(a: f64, b: f64, tol: f64) {
        assert!((a - b).abs() < tol, "expected ~{b}, got {a}");
    }

    const F32_F64_TOL: f64 = 1e-6;

    #[test]
    fn simultaneous_returns_frame_progress_for_every_tile() {
        let policy = MechanicalCascadePolicy::Simultaneous;
        for tile in 0..4 {
            assert_eq!(tile_progress_for(&policy, 0.5, meta(tile, 4)), 0.5);
        }
    }

    #[test]
    fn staggered_first_tile_starts_immediately() {
        let policy = MechanicalCascadePolicy::Staggered { fraction: 0.4 };
        let p0 = tile_progress_for(&policy, 0.0, meta(0, 4));
        assert_close(p0, 0.0, F32_F64_TOL);
        let p_mid = tile_progress_for(&policy, 0.3, meta(0, 4));
        assert_close(p_mid, 0.5, F32_F64_TOL);
    }

    #[test]
    fn staggered_last_tile_starts_at_fraction() {
        let policy = MechanicalCascadePolicy::Staggered { fraction: 0.4 };
        let p_at_start = tile_progress_for(&policy, 0.4, meta(3, 4));
        assert_close(p_at_start, 0.0, F32_F64_TOL);
        let p_landed = tile_progress_for(&policy, 1.0, meta(3, 4));
        assert_close(p_landed, 1.0, F32_F64_TOL);
    }

    #[test]
    fn staggered_clamps_below_zero_and_above_one() {
        let policy = MechanicalCascadePolicy::Staggered { fraction: 0.4 };
        assert_eq!(tile_progress_for(&policy, 0.0, meta(3, 4)), 0.0);
        assert_eq!(tile_progress_for(&policy, 2.0, meta(0, 4)), 1.0);
    }

    #[test]
    fn staggered_with_one_tile_returns_frame_progress() {
        let policy = MechanicalCascadePolicy::Staggered { fraction: 0.5 };
        assert_eq!(tile_progress_for(&policy, 0.7, meta(0, 1)), 0.7);
    }

    #[test]
    fn staggered_clamps_fraction_above_zero_point_nine_five() {
        let policy = MechanicalCascadePolicy::Staggered { fraction: 0.99 };
        let result = tile_progress_for(&policy, 1.0, meta(3, 4));
        assert_close(result, 1.0, F32_F64_TOL);
    }

    #[test]
    fn numeric_carry_holds_unchanged_tiles_at_zero() {
        let policy = MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.35,
            unchanged: UnchangedCellPolicy::Hold,
        };
        let unchanged = TileScheduleMeta {
            tile_index: 1,
            total_tiles: 3,
            changed: false,
            changed_lsb_index: 0,
            total_changed: 2,
        };
        assert_eq!(tile_progress_for(&policy, 0.5, unchanged), 0.0);
    }

    #[test]
    fn numeric_carry_spin_and_return_advances_unchanged() {
        let policy = MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.35,
            unchanged: UnchangedCellPolicy::SpinAndReturn,
        };
        let unchanged = TileScheduleMeta {
            tile_index: 1,
            total_tiles: 3,
            changed: false,
            changed_lsb_index: 0,
            total_changed: 2,
        };
        assert_eq!(tile_progress_for(&policy, 0.5, unchanged), 0.5);
    }

    #[test]
    fn numeric_carry_lsb_starts_first_msb_starts_last() {
        let policy = MechanicalCascadePolicy::NumericCarry {
            stagger_fraction: 0.5,
            unchanged: UnchangedCellPolicy::Hold,
        };
        let lsb = TileScheduleMeta {
            tile_index: 2,
            total_tiles: 3,
            changed: true,
            changed_lsb_index: 0,
            total_changed: 2,
        };
        let msb = TileScheduleMeta {
            tile_index: 0,
            total_tiles: 3,
            changed: true,
            changed_lsb_index: 1,
            total_changed: 2,
        };
        // At frame_progress = 0.0, LSB starts; MSB hasn't started.
        assert_eq!(tile_progress_for(&policy, 0.0, lsb), 0.0);
        assert_eq!(tile_progress_for(&policy, 0.0, msb), 0.0);
        // At frame_progress = 0.25 (mid-LSB-window), LSB is partway,
        // MSB has not started yet.
        let lsb_mid = tile_progress_for(&policy, 0.25, lsb);
        let msb_mid = tile_progress_for(&policy, 0.25, msb);
        assert!(lsb_mid > 0.0);
        assert_eq!(msb_mid, 0.0);
        // Both land at frame_progress = 1.0.
        assert_close(tile_progress_for(&policy, 1.0, lsb), 1.0, F32_F64_TOL);
        assert_close(tile_progress_for(&policy, 1.0, msb), 1.0, F32_F64_TOL);
    }

    #[test]
    fn randomized_is_deterministic() {
        let policy = MechanicalCascadePolicy::Randomized {
            seed: 1234,
            max_delay_fraction: 0.5,
        };
        let a = tile_progress_for(&policy, 0.4, meta(2, 4));
        let b = tile_progress_for(&policy, 0.4, meta(2, 4));
        assert_eq!(a, b);
    }

    #[test]
    fn randomized_different_seeds_produce_different_delays() {
        let p_a = MechanicalCascadePolicy::Randomized {
            seed: 1,
            max_delay_fraction: 0.5,
        };
        let p_b = MechanicalCascadePolicy::Randomized {
            seed: 99999,
            max_delay_fraction: 0.5,
        };
        // For at least one tile, the two seeds give different progress.
        let mut differs = false;
        for tile in 0..8 {
            if (tile_progress_for(&p_a, 0.3, meta(tile, 8))
                - tile_progress_for(&p_b, 0.3, meta(tile, 8)))
            .abs()
                > 1e-6
            {
                differs = true;
                break;
            }
        }
        assert!(differs);
    }

    #[test]
    fn randomized_lands_every_tile_at_one() {
        let policy = MechanicalCascadePolicy::Randomized {
            seed: 42,
            max_delay_fraction: 0.5,
        };
        for tile in 0..6 {
            assert_close(
                tile_progress_for(&policy, 1.0, meta(tile, 6)),
                1.0,
                F32_F64_TOL,
            );
        }
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_tile_progress.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
