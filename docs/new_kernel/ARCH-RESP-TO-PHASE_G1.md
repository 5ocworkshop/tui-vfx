<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G1.md</FILE> - <DESC>Architect response to Phase G1 canonical graph container status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G2 kickoff: architect approves G1 and directs graph execution proof work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect guidance for Phase G2 canonical graph execution proof.</CLOG> -->

# G1 architect verdict

**Approved.**

Phase G1 locked the canonical graph container at the right level: descriptor-backed nodes, ordered execution identity, parameters/signals/bindings in one validated graph shape, but no runtime execution yet.

The major contract lock is:

```text
GraphSpec is canonical graph shape.
GraphSpec is not source authoring recipe syntax.
GraphSpec is not the runtime engine.
```

That distinction matters.

---

# What G1 locks

```text
+====================================================================================+
|                                  PHASE G1 LOCKS                                      |
+====================================================================================+

  GRAPH CONTAINER
      [LOCK] GraphSpec exists as canonical post-compilation graph shape.
      [LOCK] GraphSpec is distinct from future source recipe authoring syntax.
      [LOCK] GraphSpec owns parameters, signals, bindings, effects, nodes, and order.

  NODE IDENTITY
      [LOCK] NodeId exists.
      [LOCK] NodeSpec references EffectId.
      [LOCK] Node order is deterministic and exhaustive.
      [LOCK] Unknown, duplicate, and missing order nodes are rejected.

  NODE INPUTS
      [LOCK] Node inputs are EffectInputId -> ValueSource.
      [LOCK] Node inputs validate against EffectDescriptor.inputs.
      [LOCK] Unknown input ids are rejected.
      [LOCK] Missing required inputs are rejected.
      [LOCK] Input kind mismatches are rejected.

  GRAPH REFERENCES
      [LOCK] Unknown effects are rejected.
      [LOCK] Unknown parameters are rejected.
      [LOCK] Unknown signals are rejected.
      [LOCK] F2 bindings are validated inside the graph.

  CAPABILITY VALIDATION
      [LOCK] Node scopes validate against descriptor scope support.
      [LOCK] Cell write policies validate against descriptor write support.
      [LOCK] Role write policies validate against descriptor write support.

  BOUNDARY
      [LOCK] No runtime graph execution yet.
      [LOCK] No phase/trigger graph yet.
      [LOCK] No source authoring recipe schema yet.
      [LOCK] No direct node/effect-input binding targets yet.

+====================================================================================+
```

---

# Next phase decision

Proceed with:

```text
Phase G2 — Canonical Graph Execution Proof
```

before Phase H.

Do **not** jump directly to strict canonical recipe schema/compiler yet.

The reason is simple:

```text
G1 proves the canonical graph can be validated.
G2 should prove the canonical graph can drive execution semantics.
H can then build recipe/compiler layers that emit a graph we already know how to validate and execute.
```

If we skip G2, the recipe/compiler phase would target a graph shape that is only structurally valid, not yet proven executable.

---

# What G2 should and should not be

G2 should be a **proof execution layer in `tui-vfx-next`**, not a new runtime system in `tui-vfx-contract`.

```text
tui-vfx-contract:
    owns GraphSpec, NodeSpec, descriptors, values, sources

tui-vfx-next:
    proves GraphSpec can execute over surfaces using toy effect implementations
```

G2 should not become the final runtime engine. It should be the minimal bridge between:

```text
canonical contract graph
    ↓
proof execution against semantic surfaces
```

---

# G2 high-level block diagram

```text
+==================================================================================================+
|                         PHASE G2 — CANONICAL GRAPH EXECUTION PROOF                                |
+==================================================================================================+

        +-------------------------------+
        | GraphSpec                     |
        |                               |
        |  effects                      |
        |  parameters                   |
        |  signals                      |
        |  bindings                     |
        |  nodes                        |
        |  order                        |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Graph Validation              |
        |                               |
        |  reuse G1 validation          |
        |  fail before execution        |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Value Resolution Snapshot     |
        |                               |
        |  literals                     |
        |  parameter defaults/overrides |
        |  signal values/defaults       |
        |  map sources                  |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Proof Effect Adapter Registry |
        |                               |
        |  effect id -> toy executor    |
        |  no real ports                |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Ordered Node Execution        |
        |                               |
        |  current surface              |
        |  node scope/write policies    |
        |  next surface                 |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Final Surface + Diagnostics   |
        +-------------------------------+

+==================================================================================================+
```

---

# G2 should lock

