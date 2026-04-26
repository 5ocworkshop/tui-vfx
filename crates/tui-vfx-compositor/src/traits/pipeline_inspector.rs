// <FILE>tui-vfx-compositor/src/traits/pipeline_inspector.rs</FILE> - <DESC>Trait for inspecting pipeline stage operations</DESC>
// <VERS>VERSION: 2.2.0</VERS>
// <WCTX>Pipeline observability Unit A — extend with four per-stage callbacks (on_stage_entered/on_stage_finished/on_stage_skipped/on_scope_evaluated) so the focused_row_btop scope-mismatch class produces evidence on the event stream. Default empty bodies keep every existing impl source-compatible.</WCTX>
// <CLOG>2.2.0: add on_stage_entered / on_stage_finished / on_stage_skipped / on_scope_evaluated callbacks with default empty bodies; existing per-cell callbacks unchanged.</CLOG>

use tui_vfx_debug::inspection::{
    PipelineSkipReason, PipelineStageKind, RoleHistogram, RoleMapSource,
};
use tui_vfx_types::{Cell, Style};

/// Trait for inspecting the render pipeline at cell-level operations.
///
/// All methods have default no-op implementations, allowing inspectors
/// to selectively implement only the hooks they care about.
///
/// The per-cell pipeline flow is:
/// 1. Sampler transforms coordinates → `on_sampler_applied`
/// 2. Mask checks visibility → `on_mask_checked`
/// 3. Shader applies style → `on_shader_applied`
/// 4. Filter modifies cell → `on_filter_applied`
/// 5. Shadow-region cell prepared → `on_shadow_cell_applied`
/// 6. Cell written to buffer → `on_cell_rendered`
///
/// Per-stage flow (added in 2.2.0):
/// - `on_stage_entered` once before each stage runs (kind + step_id + name + scope_summary)
/// - `on_scope_evaluated` once per stage application (matched/skipped counts + role histogram)
/// - `on_stage_finished` once after the stage runs (cells_modified + elapsed_ns)
/// - `on_stage_skipped` instead of entered/finished when the stage skipped iteration
///   (e.g. scope predicate matched zero cells)
pub trait CompositorInspector {
    /// Called after sampler transforms coordinates for a cell.
    ///
    /// # Arguments
    /// * `dest_x`, `dest_y` - Destination cell position (local to widget area)
    /// * `src_x`, `src_y` - Source coordinates after transform (None if skipped/gap)
    /// * `sampler_name` - Name of the sampler (e.g., "SineWave", "Ripple")
    fn on_sampler_applied(
        &mut self,
        _dest_x: u16,
        _dest_y: u16,
        _src_x: Option<u16>,
        _src_y: Option<u16>,
        _sampler_name: &str,
    ) {
    }

    /// Called after mask visibility is checked for a cell.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget area)
    /// * `visible` - Whether the cell passed the mask check
    /// * `mask_name` - Name of the mask (e.g., "Wipe", "Dissolve")
    fn on_mask_checked(&mut self, _x: u16, _y: u16, _visible: bool, _mask_name: &str) {}

