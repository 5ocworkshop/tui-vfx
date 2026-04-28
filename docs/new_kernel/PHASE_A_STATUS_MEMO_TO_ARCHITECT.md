<!-- <FILE>docs/new_kernel/PHASE_A_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase A status memo to the surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Memo summarizing Phase A surface contract proof.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add OFPF metadata around captured clean-room kernel planning/status content.</CLOG> -->

# Phase A Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`

## Executive summary

Phase A has reached the intended proof point from `PROJECT_KICKOFF.md`:

> Can the project define and prove a clean semantic surface model independent of the old compositor?

Current answer: **yes, for the bounded Phase A contract spike.**

A new clean-room crate, `tui-vfx-next`, now exists beside the legacy engine. It defines a minimal dense semantic surface, scope evaluation, write policies, structured diagnostics, one visual-only effect, one explicit-role procedural writer, and the required contract tests. It does not import the legacy compositor/style/content/shadow implementation crates.

The implementation deliberately remains a semantic proof, not a visual/effect-porting milestone.

## Current implementation state

### Added workspace crate

New crate:

```text
crates/tui-vfx-next
```

Files:

```text
crates/tui-vfx-next/Cargo.toml
crates/tui-vfx-next/src/lib.rs
crates/tui-vfx-next/src/surface.rs
crates/tui-vfx-next/src/scope.rs
crates/tui-vfx-next/src/write.rs
crates/tui-vfx-next/src/effect.rs
crates/tui-vfx-next/src/engine.rs
crates/tui-vfx-next/src/diagnostic.rs
crates/tui-vfx-next/tests/surface_contract.rs
```

Workspace wiring:

```text
Cargo.toml
Cargo.lock
```

### Added contract document

```text
docs/v3.1-surface-contract.md
```

This document defines the Phase A surface/cell/role/scope/write/diagnostic contract and names the explicit non-goals.

## Goal-by-goal status against `PROJECT_KICKOFF.md`

| Kickoff goal / constraint | Current status |
|---|---|
| Define the v3.1 semantic surface contract | **Done.** `docs/v3.1-surface-contract.md` defines canonical surface, cell channels, role model, scope model, write policies, diagnostics, and Phase A boundaries. |
| Build a minimal clean-room kernel beside the existing engine | **Done.** `crates/tui-vfx-next` is a separate crate and does not replace existing pipeline code. |
| Prove the contract with tests | **Done.** `cargo test -p tui-vfx-next` passes 8 integration tests covering the required semantics. |
| Do not port all effects | **Respected.** Only toy `DimEffect` and `ExplicitRoleWriteEffect` exist to prove semantics. |
| Do not replace the current compositor | **Respected.** No legacy compositor files were modified. |
| Do not build recipes/studio/runtime/phase graph | **Respected.** No recipe compiler, studio manifest, runtime store, trigger engine, or phase graph was added. |
| Do not preserve legacy aliases | **Respected.** The new crate has no alias/compatibility loader. |
| Do not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, or `tui-vfx-shadow` | **Respected.** `cargo tree -p tui-vfx-next` shows only `tui-vfx-types` as direct dependency. |
| May depend on `tui-vfx-types` / `tui-vfx-geometry` | **Partially used.** The final cleanup removed the unused direct `tui-vfx-geometry` dependency. `Rect` currently comes from `tui-vfx-types`. |
| Keep phase small enough to review | **Done.** New crate is small and test-focused. |

## Required semantics coverage

The required tests from the kickoff are present in `crates/tui-vfx-next/tests/surface_contract.rs`:

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

These tests prove the Phase A rules:

- copied cells use sampled-source roles by default;
- visual-only effects preserve destination roles;
- role scopes use sampled-source role space by default;
- skipped/out-of-scope cells preserve destination cell and destination role;
- zero-cell scopes emit structured diagnostics and do not mutate destination;
- procedural writes can explicitly set roles such as `RoleTag::Shadow`;
- empty transparent writes are distinguishable from skipped writes.

