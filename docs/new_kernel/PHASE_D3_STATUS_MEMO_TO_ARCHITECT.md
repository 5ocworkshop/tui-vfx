<!-- <FILE>docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D3 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>New kernel Phase D3 wrap: report boundary decisions and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — record that pre-existing unrelated worktree files are excluded from D3 ownership.
0.1.0: INIT — add Phase D3 architect memo in the established status-memo style.</CLOG> -->

# Phase D3 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: D3 — Contract / Engine Boundary + Generalized Scope / Write Model

## Executive summary

Phase D3 has completed the boundary work requested in `ARCH-RESP-TO-PHASE_D2.md`.

Current answer: **`tui-vfx-next` remains one clean-room crate, but now has a documented logical boundary between stable contract vocabulary, proof implementation, checked schema roots, and test-only scaffolding.**

The new document, `docs/v3.1-contract-boundary.md`, classifies the current public names and locks the vocabulary future descriptors should reuse. The crate now exposes three grouping modules:

```text
tui_vfx_next::contract
tui_vfx_next::proof
tui_vfx_next::schema_roots
```

These are import/documentation lanes only. No physical crate split was done.

## Current design state

New document:

```text
docs/v3.1-contract-boundary.md
```

Updated supporting docs:

```text
docs/v3.1-surface-contract.md
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
```

Code/test additions:

```text
crates/tui-vfx-next/src/lib.rs
crates/tui-vfx-next/tests/test_contract_boundary.rs
```

New status artifacts:

```text
docs/new_kernel/PHASE_D3_STATUS.md
docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md
```

Captured architect response:

```text
docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md
```

## Goal-by-goal status against the D3 recommendation

| D3 goal / question | Current status |
|---|---|
| Identify public v3.1 contract types | **Done.** Surface, scene, scope, write, sampler semantics, diagnostics, ids, and outcomes are classified. |
| Identify engine/proof implementation | **Done.** `SurfaceEngine`, low-level apply/scope helpers, proof pipeline mechanics, and proof effects are marked proof-facing. |
| Identify test-only scaffolding | **Done.** Test helpers remain test-only and are not referenced as public contract APIs. |
| Identify schema roots | **Done.** Existing nine checked schema roots remain current; no new roots were added. |
| Decide `SurfacePipeline` status | **Done.** It remains a checked proof-pipeline root, not the final runtime graph. |
| Decide `PipelineStage` status | **Done.** It is a toy proof enum, not the future descriptor model. |
| Decide `ScopeSpec` status | **Done.** It is the current canonical generalized scope vocabulary. |
| Decide coordinate/role space placement | **Done.** `CoordinateSpace` and `RoleSpace` remain operation-level context for now. |
| Confirm shared write policy | **Done.** Pipeline and scene both reuse `CellWritePolicy` and `RoleWritePolicy`. |
| Confirm diagnostics convention | **Done.** Stable codes plus path strings remain; structured identity fields are deferred. |
| Decide physical crate split | **Done.** No split now; logical modules are sufficient. |
| Avoid descriptor/recipe/runtime implementation | **Respected.** No descriptor model, recipe schema/compiler, template expander, runtime, studio, migration, or real effects were added. |

## Key decisions

### Contract vocabulary is stable, toy stages are not

Future descriptors may reference:

```text
Surface
Scene
ScopeSpec
CoordinateSpace
RoleSpace
CellWritePolicy
RoleWritePolicy
sampler semantics
SurfaceDiagnostic
cell channels
effect domains
```

Future descriptors must not copy `PipelineStage` as their shape. The toy stage enum remains useful evidence for ordered pipeline semantics, but it is not the effect descriptor model.

### `ScopeSpec` stays context-free

`ScopeSpec` remains the generalized scope value. Coordinate and role spaces stay as operation-level context instead of fields on the scope value. This keeps one scope reusable across pipeline, scene, and future descriptor contexts.

### Schema roots remain current

D3 made no JSON wire-shape changes. The checked schemas remain green. The docs now classify `sampler.schema.json` and `pipeline.schema.json` as proof-pipeline artifacts so future descriptor work does not mistake them for the final runtime graph.

### Logical boundary before physical split

The crate now exposes `contract`, `proof`, and `schema_roots` modules. This gives agents and future code a stable way to import by intent without moving every OFPF file or creating a premature crate split.

## Unrelated worktree files excluded from D3

The worktree also contains pre-existing uncommitted files outside this phase:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

They are explicitly excluded from the D3 change set and must not be staged or committed with this phase.

## What deliberately was not added

Phase D3 does not add:

```text
effect descriptor expansion
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

## Verification evidence

Final D3 verification passed:

```text
cargo fmt --package tui-vfx-next -- --check                                      PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings                       PASS
cargo test -p tui-vfx-next                                                       PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current  PASS
cargo test -p tui-vfx-next --test test_schema_generation                         PASS
cargo tree -p tui-vfx-next                                                       PASS
grep forbidden dependencies under crates/tui-vfx-next                            PASS / no matches
cargo test --workspace                                                           PASS
```

## Open questions for next assignment

1. Should Phase E start with a minimal effect descriptor DTO/schema that references the D3 vocabulary?
2. Should Phase E include descriptor schema roots immediately, or first prove descriptor semantics in Rust and tests before adding checked JSON Schema roots?
3. Should descriptor diagnostics introduce structured identity fields such as `effectId`, `stageName`, or `elementId`, or continue with path strings until recipe schemas exist?
4. Should `PipelineSampler` be renamed or wrapped in Phase E as a generalized sampler declaration, while preserving the current checked proof-pipeline schema root?

## Bottom line

Phase D3 turns the clean-room proof into a safer foundation for descriptor work. The public vocabulary is now classified, the toy proof internals are labeled, schema roots remain current, and no descriptor/recipe/runtime implementation leaked into the phase.

Recommended next architect assignment: **Phase E — Effect Descriptor Model**.

<!-- <FILE>docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D3 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
