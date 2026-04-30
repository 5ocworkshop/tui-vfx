// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into player-owned rows and styled cells</DESC>
// <VERS>VERSION: 0.4.1</VERS>
// <WCTX>Scene rendering carries element-local pipeline style evidence into the player surface.</WCTX>
// <CLOG>0.4.1: PATCH — render source.card plain borders with single-line box-drawing glyphs.
// 0.4.0: MINOR — carry bounded ANSI SGR styled-cell evidence and source resolver seams.
// 0.3.0: MINOR — preserve local pipeline styled-cell evidence when placing scene elements.
// 0.2.0: PATCH — distinguish source.card message input from source.text text input.
// 0.1.0: INIT — add scene traversal, source rendering, and grid blitting helpers.</CLOG>

use tui_vfx_contract::{
    AssetId, AssetLocator, AssetSpec, BindingTarget, CellWritePolicy, ParameterId, RecipeDocument,
    RecipeElementPipelineTiming, RecipeSceneElement, SceneAnchor, SceneElementOverflowPolicy,
    SceneElementPlacementRule, SourceInputId, SourceSpec, Value, ValuePredicate,
};

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid, PlayerWarning,
    fnc_apply_graph_effects::apply_graph_step_effects,
    fnc_render_procedural_source::render_registered_procedural_source,
    fnc_resolve_effect_input::ResolvedColor,
    fnc_resolve_source_asset::{
        MissingSourceAssetResolver, PlayerSourceAssetRequest, PlayerSourceAssetResolution,
        PlayerSourceAssetResolver, resolve_image_source_asset_id,
    },
    fnc_resolve_value_source::{resolve_integer, resolve_text, resolve_value_source},
};

/// Runtime visibility and skip result for one scene element.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct SceneElementRenderRuntime {
    pub(crate) scene_id: String,
    pub(crate) element_id: String,
    pub(crate) layer_id: Option<String>,
    pub(crate) visible: bool,
    pub(crate) skipped: bool,
    pub(crate) skip_reason: Option<String>,
}

struct SourceRenderOutput {
    rows: Vec<String>,
    styled_grid: PlayerStyledGrid,
    errors: Vec<PlayerError>,
    warnings: Vec<PlayerWarning>,
}

