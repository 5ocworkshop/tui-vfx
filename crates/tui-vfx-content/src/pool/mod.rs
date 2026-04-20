// <FILE>crates/tui-vfx-content/src/pool/mod.rs</FILE> - <DESC>General-purpose content-randomization primitives: TextPool, EffectPool, PresetPool + shared selection policy and picker. Splash taglines, dialog systems, game text variety, and error-message rotation all use these.</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Stage 1.5 of the splash library + VFX integration plan — extend pool family to rocketsplash images and fonts so recipes can cycle logos and font atlases per launch.</WCTX>
// <CLOG>0.2.0: add ImagePool + FontPool (name-reference items, AssetMap-resolved by the caller).
// 0.1.0: initial; pool types shared across every tui-vfx consumer.</CLOG>

//! Randomization pools for tui-vfx content.
//!
//! These primitives are general-purpose — they live in
//! `tui-vfx-content` (not in any specific downstream crate) so every
//! tui-vfx consumer benefits: splash taglines, dialog systems, casual
//! terminal games, error-message variety, seasonal Easter eggs.
//!
//! The family:
//! - [`TextPool`]: rotate a string (taglines, dialog lines)
//! - [`EffectPool`]: rotate a [`ContentEffect`](crate::types::ContentEffect)
//!   (typewriter, scramble, marquee, split-flap, …)
//! - [`ImagePool`]: rotate a rocketsplash `.rss` asset (asset-map keys)
//! - [`FontPool`]: rotate a rocketsplash `.rsf` font atlas (asset-map keys)
//! - [`PresetPool`] of [`Preset`]: curated bundles where specific
//!   text + effect + image + font pairings are authored deliberately
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
//! All three share:
//! - [`PoolPolicy`]: how an item is chosen (Random / FirstOnly)
//! - [`pick_index`]: the underlying time-seeded picker (no `rand` dep)
//!
//! # Precedence
//!
//! When a content config carries multiple pool fields, the recommended
//! precedence is: `PresetPool` > (`TextPool` × `EffectPool`) > static
//! `text` + `effect`. The precedence logic itself lives on the schema's
//! content-config accessors, not on the pool types, which just pick.

mod cls_effect_pool;
mod cls_font_pool;
mod cls_image_pool;
mod cls_preset_pool;
mod cls_text_pool;
mod col_pool_policy;
mod fnc_pick_index;

pub use cls_effect_pool::EffectPool;
pub use cls_font_pool::FontPool;
pub use cls_image_pool::ImagePool;
pub use cls_preset_pool::{Preset, PresetPool};
pub use cls_text_pool::TextPool;
pub use col_pool_policy::PoolPolicy;
pub use fnc_pick_index::pick_index;

// <FILE>crates/tui-vfx-content/src/pool/mod.rs</FILE>
// <VERS>END OF VERSION: 0.2.0</VERS>
