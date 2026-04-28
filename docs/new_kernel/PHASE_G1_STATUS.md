<!-- <FILE>docs/new_kernel/PHASE_G1_STATUS.md</FILE> - <DESC>Phase G1 canonical graph container implementation status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G1 wrap: summarize canonical graph DTOs, validation, schemas, docs, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase G1 status for architect handoff.</CLOG> -->

# Phase G1 Status — Canonical Node Graph Container

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: G1 — Canonical Node Graph Container

## Summary

Phase G1 adds the smallest canonical graph/container contract to `tui-vfx-contract`.

Current answer: **the contract crate can now represent a canonical graph with public parameters, host signals, F2 parameter-target bindings, declared effect descriptors, stable nodes, `ValueSource` node inputs, and deterministic node order; validation catches unknown descriptors/inputs/parameters/signals, source kind mismatches, unsupported scope/write policy requests, invalid bindings, and invalid order.**

This remains a declarative contract layer only. It does not execute a graph, add runtime stores, introduce source recipe authoring syntax, compile recipes, expand templates, add direct node/effect-input bindings, add studio metadata, or port real effects.

## Implemented contract DTOs

```text
GraphId
NodeId
NodeSpec
GraphSpec
```

`NodeSpec` carries:

```text
id: NodeId
effect: EffectId
inputs: BTreeMap<EffectInputId, ValueSource>
scope: Option<ScopeSpec>
cell_write_policy: Option<CellWritePolicy>
role_write_policy: Option<RoleWritePolicy>
```

`GraphSpec` carries:

```text
id: GraphId
version: String
parameters: BTreeMap<ParameterId, ParameterSpec>
signals: BTreeMap<SignalId, SignalSpec>
bindings: Vec<BindingSpec>
effects: BTreeMap<EffectId, EffectDescriptor>
nodes: BTreeMap<NodeId, NodeSpec>
order: Vec<NodeId>
```

## Validation locked

`GraphSpec::validate()` now checks:

- graph, parameter, signal, and node id shape where the id types expose validators
- parameter/signal map-key consistency with nested spec ids
- F2 binding validation, including unknown/non-bindable parameter targets
- node map-key consistency with nested `NodeSpec.id`
- node effect id exists in declared effects and effect map keys match descriptor ids
- descriptor-local input ids exist on the selected descriptor
- `ValueSource` kind matches the target `EffectInputSpec.value.kind`
- parameter and signal references resolve through the graph declarations
- numeric maps remain numeric-only through existing `ValueSource` rules
- required descriptor inputs with no default are supplied by the node
- requested scopes and write policies are supported by the effect descriptor
- deterministic order references known nodes, contains no duplicates, and covers every declared node

## New schema roots

```text
schemas/v3.1/contract/graph.schema.json
schemas/v3.1/contract/node.schema.json
```

Existing contract schema fixtures remain current.

## Tests added

```text
crates/tui-vfx-contract/tests/test_graph_contract.rs
crates/tui-vfx-contract/tests/test_graph_policy_contract.rs
crates/tui-vfx-contract/tests/support/mod.rs
```

Coverage includes:

- valid graph with literal input
- valid graph with parameter input
- valid graph with signal input
- unknown effect id rejection
- unknown input id rejection
- missing required input rejection
- input kind mismatch rejection
- unknown parameter source rejection
- unknown signal source rejection
- parameter map key mismatch rejection
- unsupported scope rejection
- unsupported cell write policy rejection
- unsupported role write policy rejection
- unknown ordered node rejection
- duplicate ordered node rejection
- node missing from order rejection
- accepted F2 parameter binding
- binding to unknown parameter rejection
- binding to non-bindable parameter rejection
- no runtime/store/compiler/trigger leak guard

## Docs updated

```text
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-feature-contract-checklist.md
docs/v3.1-surface-contract.md
docs/INDEX.md
```

## Deliberately not added

```text
runtime graph execution
runtime ParameterStore / SignalStore
live override precedence
direct node/effect-input binding targets
source recipe authoring schema
recipe compiler implementation
template expansion
studio manifest / controls
phase graph / trigger engine
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
cargo test --workspace
cargo tree -p tui-vfx-contract
cargo tree -p tui-vfx-next
forbidden legacy crate grep over tui-vfx-contract and tui-vfx-next
git diff --check
```

## Worktree note

The following pre-existing unrelated files remain outside Phase G1 scope and should not be staged into the G1 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_G1_STATUS.md</FILE> - <DESC>Phase G1 canonical graph container implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
