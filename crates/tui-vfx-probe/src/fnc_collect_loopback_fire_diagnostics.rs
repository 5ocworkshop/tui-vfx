// <FILE>crates/tui-vfx-probe/src/fnc_collect_loopback_fire_diagnostics.rs</FILE> - <DESC>Emit one Warning ProbeDiagnostic per binding key that fell back to recipe-author-declared loopback during the probed frame</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>Loopback Phase L5 (deferred follow-on): the L3 visibility badge is the human-facing surface for loopback fires; this is the machine-facing surface — one structured Warning per key so probe consumers (CI gates, recipe-browser status pills, batch validation) can treat fires as actionable findings without scanning the rendered grid for badge cells.</WCTX>
// <CLOG>Introduce collect_loopback_fire_diagnostics(report) -> Vec<ProbeDiagnostic> reading report.runtime.loopback_fired_keys. One Warning per key, code = "loopback_fire", message names the binding so consumers can render per-key context. Empty input → empty output (no false fires when host supplied every binding).</CLOG>

//! Per-frame loopback-fire diagnostics for probe reports.

use crate::ProbeReport;
use crate::cls_probe_diagnostic::{ProbeDiagnostic, ProbeDiagnosticSeverity};

/// Emit one Warning [`ProbeDiagnostic`] per binding key that fell back
/// to recipe-author-declared loopback during the probed frame.
///
/// Reads `report.runtime.loopback_fired_keys`. Returns an empty vec
/// when no loopback fired (host supplied every binding, or the recipe
/// declared no `requires_bindings`, or no runtime context was attached
/// to the probe).
///
/// Each diagnostic carries `code: "loopback_fire"` and a message
/// naming the offending binding. Order matches the input vec
/// (BTreeMap-stable ordering set by the L1 merge layer).
pub fn collect_loopback_fire_diagnostics(report: &ProbeReport) -> Vec<ProbeDiagnostic> {
    let Some(runtime) = report.runtime.as_ref() else {
        return Vec::new();
    };
    runtime
        .loopback_fired_keys
        .iter()
        .map(|key| ProbeDiagnostic {
            code: "loopback_fire".to_string(),
            severity: ProbeDiagnosticSeverity::Warning,
            message: format!(
                "binding `{key}` fell back to its recipe-author-declared loopback this frame; the host did not supply a value"
            ),
            widget_y: None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cls_probe_pipeline_inventory::ProbePipelineInventory;
    use crate::cls_probe_report::{
        ProbeFrame, ProbePoint, ProbeReportSource, ProbeSize,
    };
    use crate::cls_probe_request::{ProbeCellSelector, ProbePhase, ProbeRequest};
    use crate::cls_probe_runtime_context::ProbeRuntimeContext;
    use crate::cls_probe_summary::ProbeSummary;
    use crate::cls_probe_timing::ProbeTiming;
    use crate::cls_probe_widget::ProbeWidget;

    fn report_with_runtime(runtime: Option<ProbeRuntimeContext>) -> ProbeReport {
        ProbeReport {
            schema_version: "0.1.0".to_string(),
            kind: "frame_dump".to_string(),
            source: ProbeReportSource {
                input_kind: "test".to_string(),
            },
            request: ProbeRequest {
                phase: ProbePhase::Dwell,
                sample_t: 1.0,
                cells: ProbeCellSelector::All,
                emit_trace: false,
            },
            timing: ProbeTiming {
                requested_phase: ProbePhase::Dwell,
                requested_t: 1.0,
                effective_phase: ProbePhase::Dwell,
                effective_t: 1.0,
                tick_ms: None,
            },
            frame: ProbeFrame {
                size: ProbeSize { width: 10, height: 1 },
            },
            widget: ProbeWidget {
                abs_origin: ProbePoint { x: 0, y: 0 },
                size: ProbeSize { width: 10, height: 1 },
            },
            pipeline: ProbePipelineInventory::default(),
            runtime,
            summary: ProbeSummary {
                total_cells: 10,
                non_empty_cells: 0,
                modified_cells: 0,
            },
            diagnostics: Vec::new(),
            cells: Vec::new(),
        }
    }

    #[test]
    fn no_runtime_context_emits_nothing() {
        let report = report_with_runtime(None);
        assert!(collect_loopback_fire_diagnostics(&report).is_empty());
    }

    #[test]
    fn empty_fired_keys_emits_nothing() {
        let report = report_with_runtime(Some(ProbeRuntimeContext {
            supplied_params: Vec::new(),
            binding_requests: Vec::new(),
            binding_resolutions: Vec::new(),
            loopback_fired_keys: Vec::new(),
        }));
        assert!(collect_loopback_fire_diagnostics(&report).is_empty());
    }

    #[test]
    fn one_key_emits_one_warning_with_code_loopback_fire() {
        let report = report_with_runtime(Some(ProbeRuntimeContext {
            supplied_params: Vec::new(),
            binding_requests: Vec::new(),
            binding_resolutions: Vec::new(),
            loopback_fired_keys: vec!["demo_progress".to_string()],
        }));
        let diagnostics = collect_loopback_fire_diagnostics(&report);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code, "loopback_fire");
        assert_eq!(diagnostics[0].severity, ProbeDiagnosticSeverity::Warning);
        assert!(diagnostics[0].message.contains("demo_progress"));
        assert!(diagnostics[0].widget_y.is_none());
    }

    #[test]
    fn multiple_keys_emit_one_warning_each_in_input_order() {
        let report = report_with_runtime(Some(ProbeRuntimeContext {
            supplied_params: Vec::new(),
            binding_requests: Vec::new(),
            binding_resolutions: Vec::new(),
            loopback_fired_keys: vec![
                "alpha".to_string(),
                "beta".to_string(),
                "gamma".to_string(),
            ],
        }));
        let diagnostics = collect_loopback_fire_diagnostics(&report);
        assert_eq!(diagnostics.len(), 3);
        assert!(diagnostics[0].message.contains("alpha"));
        assert!(diagnostics[1].message.contains("beta"));
        assert!(diagnostics[2].message.contains("gamma"));
        for d in &diagnostics {
            assert_eq!(d.code, "loopback_fire");
            assert_eq!(d.severity, ProbeDiagnosticSeverity::Warning);
        }
    }
}

// <FILE>crates/tui-vfx-probe/src/fnc_collect_loopback_fire_diagnostics.rs</FILE> - <DESC>collect_loopback_fire_diagnostics fn</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
