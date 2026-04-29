<!-- <FILE>docs/new_kernel/PHASE_G4_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G4 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G4 wrap: report node I/O value-bus proof and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase G4 architect memo in the established status-memo style.</CLOG> -->

# Phase G4 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: G4 — Node I/O / Hint Value Bus

## Executive summary

Phase G4 implements the node I/O / graph-local value bus recommended in `ARCH-RESP-TO-PHASE_G3.md`.

Current answer: **the canonical graph model can now represent typed node-output chains. The proof executor updates a graph-local value bus through sequence and parallel topology, supports output fan-out, preserves parallel branch isolation, exposes joined values downstream, handles same-output conflicts by explicit policy, and proves spatial scalar fields as per-cell fields rather than frame-global numbers.**

The phase intentionally stops before source recipe documents, runtime stores, binding execution, phase engines, studio manifests, assets/procedurals, migration, and real effect ports.

## Current implementation state

Stable contract crate:

```text
crates/tui-vfx-contract
```

New/changed contract vocabulary:

```text
GraphValueId
GraphValueKind
GraphValueShape
EffectOutputId
EffectOutputSpec
NodeOutputSpec
NodeOutputSource
GraphValueMergePolicy
ValueSource::GraphValue
EffectDescriptor.outputs
NodeSpec.outputs
GraphStep::Parallel.valueMergePolicy
```

Proof crate:

```text
crates/tui-vfx-next
```

New proof vocabulary:

```text
ProofValue
NumberCellField
```

New/updated proof execution helpers:

```text
orc_execute_graph_step
orc_apply_proof_node
fnc_resolve_value_source
fnc_read_proof_input
fnc_apply_dim_with_number_field
```

## Goal-by-goal status against the G4 recommendation

| G4 goal / constraint | Current status |
|---|---|
| Node output identity | **Done.** Nodes publish graph-local values keyed by `GraphValueId`. |
| Typed output values | **Done.** `GraphValueKind::Number` is locked as the initial scalar lane. |
| Output shape/cardinality | **Done.** `GraphValueShape` distinguishes `frameValue` and `cellField`. |
| Input consumption | **Done.** `ValueSource::GraphValue` lets node inputs consume prior graph values. |
| Fan-out | **Done.** One output can feed multiple later inputs. |
| Sequence visibility | **Done.** Sequence updates the bus after each node. |
| Parallel isolation | **Done.** Sibling branches read the same starting bus and cannot see sibling-local outputs. |
| Parallel join | **Done.** Branch value deltas merge at join and become visible downstream. |
| Output conflict policy | **Done.** Child-order last-writer-wins and error-on-conflict are implemented. |
| Descriptor validation | **Done.** Effect outputs must be declared; input re-emission must reference known inputs. |
| Spatial scalar fields | **Done.** `proof.spatialScalarField` produces a `cellField` that drives cell-varying dim factors. |
| No F2 binding execution | **Respected.** Bindings remain validation-only. |
| Keep runtime/recipe/studio out | **Respected.** No runtime stores, compiler, phase engine, studio, assets/procedurals, migration, or real ports were added. |

## Key decisions

### Graph values are contract vocabulary; proof values are not

The durable contract names live in `tui-vfx-contract`: graph value id/kind/shape, output specs, node output specs, and merge policy. `ProofValue` and `NumberCellField` live in `tui-vfx-next` as proof execution machinery only.

### Shape is explicit at the contract boundary

The architect warned that scalar hints are not always global scalars. G4 therefore distinguishes `frameValue` from `cellField`. The proof test `spatial_field_output_can_drive_cell_varying_input` proves a normalized-x field can drive per-cell dim factors.

### Graph values are node-input sources only

`ValueSource::GraphValue` is part of the unified value-source vocabulary, but validation is contextual. It is accepted for `NodeSpec.inputs` and rejected where no graph-local bus exists, including binding validation.

### Parallel value merge mirrors surface merge intent

Parallel surface writes keep `ParallelMergePolicy`; graph values use `GraphValueMergePolicy`. Both support child-order LWW and explicit error policies, but value conflicts are keyed by `GraphValueId` rather than cell channel.

## What deliberately was not added

Phase G4 does not add:

```text
source recipe authoring schema
canonical recipe compiler
runtime ParameterStore / SignalStore
F2 BindingSpec execution
live override precedence execution
direct node/effect-input bindings
phase graph / trigger / dwell engine
visibility predicate execution
loopback / demo signal execution
asset / procedural source system
studio manifest / controls
legacy migration / aliases
real effect ports
```

## Verification evidence

Final required verification passed:

```text
cargo fmt --package tui-vfx-contract -- --check
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-next
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test -p tui-vfx-next --test test_schema_generation
cargo test --workspace
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
forbidden legacy crate grep over tui-vfx-contract and tui-vfx-next
git diff --check
```

## Request for next assignment

Please review Phase G4 as the node I/O / graph-local value bus lock point and advise the next phase.

Based on your response, the next likely decision is whether to proceed to:

```text
H0 — Source / Asset / Procedural Source Contract
```

or:

```text
H1 — Canonical Recipe Document Schema
```

G4 is now in place as the graph capability prerequisite for either path.

<!-- <FILE>docs/new_kernel/PHASE_G4_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G4 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
