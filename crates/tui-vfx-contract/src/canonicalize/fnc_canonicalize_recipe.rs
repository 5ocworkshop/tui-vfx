// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase 2 of canonicalize: lift effects[] into graph nodes and record alias-usage provenance.</WCTX>
// <CLOG>0.2.0: MINOR — add effects-to-nodes pass plus RecipeIntent.alias_usages population.
// 0.1.0: INIT — sequence card-lift and defaults; deserialize via serde_json::from_value; defer descriptor validation to the loader.</CLOG>

use serde_json::Value;

use crate::RecipeDocument;

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::cls_recipe_intent::RecipeIntent;
use super::fnc_default_recipe::apply_recipe_defaults;
use super::fnc_lift_card_to_source::lift_card_to_source;
use super::fnc_lift_effects_to_nodes::lift_effects_to_nodes;

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
pub fn canonicalize_recipe(input: Value) -> Result<RecipeDocument, CanonicalizationError> {
    let mut tree = input;

    lift_card_to_source(&mut tree)?;
    let alias_usages = lift_effects_to_nodes(&mut tree)?;
    apply_recipe_defaults(&mut tree)?;

    let mut recipe: RecipeDocument = serde_json::from_value(tree).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::SerdeError {
                underlying: e.to_string(),
            },
            e.to_string(),
        )
    })?;

    if !alias_usages.is_empty() {
        let intent = recipe.intent.get_or_insert_with(RecipeIntent::default);
        intent.alias_usages.extend(alias_usages);
    }

    Ok(recipe)
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
