// <FILE>crates/tui-vfx-types/src/scene_metadata.rs</FILE> - <DESC>Inspection metadata attached to every SemanticScene</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Sub-plan A Phase A.1 — foundation primitive for scene-level trace metadata (spec R5)</WCTX>
// <CLOG>0.1.0: initial metadata struct with recipe_id / composer_version / produced_at / layer_count; non_exhaustive; Default impl; serde via cfg_attr.</CLOG>

//! Metadata attached to every `SemanticScene` for inspection / trace use.
//!
//! The metadata is optional where it makes sense: widget-rendered
//! `SemanticScene`s (from `gtd-factory::SemanticBuffer` lowering) have no
//! `recipe_id`; composer-produced scenes do. `SceneMetadata` is
//! `#[non_exhaustive]` so additional inspection fields can be added
//! without a breaking change.
//!
//! # Examples
//!
//! ```
//! use tui_vfx_types::{RecipeId, SceneMetadata};
//!
//! let mut md = SceneMetadata::default();
//! md.recipe_id = Some(RecipeId::from("splash.v2"));
//! md.layer_count = 3;
//! assert_eq!(md.layer_count, 3);
//! ```

use crate::RecipeId;

/// Inspection metadata attached to a `SemanticScene`.
///
/// `#[non_exhaustive]` on the struct prevents foreign crates from
/// brace-initialising it directly; use `Default::default()` and mutate
/// the desired fields instead.
#[non_exhaustive]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SceneMetadata {
    /// Recipe that produced this scene, if any.
    pub recipe_id: Option<RecipeId>,
    /// Version string of the composer that produced this scene.
    pub composer_version: Option<String>,
    /// Wall-clock timestamp (milliseconds since UNIX epoch) of the producing frame.
    pub produced_at: Option<u64>,
    /// Number of layers the composer merged into this scene (0 for non-composed sources).
    pub layer_count: usize,
}

// <FILE>crates/tui-vfx-types/src/scene_metadata.rs</FILE> - <DESC>Scene-level trace metadata</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
