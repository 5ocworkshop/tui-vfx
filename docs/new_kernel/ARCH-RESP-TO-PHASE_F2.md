<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_F2.md</FILE> - <DESC>Architect response to Phase F2 declarative source and binding status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase G1 kickoff: architect approves F2 and directs canonical graph container work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect guidance for Phase G1 GraphSpec / NodeSpec validation.</CLOG> -->

# F2 architect verdict

**Approved.**

Phase F2 landed the right thing: a declarative source/binding vocabulary without crossing into runtime state, recipe compilation, or node graph execution.

The important lock is now:

```text
Value
    typed literal

ValueSource
    where a value comes from

ParameterSpec
    public recipe control namespace

SignalSpec
    host/runtime input namespace

BindingSpec
    declarative parameter-target binding

Runtime store / recipe graph / node identity
    still deferred
```

---

# What F2 locks

```text
+====================================================================================+
|                                  PHASE F2 LOCKS                                      |
+====================================================================================+

  VALUE SOURCES
      [LOCK] ValueSource exists.
      [LOCK] Literal, parameter, signal, and map sources are represented.
      [LOCK] ValueSource is declarative only.
      [LOCK] No runtime store or live override execution exists yet.

  PARAMETERS
      [LOCK] ParameterId exists.
      [LOCK] ParameterSpec is separate from EffectInputSpec.
      [LOCK] Parameters are public recipe controls.
      [LOCK] Parameter defaults validate through ValueSpec.
      [LOCK] Parameter bindability is respected by BindingSpec validation.

  SIGNALS
      [LOCK] SignalId exists.
      [LOCK] SignalSpec is separate from ParameterSpec and EffectInputSpec.
      [LOCK] SignalSpec describes host/runtime-provided values.
      [LOCK] Signal required/default policy is explicit.

  BINDINGS
      [LOCK] BindingSpec exists.
      [LOCK] Binding targets parameters only in F2.
      [LOCK] BindingMode is intentionally minimal: replace.
      [LOCK] Direct node/effect-input bindings are deferred until node identity exists.

  TRANSFORMS
      [LOCK] Map is numeric-only.
      [LOCK] Arbitrary expressions/scripts are not part of v3.1 at this layer.

  BOUNDARY
      [LOCK] No recipe compiler.
      [LOCK] No node graph.
      [LOCK] No runtime precedence execution.
      [LOCK] No studio metadata.

+====================================================================================+
```

---

# Answers to F2 open questions

## 1. Node graph first, or recipe-level container first?

Do them together, but keep the phase small.

The next useful phase should introduce a **canonical graph container** with:

```text
parameters
signals
bindings
effect descriptors
nodes
linear node order
```

Do not call it the full public authoring recipe yet. Think of it as the canonical **runtime graph document shape** that a future recipe compiler will produce.

So the next phase should be:

```text
Phase G1 — Canonical Node Graph Container
```

not full recipe schema.

## 2. Direct effect-input binding in the same phase?

No.

In G1, node inputs should be expressed as:

```text
ValueSource
```

That already allows:

```text
literal
parameter
signal
map
```

Direct runtime binding to node/effect inputs should wait until graph validation is stable.

For G1:

```text
Node input value:
    ValueSource

BindingSpec:
    parameter-target only, as in F2
```

Later:

```text
RuntimeBindingSpec:
    parameter target
    maybe node input target
```

Do not add direct node-input binding yet.

## 3. Should map transforms remain numeric-only in G?

Yes.

Keep maps numeric-only through G.

Defer:

```text
curve
select
expression
multi-source formulas
string templates
palette transforms
```

until the compiler/runtime story is more complete.

## 4. Should runtime precedence remain documented only?

Yes.

Keep precedence documented only until after canonical recipe compilation exists.

Do not implement:

```text
live override > binding > preset > default
```

yet. We need a canonical graph and then a runtime instance before that behavior has a place to live.

---

# Recommended next phase

```text
+====================================================================================+
|                    PHASE G1 — CANONICAL NODE GRAPH CONTAINER                         |
+====================================================================================+
```

The goal is not to build full recipes. The goal is to define the smallest canonical graph that proves descriptors, inputs, parameters, signals, bindings, and node identity can be validated together.

---

# G1 target model

```text
+==================================================================================================+
|                         PHASE G1 — CANONICAL NODE GRAPH CONTAINER                                 |
+==================================================================================================+

        +-------------------------------+
        | GraphDocument / GraphSpec      |
        |                               |
        |  id                            |
        |  version                       |
        |  parameters                    |
        |  signals                       |
        |  bindings                      |
        |  effects                       |
        |  nodes                         |
        |  order                         |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | NodeSpec                       |
        |                               |
        |  nodeId                        |
        |  effectId                      |
        |  inputs: ValueSource map       |
        |  scope                         |
        |  cell write policy             |
        |  role write policy             |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Graph Validation               |
        |                               |
        |  effect exists                 |
        |  node id exists                |
        |  input exists on effect        |
        |  ValueSource compatible        |
        |  scope supported               |
        |  write policy supported        |
        |  parameter/signal refs valid   |
        |  binding target valid          |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Validated Canonical Graph      |
        |                               |
        |  no runtime execution yet      |
        |  no phase graph yet            |
        |  no trigger graph yet          |
        +-------------------------------+

+==================================================================================================+
```

