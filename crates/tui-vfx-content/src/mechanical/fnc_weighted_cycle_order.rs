// <FILE>crates/tui-vfx-content/src/mechanical/fnc_weighted_cycle_order.rs</FILE> - <DESC>Deterministic seed-driven shuffle for weighted and randomized cycle face supplies</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 2 of mechanical circular content cycles plan: deterministic Fisher-Yates over splitmix64 so weighted reels are reproducible without an external RNG dependency.</WCTX>
// <CLOG>0.1.0: introduce weighted_cycle_order and shuffle_in_place; reject zero weights, duplicates, and u32 overflow before expanding.</CLOG>

use std::collections::HashSet;

use crate::types::WeightedCycleFace;

use super::enum_cycle_error::MechanicalCycleError;

/// Expand `faces` to `weight` copies each, then shuffle deterministically
/// using `seed`. The output is the resolved cycle order — index 0 is
/// the first face the route resolver will see, etc.
///
/// Rejects:
/// - any face with `weight == 0`
/// - duplicate `value` entries (recipes must combine into one entry)
/// - weight totals that overflow `u32`
///
/// The seed-derived shuffle uses splitmix64 to drive Fisher-Yates so the
/// output depends only on `(seed, faces)` and never observes runtime
/// randomness. No `rand` crate dependency is taken.
pub(crate) fn weighted_cycle_order(
    faces: &[WeightedCycleFace],
    seed: u64,
) -> Result<Vec<String>, MechanicalCycleError> {
    if faces.is_empty() {
        return Err(MechanicalCycleError::EmptyFaces);
    }
    let mut total: u32 = 0;
    let mut seen: HashSet<&str> = HashSet::with_capacity(faces.len());
    for face in faces {
        if face.value.is_empty() {
            return Err(MechanicalCycleError::EmptyFaceValue);
        }
        if face.weight == 0 {
            return Err(MechanicalCycleError::ZeroWeight {
                value: face.value.clone(),
            });
        }
        if !seen.insert(face.value.as_str()) {
            return Err(MechanicalCycleError::DuplicateFace {
                value: face.value.clone(),
            });
        }
        total = total
            .checked_add(face.weight as u32)
            .ok_or(MechanicalCycleError::WeightOverflow)?;
    }
    let mut expanded: Vec<String> = Vec::with_capacity(total as usize);
    for face in faces {
        for _ in 0..face.weight {
            expanded.push(face.value.clone());
        }
    }
    shuffle_in_place(&mut expanded, seed);
    Ok(expanded)
}

/// Shuffle `items` in place using a splitmix64 PRNG seeded with `seed`.
/// Stable across builds and platforms; only depends on the `seed` value.
pub(crate) fn shuffle_in_place<T>(items: &mut [T], seed: u64) {
    let mut state = seed.wrapping_add(0x9E3779B97F4A7C15);
    for i in (1..items.len()).rev() {
        let r = next_u64(&mut state);
        let j = (r as u128 * (i as u128 + 1) >> 64) as usize;
        items.swap(i, j);
    }
}

fn next_u64(state: &mut u64) -> u64 {
    *state = state.wrapping_add(0x9E3779B97F4A7C15);
    let mut z = *state;
    z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
    z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
    z ^ (z >> 31)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn face(value: &str, weight: u16) -> WeightedCycleFace {
        WeightedCycleFace {
            value: value.into(),
            weight,
        }
    }

    #[test]
    fn empty_faces_rejected() {
        let err = weighted_cycle_order(&[], 0).unwrap_err();
        assert_eq!(err, MechanicalCycleError::EmptyFaces);
    }

    #[test]
    fn zero_weight_rejected_with_face_value() {
        let err = weighted_cycle_order(&[face("BAR", 0)], 0).unwrap_err();
        assert!(
            matches!(err, MechanicalCycleError::ZeroWeight { ref value } if value == "BAR"),
            "{err:?}",
        );
    }

    #[test]
    fn duplicate_face_rejected() {
        let err =
            weighted_cycle_order(&[face("7", 1), face("$", 1), face("7", 2)], 0).unwrap_err();
        assert!(
            matches!(err, MechanicalCycleError::DuplicateFace { ref value } if value == "7"),
            "{err:?}",
        );
    }

    #[test]
    fn empty_face_value_rejected() {
        let err = weighted_cycle_order(&[face("", 1)], 0).unwrap_err();
        assert_eq!(err, MechanicalCycleError::EmptyFaceValue);
    }

    #[test]
    fn weight_overflow_rejected() {
        // 65537 unique faces with weight u16::MAX sum to u32::MAX
        // exactly (65535 × 65537 = 2^32 - 1). One more entry overflows.
        // Use 65_538 unique face names; dedup is HashSet so the cost
        // stays linear.
        let faces: Vec<WeightedCycleFace> = (0..65_538u32)
            .map(|i| WeightedCycleFace {
                value: format!("F{i}"),
                weight: u16::MAX,
            })
            .collect();
        let err = weighted_cycle_order(&faces, 0).unwrap_err();
        assert_eq!(err, MechanicalCycleError::WeightOverflow);
    }

    #[test]
    fn output_length_equals_total_weight() {
        let faces = [face("A", 2), face("B", 3), face("C", 1)];
        let order = weighted_cycle_order(&faces, 42).unwrap();
        assert_eq!(order.len(), 6);
    }

    #[test]
    fn output_contains_each_face_with_correct_multiplicity() {
        let faces = [face("A", 2), face("B", 3), face("C", 1)];
        let order = weighted_cycle_order(&faces, 42).unwrap();
        let count_a = order.iter().filter(|v| *v == "A").count();
        let count_b = order.iter().filter(|v| *v == "B").count();
        let count_c = order.iter().filter(|v| *v == "C").count();
        assert_eq!(count_a, 2);
        assert_eq!(count_b, 3);
        assert_eq!(count_c, 1);
    }

    #[test]
    fn same_seed_produces_same_order() {
        let faces = [face("A", 2), face("B", 3), face("C", 1)];
        let a = weighted_cycle_order(&faces, 12345).unwrap();
        let b = weighted_cycle_order(&faces, 12345).unwrap();
        assert_eq!(a, b);
    }

    #[test]
    fn different_seeds_produce_different_orders_for_non_trivial_input() {
        let faces = [
            face("A", 1),
            face("B", 1),
            face("C", 1),
            face("D", 1),
            face("E", 1),
        ];
        let a = weighted_cycle_order(&faces, 1).unwrap();
        let b = weighted_cycle_order(&faces, 99999).unwrap();
        assert_ne!(a, b, "{a:?} vs {b:?}");
    }

    #[test]
    fn shuffle_in_place_is_deterministic() {
        let mut a: Vec<i32> = (0..16).collect();
        let mut b: Vec<i32> = (0..16).collect();
        shuffle_in_place(&mut a, 7);
        shuffle_in_place(&mut b, 7);
        assert_eq!(a, b);
    }

    #[test]
    fn shuffle_in_place_preserves_multiset() {
        let mut items: Vec<u32> = (0..32).collect();
        shuffle_in_place(&mut items, 1234);
        let mut sorted = items.clone();
        sorted.sort();
        assert_eq!(sorted, (0..32).collect::<Vec<_>>());
    }
}

// <FILE>crates/tui-vfx-content/src/mechanical/fnc_weighted_cycle_order.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
