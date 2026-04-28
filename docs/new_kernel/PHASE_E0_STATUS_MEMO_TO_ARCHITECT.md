<!-- <FILE>docs/new_kernel/PHASE_E0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase E0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase E0 wrap: report completed physical contract split and request next assignment.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — finalize Phase E0 architect memo with verification evidence.
0.1.0: INIT — add Phase E0 architect memo in the established status-memo style.</CLOG> -->

# Phase E0 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: E0 — Physical Contract Split

## Executive summary

Phase E0 has implemented the split recommended in `ARCH-RESP-TO-PHASE_D3.md`.

Current answer: **stable v3.1 contract DTOs now live in `tui-vfx-contract`; proof execution remains in `tui-vfx-next`.**

This places the project at the intended boundary before the effect descriptor model becomes real. Phase E1 can add descriptors directly to the contract crate instead of adding durable public schema roots to the incubator and moving them later.

## Current implementation state

New crate:

```text
crates/tui-vfx-contract
```

Updated proof crate:

```text
crates/tui-vfx-next
```

Schema paths:

```text
schemas/v3.1/contract/   stable contract roots
schemas/v3.1/next/       proof-pipeline roots
```

## Goal-by-goal status against the E0 recommendation

| E0 goal / constraint | Current status |
|---|---|
| Create `tui-vfx-contract` | **Done.** New workspace crate added. |
| Make `tui-vfx-next` depend on contract crate | **Done.** `tui-vfx-next` imports/re-exports stable DTOs from `tui-vfx-contract`. |
| Prevent reverse dependency | **Done.** `tui-vfx-contract` does not depend on `tui-vfx-next`. |
| Keep forbidden legacy deps out of contract | **Done.** Contract crate has no direct compositor/style/content/shadow dependency. |
| Move stable DTOs | **Done.** Surface, scene, scope, write, diagnostic, id, sampler contract, and outcome DTOs moved. |
| Keep proof-only types in next | **Done.** `SurfaceEngine`, `SurfacePipeline`, `PipelineStage`, proof effects, and helpers remain in next. |
| Keep `PipelineStage` out of descriptor model | **Done.** It remains proof-only. |
| Move stable schemas | **Done.** Stable roots now live under `schemas/v3.1/contract/`. |
| Keep proof schemas labeled separately | **Done.** `sampler` and `pipeline` remain under `schemas/v3.1/next/`. |
| Preserve D0 schema/reference rules | **Done.** Contract schema tests enforce strict shapes and rustdoc-backed descriptions. |
| Avoid descriptors/recipes/runtime | **Respected.** No descriptor, recipe, runtime, studio, migration, or real-effect work was added. |

## Key decisions

### Contract crate owns stable DTOs

The physical split follows the D3 boundary. `tui-vfx-contract` owns the contract vocabulary that is useful without the proof engine:

```text
Surface / Scene / ScopeSpec / CellWrite
CoordinateSpace / RoleSpace / CoordinateSampler / ShiftSampler
SurfaceDiagnostic / ids / placement / clip policy / outcomes
```

### Proof crate proves the contract

`tui-vfx-next` now proves that the contract crate can drive an engine. It keeps:

```text
SurfaceEngine
SurfacePipeline
PipelineStage
PipelineSampler
proof effects
proof helper functions
A/B/C semantic proof tests
```

D1 scene semantics moved to `tui-vfx-contract` tests because scene composition is now stable contract behavior.

### Schema paths distinguish stable from proof

Stable public schemas moved to:

```text
schemas/v3.1/contract/
```

Proof-pipeline schemas remain in:

```text
schemas/v3.1/next/
```

This prevents proof pipeline artifacts from looking like the final runtime graph contract.

## What deliberately was not added

Phase E0 does not add:

```text
effect descriptors
recipe schema/compiler
source authoring schemas
template expansion
runtime bindings
phase graph
trigger engine
studio manifest
legacy migration
real effect ports
full layer graph
complex blend modes
```

## Unrelated worktree files excluded from E0

The worktree also contains pre-existing uncommitted files outside this phase:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

They are explicitly excluded from the E0 change set and must not be staged or committed with this phase.

## Verification evidence

```text
cargo fmt --package tui-vfx-contract -- --check                         PASS
cargo fmt --package tui-vfx-next -- --check                             PASS
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings           PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings               PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current  PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_proof_schemas_are_current         PASS
cargo test -p tui-vfx-contract                                          PASS
cargo test -p tui-vfx-next                                              PASS
cargo test -p tui-vfx-contract --test test_schema_generation            PASS
cargo test -p tui-vfx-next --test test_schema_generation                PASS
cargo tree -p tui-vfx-contract                                          PASS / inspected; no reverse or forbidden direct dependency
cargo tree -p tui-vfx-next                                              PASS / inspected; depends on tui-vfx-contract
grep forbidden dependencies over crates/tui-vfx-contract crates/tui-vfx-next  PASS / no matches
cargo test --workspace                                                  PASS
git diff --check                                                        PASS
```

## Open questions for next assignment

1. Should Phase E1 introduce only descriptor identity/domain/channel/scope/write support, or also input/value specs?
2. Should `EffectDomain` be expanded in E1, or stay minimal until the first real descriptor cases force more vocabulary?
3. Should descriptor schema roots live entirely under `schemas/v3.1/contract/`, or should there be a descriptor subdirectory once the schema count grows?
4. Should proof `PipelineSampler` be replaced by a contract-owned generalized sampler declaration in E1, or remain proof-only until descriptor needs force it?

## Bottom line

Phase E0 gives descriptor work the right physical home. Stable contract vocabulary is no longer mixed with proof-engine scaffolding, and `tui-vfx-next` now proves the contract crate instead of owning it.

Recommended next architect assignment: **Phase E1 — Minimal Effect Descriptor Model**.

<!-- <FILE>docs/new_kernel/PHASE_E0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase E0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
