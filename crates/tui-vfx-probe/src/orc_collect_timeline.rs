// <FILE>crates/tui-vfx-probe/src/orc_collect_timeline.rs</FILE> - <DESC>Collect multiple frame dumps across one phase</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Phase-1.5 probe timeline support</WCTX>
// <CLOG>NEW: Add a timeline orchestrator that samples evenly across a phase using repeated run_probe calls</CLOG>

use crate::cls_probe_error::ProbeError;
use crate::cls_probe_report::ProbeReportSource;
use crate::cls_probe_request::{ProbePhase, ProbeRequest};
use crate::cls_probe_scene_spec::ProbeSceneSpec;
use crate::cls_probe_timeline_report::ProbeTimelineReport;
use crate::orc_run_probe::run_probe;

pub fn collect_timeline(
    scene: &ProbeSceneSpec,
    phase: ProbePhase,
    frames: usize,
    base_request: &ProbeRequest,
) -> Result<ProbeTimelineReport, ProbeError> {
    if frames == 0 {
        return Err(ProbeError::InvalidRequest(
            "frames must be at least 1".to_string(),
        ));
    }

    let sample_ts = if frames == 1 {
        vec![0.0]
    } else {
        (0..frames)
            .map(|index| index as f64 / (frames.saturating_sub(1) as f64))
            .collect()
    };

    let frames = sample_ts
        .into_iter()
        .map(|sample_t| {
            run_probe(
                scene,
                &ProbeRequest {
                    phase,
                    sample_t,
                    cells: base_request.cells,
                    with_causation: base_request.with_causation,
                },
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ProbeTimelineReport {
        schema_version: "0.1.0".to_string(),
        kind: "timeline".to_string(),
        source: ProbeReportSource {
            input_kind: "probe_scene_spec".to_string(),
        },
        phase,
        frame_count: frames.len(),
        frames,
    })
}

// <FILE>crates/tui-vfx-probe/src/orc_collect_timeline.rs</FILE> - <DESC>Collect multiple frame dumps across one phase</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
