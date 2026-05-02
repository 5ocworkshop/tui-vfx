// <FILE>crates/tui-vfx-contract/src/canonicalize/mod.rs</FILE> - <DESC>Authoring shorthand canonicalize module entry point</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of the canonicalize function: skeleton, table loading, and baseline.json round-trip.</WCTX>
// <CLOG>0.1.0: INIT — module skeleton, table types, baseline-capable canonicalize_recipe entry point.</CLOG>

//! Authoring shorthand → canonical `RecipeDocument` translation.
//!
//! `canonicalize_recipe` accepts a loose JSON [`serde_json::Value`] in the
//! authoring shorthand and produces a strict canonical [`RecipeDocument`]
//! ready for [`LoadedRecipe::load`]. The function consults the alias and
//! expansion tables shipped under `schemas/v3.1/authoring/`; tables are
//! parsed once and cached.
//!
//! Canonicalization is purely structural — the function does not validate
//! descriptor catalogs, signal references, or other runtime constraints.
//! Those checks remain the responsibility of [`RecipeDocument::validate`]
//! and the loader.
//!
//! [`RecipeDocument`]: crate::RecipeDocument
//! [`LoadedRecipe::load`]: ../../tui_vfx_compost/index.html

pub mod cls_alias_table;
pub mod cls_canonicalization_error;
pub mod cls_canonicalization_rules;
pub mod cls_expansion_table;
pub mod cls_recipe_intent;
pub mod fnc_apply_alias;
pub mod fnc_apply_preset;
pub mod fnc_canonicalize_recipe;
pub mod fnc_default_recipe;
pub mod fnc_lift_bindings_to_signals;
pub mod fnc_lift_card_to_source;
pub mod fnc_lift_effects_to_nodes;
pub mod fnc_lift_lifecycle;
pub mod fnc_lift_scene_array;
pub mod fnc_lift_top_level_extras;
pub mod fnc_lift_transitions;
pub mod fnc_lift_value_envelope;
pub mod fnc_load_tables;
pub mod fnc_resolve_color;
pub mod fnc_resolve_duration;
pub mod fnc_resolve_easing;
pub mod fnc_resolve_extends;
pub mod fnc_resolve_phases;
pub mod fnc_resolve_scope;

pub use cls_alias_table::{AliasEntry, AliasTable, ParamMapping};
pub use cls_canonicalization_error::{
    CanonicalizationError, CanonicalizationErrorKind, JsonPathSegment,
};
pub use cls_canonicalization_rules::{
    CanonicalizationRules, NamedColors, NodeFieldLift, ScopeShapeMapEntry, ShadowFieldRename,
};
pub use cls_expansion_table::{ExpansionTable, ParamSpec, PresetEntry};
pub use cls_recipe_intent::{AliasUsage, ExtendsChainEntry, PresetUsage, RecipeIntent};
pub use fnc_canonicalize_recipe::{canonicalize_recipe, canonicalize_recipe_with_templates};

// <FILE>crates/tui-vfx-contract/src/canonicalize/mod.rs</FILE> - <DESC>Authoring shorthand canonicalize module entry point</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