    /// Called when a shader is applied to a specific cell.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget area)
    /// * `before` - Style before shader application
    /// * `after` - Style after shader application
    /// * `shader_name` - Name of the shader (e.g., "PulseWave", "BorderSweep")
    fn on_shader_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: Style,
        _after: Style,
        _shader_name: &str,
    ) {
    }

    /// Called after a filter is applied to a cell.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget area)
    /// * `before` - Cell state before filter application
    /// * `after` - Cell state after filter application
    /// * `filter_name` - Name of the filter (e.g., "Tint", "Dim")
    fn on_filter_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _before: &Cell,
        _after: &Cell,
        _filter_name: &str,
    ) {
    }

    /// Called after a shadow-region cell has been prepared in the shadow branch.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget/render area)
    /// * `shadow_cell` - Shadow-region cell before final destination blending
    /// * `source_empty` - Whether the corresponding source cell is empty/missing
    fn on_shadow_cell_applied(
        &mut self,
        _x: u16,
        _y: u16,
        _shadow_cell: &Cell,
        _source_empty: bool,
    ) {
    }

    /// Called after all effects have been applied to a cell.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget area)
    /// * `final_cell` - The fully rendered cell with all effects
    fn on_cell_rendered(&mut self, _x: u16, _y: u16, _final_cell: &Cell) {}

    /// Called once before a pipeline stage starts iterating cells.
    ///
    /// Paired with [`Self::on_stage_finished`] (or replaced by
    /// [`Self::on_stage_skipped`] when the stage skipped iteration).
    ///
    /// # Arguments
    /// * `kind` - Which kind of stage just started.
    /// * `step_id` - 1-based per-render stage counter; the join key for inspector queries.
    /// * `name` - Stage display name (e.g. `"FocusedRowGradient"`, `"FadeIn"`).
    /// * `scope_summary` - Closed-vocabulary summary of the stage's effective scope predicate
    ///   (e.g. `"Role(Text)"`, `"And(RowRange,Channel(Background))"`).
    ///   Empty string when the stage has no explicit scope (whole-area).
    fn on_stage_entered(
        &mut self,
        _kind: PipelineStageKind,
        _step_id: u32,
        _name: &str,
        _scope_summary: &str,
    ) {
    }

    /// Called once after a pipeline stage finishes iterating cells.
    ///
    /// Pairs with the matching [`Self::on_stage_entered`] (same `kind` + `step_id`).
    ///
    /// # Arguments
    /// * `kind` - Which kind of stage just finished.
    /// * `step_id` - 1-based per-render stage counter (matches the paired `on_stage_entered`).
    /// * `cells_modified` - Number of cells the stage modified.
    /// * `elapsed_ns` - Wall-clock duration of the stage application, in nanoseconds.
    fn on_stage_finished(
        &mut self,
        _kind: PipelineStageKind,
        _step_id: u32,
        _cells_modified: u32,
        _elapsed_ns: u64,
    ) {
    }

    /// Called once when a pipeline stage was skipped without iterating cells.
    ///
    /// Replaces the [`Self::on_stage_entered`] / [`Self::on_stage_finished`]
    /// pair when the stage's scope matched zero cells, the stage's area
    /// was empty, the stage was disabled by policy, or the stage exceeded
    /// its budget. The `reason` discriminator carries the data needed to
    /// reproduce the decision.
    ///
    /// # Arguments
    /// * `kind` - Which kind of stage was skipped.
    /// * `step_id` - 1-based per-render stage counter.
    /// * `reason` - Why the stage was skipped.
    fn on_stage_skipped(
        &mut self,
        _kind: PipelineStageKind,
        _step_id: u32,
        _reason: PipelineSkipReason,
    ) {
    }

    /// Called once per stage application with the scope predicate evaluation summary.
    ///
    /// Sum of `matched + skipped` equals the number of cells in the stage's
    /// effective area. The histogram shows what the predicate saw — load-bearing
    /// for diagnosing role-map mismatches.
    ///
    /// # Arguments
    /// * `step_id` - 1-based per-render stage counter (matches the surrounding
    ///   `on_stage_entered` / `on_stage_finished`).
    /// * `matched` - Number of cells the predicate matched.
    /// * `skipped` - Number of cells the predicate skipped.
    /// * `role_histogram` - Per-role cell counts the predicate visited.
    fn on_scope_evaluated(
        &mut self,
        _step_id: u32,
        _matched: u32,
        _skipped: u32,
        _role_histogram: RoleHistogram,
    ) {
    }

    /// Called once per render at the moment the role map becomes available.
    ///
    /// The `source` discriminator names where the role map came from
    /// (geometric inference vs explicit producer-tagged vs externally
    /// injected). Diagnosing role-map mismatches without source archaeology
    /// (the focused_row_btop bug class) depends on this event firing with
    /// an honest source for both production and any introspection re-render.
    ///
    /// # Arguments
    /// * `source` - Where the role map was materialized.
    /// * `histogram` - Per-role cell counts in the materialized map.
    fn on_role_map_materialized(&mut self, _source: RoleMapSource, _histogram: RoleHistogram) {}
}

// <FILE>tui-vfx-compositor/src/traits/pipeline_inspector.rs</FILE> - <DESC>Trait for inspecting pipeline stage operations</DESC>
// <VERS>END OF VERSION: 2.2.0</VERS>
