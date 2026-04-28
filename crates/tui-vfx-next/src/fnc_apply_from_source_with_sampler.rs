// <FILE>crates/tui-vfx-next/src/fnc_apply_from_source_with_sampler.rs</FILE> - <DESC>Sampler-aware source-to-destination apply routine</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase C preflight OFPF split.</WCTX>
// <CLOG>0.3.0: REFACTOR — extract the long sampled apply routine from cls_surface_engine.rs to stay below OFPF file-size limits.</CLOG>

use tui_vfx_types::{Cell, RoleTag};

use crate::{
    ApplyOutcome, CellWrite, CoordinateSampler, CoordinateSpace, RoleSpace, RoleWritePolicy,
    ScopeEvalInput, ScopeSpec, Surface, SurfaceDiagnostic, SurfaceEngine,
};

impl SurfaceEngine {
    /// Apply a source-sampling operation with an explicit sampler and spaces.
    pub fn apply_from_source_with_sampler<F>(
        source: &Surface,
        destination: &mut Surface,
        scope: &ScopeSpec,
        coordinate_space: CoordinateSpace,
        role_space: RoleSpace,
        sampler: &impl CoordinateSampler,
        mut write_fn: F,
    ) -> ApplyOutcome
    where
        F: FnMut(Cell, RoleTag) -> CellWrite,
    {
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

        let mut pending = Vec::new();
        for y in 0..destination.height() {
            for x in 0..destination.width() {
                let Some((source_x, source_y)) =
                    sampler.sample(x, y, source.width(), source.height())
                else {
                    continue;
                };

                let sampled_source_role = source
                    .role(source_x, source_y)
                    .expect("sampler returned in-bounds source role")
                    .clone();
                let destination_role = destination
                    .role(x, y)
                    .expect("in-bounds destination role")
                    .clone();
                let input = ScopeEvalInput {
                    destination_x: x,
                    destination_y: y,
                    sampled_source_x: source_x,
                    sampled_source_y: source_y,
                    sampled_source_role: sampled_source_role.clone(),
                    destination_role,
                };
                if scope.matches(&input, coordinate_space, role_space) {
                    let cell = *source
                        .cell(source_x, source_y)
                        .expect("sampler returned in-bounds source cell");
                    pending.push((x, y, cell, sampled_source_role));
                }
            }
        }

        outcome.matched_cells = pending.len();
        if pending.is_empty() {
            outcome
                .diagnostics
                .push(SurfaceDiagnostic::zero_cell_scope(&format!("{scope:?}")));
            return outcome;
        }

        for (x, y, cell, sampled_source_role) in pending {
            let write = write_fn(cell, sampled_source_role.clone());
            if write.is_skipped() {
                continue;
            }

            destination.set_cell(x, y, write.cell);
            match write.role_policy {
                RoleWritePolicy::PreserveDestination => {}
                RoleWritePolicy::CopySampledSource => {
                    destination.set_role(x, y, sampled_source_role)
                }
                RoleWritePolicy::SetExplicit { role } => destination.set_role(x, y, role),
            }
            outcome.written_cells += 1;
        }

        outcome
    }
}

// <FILE>crates/tui-vfx-next/src/fnc_apply_from_source_with_sampler.rs</FILE> - <DESC>Sampler-aware source-to-destination apply routine</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
