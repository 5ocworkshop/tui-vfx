<!-- <FILE>docs/new_kernel/PHASE_G1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G1 wrap: report canonical graph container and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase G1 architect memo in the established status-memo style.</CLOG> -->

# Phase G1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: G1 — Canonical Node Graph Container

## Executive summary

Phase G1 implements the canonical graph/container layer recommended in `ARCH-RESP-TO-PHASE_F2.md`.

Current answer: **`tui-vfx-contract` can now describe a canonical graph with stable graph/node identity, public parameters, host signals, F2 parameter-target bindings, declared effect descriptors, descriptor-backed nodes, `ValueSource` node inputs, and deterministic order. Graph validation ties these declarations together without crossing into runtime execution.**

The implementation intentionally stops before source recipe authoring syntax, recipe compilation, runtime graph execution, runtime stores, live override precedence, direct node/effect-input binding targets, phase/trigger semantics, template expansion, studio metadata, migration, and real effect ports.

## Current implementation state

Contract crate:

```text
crates/tui-vfx-contract
```

New contract DTOs:

```text
GraphId
NodeId
NodeSpec
GraphSpec
```

Expanded validation vocabulary includes graph/node errors for:

```text
InvalidGraphId
InvalidNodeId
ParameterIdMismatch
SignalIdMismatch
NodeIdMismatch
EffectIdMismatch
UnknownEffect
UnknownNodeInput
MissingRequiredNodeInput
UnknownOrderNode
DuplicateOrderNode
NodeMissingFromOrder
```

New schema roots:

```text
schemas/v3.1/contract/graph.schema.json
schemas/v3.1/contract/node.schema.json
```

## Goal-by-goal status against the G1 recommendation

| G1 goal / constraint | Current status |
|---|---|
| Add canonical graph/container DTO | **Done.** `GraphSpec` contains id, version, parameters, signals, bindings, effects, nodes, and order. |
| Keep it separate from source authoring recipe syntax | **Done.** It is documented as the canonical post-compilation graph shape, not an authoring schema. |
| Add stable node identity | **Done.** `NodeId` exists, validates an ASCII identifier-like shape, and is used in node maps and order. |
| Add stable graph identity | **Done.** `GraphId` exists and validates the same constrained id shape. |
| Add node DTO | **Done.** `NodeSpec` references an `EffectId` and supplies `EffectInputId -> ValueSource` inputs plus optional scope/write policies. |
| Validate node effect ids | **Done.** Unknown effect ids are rejected. |
| Validate node inputs against descriptors | **Done.** Unknown input ids are rejected and source kinds must match `EffectInputSpec.value.kind`. |
| Validate missing required inputs | **Done.** Descriptor inputs with no default must be present on the node. |
| Validate parameter/signal refs | **Done.** Reuses `ValueSource` resolution against graph parameter/signal maps. |
| Keep maps numeric-only | **Done.** Reuses F2 `ValueSource::Map` validation. |
| Validate scope/write support | **Done.** Reuses `EffectDescriptor::validate_scope`, `validate_cell_write_policy`, and `validate_role_write_policy`. |
| Carry F2 bindings in the graph | **Done.** `GraphSpec.bindings` is `Vec<BindingSpec>` and validates with existing F2 parameter-target rules. |
| Keep direct node-input bindings deferred | **Done.** `BindingSpec` remains parameter-target only; node inputs are plain `ValueSource`. |
| Validate deterministic order | **Done.** Order rejects unknown nodes, duplicates, and declared nodes missing from order. |
| Add checked schemas | **Done.** Graph and node schema fixtures are generated and checked. |
| Avoid runtime/recipe/studio scope | **Respected.** No runtime execution, stores, recipe compiler, source recipe schema, template expansion, studio controls, migration, or real effect ports were added. |

## Key decisions

### GraphSpec is canonical, not authoring syntax

I used `GraphSpec` rather than `RecipeDocument` to preserve your distinction between the canonical runtime graph document shape and future source authoring recipes. This lets the contract prove descriptor/node/input compatibility without prematurely designing the public recipe schema.

### Node inputs are ValueSource maps, not bindings

`NodeSpec.inputs` is a `BTreeMap<EffectInputId, ValueSource>`. That gives nodes literal, parameter, signal, and numeric map inputs immediately while keeping `BindingSpec` parameter-target only. Direct node/effect-input binding remains deferred until the graph model and later runtime binding story are stable.

### Bindings are a vector, not BindingId-keyed yet

Because F2 did not introduce `BindingId`, G1 keeps graph bindings as `Vec<BindingSpec>`. This avoids creating a new identity namespace before there is a concrete need for binding identity.

### Deterministic order covers every declared node

The architect-required checks were unknown order references and duplicates. I also made validation reject declared nodes missing from order, because otherwise the graph could contain unordered executable nodes while still claiming deterministic order.

## What deliberately was not added

Phase G1 does not add:

```text
runtime graph execution
runtime ParameterStore / SignalStore
live override precedence execution
direct node/effect-input bindings
source recipe authoring schema
recipe compiler implementation
template expansion
studio manifest / controls
phase graph / trigger engine
legacy migration / aliases
real effect ports
```

## Verification evidence

Full required verification passed and will be recorded in the commit trailer:

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

## Request for next assignment

Please review Phase G1 as the canonical graph-container lock point and advise the next phase.

My expected next candidates are either:

```text
Phase G2 — optional graph execution proof in tui-vfx-next
```

or:

```text
Phase H — strict canonical recipe v3.1 schema/compiler
```

depending on whether you want an execution proof for the canonical graph before moving up to recipe authoring.

<!-- <FILE>docs/new_kernel/PHASE_G1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase G1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