```text
+====================================================================================+
|                                  PHASE G2 LOCK TARGETS                              |
+====================================================================================+

  CONTRACT/ENGINE BRIDGE
      [LOCK] tui-vfx-next can consume GraphSpec from tui-vfx-contract.
      [LOCK] Graph validation runs before execution.
      [LOCK] Execution failure does not mutate output partially, unless explicitly documented.

  PROOF EFFECT ADAPTERS
      [LOCK] EffectId maps to a proof executor in tui-vfx-next.
      [LOCK] Proof executors are not real effect ports.
      [LOCK] Missing proof executor is reported clearly.

  VALUE RESOLUTION
      [LOCK] Literal ValueSource resolves.
      [LOCK] Parameter ValueSource resolves from parameter override or default.
      [LOCK] Signal ValueSource resolves from supplied signal value, fallback, or default.
      [LOCK] Map ValueSource resolves for numeric sources.
      [LOCK] Missing required signal behavior is explicit.

  NODE EXECUTION
      [LOCK] Graph order drives node execution order.
      [LOCK] Later nodes see earlier node cell and role writes.
      [LOCK] Node scope/write policy semantics reuse existing surface/pipeline rules.
      [LOCK] Node inputs influence proof effect behavior.

  BINDINGS
      [LOCK] F2 BindingSpec remains validated but not runtime-applied, unless G2 explicitly defines a one-shot proof application.
      [RECOMMENDATION] Do not apply bindings in G2. Leave binding execution for runtime store phase.

  DIAGNOSTICS
      [LOCK] Execution diagnostics include graph/node identity.
      [LOCK] Unknown proof executor and value-resolution failure are structured enough for tests.

  BOUNDARY
      [LOCK] No source recipe compiler.
      [LOCK] No runtime stores.
      [LOCK] No live override precedence.
      [LOCK] No real effect ports.

+====================================================================================+
```

---

# Important G2 boundary decision: bindings

I recommend this for G2:

```text
GraphSpec.bindings are validated but not executed.
```

Why?

Binding execution belongs to a future runtime parameter store, because it involves precedence and live state:

```text
live override
    >
runtime binding
    >
preset/profile override
    >
recipe default
    >
effect input default
```

G2 should not invent that store early.

Instead, G2 should resolve node `ValueSource`s directly from:

```text
literal values
parameter defaults
explicit parameter values in an execution snapshot
signal values in an execution snapshot
signal fallback/default values
numeric maps
```

That gives us enough proof without crossing into runtime architecture.

---

# Suggested G2 proof API

Names can vary, but the concept should be:

```rust
GraphExecutionContext
    parameter_values: BTreeMap<ParameterId, Value>
    signal_values: BTreeMap<SignalId, Value>

GraphExecutor
    proof effect adapters
    execute(graph, input_surface, context) -> GraphExecutionOutcome

GraphExecutionOutcome
    surface
    diagnostics
    executed_nodes
```

This belongs in:

```text
crates/tui-vfx-next
```

not `tui-vfx-contract`.

---

# Proof effects to support in G2

Only toy effects are needed.

Recommended proof effect adapters:

```text
proof.replaceGlyph
    input:
        glyph: string or text, first char used
    writes:
        glyph

proof.dim
    input:
        factor: number
    writes:
        foreground/background
    preserves role

proof.explicitRoleWrite
    input:
        role: role
    writes:
        role

proof.copy
    optional if useful
```

These adapters should be clearly named proof effects, not terminal production effects.

---

# Required G2 tests

```text
graph_executor_runs_literal_input_node
    Node uses literal glyph input.
    Final surface proves value reached executor.

graph_executor_resolves_parameter_input
    Node input references parameter.
    Parameter default or context override drives output.

graph_executor_resolves_signal_input
    Node input references signal.
    Context signal value drives output.

graph_executor_uses_signal_fallback_when_missing
    Missing optional signal with fallback resolves.

graph_executor_rejects_missing_required_signal_without_default
    Required signal missing and no fallback/default fails.

graph_executor_resolves_numeric_map_source
    Map signal 0..1 to output range and proof effect observes mapped value.

graph_executor_order_is_node_order
    Two nodes in order produce output A then B.
    Reversing order changes output.

graph_executor_later_node_sees_prior_node_role
    First node writes role.
    Second role-scoped node matches that role.

graph_executor_reuses_scope_and_write_policy_semantics
    SkipTransparentEmpty or unsupported scope behavior remains consistent.

graph_executor_rejects_unknown_proof_effect_adapter
    Graph validates structurally, but executor has no adapter for effect id.

graph_executor_runs_graph_validation_before_execution
    Invalid graph fails before mutating/executing nodes.

graph_executor_does_not_apply_f2_bindings
    If included, prove/document that bindings are validation-only in G2.
```

---

# G2 should avoid

```text
Do not add:
    canonical recipe schema
    source recipe authoring schema
    template expansion
    descriptor registry as production API
    runtime ParameterStore
    runtime SignalStore
    live override precedence
    direct node/effect-input bindings
    phase graph
    trigger engine
    studio manifest
    real effect ports
    legacy migration
```

A local proof adapter map is okay.

