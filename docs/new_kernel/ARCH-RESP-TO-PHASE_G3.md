<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G3.md</FILE> - <DESC>Architect response approving Phase G3 and assigning Phase G4 node I/O value bus</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G4 intake: architect response defining graph-local node output/value bus requirements.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect Phase G3 response and Phase G4 assignment with metadata.</CLOG> -->

G3 looks architecturally right. **I approve the G3 direction**, with final approval contingent on the memo’s pending full verification actually being run and recorded before commit.

The important lock is this:

```text
linear graph execution      -> proven in G2
topology execution          -> proven in G3
parallel snapshot isolation -> proven in G3
channel-aware merge         -> proven in G3
node I/O value bus          -> still deferred
```

That is exactly the right boundary. Do **not** move to H1 yet. The next phase should be **G4 — Node I/O / Hint Value Bus**.

One clarification for G4: the examples strongly suggest that “scalar hint” is not always a single global scalar. In many recipes, it is effectively a **spatial scalar field** evaluated per cell, such as `sample_norm_x` driving a filter factor or shader intensity. G4 should not accidentally model every hint as one frame-global `number`.

# Phase G4 — Node I/O / Hint Value Bus

```text
+====================================================================================+
|                         PHASE G4 — NODE I/O / HINT VALUE BUS                         |
+====================================================================================+

  Goal:
      Let nodes produce typed value outputs and let later nodes consume them,
      across sequence and parallel topology, without adding runtime stores,
      recipe compiler, studio, phases, triggers, or real effect ports.

  Why:
      Existing recipes use producer -> consumer chains:

          spatial signal -> dim filter
          dim filter -> diffusion shader
          sampler output -> sampler input
          filter output -> mask input
          parallel branch output -> post-join shader

      These cannot be represented by parameters/signals alone.

+====================================================================================+
```

## What G4 should lock

```text
1. Node output identity
   A node can publish named values into a graph-local value bus.

2. Typed output values
   Outputs have a declared kind, e.g. scalar/number initially.

3. Output shape/cardinality
   Distinguish at least:
       frame/global value
       per-cell/spatial field value

4. Input consumption
   A later node can use a prior output as an input source.

5. Fan-out
   One output can feed multiple later node inputs.

6. Sequence visibility
   Sequence child N+1 can consume outputs from child N.

7. Parallel isolation
   Parallel siblings cannot consume each other’s branch-local outputs.

8. Parallel join
   Outputs produced in parallel branches become visible after the join.

9. Output conflict policy
   If two parallel branches publish the same output id, behavior is explicit:
       last-writer-wins by child order
       or error-on-conflict

10. Descriptor validation
   Descriptors declare what outputs they can produce, or nodes explicitly
   re-emit known input values.

11. No runtime binding execution yet
   G4 still does not apply F2 BindingSpec. Bindings remain validation-only
   until the runtime-store phase.
```

# Recommended contract additions

I would avoid canonizing the old word `hint` as the primary contract term. Treat it as legacy/source-authoring vocabulary that lowers to a stricter contract name.

Suggested canonical vocabulary:

```text
GraphValueId
GraphValueKind
GraphValueShape
EffectOutputId
EffectOutputSpec
NodeOutputSpec
NodeOutputSource
GraphValueMergePolicy
```

For example:

```text
GraphValueId:
    "dimFactor"
    "portalField"
    "checkerSize"
    "layerShade"

GraphValueKind:
    number / scalar initially
    later maybe boolean, color, vec2, rect, etc.

GraphValueShape:
    frameValue       -> one value for the node/frame
    cellField        -> value may vary by sampled/destination cell
```

That `GraphValueShape` distinction is important because these existing recipes:

```text
sample_norm_x -> dim factor
sample_norm_x -> diffusion intensity
```

are not just “one number.” They behave like fields.

## Descriptor outputs

E1 descriptors currently describe identity, domain, inputs, cell access, scope/write support, and lifecycle. G4 should add output capability to descriptors:

```text
EffectDescriptor.outputs: BTreeMap<EffectOutputId, EffectOutputSpec>
```

An output spec should answer:

```text
id
kind
shape
description
```

