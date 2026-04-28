<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G2.md</FILE> - <DESC>Architect response approving Phase G2 and assigning Phase G3 topology semantics</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G3 intake: preserve architect guidance for topology, parallel snapshot, and channel-aware merge.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture architect response as phase input.</CLOG> -->

# G2 verdict

**Approved.** G2 did exactly what it should: it proved that a validated canonical `GraphSpec` can execute linearly with toy proof adapters, `ValueSource` resolution, graph order, prior-node visibility, and existing surface scope/write semantics.

However, after reviewing the additional recipes, I would **not** proceed to H1 yet.

The next phase should be:

```text
Phase G3 — Topology / Parallel Snapshot / Channel-Aware Merge Semantics
```

Then:

```text
Phase G4 — Node I/O / Hint Value Bus
```

Only after those should we move to canonical recipe document schema.

---

# Why G3 is now necessary

The decisive examples are:

```text
complex_parallel_overlap_conflict_snapshot.json
v3_io_parallel_merge_shader.json
v3_cross_family_sequence_disjoint.json
```

These show that `parallel` is not just a convenient authoring wrapper around a linear list. It has semantic meaning:

```text
parallel branches read the same pre-parallel snapshot
branch outputs merge at the join
overlapping writes resolve deterministically
independent channel writes can compose
```

A flat `GraphSpec.order` cannot faithfully model that. If we linearize too early, branch B sees branch A’s writes, which contradicts the current recipe semantics.

The most important new contract requirement is **channel-aware merge**.

For example, `complex_parallel_overlap_conflict_snapshot.json` says:

```text
filter branch writes foreground
shader branch writes background
later dim branch wins foreground conflict
background shader remains active
```

That cannot be represented as simple whole-cell “later branch wins,” because whole-cell merge would let the later foreground dim overwrite the earlier background shader result with stale background from the branch snapshot.

So G3 needs to prove:

```text
parallel branches produce deltas
deltas know which cell channels they wrote
merge applies channel writes in child order
same-channel conflicts resolve by policy
different-channel writes compose
```

---

# Updated roadmap

```text
+====================================================================================+
| UPDATED ROADMAP                                                                      |
+====================================================================================+

  [DONE]   A   — Semantic surface contract
  [DONE]   B   — Sampled-source semantics
  [DONE]   C   — Ordered pipeline/pass semantics
  [DONE]   D0  — Schema/reference backfill
  [DONE]   D1  — Scene / element / layer composition semantics
  [DONE]   D2  — Template composition design
  [DONE]   D3  — Contract/engine boundary
  [DONE]   E0  — Physical contract split
  [DONE]   E1  — Minimal effect descriptor model
  [DONE]   F1  — ValueKind / Value / EffectInputSpec
  [DONE]   F2  — ValueSource / ParameterSpec / SignalSpec / BindingSpec
  [DONE]   G1  — Canonical Node Graph Container
  [DONE]   G2  — Canonical Graph Execution Proof, linear

  [NEXT]   G3  — Topology / parallel snapshot / channel-aware merge semantics

  [NEXT+]  G4  — Node I/O / hint value bus

  [LATER]  H1  — Canonical Recipe Document Schema
  [LATER]  H2  — Source recipe / template expansion implementation
  [LATER]  I   — Phase / trigger / dwell engine
  [LATER]  J   — First real effect ports

+====================================================================================+
```

---

# Additional recipe findings

## 1. Node I/O is broader than one special case

These examples all reinforce the need for a real node-output value bus:

```text
v3_io_scalar_filter.json
v3_io_radial_twist_spiral_chain.json
v3_io_authoring_ladder_toast_glow_chain.json
v3_cross_family_sequence_disjoint.json
scene_layer_io_filter_shader.json
```

Patterns present:

```text
one output consumed by multiple later nodes
filter input re-emitted as output
sampler output drives another sampler
sampler output drives shader intensity
layer-local pipelines use the same I/O substrate as root pipelines
parallel branch output becomes visible after join
```

This is G4, not G3, because it builds on topology and merge semantics.

## 2. `binds` and `io.inputs` need one canonical form

`v3_cross_family_sequence_disjoint.json` has:

```json
"binds": { "amplitude": "cross_wave" }
```

while other recipes use:

```json
"io": {
  "inputs": [
    { "input": "factor", "hint": "dim_factor", "kind": "scalar" }
  ]
}
```

