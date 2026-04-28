// <FILE>crates/tui-vfx-next/src/cls_surface_engine.rs</FILE> - <DESC>Minimal surface engine entry points</DESC>
// <VERS>VERSION: 0.3.0</VERS>
// <WCTX>New kernel Phase C preflight OFPF split.</WCTX>
// <CLOG>0.3.0: REFACTOR — keep public engine entry points while moving the long sampled apply routine into a fnc_ file.</CLOG>

use tui_vfx_types::{Cell, RoleTag};

use crate::{
    ApplyOutcome, CellWrite, CoordinateSampler, CoordinateSpace, DimEffect,
    ExplicitRoleWriteEffect, IdentitySampler, RoleSpace, ScopeSpec, Surface,
};

/// Minimal Phase A/B engine.
pub struct SurfaceEngine;

impl SurfaceEngine {
    /// Copy source cells to destination under a scope, preserving sampled-source roles.
    pub fn copy(source: &Surface, destination: &mut Surface, scope: &ScopeSpec) -> ApplyOutcome {
        Self::copy_with_sampler(source, destination, scope, &IdentitySampler)
    }

    /// Copy source cells to destination through a coordinate sampler.
    pub fn copy_with_sampler(
        source: &Surface,
        destination: &mut Surface,
        scope: &ScopeSpec,
        sampler: &impl CoordinateSampler,
    ) -> ApplyOutcome {
        Self::apply_from_source_with_sampler(
            source,
            destination,
            scope,
            CoordinateSpace::default(),
            RoleSpace::default(),
            sampler,
            |cell, _role| CellWrite::copy_sampled_source(cell),
        )
    }

    /// Apply a visual-only dim effect. Cell visual channels change and roles are preserved.
    pub fn apply_dim(
        source: &Surface,
        destination: &mut Surface,
        scope: &ScopeSpec,
        effect: DimEffect,
    ) -> ApplyOutcome {
        Self::apply_dim_with_sampler(source, destination, scope, effect, &IdentitySampler)
    }

    /// Apply a visual-only dim effect through a coordinate sampler.
    pub fn apply_dim_with_sampler(
        source: &Surface,
        destination: &mut Surface,
        scope: &ScopeSpec,
        effect: DimEffect,
        sampler: &impl CoordinateSampler,
    ) -> ApplyOutcome {
        Self::apply_from_source_with_sampler(
            source,
            destination,
            scope,
            CoordinateSpace::default(),
            RoleSpace::default(),
            sampler,
            |cell, _role| effect.write(&cell),
        )
    }

    /// Apply a procedural explicit-role writer without sampling source cells.
    pub fn apply_explicit_role_write(
        destination: &mut Surface,
        scope: &ScopeSpec,
        effect: &ExplicitRoleWriteEffect,
    ) -> ApplyOutcome {
        let snapshot = destination.clone();
        Self::apply_from_source_with_sampler(
            &snapshot,
            destination,
            scope,
            CoordinateSpace::default(),
            RoleSpace::Destination,
            &IdentitySampler,
            |_cell, _role| effect.write(),
        )
    }

    /// Apply a source-sampling operation with identity sampling and explicit spaces.
    pub fn apply_from_source<F>(
        source: &Surface,
        destination: &mut Surface,
        scope: &ScopeSpec,
        coordinate_space: CoordinateSpace,
        role_space: RoleSpace,
        write_fn: F,
    ) -> ApplyOutcome
    where
        F: FnMut(Cell, RoleTag) -> CellWrite,
    {
        Self::apply_from_source_with_sampler(
            source,
            destination,
            scope,
            coordinate_space,
            role_space,
            &IdentitySampler,
            write_fn,
        )
    }
}

// <FILE>crates/tui-vfx-next/src/cls_surface_engine.rs</FILE> - <DESC>Minimal surface engine entry points</DESC>
// <VERS>END OF VERSION: 0.3.0</VERS>