/// Render the first recipe scene into player-owned rows and styled-cell evidence.
pub fn render_scene(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> (
    Vec<String>,
    PlayerStyledGrid,
    Vec<PlayerError>,
    Vec<PlayerWarning>,
) {
    render_scene_with_source_asset_resolver(recipe, request, &MissingSourceAssetResolver)
}

/// Render the first recipe scene using a caller-supplied source asset resolver.
pub fn render_scene_with_source_asset_resolver(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    asset_resolver: &dyn PlayerSourceAssetResolver,
) -> (
    Vec<String>,
    PlayerStyledGrid,
    Vec<PlayerError>,
    Vec<PlayerWarning>,
) {
    let Some(scene) = recipe.scenes.first() else {
        return (
            vec![],
            PlayerStyledGrid::blank(0, 0, false),
            vec![missing_scene_error()],
            vec![],
        );
    };
    let width = request.width.unwrap_or(scene.width);
    let height = request.height.unwrap_or(scene.height);
    let mut grid = blank_grid(width, height);
    let mut styled_grid = PlayerStyledGrid::blank(width, height, false);
    let mut errors = Vec::new();
    let mut warnings = Vec::new();
    let mut elements = scene.elements.iter().enumerate().collect::<Vec<_>>();
    elements.sort_by_key(|(declaration_index, element)| (element.z_index, *declaration_index));
    let runtime_results = scene_element_render_runtime(recipe, request);
    let mut placed_layers = std::collections::BTreeMap::new();
    for (_, element) in elements {
        let Some(runtime) = runtime_results
            .iter()
            .find(|runtime| runtime.element_id == element.id.as_str())
        else {
            continue;
        };
        if runtime.skipped {
            warnings.push(layer_skipped_warning(runtime));
            continue;
        }
        match recipe.sources.get(&element.source) {
            Some(source) => {
                let mut output = render_source(
                    recipe,
                    element.source.as_str(),
                    source,
                    request,
                    asset_resolver,
                );
                let mut source_rows = output.rows;
                let mut local_grid = output.styled_grid;
                let mut source_errors = output.errors;
                warnings.append(&mut output.warnings);
                apply_scene_element_surface(element, &mut local_grid);
                if let Some(pipeline) = &element.pipeline
                    && let Some(topology) = &pipeline.topology
                {
                    let mut local_request =
                        element_pipeline_request(request, pipeline.timing.as_ref());
                    apply_graph_step_effects(
                        recipe,
                        None,
                        topology,
                        &mut local_request,
                        &mut source_rows,
                        &mut local_grid,
                        &mut source_errors,
                        &mut warnings,
                    );
                }
                let (placement_x, placement_y) = resolve_scene_element_placement(
                    width,
                    height,
                    element,
                    &source_rows,
                    &placed_layers,
                );
                if element_overflow_hides(
                    element,
                    width,
                    height,
                    &source_rows,
                    placement_x,
                    placement_y,
                ) {
                    warnings.push(scene_element_overflow_hidden_warning(element));
                    continue;
                }
                blit_rows(
                    &mut grid,
                    &source_rows,
                    placement_x,
                    placement_y,
                    element.cell_write_policy,
                    element.overflow,
                );
                blit_styles(
                    &mut styled_grid,
                    &local_grid,
                    placement_x,
                    placement_y,
                    element.cell_write_policy,
                    element.overflow,
                );
                if let Some(layer) = &element.layer {
                    placed_layers.insert(
                        layer.clone(),
                        PlacedElementBounds {
                            x: placement_x,
                            y: placement_y,
                            width: source_width_from_rows(&source_rows),
                            height: source_rows.len() as i32,
                        },
                    );
                }
                errors.append(&mut source_errors);
            }
            None => errors.push(PlayerError::new(
                "unknownSourceInstance",
                format!("scenes[0].elements.{}.source", element.id.as_str()),
                format!(
                    "Scene element references missing source `{}`",
                    element.source.as_str()
                ),
                Some("Declare the source instance before rendering."),
                serde_json::Value::Null,
            )),
        }
    }
    let rows = grid_to_rows(&grid);
    styled_grid.sync_glyphs_from_rows(&rows);
    (rows, styled_grid, errors, warnings)
}

fn element_pipeline_request(
    request: &PlayerSampleRequest,
    timing: Option<&RecipeElementPipelineTiming>,
) -> PlayerSampleRequest {
    let Some(timing) = timing else {
        return request.clone();
    };
    let mut local = request.clone();
    match request.phase {
        tui_vfx_contract::LifecyclePhase::Enter => {
            if let Some(duration_ms) = timing.enter_ms.filter(|duration| *duration > 0) {
                let elapsed_ms = request
                    .absolute_t_ms
                    .unwrap_or_else(|| request.phase_t.clamp(0.0, 1.0) * duration_ms as f64);
                let local_elapsed = elapsed_ms - timing.enter_offset_ms.unwrap_or_default() as f64;
                local.phase_t = (local_elapsed / duration_ms as f64).clamp(0.0, 1.0);
            }
        }
        tui_vfx_contract::LifecyclePhase::Exit => {
            if let Some(duration_ms) = timing.exit_ms.filter(|duration| *duration > 0) {
                let elapsed_ms = request
                    .absolute_t_ms
                    .unwrap_or_else(|| request.phase_t.clamp(0.0, 1.0) * duration_ms as f64);
                let local_elapsed = elapsed_ms - timing.exit_offset_ms.unwrap_or_default() as f64;
                local.phase_t = (local_elapsed / duration_ms as f64).clamp(0.0, 1.0);
            }
        }
        tui_vfx_contract::LifecyclePhase::Dwell => {}
    }
    local
}

/// Resolve scene element visibility and skip decisions without rendering sources.
pub(crate) fn scene_element_render_runtime(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> Vec<SceneElementRenderRuntime> {
    let Some(scene) = recipe.scenes.first() else {
        return vec![];
    };
    scene
        .elements
        .iter()
        .map(|element| {
            let visible = scene_element_visible(recipe, request, element);
            SceneElementRenderRuntime {
                scene_id: scene.id.as_str().to_string(),
                element_id: element.id.as_str().to_string(),
                layer_id: element.layer.as_ref().map(|id| id.as_str().to_string()),
                visible,
                skipped: !visible,
                skip_reason: (!visible).then(|| "visibilityFalse".to_string()),
            }
        })
        .collect()
}

fn scene_element_visible(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    element: &RecipeSceneElement,
) -> bool {
    if let Some(visibility) = &element.visibility {
        return match visibility {
            tui_vfx_contract::SceneElementVisibility::Always => true,
            tui_vfx_contract::SceneElementVisibility::Phase { phases } => {
                phases.iter().any(|phase| phase == &request.phase)
            }
            tui_vfx_contract::SceneElementVisibility::Predicate { source, predicate } => {
                resolve_value_source(source, &request.signals)
                    .as_ref()
                    .is_some_and(|value| value_matches_predicate(value, predicate))
            }
        };
    }
    visibility_parameter_candidates(element)
        .into_iter()
        .find_map(|id| resolve_visibility_parameter(recipe, request, &id))
        .unwrap_or(true)
}

fn visibility_parameter_candidates(element: &RecipeSceneElement) -> Vec<ParameterId> {
    let mut candidates = Vec::new();
    if let Some(layer) = &element.layer {
        candidates.push(ParameterId::new(format!(
            "layer_{}_visible",
            layer.as_str()
        )));
    }
    candidates.push(ParameterId::new(format!(
        "element_{}_visible",
        element.id.as_str()
    )));
    candidates
}

fn resolve_visibility_parameter(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
    parameter_id: &ParameterId,
) -> Option<bool> {
    let parameter = recipe.graph.parameters.get(parameter_id)?;
    recipe
        .graph
        .bindings
        .iter()
        .find_map(|binding| match &binding.target {
            BindingTarget::Parameter { id } if id == parameter_id => {
                resolve_value_source(&binding.source, &request.signals)
            }
            _ => None,
        })
        .or_else(|| parameter.value.default.clone())
        .and_then(boolean_value)
}

fn boolean_value(value: Value) -> Option<bool> {
    match value {
        Value::Boolean(value) => Some(value),
        _ => None,
    }
}

fn value_matches_predicate(value: &Value, predicate: &ValuePredicate) -> bool {
    match predicate {
        ValuePredicate::IsTrue => matches!(value, Value::Boolean(true)),
        ValuePredicate::IsFalse => matches!(value, Value::Boolean(false)),
        ValuePredicate::NonZero => numeric_value(value).is_some_and(|value| value != 0.0),
        ValuePredicate::NonEmpty => match value {
            Value::String(value) | Value::Text(value) => !value.is_empty(),
            _ => false,
        },
        ValuePredicate::Equals { value: expected } => value == expected,
        ValuePredicate::NotEquals { value: expected } => value != expected,
        ValuePredicate::GreaterThan { value: expected } => numeric_value(value)
            .zip(numeric_value(expected))
            .is_some_and(|(actual, expected)| actual > expected),
        ValuePredicate::LessThan { value: expected } => numeric_value(value)
            .zip(numeric_value(expected))
            .is_some_and(|(actual, expected)| actual < expected),
        ValuePredicate::Truthy => value_truthy(value),
    }
}

fn numeric_value(value: &Value) -> Option<f64> {
    match value {
        Value::Integer(value) => Some(*value as f64),
        Value::Number(value) | Value::Duration(value) => Some(*value),
        _ => None,
    }
}

fn value_truthy(value: &Value) -> bool {
    match value {
        Value::Boolean(value) => *value,
        Value::Integer(value) => *value != 0,
        Value::Number(value) | Value::Duration(value) => value.is_finite() && *value != 0.0,
        Value::String(value) | Value::Text(value) => !value.is_empty(),
        Value::Color(_) | Value::Gradient(_) => true,
        _ => false,
    }
}

fn layer_skipped_warning(runtime: &SceneElementRenderRuntime) -> PlayerWarning {
    PlayerWarning::new(
        "sceneLayerSkipped",
        format!("scenes[0].elements.{}", runtime.element_id),
        format!(
            "Scene element `{}` in layer `{}` was skipped because visibility resolved false",
            runtime.element_id,
            runtime.layer_id.as_deref().unwrap_or("<none>")
        ),
        Some("Set the layer visibility parameter or binding to true to render this element."),
    )
}

#[derive(Clone, Copy)]
struct PlacedElementBounds {
    x: i32,
    y: i32,
    width: i32,
    height: i32,
}

fn resolve_scene_element_placement(
    scene_width: usize,
    scene_height: usize,
    element: &RecipeSceneElement,
    rows: &[String],
    placed_layers: &std::collections::BTreeMap<tui_vfx_contract::LayerId, PlacedElementBounds>,
) -> (i32, i32) {
    match &element.placement_rule {
        Some(SceneElementPlacementRule::Absolute { rect, .. }) => (rect.x as i32, rect.y as i32),
        Some(SceneElementPlacementRule::Anchor {
            anchor,
            offset_rows,
            offset_columns,
            sibling_layer,
            ..
        }) => {
            let source_width = source_width_from_rows(rows);
            let source_height = rows.len() as i32;
            let sibling_bounds = sibling_layer
                .as_ref()
                .and_then(|layer| placed_layers.get(layer))
                .copied();
            if let Some(sibling) = sibling_bounds {
                return (
                    sibling.x + *offset_columns,
                    sibling.y + sibling.height + *offset_rows,
                );
            }
            let frame = PlacedElementBounds {
                x: 0,
                y: 0,
                width: scene_width as i32,
                height: scene_height as i32,
            };
            let (x, y) = anchor_position(*anchor, frame, source_width, source_height);
            (x + *offset_columns, y + *offset_rows)
        }
        None => (element.placement.x, element.placement.y),
    }
}

fn anchor_position(
    anchor: SceneAnchor,
    frame: PlacedElementBounds,
    source_width: i32,
    source_height: i32,
) -> (i32, i32) {
    let left = frame.x;
    let center_x = frame.x + (frame.width - source_width) / 2;
    let right = frame.x + frame.width - source_width;
    let top = frame.y;
    let visual_center_y = frame.y + frame.height * 45 / 100;
    let center_y = visual_center_y - source_height / 2;
    let bottom = frame.y + frame.height - source_height;
    match anchor {
        SceneAnchor::TopLeft => (left, top),
        SceneAnchor::TopCenter => (center_x, top),
        SceneAnchor::TopRight => (right, top),
        SceneAnchor::CenterLeft => (left, center_y),
        SceneAnchor::Center => (center_x, center_y),
        SceneAnchor::CenterRight => (right, center_y),
        SceneAnchor::BottomLeft => (left, bottom),
        SceneAnchor::BottomCenter => (center_x, bottom),
        SceneAnchor::BottomRight => (right, bottom),
    }
}

fn source_width_from_rows(rows: &[String]) -> i32 {
    rows.iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0) as i32
}

fn element_overflow_hides(
    element: &RecipeSceneElement,
    scene_width: usize,
    scene_height: usize,
    rows: &[String],
    dx: i32,
    dy: i32,
) -> bool {
    if element.overflow != Some(SceneElementOverflowPolicy::Hide) {
        return false;
    }
    rows.iter().enumerate().any(|(source_y, row)| {
        row.chars().enumerate().any(|(source_x, ch)| {
            let x = dx + source_x as i32;
            let y = dy + source_y as i32;
            ch != ' ' && (x < 0 || y < 0 || x as usize >= scene_width || y as usize >= scene_height)
        })
    })
}

fn scene_element_overflow_hidden_warning(element: &RecipeSceneElement) -> PlayerWarning {
    PlayerWarning::new(
        "sceneLayerOverflowHidden",
        format!("scenes[0].elements.{}", element.id.as_str()),
        format!(
            "Scene element `{}` was hidden because overflow policy is hide",
            element.id.as_str()
        ),
        Some("Use clip or wrap overflow when out-of-bounds content should remain visible."),
    )
}

fn render_source(
    recipe: &RecipeDocument,
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    asset_resolver: &dyn PlayerSourceAssetResolver,
) -> SourceRenderOutput {
    match source.source.as_str() {
        "source.card" => render_card_source(source_instance_id, source, request),
        "source.text" => source_rows(render_text_source(
            source_instance_id,
            source,
            request,
            "text",
        )),
        "source.ansi" => render_ansi_source(source_instance_id, source, request),
        "source.image" => render_image_source(source_instance_id, source, request, asset_resolver),
        "source.procedural" => {
            render_procedural_source(recipe, source_instance_id, source, request)
        }
        source_id => SourceRenderOutput {
            rows: vec![],
            styled_grid: PlayerStyledGrid::blank(0, 0, false),
            errors: vec![PlayerError::new(
                "unsupportedSourceAdapter",
                "sources.*.source",
                format!("No player adapter registered for {source_id}"),
                Some("Add a contract-native source adapter before expecting pixels."),
                serde_json::json!({ "source": source_id }),
            )],
            warnings: vec![],
        },
    }
}

fn render_card_source(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> SourceRenderOutput {
    if !card_chrome_inputs_present(source) {
        return source_rows(render_text_source(
            source_instance_id,
            source,
            request,
            "message",
        ));
    }
    let text = resolve_source_text(source_instance_id, source, request, "message", "");
    let width = source_width(source_instance_id, source, request, &text);
    let height = source_height(source_instance_id, source, request, &text);
    let border_style =
        resolve_source_enum(source_instance_id, source, request, "borderStyle", "none");
    let border = CardBorder::for_style(border_style.as_str());
    let foreground = resolve_source_color(
        source_instance_id,
        source,
        request,
        "foreground",
        ResolvedColor::rgb(255, 255, 255),
    )
    .rgba_label();
    let background = resolve_source_color(
        source_instance_id,
        source,
        request,
        "background",
        ResolvedColor::new(0, 0, 0, 0),
    )
    .rgba_label();
    if let Some(border) = border {
        render_bordered_card(&text, width, height, border, &foreground, &background)
    } else {
        render_plain_card(&text, width, height, &foreground, &background)
    }
}

fn card_chrome_inputs_present(source: &SourceSpec) -> bool {
    ["foreground", "background", "borderStyle", "borderTrim"]
        .into_iter()
        .any(|input| source.inputs.contains_key(&SourceInputId::new(input)))
}

#[derive(Clone, Copy)]
struct CardBorder {
    top_left: char,
    top_right: char,
    bottom_left: char,
    bottom_right: char,
    horizontal: char,
    vertical: char,
}

impl CardBorder {
    fn for_style(style: &str) -> Option<Self> {
        match style {
            "plain" => Some(Self {
                top_left: '┌',
                top_right: '┐',
                bottom_left: '└',
                bottom_right: '┘',
                horizontal: '─',
                vertical: '│',
            }),
            "rounded" => Some(Self {
                top_left: '╭',
                top_right: '╮',
                bottom_left: '╰',
                bottom_right: '╯',
                horizontal: '─',
                vertical: '│',
            }),
            "double" => Some(Self {
                top_left: '╔',
                top_right: '╗',
                bottom_left: '╚',
                bottom_right: '╝',
                horizontal: '═',
                vertical: '║',
            }),
            _ => None,
        }
    }
}

fn render_bordered_card(
    text: &str,
    width: usize,
    height: usize,
    border: CardBorder,
    foreground: &str,
    background: &str,
) -> SourceRenderOutput {
    let mut rows = vec![" ".repeat(width); height];
    let mut styled_grid = PlayerStyledGrid::blank(width, height, true);
    if width == 1 || height == 1 {
        return render_plain_card(text, width, height, foreground, background);
    }
    rows[0] = format!(
        "{}{}{}",
        border.top_left,
        std::iter::repeat_n(border.horizontal, width.saturating_sub(2)).collect::<String>(),
        border.top_right
    );
    rows[height - 1] = format!(
        "{}{}{}",
        border.bottom_left,
        std::iter::repeat_n(border.horizontal, width.saturating_sub(2)).collect::<String>(),
        border.bottom_right
    );
    for row in rows.iter_mut().take(height.saturating_sub(1)).skip(1) {
        *row = format!(
            "{}{}{}",
            border.vertical,
            " ".repeat(width.saturating_sub(2)),
            border.vertical
        );
    }
    for (index, line) in text.lines().take(height.saturating_sub(2)).enumerate() {
        let y = index + 1;
        replace_inner_text(&mut rows[y], line, 1, width.saturating_sub(2));
    }
    style_card_cells(&mut styled_grid, &rows, foreground, background, true);
    SourceRenderOutput {
        rows,
        styled_grid,
        errors: vec![],
        warnings: vec![],
    }
}

fn render_plain_card(
    text: &str,
    width: usize,
    height: usize,
    foreground: &str,
    background: &str,
) -> SourceRenderOutput {
    let rows = render_text_rows(text, width, height);
    let mut styled_grid = PlayerStyledGrid::blank(width, height, true);
    style_card_cells(&mut styled_grid, &rows, foreground, background, false);
    SourceRenderOutput {
        rows,
        styled_grid,
        errors: vec![],
        warnings: vec![],
    }
}

fn style_card_cells(
    styled_grid: &mut PlayerStyledGrid,
    rows: &[String],
    foreground: &str,
    background: &str,
    bordered: bool,
) {
    styled_grid.sync_glyphs_from_rows(rows);
    for y in 0..styled_grid.height() {
        for x in 0..styled_grid.width() {
            let role =
                if bordered && is_border_cell(x, y, styled_grid.width(), styled_grid.height()) {
                    "Border"
                } else if glyph_at(rows, x, y).is_some_and(|glyph| glyph != ' ') {
                    "Text"
                } else {
                    "Background"
                };
            styled_grid.set_cell_style(
                x,
                y,
                foreground,
                background,
                vec![],
                Some(role.to_string()),
            );
        }
    }
}

fn is_border_cell(x: usize, y: usize, width: usize, height: usize) -> bool {
    x == 0 || y == 0 || x + 1 == width || y + 1 == height
}

fn glyph_at(rows: &[String], x: usize, y: usize) -> Option<char> {
    rows.get(y)?.chars().nth(x)
}

fn replace_inner_text(row: &mut String, value: &str, start: usize, width: usize) {
    let mut chars = row.chars().collect::<Vec<_>>();
    for (offset, ch) in value.chars().take(width).enumerate() {
        if let Some(slot) = chars.get_mut(start + offset) {
            *slot = ch;
        }
    }
    *row = chars.into_iter().collect();
}

fn source_rows(rows: Vec<String>) -> SourceRenderOutput {
    SourceRenderOutput {
        styled_grid: PlayerStyledGrid::from_rows(&rows),
        rows,
        errors: vec![],
        warnings: vec![],
    }
}

fn apply_scene_element_surface(element: &RecipeSceneElement, styled_grid: &mut PlayerStyledGrid) {
    let Some(surface) = &element.surface else {
        return;
    };
    let Some(base_style) = &surface.base_style else {
        return;
    };
    let base_style = structured_to_json(base_style);
    let foreground = color_label_from_json(base_style.get("foreground"));
    let background = color_label_from_json(base_style.get("background"));
    let added_modifiers = string_array_from_json(
        base_style
            .get("addedModifiers")
            .or_else(|| base_style.get("added_modifiers")),
    );
    let removed_modifiers = string_array_from_json(
        base_style
            .get("removedModifiers")
            .or_else(|| base_style.get("removed_modifiers")),
    );
    if foreground.is_none() && background.is_none() && added_modifiers.is_empty() {
        return;
    }
    for cell in styled_grid.cells().to_vec() {
        if cell.glyph == " " {
            continue;
        }
        let mut modifiers = cell.modifiers.clone();
        modifiers.retain(|modifier| !removed_modifiers.iter().any(|removed| removed == modifier));
        for modifier in &added_modifiers {
            if !modifiers.iter().any(|known| known == modifier) {
                modifiers.push(modifier.clone());
            }
        }
        let foreground = foreground
            .as_deref()
            .unwrap_or(cell.foreground.as_str())
            .to_string();
        let background = background
            .as_deref()
            .unwrap_or(cell.background.as_str())
            .to_string();
        styled_grid.set_cell_style(
            cell.x,
            cell.y,
            &foreground,
            &background,
            modifiers,
            cell.role.clone(),
        );
    }
}

fn color_label_from_json(value: Option<&serde_json::Value>) -> Option<String> {
    let value = value?;
    Some(
        ResolvedColor::new(
            json_u8(value, "r")?,
            json_u8(value, "g")?,
            json_u8(value, "b")?,
            json_u8(value, "a").unwrap_or(255),
        )
        .rgba_label(),
    )
}

fn json_u8(value: &serde_json::Value, key: &str) -> Option<u8> {
    let value = value.get(key)?;
    value
        .as_u64()
        .and_then(|value| u8::try_from(value).ok())
        .or_else(|| {
            value
                .as_f64()
                .filter(|value| (0.0..=255.0).contains(value))
                .map(|value| value.round() as u8)
        })
}

fn string_array_from_json(value: Option<&serde_json::Value>) -> Vec<String> {
    value
        .and_then(serde_json::Value::as_array)
        .map(|values| {
            values
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(ToOwned::to_owned)
                .collect()
        })
        .unwrap_or_default()
}

fn render_text_source(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text_input_id: &str,
) -> Vec<String> {
    let text = resolve_source_text(source_instance_id, source, request, text_input_id, "");
    let width = resolve_source_integer(
        source_instance_id,
        source,
        request,
        "width",
        fallback_width(&text),
    )
    .max(1) as usize;
    let height = resolve_source_integer(
        source_instance_id,
        source,
        request,
        "height",
        fallback_height(&text),
    )
    .max(1) as usize;
    let mut rows = vec![" ".repeat(width); height];
    for (index, line) in text.lines().take(height).enumerate() {
        rows[index] = clip_or_pad(line, width);
    }
    rows
}

fn render_ansi_source(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> SourceRenderOutput {
    let ansi_text = resolve_source_text(source_instance_id, source, request, "ansiText", "");
    let width = source_width(
        source_instance_id,
        source,
        request,
        &strip_sgr_sequences(&ansi_text),
    );
    let height = source_height(
        source_instance_id,
        source,
        request,
        &strip_sgr_sequences(&ansi_text),
    );
    render_ansi_styled_text(&ansi_text, width, height)
}

fn render_image_source(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    asset_resolver: &dyn PlayerSourceAssetResolver,
) -> SourceRenderOutput {
    let asset_id = resolve_image_source_asset_id(source, request);
    match asset_resolver.resolve_image_asset(PlayerSourceAssetRequest {
        asset_id: &asset_id,
    }) {
        PlayerSourceAssetResolution::ResolvedGrid { rows, styled_grid } => SourceRenderOutput {
            rows,
            styled_grid,
            errors: vec![],
            warnings: vec![],
        },
        PlayerSourceAssetResolution::MissingFallback { asset_id } => {
            let fallback = format!("[image fallback: {asset_id}]");
            let rows = render_text_like_source(source_instance_id, source, request, &fallback);
            SourceRenderOutput {
                styled_grid: PlayerStyledGrid::from_rows(&rows),
                rows,
                errors: vec![],
                warnings: vec![PlayerWarning::new(
                    "imageFallbackRendered",
                    "sources.*.inputs.asset",
                    format!("Image source rendered deterministic fallback for asset `{asset_id}`"),
                    Some(
                        "Provide an image resolver/backend adapter before treating source.image as visual parity.",
                    ),
                )],
            }
        }
        PlayerSourceAssetResolution::Unsupported { asset_id, reason } => SourceRenderOutput {
            rows: vec![],
            styled_grid: PlayerStyledGrid::blank(0, 0, false),
            errors: vec![PlayerError::new(
                "unsupportedSourceImageAsset",
                "sources.*.inputs.asset",
                format!("Image asset `{asset_id}` is unsupported by the player resolver: {reason}"),
                Some("Use a resolver-supported source asset or expect fallback-only rendering."),
                serde_json::json!({ "asset": asset_id, "reason": reason }),
            )],
            warnings: vec![],
        },
    }
}

fn render_procedural_source(
    recipe: &RecipeDocument,
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> SourceRenderOutput {
    let generator = resolve_source_text(
        source_instance_id,
        source,
        request,
        "generator",
        "dots_spinner",
    );
    let seed =
        resolve_source_integer(source_instance_id, source, request, "seed", 0).max(0) as usize;
    let width = source_width(source_instance_id, source, request, &generator);
    let height = source_height(source_instance_id, source, request, &generator);
    let params = resolve_source_structured(source_instance_id, source, request, "params")
        .map(|value| resolve_authored_param_bindings(value, request, &recipe.assets))
        .unwrap_or(serde_json::Value::Null);
    match render_registered_procedural_source(&generator, width, height, seed, request, &params) {
        Some(rendered) => SourceRenderOutput {
            rows: rendered.rows,
            styled_grid: rendered.styled_grid,
            errors: vec![],
            warnings: vec![],
        },
        None => SourceRenderOutput {
            rows: vec![],
            styled_grid: PlayerStyledGrid::blank(0, 0, false),
            errors: vec![PlayerError::new(
                "unsupportedProceduralGenerator",
                "sources.*.inputs.generator",
                format!("No player procedural source generator registered for `{generator}`"),
                Some("Use a registered bounded procedural source generator."),
                serde_json::json!({ "generator": generator }),
            )],
            warnings: vec![],
        },
    }
}

fn resolve_source_structured(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
) -> Option<serde_json::Value> {
    match source_runtime_override(source_instance_id, source, request, input_id) {
        Some(Value::Structured(value)) => Some(structured_to_json(value)),
        _ => source
            .inputs
            .get(&SourceInputId::new(input_id))
            .and_then(|source| resolve_value_source(source, &request.signals))
            .and_then(|value| match value {
                Value::Structured(value) => Some(structured_to_json(&value)),
                _ => None,
            }),
    }
}

fn resolve_authored_param_bindings(
    value: serde_json::Value,
    request: &PlayerSampleRequest,
    assets: &std::collections::BTreeMap<AssetId, AssetSpec>,
) -> serde_json::Value {
    match value {
        serde_json::Value::Array(values) => serde_json::Value::Array(
            values
                .into_iter()
                .map(|value| resolve_authored_param_bindings(value, request, assets))
                .collect(),
        ),
        serde_json::Value::Object(mut object) => {
            if let Some(binding) = object.get("binding").and_then(serde_json::Value::as_str) {
                let resolved = request
                    .signals
                    .get(&tui_vfx_contract::SignalId::new(binding))
                    .map(contract_value_to_json)
                    .unwrap_or_else(|| object.remove("default").unwrap_or(serde_json::Value::Null));
                if let Some(gate) = object.get("gate").and_then(serde_json::Value::as_object)
                    && let Some(bool_value) = resolved.as_bool()
                {
                    return gate
                        .get(if bool_value { "true" } else { "false" })
                        .cloned()
                        .unwrap_or(resolved);
                }
                resolved
            } else {
                if let Some(resolved) = resolve_asset_param_object(&object, assets) {
                    return resolved;
                }
                serde_json::Value::Object(
                    object
                        .into_iter()
                        .map(|(key, value)| {
                            (key, resolve_authored_param_bindings(value, request, assets))
                        })
                        .collect(),
                )
            }
        }
        value => value,
    }
}

fn resolve_asset_param_object(
    object: &serde_json::Map<String, serde_json::Value>,
    assets: &std::collections::BTreeMap<AssetId, AssetSpec>,
) -> Option<serde_json::Value> {
    let id = object.get("id").and_then(serde_json::Value::as_str)?;
    let asset = assets.get(&AssetId::new(id))?;
    let AssetLocator::Path { path } = &asset.locator else {
        return None;
    };
    let mut resolved = object.clone();
    resolved.insert("path".to_string(), serde_json::Value::String(path.clone()));
    Some(serde_json::Value::Object(resolved))
}

fn contract_value_to_json(value: &Value) -> serde_json::Value {
    match value {
        Value::Null => serde_json::Value::Null,
        Value::Boolean(value) => serde_json::Value::Bool(*value),
        Value::Integer(value) => serde_json::json!(value),
        Value::Number(value) | Value::Duration(value) => serde_json::json!(value),
        Value::String(value) | Value::Text(value) | Value::Enum(value) => {
            serde_json::Value::String(value.clone())
        }
        Value::Color(value) => serde_json::json!({
            "type": "rgb",
            "r": value.r,
            "g": value.g,
            "b": value.b
        }),
        Value::Structured(value) => structured_to_json(value),
        _ => serde_json::Value::Null,
    }
}

fn structured_to_json(value: &tui_vfx_contract::StructuredValue) -> serde_json::Value {
    match value {
        tui_vfx_contract::StructuredValue::Null => serde_json::Value::Null,
        tui_vfx_contract::StructuredValue::Boolean(value) => serde_json::Value::Bool(*value),
        tui_vfx_contract::StructuredValue::Number(value) => serde_json::json!(value),
        tui_vfx_contract::StructuredValue::String(value) => {
            serde_json::Value::String(value.clone())
        }
        tui_vfx_contract::StructuredValue::Array(values) => {
            serde_json::Value::Array(values.iter().map(structured_to_json).collect())
        }
        tui_vfx_contract::StructuredValue::Object(values) => serde_json::Value::Object(
            values
                .iter()
                .map(|(key, value)| (key.clone(), structured_to_json(value)))
                .collect(),
        ),
    }
}

fn render_text_like_source(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text: &str,
) -> Vec<String> {
    render_text_rows(
        text,
        source_width(source_instance_id, source, request, text),
        source_height(source_instance_id, source, request, text),
    )
}

fn source_width(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text: &str,
) -> usize {
    resolve_source_integer(
        source_instance_id,
        source,
        request,
        "width",
        fallback_width(text),
    )
    .max(1) as usize
}

fn source_height(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text: &str,
) -> usize {
    resolve_source_integer(
        source_instance_id,
        source,
        request,
        "height",
        fallback_height(text),
    )
    .max(1) as usize
}

fn resolve_source_text(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: &str,
) -> String {
    match source_runtime_override(source_instance_id, source, request, input_id) {
        Some(value) => value_to_text(value, fallback),
        None => resolve_text(
            source.inputs.get(&SourceInputId::new(input_id)),
            &request.signals,
            fallback,
        ),
    }
}

fn resolve_source_integer(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: i64,
) -> i64 {
    match source_runtime_override(source_instance_id, source, request, input_id) {
        Some(Value::Integer(value)) => *value,
        Some(Value::Number(value)) => value.round() as i64,
        Some(Value::String(value) | Value::Text(value) | Value::Enum(value)) => {
            value.parse::<i64>().unwrap_or(fallback)
        }
        _ => resolve_integer(
            source.inputs.get(&SourceInputId::new(input_id)),
            &request.signals,
            fallback,
        ),
    }
}

fn resolve_source_enum(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: &str,
) -> String {
    match source_runtime_override(source_instance_id, source, request, input_id) {
        Some(Value::Enum(value) | Value::String(value) | Value::Text(value)) => value.clone(),
        _ => resolve_source_text(source_instance_id, source, request, input_id, fallback),
    }
}

fn resolve_source_color(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    input_id: &str,
    fallback: ResolvedColor,
) -> ResolvedColor {
    match source_runtime_override(source_instance_id, source, request, input_id) {
        Some(Value::Color(value)) => ResolvedColor::new(value.r, value.g, value.b, value.a),
        _ => source
            .inputs
            .get(&SourceInputId::new(input_id))
            .and_then(|source| resolve_value_source(source, &request.signals))
            .and_then(|value| match value {
                Value::Color(value) => Some(ResolvedColor::new(value.r, value.g, value.b, value.a)),
                _ => None,
            })
            .unwrap_or(fallback),
    }
}

fn source_runtime_override<'a>(
    source_instance_id: &str,
    source: &SourceSpec,
    request: &'a PlayerSampleRequest,
    input_id: &str,
) -> Option<&'a Value> {
    let candidates = [
        format!(
            "source:{}:{}:{}",
            source.source.as_str(),
            source_instance_id,
            input_id
        ),
        format!("{}.{}", source_instance_id, input_id),
        format!("{}.{}", source.source.as_str(), input_id),
        format!("source:{}:{}", source.source.as_str(), input_id),
        input_id.to_string(),
    ];
    candidates
        .iter()
        .find_map(|candidate| request.runtime_input_overrides.get(candidate))
        .or_else(|| {
            let normalized_candidates = candidates
                .iter()
                .map(|candidate| normalize_runtime_key(candidate))
                .collect::<Vec<_>>();
            request
                .runtime_input_overrides
                .iter()
                .find(|(override_key, _)| {
                    let normalized_override = normalize_runtime_key(override_key);
                    normalized_candidates
                        .iter()
                        .any(|candidate| candidate == &normalized_override)
                })
                .map(|(_, value)| value)
        })
}

fn normalize_runtime_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn value_to_text(value: &Value, fallback: &str) -> String {
    match value {
        Value::Text(value) | Value::String(value) | Value::Enum(value) => value.clone(),
        Value::Integer(value) => value.to_string(),
        Value::Number(value) => value.to_string(),
        Value::Boolean(value) => value.to_string(),
        _ => fallback.to_string(),
    }
}

fn render_text_rows(text: &str, width: usize, height: usize) -> Vec<String> {
    let mut rows = vec![" ".repeat(width); height];
    for (index, line) in text.lines().take(height).enumerate() {
        rows[index] = clip_or_pad(line, width);
    }
    rows
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct AnsiStyle {
    foreground: String,
    background: String,
    modifiers: Vec<String>,
}

impl Default for AnsiStyle {
    fn default() -> Self {
        Self {
            foreground: "defaultForeground".to_string(),
            background: "transparent".to_string(),
            modifiers: vec![],
        }
    }
}

impl AnsiStyle {
    fn is_default(&self) -> bool {
        self.foreground == "defaultForeground"
            && self.background == "transparent"
            && self.modifiers.is_empty()
    }
}

#[derive(Clone, Debug)]
struct AnsiCell {
    glyph: char,
    style: AnsiStyle,
}

fn render_ansi_styled_text(value: &str, width: usize, height: usize) -> SourceRenderOutput {
    let cells = parse_ansi_cells(value);
    let mut rows = vec![" ".repeat(width); height];
    let mut styled_grid = PlayerStyledGrid::blank(width, height, false);
    let mut x = 0usize;
    let mut y = 0usize;
    for cell in cells {
        if cell.glyph == '\n' {
            x = 0;
            y += 1;
            if y >= height {
                break;
            }
            continue;
        }
        if x >= width {
            continue;
        }
        replace_char(&mut rows[y], x, cell.glyph);
        if !cell.style.is_default() {
            styled_grid.set_cell_style(
                x,
                y,
                &cell.style.foreground,
                &cell.style.background,
                cell.style.modifiers,
                Some("Ansi".to_string()),
            );
        }
        x += 1;
    }
    styled_grid.sync_glyphs_from_rows(&rows);
    SourceRenderOutput {
        rows,
        styled_grid,
        errors: vec![],
        warnings: vec![],
    }
}

fn parse_ansi_cells(value: &str) -> Vec<AnsiCell> {
    let mut cells = Vec::new();
    let mut style = AnsiStyle::default();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            let mut sequence = String::new();
            for next in chars.by_ref() {
                if next == 'm' {
                    apply_sgr_sequence(&mut style, &sequence);
                    break;
                }
                if next.is_ascii_alphabetic() {
                    break;
                }
                sequence.push(next);
            }
        } else {
            cells.push(AnsiCell {
                glyph: ch,
                style: style.clone(),
            });
        }
    }
    cells
}

fn apply_sgr_sequence(style: &mut AnsiStyle, sequence: &str) {
    let codes = if sequence.is_empty() {
        vec![0]
    } else {
        sequence
            .split(';')
            .filter_map(|part| part.parse::<u16>().ok())
            .collect::<Vec<_>>()
    };
    for code in codes {
        match code {
            0 => *style = AnsiStyle::default(),
            1 => add_modifier(style, "bold"),
            3 => add_modifier(style, "italic"),
            22 => style.modifiers.retain(|modifier| modifier != "bold"),
            23 => style.modifiers.retain(|modifier| modifier != "italic"),
            30..=37 | 90..=97 => style.foreground = ansi_color_name(code, false),
            39 => style.foreground = "defaultForeground".to_string(),
            40..=47 | 100..=107 => style.background = ansi_color_name(code, true),
            49 => style.background = "transparent".to_string(),
            _ => {}
        }
    }
}

fn add_modifier(style: &mut AnsiStyle, modifier: &str) {
    if !style.modifiers.iter().any(|existing| existing == modifier) {
        style.modifiers.push(modifier.to_string());
    }
}

fn ansi_color_name(code: u16, background: bool) -> String {
    let base = if background { 40 } else { 30 };
    let bright_base = if background { 100 } else { 90 };
    let offset = if code >= bright_base {
        code - bright_base
    } else {
        code - base
    };
    let name = match offset {
        0 => "black",
        1 => "red",
        2 => "green",
        3 => "yellow",
        4 => "blue",
        5 => "magenta",
        6 => "cyan",
        7 => "white",
        _ => "default",
    };
    if code >= bright_base {
        format!("ansi.bright.{name}")
    } else {
        format!("ansi.{name}")
    }
}

fn replace_char(row: &mut String, x: usize, glyph: char) {
    let mut chars = row.chars().collect::<Vec<_>>();
    if let Some(slot) = chars.get_mut(x) {
        *slot = glyph;
    }
    *row = chars.into_iter().collect();
}

fn strip_sgr_sequences(value: &str) -> String {
    let mut output = String::new();
    let mut chars = value.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '\u{1b}' && chars.peek() == Some(&'[') {
            chars.next();
            for next in chars.by_ref() {
                if next.is_ascii_alphabetic() {
                    break;
                }
            }
        } else {
            output.push(ch);
        }
    }
    output
}