Canonical v3.1 should pick one strict form. I would not preserve both in canonical schema.

Recommendation:

```text
Canonical v3.1:
    node input source is ValueSource

Source recipe compiler/migrator:
    may accept legacy "binds" and lower to canonical ValueSource/Hints
```

## 3. Assets and procedural sources are real, but later

The flag examples show:

```text
requires_assets
asset tokens like "{{ flag_art }}"
procedural sources
source_id
source params
asset format contracts
```

This should eventually become a contract layer, probably after G4 and before full source authoring compiler work.

Possible future phase:

```text
H0 or H2a — Source / Asset / Procedural Source Contract
```

But do not put it into G3.

## 4. Visibility is a predicate, not just a bool field

Scene examples show:

```json
"visibility": {
  "predicate": "show_spinner"
}
```

and:

```json
"visibility": {
  "predicate": "show_detail"
}
```

This is a future predicate/trigger/value-source problem. It should probably become:

```text
visibility: ValueSource<boolean>
```

or a small predicate AST later.

Do not add this to G3.

## 5. Event-driven dwell is a real phase/trigger requirement

The dwell examples show:

```text
dwell_until_binding
dwell_fallback_ms
latched truthy behavior
```

This confirms the earlier trigger-engine plan. It belongs in Phase I or a dedicated phase/trigger phase.

Do not add this to G3 or G4.

## 6. Motion and resize are host/runtime contract concerns

These examples show important future needs:

```text
resize_preserve_phase_chain.json
motion_figure_eight_infinity.json
scene_layer_follow_lag.json
toast_shadow_edge_crossing.json
```

Findings:

```text
host owns resize events
phase/sample/runtime state should survive resize
scene layer placement may depend on sibling motion
edge crossing can affect border/shadow visibility
motion route aliases exist today
```

Do not add motion to G3. But later, we should separate:

```text
layout/placement contract
motion path contract
host resize/runtime contract
phase clock contract
```

Also note: `infinity` is an alias for `figure_eight` in current recipes. Canonical v3.1 should not allow aliases; migration/source authoring may lower it.

---

# G3 target model

G3 should add topology semantics without touching recipe authoring or node I/O yet.

```text
+==================================================================================================+
|                   PHASE G3 — TOPOLOGY / PARALLEL SNAPSHOT / MERGE                                |
+==================================================================================================+

        +-------------------------------+
        | GraphSpec                     |
        |                               |
        |  nodes                        |
        |  order                        |
        |  topology / execution plan    |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | GraphStep                     |
        |                               |
        |  node                         |
        |  sequence                     |
        |  parallel                     |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Sequence Semantics            |
        |                               |
        |  child 1 reads current        |
        |  child 1 writes next          |
        |  child 2 reads child 1 output |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Parallel Semantics            |
        |                               |
        |  all branches read snapshot   |
        |  each branch produces delta   |
        |  join merges deltas           |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Channel-Aware Merge           |
        |                               |
        |  glyph                        |
        |  foreground                   |
        |  background                   |
        |  modifiers                    |
        |  modifier alpha               |
        |  role                         |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Final Surface + Diagnostics   |
        +-------------------------------+

+==================================================================================================+
```

---

# G3 should lock

```text
+====================================================================================+
|                                  PHASE G3 LOCK TARGETS                              |
+====================================================================================+

  TOPOLOGY
      [LOCK] A canonical graph can express execution topology.
      [LOCK] Topology is distinct from source recipe syntax.
      [LOCK] Topology supports node, sequence, and parallel.

  SEQUENCE
      [LOCK] Sequence children execute in order.
      [LOCK] Later sequence children see earlier sequence writes.

  PARALLEL
      [LOCK] Parallel children all read the same pre-parallel snapshot.
      [LOCK] Parallel children do not see sibling branch writes.
      [LOCK] Parallel branch order is still deterministic for merge.

  DELTAS
      [LOCK] Parallel branches produce deltas, not just final surfaces.
      [LOCK] Deltas know which cell channels were written.

  MERGE
      [LOCK] Different-channel writes compose.
      [LOCK] Same-channel conflicts resolve deterministically.
      [LOCK] Default conflict policy is explicit.
      [LOCK] Authored child order can be the default last-writer-wins policy.

  DIAGNOSTICS
      [LOCK] Merge/conflict diagnostics can identify graph path and branch index.
      [LOCK] Conflict diagnostics are structured enough to test.

  BOUNDARY
      [LOCK] No node I/O hint bus yet.
      [LOCK] No phase/trigger semantics.
      [LOCK] No recipe compiler.
      [LOCK] No source authoring aliases.
      [LOCK] No real effect ports.

+====================================================================================+
```

