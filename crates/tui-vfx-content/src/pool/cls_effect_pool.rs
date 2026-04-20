// <FILE>crates/tui-vfx-content/src/pool/cls_effect_pool.rs</FILE> - <DESC>Pool of ContentEffect values with a selection policy. Rotates which reveal/animation effect is applied to content on each resolution — author rotates text, effect, or both independently, or curates specific pairings via PresetPool.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan.</WCTX>
// <CLOG>0.1.0: initial; EffectPool alongside TextPool so text variety and effect variety compose independently.</CLOG>

use serde::{Deserialize, Serialize};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;
use crate::types::ContentEffect;

/// A pool of [`ContentEffect`] values. Each resolution picks one effect
/// to apply to the content for that playback.
///
/// Pairs naturally with [`TextPool`](super::TextPool): the author can
/// rotate text, effect, or both independently. For curated text+effect
/// pairings (where specific lines should always use specific effects),
/// see [`PresetPool`](super::PresetPool).
///
/// # Example
/// ```
/// use tui_vfx_content::pool::{EffectPool, PoolPolicy};
/// use tui_vfx_content::types::{ContentEffect, ScrambleCharset};
/// use mixed_signals::prelude::SignalOrFloat;
///
/// let pool = EffectPool::new(
///     vec![
///         ContentEffect::Typewriter {
///             speed_variance: SignalOrFloat::Static(0.0),
///             cursor: None,
///         },
///         ContentEffect::Scramble {
///             resolve_pace: SignalOrFloat::Static(1.0),
///             charset: ScrambleCharset::Alphanumeric,
///             seed: 0,
///         },
///     ],
///     PoolPolicy::Random,
/// );
/// let chosen = pool.pick();
/// assert!(chosen.is_some());
/// ```
#[derive(
    Clone, Debug, Default, PartialEq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
pub struct EffectPool {
    /// Pool entries.
    #[serde(default)]
    pub items: Vec<ContentEffect>,

    /// How [`EffectPool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl EffectPool {
    /// Construct a new effect pool.
    pub fn new(items: Vec<ContentEffect>, policy: PoolPolicy) -> Self {
        Self { items, policy }
    }

    /// Pick one effect according to this pool's policy. Returns `None`
    /// when the pool is empty.
    pub fn pick(&self) -> Option<&ContentEffect> {
        pick_index(self.items.len(), self.policy).map(|idx| &self.items[idx])
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
    use crate::types::{ScrambleCharset, TypewriterCursor};
    use mixed_signals::prelude::SignalOrFloat;

    fn tw() -> ContentEffect {
        ContentEffect::Typewriter {
            speed_variance: SignalOrFloat::Static(0.0),
            cursor: Some(TypewriterCursor::block()),
        }
    }

    fn sc() -> ContentEffect {
        ContentEffect::Scramble {
            resolve_pace: SignalOrFloat::Static(1.0),
            charset: ScrambleCharset::Alphanumeric,
            seed: 0,
        }
    }

    #[test]
    fn empty_pool_returns_none() {
        let pool = EffectPool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_returns_first_effect() {
        let pool = EffectPool::new(vec![tw(), sc()], PoolPolicy::FirstOnly);
        assert!(matches!(pool.pick(), Some(ContentEffect::Typewriter { .. })));
    }

    #[test]
    fn random_returns_some_effect() {
        let pool = EffectPool::new(vec![tw(), sc()], PoolPolicy::Random);
        assert!(pool.pick().is_some());
    }

    #[test]
    fn serde_roundtrip() {
        let pool = EffectPool::new(vec![tw(), sc()], PoolPolicy::FirstOnly);
        let json = serde_json::to_string(&pool).unwrap();
        let back: EffectPool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, back);
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_effect_pool.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
