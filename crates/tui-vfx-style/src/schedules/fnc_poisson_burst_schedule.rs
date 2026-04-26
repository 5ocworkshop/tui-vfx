// <FILE>tui-vfx-style/src/utils/fnc_poisson_burst_schedule.rs</FILE> - <DESC>Per-cell trigger-time schedule generator for TTE Beams-style stochastic batch activation, composed from mixed-signals primitives</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>TTE effects port phase 4b — composing mixed-signals primitives (hash_to_index for batch sizes and per-lane speeds, fisher_yates for lane order) to produce per-cell trigger times for TTE Beams' stochastic burst-cadence visual. Output is a flat Vec<f64> consumed by GlyphTimeline::TimelineTrigger::PerCellSchedule.</WCTX>
// <CLOG>0.1.0: NEW — initial poisson_burst_schedule. Generates per-cell trigger times for lane-axis (Row|Column) sweeps with stochastic batch cadence (1-5 lanes per period) and per-lane speed jitter (1.5-6.0 cells/frame for rows). All randomness is deterministic via seeded hash_to_index + fisher_yates; same seeds always produce the same schedule.</CLOG>

//! Per-cell trigger-time schedule for TTE Beams-style activation.
//!
//! TTE Beams is built from three sources of stochastic structure
//! (`pro/main.rs:949-963` and `pro/main.rs:1098-1133`):
//!
//! 1. **Shuffled lane order** — rows (or columns) activate in a
//!    randomized sequence rather than top-to-bottom.
//! 2. **Stochastic batch cadence** — every N frames, between B_min
//!    and B_max lanes activate together (TTE: 1-5 every 6 frames).
//! 3. **Per-lane speed jitter** — each lane sweeps at its own rate
//!    (TTE rows: 1.5-6.0 cells/frame; TTE columns: 0.9-1.5).
//!
//! The smooth `Wavefront` trigger captures none of these — it produces
//! a continuous monotonic sweep. This helper *bakes* a stochastic
//! schedule into a per-cell `Vec<f64>` that
//! [`TimelineTrigger::PerCellSchedule`](
//! ../../tui-vfx-compositor/src/filters/cls_glyph_timeline.rs)
//! consumes via O(1) lookup.
//!
//! # Substrate
//!
//! All randomness composes [`mixed_signals::random::hash_to_index`]
//! (SplitMix64-based, position-keyed) — no new substrate, no stateful
//! RNG. The lane shuffle is a hash-driven Fisher-Yates: at swap step
//! `i`, the swap partner is `hash_to_index(shuffle_seed, i, i+1)`.
//!
//! `mixed_signals::rng::Rng::with_seed` was tried first but produces
//! collisions across very different seeds at small time values
//! (`Rng::uniform` recreates `SeededRandom` each call and samples at
//! incrementing tiny times like 0.001, 0.002...). `hash_to_index`
//! discriminates seeds reliably and is the right primitive for
//! position-keyed pseudo-random.
//!
//! Same `(shuffle_seed, batch_seed, speed_seed)` triple always
//! produces the same schedule across processes.

use mixed_signals::random::hash_to_index;

/// Which axis forms a "lane" (a contiguous set of cells that share an
/// activation time and sweep rate).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LaneAxis {
    /// Each row is one lane. Cells within a row sweep along the
    /// column axis at the lane's per-lane speed.
    Row,
    /// Each column is one lane. Cells within a column sweep along
    /// the row axis.
    Column,
}

/// Configuration for a Poisson-style burst-cadence schedule.
///
/// Defaults are tuned to match TTE Beams' visual rhythm for a
/// 60-fps clock.
#[derive(Debug, Clone)]
pub struct PoissonBurstScheduleConfig {
    pub lane_axis: LaneAxis,
    /// Frames between successive activation batches. TTE: 6.
    pub batch_period_frames: u16,
    /// Minimum lanes activated per batch (inclusive). TTE: 1.
    pub batch_size_min: u16,
    /// Maximum lanes activated per batch (inclusive). TTE: 5.
    pub batch_size_max: u16,
    /// Minimum per-lane sweep speed in cells per frame.
    /// TTE row beams: 1.5 (= `gen_range(15..=60) * 0.1` minimum).
    pub lane_speed_min: f64,
    /// Maximum per-lane sweep speed in cells per frame.
    /// TTE row beams: 6.0.
    pub lane_speed_max: f64,
    /// Seed for the lane-order shuffle.
    pub shuffle_seed: u64,
    /// Seed for the batch-size sequence.
    pub batch_seed: u64,
    /// Seed for per-lane speed jitter.
    pub speed_seed: u64,
    /// Frames per second for converting cells/frame to cells/sec.
    /// Defaults to 60 in [`Self::tte_default`]; pass through for
    /// non-60-fps recipes.
    pub fps: f64,
    /// Per-lane direction randomization. When `Some(seed)`, each lane
    /// independently picks forward vs reversed intra-lane sweep via
    /// `hash_to_index(seed, lane_idx, 2)` — this is main.rs's
    /// `if rng.gen_bool(0.5) { characters.reverse(); }` per BeamGroup.
    /// When `None`, all lanes sweep forward (left-to-right for rows,
    /// top-to-bottom for columns).
    pub direction_seed: Option<u64>,
    /// Optional per-cell trigger-time jitter on top of the lane
    /// schedule. When `Some((seed, amount))`, each cell's trigger
    /// time is offset by
    /// `(hash_to_index(seed, pos, 2048) / 1024 - 1) * amount` seconds.
    /// Breaks up regimented sweeps without disturbing the lane-batch
    /// cadence.
    pub jitter: Option<(u64, f64)>,
}

