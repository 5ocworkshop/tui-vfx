// <FILE>crates/tui-vfx-probe/src/lib.rs</FILE> - <DESC>Library root for tui-vfx-probe</DESC>
// <VERS>VERSION: 0.5.0</VERS>
// <WCTX>Initial probe-side diagnostics for border/text integrity issues</WCTX>
// <CLOG>MINOR: Export typed diagnostics helpers so probe consumers can detect common border-row and underline-placement defects without ad hoc SQL</CLOG>

//! Engine-owned structured pipeline observability for `tui-vfx`.
//!
//! `tui-vfx-probe` exists so callers can inspect rendered output as typed data rather
//! than human-oriented prose. The phase-1 slice is intentionally narrow:
//!
//! - direct engine input via [`ProbeSceneSpec`]
//! - one-frame execution via [`run_probe`]
//! - JSON-friendly report output via [`ProbeReport`]
//! - selectors for `all`, `non-empty`, and `modified` cells
//! - compositor-stage `last_touch` attribution
//! - timeline and frame-diff helpers built on repeated frame dumps
//! - optional in-memory SQLite indexing/query support for large playback datasets
//!
//! This crate does **not** depend on `tui-vfx-recipes`; recipe-side tooling is expected
//! to adapt into `ProbeSceneSpec` later.
//!
//! # Current limitations
//!
//! The probe now supports multi-frame timeline and diff reports, but it still does not
//! provide style/content-stage hooks or full engine-wide causation coverage outside the
//! compositor callbacks currently exposed by `CompositorInspector`.
//!
//! # See also
//!
//! - `docs/PIPELINE_PROBE_LLM_GUIDE.md`
//! - `docs/design/pipeline-probe-design.md`

mod cls_probe_cell;
mod cls_probe_color;
mod cls_probe_diagnostic;
mod cls_probe_diff_cell;
mod cls_probe_diff_report;
mod cls_probe_error;
mod cls_probe_grid_spec;
mod cls_probe_inspector;
mod cls_probe_last_touch;
mod cls_probe_pipeline_inventory;
mod cls_probe_report;
mod cls_probe_request;
mod cls_probe_scene_spec;
mod cls_probe_sqlite_store;
mod cls_probe_state_snapshot;
mod cls_probe_summary;
mod cls_probe_timeline_report;
mod cls_probe_timing;
mod cls_probe_trace_event;
mod cls_probe_widget;
mod fnc_build_owned_grid;
mod fnc_collect_basic_diagnostics;
mod fnc_diff_frames;
mod fnc_has_ascii_alpha;
mod fnc_max_widget_y;
mod fnc_modifier_names;
mod fnc_normalize_color;
mod fnc_row_text;
mod fnc_select_cells;
mod orc_collect_timeline;
mod orc_run_probe;

pub use cls_probe_cell::ProbeCell;
pub use cls_probe_color::ProbeColor;
pub use cls_probe_diagnostic::{ProbeDiagnostic, ProbeDiagnosticSeverity};
pub use cls_probe_diff_cell::ProbeDiffCell;
pub use cls_probe_diff_report::ProbeDiffReport;
pub use cls_probe_error::ProbeError;
pub use cls_probe_grid_spec::ProbeGridSpec;
pub use cls_probe_last_touch::ProbeLastTouch;
pub use cls_probe_pipeline_inventory::ProbePipelineInventory;
pub use cls_probe_report::{ProbeFrame, ProbePoint, ProbeReport, ProbeReportSource, ProbeSize};
pub use cls_probe_request::{ProbeCellSelector, ProbePhase, ProbeRequest};
pub use cls_probe_scene_spec::ProbeSceneSpec;
pub use cls_probe_sqlite_store::ProbeSqliteStore;
pub use cls_probe_state_snapshot::ProbeStateSnapshot;
pub use cls_probe_summary::ProbeSummary;
pub use cls_probe_timeline_report::ProbeTimelineReport;
pub use cls_probe_timing::ProbeTiming;
pub use cls_probe_trace_event::ProbeTraceEvent;
pub use cls_probe_widget::ProbeWidget;
pub use fnc_collect_basic_diagnostics::collect_basic_diagnostics;
pub use fnc_diff_frames::diff_frames;
pub use fnc_has_ascii_alpha::has_ascii_alpha;
pub use fnc_max_widget_y::max_widget_y;
pub use orc_collect_timeline::collect_timeline;
pub use orc_run_probe::{run_probe, run_probe_diff};
pub use fnc_row_text::row_text;

// <FILE>crates/tui-vfx-probe/src/lib.rs</FILE> - <DESC>Library root for tui-vfx-probe</DESC>
// <VERS>END OF VERSION: 0.5.0</VERS>
