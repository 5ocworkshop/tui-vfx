// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/mod.rs</FILE> - <DESC>Overlay primitives painted onto a final OwnedGrid after the render pipeline finishes — abandoned hardcoded-cell-painter approach to the L3 LB visibility badge, superseded by Intention 39's recipe-as-source-of-truth model</DESC>
// <VERS>VERSION: 0.1.0 (recycled)</VERS>
// <WCTX>Loopback Phase L3 first attempt (recycled 2026-04-26): introduced an `overlays` module under tui-vfx-compositor with a hardcoded `apply_loopback_badge` function painting orange `[LB]` / nf-warning cells directly. The user's correction landed before the build went green: "Don't re-invent or hard code things we can do with recipes and maintain flexibility." See Intention 39 for the resulting principle and the recipe-based replacement architecture.</WCTX>
// <CLOG>0.1.0 (recycled): preserved here as the artefact that surfaced Intention 39. Do not restore — the L3 badge is now a V3 recipe inlined via include_str! and rendered through the standard recipe path.</CLOG>

//! Overlay primitives painted onto the final rendered grid.
//!
//! ABANDONED: superseded by recipe-based architecture per Intention 39.

pub mod cls_loopback_badge_state;
pub mod enum_loopback_badge_style;
pub mod fnc_apply_loopback_badge;

pub use cls_loopback_badge_state::LoopbackBadgeState;
pub use enum_loopback_badge_style::LoopbackBadgeStyle;
pub use fnc_apply_loopback_badge::apply_loopback_badge;

// <FILE>recyclebin/crates/tui-vfx-compositor/src/overlays/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0 (recycled)</VERS>
