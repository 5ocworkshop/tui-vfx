// <FILE>crates/tui-vfx-probe/src/fnc_build_owned_grid.rs</FILE> - <DESC>Build an OwnedGrid from a ProbeGridSpec</DESC>
// <VERS>VERSION: 0.2.0</VERS>
// <WCTX>Phase-1 pipeline probe implementation</WCTX>
// <CLOG>MINOR: Validate row-major cell counts and materialize ProbeGridSpec into an OwnedGrid for probe execution</CLOG>

use tui_vfx_types::OwnedGrid;

use crate::cls_probe_error::ProbeError;
use crate::cls_probe_grid_spec::ProbeGridSpec;

pub fn build_owned_grid(grid_spec: &ProbeGridSpec) -> Result<OwnedGrid, ProbeError> {
    let expected_len = grid_spec.width as usize * grid_spec.height as usize;
    if grid_spec.cells.len() != expected_len {
        return Err(ProbeError::InvalidScene(format!(
            "grid cell count mismatch: expected {expected_len}, got {}",
            grid_spec.cells.len()
        )));
    }

    Ok(OwnedGrid::from_cells(
        grid_spec.width as usize,
        grid_spec.height as usize,
        grid_spec.cells.clone(),
    ))
}

// <FILE>crates/tui-vfx-probe/src/fnc_build_owned_grid.rs</FILE> - <DESC>Build an OwnedGrid from a ProbeGridSpec</DESC>
// <VERS>END OF VERSION: 0.2.0</VERS>
