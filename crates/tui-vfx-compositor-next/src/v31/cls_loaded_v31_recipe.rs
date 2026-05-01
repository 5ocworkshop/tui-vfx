// <FILE>crates/tui-vfx-compositor-next/src/v31/cls_loaded_v31_recipe.rs</FILE> - <DESC>Load-validated canonical v3.1 recipe wrapper</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Public loaded-recipe wrapper stays thin; validation orchestration lives in OFPF-sized modules.</WCTX>
// <CLOG>0.1.0: INIT — move LoadedV31Recipe out of the former load hub.</CLOG>

use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use super::V31LoadError;
use super::validation::validate_direct_render_contract;

/// Canonical v3.1 recipe accepted after load-time validation.
#[derive(Clone, Debug, PartialEq)]
pub struct LoadedV31Recipe {
    recipe: RecipeDocument,
}

impl LoadedV31Recipe {
    /// Validate a canonical v3.1 recipe once at load time.
    pub fn load(recipe: RecipeDocument, catalog: &DescriptorCatalog) -> Result<Self, V31LoadError> {
        recipe.validate_with_catalog(catalog)?;
        if recipe.version != "3.1" || recipe.graph.version != "3.1" {
            return Err(V31LoadError::UnsupportedVersion {
                recipe_version: recipe.version.clone(),
                graph_version: recipe.graph.version.clone(),
            });
        }
        validate_direct_render_contract(&recipe)?;
        Ok(Self { recipe })
    }

    /// Borrow the validated canonical recipe document.
    pub fn recipe(&self) -> &RecipeDocument {
        &self.recipe
    }
}

// <FILE>crates/tui-vfx-compositor-next/src/v31/cls_loaded_v31_recipe.rs</FILE> - <DESC>Load-validated canonical v3.1 recipe wrapper</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