---

# What G1 should lock

```text
+====================================================================================+
|                                  PHASE G1 LOCK TARGETS                              |
+====================================================================================+

  GRAPH CONTAINER
      [LOCK] There is a canonical graph/container DTO.
      [LOCK] It is not source authoring syntax.
      [LOCK] It is not template syntax.
      [LOCK] It is the shape a future recipe compiler can emit.

  NODE IDENTITY
      [LOCK] NodeId exists.
      [LOCK] Node ids are stable and schema-constrained.
      [LOCK] Node order is deterministic.

  NODE SPEC
      [LOCK] A node references an EffectId.
      [LOCK] A node has input values as ValueSource.
      [LOCK] A node can carry scope/write policy using existing vocabulary.
      [LOCK] A node does not own parameters or signals.

  EFFECT LOOKUP
      [LOCK] Graph validation checks node effect ids against declared descriptors.
      [LOCK] Node inputs are checked against EffectDescriptor.inputs.
      [LOCK] Unknown effect id is rejected.
      [LOCK] Unknown input id is rejected.

  SOURCE VALIDATION
      [LOCK] Node ValueSource references validate against graph parameters/signals.
      [LOCK] Source kind must match the effect input ValueSpec.
      [LOCK] Map source remains numeric-only.

  SCOPE / WRITE VALIDATION
      [LOCK] Node scope must be supported by the effect descriptor.
      [LOCK] Node cell write policy must be supported by the effect descriptor.
      [LOCK] Node role write policy must be supported by the effect descriptor.

  BINDINGS
      [LOCK] Existing F2 parameter-target BindingSpec can live in the graph container.
      [LOCK] Direct node-input bindings remain deferred.

  SCHEMA
      [LOCK] graph schema root exists.
      [LOCK] node schema root exists if useful.
      [LOCK] Existing schema roots remain current.

+====================================================================================+
```

---

# G1 should avoid

```text
Do not add:
    source recipe authoring schema
    template expansion implementation
    phase graph
    trigger engine
    runtime graph execution
    runtime stores
    live override precedence
    direct node/effect-input bindings
    studio manifest
    legacy migration
    real effect ports
    full descriptor registry
```

A tiny in-memory descriptor map for validation tests is fine.

---

# Suggested G1 types

Names can vary, but the shape should be close to this.

```rust
/// Stable node identifier inside a canonical graph.
pub struct NodeId(...);
```

```rust
/// One effect node in a canonical v3.1 graph.
pub struct NodeSpec {
    /// Effect implementation/capability descriptor used by this node.
    pub effect: EffectId,

    /// Values supplied to effect inputs.
    pub inputs: BTreeMap<EffectInputId, ValueSource>,

    /// Optional scope limiting where this node applies.
    pub scope: Option<ScopeSpec>,

    /// Cell write policy requested by this node.
    pub cell_write_policy: Option<CellWritePolicy>,

    /// Role write policy requested by this node.
    pub role_write_policy: Option<RoleWritePolicy>,
}
```

```rust
/// Canonical node graph container.
pub struct GraphSpec {
    /// Stable graph id.
    pub id: GraphId,

    /// Public parameters available to node ValueSources.
    pub parameters: BTreeMap<ParameterId, ParameterSpec>,

    /// Host/runtime signals available to node ValueSources.
    pub signals: BTreeMap<SignalId, SignalSpec>,

    /// Declarative parameter-target bindings.
    pub bindings: BTreeMap<BindingId, BindingSpec>,

    /// Effect descriptors available to this graph.
    pub effects: BTreeMap<EffectId, EffectDescriptor>,

    /// Nodes by id.
    pub nodes: BTreeMap<NodeId, NodeSpec>,

    /// Deterministic node execution order.
    pub order: Vec<NodeId>,
}
```

I would use `GraphSpec` or `CanonicalGraphSpec`, not `RecipeDocument`, to avoid prematurely claiming the full recipe schema is done.

---

# Validation cases G1 must prove

```text
valid_graph_with_literal_input_passes

valid_graph_with_parameter_input_passes

valid_graph_with_signal_input_passes

graph_rejects_unknown_effect_id

graph_rejects_unknown_input_id

graph_rejects_input_kind_mismatch

graph_rejects_parameter_source_unknown_parameter

graph_rejects_signal_source_unknown_signal

graph_rejects_unsupported_scope_for_effect

graph_rejects_unsupported_cell_write_policy_for_effect

graph_rejects_unsupported_role_write_policy_for_effect

graph_rejects_order_reference_to_unknown_node

graph_rejects_duplicate_order_entries

graph_accepts_f2_parameter_binding

graph_rejects_binding_to_unknown_parameter

graph_rejects_binding_to_non_bindable_parameter

graph_schema_is_current

node_schema_is_current
```

