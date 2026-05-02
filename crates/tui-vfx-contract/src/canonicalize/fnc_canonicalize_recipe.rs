// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Add canonicalize_recipe_with_templates entry that resolves the extends: chain before the lift sequence.</WCTX>
// <CLOG>0.5.0: MINOR — add canonicalize_recipe_with_templates entry; resolve extends before the lift sequence.</CLOG>

use std::collections::BTreeMap;

use serde_json::Value;

use crate::RecipeDocument;

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::cls_recipe_intent::RecipeIntent;
use super::fnc_default_recipe::apply_recipe_defaults;
use super::fnc_lift_assets::lift_assets;
use super::fnc_lift_bindings_to_signals::lift_bindings_to_signals;
use super::fnc_lift_card_to_source::lift_card_to_source;
use super::fnc_lift_effects_to_nodes::lift_effects_to_nodes;
use super::fnc_lift_lifecycle::lift_lifecycle;
use super::fnc_lift_scene_array::lift_scene_array;
use super::fnc_lift_top_level_extras::{apply_anchor_to_default_element, lift_top_level_extras};
use super::fnc_lift_transitions::lift_transitions;
use super::fnc_resolve_extends::resolve_extends;

/// Translate authoring shorthand into a canonical [`RecipeDocument`].
///
/// The function mutates a working copy of the input JSON tree, applies
/// canonicalization passes in order, then deserializes the result.
/// Returns a structurally-valid `RecipeDocument` — descriptor catalog
/// resolution, signal references, and other semantic checks run later in
/// `LoadedRecipe::load`. The recipes in the authoring corpus reference
/// descriptors via `descriptorPacks`, so a plain `RecipeDocument::validate`
/// at the end of canonicalize would reject most legitimate inputs.
///
/// Errors carry a JSON-path stack pointing at the failure site.
///
/// Use [`canonicalize_recipe_with_templates`] when the input may declare
/// `extends: "<template-path>"`; this entry rejects that shape with
/// [`CanonicalizationErrorKind::ExtendsTargetNotFound`] because no template
/// map is supplied.
pub fn canonicalize_recipe(input: Value) -> Result<RecipeDocument, CanonicalizationError> {
    let templates: BTreeMap<String, Value> = BTreeMap::new();
    canonicalize_recipe_with_templates(input, &templates)
}

/// Translate authoring shorthand into a canonical [`RecipeDocument`], first
/// resolving any `extends: "<path>"` chain against the supplied template map.
/// Each template is itself a recipe-shaped JSON value (including possibly its
/// own `extends`); deep-merge follows the child-wins rule and detects cycles.
pub fn canonicalize_recipe_with_templates(
    input: Value,
    templates: &BTreeMap<String, Value>,
) -> Result<RecipeDocument, CanonicalizationError> {
    let mut tree = input;

    let extends_chain = resolve_extends(&mut tree, templates)?;
    let extras = lift_top_level_extras(&mut tree)?;
    lift_lifecycle(&mut tree)?;
    lift_assets(&mut tree)?;
    lift_scene_array(&mut tree)?;
    lift_card_to_source(&mut tree)?;
    lift_bindings_to_signals(&mut tree)?;
    let alias_usages = lift_effects_to_nodes(&mut tree)?;
    let preset_usages = lift_transitions(&mut tree)?;
    apply_recipe_defaults(&mut tree)?;

    if let Some(anchor) = extras.anchor.as_deref()
        && let Some(scenes) = tree.get_mut("scenes").and_then(Value::as_array_mut)
    {
        apply_anchor_to_default_element(scenes, anchor)?;
    }

    let mut recipe: RecipeDocument = serde_json::from_value(tree).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::SerdeError {
                underlying: e.to_string(),
            },
            e.to_string(),
        )
    })?;

    let has_provenance =
        !alias_usages.is_empty() || !preset_usages.is_empty() || !extends_chain.is_empty();
    if has_provenance {
        let intent = recipe.intent.get_or_insert_with(RecipeIntent::default);
        intent.alias_usages.extend(alias_usages);
        intent.preset_usages.extend(preset_usages);
        intent.extends_chain.extend(extends_chain);
    }

    Ok(recipe)
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
