// <FILE>tui-vfx-compositor/src/traits/pipeline_inspector.rs</FILE> - <DESC>Trait for inspecting pipeline stage operations</DESC>
// <VERS>VERSION: 2.0.0</VERS>
// <WCTX>L2/L3 abstraction: make compositor framework-agnostic</WCTX>
// <CLOG>Changed Cell and Style types from ratatui to mixed_types</CLOG>

use tui_vfx_types::{Cell, Style};

/// Trait for inspecting the render pipeline at cell-level operations.
///
/// All methods have default no-op implementations, allowing inspectors
/// to selectively implement only the hooks they care about.
///
/// The per-cell pipeline flow is:
/// 1. Sampler transforms coordinates → on_sampler_applied
/// 2. Mask checks visibility → on_mask_checked
/// 3. Shader applies style → on_shader_applied
/// 4. Filter modifies cell → on_filter_applied
/// 5. Cell written to buffer → on_cell_rendered
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

    /// Called after all effects have been applied to a cell.
    ///
    /// # Arguments
    /// * `x`, `y` - Cell position (local to widget area)
    /// * `final_cell` - The fully rendered cell with all effects
    fn on_cell_rendered(&mut self, _x: u16, _y: u16, _final_cell: &Cell) {}
}

// <FILE>tui-vfx-compositor/src/traits/pipeline_inspector.rs</FILE> - <DESC>Trait for inspecting pipeline stage operations</DESC>
// <VERS>END OF VERSION: 2.0.0</VERS>