---

# Recommended G3 contract additions

Names can vary, but I would expect something like:

```rust
GraphStep
GraphTopology
ParallelMergePolicy
MergeConflictPolicy
SurfaceDelta
CellDelta
```

A possible shape:

```rust
pub enum GraphStep {
    Node {
        node: NodeId,
    },
    Sequence {
        children: Vec<GraphStep>,
    },
    Parallel {
        children: Vec<GraphStep>,
        merge_policy: ParallelMergePolicy,
    },
}
```

```rust
pub enum ParallelMergePolicy {
    ChildOrderLastWriterWins,
    ErrorOnSameChannelConflict,
}
```

`GraphSpec` can add topology without breaking the existing linear proof too hard:

```rust
pub struct GraphSpec {
    ...
    pub order: Vec<NodeId>,

    /// Optional explicit execution topology.
    /// When omitted, `order` is interpreted as a linear sequence.
    pub topology: Option<GraphStep>,
}
```

That lets G1/G2 linear graphs remain valid while G3 proves topology.

---

# The subtle but important part: channel deltas

This should not be modeled as “branch returns final surface only.”

For parallel merge to match your recipes, each branch needs to report what it wrote:

```text
CellDelta at (x, y):
    glyph written?
    foreground written?
    background written?
    modifiers written?
    modifier alpha written?
    role written?
```

Then merge is:

```text
for each child branch in authored order:
    for each cell delta:
        for each written channel:
            if prior branch wrote same channel:
                resolve conflict by policy
            write channel into join surface
```

This is what preserves the background shader while allowing the later dim branch to win foreground.

---

# G3 tests to require

```text
sequence_later_child_sees_earlier_child_write

parallel_children_read_same_snapshot

parallel_sibling_does_not_see_prior_sibling_write

parallel_different_channel_writes_compose

parallel_same_channel_conflict_child_order_last_wins

parallel_same_channel_conflict_can_error_if_policy_requires

parallel_nested_sequence_branch_reads_own_prior_step

parallel_merge_preserves_roles_by_channel_policy

topology_rejects_unknown_node

topology_rejects_duplicate_node_reference_unless_reuse_is_explicitly_supported

topology_leaves_cover_declared_nodes_or_linear_order_fallback_is_used

linear_order_still_executes_when_topology_absent

topology_schema_is_current
```

---

# G4 will handle the I/O examples

G4 should then address the `io` / hint patterns.

```text
+====================================================================================+
|                            PHASE G4 — NODE I/O / HINT BUS                            |
+====================================================================================+

  Required semantics from recipes:
      sampler emits scalar hint
      filter consumes scalar hint
      filter re-emits input/payload value
      shader consumes same hint
      one hint can fan out to multiple consumers
      parallel branch hint becomes visible after join
      branch-local hint is not visible before join
      hint kind mismatches are rejected
      unknown hint is rejected

+====================================================================================+
```

G4 should likely introduce:

```text
OutputId or HintId
NodeOutputSpec
NodeInputBinding / OutputValueSource
GraphValueState
ValueDelta
ValueMergePolicy
```

But not yet in G3.

---

# Immediate next assignment: Phase G3

Here is the prompt I would give the next agent.