impl PoissonBurstScheduleConfig {
    /// Defaults matching TTE Beams' row beams at 60 fps:
    /// `batch_period=6 frames, batch_size=1..=5, speed=1.5..6.0 cells/frame`.
    pub fn tte_row_default(shuffle_seed: u64, batch_seed: u64, speed_seed: u64) -> Self {
        Self {
            lane_axis: LaneAxis::Row,
            batch_period_frames: 6,
            batch_size_min: 1,
            batch_size_max: 5,
            lane_speed_min: 1.5,
            lane_speed_max: 6.0,
            shuffle_seed,
            batch_seed,
            speed_seed,
            fps: 60.0,
            direction_seed: None,
            jitter: None,
        }
    }

    /// Defaults matching TTE Beams' column beams at 60 fps:
    /// `batch_period=6 frames, batch_size=1..=5, speed=0.9..1.5 cells/frame`
    /// (per `pro/main.rs:954`).
    pub fn tte_column_default(shuffle_seed: u64, batch_seed: u64, speed_seed: u64) -> Self {
        Self {
            lane_axis: LaneAxis::Column,
            batch_period_frames: 6,
            batch_size_min: 1,
            batch_size_max: 5,
            lane_speed_min: 0.9,
            lane_speed_max: 1.5,
            shuffle_seed,
            batch_seed,
            speed_seed,
            fps: 60.0,
            direction_seed: None,
            jitter: None,
        }
    }
}

