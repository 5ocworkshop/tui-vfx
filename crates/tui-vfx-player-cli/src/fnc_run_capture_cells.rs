// <FILE>crates/tui-vfx-player-cli/src/fnc_run_capture_cells.rs</FILE> - <DESC>Run v3.1 cell-capture SQLite command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 validation tooling: persist dense per-frame/player-cell evidence for deterministic parity localization.</WCTX>
// <CLOG>0.1.0: INIT — add capture-cells SQLite writer over player-owned render IR.</CLOG>

use std::{collections::HashMap, path::Path};

use rusqlite::{Connection, params};
use serde_json::json;
use tui_vfx_player::{
    PlayerRenderCell, PlayerRenderIrReport, RecipePlayer, load_descriptor_catalog,
    render_recipe_file_ir,
};

use crate::{
    cls_cli_options::CliOptions,
    fnc_cli_sample_request::{cli_sample_request, sample_time_from_millis},
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
};

const CAPTURE_SCHEMA_VERSION: &str = "v3.1.player.cellCapture.sqlite.1";

/// Run the capture-cells command and write dense frame/cell evidence to SQLite.
pub fn run_capture_cells(mut options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let Some(path) = paths.first() else {
        return Err("capture-cells requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err("capture-cells currently accepts exactly one recipe path".to_string());
    }
    let sqlite_output = options
        .sqlite_output
        .clone()
        .ok_or_else(|| "capture-cells requires --sqlite-output PATH".to_string())?;

    if let Some(parent) = Path::new(&sqlite_output).parent()
        && !parent.as_os_str().is_empty()
    {
        std::fs::create_dir_all(parent)
            .map_err(|error| format!("create sqlite output directory failed: {error}"))?;
    }
    if Path::new(&sqlite_output).exists() {
        std::fs::remove_file(&sqlite_output)
            .map_err(|error| format!("replace sqlite output failed: {error}"))?;
    }

    let descriptor_load =
        load_descriptor_catalog(&options.descriptor_packs, &options.descriptor_pack_dirs)?;
    let player = RecipePlayer::new(descriptor_load.catalog);
    let mut conn = Connection::open(&sqlite_output)
        .map_err(|error| format!("open sqlite output failed: {error}"))?;
    create_capture_schema(&conn)
        .map_err(|error| format!("create sqlite schema failed: {error}"))?;

    let frames = options.frames.max(1);
    let fixed_sample_ms = options.sample_ms;
    let mut reports = Vec::with_capacity(frames);
    let mut frame_sample_ms = Vec::with_capacity(frames);
    for frame_index in 0..frames {
        let sample_ms = if let Some(sample_ms) = fixed_sample_ms {
            sample_ms
        } else if frames == 1 {
            (options.phase_t.clamp(0.0, 1.0) * options.duration_ms as f64).round() as u64
        } else {
            ((options.duration_ms as f64 * frame_index as f64) / (frames - 1) as f64).round() as u64
        };
        let (phase_t, loop_t) = sample_time_from_millis(sample_ms, options.duration_ms);
        options.phase_t = phase_t;
        options.loop_t = loop_t;
        options.sample_ms = Some(sample_ms);
        let request = cli_sample_request(&options);
        frame_sample_ms.push(sample_ms);
        reports.push(render_recipe_file_ir(&player, path, &request));
    }

    let run_id = capture_run_id(path);
    insert_run(
        &mut conn,
        &run_id,
        path,
        &sqlite_output,
        &reports,
        &frame_sample_ms,
        &options,
    )
    .map_err(|error| format!("write sqlite capture failed: {error}"))?;

    let cell_count: usize = reports
        .iter()
        .map(|report| report.width.saturating_mul(report.height))
        .sum();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "schemaVersion": "v3.1.player.cellCapture.1",
            "sqliteSchemaVersion": CAPTURE_SCHEMA_VERSION,
            "sqliteOutput": sqlite_output,
            "runId": run_id,
            "recipeId": reports.first().map(|report| report.recipe_id.as_str()).unwrap_or(""),
            "recipePath": path.display().to_string(),
            "frameCount": reports.len(),
            "cellCount": cell_count,
            "clock": reports.first().map(|report| serde_json::json!({
                "mode": report.clock.mode,
                "periodMs": report.clock.period_ms,
            })),
        }))
        .expect("capture summary serializes")
    );
    Ok(())
}

