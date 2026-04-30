// <FILE>crates/tui-vfx-player-cli/src/fnc_run_studio_snapshot.rs</FILE> - <DESC>Run studio-snapshot CLI command</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>v3.1 studio control pilot: derive controls from descriptors/recipe data and prove scripted control changes affect rendering.</WCTX>
// <CLOG>0.3.0: MINOR — move changed-cell counting into its own function file.</CLOG>

use std::path::Path;

use serde_json::json;
use tui_vfx_contract::{RecipeDocument, SignalId, Value};
use tui_vfx_player::{
    PlayerRenderBackendOutput, PlayerRenderBackendRequest, PlayerRenderIrReport, RecipePlayer,
    build_control_catalog_report, load_descriptor_catalog, render_recipe_file_ir,
};

use crate::{
    cls_cli_options::CliOptions,
    fnc_cli_sample_request::cli_sample_request,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
    fnc_count_studio_snapshot_changed_cells::count_studio_snapshot_changed_cells,
    fnc_run_render_backend::{backend_options, validate_backend_output},
};

/// Run the studio-snapshot command for one recipe and scripted control assignments.
pub fn run_studio_snapshot(options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let Some(path) = paths.first() else {
        return Err("studio-snapshot requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err("studio-snapshot currently accepts exactly one recipe path".to_string());
    }
    let recipe_json = read_recipe_json(path)?;
    let recipe_document: RecipeDocument = serde_json::from_value(recipe_json.clone())
        .map_err(|error| format!("failed to parse recipe `{}`: {error}", path.display()))?;
    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let control_report = build_control_catalog_report(
        descriptor_load.reports,
        &descriptor_load.catalog,
        Some(path),
    )?;
    let catalog = descriptor_load.catalog;
    let player = RecipePlayer::new(catalog.clone());
    let backend_options = backend_options(&options)?;

    let mut before_request = cli_sample_request(&options);
    before_request.phase_t = 0.0;
    before_request.loop_t = Some(0.0);
    let before_ir = render_recipe_file_ir(&player, path, &before_request);
    let before_source_ir =
        render_recipe_file_source_ir(&player, &recipe_document, path, &before_request);
    let before = render_backend_snapshot(
        &options,
        &recipe_document,
        catalog.clone(),
        &before_request,
        before_ir,
        before_source_ir,
        backend_options.clone(),
    )?;

    let mut after_request = before_request.clone();
    let mutations = apply_set_assignments(&recipe_json, &options.sets, &mut after_request)?;
    let after_ir = render_recipe_file_ir(&player, path, &after_request);
    let after_source_ir =
        render_recipe_file_source_ir(&player, &recipe_document, path, &after_request);
    let after = render_backend_snapshot(
        &options,
        &recipe_document,
        catalog,
        &after_request,
        after_ir,
        after_source_ir,
        backend_options,
    )?;
    let changed_cells = count_studio_snapshot_changed_cells(&before, &after);
    let studio_diagnostics = studio_mutation_diagnostics(&mutations, changed_cells);
    let before = before.with_changed_cells(changed_cells);
    let after = after.with_changed_cells(changed_cells);

    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schemaVersion": "v3.1.player.studioSnapshot.1",
            "backend": options.backend,
            "recipePath": path.display().to_string(),
            "controls": control_report.controls,
            "mutations": mutations,
            "studioDiagnostics": studio_diagnostics,
            "beforeBackendHash": before.backend_hash,
            "afterBackendHash": after.backend_hash,
            "changedCells": changed_cells,
            "before": before,
            "after": after,
        }))
        .expect("studio snapshot serializes")
    );
    Ok(())
}

fn studio_mutation_diagnostics(
    mutations: &[serde_json::Value],
    changed_cells: usize,
) -> Vec<serde_json::Value> {
    if mutations.is_empty() || changed_cells > 0 {
        return Vec::new();
    }
    vec![json!({
        "code": "studioMutationNoVisualChange",
        "message": "Studio mutation was accepted but did not change rendered backend cells for this sample.",
    })]
}