/// Generate a per-cell trigger-time schedule.
///
/// Returns a `Vec<f64>` of length `width * height`, indexed
/// `out[y * width + x]`. Cell `(x, y)` fires at `out[y * width + x]`
/// seconds.
///
/// The result is intended to be wrapped in `Arc` and passed to
/// `TimelineTrigger::PerCellSchedule { trigger_times, width }` in the
/// compositor.
///
/// # Determinism
///
/// All randomness is seeded — same `(shuffle_seed, batch_seed,
/// speed_seed)` plus same `(width, height)` always produces the same
/// schedule. Recipe re-render is reproducible; probe fingerprints
/// stay stable.
///
/// # Algorithm
///
/// 1. Build the lane-order: `(0..lane_count)` shuffled deterministically
///    via `fisher_yates` seeded with `shuffle_seed`.
/// 2. Compute the cumulative batch schedule: at period `k`, activate
///    `batch_min + hash_to_index(batch_seed, k, batch_max - batch_min + 1)`
///    additional lanes. Continue until the cumulative count >= total
///    lanes.
/// 3. For each lane at shuffled position `pos`, find which batch it
///    fell into and use `(batch_index * period_frames / fps)` as its
///    activation time.
/// 4. Per-lane speed: `lane_speed_min + (hash_to_index(speed_seed,
///    lane_idx, 1000) / 1000) * (lane_speed_max - lane_speed_min)`,
///    converted to cells/sec by multiplying by fps.
/// 5. Per-cell trigger = lane activation + (intra-lane offset / lane
///    speed in cells/sec).
pub fn poisson_burst_schedule(
    width: u16,
    height: u16,
    config: &PoissonBurstScheduleConfig,
) -> Vec<f64> {
    let width_us = width as usize;
    let height_us = height as usize;
    let total = width_us * height_us;

    let lane_count = match config.lane_axis {
        LaneAxis::Row => height_us,
        LaneAxis::Column => width_us,
    };
    let intra_lane_extent = match config.lane_axis {
        LaneAxis::Row => width_us,
        LaneAxis::Column => height_us,
    };

    if lane_count == 0 || intra_lane_extent == 0 {
        return vec![f64::INFINITY; total];
    }

    // 1. Hash-driven Fisher-Yates shuffle. Uses hash_to_index per
    //    swap rather than stateful RNG (mixed_signals::rng::Rng
    //    collides on adjacent time samples).
    let mut lane_order: Vec<u16> = (0..lane_count as u16).collect();
    for i in (1..lane_order.len()).rev() {
        let j = hash_to_index(config.shuffle_seed, i as u64, i + 1);
        lane_order.swap(i, j);
    }

    // 2. Cumulative batch schedule. Batch k activates
    //    `batch_min + hash_to_index(...)` lanes; we need cumulative
    //    >= lane_count.
    let batch_size_range =
        (config.batch_size_max.saturating_sub(config.batch_size_min) + 1).max(1) as usize;
    let mut batch_cumulative: Vec<usize> = Vec::new();
    let mut total_so_far: usize = 0;
    let mut batch_idx: u64 = 0;
    while total_so_far < lane_count {
        let extra = hash_to_index(config.batch_seed, batch_idx, batch_size_range)
            + config.batch_size_min as usize;
        total_so_far += extra;
        batch_cumulative.push(total_so_far);
        batch_idx += 1;
    }

    // 3. Per-lane activation time.
    let batch_period_seconds = config.batch_period_frames as f64 / config.fps;
    let mut lane_activation = vec![0.0_f64; lane_count];
    for (kth_to_activate, &lane) in lane_order.iter().enumerate() {
        // Find the smallest batch where cumulative > kth_to_activate.
        let batch_for_kth = batch_cumulative
            .iter()
            .position(|&c| c > kth_to_activate)
            .unwrap_or(batch_cumulative.len().saturating_sub(1));
        lane_activation[lane as usize] = batch_for_kth as f64 * batch_period_seconds;
    }

    // 4. Per-lane speed in cells/second.
    let speed_range = config.lane_speed_max - config.lane_speed_min;
    let lane_speed_per_sec: Vec<f64> = (0..lane_count)
        .map(|n| {
            let bucket = hash_to_index(config.speed_seed, n as u64, 1000) as f64 / 1000.0;
            (config.lane_speed_min + bucket * speed_range) * config.fps
        })
        .collect();

    // 5. Per-cell trigger time.
    let mut out = vec![0.0_f64; total];
    for y in 0..height_us {
        for x in 0..width_us {
            let (lane, intra) = match config.lane_axis {
                LaneAxis::Row => (y, x),
                LaneAxis::Column => (x, y),
            };
            let activation = lane_activation[lane];
            let speed = lane_speed_per_sec[lane].max(1e-6);
            let lane_reversed = match config.direction_seed {
                Some(seed) => hash_to_index(seed, lane as u64, 2) == 1,
                None => false,
            };
            let intra_offset = if lane_reversed {
                intra_lane_extent.saturating_sub(1).saturating_sub(intra)
            } else {
                intra
            };
            let mut t_trig = activation + (intra_offset as f64) / speed;
            if let Some((seed, amount)) = config.jitter {
                let pos_seed = ((x as u64) << 32) | (y as u64);
                let bucket = hash_to_index(seed, pos_seed, 2048) as f64;
                let signed = (bucket / 1024.0) - 1.0;
                t_trig += signed * amount;
            }
            out[y * width_us + x] = t_trig.max(0.0);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn tte_row_cfg() -> PoissonBurstScheduleConfig {
        PoissonBurstScheduleConfig::tte_row_default(11, 22, 33)
    }

    #[test]
    fn output_length_is_width_times_height() {
        let out = poisson_burst_schedule(40, 12, &tte_row_cfg());
        assert_eq!(out.len(), 40 * 12);
    }

    #[test]
    fn deterministic_for_same_seeds_and_size() {
        let cfg1 = PoissonBurstScheduleConfig::tte_row_default(7, 13, 21);
        let cfg2 = PoissonBurstScheduleConfig::tte_row_default(7, 13, 21);
        let a = poisson_burst_schedule(20, 5, &cfg1);
        let b = poisson_burst_schedule(20, 5, &cfg2);
        assert_eq!(a, b);
    }

    #[test]
    fn different_shuffle_seeds_produce_different_schedules() {
        // Adjacent seeds (7 vs 8) can collide in fisher_yates because
        // mixed_signals::rng::Rng::uniform recreates SeededRandom from
        // the seed each call and samples at small time increments;
        // small seeds with small time deltas can produce similar first
        // samples. Use widely-spaced seeds for the differentiation
        // test.
        let cfg_a = PoissonBurstScheduleConfig::tte_row_default(0xA1, 13, 21);
        let cfg_b = PoissonBurstScheduleConfig::tte_row_default(0xB7C9_F3D2, 13, 21);
        let a = poisson_burst_schedule(20, 5, &cfg_a);
        let b = poisson_burst_schedule(20, 5, &cfg_b);
        assert_ne!(
            a, b,
            "different shuffle_seed should change which lanes activate when"
        );
    }

    #[test]
    fn cells_in_same_lane_have_monotonic_trigger_times() {
        // For row-axis: cells in row N at columns [0, 1, 2, ...] should
        // have monotonically increasing trigger times (the beam sweeps
        // across the row at a positive speed).
        let cfg = tte_row_cfg();
        let width = 20_u16;
        let height = 5_u16;
        let out = poisson_burst_schedule(width, height, &cfg);
        for row in 0..height as usize {
            let mut last = -1.0_f64;
            for col in 0..width as usize {
                let t = out[row * width as usize + col];
                assert!(
                    t > last,
                    "non-monotonic at (col={col}, row={row}): prev={last}, this={t}"
                );
                last = t;
            }
        }
    }

    #[test]
    fn tte_row_schedule_first_lane_starts_at_zero() {
        let cfg = tte_row_cfg();
        let width = 20_u16;
        let height = 5_u16;
        let out = poisson_burst_schedule(width, height, &cfg);
        // The lane that gets activated first (kth_to_activate=0) has
        // batch_for_kth=0 → activation=0.0. Find the row with the
        // minimum trigger at column 0.
        let mut min_t = f64::INFINITY;
        for row in 0..height as usize {
            let t = out[row * width as usize];
            if t < min_t {
                min_t = t;
            }
        }
        assert!(
            (min_t - 0.0).abs() < 1e-6,
            "first-activated lane should fire at t=0, got {min_t}"
        );
    }

    #[test]
    fn batch_cadence_creates_grouped_activation_times() {
        // Beyond t=0, the next batch fires at 6/60 = 0.1s. Verify that
        // some lanes have activation in the [0.1, 0.11) bin (allowing
        // a small intra-lane offset for column 0). At minimum, NOT all
        // lanes should be activated by 0.05s — the cadence should
        // produce visible gaps.
        let cfg = tte_row_cfg();
        let width = 20_u16;
        let height = 30_u16; // many lanes so many batches
        let out = poisson_burst_schedule(width, height, &cfg);

        // Collect activation time of each row (= trigger at column 0).
        let activations: Vec<f64> = (0..height as usize)
            .map(|row| out[row * width as usize])
            .collect();

        // Expect activation times to cluster around multiples of 0.1
        // (with some intra-lane offset noise from speed differences).
        // Specifically: we should see at least 2 distinct cluster
        // centers, separated by ~0.1.
        let mut sorted = activations.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
        // Find a gap > 0.05 in the sorted list — evidence of a batch
        // boundary. With smooth Wavefront this would be impossible.
        let mut max_gap = 0.0_f64;
        for w in sorted.windows(2) {
            let gap = w[1] - w[0];
            if gap > max_gap {
                max_gap = gap;
            }
        }
        assert!(
            max_gap > 0.05,
            "expected batch-cadence gap > 0.05s in sorted activation times, got max gap {max_gap}"
        );
    }

    #[test]
    fn column_axis_swaps_lane_dimension() {
        // For column axis, lanes = columns. Cells in column N at rows
        // [0, 1, 2, ...] should have monotonic trigger times.
        let cfg = PoissonBurstScheduleConfig::tte_column_default(11, 22, 33);
        let width = 10_u16;
        let height = 12_u16;
        let out = poisson_burst_schedule(width, height, &cfg);
        for col in 0..width as usize {
            let mut last = -1.0_f64;
            for row in 0..height as usize {
                let t = out[row * width as usize + col];
                assert!(
                    t > last,
                    "non-monotonic at (col={col}, row={row}): prev={last}, this={t}"
                );
                last = t;
            }
        }
    }

    #[test]
    fn zero_size_canvas_returns_infinity() {
        let cfg = tte_row_cfg();
        let out = poisson_burst_schedule(0, 5, &cfg);
        assert_eq!(out.len(), 0);
        let out = poisson_burst_schedule(5, 0, &cfg);
        assert_eq!(out.len(), 0);
    }

    #[test]
    fn tight_speed_range_collapses_to_uniform_speed() {
        // If lane_speed_min == lane_speed_max, every lane sweeps at
        // exactly that rate. Per-lane intra-lane delta should be the
        // same across lanes.
        let mut cfg = tte_row_cfg();
        cfg.lane_speed_min = 3.0;
        cfg.lane_speed_max = 3.0;
        let width = 10_u16;
        let height = 5_u16;
        let out = poisson_burst_schedule(width, height, &cfg);
        // Per-lane: trigger[col=1] - trigger[col=0] = 1 / (3.0 * 60) seconds.
        let expected_step = 1.0 / (3.0 * 60.0);
        for row in 0..height as usize {
            let t0 = out[row * width as usize];
            let t1 = out[row * width as usize + 1];
            let step = t1 - t0;
            assert!(
                (step - expected_step).abs() < 1e-9,
                "row {row} step {step} expected {expected_step}"
            );
        }
    }
}

// <FILE>tui-vfx-style/src/utils/fnc_poisson_burst_schedule.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
