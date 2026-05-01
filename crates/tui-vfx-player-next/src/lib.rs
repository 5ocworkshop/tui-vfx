// <FILE>crates/tui-vfx-player-next/src/lib.rs</FILE> - <DESC>Pure v3.1 player-next facade</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Thin player-next path for v3.1 visual end-to-end tests; loader validation lives in compositor-next v31.</WCTX>
// <CLOG>0.1.0: INIT — expose direct load/render calls without legacy compatibility layers.</CLOG>

//! Pure v3.1 player-next facade for visual end-to-end testing.
//!
//! This crate intentionally owns no recipe acceptance rules. It delegates load validation and
//! rendering to `tui_vfx_compositor_next::v31` so there is one v3.1 acceptance path.

use tui_vfx_compositor_next::v31::{
    LoadedV31Recipe, V31Frame, V31LoadError, V31RenderError, V31SampleContext, render_v31_recipe,
};
use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

/// Load a canonical v3.1 recipe through the single compositor-next acceptance path.
pub fn load_player_next_recipe(
    recipe: RecipeDocument,
    catalog: &DescriptorCatalog,
) -> Result<LoadedV31Recipe, V31LoadError> {
    LoadedV31Recipe::load(recipe, catalog)
}

/// Render a load-validated canonical v3.1 recipe through compositor-next.
pub fn render_player_next_recipe(
    loaded: &LoadedV31Recipe,
    sample: &V31SampleContext,
) -> Result<V31Frame, V31RenderError> {
    render_v31_recipe(loaded, sample)
}

// <FILE>crates/tui-vfx-player-next/src/lib.rs</FILE> - <DESC>Pure v3.1 player-next facade</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
