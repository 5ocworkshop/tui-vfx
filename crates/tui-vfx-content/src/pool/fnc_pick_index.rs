// <FILE>crates/tui-vfx-content/src/pool/fnc_pick_index.rs</FILE> - <DESC>Shared index-picker used by every pool type. Time-seeded hash-based PRNG (no `rand` crate dep) plus a deterministic FirstOnly branch.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan.</WCTX>
// <CLOG>0.1.0: initial; pick_index(len, policy) used by TextPool/EffectPool/PresetPool.</CLOG>

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{SystemTime, UNIX_EPOCH};

use super::col_pool_policy::PoolPolicy;

/// Pick an index in `0..len` according to `policy`. Returns `None` when
/// the pool is empty (caller should fall through to a default / static
/// value in that case).
///
/// Implementation notes:
/// - [`PoolPolicy::Random`] seeds a [`DefaultHasher`] with the current
///   nanosecond-resolution system time and mods the hash into `0..len`.
///   No `rand` crate dep. Distribution is suitable for per-launch variety;
///   not appropriate for cryptographic sampling.
/// - [`PoolPolicy::FirstOnly`] returns `Some(0)` whenever `len > 0` —
///   use this in probe/snapshot tests to pin pool-backed recipes to a
///   deterministic output.
pub fn pick_index(len: usize, policy: PoolPolicy) -> Option<usize> {
    if len == 0 {
        return None;
    }
    match policy {
        PoolPolicy::FirstOnly => Some(0),
        PoolPolicy::Random => {
            let nanos = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_nanos())
                .unwrap_or(0);
            let mut hasher = DefaultHasher::new();
            nanos.hash(&mut hasher);
            Some((hasher.finish() as usize) % len)
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pool_returns_none() {
        assert!(pick_index(0, PoolPolicy::Random).is_none());
        assert!(pick_index(0, PoolPolicy::FirstOnly).is_none());
    }

    #[test]
    fn first_only_is_deterministic() {
        for _ in 0..50 {
            assert_eq!(pick_index(5, PoolPolicy::FirstOnly), Some(0));
        }
    }

    #[test]
    fn random_stays_in_bounds() {
        for _ in 0..200 {
            let idx = pick_index(7, PoolPolicy::Random).unwrap();
            assert!(idx < 7);
        }
    }

    #[test]
    fn random_eventually_distributes() {
        // Over many samples with varying time seeds, we should see more
        // than one unique index. (Not a tight statistical guarantee —
        // just a smoke check that we aren't always returning 0.)
        let mut seen = std::collections::HashSet::new();
        for _ in 0..500 {
            if let Some(idx) = pick_index(8, PoolPolicy::Random) {
                seen.insert(idx);
            }
            // Tiny sleep to advance the nanosecond clock seed so the
            // hasher produces a new value.
            std::thread::sleep(std::time::Duration::from_nanos(1));
        }
        assert!(
            seen.len() > 1,
            "expected >1 unique index across 500 samples, got {}",
            seen.len()
        );
    }

    #[test]
    fn singleton_pool_always_returns_index_zero() {
        assert_eq!(pick_index(1, PoolPolicy::Random), Some(0));
        assert_eq!(pick_index(1, PoolPolicy::FirstOnly), Some(0));
    }
}

// <FILE>crates/tui-vfx-content/src/pool/fnc_pick_index.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