fn fallback_width(text: &str) -> i64 {
    text.lines()
        .map(|line| line.chars().count())
        .max()
        .unwrap_or(35)
        .max(1) as i64
}

fn fallback_height(text: &str) -> i64 {
    text.lines().count().max(1) as i64
}

fn blank_grid(width: usize, height: usize) -> Vec<Vec<char>> {
    vec![vec![' '; width]; height]
}

fn blit_rows(
    grid: &mut [Vec<char>],
    rows: &[String],
    dx: i32,
    dy: i32,
    cell_write_policy: CellWritePolicy,
    overflow: Option<SceneElementOverflowPolicy>,
) {
    for (source_y, row) in rows.iter().enumerate() {
        let y = dy + source_y as i32;
        let Some(y) = overflow_coordinate(y, grid.len(), overflow) else {
            continue;
        };
        blit_row(&mut grid[y], row, dx, cell_write_policy, overflow);
    }
}

fn blit_row(
    destination: &mut [char],
    row: &str,
    dx: i32,
    cell_write_policy: CellWritePolicy,
    overflow: Option<SceneElementOverflowPolicy>,
) {
    for (source_x, ch) in row.chars().enumerate() {
        let x = dx + source_x as i32;
        let Some(x) = overflow_coordinate(x, destination.len(), overflow) else {
            continue;
        };
        if ch != ' ' || cell_write_policy == CellWritePolicy::WriteCell {
            destination[x] = ch;
        }
    }
}

