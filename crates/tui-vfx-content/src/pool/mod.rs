// <FILE>crates/tui-vfx-content/src/pool/mod.rs</FILE> - <DESC>General-purpose content-randomization primitives. Single generic Pool&lt;T&gt; with type aliases for the four canonical concrete pools (ImagePool, FontPool, EffectPool, PresetPool); TextPool stays as a sibling newtype with sanitize-on-construct. Splash taglines, dialog systems, game text variety, and error-message rotation all use these.</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Buy-once sweep finding 1.2.B — five hand-rolled pool types collapse into Pool&lt;T&gt; with four type aliases. The Preset item type extracts to its own file so PresetPool becomes Pool&lt;Preset&gt;. TextPool stays as a newtype to preserve sanitize-on-construct.</WCTX>
// <CLOG>0.3.0: register Pool&lt;T&gt; (cls_pool) and Preset (cls_preset); replace per-pool re-exports with four type aliases. Move four hand-rolled pool files (cls_image_pool, cls_font_pool, cls_effect_pool, cls_preset_pool) to recyclebin/.
// 0.2.0: add ImagePool + FontPool (name-reference items, AssetMap-resolved by the caller).
// 0.1.0: initial; pool types shared across every tui-vfx consumer.</CLOG>

//! Randomization pools for tui-vfx content.
//!
//! These primitives are general-purpose — they live in
//! `tui-vfx-content` (not in any specific downstream crate) so every
//! tui-vfx consumer benefits: splash taglines, dialog systems, casual
//! terminal games, error-message variety, seasonal Easter eggs.
//!
//! The family:
//! - [`Pool<T>`]: the canonical generic pool with `{ items, policy }`
//!   and the `new` / `pick` / `is_empty` API.
//! - [`TextPool`]: rotate a string with sanitize-on-construct
//!   (taglines, dialog lines). Sibling newtype rather than a
//!   `Pool<String>` alias because sanitize is behavioral.
//! - [`EffectPool`]: rotate a [`ContentEffect`](crate::types::ContentEffect)
//!   (typewriter, scramble, marquee, split-flap, …) — alias of
//!   `Pool<ContentEffect>`.
//! - [`ImagePool`]: rotate a rocketsplash `.rss` asset key — alias of
//!   `Pool<String>`.
//! - [`FontPool`]: rotate a rocketsplash `.rsf` font atlas key — alias
//!   of `Pool<String>`.
//! - [`PresetPool`] of [`Preset`]: curated bundles where specific
//!   text + effect + image + font pairings are authored deliberately
//!   — alias of `Pool<Preset>`.
//!
//! # Asset naming convention
//!
//! [`ImagePool`] and [`FontPool`] store *asset names* (strings), not
//! raw bytes. The caller is expected to maintain an AssetMap that
//! resolves each name to `.rss` or `.rsf` bytes at render time.
//! Naming parallels the substitution-token pattern — recipes stay
//! lightweight, and the caller owns distribution (embed via
//! `include_bytes!`, load from disk, stream from network, whatever).
//!
//! All variants share:
//! - [`PoolPolicy`]: how an item is chosen (Random / FirstOnly)
//! - [`pick_index`]: the underlying time-seeded picker (no `rand` dep)
//!
//! # Precedence
//!
//! When a content config carries multiple pool fields, the recommended
//! precedence is: `PresetPool` > (`TextPool` × `EffectPool`) > static
//! `text` + `effect`. The precedence logic itself lives on the schema's
//! content-config accessors, not on the pool types, which just pick.

mod cls_pool;
mod cls_preset;
mod cls_text_pool;
mod col_pool_policy;
mod fnc_pick_index;

pub use cls_pool::Pool;
pub use cls_preset::Preset;
pub use cls_text_pool::TextPool;
pub use col_pool_policy::PoolPolicy;
pub use fnc_pick_index::pick_index;

use crate::types::ContentEffect;

/// Pool of rocketsplash image asset *names* (strings the caller resolves
/// to `.rss` bytes via an AssetMap). Inline byte vectors in a recipe
/// would force 30–100KB duplication per entry and break JSON
/// serialization; name references keep recipes lightweight and let the
/// caller own asset distribution. Alias of [`Pool<String>`].
pub type ImagePool = Pool<String>;

/// Pool of rocketsplash font atlas *names* (strings the caller resolves
/// to `.rsf` bytes via an AssetMap). Same name-not-bytes contract as
/// [`ImagePool`]. Alias of [`Pool<String>`].
pub type FontPool = Pool<String>;

/// Pool of [`ContentEffect`] values rotated per launch — the author can
/// rotate text, effect, or both independently. For curated text+effect
/// pairings (where specific lines should always use specific effects),
/// see [`PresetPool`]. Alias of `Pool<ContentEffect>`.
pub type EffectPool = Pool<ContentEffect>;

/// Pool of curated [`Preset`] bundles. When a `ContentConfig` carries
/// both a `PresetPool` and independent [`TextPool`] / [`EffectPool`]
/// fields, the preset pool wins if non-empty — a curated bundle
/// represents deliberate authorial intent. Precedence logic lives on
/// the schema's content-config accessors, not on the pool itself.
/// Alias of [`Pool<Preset>`].
pub type PresetPool = Pool<Preset>;

// <FILE>crates/tui-vfx-content/src/pool/mod.rs</FILE>
// <VERS>END OF VERSION: 0.3.0</VERS>