No execution needed. This is validation-only.

---

# Copy-paste Phase G1 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–F2 built the v3.1 contract foundation:
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

Your task is Phase G1: Canonical Node Graph Container.

Goal:
Add the smallest canonical graph/container contract to `tui-vfx-contract` that can validate descriptors, node identity, node inputs, parameters, signals, bindings, scopes, and write policies together. This is not the full source recipe schema and not runtime execution.

Primary question:
Can a canonical graph declare effect nodes with ValueSource inputs and validate them against effect descriptors, parameters, signals, and descriptor capabilities?

Hard constraints:
- Add DTOs to `tui-vfx-contract`.
- Do not implement runtime graph execution.
- Do not implement phase graph or trigger engine.
- Do not implement runtime ParameterStore / SignalStore.
- Do not implement live override precedence.
- Do not add direct node/effect-input bindings.
- Do not add source recipe authoring schema.
- Do not implement template expansion.
- Do not add studio manifest or studio controls.
- Do not port real effects.
- Do not add legacy aliases.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules.
- Keep proof engine types out of the contract graph model.

Required concepts:
- NodeId
- NodeSpec
- GraphId or CanonicalGraphId
- GraphSpec or CanonicalGraphSpec
- deterministic node order
- graph validation helpers

GraphSpec should include, at minimum:
- parameters: BTreeMap<ParameterId, ParameterSpec>
- signals: BTreeMap<SignalId, SignalSpec>
- bindings: BTreeMap<BindingId, BindingSpec> or Vec<BindingSpec> if BindingId does not exist
- effects: BTreeMap<EffectId, EffectDescriptor>
- nodes: BTreeMap<NodeId, NodeSpec>
- order: Vec<NodeId>

NodeSpec should include:
- effect: EffectId
- inputs: BTreeMap<EffectInputId, ValueSource>
- optional scope: ScopeSpec
- optional cell write policy
- optional role write policy

Validation requirements:
- Node effect id must exist in graph effects.
- Node input ids must exist on the effect descriptor.
- Required effect inputs, if the current model can express them, must be present or have defaults.
- Node input ValueSource kind must be compatible with the effect input ValueSpec.
- Parameter references must exist.
- Signal references must exist.
- Map sources remain numeric-only.
- Node scope must be supported by effect descriptor ScopeSupport.
- Node cell write policy must be supported by effect descriptor WriteSupport.
- Node role write policy must be supported by effect descriptor WriteSupport.
- Node order entries must reference existing nodes.
- Node order must not contain duplicates.
- Bindings must validate using the F2 parameter-target rules.

Do not add:
- direct node input binding targets
- runtime override layers
- phase/trigger semantics
- source authoring recipe aliases
- template expansion

Schema requirements:
Add checked schema roots under schemas/v3.1/contract/:
- graph.schema.json
- node.schema.json if useful as a separate root

Existing schemas must remain current.

All public DTOs must:
- derive or intentionally implement Serialize, Deserialize, JsonSchema
- use strict Serde shape
- include rustdoc comments on public types, fields, and variants
- pass schema description tests

Docs to update:
- docs/v3.1-contract-boundary.md
- docs/v3.1-architecture-overview.md
- docs/v3.1-feature-contract-checklist.md
- docs/new_kernel/AGENT_BRIEFING.md
- docs/new_kernel/INDEX.md
- docs/INDEX.md if applicable

Tests:
Add tests covering:
- schema fixtures are current
- valid graph with literal input passes
- valid graph with parameter input passes
- valid graph with signal input passes
- unknown effect id rejected
- unknown input id rejected
- input kind mismatch rejected
- unknown parameter source rejected
- unknown signal source rejected
- unsupported scope rejected
- unsupported cell write policy rejected
- unsupported role write policy rejected
- order references unknown node rejected
- duplicate node order rejected
- binding to known bindable parameter accepted
- binding to unknown parameter rejected
- binding to non-bindable parameter rejected

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
- Node/graph DTOs in tui-vfx-contract
- Graph validation helpers and tests
- Checked schema updates
- Updated docs
- docs/new_kernel/PHASE_G1_STATUS.md
- docs/new_kernel/PHASE_G1_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase G1 is done when `tui-vfx-contract` can represent a canonical node graph with descriptors, parameters, signals, bindings, ordered nodes, and ValueSource inputs; validation catches unknown effects/inputs/parameters/signals, incompatible input kinds, unsupported scopes/write policies, and invalid node order; schemas are current and described; and no runtime execution, phase graph, source recipe compiler, template expansion, studio manifest, or real effect ports have leaked into the phase.
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

  [NEXT]   Phase G1 — Canonical Node Graph Container

  [NEXT+]  Phase G2 — Optional graph execution proof in tui-vfx-next
  [LATER]  Phase H  — Strict canonical recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase G1**.
