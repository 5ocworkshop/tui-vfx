<!-- <FILE>docs/new_kernel/PHASE_D3_STATUS.md</FILE> - <DESC>Concise Phase D3 contract/engine boundary status</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>New kernel Phase D3 wrap: record contract/proof boundary decisions and verification evidence.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — record that pre-existing unrelated worktree files are excluded from D3 ownership.
0.1.0: INIT — record Phase D3 logical boundary, docs, code grouping, and green verification.</CLOG> -->

# Phase D3 Status — Contract / Engine Boundary

## Status

Phase D3 is complete, verified green, and ready for architect review.

## What changed

Phase D3 adds the durable boundary document:

```text
docs/v3.1-contract-boundary.md
```

It also adds logical grouping modules in `tui-vfx-next`:

```text
tui_vfx_next::contract
tui_vfx_next::proof
tui_vfx_next::schema_roots
```

These modules classify existing exports without moving files or splitting crates.

## Decisions locked

Phase D3 locks:

- `Surface`, `Scene`, `ScopeSpec`, write policies, sampler semantics, diagnostics, ids, and outcomes are stable v3.1 contract vocabulary.
- `SurfaceEngine`, low-level apply helpers, and scope-coordinate helpers are proof implementation.
- `SurfacePipeline` is a checked proof-pipeline schema root, not the final runtime graph.
- `PipelineStage` is a toy proof enum, not the future effect descriptor model.
- `DimEffect`, `ExplicitRoleWriteEffect`, and the tiny `EffectDescriptor` are proof artifacts.
- `ScopeSpec` remains the current canonical generalized scope vocabulary.
- `CoordinateSpace` and `RoleSpace` remain operation-level context for now.
- Scene composition and pipeline execution reuse the same `CellWritePolicy` and `RoleWritePolicy` vocabulary.
- Diagnostic path strings remain the D3 convention; structured identity fields are deferred until descriptor/recipe schemas exist.
- `tui-vfx-next` remains one physical crate; the D3 boundary is logical/module-level.

## Schema impact

No schema roots changed.

Current checked roots remain:

```text
surface.schema.json
scope.schema.json
write.schema.json
sampler.schema.json
pipeline.schema.json
diagnostic.schema.json
scene.schema.json
element.schema.json
outcome.schema.json
```

D3 clarifies that `sampler.schema.json` and `pipeline.schema.json` are proof-pipeline roots, not the future descriptor/runtime model.

## Docs updated

```text
docs/v3.1-contract-boundary.md
docs/v3.1-surface-contract.md
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
```

Phase artifacts:

```text
docs/new_kernel/ARCH-RESP-TO-PHASE_D2.md
docs/new_kernel/PHASE_D3_STATUS.md
docs/new_kernel/PHASE_D3_STATUS_MEMO_TO_ARCHITECT.md
```

## Code updated

```text
crates/tui-vfx-next/src/lib.rs
crates/tui-vfx-next/tests/test_contract_boundary.rs
```

The test proves that the `contract`, `proof`, and `schema_roots` import lanes compile and preserve existing copy/write behavior.

## Verification evidence

Final Phase D3 verification passed:

- `cargo fmt --package tui-vfx-next -- --check` — PASS
- `cargo clippy -p tui-vfx-next --all-targets -- -D warnings` — PASS
- `cargo test -p tui-vfx-next` — PASS
- `UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current` — PASS
- `cargo test -p tui-vfx-next --test test_schema_generation` — PASS
- `cargo tree -p tui-vfx-next` — PASS; no forbidden clean-room dependency in tree
- forbidden dependency grep over `crates/tui-vfx-next` — PASS / no matches
- `cargo test --workspace` — PASS

## Unrelated worktree files excluded from D3

The worktree also contains pre-existing uncommitted files outside the Phase D3 scope, including `docs/new_kernel/PHASE_D0_STATUS.md`, `docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md`, and `pro/*`. They are explicitly excluded from the D3 change set and must not be staged or committed with this phase.

## What deliberately did not change

Phase D3 does not implement:

- effect descriptor expansion;
- recipe schema/compiler;
- source authoring schemas;
- template expansion;
- runtime bindings;
- phase graph or trigger engine;
- studio manifest;
- legacy migration;
- real effect ports;
- full layer graph;
- complex blend modes.

## Next recommended phase

Phase E should start the effect descriptor model using the contract vocabulary locked by D3 and without copying the toy `PipelineStage` shape.

<!-- <FILE>docs/new_kernel/PHASE_D3_STATUS.md</FILE> - <DESC>Concise Phase D3 contract/engine boundary status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