Example conceptual descriptor:

```text
effect: proof.spatialSignal
outputs:
    value:
        kind: number
        shape: cellField
```

## Node outputs

A node should be able to publish graph values in one of two basic ways:

```text
1. fromEffectOutput
   Adapter/effect computes an output declared by its descriptor.

2. fromInput
   Node republishes a resolved input value.
   This covers recipes like:
       dim filter consumes factor
       dim filter outputs source: factor
```

Canonical examples:

```text
node "field":
    effect: proof.spatialSignal
    outputs:
        portalField:
            source: effectOutput("value")

node "dim":
    effect: proof.dim
    inputs:
        factor: graphValue("portalField")
    outputs:
        maskCellSize:
            source: input("factor")
```

Legacy source examples like:

```json
{ "hint": "mask_cell_size", "kind": "scalar", "source": "factor" }
```

should eventually lower to the canonical `NodeOutputSpec`.

# ValueSource update

G4 probably needs one new source variant:

```text
ValueSource::GraphValue {
    id: GraphValueId,
    fallback: Option<Value>
}
```

or, if you want to keep F2 `ValueSource` pure, introduce a node-input-only wrapper. I lean toward adding the source variant and validating it contextually:

```text
Allowed in:
    NodeSpec.inputs

Rejected in:
    ParameterSpec default/source contexts
    SignalSpec contexts
```

This keeps the expression vocabulary unified while still enforcing where graph-local outputs are legal.

# G4 execution semantics

```text
+====================================================================================+
|                          G4 EXECUTION SEMANTICS                                      |
+====================================================================================+

  Sequence:
      bus starts with current graph values
      node A consumes bus
      node A publishes value deltas
      bus updates
      node B can consume node A outputs

  Parallel:
      all branches receive the same surface snapshot
      all branches receive the same value-bus snapshot
      branch A outputs are not visible to branch B
      branch B outputs are not visible to branch A
      join merges surface deltas using G3 rules
      join merges value deltas using G4 value merge rules
      downstream steps can consume joined values

+====================================================================================+
```

# G4 tests to require

```text
sequence_node_can_consume_prior_node_output

one_output_can_feed_multiple_later_inputs

node_can_reemit_resolved_input_as_output

graph_value_source_rejects_unknown_output

graph_value_source_rejects_kind_mismatch

spatial_field_output_can_drive_cell_varying_input

parallel_sibling_cannot_see_other_sibling_output

parallel_outputs_visible_after_join

parallel_output_conflict_child_order_last_wins

parallel_output_conflict_can_error_if_policy_requires

branch_local_output_does_not_leak_before_join

descriptor_rejects_node_output_not_declared_by_effect

node_output_from_input_rejects_unknown_input

node_output_from_input_preserves_kind_and_shape

existing_g2_linear_execution_still_passes

existing_g3_parallel_surface_merge_still_passes

schema_generation_is_current
```

# G4 non-goals

Keep these explicitly out:

```text
runtime ParameterStore / SignalStore
live override precedence
F2 BindingSpec execution
direct node/effect-input binding targets
phase / trigger / dwell engine
visibility predicate execution
loopback/demo signal execution
procedural source system
asset loading
recipe compiler
source authoring schema
studio manifest
legacy migration
real effect ports
```

# G4 prompt for the implementer

