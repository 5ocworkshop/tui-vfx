<!-- <FILE>docs/new_kernel/PHASE_B_STATUS.md</FILE> - <DESC>Phase B sampled-source semantics status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Concise status for Phase B sampled-source semantics proof.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

# Phase B sampled-source semantics status

Status: implemented as a narrow `tui-vfx-next` contract proof.

Decisions:

- Sampling is represented by a minimal `CoordinateSampler` trait.
- `IdentitySampler` preserves Phase A identity behavior.
- `ShiftSampler { dx, dy }` maps destination `(x, y)` to source `(x + dx, y + dy)`; positive offsets sample right/down from the source relative to the destination.
- Out-of-bounds samples return `None` and skip the destination write, preserving destination cell and role.
- Scope matching and zero-cell diagnostics use the same sampled-source path as writes.
- Geometry scopes still default to destination-local coordinates.
- Role scopes still default to sampled-source roles, with explicit `RoleSpace::Destination` available.

Not done in Phase B: descriptor registry, recipe compiler, runtime/phase graph, legacy migration, or real effect ports.

<!-- <FILE>docs/new_kernel/PHASE_B_STATUS.md</FILE> - <DESC>Phase B sampled-source semantics status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
