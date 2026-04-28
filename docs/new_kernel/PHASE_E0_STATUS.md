<!-- <FILE>docs/new_kernel/PHASE_E0_STATUS.md</FILE> - <DESC>Concise Phase E0 physical contract split status</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase E0 wrap: record completed physical tui-vfx-contract split and verification evidence.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — finalize Phase E0 status with PASS verification evidence.
0.1.0: INIT — record contract crate split, proof crate dependency direction, schema path split, and verification plan.</CLOG> -->

# Phase E0 Status — Physical Contract Split

## Status

Phase E0 is complete.

## What changed

Phase E0 creates a dedicated stable contract crate:

```text
crates/tui-vfx-contract
```

`crates/tui-vfx-next` now depends on `tui-vfx-contract` and keeps proof-only execution pieces.

## Decisions locked

- Stable surface, scene, scope, write, sampler, diagnostic, id, and outcome DTOs live in `tui-vfx-contract`.
- `tui-vfx-contract` depends directly only on `tui-vfx-types`, `serde`, and `schemars` plus `serde_json` for tests.
- `tui-vfx-contract` does not depend on `tui-vfx-next` or legacy compositor/style/content/shadow crates.
- `tui-vfx-next` depends on `tui-vfx-contract` and keeps `SurfaceEngine`, toy pipeline stages, proof effects, and proof helpers.
- Stable schemas moved to `schemas/v3.1/contract/`.
- Proof-pipeline schemas remain in `schemas/v3.1/next/`.
- `PipelineStage` remains proof-only and is not promoted to descriptor model.
- Phase E0 does not implement descriptors, recipes, runtime, studio, migration, or real effects.

## Schema impact

Stable contract schemas now generate from `tui-vfx-contract`:

```text
schemas/v3.1/contract/surface.schema.json
schemas/v3.1/contract/scope.schema.json
schemas/v3.1/contract/write.schema.json
schemas/v3.1/contract/diagnostic.schema.json
schemas/v3.1/contract/scene.schema.json
schemas/v3.1/contract/element.schema.json
schemas/v3.1/contract/outcome.schema.json
```

Proof schemas still generate from `tui-vfx-next`:

```text
schemas/v3.1/next/sampler.schema.json
schemas/v3.1/next/pipeline.schema.json
```

## Code moved

Moved to `tui-vfx-contract`:

```text
Surface / SurfaceMetadata / CellChannel
CellWrite / CellWritePolicy / RoleWritePolicy
ScopeSpec / CoordinateSpace / RoleSpace / ScopeEvalInput / ShiftSampler / CoordinateSampler
SurfaceDiagnostic / SurfaceDiagnosticCode / DiagnosticLevel
Scene / SceneElement / SceneOutcome
ElementId / LayerId / ElementPlacement / ClipPolicy
ApplyOutcome / EffectDomain
```

Kept in `tui-vfx-next`:

```text
SurfaceEngine
SurfacePipeline / PipelineStage / PipelineSampler / PipelineOutcome
DimEffect / ExplicitRoleWriteEffect / EffectDescriptor / IdentitySampler
proof apply/rewrite/diagnostic helper functions
```

## Docs updated

```text
docs/new_kernel/ARCH-RESP-TO-PHASE_D3.md
docs/v3.1-contract-boundary.md
docs/v3.1-surface-contract.md
docs/v3.1-architecture-overview.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/INDEX.md
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

## Next recommended phase

Phase E1 should start the minimal effect descriptor model in `tui-vfx-contract`.

<!-- <FILE>docs/new_kernel/PHASE_E0_STATUS.md</FILE> - <DESC>Concise Phase E0 physical contract split status</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