```text
You are working in the tui-vfx Rust workspace.

Phases A–G2 built the v3.1 contract foundation:
- A: semantic surface contract
- B: sampled-source semantics
- C: ordered pipeline/pass semantics
- D0: schema/reference backfill
- D1: scene / element / layer composition semantics
- D2: template composition design
- D3: contract/engine boundary
- E0: physical contract split
- E1: minimal effect descriptor model
- F1: ValueKind / Value / EffectInputSpec
- F2: ValueSource / ParameterSpec / SignalSpec / BindingSpec
- G1: canonical GraphSpec / NodeSpec validation
- G2: linear GraphSpec execution proof in tui-vfx-next

Your task is Phase G3: Topology / Parallel Snapshot / Channel-Aware Merge Semantics.

Motivation:
Existing debug recipes prove that `parallel` is semantic, not just authoring sugar. Parallel children read the same pre-parallel snapshot. Their outputs merge at a join. Same-channel conflicts resolve deterministically by child order, while different-channel writes compose.

Primary question:
Can the canonical graph model represent and execute node/sequence/parallel topology while preserving snapshot isolation and channel-aware merge semantics?

Hard constraints:
- Add stable topology DTOs to `tui-vfx-contract`.
- Add proof execution support to `tui-vfx-next`.
- Do not implement node I/O hint bus yet.
- Do not implement phase/trigger semantics.
- Do not implement source recipe schema/compiler.
- Do not implement runtime ParameterStore / SignalStore.
- Do not implement live override precedence.
- Do not add direct node/effect-input bindings.
- Do not add studio manifest or studio controls.
- Do not port real effects.
- Do not add legacy aliases.
- Preserve D0 schema/reference rules for public contract DTOs.
- Preserve existing G1/G2 linear graph behavior.

Required contract concepts:
- GraphStep or equivalent topology enum
- node step
- sequence step
- parallel step
- explicit merge/conflict policy
- schema-backed topology root or GraphSpec schema update
- channel-aware branch delta model, if public contract-facing

Recommended GraphSpec change:
- Add optional topology/execution field.
- When topology is absent, existing `order` is interpreted as linear sequence.
- When topology is present, topology leaves must reference declared nodes and must be deterministic.

Parallel semantics:
- All parallel children read the same input snapshot.
- A child branch may contain a sequence.
- Branches produce deltas against the snapshot.
- Join merges branch deltas in authored child order.
- Different cell-channel writes compose.
- Same cell-channel conflicts resolve by explicit policy.
- Default proof policy may be child-order-last-writer-wins.
- Optional error-on-conflict policy may be added if small.

Cell channels:
- glyph
- foreground
- background
- modifiers
- modifier alpha
- role

Do not merge whole cells blindly. Parallel merge must be channel-aware.

Proof execution:
- Reuse G2 proof adapters where possible.
- Extend proof execution so adapters/stages can report written channels.
- If a proof adapter writes only foreground, it must not overwrite background during parallel merge.
- If two branches write foreground for the same cell, conflict policy applies.
- Later sequence nodes still see prior sequence writes.
- Parallel sibling branches must not see each other’s writes before join.

Validation requirements:
- Topology node references must exist.
- Duplicate node references are rejected unless explicit reuse is intentionally designed; default should reject.
- Topology leaves should cover declared nodes when topology is present.
- Existing order validation should still work for linear graphs.
- Existing graph validation must still run before execution.

Tests:
Add tests covering:
- linear graph execution still works when topology is absent
- sequence later child sees earlier child write
- parallel children read the same snapshot
- parallel sibling does not see prior sibling write
- parallel different-channel writes compose
- parallel same-channel conflict child-order-last-wins
- optional error-on-conflict policy rejects same-channel conflict, if implemented
- nested sequence inside parallel branch sees its own prior write
- topology rejects unknown node
- topology rejects duplicate node reference
- topology leaves cover declared nodes
- topology schema is current
- graph schema is current

Docs to update:
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
    cargo tree -p tui-vfx-contract
    cargo tree -p tui-vfx-next
    grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-contract crates/tui-vfx-next
    git diff --check

Deliverables:
- Topology DTOs in tui-vfx-contract
- Checked schema updates
- Proof topology execution in tui-vfx-next
- Tests proving sequence/parallel/snapshot/merge semantics
- Updated docs
- docs/new_kernel/PHASE_G3_STATUS.md
- docs/new_kernel/PHASE_G3_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase G3 is done when the canonical graph can express and proof-execute node, sequence, and parallel topology; parallel branches read the same snapshot; branch outputs merge channel-by-channel; different-channel branch writes compose; same-channel conflicts resolve by explicit policy; existing linear G2 execution remains valid; schemas are current and described; and no node I/O hint bus, recipe compiler, phase engine, runtime store, studio manifest, or real effect port leaks into the phase.
```

---

# Final call

Do **not** proceed to H1 yet.

Proceed with:

```text
Phase G3 — Topology / Parallel Snapshot / Channel-Aware Merge Semantics
```

Then:

```text
Phase G4 — Node I/O / Hint Value Bus
```

The additional examples make that the safer path.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G2.md</FILE> - <DESC>Architect response approving Phase G2 and assigning Phase G3 topology semantics</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
