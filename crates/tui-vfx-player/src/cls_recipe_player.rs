// <FILE>crates/tui-vfx-player/src/cls_recipe_player.rs</FILE> - <DESC>Contract-native skeleton recipe player</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Player orchestration work: keep coordinator focused after adapter extraction.</WCTX>
// <CLOG>0.2.0: PATCH — split scene, effect, and frame helpers into OFPF-sized modules.
// 0.1.0: INIT — add supported primitive adapters and explicit unsupported diagnostics.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{DescriptorCatalog, GraphValueId, RecipeDocument, Value};

use crate::{
    PlayerError, PlayerFrame, PlayerFrameReport, PlayerRenderIrReport, PlayerSampleRequest,
    PlayerStatus, fnc_apply_graph_effects::apply_graph_effects,
    fnc_build_player_frame::build_player_frame, fnc_build_player_render_ir::build_player_render_ir,
    fnc_render_hash::render_hash, fnc_render_scene::render_scene,
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
        self.render_recipe_with_graph_values(recipe, request).0
    }

    /// Render one canonical recipe sample into the player-owned render IR.
    pub fn render_recipe_ir(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> PlayerRenderIrReport {
        let (report, graph_values) = self.render_recipe_with_graph_values(recipe, request);
        build_player_render_ir(recipe, request, report, graph_values)
    }

    fn render_recipe_with_graph_values(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> (PlayerFrameReport, BTreeMap<GraphValueId, Value>) {
        if let Err(error) = recipe.validate_with_catalog(&self.catalog) {
            return (
                self.error_report(recipe, request, format!("{error:?}")),
                BTreeMap::new(),
            );
        }
        let (mut rows, mut styled_grid, mut errors, mut warnings) = render_scene(recipe, request);
        let mut graph_request = request.clone();
        apply_graph_effects(
            recipe,
            Some(&self.catalog),
            &mut graph_request,
            &mut rows,
            &mut styled_grid,
            &mut errors,
            &mut warnings,
        );
        let status = if errors.is_empty() {
            PlayerStatus::Rendered
        } else {
            PlayerStatus::Unsupported
        };
        let styled_grid = styled_grid.style_known().then_some(styled_grid);
        let frame = build_player_frame(recipe, request, &rows, &errors, styled_grid);
        let report = PlayerFrameReport::from_frame_with_warnings(
            recipe.id.as_str().to_string(),
            frame,
            status,
            request,
            false,
            errors,
            warnings,
        );
        (report, graph_request.graph_values)
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