fn create_capture_schema(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute_batch(
        "
        create table player_capture_runs (
            run_id text primary key,
            schema_version text not null,
            recipe_id text not null,
            recipe_path text not null,
            sqlite_output text not null,
            frame_count integer not null,
            cell_count integer not null,
            descriptor_packs_json text not null,
            descriptor_pack_dirs_json text not null
        );
        create table player_capture_frames (
            run_id text not null,
            frame_index integer not null,
            recipe_id text not null,
            recipe_path text,
            status text not null,
            phase text not null,
            phase_t real not null,
            loop_t real,
            clock_mode text not null,
            clock_period_ms real,
            absolute_t_ms real,
            sample_ms integer,
            width integer not null,
            height integer not null,
            render_hash text not null,
            non_empty_cells integer not null,
            error_count integer not null,
            warning_count integer not null,
            rows_json text not null,
            primary key (run_id, frame_index)
        );
        create table player_capture_cells (
            run_id text not null,
            frame_index integer not null,
            recipe_id text not null,
            row integer not null,
            col integer not null,
            glyph text not null,
            foreground text not null,
            background text not null,
            modifiers_json text not null,
            role text,
            scene_id text,
            element_id text,
            layer_id text,
            source_id text,
            source_descriptor_id text,
            style_known integer not null,
            primary key (run_id, frame_index, row, col)
        );
        create table player_capture_diagnostics (
            run_id text not null,
            frame_index integer not null,
            severity text not null,
            code text not null,
            path text not null,
            message text not null,
            hint text,
            details_json text
        );
        create table player_capture_provenance (
            run_id text not null,
            frame_index integer not null,
            scene_id text not null,
            element_id text not null,
            layer_id text,
            source_id text,
            source_descriptor_id text,
            x integer not null,
            y integer not null,
            z_index integer not null,
            cell_write_policy text not null,
            rendered integer not null,
            skip_reason text
        );
        create table player_capture_layers (
            run_id text not null,
            frame_index integer not null,
            scene_id text not null,
            element_id text not null,
            layer_id text,
            visible integer not null,
            skipped integer not null,
            skip_reason text
        );
        create table player_capture_graph_values (
            run_id text not null,
            frame_index integer not null,
            id text not null,
            value_json text not null
        );
        ",
    )
}

fn insert_run(
    conn: &mut Connection,
    run_id: &str,
    path: &Path,
    sqlite_output: &str,
    reports: &[PlayerRenderIrReport],
    frame_sample_ms: &[u64],
    options: &CliOptions,
) -> rusqlite::Result<()> {
    let transaction = conn.transaction()?;
    let recipe_id = reports
        .first()
        .map(|report| report.recipe_id.as_str())
        .unwrap_or("");
    let cell_count: usize = reports
        .iter()
        .map(|report| report.width.saturating_mul(report.height))
        .sum();
    transaction.execute(
        "insert into player_capture_runs(run_id, schema_version, recipe_id, recipe_path, sqlite_output, frame_count, cell_count, descriptor_packs_json, descriptor_pack_dirs_json) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
        params![
            run_id,
            CAPTURE_SCHEMA_VERSION,
            recipe_id,
            path.display().to_string(),
            sqlite_output,
            reports.len() as i64,
            cell_count as i64,
            serde_json::to_string(&options.descriptor_packs).unwrap_or_else(|_| "[]".to_string()),
            serde_json::to_string(&options.descriptor_pack_dirs).unwrap_or_else(|_| "[]".to_string()),
        ],
    )?;
    for (frame_index, report) in reports.iter().enumerate() {
        insert_frame(
            &transaction,
            run_id,
            frame_index,
            report,
            frame_sample_ms.get(frame_index).copied(),
        )?;
    }
    transaction.commit()?;
    Ok(())
}