fn render_backend_snapshot(
    options: &CliOptions,
    recipe: &RecipeDocument,
    descriptor_catalog: tui_vfx_contract::DescriptorCatalog,
    sample: &tui_vfx_player::PlayerSampleRequest,
    ir: tui_vfx_player::PlayerRenderIrReport,
    source_ir: tui_vfx_player::PlayerRenderIrReport,
    backend_options: tui_vfx_player::PlayerRenderBackendOptions,
) -> Result<PlayerRenderBackendOutput, String> {
    let output = if options.backend == "compositor" {
        tui_vfx_player_backend_compositor::render_compositor_backend_request(
            &PlayerRenderBackendRequest {
                ir,
                source_ir,
                recipe: recipe.clone(),
                descriptor_catalog,
                sample: sample.clone(),
                backend_options,
            },
        )
    } else {
        crate::fnc_run_render_backend::render_report_with_backend(&ir, &options.backend)?
    };
    validate_backend_output(&output, options)?;
    Ok(output)
}

fn render_recipe_file_source_ir(
    player: &RecipePlayer,
    recipe: &RecipeDocument,
    path: &Path,
    request: &tui_vfx_player::PlayerSampleRequest,
) -> PlayerRenderIrReport {
    let mut report = player.render_recipe_source_ir(recipe, request);
    report.path = Some(path.display().to_string());
    report
}

fn read_recipe_json(path: &Path) -> Result<serde_json::Value, String> {
    let text = std::fs::read_to_string(path)
        .map_err(|error| format!("failed to read recipe `{}`: {error}", path.display()))?;
    serde_json::from_str(&text)
        .map_err(|error| format!("failed to parse recipe `{}`: {error}", path.display()))
}

fn apply_set_assignments(
    recipe: &serde_json::Value,
    assignments: &[String],
    request: &mut tui_vfx_player::PlayerSampleRequest,
) -> Result<Vec<serde_json::Value>, String> {
    let mut mutations = Vec::new();
    for assignment in assignments {
        let (key, value) = parse_set_assignment(assignment)?;
        if let Some(signal_id) = signal_id_for_set_key(recipe, key) {
            request
                .signals
                .insert(SignalId::new(signal_id.clone()), value.clone());
            mutations.push(json!({
                "control": key,
                "targetKind": "signal",
                "signalId": signal_id,
                "value": value,
            }));
        } else if let Some(runtime_key) = runtime_override_key_for_set_key(recipe, key) {
            request
                .runtime_input_overrides
                .insert(runtime_key.clone(), value.clone());
            mutations.push(json!({
                "control": key,
                "targetKind": "runtimeInputOverride",
                "runtimeInput": runtime_key,
                "value": value,
            }));
        } else {
            return Err(format!(
                "could not map studio control `{key}` to a recipe signal or runtime input"
            ));
        }
    }
    Ok(mutations)
}

fn parse_set_assignment(assignment: &str) -> Result<(&str, Value), String> {
    let (key, value) = assignment
        .split_once('=')
        .ok_or_else(|| format!("studio --set expects key=value, got `{assignment}`"))?;
    Ok((key, parse_control_value(value)))
}

fn signal_id_for_set_key(recipe: &serde_json::Value, key: &str) -> Option<String> {
    let normalized_key = normalize_key(key);
    if normalized_key == "sweepprogress" {
        return Some("sweepPosition".to_string());
    }
    if normalized_key == "demoprogress" {
        return Some("pillProgress".to_string());
    }

    let signals = recipe.pointer("/graph/signals")?.as_object()?;
    for signal_id in signals.keys() {
        if normalize_key(signal_id) == normalized_key {
            return Some(signal_id.clone());
        }
    }

    let mut input_matches = Vec::new();
    if let Some(nodes) = recipe
        .pointer("/graph/nodes")
        .and_then(|value| value.as_object())
    {
        for node in nodes.values() {
            let Some(inputs) = node.get("inputs").and_then(|value| value.as_object()) else {
                continue;
            };
            for (input_name, source) in inputs {
                if normalize_key(input_name) == normalized_key
                    && source.get("kind").and_then(|value| value.as_str()) == Some("signal")
                    && let Some(signal_id) = source.get("id").and_then(|value| value.as_str())
                {
                    input_matches.push(signal_id.to_string());
                }
            }
        }
    }
    input_matches.sort();
    input_matches.dedup();
    if input_matches.len() == 1 {
        return input_matches.pop();
    }

    if signals.len() == 1
        && (normalized_key.ends_with("progress") || normalized_key.ends_with("position"))
    {
        return signals.keys().next().cloned();
    }
    None
}

