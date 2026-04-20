// <FILE>crates/tui-vfx-content/src/pool/cls_image_pool.rs</FILE> - <DESC>Pool of rocketsplash image asset names. Each item is a key resolved against a caller-supplied AssetMap at blit time, so recipes stay lightweight (strings only) while the caller owns asset bytes via include_bytes!, file loads, or any other source.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan — per-launch logo rotation.</WCTX>
// <CLOG>0.1.0: initial; ImagePool with name-reference items (not inline bytes) matching the Substitutions pattern.</CLOG>

use serde::{Deserialize, Serialize};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;

/// A pool of rocketsplash image asset *names*. Each launch, a name is
/// picked according to [`PoolPolicy`] and the caller resolves it to
/// `.rss` bytes via an `AssetMap` (lives in the downstream splash
/// library alongside [`crate::pool::TextPool`] / effect substitutions).
///
/// # Why names, not bytes
///
/// Inline byte vectors in a recipe struct force 30–100KB duplication
/// per entry in memory and break when a recipe is serialized to JSON.
/// Name references keep recipes lightweight and let the caller own
/// asset distribution (embed with `include_bytes!`, stream from disk,
/// fetch over the network — the pool doesn't care).
///
/// # Example
/// ```
/// use tui_vfx_content::pool::{ImagePool, PoolPolicy};
/// let pool = ImagePool::new(
///     vec!["logo_light".into(), "logo_dark".into(), "logo_halloween".into()],
///     PoolPolicy::Random,
/// );
/// let picked_name = pool.pick();
/// assert!(picked_name.is_some());
/// // Caller then looks `picked_name` up in its AssetMap to get bytes.
/// ```
#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct ImagePool {
    /// Asset-map keys — strings that the caller resolves to `.rss` bytes.
    #[serde(default)]
    pub items: Vec<String>,

    /// How [`ImagePool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl ImagePool {
    /// Construct a new image pool from asset names.
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
        let pool = ImagePool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_picks_first_name() {
        let pool = ImagePool::new(
            vec!["logo_a".into(), "logo_b".into()],
            PoolPolicy::FirstOnly,
        );
        assert_eq!(pool.pick(), Some("logo_a"));
    }

    #[test]
    fn random_picks_a_valid_name() {
        let pool = ImagePool::new(
            vec!["one".into(), "two".into(), "three".into()],
            PoolPolicy::Random,
        );
        let picked = pool.pick().unwrap();
        assert!(["one", "two", "three"].contains(&picked));
    }

    #[test]
    fn serde_roundtrip() {
        let pool = ImagePool::new(
            vec!["alpha".into(), "beta".into()],
            PoolPolicy::Random,
        );
        let json = serde_json::to_string(&pool).unwrap();
        let back: ImagePool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, back);
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_image_pool.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