fn insert_frame(
    conn: &Connection,
    run_id: &str,
    frame_index: usize,
    report: &PlayerRenderIrReport,
    sample_ms: Option<u64>,
) -> rusqlite::Result<()> {
    conn.execute(
        "insert into player_capture_frames(run_id, frame_index, recipe_id, recipe_path, status, phase, phase_t, loop_t, clock_mode, clock_period_ms, absolute_t_ms, sample_ms, width, height, render_hash, non_empty_cells, error_count, warning_count, rows_json) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19)",
        params![
            run_id,
            frame_index as i64,
            report.recipe_id,
            report.path,
            serde_label(&report.status),
            serde_label(&report.phase),
            report.phase_t,
            report.loop_t,
            report.clock.mode,
            report.clock.period_ms,
            report.clock.absolute_t_ms,
            sample_ms.map(|value| value as i64),
            report.width as i64,
            report.height as i64,
            report.render_hash.to_string(),
            report.non_empty_cells as i64,
            report.errors.len() as i64,
            report.warnings.len() as i64,
            serde_json::to_string(&report.rows).unwrap_or_else(|_| "[]".to_string()),
        ],
    )?;

    let styled_cells = report
        .styled_cells
        .iter()
        .map(|cell| ((cell.x, cell.y), cell))
        .collect::<HashMap<_, _>>();
    for row in 0..report.height {
        for col in 0..report.width {
            let glyph = glyph_at(&report.rows, col, row);
            let styled = styled_cells.get(&(col, row)).copied();
            insert_cell(conn, run_id, frame_index, report, row, col, glyph, styled)?;
        }
    }

    for error in &report.errors {
        conn.execute(
            "insert into player_capture_diagnostics(run_id, frame_index, severity, code, path, message, hint, details_json) values (?1, ?2, 'error', ?3, ?4, ?5, ?6, ?7)",
            params![
                run_id,
                frame_index as i64,
                error.code,
                error.path,
                error.message,
                error.hint,
                serde_json::to_string(&error.details).unwrap_or_else(|_| "null".to_string()),
            ],
        )?;
    }
    for warning in &report.warnings {
        conn.execute(
            "insert into player_capture_diagnostics(run_id, frame_index, severity, code, path, message, hint, details_json) values (?1, ?2, 'warning', ?3, ?4, ?5, ?6, null)",
            params![
                run_id,
                frame_index as i64,
                warning.code,
                warning.path,
                warning.message,
                warning.hint,
            ],
        )?;
    }
    for entry in &report.provenance {
        conn.execute(
            "insert into player_capture_provenance(run_id, frame_index, scene_id, element_id, layer_id, source_id, source_descriptor_id, x, y, z_index, cell_write_policy, rendered, skip_reason) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                run_id,
                frame_index as i64,
                entry.scene_id,
                entry.element_id,
                entry.layer_id,
                entry.source_id,
                entry.source_descriptor_id,
                entry.x as i64,
                entry.y as i64,
                entry.z_index as i64,
                entry.cell_write_policy,
                i64::from(entry.rendered),
                entry.skip_reason,
            ],
        )?;
    }
    for layer in &report.layers {
        conn.execute(
            "insert into player_capture_layers(run_id, frame_index, scene_id, element_id, layer_id, visible, skipped, skip_reason) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
            params![
                run_id,
                frame_index as i64,
                layer.scene_id,
                layer.element_id,
                layer.layer_id,
                i64::from(layer.visible),
                i64::from(layer.skipped),
                layer.skip_reason,
            ],
        )?;
    }
    for graph_value in &report.graph_values {
        conn.execute(
            "insert into player_capture_graph_values(run_id, frame_index, id, value_json) values (?1, ?2, ?3, ?4)",
            params![
                run_id,
                frame_index as i64,
                graph_value.id,
                serde_json::to_string(&graph_value.value).unwrap_or_else(|_| "null".to_string()),
            ],
        )?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn insert_cell(
    conn: &Connection,
    run_id: &str,
    frame_index: usize,
    report: &PlayerRenderIrReport,
    row: usize,
    col: usize,
    glyph: String,
    styled: Option<&PlayerRenderCell>,
) -> rusqlite::Result<()> {
    conn.execute(
        "insert into player_capture_cells(run_id, frame_index, recipe_id, row, col, glyph, foreground, background, modifiers_json, role, scene_id, element_id, layer_id, source_id, source_descriptor_id, style_known) values (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16)",
        params![
            run_id,
            frame_index as i64,
            report.recipe_id,
            row as i64,
            col as i64,
            styled.map(|cell| cell.glyph.clone()).unwrap_or(glyph),
            styled.map(|cell| cell.foreground.as_str()).unwrap_or("defaultForeground"),
            styled.map(|cell| cell.background.as_str()).unwrap_or("transparent"),
            styled
                .map(|cell| serde_json::to_string(&cell.modifiers).unwrap_or_else(|_| "[]".to_string()))
                .unwrap_or_else(|| "[]".to_string()),
            styled.and_then(|cell| cell.role.as_deref()),
            Option::<&str>::None,
            Option::<&str>::None,
            Option::<&str>::None,
            Option::<&str>::None,
            Option::<&str>::None,
            i64::from(styled.is_some()),
        ],
    )?;
    Ok(())
}

fn glyph_at(rows: &[String], col: usize, row: usize) -> String {
    rows.get(row)
        .and_then(|line| line.chars().nth(col))
        .unwrap_or(' ')
        .to_string()
}

fn serde_label<T: serde::Serialize>(value: &T) -> String {
    serde_json::to_value(value)
        .ok()
        .and_then(|value| value.as_str().map(str::to_string))
        .unwrap_or_else(|| "unknown".to_string())
}

fn capture_run_id(path: &Path) -> String {
    let stem = path
        .file_stem()
        .and_then(|stem| stem.to_str())
        .unwrap_or("recipe");
    format!("player-cell-capture-{stem}")
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_capture_cells.rs</FILE> - <DESC>Run v3.1 cell-capture SQLite command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
