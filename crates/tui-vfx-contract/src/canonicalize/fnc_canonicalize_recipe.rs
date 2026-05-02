// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase 1 of canonicalize: wire the lift passes that handle baseline.json end-to-end.</WCTX>
// <CLOG>0.1.0: INIT — sequence card-lift and defaults; deserialize via serde_json::from_value; defer descriptor validation to the loader.</CLOG>

use serde_json::Value;

use crate::RecipeDocument;

use super::cls_canonicalization_error::{CanonicalizationError, CanonicalizationErrorKind};
use super::fnc_default_recipe::apply_recipe_defaults;
use super::fnc_lift_card_to_source::lift_card_to_source;

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
    apply_recipe_defaults(&mut tree)?;

    serde_json::from_value(tree).map_err(|e| {
        CanonicalizationError::new(
            CanonicalizationErrorKind::SerdeError {
                underlying: e.to_string(),
            },
            e.to_string(),
        )
    })
}

// <FILE>crates/tui-vfx-contract/src/canonicalize/fnc_canonicalize_recipe.rs</FILE> - <DESC>Top-level canonicalize orchestrator</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
