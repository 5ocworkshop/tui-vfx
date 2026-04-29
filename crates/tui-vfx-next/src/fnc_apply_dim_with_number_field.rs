// <FILE>crates/tui-vfx-next/src/fnc_apply_dim_with_number_field.rs</FILE> - <DESC>Apply dim proof effect from a spatial number field</DESC>
// <VERS>VERSION: 0.1.0</VERS>
// <WCTX>New kernel Phase G4: prove cell-field values can drive cell-varying inputs.</WCTX>
// <CLOG>0.1.0: INIT — add destination-local coordinate-aware dim application.</CLOG>

use crate::{
    ApplyOutcome, CellWritePolicy, CoordinateSpace, NumberCellField, RoleSpace, RoleWritePolicy,
    ScopeEvalInput, ScopeSpec, Surface, SurfaceDiagnostic,
};

/// Apply a dim factor sampled from a destination-local numeric cell field.
pub(crate) fn apply_dim_with_number_field(
    source: &Surface,
    destination: &mut Surface,
    scope: &ScopeSpec,
    field: &NumberCellField,
    cell_policy: CellWritePolicy,
    role_policy: RoleWritePolicy,
) -> ApplyOutcome {
    let mut outcome = ApplyOutcome::default();
    if source.width() != destination.width() || source.height() != destination.height() {
        outcome
            .diagnostics
            .push(SurfaceDiagnostic::surface_size_mismatch(
                (source.width(), source.height()),
                (destination.width(), destination.height()),
            ));
        return outcome;
    }
    if field.width() != destination.width() || field.height() != destination.height() {
        outcome
            .diagnostics
            .push(SurfaceDiagnostic::surface_size_mismatch(
                (field.width(), field.height()),
                (destination.width(), destination.height()),
            ));
        return outcome;
    }

    for y in 0..destination.height() {
        for x in 0..destination.width() {
            let role = destination.role(x, y).expect("in-bounds role").clone();
            let input = ScopeEvalInput {
                destination_x: x,
                destination_y: y,
                sampled_source_x: x,
                sampled_source_y: y,
                sampled_source_role: role.clone(),
                destination_role: role,
            };
            if !scope.matches(&input, CoordinateSpace::default(), RoleSpace::default()) {
                continue;
            }
            outcome.matched_cells += 1;
            let factor = field.sample(x, y).expect("field dimensions prechecked") as f32;
            let mut cell = *source.cell(x, y).expect("in-bounds source cell");
            cell.fg = cell.fg.dim(factor);
            cell.bg = cell.bg.dim(factor);
            if cell_policy == CellWritePolicy::SkipTransparentEmpty && cell.is_empty() {
                continue;
            }
            destination.set_cell(x, y, cell);
            if let RoleWritePolicy::SetExplicit { role } = &role_policy {
                destination.set_role(x, y, role.clone());
            }
            outcome.written_cells += 1;
        }
    }
    if outcome.matched_cells == 0 {
        outcome
            .diagnostics
            .push(SurfaceDiagnostic::zero_cell_scope(&format!("{scope:?}")));
    }
    outcome
}

// <FILE>crates/tui-vfx-next/src/fnc_apply_dim_with_number_field.rs</FILE> - <DESC>Apply dim proof effect from a spatial number field</DESC>
// <VERS>END OF VERSION: 0.1.0</VERS>
