// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_backend.rs</FILE> - <DESC>Run render-backend CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 player backend playback: expose player backend rendering with JSON, ANSI, and text output.</WCTX>
// <CLOG>0.1.0: INIT — route render IR through text, styled-cell, or compositor backends and print visible output formats.</CLOG>

use std::path::Path;

use tui_vfx_contract::RecipeDocument;
use tui_vfx_player::{
    PlayerRenderBackend, PlayerRenderBackendOptions, PlayerRenderBackendOutput,
    PlayerRenderBackendRequest, PlayerRenderCell, PlayerRenderCompositionMode,
    PlayerRenderIrReport, RecipePlayer, StyledCellRenderBackend, TextGridRenderBackend,
    load_descriptor_catalog, render_recipe_file_ir,
};
use tui_vfx_player_backend_compositor::{
    CompositorRenderBackend, render_compositor_backend_request,
};

use crate::{
    cls_cli_options::CliOptions, fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
};

/// Run the render-backend command for one recipe.
pub fn run_render_backend(options: CliOptions) -> Result<(), String> {
    let output = render_backend_from_options(&options)?;
    print_backend_output(&output, &options.format)
}

pub(crate) fn render_backend_from_options(
    options: &CliOptions,
) -> Result<PlayerRenderBackendOutput, String> {
    let paths = collect_cli_recipe_paths(options)?;
    let Some(path) = paths.first() else {
        return Err("render-backend requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err("render-backend currently accepts exactly one recipe path".to_string());
    }
    render_backend_for_path(options, path)
}

pub(crate) fn render_backend_for_path(
    options: &CliOptions,
    path: &Path,
) -> Result<PlayerRenderBackendOutput, String> {
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let catalog = descriptor_load.catalog;
    let player = RecipePlayer::new(catalog.clone());
    let request = cli_sample_request(options);
    let report = render_recipe_file_ir(&player, path, &request);
    if options.backend == "compositor" {
        let recipe = read_recipe_document(path)?;
        let mut source_ir = player.render_recipe_source_ir(&recipe, &request);
        source_ir.path = Some(path.display().to_string());
        let backend_request = PlayerRenderBackendRequest {
            ir: report,
            source_ir,
            recipe,
            descriptor_catalog: catalog,
            sample: request,
            backend_options: backend_options(options)?,
        };
        let output = render_compositor_backend_request(&backend_request);
        validate_backend_output(&output, options)?;
        Ok(output)
    } else {
        let output = render_report_with_backend(&report, &options.backend)?;
        validate_backend_output(&output, options)?;
        Ok(output)
    }
}

pub(crate) fn backend_options(options: &CliOptions) -> Result<PlayerRenderBackendOptions, String> {
    Ok(PlayerRenderBackendOptions {
        composition_mode: PlayerRenderCompositionMode::parse(&options.composition_mode)?,
        fail_on_fallback: options.fail_on_fallback,
    })
}

pub(crate) fn validate_backend_output(
    output: &PlayerRenderBackendOutput,
    options: &CliOptions,
) -> Result<(), String> {
    if options.fail_on_fallback && output.fallback_used {
        return Err(format!(
            "backend fallback forbidden by --fail-on-fallback; diagnostics={}",
            diagnostic_codes(output).join(",")
        ));
    }
    if options.composition_mode == "native" && !output.native_lowering_succeeded {
        return Err(format!(
            "native composition mode did not lower every graph node; diagnostics={}",
            diagnostic_codes(output).join(",")
        ));
    }
    Ok(())
}

fn diagnostic_codes(output: &PlayerRenderBackendOutput) -> Vec<String> {
    output
        .diagnostics
        .iter()
        .map(|diagnostic| diagnostic.code.clone())
        .collect()
}

fn read_recipe_document(path: &Path) -> Result<RecipeDocument, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read recipe `{}`: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse recipe `{}`: {error}", path.display()))
}

pub(crate) fn render_report_with_backend(
    report: &PlayerRenderIrReport,
    backend: &str,
) -> Result<PlayerRenderBackendOutput, String> {
    match backend {
        "text" | "textGrid" | "text-grid" => Ok(TextGridRenderBackend.render(report)),
        "styled" | "styledCell" | "styled-cell" => Ok(StyledCellRenderBackend.render(report)),
        "compositor" => Ok(CompositorRenderBackend.render(report)),
        other => Err(format!(
            "unknown backend `{other}`; expected textGrid, styledCell, or compositor"
        )),
    }
}

