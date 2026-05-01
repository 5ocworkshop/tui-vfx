// <FILE>crates/tui-vfx-compost/src/loader/cls_loaded_recipe.rs</FILE> - <DESC>Load-validated native v3.1 recipe wrapper</DESC>
// <VERS>VERSION: 0.1.1</VERS>
// <WCTX>LoadedRecipe owns the accepted canonical v3.1 RecipeDocument and stays thin.</WCTX>
// <CLOG>0.1.1: PATCH — reject unsupported recipe versions before descriptor validation.</CLOG>

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

    /// Borrow the validated canonical recipe document.
    pub fn recipe(&self) -> &RecipeDocument {
        &self.recipe
    }
}

// <FILE>crates/tui-vfx-compost/src/loader/cls_loaded_recipe.rs</FILE> - <DESC>Load-validated native v3.1 recipe wrapper</DESC>
// <VERS>END OF VERSION: 0.1.1</VERS>