```text
You are working in the tui-vfx Rust workspace.

Phases A–G3 built the v3.1 contract foundation:
- A/B/C: surface, sampled-source, ordered pipeline semantics
- D0/D1/D2/D3: schema/reference, scene, template design, boundary
- E0/E1: physical contract split and effect descriptors
- F1/F2: typed values, inputs, parameters, signals, bindings
- G1/G2: canonical graph container and linear graph execution proof
- G3: topology, parallel snapshot isolation, channel-aware surface merge

Your task is Phase G4: Node I/O / Hint Value Bus.

Motivation:
Existing debug recipes use node-local I/O chains:
- a spatial-signal producer emits a scalar hint consumed by a dim filter and shader
- a filter consumes an input and re-emits it for a mask
- a sampler output drives another sampler
- a parallel branch emits a value visible only after the parallel join
- layer-local pipelines use the same I/O substrate as root pipelines

Primary question:
Can the canonical graph model represent and proof-execute typed node outputs and downstream node input consumption across sequence and parallel topology?

Hard constraints:
- Add stable node-output/value-bus DTOs to `tui-vfx-contract`.
- Add proof execution support to `tui-vfx-next`.
- Preserve all G2/G3 behavior.
- Do not implement recipe source syntax.
- Do not implement a recipe compiler.
- Do not execute F2 BindingSpec.
- Do not implement runtime parameter/signal stores.
- Do not add live override precedence.
- Do not add phases, triggers, dwell, visibility predicates, loopback, assets, procedurals, studio, migration, or real effects.
- Preserve D0 schema/reference rules.

Required contract concepts:
- graph-local value/output id
- output kind
- output shape/cardinality
- effect output descriptor
- node output declaration
- output source:
    - effect output
    - re-emitted node input
- ValueSource support for consuming graph-local outputs, or an equivalent node-input-only source
- value merge policy for parallel joins

Important modeling requirement:
Do not assume every “scalar hint” is one global number. Some scalar outputs are spatial/cell fields, such as `sample_norm_x`, and must be able to drive per-cell filter/shader inputs.

Execution semantics:
- Sequence updates the value bus after each node.
- Later sequence nodes can consume prior outputs.
- Parallel branches read the same value-bus snapshot.
- Parallel siblings cannot see each other’s branch-local outputs.
- Parallel branch outputs merge at join.
- Downstream steps can consume joined outputs.
- Output id conflicts in parallel are handled by explicit policy.

Proof adapters:
Add only toy proof adapters as needed, such as:
- proof.spatialScalarField
- proof.consumeNumber
- proof.reemitInput
Do not port real effects.

Tests:
Add tests covering:
- sequence node consumes prior output
- one output feeds multiple later inputs
- node re-emits resolved input as output
- unknown graph output rejected
- kind mismatch rejected
- spatial field output can drive cell-varying input
- parallel sibling cannot consume sibling output
- parallel output visible after join
- parallel output conflict child-order last-writer-wins
- optional error-on-conflict policy rejects output conflict
- graph value source is rejected outside allowed node-input context, if applicable
- descriptors reject undeclared output publication
- existing G2 and G3 tests still pass
- schemas are current

Docs:
Update:
- docs/v3.1-architecture-overview.md
- docs/v3.1-contract-boundary.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Verification:
Run:
    cargo fmt --package tui-vfx-contract -- --check
    cargo fmt --package tui-vfx-next -- --check
    cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
    cargo clippy -p tui-vfx-next --all-targets -- -D warnings
    cargo test -p tui-vfx-contract
    cargo test -p tui-vfx-next
    cargo test --workspace
    UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
    cargo test -p tui-vfx-contract --test test_schema_generation
    cargo tree -p tui-vfx-contract
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-contract crates/tui-vfx-next
    git diff --check

Deliverables:
- Contract DTOs for node output/value bus semantics
- Descriptor output support
- Checked schema updates
- Proof graph execution support for value bus
- Tests proving sequence/parallel output visibility and isolation
- Updated docs
- docs/new_kernel/PHASE_G4_STATUS.md
- docs/new_kernel/PHASE_G4_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase G4 is complete when the canonical graph can proof-execute typed node-output chains; sequence nodes can consume prior outputs; one output can fan out to multiple consumers; parallel branches have isolated value-bus snapshots; joined outputs become visible downstream; value conflicts are deterministic or policy errors; spatial scalar field outputs are not reduced to global numbers; schemas are current and documented; and no runtime store, recipe compiler, phase engine, studio, procedural, asset, loopback, or real effect port leaks into the phase.
```

# After G4

After G4, I would pause and decide whether to do:

```text
H0 — Source / Asset / Procedural Source Contract
```

or proceed to:

```text
H1 — Canonical Recipe Document Schema
```

The new recipe examples make assets/procedurals look important, but I would still finish G4 first. G4 is the missing graph capability needed by many examples.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G3.md</FILE> - <DESC>Architect response approving Phase G3 and assigning Phase G4 node I/O value bus</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
