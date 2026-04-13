// <FILE>crates/tui-vfx-probe/src/bin/pipeline-probe.rs</FILE> - <DESC>CLI entry point for tui-vfx-probe</DESC>
// <VERS>VERSION: 0.4.0</VERS>
// <WCTX>Phase-1.5 probe timeline and diff support</WCTX>
// <CLOG>MINOR: Add --frames and --diff-to modes so the CLI can emit timeline and frame-diff reports in addition to single frame dumps</CLOG>

//! Command-line wrapper for the `tui-vfx-probe` library.
//!
//! `pipeline-probe` accepts a direct `ProbeSceneSpec` JSON document and can emit:
//!
//! - a single frame dump
//! - a timeline sampled evenly across one phase (`--frames N`)
//! - a frame diff between two phase-local times (`--diff-to T`)

use std::fs;

use tui_vfx_probe::{
    ProbeCellSelector, ProbePhase, ProbeRequest, ProbeSceneSpec, collect_timeline, run_probe,
    run_probe_diff,
};

fn main() {
    if let Err(error) = run() {
        eprintln!("{error}");
        std::process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let mut input_path = None;
    let mut format = String::from("json");
    let mut phase = ProbePhase::Dwelling;
    let mut sample_t = 0.5;
    let mut cells = ProbeCellSelector::All;
    let mut with_causation = false;
    let mut frames = None;
    let mut diff_to = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--input" => input_path = args.next(),
            "--format" => format = args.next().ok_or("missing value for --format")?,
            "--phase" => {
                phase = match args.next().ok_or("missing value for --phase")?.as_str() {
                    "entering" => ProbePhase::Entering,
                    "dwelling" => ProbePhase::Dwelling,
                    "exiting" => ProbePhase::Exiting,
                    other => return Err(format!("unsupported phase: {other}").into()),
                }
            }
            "--sample-t" => {
                sample_t = args
                    .next()
                    .ok_or("missing value for --sample-t")?
                    .parse::<f64>()?;
            }
            "--cells" => {
                cells = match args.next().ok_or("missing value for --cells")?.as_str() {
                    "all" => ProbeCellSelector::All,
                    "non-empty" => ProbeCellSelector::NonEmpty,
                    "modified" => ProbeCellSelector::Modified,
                    other => return Err(format!("unsupported cell selector: {other}").into()),
                }
            }
            "--with-causation" => with_causation = true,
            "--frames" => {
                frames = Some(
                    args.next()
                        .ok_or("missing value for --frames")?
                        .parse::<usize>()?,
                );
            }
            "--diff-to" => {
                diff_to = Some(
                    args.next()
                        .ok_or("missing value for --diff-to")?
                        .parse::<f64>()?,
                );
            }
            other => return Err(format!("unsupported argument: {other}").into()),
        }
    }

    if frames.is_some() && diff_to.is_some() {
        return Err("--frames and --diff-to are mutually exclusive".into());
    }

    let input_path = input_path.ok_or("--input is required")?;
    let scene: ProbeSceneSpec = serde_json::from_str(&fs::read_to_string(input_path)?)?;

    if let Some(frame_count) = frames {
        let timeline = collect_timeline(
            &scene,
            phase,
            frame_count,
            &ProbeRequest {
                phase,
                sample_t,
                cells,
                with_causation,
            },
        )?;
        print_output(&format, &timeline)?;
        return Ok(());
    }

    if let Some(to_t) = diff_to {
        let diff = run_probe_diff(&scene, phase, sample_t, to_t, with_causation)?;
        print_output(&format, &diff)?;
        return Ok(());
    }

    let report = run_probe(
        &scene,
        &ProbeRequest {
            phase,
            sample_t,
            cells,
            with_causation,
        },
    )?;
    print_output(&format, &report)?;
    Ok(())
}

fn print_output(
    format: &str,
    value: &impl serde::Serialize,
) -> Result<(), Box<dyn std::error::Error>> {
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(value)?),
        "ndjson" => println!("{}", serde_json::to_string(value)?),
        other => return Err(format!("unsupported format: {other}").into()),
    }
    Ok(())
}

// <FILE>crates/tui-vfx-probe/src/bin/pipeline-probe.rs</FILE> - <DESC>CLI entry point for tui-vfx-probe</DESC>
// <VERS>END OF VERSION: 0.4.0</VERS>