---

# Copy-paste Phase G2 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–G1 built the v3.1 contract foundation:
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

Your task is Phase G2: Canonical Graph Execution Proof.

Goal:
Prove that `tui-vfx-next` can execute a validated `GraphSpec` from `tui-vfx-contract` over semantic surfaces using toy proof effect adapters. This is not the final runtime engine and not a real effect port.

Primary question:
Can the canonical graph contract drive ordered node execution with ValueSource resolution, descriptor-backed inputs, scopes, write policies, and prior-node visibility?

Hard constraints:
- Add execution proof code to `tui-vfx-next`, not `tui-vfx-contract`.
- Do not add source recipe schema/compiler.
- Do not add runtime ParameterStore or SignalStore.
- Do not implement live override precedence.
- Do not execute F2 BindingSpec unless explicitly approved; recommended: validate only, do not apply bindings.
- Do not add direct node/effect-input bindings.
- Do not add phase graph or trigger engine.
- Do not add studio manifest or studio controls.
- Do not port real effects.
- Do not add legacy aliases.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules for any public proof DTOs, though prefer keeping G2 proof types internal/proof-facing.

Implementation requirements:
- Add a proof graph executor in `tui-vfx-next`.
- It should accept:
    - GraphSpec
    - input Surface
    - execution context containing parameter values and signal values
    - proof effect adapter map
- It should validate the graph before execution.
- It should execute nodes in GraphSpec.order.
- It should resolve NodeSpec.inputs from ValueSource.
- It should apply node scope and write policy semantics consistently with earlier phases.
- Later nodes must see earlier node cell and role writes.
- Missing proof effect adapters should fail clearly.

Recommended proof effect adapters:
- proof.replaceGlyph
- proof.dim
- proof.explicitRoleWrite
- optional proof.copy

Value resolution requirements:
- literal source resolves
- parameter source resolves from context parameter value or ParameterSpec default
- signal source resolves from context signal value, fallback, or SignalSpec default
- required signal missing with no fallback/default fails
- map source resolves numeric values only
- source kind must match effect input spec

Binding requirement:
- G2 should not apply GraphSpec.bindings. They remain declarative and validation-only.
- Document this explicitly in code/docs/tests.

Docs to update:
- docs/v3.1-architecture-overview.md
- docs/v3.1-contract-boundary.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Tests:
Add tests covering:
- literal node input execution
- parameter input execution
- signal input execution
- missing signal fallback behavior
- missing required signal failure
- numeric map source execution
- node order controls output
- later node sees prior node role write
- scope/write policies reuse previous semantics
- unknown proof effect adapter fails
- invalid graph fails before execution
- F2 bindings are not applied in G2

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

Deliverables:
- Graph execution proof in tui-vfx-next
- Proof adapter tests
- Updated docs
- docs/new_kernel/PHASE_G2_STATUS.md
- docs/new_kernel/PHASE_G2_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase G2 is done when `tui-vfx-next` can execute a validated canonical GraphSpec using toy proof adapters; ValueSource resolution works for literals, parameters, signals, and numeric maps; node order and prior-node visibility are proven; scopes/write policies reuse previous semantics; missing adapters and value-resolution failures produce clear errors; F2 bindings remain validation-only; and no recipe compiler, runtime store, phase graph, studio manifest, or real effect port leaks into the phase.
```

---

# After G2

After G2, proceed to:

```text
Phase H1 — Canonical Recipe Document Schema
```

Not source authoring yet, and not template expansion yet.

H1 should wrap the already-proven graph shape into the canonical recipe document boundary:

```text
RecipeDocument
    metadata
    requires
    graph
    maybe scene
```

Then later:

```text
H2 — source authoring recipe / template expansion
H3 — recipe compiler from source to canonical graph
```

---

# Roadmap update

```text
+====================================================================================+
| CURRENT ROADMAP                                                                     |
+====================================================================================+

  [DONE]   Phase A  — Semantic surface contract
  [DONE]   Phase B  — Sampled-source semantics
  [DONE]   Phase C  — Ordered pipeline/pass semantics
  [DONE]   Phase D0 — Schema/reference backfill
  [DONE]   Phase D1 — Scene / element / layer composition semantics
  [DONE]   Phase D2 — Template composition design
  [DONE]   Phase D3 — Contract/engine boundary
  [DONE]   Phase E0 — Physical contract split
  [DONE]   Phase E1 — Minimal effect descriptor model
  [DONE]   Phase F1 — ValueKind / Value / EffectInputSpec
  [DONE]   Phase F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec
  [DONE]   Phase G1 — Canonical Node Graph Container

  [NEXT]   Phase G2 — Canonical Graph Execution Proof

  [NEXT+]  Phase H1 — Canonical Recipe Document Schema
  [LATER]  Phase H2 — Source recipe/template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase G2**.
