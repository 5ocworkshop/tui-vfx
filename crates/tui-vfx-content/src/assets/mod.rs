// <FILE>crates/tui-vfx-content/src/assets/mod.rs</FILE> - <DESC>Asset registry module — name → bytes mapping consumed by future scene-layer source variants that load rocketsplash images and other byte-source assets by name</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 7 (breadcrumb) of mechanical circular content cycles plan: byte-supplying side of the asset-binding shape mirrors FontRegistry's name-with-default-sentinel routing for ergonomic consistency. Consuming source surface deferred until V3 scene-layer composition decisions land.</WCTX>
// <CLOG>0.1.0: introduce assets module with cls_asset_registry; export AssetRegistry and DEFAULT_LOGO_SENTINEL.</CLOG>

//! Asset registries.
//!
//! Currently holds [`AssetRegistry`] — a name → bytes mapping with a
//! registered default and a reserved sentinel literal (`default_logo`)
//! that routes to whatever asset is currently the default. Mirrors the
//! shape of `crate::fonts::FontRegistry` so authoring stays consistent
//! across binding kinds (Phase 6 / Phase 7 of the mechanical circular
//! content cycles plan).
//!
//! The consumer surface — a scene-layer source variant that loads an
//! asset by name and renders it onto a rect — is deferred. See the
//! Phase 7 sub-plan in `docs/design/tui-vfx-mechanical-circular-content-
//! cycles-plan.md` for the target shape.

mod cls_asset_registry;

pub use cls_asset_registry::{AssetRegistry, DEFAULT_LOGO_SENTINEL};

// <FILE>crates/tui-vfx-content/src/assets/mod.rs</FILE>
// <VERS>END OF VERSION: 0.1.0</VERS>
