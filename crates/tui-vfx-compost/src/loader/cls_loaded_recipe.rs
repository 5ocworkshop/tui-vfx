// <FILE>crates/tui-vfx-compost/src/loader/cls_loaded_recipe.rs</FILE> - <DESC>Load-validated native v3.1 recipe wrapper</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Add a single-step authoring-shorthand load entry point so callers no longer have to invoke canonicalize and load_canonical separately.</WCTX>
// <CLOG>0.2.0: MINOR — add load_authoring_shorthand bridging canonicalize_recipe_with_templates and the canonical-recipe load path.</CLOG>

use std::collections::BTreeMap;

use serde_json::Value;
use tui_vfx_contract::canonicalize::canonicalize_recipe_with_templates;
use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use crate::validation::validate_render_contract;

use super::LoadError;

/// Canonical v3.1 recipe accepted after load-time validation.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedRecipe {
    recipe: RecipeDocument,
}

impl LoadedRecipe {
    /// Validate a canonical v3.1 recipe once at load time.
    pub fn load(recipe: RecipeDocument, catalog: &DescriptorCatalog) -> Result<Self, LoadError> {
        if recipe.version != "3.1" || recipe.graph.version != "3.1" {
            return Err(LoadError::UnsupportedVersion {
                recipe_version: recipe.version.clone(),
                graph_version: recipe.graph.version.clone(),
            });
        }
        recipe.validate_with_catalog(catalog)?;
        validate_render_contract(&recipe)?;
        Ok(Self { recipe })
    }

    /// Translate authoring shorthand JSON into a canonical v3.1 recipe and
    /// validate it in one step. `templates` resolves any `extends:` chain in
    /// the input; pass an empty map when no templates are needed. The result
    /// is the same as calling [`canonicalize_recipe_with_templates`] followed
    /// by [`LoadedRecipe::load`]; any failure surfaces as a `LoadError`,
    /// either as `Canonicalize { … }` for shorthand-translation failures or
    /// the existing variants for canonical-side validation failures.
    pub fn load_authoring_shorthand(
        json_value: Value,
        templates: &BTreeMap<String, Value>,
        catalog: &DescriptorCatalog,
    ) -> Result<Self, LoadError> {
        let recipe = canonicalize_recipe_with_templates(json_value, templates)?;
        Self::load(recipe, catalog)
    }

    /// Borrow the validated canonical recipe document.
    pub fn recipe(&self) -> &RecipeDocument {
        &self.recipe
    }
}

// <FILE>crates/tui-vfx-compost/src/loader/cls_loaded_recipe.rs</FILE> - <DESC>Load-validated native v3.1 recipe wrapper</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
