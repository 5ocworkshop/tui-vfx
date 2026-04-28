<!-- <FILE>docs/new_kernel/PHASE_E1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase E1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase E1 wrap: report minimal effect descriptor model and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase E1 architect memo in the established status-memo style.</CLOG> -->

# Phase E1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: E1 — Minimal Effect Descriptor Model

## Executive summary

Phase E1 has implemented the minimal descriptor capability model recommended in `ARCH-RESP-TO-PHASE_E0.md`.

Current answer: **`tui-vfx-contract` now owns a schema-backed `EffectDescriptor` that declares identity, domain, cell access, scope support, write support, and lifecycle category.**

The implementation intentionally stops before the input/value/parameter/signal system. Phase F can now design that layer against a stable descriptor capability shell instead of mixing capability and input semantics in one step.

## Current implementation state

Descriptor contract crate:

```text
crates/tui-vfx-contract
```

Proof crate remains:

```text
crates/tui-vfx-next
```

New schema root:

```text
schemas/v3.1/contract/effect-descriptor.schema.json
```

## Goal-by-goal status against the E1 recommendation

| E1 goal / constraint | Current status |
|---|---|
| Add `EffectDescriptor` to `tui-vfx-contract` | **Done.** Durable descriptor DTO is exported from the contract crate. |
| Keep descriptors out of `tui-vfx-next` | **Done.** The old proof descriptor was removed from active source; `tui-vfx-next` keeps proof effects only. |
| Add stable effect identity | **Done.** `EffectId` is a transparent stable id newtype. |
| Lock initial domain vocabulary | **Done.** `EffectDomain` contains the requested E1 domain set. |
| Declare cell access | **Done.** `CellAccess` declares readable and writable `CellChannel` values. |
| Declare scope support | **Done.** `ScopeSupport` and `ScopeKind` declare supported scope variants and spaces. |
| Declare write support | **Done.** `WriteSupport` and `RoleWritePolicyKind` declare supported cell and role policies. |
| Add lifecycle stub | **Done.** `EffectLifecycle` and `EffectCompletion` cover completion/reset/seek/determinism metadata. |
| Add validation helpers | **Done.** Descriptor checks reject unsupported scope kinds, role policies, cell policies, and write channels. |
| Add checked descriptor schema | **Done.** `effect-descriptor.schema.json` is generated from Rust and checked in. |
| Preserve schema/reference rules | **Done.** Descriptor DTOs derive Serde/Schemars, use strict shapes where needed, and carry rustdoc descriptions. |
| Avoid Phase F+ scope | **Respected.** No input/value/source/parameter/signal/recipe/runtime/registry/effect-port work was added. |

## Key decisions

### Descriptor is capability-only

`EffectDescriptor` answers what an effect is allowed to do:

```text
identity
domain
cell reads / cell writes
scope support
write-policy support
lifecycle category
```

It does not answer how authors configure effect inputs. That keeps Phase E1 stable and avoids a premature partial `InputSpec`.

### Validation is local and explicit

The contract crate now has small validation helpers for descriptor capabilities. Tests prove:

```text
supported role scope accepted
unsupported rect scope rejected
supported cell write policy accepted
unsupported cell write policy rejected
supported SetExplicit role write accepted
unsupported SetExplicit role write rejected for visual-only descriptor
undeclared role-channel write rejected
coordinate sampler domain does not claim cell writes
contract descriptor source does not import proof PipelineStage / SurfacePipeline
```

### Proof pipeline stays proof-only

`PipelineStage` remains in `tui-vfx-next`. `SurfacePipeline` and `PipelineSampler` remain proof schema roots under `schemas/v3.1/next/`.

The old tiny proof `EffectDescriptor` was removed from active `tui-vfx-next` source. A local ignored copy remains under `recyclebin/` for recovery according to the recyclebin protocol; the committed change set records only the active-source deletion.

## What deliberately was not added

Phase E1 does not add:

```text
full input specs
ValueKind / ValueSource
defaults / ranges
runtime mutability
parameters / signals / bindings
recipe schema/compiler
recipe nodes / node graph
descriptor registry
runtime graph / phase graph / trigger engine
studio controls / studio manifest
legacy migration
real effect ports
```

## Unrelated worktree files excluded from E1

The worktree also contains pre-existing uncommitted files outside this phase:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

They are explicitly excluded from the E1 change set and must not be staged or committed with this phase.

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

## Open questions for next assignment

1. Should Phase F define a closed `ValueKind` enum first, or start from `ValueSource` and derive kind support from sources?
2. Should descriptor input specs include studio-facing metadata immediately, or should studio metadata remain a later manifest layer?
3. Should effect descriptor validation return a generic contract diagnostic type, or keep the narrow `DescriptorValidationError` until recipe validation exists?
4. Should descriptor schemas remain flat under `schemas/v3.1/contract/`, or should Phase F introduce a descriptor/input subdirectory once schema count grows?

## Bottom line

Phase E1 gives future effect and recipe work a stable capability shell. The next architectural decision is the input/value/source model, not more descriptor boundary work.

Recommended next architect assignment: **Phase F — Effect Input / Value / Parameter / Signal Model**.

<!-- <FILE>docs/new_kernel/PHASE_E1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase E1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
