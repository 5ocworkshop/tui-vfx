// <FILE>crates/tui-vfx-content/src/pool/cls_text_pool.rs</FILE> - <DESC>Pool of strings with sanitize-on-construct. Newtype wrapper around Pool&lt;String&gt; — TextPool is the one pool whose constructor strips control bytes (ESC, newlines, tabs) so pool entries can't corrupt downstream render paths. Splash taglines, dialog lines, error-message variety.</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.B — TextPool stays as a thin newtype around Pool&lt;String&gt; rather than collapsing to a type alias because sanitize() is behavioral, not docstring-level. Other String pools (ImagePool, FontPool) use the alias form because they accept arbitrary asset-map keys.</WCTX>
// <CLOG>0.2.0: TextPool re-implemented as `pub struct TextPool(Pool&lt;String&gt;)` with `new` running sanitize before delegating; pick / is_empty delegate. Deref&lt;Target = Pool&lt;String&gt;&gt; for read-only field access. Serde transparent so the wire format stays `{ items, policy }`.
// 0.1.0: initial; TextPool with items sanitization at construction, pick() helper.</CLOG>

use std::ops::Deref;

use serde::{Deserialize, Serialize};

use super::cls_pool::Pool;
use super::col_pool_policy::PoolPolicy;

/// A general-purpose pool of strings with sanitize-on-construct.
///
/// Designed for any content-randomization use case — splash taglines,
/// game NPC dialog, error-message variety, seasonal Easter eggs. Ships
/// in `tui-vfx-content` (not in gt-design) so any tui-vfx consumer
/// benefits without pulling the whole GT stack.
///
/// Items are sanitized on construction: control bytes (ESC, backspace,
/// etc.) are stripped so pool entries can't corrupt downstream render
/// paths. Newlines are also stripped in v1 to keep layout predictable;
/// multi-line splash text should be modeled as separate recipe slots.
///
/// # Why this is a newtype, not an alias
///
/// The other four pools in this family are type aliases over
/// [`Pool<T>`]. `TextPool` is the exception because [`TextPool::new`]
/// runs a sanitize pass that the generic pool does not. That is a
/// behavioral difference, not a docstring difference, so the family
/// stays honest by keeping `TextPool` as a thin wrapper.
///
/// # Example
/// ```
/// use tui_vfx_content::pool::{TextPool, PoolPolicy};
/// let pool = TextPool::new(
///     vec!["Welcome!".into(), "Salut!".into(), "こんにちは".into()],
///     PoolPolicy::Random,
/// );
/// let chosen = pool.pick();
/// assert!(chosen.is_some());
/// ```
#[derive(
    Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize, tui_vfx_core::ConfigSchema,
)]
#[serde(transparent)]
pub struct TextPool(Pool<String>);

impl TextPool {
    /// Construct a new pool, sanitizing every entry by stripping control
    /// bytes (including `\x1b` ESC) and newlines.
    pub fn new(items: Vec<String>, policy: PoolPolicy) -> Self {
        let sanitized = items.into_iter().map(|s| sanitize(&s)).collect();
        Self(Pool::new(sanitized, policy))
    }

    /// Pick one entry according to this pool's policy. Returns `None`
    /// when the pool is empty — callers should fall through to a default
    /// (a static text field in the same content config, typically).
    pub fn pick(&self) -> Option<&str> {
        self.0.pick().map(String::as_str)
    }

    /// True if the pool has no items.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl Deref for TextPool {
    type Target = Pool<String>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Strip control bytes (including `\x1b` ESC, `\n`, `\t`, etc.) from a
/// pool entry so it can't corrupt downstream render paths with stray
/// ANSI sequences or layout-breaking whitespace. Unicode graphemes pass
/// through unchanged — only ASCII-control and Unicode-category-Cc code
/// points are filtered.
fn sanitize(s: &str) -> String {
    s.chars().filter(|c| !c.is_control()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_pool_returns_none() {
        let pool = TextPool::default();
        assert!(pool.pick().is_none());
        assert!(pool.is_empty());
    }

    #[test]
    fn first_only_returns_first_item() {
        let pool = TextPool::new(
            vec!["first".into(), "second".into(), "third".into()],
            PoolPolicy::FirstOnly,
        );
        assert_eq!(pool.pick(), Some("first"));
    }

    #[test]
    fn random_returns_one_of_the_items() {
        let pool = TextPool::new(vec!["a".into(), "b".into(), "c".into()], PoolPolicy::Random);
        let chosen = pool.pick().unwrap();
        assert!(["a", "b", "c"].contains(&chosen));
    }

    #[test]
    fn sanitization_strips_escape_sequences() {
        let pool = TextPool::new(
            vec!["plain\x1b[31mred\x1b[0m".into()],
            PoolPolicy::FirstOnly,
        );
        let picked = pool.pick().unwrap();
        assert!(!picked.contains('\x1b'));
        assert!(picked.contains("plain"));
    }

    #[test]
    fn sanitization_strips_newlines_and_tabs() {
        let pool = TextPool::new(vec!["line\nbreak\there".into()], PoolPolicy::FirstOnly);
        assert_eq!(pool.pick(), Some("linebreakhere"));
    }

    #[test]
    fn unicode_graphemes_survive_sanitization() {
        let pool = TextPool::new(vec!["こんにちは 🚀".into()], PoolPolicy::FirstOnly);
        assert_eq!(pool.pick(), Some("こんにちは 🚀"));
    }

    #[test]
    fn serde_roundtrip() {
        let pool = TextPool::new(vec!["one".into(), "two".into()], PoolPolicy::Random);
        let json = serde_json::to_string(&pool).unwrap();
        let back: TextPool = serde_json::from_str(&json).unwrap();
        assert_eq!(pool, back);
    }

    #[test]
    fn deref_exposes_pool_items() {
        let pool = TextPool::new(vec!["alpha".into(), "beta".into()], PoolPolicy::FirstOnly);
        assert_eq!(pool.items.len(), 2);
        assert_eq!(pool.policy, PoolPolicy::FirstOnly);
    }
}

// <FILE>crates/tui-vfx-content/src/pool/cls_text_pool.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