fn runtime_override_key_for_set_key(recipe: &serde_json::Value, key: &str) -> Option<String> {
    let normalized_key = normalize_key(key);

    let mut matches = Vec::new();
    collect_effect_runtime_override_matches(recipe, &normalized_key, &mut matches);
    collect_source_runtime_override_matches(recipe, &normalized_key, &mut matches);
    matches.sort();
    matches.dedup();
    (matches.len() == 1).then(|| matches.remove(0))
}

fn collect_effect_runtime_override_matches(
    recipe: &serde_json::Value,
    normalized_key: &str,
    matches: &mut Vec<String>,
) {
    if let Some(nodes) = recipe
        .pointer("/graph/nodes")
        .and_then(|value| value.as_object())
    {
        for node in nodes.values() {
            let Some(node_id) = node.get("id").and_then(|value| value.as_str()) else {
                continue;
            };
            let Some(effect_id) = node.get("effect").and_then(|value| value.as_str()) else {
                continue;
            };
            let Some(inputs) = node.get("inputs").and_then(|value| value.as_object()) else {
                continue;
            };
            for input_name in inputs.keys() {
                let candidates = [
                    input_name.to_string(),
                    format!("{node_id}.{input_name}"),
                    format!("{effect_id}.{input_name}"),
                    format!("effect:{effect_id}:{node_id}:{input_name}"),
                    format!("effect:{effect_id}:{input_name}"),
                ];
                if candidates
                    .iter()
                    .any(|candidate| normalize_key(candidate) == normalized_key)
                {
                    matches.push(format!("effect:{effect_id}:{node_id}:{input_name}"));
                }
            }
        }
    }
}

fn collect_source_runtime_override_matches(
    recipe: &serde_json::Value,
    normalized_key: &str,
    matches: &mut Vec<String>,
) {
    if let Some(sources) = recipe
        .pointer("/sources")
        .and_then(|value| value.as_object())
    {
        for (instance_id, source) in sources {
            let Some(source_id) = source.get("source").and_then(|value| value.as_str()) else {
                continue;
            };
            let Some(inputs) = source.get("inputs").and_then(|value| value.as_object()) else {
                continue;
            };
            for input_name in inputs.keys() {
                let candidates = [
                    input_name.to_string(),
                    format!("{instance_id}.{input_name}"),
                    format!("{source_id}.{input_name}"),
                    format!("source:{source_id}:{instance_id}:{input_name}"),
                    format!("source:{source_id}:{input_name}"),
                ];
                if candidates
                    .iter()
                    .any(|candidate| normalize_key(candidate) == normalized_key)
                {
                    matches.push(format!("source:{source_id}:{instance_id}:{input_name}"));
                }
            }
        }
    }
}

fn normalize_key(value: &str) -> String {
    value
        .chars()
        .filter(|ch| ch.is_ascii_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect()
}

fn parse_control_value(value: &str) -> Value {
    if value.eq_ignore_ascii_case("true") {
        return Value::Boolean(true);
    }
    if value.eq_ignore_ascii_case("false") {
        return Value::Boolean(false);
    }
    if let Ok(integer) = value.parse::<i64>() {
        return Value::Integer(integer);
    }
    if let Ok(number) = value.parse::<f64>() {
        return Value::Number(number);
    }
    Value::String(value.to_string())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_studio_snapshot.rs</FILE> - <DESC>Run studio-snapshot CLI command</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