pub(crate) fn print_backend_output(
    output: &PlayerRenderBackendOutput,
    format: &str,
) -> Result<(), String> {
    match format {
        "json" => println!(
            "{}",
            serde_json::to_string_pretty(output).expect("backend output serializes")
        ),
        "text" => print!("{}", backend_output_to_text(output)),
        "ansi" => print!("{}", backend_output_to_ansi(output)),
        other => {
            return Err(format!(
                "unknown format `{other}`; expected json, ansi, or text"
            ));
        }
    }
    Ok(())
}

pub(crate) fn backend_output_to_text(output: &PlayerRenderBackendOutput) -> String {
    let mut rendered = String::new();
    rendered.push_str(&format!("backend: {}\n", output.backend));
    rendered.push_str(&format!("composition_mode: {}\n", output.composition_mode));
    rendered.push_str(&format!("fallback_used: {}\n", output.fallback_used));
    rendered.push_str(&format!(
        "composition_spec_non_empty: {}\n",
        output.composition_spec_non_empty
    ));
    rendered.push_str(&format!("recipe: {}\n", output.recipe_id));
    rendered.push_str(&format!("render_hash: {}\n", output.render_hash));
    rendered.push_str(&format!("backend_hash: {}\n", output.backend_hash));
    rendered.push_str(&format!(
        "non_default_styled_cells: {}\n",
        output.non_default_styled_cells
    ));
    for row in &output.rows {
        rendered.push_str(row);
        rendered.push('\n');
    }
    rendered
}

pub(crate) fn backend_output_to_ansi(output: &PlayerRenderBackendOutput) -> String {
    let width = output
        .rows
        .iter()
        .map(|row| row.chars().count())
        .max()
        .unwrap_or(0)
        .max(
            output
                .styled_cells
                .iter()
                .map(|cell| cell.x + 1)
                .max()
                .unwrap_or(0),
        );
    let height = output.rows.len().max(
        output
            .styled_cells
            .iter()
            .map(|cell| cell.y + 1)
            .max()
            .unwrap_or(0),
    );
    let mut dense = output
        .rows
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, ch)| PlayerRenderCell {
                    x,
                    y,
                    glyph: ch.to_string(),
                    foreground: "transparent".to_string(),
                    background: "transparent".to_string(),
                    modifiers: vec![],
                    role: None,
                })
        })
        .collect::<Vec<_>>();
    dense.extend(output.styled_cells.clone());
    let mut cells = vec![
        PlayerRenderCell {
            x: 0,
            y: 0,
            glyph: " ".to_string(),
            foreground: "transparent".to_string(),
            background: "transparent".to_string(),
            modifiers: vec![],
            role: None,
        };
        width.saturating_mul(height)
    ];
    for cell in dense {
        if cell.x < width && cell.y < height {
            let index = cell.y * width + cell.x;
            cells[index] = cell;
        }
    }
    let mut rendered = String::new();
    for y in 0..height {
        for x in 0..width {
            let cell = &cells[y * width + x];
            rendered.push_str(&sgr_for_cell(cell));
            rendered.push_str(&cell.glyph);
        }
        rendered.push_str("\x1b[0m\n");
    }
    rendered
}

fn sgr_for_cell(cell: &PlayerRenderCell) -> String {
    let mut codes = Vec::new();
    if let Some((r, g, b)) = rgb_from_label(&cell.foreground) {
        codes.push(format!("38;2;{r};{g};{b}"));
    }
    if let Some((r, g, b)) = rgb_from_label(&cell.background) {
        codes.push(format!("48;2;{r};{g};{b}"));
    }
    for modifier in &cell.modifiers {
        match modifier.as_str() {
            "bold" => codes.push("1".to_string()),
            "dim" => codes.push("2".to_string()),
            "italic" => codes.push("3".to_string()),
            "underline" => codes.push("4".to_string()),
            "reverse" => codes.push("7".to_string()),
            "strikethrough" => codes.push("9".to_string()),
            _ => {}
        }
    }
    if codes.is_empty() {
        "\x1b[0m".to_string()
    } else {
        format!("\x1b[{}m", codes.join(";"))
    }
}

fn rgb_from_label(label: &str) -> Option<(u8, u8, u8)> {
    let inner = label.strip_prefix("rgba(")?.strip_suffix(')')?;
    let mut parts = inner.split(',').map(str::trim);
    let r = parts.next()?.parse::<u8>().ok()?;
    let g = parts.next()?.parse::<u8>().ok()?;
    let b = parts.next()?.parse::<u8>().ok()?;
    let a = parts.next()?.parse::<u8>().ok()?;
    if a == 0 || parts.next().is_some() {
        None
    } else {
        Some((r, g, b))
    }
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_backend.rs</FILE> - <DESC>Run render-backend CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
