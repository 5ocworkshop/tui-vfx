<!-- <FILE>docs/new_kernel/PHASE_B_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase B status memo to the surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Memo summarizing Phase B sampled-source semantics proof.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

# Phase B Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`

## Executive summary

Phase B has reached the proof point recommended in `ARCH-RESP-TO-PHASE_A.md`:

> Can the clean-room kernel preserve correct cell/role/scope/write semantics when source sampling is non-identity?

Current answer: **yes, for the bounded Phase B contract spike.**

The clean-room crate, `tui-vfx-next`, now includes a minimal sampled-source pipeline. It can map each destination coordinate to a sampled source coordinate, copy sampled cells, copy sampled-source roles, evaluate role scopes against sampled-source roles by default, evaluate geometry scopes against destination-local coordinates by default, skip out-of-bounds samples without mutating destination state, and keep transparent empty writes distinct from skipped sampled cells.

The implementation deliberately remains a semantic proof, not a recipe, descriptor, runtime, migration, or real effect-porting milestone.

## Current implementation state

### Existing clean-room crate extended

Crate:

```text
crates/tui-vfx-next
```

Phase B-relevant files:

```text
crates/tui-vfx-next/src/lib.rs
crates/tui-vfx-next/src/sampler.rs
crates/tui-vfx-next/src/engine.rs
crates/tui-vfx-next/src/scope.rs
crates/tui-vfx-next/src/write.rs
crates/tui-vfx-next/tests/surface_contract.rs
```

Workspace wiring remains:

```text
Cargo.toml
Cargo.lock
```

### Added sampled-source semantics

New sampling surface:

```text
CoordinateSampler
IdentitySampler
ShiftSampler { dx, dy }
```

New sampled operation entry points:

```text
SurfaceEngine::copy_with_sampler
SurfaceEngine::apply_dim_with_sampler
SurfaceEngine::apply_from_source_with_sampler
```

`ShiftSampler { dx, dy }` maps:

```text
destination (x, y) -> source (x + dx, y + dy)
```

Positive `dx` samples from the source cell to the right of the destination coordinate. Positive `dy` samples from the source cell below the destination coordinate. Coordinates outside the source surface return `None` and skip the destination write.

### Updated contract/status documents

```text
docs/v3.1-surface-contract.md
docs/new_kernel/PHASE_B_STATUS.md
```

The surface contract now documents Phase B sampling behavior and includes the Phase A architecture corrections:

- `Custom(name)` is documented as a custom role, not a first-class built-in role.
- `role` is documented as a semantic channel for a surface position, not a `Cell` field.
- Visual-only role preservation is clarified by operation shape: in-place visual effects preserve destination roles; source-to-destination copy/transform writes sampled-source roles first, then visual effects preserve the resulting role.

## Goal-by-goal status against the Phase B recommendation

