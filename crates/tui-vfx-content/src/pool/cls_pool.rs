// <FILE>crates/tui-vfx-content/src/pool/cls_pool.rs</FILE> - <DESC>Generic content-randomization pool. Five sibling pool types collapsed into one type; concrete pools are aliases (ImagePool / FontPool / EffectPool / PresetPool) — TextPool stays as a thin newtype because it sanitizes on construction.</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.B — five hand-rolled pool types collapse into one Pool&lt;T&gt; carrying the canonical { items, policy } shape and { new, pick, is_empty } API. Type aliases preserve the public names consumers import.</WCTX>
// <CLOG>0.1.0: introduce Pool&lt;T&gt; with new / pick / is_empty / Default. Hand-written ConfigSchema impl gated on T: ConfigSchema (the derive macro does not yet emit the bound). Eq via separate manual impl gated on T: Eq.</CLOG>

//! Generic content-randomization pool.
//!
//! Five sibling `*Pool` types previously lived as parallel hand-rolled
//! files. They all carried the same `{ items: Vec<T>, policy: PoolPolicy }`
//! shape and the same `new` / `pick` / `is_empty` API. This file is the
//! consolidation: concrete pools are now type aliases over `Pool<T>`,
//! except [`TextPool`](super::TextPool), which stays as a thin newtype
//! because it sanitizes inputs at construction time and that behavior
//! does not generalize across the family.
//!
//! # Example
//! ```
//! use tui_vfx_content::pool::{Pool, PoolPolicy};
//! let pool = Pool::new(vec!["alpha".to_string(), "beta".to_string()], PoolPolicy::FirstOnly);
//! assert_eq!(pool.pick(), Some(&"alpha".to_string()));
//! ```

use serde::{Deserialize, Serialize};
use tui_vfx_core::schema::{ConfigSchema, FieldMeta, SchemaField, SchemaNode};

use super::col_pool_policy::PoolPolicy;
use super::fnc_pick_index::pick_index;

/// A pool of `T` values selected per pick under [`PoolPolicy`].
///
/// Use the type aliases ([`ImagePool`](super::ImagePool),
/// [`FontPool`](super::FontPool), [`EffectPool`](super::EffectPool),
/// [`PresetPool`](super::PresetPool)) for the canonical concrete shapes;
/// [`TextPool`](super::TextPool) is a sibling newtype that adds
/// sanitize-on-construct.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Pool<T> {
    /// Pool entries. Order is preserved; `pick` selects under
    /// [`PoolPolicy`].
    #[serde(default = "Vec::new")]
    pub items: Vec<T>,

    /// How [`Pool::pick`] selects an entry.
    #[serde(default)]
    pub policy: PoolPolicy,
}

impl<T> Eq for Pool<T> where T: Eq {}

impl<T> Pool<T> {
    /// Construct a new pool from items and a selection policy.
    pub fn new(items: Vec<T>, policy: PoolPolicy) -> Self {
        Self { items, policy }
    }

    /// Pick one entry under this pool's [`PoolPolicy`]. Returns `None`
    /// when the pool is empty.
    pub fn pick(&self) -> Option<&T> {
        pick_index(self.items.len(), self.policy).map(|idx| &self.items[idx])
    }

    /// True when the pool has no items.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: ConfigSchema> ConfigSchema for Pool<T> {
    fn schema() -> SchemaNode {
        SchemaNode::Struct {
            name: "Pool".to_string(),
            description: Some(
                "Pool of items with a selection policy (Random / FirstOnly).".to_string(),
            ),
            json_name: None,
            fields: vec![
                SchemaField::new(
                    "items",
                    SchemaNode::Vec {
                        item: Box::new(T::schema()),
                    },
                    FieldMeta {
                        help: Some("Pool entries.".to_string()),
                        description: None,
                        default: None,
                        range: None,
                        json_key: None,
                        optional: false,
                    },
                ),
                SchemaField::new(
                    "policy",
                    PoolPolicy::schema(),
                    FieldMeta {
                        help: Some("How pick() selects an entry.".to_string()),
                        description: None,
                        default: None,
                        range: None,
                        json_key: None,
                        optional: false,
                    },
                ),
            ],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pool_returns_none() {
        let pool: Pool<String> = Pool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_picks_first_entry() {
        let pool = Pool::new(
            vec!["alpha".to_string(), "beta".to_string()],
            PoolPolicy::FirstOnly,
        );
        assert_eq!(pool.pick(), Some(&"alpha".to_string()));
    }

    #[test]
    fn random_picks_one_of_the_entries() {
        let entries = vec!["a".to_string(), "b".to_string(), "c".to_string()];
        let pool = Pool::new(entries.clone(), PoolPolicy::Random);
        let picked = pool.pick().expect("picks");
        assert!(entries.contains(picked));
    }

    #[test]
    fn serde_roundtrip_string_pool() {
        let pool = Pool::new(vec!["x".to_string(), "y".to_string()], PoolPolicy::Random);
        let json = serde_json::to_string(&pool).expect("serializes");
        let back: Pool<String> = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(pool, back);
    }

    #[test]
    fn pool_eq_when_t_eq() {
        let a: Pool<u32> = Pool::new(vec![1, 2, 3], PoolPolicy::FirstOnly);
        let b: Pool<u32> = Pool::new(vec![1, 2, 3], PoolPolicy::FirstOnly);
        assert!(a == b);
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_pool.rs</FILE> - <DESC>Pool&lt;T&gt;</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
