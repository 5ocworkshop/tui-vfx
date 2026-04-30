// <FILE>crates/tui-vfx-player-cli/src/fnc_run_play_backend.rs</FILE> - <DESC>Run live backend playback CLI command</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>v3.1 player backend playback: provide actual timed compositor-colored playback instead of only static frame dumps.</WCTX>
// <CLOG>0.1.0: INIT — repeatedly sample one recipe, render through selected backend, and paint ANSI frames with timing.</CLOG>

use std::{io::Write, thread, time::Duration};

use serde_json::json;

use crate::{
    cls_cli_options::CliOptions,
    fnc_cli_sample_request::sample_time_from_millis,
    fnc_collect_cli_recipe_paths::collect_cli_recipe_paths,
    fnc_run_render_backend::{
        backend_output_to_ansi, backend_output_to_text, render_backend_for_path,
    },
};

/// Run a live terminal playback loop for one recipe/backend.
pub fn run_play_backend(mut options: CliOptions) -> Result<(), String> {
    validate_playback_options(&options)?;
    let paths = collect_cli_recipe_paths(&options)?;
    let Some(path) = paths.first() else {
        return Err("play-backend requires one recipe path".to_string());
    };
    if paths.len() > 1 {
        return Err("play-backend currently accepts exactly one recipe path".to_string());
    }

    let frame_count = playback_frame_count(&options);
    let frame_delay_ms = options
        .sample_ms
        .unwrap_or_else(|| 1000u64.saturating_div(options.fps).max(1));

    if options.format == "json" {
        let mut frames = Vec::with_capacity(frame_count);
        for frame_index in 0..frame_count {
            let elapsed_ms = elapsed_ms_for_frame(frame_index, frame_count, options.duration_ms);
            let (phase_t, loop_t) = sample_time_from_millis(elapsed_ms, options.duration_ms);
            options.phase_t = phase_t;
            options.loop_t = loop_t;
            options.sample_ms = None;
            let output = render_backend_for_path(&options, path)?;
            frames.push(json!({
                "frame": frame_index,
                "sample": {
                    "sampleMs": elapsed_ms,
                    "phaseT": phase_t,
                    "loopT": loop_t,
                },
                "output": output,
            }));
        }
        println!(
            "{}",
            serde_json::to_string_pretty(&json!({
                "schemaVersion": "v3.1.player.backendPlayback.1",
                "backend": options.backend,
                "format": options.format,
                "fps": options.fps,
                "durationMs": options.duration_ms,
                "recipePath": path.display().to_string(),
                "frames": frames,
            }))
            .expect("playback report serializes")
        );
        return Ok(());
    }

    let mut stdout = std::io::stdout();
    for frame_index in 0..frame_count {
        let elapsed_ms = elapsed_ms_for_frame(frame_index, frame_count, options.duration_ms);
        let (phase_t, loop_t) = sample_time_from_millis(elapsed_ms, options.duration_ms);
        options.phase_t = phase_t;
        options.loop_t = loop_t;
        options.sample_ms = None;

        let output = render_backend_for_path(&options, path)?;
        if !options.no_clear {
            write!(stdout, "\x1b[2J\x1b[H").map_err(|error| error.to_string())?;
        }
        writeln!(
            stdout,
            "frame: {} of {} elapsed_ms={} backend={} recipe={} render_hash={} backend_hash={} styled_cells={}",
            frame_index,
            frame_count,
            elapsed_ms,
            output.backend,
            output.recipe_id,
            output.render_hash,
            output.backend_hash,
            output.non_default_styled_cells
        )
        .map_err(|error| error.to_string())?;
        match options.format.as_str() {
            "text" => write!(stdout, "{}", backend_output_to_text(&output))
                .map_err(|error| error.to_string())?,
            "ansi" => write!(stdout, "{}", backend_output_to_ansi(&output))
                .map_err(|error| error.to_string())?,
            other => {
                return Err(format!(
                    "unknown format `{other}`; expected json, ansi, or text"
                ));
            }
        }
        stdout.flush().map_err(|error| error.to_string())?;
        if frame_delay_ms > 0 && frame_index + 1 < frame_count {
            thread::sleep(Duration::from_millis(frame_delay_ms));
        }
    }
    Ok(())
}

fn validate_playback_options(options: &CliOptions) -> Result<(), String> {
    if options.fps == 0 {
        return Err("fps must be greater than 0".to_string());
    }
    if options.duration_ms == 0 {
        return Err("duration-ms must be greater than 0".to_string());
    }
    Ok(())
}

fn playback_frame_count(options: &CliOptions) -> usize {
    if options.frames > 1 {
        return options.frames;
    }
    ((options.duration_ms.saturating_mul(options.fps)) / 1000).max(2) as usize
}

fn elapsed_ms_for_frame(frame_index: usize, frame_count: usize, duration_ms: u64) -> u64 {
    if frame_count <= 1 {
        return 0;
    }
    (frame_index as u64 * duration_ms) / frame_count as u64
}

// <FILE>crates/tui-vfx-player-cli/src/fnc_run_play_backend.rs</FILE> - <DESC>Run live backend playback CLI command</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