## Dependency and architecture status

Current dependency direction is:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

`cargo tree -p tui-vfx-next` currently shows no dependency on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

This is slightly stricter than the kickoff allowed: `tui-vfx-geometry` was allowed, but the direct dependency was removed because the crate does not currently need it.

## Verification evidence

Fresh check run after re-reading the kickoff:

```text
cargo test -p tui-vfx-next
```

Result:

```text
8 passed; 0 failed
Doc-tests tui_vfx_next: 0 passed; 0 failed
```

Earlier post-cleanup leader verification in this session also passed:

```text
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-next
cargo test --workspace
cargo tree -p tui-vfx-next
```

The independent verifier returned `APPROVED` before the final cleanup. The cleanup only removed an unused direct dependency and a single-use helper trait; package tests and workspace tests passed after that cleanup.

## Notable implementation choices

1. **Single crate for Phase A.**
   The kickoff allowed either `tui-vfx-next` or a longer-term split. The implementation chose `tui-vfx-next` to keep the spike bounded.

2. **Surface stores roles separately from cells.**
   `Surface` owns an `OwnedGrid` plus a row-major `Vec<RoleTag>`. This makes the role grid explicit without modifying existing foundational types.

3. **Role-space and coordinate-space are explicit enums.**
   `CoordinateSpace::DestinationLocal` and `RoleSpace::SampledSource` are defaults, matching the kickoff.

4. **Only identity sampling exists.**
   The engine currently samples source and destination at the same coordinate. This is enough to prove the role-space contract but intentionally leaves non-identity sampled-source coordinate behavior for later.

5. **Structured diagnostics are first-class.**
   Zero-cell scope and surface-size mismatch diagnostics use `SurfaceDiagnostic` with level/code/message/path/hint shape.

6. **No direct geometry dependency remains.**
   `ScopeSpec::Rect` uses `tui_vfx_types::Rect`; direct `tui-vfx-geometry` dependency was unnecessary for Phase A.

## Scope-control status

The implementation did **not** attempt to:

```text
port CRT
port typewriter
port matrix rain
port all masks
replace render_pipeline
rewrite FilterSpec
rewrite StyleRegion
build the recipe compiler
build the studio
support all legacy aliases
```

That matches the kickoff boundary.

## Open questions / recommended next decisions

1. **Name and crate split.**
   Decide whether `tui-vfx-next` remains the incubator or whether the next phase splits it into `tui-vfx-contract` and `tui-vfx-engine`.

2. **Role grid representation.**
   Phase A uses `Vec<RoleTag>`. Before broader migration, decide whether to reuse/adapt `RoleMap` or keep the new surface role storage independent.

3. **Non-identity sampling.**
   The kickoff allowed identity/fake sampling for Phase A. The next phase should prove sampled-source coordinates differ from destination coordinates while preserving the source-role default.

4. **Custom role strictness.**
   The doc states the custom-role policy, but there is no strict declaration/validation layer yet. That belongs with future schema/compiler work.

5. **Effect descriptor maturity.**
   `EffectDescriptor` is intentionally tiny. The larger descriptor model from `DRAFT_CONTRACTS.md` is not implemented yet.

6. **Manifest/schema/runtime integration.**
   None of the recipe compiler, studio manifest, parameter store, signal store, phase engine, or trigger validation exists in `tui-vfx-next`; that remains future work by design.

## Bottom line

Phase A is complete as a contract proof. The project now has a clean-room kernel that demonstrates the semantic surface rules independently of the old compositor. The next useful step is not to port effects yet; it is to choose the Phase B boundary: either deepen the surface engine with non-identity sampling/layer behavior, or start the Rust-owned contract/model layer that will eventually feed effect descriptors and recipe compilation.

<!-- <FILE>docs/new_kernel/PHASE_A_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase A status memo to the surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