fn blit_styles(
    destination: &mut PlayerStyledGrid,
    source: &PlayerStyledGrid,
    x_offset: i32,
    y_offset: i32,
    cell_write_policy: CellWritePolicy,
    overflow: Option<SceneElementOverflowPolicy>,
) {
    if !source.style_known() {
        return;
    }
    for cell in source.cells() {
        if cell_write_policy == CellWritePolicy::SkipTransparentEmpty && cell.glyph == " " {
            continue;
        }
        let x = x_offset + cell.x as i32;
        let y = y_offset + cell.y as i32;
        let Some(x) = overflow_coordinate(x, destination.width(), overflow) else {
            continue;
        };
        let Some(y) = overflow_coordinate(y, destination.height(), overflow) else {
            continue;
        };
        destination.set_cell_style(
            x,
            y,
            &cell.foreground,
            &cell.background,
            cell.modifiers.clone(),
            cell.role.clone(),
        );
    }
}

fn overflow_coordinate(
    value: i32,
    extent: usize,
    overflow: Option<SceneElementOverflowPolicy>,
) -> Option<usize> {
    if extent == 0 {
        return None;
    }
    match overflow {
        Some(SceneElementOverflowPolicy::Wrap) => Some(value.rem_euclid(extent as i32) as usize),
        _ if value >= 0 && (value as usize) < extent => Some(value as usize),
        _ => None,
    }
}

fn grid_to_rows(grid: &[Vec<char>]) -> Vec<String> {
    grid.iter().map(|row| row.iter().collect()).collect()
}

fn clip_or_pad(value: &str, width: usize) -> String {
    let mut clipped = value.chars().take(width).collect::<String>();
    let clipped_width = clipped.chars().count();
    clipped.extend(std::iter::repeat_n(
        ' ',
        width.saturating_sub(clipped_width),
    ));
    clipped
}

fn missing_scene_error() -> PlayerError {
    PlayerError::new(
        "missingScene",
        "scenes",
        "Recipe has no scenes to render",
        Some("Add at least one canonical recipe scene."),
        serde_json::Value::Null,
    )
}

// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into player-owned rows and styled cells</DESC>
// <VERS>END OF VERSION: 0.4.1</VERS>