| Phase B goal / constraint | Current status |
|---|---|
| Add minimal non-identity sampling path | **Done.** `CoordinateSampler` plus `ShiftSampler` prove destination coordinate can differ from sampled source coordinate. |
| Preserve Phase A identity behavior | **Done.** `IdentitySampler` backs the existing `copy`, `apply_dim`, and identity `apply_from_source` paths. |
| Prove shifted sampler copies sampled source cells | **Done.** `shift_sampler_copies_sampled_source_cell` verifies destination cells receive cells from shifted source coordinates. |
| Copied cells receive sampled-source roles | **Done.** `shift_sampler_copies_sampled_source_role` verifies roles come from the sampled source coordinate, not the destination coordinate. |
| Role scopes match sampled source roles by default | **Done.** `role_scope_uses_sampled_source_role_with_shift` verifies default role-space behavior under non-identity sampling. |
| Geometry scopes match destination-local coordinates by default | **Done.** `geometry_scope_uses_destination_local_with_shift` verifies geometry scope is evaluated against destination coordinates, not sampled coordinates. |
| Out-of-bounds samples skip writes and preserve destination | **Done.** `out_of_bounds_sample_preserves_destination` verifies skipped samples leave destination cell and role unchanged. |
| Zero-cell diagnostics use actual sampled-source semantics | **Done.** `zero_cell_scope_with_sampler_emits_diagnostic` verifies zero matches after sampling emit the structured diagnostic and do not mutate. |
| Destination role-space can be selected | **Done.** `destination_role_space_can_be_selected` proves `RoleSpace::Destination` behaves differently from sampled-source role space. |
| Transparent empty sampled writes are not skipped samples | **Done.** `transparent_empty_sample_write_is_not_skip` proves an empty transparent sampled cell is still written when policy says write. |
| Do not start recipes/descriptors/studio/runtime/phase graph | **Respected.** No descriptor registry, recipe compiler, studio manifest, runtime store, trigger engine, or phase graph was added. |
| Do not port real effects | **Respected.** No CRT/typewriter/matrix/shadow/etc. porting was attempted. |
| Do not replace old compositor | **Respected.** Legacy compositor/style/content/shadow implementation files were not modified for this phase. |
| Preserve clean-room dependency boundary | **Respected.** `tui-vfx-next` still has no dependency on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow`. |

## Required tests now present

The required Phase B tests from the architecture response are present in `crates/tui-vfx-next/tests/surface_contract.rs`:

```text
shift_sampler_copies_sampled_source_cell
shift_sampler_copies_sampled_source_role
role_scope_uses_sampled_source_role_with_shift
geometry_scope_uses_destination_local_with_shift
out_of_bounds_sample_preserves_destination
zero_cell_scope_with_sampler_emits_diagnostic
destination_role_space_can_be_selected
transparent_empty_sample_write_is_not_skip
```

The original Phase A tests remain present in the same test file:

```text
copy_preserves_sampled_source_roles
visual_effect_preserves_roles
role_scope_affects_only_matching_roles
skipped_cells_preserve_destination_cell_and_role
zero_cell_scope_emits_diagnostic
explicit_role_write_sets_role
empty_transparent_cell_is_not_the_same_as_skip
scope_role_space_defaults_to_sampled_source
```

Together, the package currently has 16 integration tests for the clean-room surface contract.

## Dependency and architecture status

Current dependency direction remains:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

`cargo tree -p tui-vfx-next` shows no dependency on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

No new third-party dependency was added for Phase B.

## Verification evidence

Post-implementation and post-deslop verification passed:

```text
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-next
cargo test --workspace
cargo tree -p tui-vfx-next
```

Package test result:

```text
16 passed; 0 failed
Doc-tests tui_vfx_next: 0 passed; 0 failed
```

Dependency guardrail check:

```text
grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next
```

Result: no matches.

An independent verifier agent reviewed the Phase B implementation and returned:

```text
APPROVED
```

The verifier found no missing acceptance criteria and no blocking findings.

## Notable implementation choices

1. **Minimal sampler trait instead of a broad sampling subsystem.**
   `CoordinateSampler` has one method: map destination coordinates to optional sampled source coordinates. This keeps Phase B focused on semantics rather than filter/sampler architecture.

2. **Shift direction is explicit.**
   The contract and implementation define `ShiftSampler { dx, dy }` as destination `(x, y)` sampling source `(x + dx, y + dy)`. This avoids ambiguity before more complex samplers are introduced.

3. **Out-of-bounds sampling is skip, not clamp or wrap.**
   A sampler returning `None` preserves destination cell and role. This creates a crisp distinction between skipped samples and transparent empty sampled writes.

4. **Scope evaluation uses both destination and sampled-source facts.**
   `ScopeEvalInput` carries destination coordinates, sampled-source coordinates, sampled-source role, and destination role. That lets the engine prove the intended default spaces and still allow explicit destination role-space selection.

5. **Geometry and role defaults remain deliberately different.**
   Geometry scopes default to destination-local coordinates. Role scopes default to sampled-source roles. The tests intentionally use shifted coordinates and differing roles so these defaults cannot pass accidentally through identity behavior.

6. **Zero-cell diagnostics are sampler-aware.**
   Pending writes are built only after sampling and scope evaluation. A zero-cell diagnostic therefore reflects the actual sampled-source semantics used by writes.

7. **No split yet.**
   The crate remains `tui-vfx-next`. Phase B did not split contract/engine crates because the goal was to deepen the proof, not settle final package topology.

## Scope-control status

The implementation did **not** attempt to:

```text
build full recipes
build descriptor registry
build studio manifest
build phase engine
build trigger engine
port CRT
port typewriter
port matrix rain
port shadow rendering
replace old compositor
migrate legacy filters
support legacy aliases
split tui-vfx-next into multiple crates
```

That matches the Phase B boundary recommended in the architecture response.

## Deslop / cleanup performed after verification

After verifier approval, a small bounded deslop pass was run. It only corrected stale Phase-A-only wording in `tui-vfx-next` crate/test documentation so the docs now accurately describe Phase A/B. No behavior or API was broadened during the cleanup.

All verification gates were rerun after that cleanup.

## Open questions / recommended next decisions

1. **Should Phase C start descriptor/schema work or one more semantic proof?**
   Phase B locked non-identity sampling. The next decision is whether to begin Rust-owned effect descriptor contracts, or first prove another semantic dimension such as layering, masks, or multi-stage pipeline composition.

2. **Sampler trait ownership and coordinate types.**
   Phase B uses `usize` coordinates to match `Surface` dimensions. Before stabilizing public contracts, decide whether final sampler APIs should use `usize`, `u16`, or a domain-specific coordinate type.

3. **Out-of-bounds observability.**
   Phase B treats out-of-bounds samples as skipped writes without a separate diagnostic/count. Decide whether later phases need sampled-skip telemetry distinct from zero-cell diagnostics.

4. **Role storage representation.**
   `Surface` still stores roles in a direct row-major vector. Before broader migration, decide whether to keep this representation, adapt `RoleMap`, or introduce a contract-owned role layer abstraction.

5. **Strict custom role declaration.**
   The docs distinguish first-class roles from `Custom(name)`, but no strict declaration/validation layer exists yet. That likely belongs with descriptor/schema/compiler work.

6. **Crate topology.**
   `tui-vfx-next` remains an incubator. Decide when, if ever, to split stable contract types from the engine proof.

7. **Effect descriptor maturity.**
   `EffectDescriptor` remains intentionally tiny. The larger descriptor model from `DRAFT_CONTRACTS.md` is still future work.

8. **Legacy integration boundary.**
   No old compositor migration path has been validated yet. That should remain deferred until the contract/model layer is stable enough to avoid importing legacy assumptions into the clean-room kernel.

## Bottom line

Phase B is complete as a sampled-source contract proof. The clean-room kernel now demonstrates that the semantic surface model still holds when destination coordinates and sampled source coordinates diverge. This closes the main semantic gap called out in the Phase A architecture response and keeps the project on the intended path: prove the contract model first, then decide the next bounded layer to add.

<!-- <FILE>docs/new_kernel/PHASE_B_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase B status memo to the surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
