// <FILE>crates/tui-vfx-content/src/pool/col_pool_policy.rs</FILE> - <DESC>Selection policy enum shared by every pool type (TextPool, EffectPool, PresetPool). Governs how the pool picks an index from its items on each resolution.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan.</WCTX>
// <CLOG>0.1.0: initial; Random (time-seeded default) + FirstOnly (deterministic/test-friendly).</CLOG>

use serde::{Deserialize, Serialize};

/// How a pool selects which item to return on each resolution.
///
/// Additional policies (RoundRobin with persistent state, Weighted,
/// Conditional) are reserved for future work when a concrete second use
/// case surfaces — they'd either need a `PoolState` parameter or a
/// weighted-items schema.
#[derive(
    Default, Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(rename_all = "snake_case")]
pub enum PoolPolicy {
    /// Pick uniformly at random, seeded from the current system time.
    ///
    /// Stateless — each call draws a fresh sample. Good for per-launch
    /// variety (splash taglines, dialog lines). Because the seed advances
    /// with real time, repeated calls within the same millisecond may
    /// return the same index; that's intentional — splash-like surfaces
    /// typically resolve once per launch, not in tight loops.
    #[default]
    Random,

    /// Always pick index 0. Deterministic and test-friendly — use this
    /// in snapshot/probe tests so pool-backed recipes render a fixed
    /// output regardless of wall-clock time.
    FirstOnly,
}

// <FILE>crates/tui-vfx-content/src/pool/col_pool_policy.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
