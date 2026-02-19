// <FILE>tui-vfx-geometry/src/layout/fnc_grid_snap.rs</FILE> - <DESC>Grid snapping logic</DESC>
// <VERS>VERSION: 1.1.0 - 2025-12-16T20:10:03Z</VERS>
// <WCTX>Turn 5 Restoration</WCTX>
// <CLOG>Re-emit</CLOG>

use crate::types::SnappingStrategy;
const MIX_CONST_1: u64 = 0x9E3779B97F4A7C15;
const MIX_CONST_2: u64 = 0xBF58476D1CE4E5B9;
const MIX_CONST_3: u64 = 0x94D049BB133111EB;

#[inline]
fn mix_seed(seed: u64, value: u32) -> u64 {
    let mut z = seed ^ (value as u64).wrapping_add(MIX_CONST_1);
    z = (z ^ (z >> 30)).wrapping_mul(MIX_CONST_2);
    z = (z ^ (z >> 27)).wrapping_mul(MIX_CONST_3);
    z ^ (z >> 31)
}
pub fn snap(val: f32, strategy: SnappingStrategy) -> u16 {
    if !val.is_finite() || val <= 0.0 {
        return 0;
    }
    let max = u16::MAX as f32;
    let clamp_to_u16 = |value: f32| value.clamp(0.0, max) as u16;
    match strategy {
        SnappingStrategy::Floor => clamp_to_u16(val.floor()),
        SnappingStrategy::Round => clamp_to_u16(val.round()),
        SnappingStrategy::Stochastic { seed } => {
            let fract = val.fract();
            if fract <= 0.0 {
                return clamp_to_u16(val);
            }
            let hash = mix_seed(seed, val as u32);
            let rand = (hash as f64) / (u64::MAX as f64);
            let snapped = if rand < (fract as f64) {
                val.ceil()
            } else {
                val.floor()
            };
            clamp_to_u16(snapped)
        }
    }
}

// <FILE>tui-vfx-geometry/src/layout/fnc_grid_snap.rs</FILE> - <DESC>Grid snapping logic</DESC>
// <VERS>END OF VERSION: 1.1.0 - 2025-12-16T20:10:03Z</VERS>
