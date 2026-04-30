// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_backend_timeline.rs</FILE> - <DESC>Run render-backend-timeline CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 player backend playback: emit repeated backend samples for visible animation/effect evidence.</WCTX>
// <CLOG>0.1.0: INIT — sample one recipe through the selected backend and serialize backend output samples.</CLOG>

use std::{thread, time::Duration};

use serde_json::json;

use crate::{
    cls_cli_options::CliOptions,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
    fnc_run_render_backend::{
        backend_output_to_ansi, backend_output_to_text, render_backend_for_path,
    },
};

/// Run the render-backend-timeline command for one recipe.
pub fn run_render_backend_timeline(mut options: CliOptions) -> Result<(), String> {
    let paths = collect_cli_recipe_paths(&options)?;
    let Some(path) = paths.first() else {
        return Err("render-backend-timeline requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err(
            "render-backend-timeline currently accepts exactly one recipe path".to_string(),
        );
    }
    let samples = options.samples.max(1);
    let mut outputs = Vec::with_capacity(samples);
    let frame_delay_ms = options.sample_ms;
    for index in 0..samples {
        options.phase_t = if samples == 1 {
            options.phase_t
        } else {
            index as f64 / (samples - 1) as f64
        };
        options.loop_t = Some(options.phase_t);
        options.sample_ms = Some((options.phase_t * options.duration_ms as f64).round() as u64);
        outputs.push(render_backend_for_path(&options, path)?);
    }

    if options.format == "json" {
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "v3.1.player.renderBackendTimeline.1",
                "backend": options.backend,
                "recipePath": path.display().to_string(),
                "sampleMs": frame_delay_ms,
                "samples": outputs,
            }))
            .expect("backend timeline serializes")
        );
        return Ok(());
    }

    for (index, output) in outputs.iter().enumerate() {
        if !options.no_clear {
            print!("\x1b[2J\x1b[H");
        }
        println!(
            "frame {}/{} backend={} recipe={} render_hash={} backend_hash={} styled_cells={}",
            index + 1,
            outputs.len(),
            output.backend,
            output.recipe_id,
            output.render_hash,
            output.backend_hash,
            output.non_default_styled_cells
        );
        match options.format.as_str() {
            "text" => print!("{}", backend_output_to_text(output)),
            "ansi" => print!("{}", backend_output_to_ansi(output)),
            other => {
                return Err(format!(
                    "unknown format `{other}`; expected json, ansi, or text"
                ));
            }
        }
        if let Some(delay_ms) = frame_delay_ms
            && delay_ms > 0
            && index + 1 < outputs.len()
        {
            thread::sleep(Duration::from_millis(delay_ms));
        }
    }
    Ok(())
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_render_backend_timeline.rs</FILE> - <DESC>Run render-backend-timeline CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
