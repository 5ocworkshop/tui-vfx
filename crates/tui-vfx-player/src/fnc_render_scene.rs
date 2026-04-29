// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into K0 text-grid rows</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>New kernel Phase K0: render source.text using its canonical text input.</WCTX>
// <CLOG>0.2.0: PATCH — distinguish source.card message input from source.text text input.
// 0.1.0: INIT — add scene traversal, source rendering, and grid blitting helpers.</CLOG>

use tui_vfx_contract::{RecipeDocument, SourceInputId, SourceSpec};

use crate::{
    PlayerError, PlayerSampleRequest, fnc_resolve_value_source::resolve_integer,
    fnc_resolve_value_source::resolve_text,
};

/// Render the first recipe scene into K0 text-grid rows.
pub fn render_scene(
    recipe: &RecipeDocument,
    request: &PlayerSampleRequest,
) -> (Vec<String>, Vec<PlayerError>) {
    let Some(scene) = recipe.scenes.first() else {
        return (vec![], vec![missing_scene_error()]);
    };
    let width = request.width.unwrap_or(scene.width);
    let height = request.height.unwrap_or(scene.height);
    let mut grid = blank_grid(width, height);
    let mut errors = Vec::new();
    let mut elements = scene.elements.iter().collect::<Vec<_>>();
    elements.sort_by_key(|element| element.z_index);
    for element in elements {
        match recipe.sources.get(&element.source) {
            Some(source) => {
                let (source_rows, mut source_errors) = render_source(source, request);
                blit_rows(
                    &mut grid,
                    &source_rows,
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
    (grid_to_rows(&grid), errors)
}

fn render_source(
    source: &SourceSpec,
    request: &PlayerSampleRequest,
) -> (Vec<String>, Vec<PlayerError>) {
    match source.source.as_str() {
        "source.card" => (render_text_source(source, request, "message"), vec![]),
        "source.text" => (render_text_source(source, request, "text"), vec![]),
        source_id => (
            vec![],
            vec![PlayerError::new(
                "unsupportedSourceAdapter",
                "sources.*.source",
                format!("No player adapter registered for {source_id}"),
                Some("Add a contract-native source adapter before expecting pixels."),
                serde_json::json!({ "source": source_id }),
            )],
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

// <FILE>crates/tui-vfx-player/src/fnc_render_scene.rs</FILE> - <DESC>Render recipe scenes into K0 text-grid rows</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
