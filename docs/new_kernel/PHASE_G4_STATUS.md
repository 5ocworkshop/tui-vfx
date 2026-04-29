<!-- <FILE>docs/new_kernel/PHASE_G4_STATUS.md</FILE> - <DESC>Phase G4 node I/O and graph value bus implementation status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G4 wrap: summarize node outputs, value-bus execution, spatial fields, docs, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase G4 status for architect handoff.</CLOG> -->

# Phase G4 Status — Node I/O / Graph Value Bus

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: G4 — Node I/O / Hint Value Bus

## Summary

Phase G4 adds graph-local node outputs and a proof value bus across canonical graph execution.

Current answer: **`GraphSpec` can now represent typed node-output chains. Descriptors declare effect outputs, nodes publish graph-local values from effect outputs or re-emitted inputs, later node inputs consume those values through `ValueSource::GraphValue`, sequence nodes see prior outputs, one output can fan out, parallel branches are value-bus isolated, and joined branch outputs become visible downstream under explicit value conflict policy.**

This remains a clean-room proof layer. It does not add runtime parameter/signal stores, F2 binding execution, direct node/effect-input binding targets, recipe source syntax/compiler, phase/trigger/dwell semantics, visibility predicates, loopback/demo signals, asset/procedural sources, studio metadata, migration, aliases, or real effect ports.

## Implemented contract APIs

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

Graph values currently lock the required initial scalar lane:

```text
GraphValueKind::Number
GraphValueShape::FrameValue
GraphValueShape::CellField
```

The shape split is intentional: spatial scalar fields such as normalized-x remain per-cell fields and are not collapsed into one frame-global number.

## Implemented proof APIs

```text
ProofValue
NumberCellField
```

New/updated proof helpers:

```text
orc_execute_graph_step
orc_apply_proof_node
fnc_resolve_value_source
fnc_read_proof_input
fnc_apply_dim_with_number_field
```

## Proof adapters

G4 keeps the existing toy proof adapters and adds proof-only value-bus adapters:

```text
proof.consumeNumber
proof.spatialScalarField
```

These adapters exist only to prove value-bus semantics. They are not real effect ports.

## Validation/execution behavior locked

- Descriptor output ids validate through `EffectDescriptor.outputs`.
- Node output ids validate as graph-local `GraphValueId` keys.
- Nodes can publish from descriptor-declared effect outputs.
- Nodes can re-emit resolved inputs.
- Unknown graph values are rejected at contract validation.
- Graph value kind mismatches are rejected.
- Undeclared effect-output publication is rejected.
- Unknown input re-emission is rejected.
- `ValueSource::GraphValue` is rejected outside node-input validation contexts.
- Sequence execution updates the value bus after each node.
- Later sequence nodes can consume prior node outputs.
- One output can feed multiple later node inputs.
- Spatial `cellField` values can drive per-cell dim factors.
- Parallel branches receive the same value-bus snapshot.
- Parallel siblings cannot consume each other’s branch-local outputs.
- Parallel outputs become visible after join.
- Same-output conflicts use explicit `GraphValueMergePolicy`.

## Tests added/updated

```text
crates/tui-vfx-contract/tests/test_graph_contract.rs
crates/tui-vfx-contract/tests/test_schema_generation.rs
crates/tui-vfx-next/tests/test_graph_execution_values.rs
crates/tui-vfx-next/tests/support/mod.rs
```

Coverage includes:

- sequence node consumes prior node output
- one output feeds multiple later inputs
- node re-emits resolved input as output
- unknown graph value rejected
- graph value kind mismatch rejected
- spatial field output drives cell-varying input
- parallel sibling cannot consume sibling output
- parallel outputs visible after join
- parallel output conflict child-order last-writer-wins
- parallel output conflict can error if policy requires
- branch-local output does not leak before join
- descriptor rejects node output not declared by effect
- node output from input rejects unknown input
- node output from input preserves kind and shape for re-emitted cell fields
- existing G2 linear execution tests still pass
- existing G3 parallel surface merge tests still pass
- contract schemas are current

## Docs updated

```text
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/ARCH-RESP-TO-PHASE_G3.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-feature-contract-checklist.md
docs/v3.1-surface-contract.md
docs/INDEX.md
```

## Deliberately not added

```text
source recipe authoring schema
canonical recipe compiler
runtime ParameterStore / SignalStore
F2 BindingSpec execution
live override precedence
direct node/effect-input binding targets
phase graph / trigger / dwell engine
visibility predicate execution
loopback / demo signal execution
asset / procedural source system
studio manifest / controls
legacy migration / aliases
real effect ports
```

## Verification status

Final phase verification passed:

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
```

Final wrap verification additionally ran:

```text
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
forbidden legacy crate grep over tui-vfx-contract and tui-vfx-next
git diff --check
```

## Worktree note

The following pre-existing unrelated files remain outside Phase G4 scope and should not be staged into the G4 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_G4_STATUS.md</FILE> - <DESC>Phase G4 node I/O and graph value bus implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
