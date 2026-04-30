// <FILE>crates/tui-vfx-player/src/cls_recipe_player.rs</FILE> - <DESC>Contract-native skeleton recipe player</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Native compositor source isolation: expose a source-only render IR for backend adapters that apply effects natively.</WCTX>
// <CLOG>0.3.0: MINOR — add source-only render IR that omits recipe-level graph effect adapters for native backends.
// 0.2.0: PATCH — split scene, effect, and frame helpers into OFPF-sized modules.
// 0.1.0: INIT — add supported primitive adapters and explicit unsupported diagnostics.</CLOG>

use std::collections::BTreeMap;

use tui_vfx_contract::{
    ClockMode, DescriptorCatalog, DurationSpec, GraphValueId, RecipeDocument, Value,
};

use crate::{
    PlayerError, PlayerFrame, PlayerFrameReport, PlayerRenderIrReport, PlayerSampleRequest,
    PlayerStatus, fnc_apply_authored_loopback_indicator::apply_authored_loopback_indicator,
    fnc_apply_graph_effects::apply_graph_effects,
    fnc_apply_preview_loopbacks::resolve_preview_loopbacks,
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

    /// Borrow the descriptor catalog used by this player instance.
    pub fn descriptor_catalog(&self) -> &DescriptorCatalog {
        &self.catalog
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
        let (report, graph_values, effective_request) =
            self.render_recipe_with_graph_values(recipe, request);
        build_player_render_ir(recipe, &effective_request, report, graph_values)
    }

    /// Render one canonical recipe sample into source-only player IR.
    ///
    /// This preserves scene/source rendering and provenance while intentionally
    /// skipping recipe-level graph effects. Native render backends use this as
    /// their source substrate before applying graph effects through backend
    /// native primitives.
    pub fn render_recipe_source_ir(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> PlayerRenderIrReport {
        let effective_request = request_with_recipe_clock_sample(recipe, request);
        let report = self.render_recipe_source_report(recipe, &effective_request);
        build_player_render_ir(recipe, &effective_request, report, BTreeMap::new())
    }

    fn render_recipe_source_report(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> PlayerFrameReport {
        if let Err(error) = recipe.validate_with_catalog(&self.catalog) {
            return self.error_report(recipe, request, format!("{error:?}"));
        }
        let sample_request = request_with_recipe_clock_sample(recipe, request);
        let resolution = resolve_preview_loopbacks(recipe, &sample_request);
        let (mut rows, mut styled_grid, errors, mut warnings) =
            render_scene(recipe, &resolution.request);
        warnings.extend(resolution.warnings());
        apply_authored_loopback_indicator(&mut rows, &mut styled_grid, &resolution.fired_keys);
        let status = if errors.is_empty() {
            PlayerStatus::Rendered
        } else {
            PlayerStatus::Unsupported
        };
        let styled_grid = styled_grid.style_known().then_some(styled_grid);
        let frame = build_player_frame(recipe, &resolution.request, &rows, &errors, styled_grid);
        PlayerFrameReport::from_frame_with_warnings(
            recipe.id.as_str().to_string(),
            frame,
            status,
            &resolution.request,
            false,
            errors,
            warnings,
        )
    }

    fn render_recipe_with_graph_values(
        &self,
        recipe: &RecipeDocument,
        request: &PlayerSampleRequest,
    ) -> (
        PlayerFrameReport,
        BTreeMap<GraphValueId, Value>,
        PlayerSampleRequest,
    ) {
        if let Err(error) = recipe.validate_with_catalog(&self.catalog) {
            return (
                self.error_report(recipe, request, format!("{error:?}")),
                BTreeMap::new(),
                request.clone(),
            );
        }
        let sample_request = request_with_recipe_clock_sample(recipe, request);
        let resolution = resolve_preview_loopbacks(recipe, &sample_request);
        let (mut rows, mut styled_grid, mut errors, mut warnings) =
            render_scene(recipe, &resolution.request);
        warnings.extend(resolution.warnings());
        let mut graph_request = resolution.request.clone();
        apply_graph_effects(
            recipe,
            Some(&self.catalog),
            &mut graph_request,
            &mut rows,
            &mut styled_grid,
            &mut errors,
            &mut warnings,
        );
        apply_authored_loopback_indicator(&mut rows, &mut styled_grid, &resolution.fired_keys);
        let status = if errors.is_empty() {
            PlayerStatus::Rendered
        } else {
            PlayerStatus::Unsupported
        };
        let styled_grid = styled_grid.style_known().then_some(styled_grid);
        let frame = build_player_frame(recipe, &graph_request, &rows, &errors, styled_grid);
        let report = PlayerFrameReport::from_frame_with_warnings(
            recipe.id.as_str().to_string(),
            frame,
            status,
            &graph_request,
            false,
            errors,
            warnings,
        );
        (report, graph_request.graph_values.clone(), graph_request)
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

fn request_with_recipe_clock_sample(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> PlayerSampleRequest {
    let Some(lifecycle) = recipe.lifecycle.as_ref() else {
        return request.clone();
    };
    match lifecycle.clock.mode {
        ClockMode::Looping => {
            let Some(period_ms) = recipe_clock_period_ms(recipe) else {
                return request.clone();
            };
            let absolute_t_ms = request.absolute_t_ms.or_else(|| {
                request
                    .loop_t
                    .map(|loop_t| loop_t.clamp(0.0, 1.0) * period_ms)
            });
            let loop_t = absolute_t_ms
                .map(|absolute_t_ms| absolute_t_ms.max(0.0).rem_euclid(period_ms) / period_ms)
                .or(request.loop_t);
            PlayerSampleRequest {
                absolute_t_ms,
                loop_t,
                ..request.clone()
            }
        }
        ClockMode::Monotonic => PlayerSampleRequest {
            loop_t: None,
            ..request.clone()
        },
    }
}

fn recipe_clock_period_ms(recipe: &RecipeDocument) -> Option<f64> {
    let lifecycle = recipe.lifecycle.as_ref()?;
    if lifecycle.clock.mode != ClockMode::Looping {
        return None;
    }
    match lifecycle.clock.period? {
        DurationSpec::Milliseconds { value } => Some(value as f64),
        DurationSpec::Seconds { value } => Some(value * 1000.0),
    }
    .filter(|period_ms| period_ms.is_finite() && *period_ms > 0.0)
}

// <FILE>crates/tui-vfx-player/src/cls_recipe_player.rs</FILE> - <DESC>Contract-native skeleton recipe player</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
