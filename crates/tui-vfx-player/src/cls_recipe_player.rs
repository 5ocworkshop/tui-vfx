// <FILE>crates/tui-vfx-player/src/cls_recipe_player.rs</FILE> - <DESC>Contract-native skeleton recipe player</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player orchestration work: keep coordinator focused after adapter extraction.</WCTX>
// <CLOG>0.2.0: PATCH — split scene, effect, and frame helpers into OFPF-sized modules.
// 0.1.0: INIT — add supported primitive adapters and explicit unsupported diagnostics.</CLOG>

use tui_vfx_contract::{DescriptorCatalog, RecipeDocument};

use crate::{
    PlayerError, PlayerFrame, PlayerFrameReport, PlayerSampleRequest, PlayerStatus,
    PlayerStyledGrid, fnc_apply_graph_effects::apply_graph_effects,
    fnc_build_player_frame::build_player_frame, fnc_render_hash::render_hash,
    fnc_render_scene::render_scene,
};

/// Contract-native player for a minimal primitive adapter subset.
#[derive(Clone, Debug, Default)]
pub struct RecipePlayer {
    catalog: DescriptorCatalog,
}

impl RecipePlayer {
    /// Create a player backed by a loaded descriptor catalog.
    pub fn new(catalog: DescriptorCatalog) -> Self {
        Self { catalog }
    }

    /// Render one canonical recipe sample into a stable frame report.
    pub fn render_recipe(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> PlayerFrameReport {
        if let Err(error) = recipe.validate_with_catalog(&self.catalog) {
            return self.error_report(recipe, request, format!("{error:?}"));
        }
        let (mut rows, mut errors) = render_scene(recipe, request);
        let mut styled_grid = PlayerStyledGrid::from_rows(&rows);
        apply_graph_effects(recipe, request, &mut rows, &mut styled_grid, &mut errors);
        let status = if errors.is_empty() {
            PlayerStatus::Rendered
        } else {
            PlayerStatus::Unsupported
        };
        let styled_grid = styled_grid.style_known().then_some(styled_grid);
        let frame = build_player_frame(recipe, request, &rows, &errors, styled_grid);
        PlayerFrameReport::from_frame(
            recipe.id.as_str().to_string(),
            frame,
            status,
            request,
            false,
            errors,
        )
    }

    fn error_report(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
        message: String,
    ) -> PlayerFrameReport {
        let frame = PlayerFrame {
            width: request.width.unwrap_or(0),
            height: request.height.unwrap_or(0),
            render_hash: render_hash(&[recipe.id.as_str().to_string(), message.clone()]),
            non_empty_cells: 0,
            rows: vec![],
            styled_grid: None,
        };
        PlayerFrameReport::from_frame(
            recipe.id.as_str().to_string(),
            frame,
            PlayerStatus::Error,
            request,
            false,
            vec![PlayerError::new(
                "contractValidationFailed",
                "$",
                message,
                Some("Validate the canonical recipe and descriptor packs before rendering."),
                serde_json::Value::Null,
            )],
        )
    }
}

// <FILE>crates/tui-vfx-player/src/cls_recipe_player.rs</FILE> - <DESC>Contract-native skeleton recipe player</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
