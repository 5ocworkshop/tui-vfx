// <FILE>crates/tui-vfx-content/src/pool/cls_font_pool.rs</FILE> - <DESC>Pool of rocketsplash font atlas names. Each item is a key resolved against a caller-supplied AssetMap at render time, letting a single recipe rotate between font faces (bold display, script, serif) per launch.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan — per-launch font rotation pairs with ImagePool for total brand variety.</WCTX>
// <CLOG>0.1.0: initial; FontPool with name-reference items matching ImagePool.</CLOG>

use serde::{Deserialize, Serialize};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;

/// A pool of rocketsplash font atlas *names*. Each launch, a name is
/// picked and resolved to `.rsf` bytes via a caller-owned AssetMap.
/// Pairs with [`ImagePool`](super::ImagePool) — recipes can rotate
/// both logos and fonts independently, or curate specific pairings via
/// [`PresetPool`](super::PresetPool).
///
/// See [`ImagePool`](super::ImagePool) for the design rationale on
/// using name references instead of inline bytes.
///
/// # Example
/// ```
/// use tui_vfx_content::pool::{FontPool, PoolPolicy};
/// let pool = FontPool::new(
///     vec!["heading_bold".into(), "heading_script".into(), "mono_20".into()],
///     PoolPolicy::Random,
/// );
/// let picked_name = pool.pick();
/// assert!(picked_name.is_some());
/// ```
#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct FontPool {
    /// Asset-map keys — strings that the caller resolves to `.rsf` bytes.
    #[serde(default)]
    pub items: Vec<String>,

    /// How [`FontPool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl FontPool {
    /// Construct a new font pool from asset names.
    pub fn new(items: Vec<String>, policy: PoolPolicy) -> Self {
        Self { items, policy }
    }

    /// Pick one asset name according to this pool's policy. Returns
    /// `None` when the pool is empty.
    pub fn pick(&self) -> Option<&str> {
        pick_index(self.items.len(), self.policy).map(|idx| self.items[idx].as_str())
    }

    /// True if the pool has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
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
        let pool = FontPool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_picks_first_name() {
        let pool = FontPool::new(
            vec!["bold_20".into(), "script_40".into()],
            PoolPolicy::FirstOnly,
        );
        assert_eq!(pool.pick(), Some("bold_20"));
    }

    #[test]
    fn random_picks_a_valid_name() {
        let pool = FontPool::new(vec!["a".into(), "b".into(), "c".into()], PoolPolicy::Random);
        let picked = pool.pick().unwrap();
        assert!(["a", "b", "c"].contains(&picked));
    }

    #[test]
    fn serde_roundtrip() {
        let pool = FontPool::new(vec!["one".into(), "two".into()], PoolPolicy::Random);
        let json = serde_json::to_string(&pool).unwrap();
        let back: FontPool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, back);
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_font_pool.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
