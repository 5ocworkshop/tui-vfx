<!-- <FILE>docs/new_kernel/PHASE_E1_STATUS.md</FILE> - <DESC>Concise Phase E1 minimal effect descriptor status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase E1 wrap: record minimal effect descriptor decisions and verification evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record descriptor DTOs, schema root, validation tests, and Phase F deferral.</CLOG> -->

# Phase E1 Status — Minimal Effect Descriptor Model

## Status

Phase E1 is complete and ready for commit.

## What changed

Phase E1 adds the first durable effect descriptor contract to:

```text
crates/tui-vfx-contract
```

The descriptor model is capability-only. It declares what an effect is allowed to read, write, target, support, and report as lifecycle metadata. It does not configure typed effect inputs.

## Decisions locked

- `EffectDescriptor` now lives in `tui-vfx-contract` as a stable schema-backed contract root.
- `EffectId`, `EffectDomain`, `CellAccess`, `ScopeSupport`, `ScopeKind`, `WriteSupport`, `RoleWritePolicyKind`, `EffectLifecycle`, `EffectCompletion`, and `DescriptorValidationError` support the descriptor root.
- `EffectDomain` now uses the E1 domain vocabulary: `contentGenerator`, `contentTransform`, `cellShader`, `frameFilter`, `coordinateSampler`, `mask`, `shadow`, `postProcess`, and `diagnosticTooling`.
- Descriptor validation accepts supported scope/write/channel requests and rejects unsupported ones.
- `PipelineStage`, `SurfacePipeline`, `PipelineSampler`, `SurfaceEngine`, and proof effects remain proof-only in `tui-vfx-next`.
- The old tiny proof descriptor was removed from active `tui-vfx-next` source; durable descriptor vocabulary is owned by `tui-vfx-contract`. A local ignored recyclebin copy exists for recovery per repo cleanup protocol, but it is not part of the committed change set.
- Effect input contracts are intentionally deferred to Phase F.

## Schema impact

New checked stable contract schema:

```text
schemas/v3.1/contract/effect-descriptor.schema.json
```

Existing proof schemas remain in:

```text
schemas/v3.1/next/
```

## Deliberately not added

Phase E1 does not add:

```text
ValueKind / ValueSource
input specs / defaults / ranges
runtime mutability
parameters / signals / bindings
recipe nodes or recipe compiler
descriptor registry
runtime graph / phase graph / trigger engine
studio controls / studio manifest
legacy migration
real effect ports
```

## Verification evidence

```text
cargo fmt --package tui-vfx-contract -- --check                         PASS
cargo fmt --package tui-vfx-next -- --check                             PASS
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings           PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings               PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current  PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_proof_schemas_are_current         PASS
cargo test -p tui-vfx-contract --test test_schema_generation            PASS
cargo test -p tui-vfx-next --test test_schema_generation                PASS
cargo test -p tui-vfx-contract --test test_effect_descriptor_contract   PASS
cargo test -p tui-vfx-contract                                          PASS
cargo test -p tui-vfx-next                                              PASS
cargo tree -p tui-vfx-contract                                          PASS / inspected; no reverse or forbidden direct dependency
cargo tree -p tui-vfx-next                                              PASS / inspected; depends on tui-vfx-contract
grep forbidden dependencies over crates/tui-vfx-contract crates/tui-vfx-next  PASS / no matches
cargo test --workspace                                                  PASS
git diff --check                                                        PASS
```

## Next recommended phase

Phase F should define the effect input/value/parameter/signal model, if the architect agrees.

<!-- <FILE>docs/new_kernel/PHASE_E1_STATUS.md</FILE> - <DESC>Concise Phase E1 minimal effect descriptor status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
