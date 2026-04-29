// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into player-owned rows and styled cells</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>Scene rendering carries element-local pipeline style evidence into the player surface.</WCTX>
// <CLOG>0.3.0: MINOR — preserve local pipeline styled-cell evidence when placing scene elements.
// 0.2.0: PATCH — distinguish source.card message input from source.text text input.
// 0.1.0: INIT — add scene traversal, source rendering, and grid blitting helpers.</CLOG>

use tui_vfx_contract::{RecipeDocument, SourceInputId, SourceSpec};

use crate::{
    PlayerError, PlayerSampleRequest, PlayerStyledGrid, PlayerWarning,
    fnc_apply_graph_effects::apply_graph_step_effects, fnc_resolve_value_source::resolve_integer,
    fnc_resolve_value_source::resolve_text,
};

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
    let mut elements = scene.elements.iter().collect::<Vec<_>>();
    elements.sort_by_key(|element| element.z_index);
    for element in elements {
        match recipe.sources.get(&element.source) {
            Some(source) => {
                let (mut source_rows, mut source_errors, mut source_warnings) =
                    render_source(source, request);
                warnings.append(&mut source_warnings);
                let mut local_grid = PlayerStyledGrid::from_rows(&source_rows);
                if let Some(pipeline) = &element.pipeline
                    && let Some(topology) = &pipeline.topology
                {
                    let mut local_request = request.clone();
                    apply_graph_step_effects(
                        recipe,
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
                );
                blit_styles(
                    &mut styled_grid,
                    &local_grid,
                    element.placement.x,
                    element.placement.y,
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

fn render_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> (Vec<String>, Vec<PlayerError>, Vec<PlayerWarning>) {
    match source.source.as_str() {
        "source.card" => (
            render_text_source(source, request, "message"),
            vec![],
            vec![],
        ),
        "source.text" => (render_text_source(source, request, "text"), vec![], vec![]),
        "source.ansi" => (render_ansi_source(source, request), vec![], vec![]),
        "source.image" => render_image_source(source, request),
        "source.procedural" => (render_procedural_source(source, request), vec![], vec![]),
        source_id => (
            vec![],
            vec![PlayerError::new(
                "unsupportedSourceAdapter",
                "sources.*.source",
                format!("No player adapter registered for {source_id}"),
                Some("Add a contract-native source adapter before expecting pixels."),
                serde_json::json!({ "source": source_id }),
            )],
            vec![],
        ),
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

fn render_ansi_source(source: &SourceSpec, request: &PlayerSampleRequest) -> Vec<String> {
    let ansi_text = resolve_text(
        source.inputs.get(&SourceInputId::new("ansiText")),
        &request.signals,
        "",
    );
    render_text_like_source(source, request, &strip_sgr_sequences(&ansi_text))
}

fn render_image_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> (Vec<String>, Vec<PlayerError>, Vec<PlayerWarning>) {
    let asset = resolve_text(
        source.inputs.get(&SourceInputId::new("asset")),
        &request.signals,
        "missing",
    );
    let fallback = format!("[image fallback: {asset}]");
    let rows = render_text_like_source(source, request, &fallback);
    let warning = PlayerWarning::new(
        "imageFallbackRendered",
        "sources.*.inputs.asset",
        format!("Image source rendered deterministic fallback for asset `{asset}`"),
        Some(
            "Provide an image resolver/backend adapter before treating source.image as visual parity.",
        ),
    );
    (rows, vec![], vec![warning])
}

fn render_procedural_source(source: &SourceSpec, request: &PlayerSampleRequest) -> Vec<String> {
    let generator = resolve_text(
        source.inputs.get(&SourceInputId::new("generator")),
        &request.signals,
        "dots_spinner",
    );
    let glyphs = ["⠁", "⠂", "⠄", "⡀", "⢀", "⠠", "⠐", "⠈"];
    let seed = resolve_integer(
        source.inputs.get(&SourceInputId::new("seed")),
        &request.signals,
        0,
    )
    .max(0) as usize;
    let frame = (((request.loop_t.unwrap_or(request.phase_t).clamp(0.0, 1.0) * glyphs.len() as f64)
        .floor() as usize)
        + seed)
        % glyphs.len();
    let text = if generator == "dots_spinner" {
        format!("{} dots spinner", glyphs[frame])
    } else {
        format!("procedural fallback: {generator}")
    };
    render_text_like_source(source, request, &text)
}

fn render_text_like_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
    text: &str,
) -> Vec<String> {
    let width = resolve_integer(
        source.inputs.get(&SourceInputId::new("width")),
        &request.signals,
        fallback_width(text),
    )
    .max(1) as usize;
    let height = resolve_integer(
        source.inputs.get(&SourceInputId::new("height")),
        &request.signals,
        fallback_height(text),
    )
    .max(1) as usize;
    let mut rows = vec![" ".repeat(width); height];
    for (index, line) in text.lines().take(height).enumerate() {
        rows[index] = clip_or_pad(line, width);
    }
    rows
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

fn blit_rows(grid: &mut [Vec<char>], rows: &[String], dx: i32, dy: i32) {
    for (source_y, row) in rows.iter().enumerate() {
        let y = dy + source_y as i32;
        if y < 0 || y as usize >= grid.len() {
            continue;
        }
        blit_row(&mut grid[y as usize], row, dx);
    }
}

fn blit_row(destination: &mut [char], row: &str, dx: i32) {
    for (source_x, ch) in row.chars().enumerate() {
        let x = dx + source_x as i32;
        if x >= 0 && (x as usize) < destination.len() && ch != ' ' {
            destination[x as usize] = ch;
        }
    }
}

fn blit_styles(
    destination: &mut PlayerStyledGrid,
    source: &PlayerStyledGrid,
    x_offset: i32,
    y_offset: i32,
) {
    if !source.style_known() {
        return;
    }
    for cell in source.cells() {
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
// <VERS>END OF VERSION: 0.3.0</VERS>
