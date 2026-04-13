// <FILE>crates/tui-vfx-probe/src/bin/pipeline-probe.rs</FILE> - <DESC>CLI entry point for tui-vfx-probe</DESC>
// <VERS>VERSION: 0.6.0</VERS>
// <WCTX>Embedded SQLite query backend polish</WCTX>
// <CLOG>PATCH: Remove an unreachable return and keep the SQLite query flag as the single branching point for query-vs-report output</CLOG>

//! Command-line wrapper for the `tui-vfx-probe` library.
//!
//! `pipeline-probe` accepts a direct `ProbeSceneSpec` JSON document and can emit:
//!
//! - a single frame dump
//! - a timeline sampled evenly across one phase (`--frames N`)
//! - a frame diff between two phase-local times (`--diff-to T`)

use std::fs;

use tui_vfx_probe::{
    ProbeCellSelector, ProbePhase, ProbeRequest, ProbeSceneSpec, ProbeSqliteStore,
    collect_timeline, run_probe, run_probe_diff,
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
    let mut sqlite_query = None;

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
            "--sqlite-query" => sqlite_query = args.next(),
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
        return print_query_or_output(&format, sqlite_query.as_deref(), &timeline, |store| {
            store.ingest_timeline("run", &timeline)
        });
    }

    if let Some(to_t) = diff_to {
        let diff = run_probe_diff(&scene, phase, sample_t, to_t, with_causation)?;
        return print_query_or_output(&format, sqlite_query.as_deref(), &diff, |store| {
            store.ingest_diff("run", &diff)
        });
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
    print_query_or_output(&format, sqlite_query.as_deref(), &report, |store| {
        store.ingest_report("run", &report)
    })
}

fn print_query_or_output<T>(
    format: &str,
    sqlite_query: Option<&str>,
    value: &T,
    ingest: impl FnOnce(&ProbeSqliteStore) -> Result<(), rusqlite::Error>,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: serde::Serialize,
{
    if let Some(sql) = sqlite_query {
        let store = ProbeSqliteStore::new_in_memory()?;
        ingest(&store)?;
        let rows = store.query_json(sql)?;
        return print_output(format, &rows);
    }
    print_output(format, value)
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
// <VERS>END OF VERSION: 0.6.0</VERS>
