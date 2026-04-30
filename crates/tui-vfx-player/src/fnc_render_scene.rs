// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into player-owned rows and styled cells</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Scene rendering carries element-local pipeline style evidence into the player surface.</WCTX>
// <CLOG>0.4.0: MINOR — carry bounded ANSI SGR styled-cell evidence and source resolver seams.
// 0.3.0: MINOR — preserve local pipeline styled-cell evidence when placing scene elements.
// 0.2.0: PATCH — distinguish source.card message input from source.text text input.
// 0.1.0: INIT — add scene traversal, source rendering, and grid blitting helpers.</CLOG>

use tui_vfx_contract::{
    BindingTarget, CellWritePolicy, ParameterId, RecipeDocument, RecipeSceneElement, SourceInputId,
    SourceSpec, Value,
};

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid, PlayerWarning,
    fnc_apply_graph_effects::apply_graph_step_effects,
    fnc_render_procedural_source::render_registered_procedural_source,
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
                let mut output = render_source(source, request, asset_resolver);
                let mut source_rows = output.rows;
                let mut local_grid = output.styled_grid;
                let mut source_errors = output.errors;
                warnings.append(&mut output.warnings);
                if let Some(pipeline) = &element.pipeline
                    && let Some(topology) = &pipeline.topology
                {
                    let mut local_request = request.clone();
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
                blit_rows(
                    &mut grid,
                    &source_rows,
                    element.placement.x,
                    element.placement.y,
                    element.cell_write_policy,
                );
                blit_styles(
                    &mut styled_grid,
                    &local_grid,
                    element.placement.x,
                    element.placement.y,
                    element.cell_write_policy,
                );
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

fn render_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    asset_resolver: &dyn PlayerSourceAssetResolver,
) -> SourceRenderOutput {
    match source.source.as_str() {
        "source.card" => source_rows(render_text_source(source, request, "message")),
        "source.text" => source_rows(render_text_source(source, request, "text")),
        "source.ansi" => render_ansi_source(source, request),
        "source.image" => render_image_source(source, request, asset_resolver),
        "source.procedural" => render_procedural_source(source, request),
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

fn source_rows(rows: Vec<String>) -> SourceRenderOutput {
    SourceRenderOutput {
        styled_grid: PlayerStyledGrid::from_rows(&rows),
        rows,
        errors: vec![],
        warnings: vec![],
    }
}

fn render_text_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text_input_id: &str,
) -> Vec<String> {
    let text = resolve_text(
        source.inputs.get(&SourceInputId::new(text_input_id)),
        &request.signals,
        "",
    );
    let width = resolve_integer(
        source.inputs.get(&SourceInputId::new("width")),
        &request.signals,
        fallback_width(&text),
    )
    .max(1) as usize;
    let height = resolve_integer(
        source.inputs.get(&SourceInputId::new("height")),
        &request.signals,
        fallback_height(&text),
    )
    .max(1) as usize;
    let mut rows = vec![" ".repeat(width); height];
    for (index, line) in text.lines().take(height).enumerate() {
        rows[index] = clip_or_pad(line, width);
    }
    rows
}

fn render_ansi_source(source: &SourceSpec, request: &PlayerSampleRequest) -> SourceRenderOutput {
    let ansi_text = resolve_text(
        source.inputs.get(&SourceInputId::new("ansiText")),
        &request.signals,
        "",
    );
    let width = source_width(source, request, &strip_sgr_sequences(&ansi_text));
    let height = source_height(source, request, &strip_sgr_sequences(&ansi_text));
    render_ansi_styled_text(&ansi_text, width, height)
}

fn render_image_source(
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
            let rows = render_text_like_source(source, request, &fallback);
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
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> SourceRenderOutput {
    let generator = resolve_text(
        source.inputs.get(&SourceInputId::new("generator")),
        &request.signals,
        "dots_spinner",
    );
    let seed = resolve_integer(
        source.inputs.get(&SourceInputId::new("seed")),
        &request.signals,
        0,
    )
    .max(0) as usize;
    let width = source_width(source, request, &generator);
    let height = source_height(source, request, &generator);
    match render_registered_procedural_source(&generator, width, height, seed, request) {
        Some(rows) => source_rows(rows),
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

fn render_text_like_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text: &str,
) -> Vec<String> {
    render_text_rows(
        text,
        source_width(source, request, text),
        source_height(source, request, text),
    )
}

fn source_width(source: &SourceSpec, request: &PlayerSampleRequest, text: &str) -> usize {
    resolve_integer(
        source.inputs.get(&SourceInputId::new("width")),
        &request.signals,
        fallback_width(text),
    )
    .max(1) as usize
}

fn source_height(source: &SourceSpec, request: &PlayerSampleRequest, text: &str) -> usize {
    resolve_integer(
        source.inputs.get(&SourceInputId::new("height")),
        &request.signals,
        fallback_height(text),
    )
    .max(1) as usize
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
) {
    for (source_y, row) in rows.iter().enumerate() {
        let y = dy + source_y as i32;
        if y < 0 || y as usize >= grid.len() {
            continue;
        }
        blit_row(&mut grid[y as usize], row, dx, cell_write_policy);
    }
}

fn blit_row(destination: &mut [char], row: &str, dx: i32, cell_write_policy: CellWritePolicy) {
    for (source_x, ch) in row.chars().enumerate() {
        let x = dx + source_x as i32;
        if x >= 0
            && (x as usize) < destination.len()
            && (ch != ' ' || cell_write_policy == CellWritePolicy::WriteCell)
        {
            destination[x as usize] = ch;
        }
    }
}

fn blit_styles(
    destination: &mut PlayerStyledGrid,
    source: &PlayerStyledGrid,
    x_offset: i32,
    y_offset: i32,
    cell_write_policy: CellWritePolicy,
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
        if x >= 0 && y >= 0 && destination.contains(x as usize, y as usize) {
            destination.set_cell_style(
                x as usize,
                y as usize,
                &cell.foreground,
                &cell.background,
                cell.modifiers.clone(),
                cell.role.clone(),
            );
        }
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
// <VERS>END OF VERSION: 0.4.0</VERS>
