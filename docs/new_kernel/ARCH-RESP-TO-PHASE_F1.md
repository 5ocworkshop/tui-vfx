<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_F1.md</FILE> - <DESC>Architect response to Phase F1 typed value/input status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F2 kickoff: architect approves F1 and directs value source / parameter / signal / binding work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect guidance for Phase F2 ValueSource / ParameterSpec / SignalSpec / BindingSpec.</CLOG> -->

# F1 architect verdict

**Approved.**

Phase F1 landed the right layer: typed effect inputs without jumping into source/binding/runtime semantics.

The important lock is now:

```text
EffectDescriptor
    declares what an effect can do

EffectInputSpec
    declares what an effect input expects

Value / ValueKind / ValueSpec
    declare typed literal value contracts

ValueSource / parameters / signals / bindings
    still deferred to F2
```

That separation is exactly what we wanted.

---

# What F1 locks

```text
+====================================================================================+
|                                  PHASE F1 LOCKS                                      |
+====================================================================================+

  VALUE MODEL
      [LOCK] Closed initial ValueKind vocabulary exists.
      [LOCK] Value uses strict typed literal shape.
      [LOCK] ValueKind / Value compatibility is validated.

  VALUE SPEC
      [LOCK] Defaults are typed and validated.
      [LOCK] Numeric ranges are validated.
      [LOCK] Enum allowed values are validated.
      [LOCK] Role, scope, color, and rect values reuse existing schema-backed contract types.

  EFFECT INPUTS
      [LOCK] EffectDescriptor owns inputs as BTreeMap<EffectInputId, EffectInputSpec>.
      [LOCK] EffectInputId exists and is schema-constrained.
      [LOCK] RuntimeMutability vocabulary exists.
      [LOCK] bindable exists as capability metadata only.

  BOUNDARY
      [LOCK] No ValueSource yet.
      [LOCK] No parameters/signals/bindings yet.
      [LOCK] No runtime precedence model yet.
      [LOCK] No studio controls yet.

  SCHEMA
      [LOCK] value.schema.json exists.
      [LOCK] effect-input.schema.json exists.
      [LOCK] effect-descriptor.schema.json includes inputs.
      [LOCK] F1 follows D0 schema-reference rules.

+====================================================================================+
```

One small note for later: duplicate JSON keys in maps are inherently a parser/source-document concern. It is fine that F1 relies on map semantics for now. If strict duplicate-key rejection becomes important, handle it in the JSON parse/source loader layer, not in the already-deserialized Rust model.

---

# Answers to F1 open questions

## 1. Should F2 introduce `ValueSource` before `ParameterSpec` / `SignalSpec`, or define all three together?

Define them **together**, but keep the phase bounded.

`ValueSource::Param` and `ValueSource::Signal` need declared namespaces to validate against. So F2 should introduce:

```text
ValueSource
ParameterSpec
SignalSpec
```

in the same phase.

Add `BindingSpec` too, but only as a declarative contract. Do not implement a runtime parameter store yet.

## 2. Should `EffectInputSpec` remain descriptor-local, or should F2 add parameter/signal namespaces beside it?

Add separate namespaces:

```text
EffectInputSpec
    belongs to an effect descriptor
    says what the implementation accepts

ParameterSpec
    belongs to recipe/canonical graph layer
    says what public controls exist

SignalSpec
    belongs to runtime/host contract
    says what external values may be provided
```

Do not merge these concepts. They have different owners.

## 3. Should F2 keep bindings declarative, or introduce runtime override precedence?

Keep bindings **declarative** in F2.

It is okay to document the intended future precedence:

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

But do not implement runtime stores or override resolution yet. That belongs later, after node graph / recipe compiler shape exists.

## 4. Should studio control metadata remain out-of-band?

Yes.

F2 should not add studio controls. It may keep lightweight semantic metadata already present from F1:

```text
displayName
description
unit
semantic
```

But defer:

```text
control widget
group
order
visibility
advanced/basic
layout
usedBy links
studio manifest generation
```

Those belong to the studio manifest phase.

---

# Recommended next phase

```text
+====================================================================================+
|              PHASE F2 — VALUE SOURCE / PARAMETER / SIGNAL / BINDING MODEL           |
+====================================================================================+
```

F2 should answer:

```text
Can the contract describe where an effect input value comes from,
without yet building the recipe compiler or runtime store?
```

The phase should establish the declarative vocabulary for:

```text
literal values
parameter references
signal references
simple mapping transforms
public parameters
host/runtime signals
bindings from sources to targets
```

---

# F2 target model

```text
+==================================================================================================+
|                         PHASE F2 — VALUE SOURCES AND BINDINGS                                     |
+==================================================================================================+

        +-------------------------------+
        | Value                          |
        | typed literal from F1          |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | ValueSource                    |
        |                               |
        |  literal                       |
        |  parameter                     |
        |  signal                        |
        |  map                           |
        |  select? optional              |
        +---------------+---------------+
                        |
        +---------------+-------------------------------+
        |                                               |
        v                                               v
+-------------------------------+        +-------------------------------+
| ParameterSpec                 |        | SignalSpec                    |
|                               |        |                               |
| public recipe control         |        | host/runtime-provided value   |
| value spec                    |        | value spec                    |
| default                       |        | default/fallback behavior     |
| bindable                      |        | optional/required policy      |
+---------------+---------------+        +---------------+---------------+
                |                                        |
                +--------------------+-------------------+
                                     |
                                     v
                         +-------------------------------+
                         | BindingSpec                   |
                         |                               |
                         | source -> target              |
                         | mode                          |
                         | fallback                      |
                         | transform                     |
                         +-------------------------------+

  DEFERRED
  ────────────────────────────────────────────────────────────────────────────────────────────────

        runtime parameter store
        signal store
        live override layer
        preset/profile persistence
        studio manifest
        recipe compiler
        node graph execution

+==================================================================================================+
```

---

# F2 should lock

```text
+====================================================================================+
|                                  PHASE F2 LOCK TARGETS                              |
+====================================================================================+

  VALUE SOURCE
      [LOCK] ValueSource vocabulary exists.
      [LOCK] Literal sources carry typed Value.
      [LOCK] Param sources reference ParameterSpec ids.
      [LOCK] Signal sources reference SignalSpec ids.
      [LOCK] Fallback behavior is represented.
      [LOCK] Simple numeric map transform is represented if included.

  PARAMETER CONTRACT
      [LOCK] ParameterId exists.
      [LOCK] ParameterSpec owns public recipe controls.
      [LOCK] ParameterSpec has ValueSpec/default.
      [LOCK] Parameter defaults validate against their specs.
      [LOCK] Parameters are separate from effect inputs.

  SIGNAL CONTRACT
      [LOCK] SignalId exists.
      [LOCK] SignalSpec declares host/runtime-provided values.
      [LOCK] Signal default/optional/fallback semantics are explicit.
      [LOCK] Signal values validate against ValueSpec.

  BINDING CONTRACT
      [LOCK] BindingTarget vocabulary exists.
      [LOCK] BindingSource references ValueSource.
      [LOCK] BindingSpec is declarative only.
      [LOCK] Binding modes are declared but not executed.
      [LOCK] Runtime precedence is documented but not implemented.

  VALIDATION
      [LOCK] Parameter references validate against declared parameters.
      [LOCK] Signal references validate against declared signals.
      [LOCK] ValueSource inferred kind must be compatible with target kind.
      [LOCK] Map transforms only apply to numeric-compatible values.
      [LOCK] Invalid fallback value kind is rejected.

  SCHEMA
      [LOCK] value-source.schema.json exists.
      [LOCK] parameter.schema.json exists.
      [LOCK] signal.schema.json exists.
      [LOCK] binding.schema.json exists if BindingSpec is added as root.
      [LOCK] Existing schemas remain current.

+====================================================================================+
```

---

# Recommended `ValueSource` vocabulary

Keep F2 small. I would include:

```text
Literal
Param
Signal
Map
```

Optional if easy:

```text
Select
```

But I would not include arbitrary expressions.

Recommended initial shape:

```text
ValueSource::Literal {
    value: Value
}

ValueSource::Param {
    id: ParameterId
    fallback: Option<Value>
}

ValueSource::Signal {
    id: SignalId
    fallback: Option<Value>
}

ValueSource::Map {
    from: Box<ValueSource>
    input: NumericRange
    output: NumericRange
    clamp: bool
}
```

Defer:

```text
arithmetic expressions
scripts
multi-source formulas
curves beyond simple map
string templates
conditionals beyond maybe select
```

---

# Binding model recommendation

Add `BindingSpec`, but keep it declarative.

Targets:

```text
parameter
effectInput
```

For F2, it is okay to include both but document that effect-input direct binding will be validated later when nodes exist.

```text
BindingTarget::Parameter { id: ParameterId }

BindingTarget::EffectInput {
    node: Option<NodeId> or string placeholder?  // maybe defer node-specific target
    input: EffectInputId
}
```

Because NodeId does not exist yet, I would either:

1. make F2 binding target **parameter-only**, or
2. define a generic future-facing target but do not validate node references yet.

My recommendation:

```text
F2 BindingSpec targets parameters only.
Direct effect-input binding is deferred until node graph exists.
```

That avoids inventing node identity before Phase G.

Binding modes:

```text
replace
add
multiply
min
max
mix
```

But F2 can start with:

```text
replace
```

and defer arithmetic modes. If modes are included now, validation must restrict them to numeric kinds.

---

# F2 should avoid

```text
Do not add:
    NodeId / recipe nodes
    direct node input bindings unless node identity exists
    runtime parameter store
    signal store implementation
    override precedence execution
    preset/profile implementation
    studio UI metadata
    expression language
    arbitrary scripting
    recipe compiler
    effect registry
    real effect ports
```

---

# Required F2 tests

```text
value_source_schema_is_current
parameter_schema_is_current
signal_schema_is_current
binding_schema_is_current_if_added

literal_value_source_validates_kind
literal_value_source_rejects_wrong_target_kind

parameter_spec_default_validates
parameter_reference_resolves_declared_parameter
parameter_reference_rejects_unknown_parameter
parameter_fallback_must_match_parameter_kind

signal_spec_default_validates
signal_reference_resolves_declared_signal
signal_reference_rejects_unknown_signal
signal_fallback_must_match_signal_kind

map_source_accepts_numeric_source
map_source_rejects_non_numeric_source
map_source_output_kind_is_number

binding_parameter_target_accepts_compatible_source
binding_parameter_target_rejects_incompatible_source
binding_unknown_parameter_target_rejected

f2_does_not_add_runtime_store_or_node_graph
```

If F2 is split further, prioritize:

```text
ValueSource + ParameterSpec + SignalSpec first
BindingSpec second
```

But I think a minimal parameter-only `BindingSpec` is manageable.

---

# Copy-paste Phase F2 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–F1 built the v3.1 contract foundation:
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

Your task is Phase F2: ValueSource / ParameterSpec / SignalSpec / BindingSpec.

Goal:
Add the declarative source layer to `tui-vfx-contract`: typed values can now come from literals, declared parameters, declared signals, or simple transforms. Keep this purely declarative; do not implement runtime stores or recipe compilation.

Primary question:
Can the contract describe where values come from and validate parameter/signal references and kind compatibility without building the runtime?

Hard constraints:
- Add DTOs to `tui-vfx-contract`.
- Do not implement runtime ParameterStore or SignalStore.
- Do not implement live override precedence.
- Do not add recipe nodes or node graph.
- Do not add direct node/effect-input binding unless node identity exists; prefer parameter-only BindingSpec for F2.
- Do not add recipe schema/compiler.
- Do not add template expansion.
- Do not add studio manifest or studio controls.
- Do not add phase graph, trigger engine, or legacy migration.
- Do not port real effects.
- Do not add legacy aliases.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules.
- Keep proof engine types out of the contract source model.

Required concepts:
- ValueSource
- ParameterId
- ParameterSpec
- SignalId
- SignalSpec
- BindingSpec, preferably parameter-target only for F2
- BindingTarget
- BindingMode, if included
- validation helpers for source/target compatibility

Recommended ValueSource variants:
- literal
- parameter
- signal
- map

Optional only if small:
- select

Do not add arbitrary expression language.

Recommended BindingSpec scope:
- target: ParameterId
- source: ValueSource
- mode: replace initially

If arithmetic modes are added, validate they only apply to numeric-compatible kinds.

Validation requirements:
- Literal source kind must match expected target kind.
- Parameter references must exist.
- Signal references must exist.
- Fallback values must match referenced/expected kind.
- Parameter defaults validate against ParameterSpec.
- Signal defaults validate against SignalSpec.
- Map source must be numeric-compatible.
- Map output should be number-compatible.
- Binding target parameter must exist.
- Binding source must be compatible with target parameter kind.
- Unknown ids are rejected with narrow validation errors.

Schema requirements:
Add checked schema roots under schemas/v3.1/contract/:
- value-source.schema.json
- parameter.schema.json
- signal.schema.json
- binding.schema.json if BindingSpec is a root

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
- parameter default validates
- signal default validates
- literal source validates expected kind
- parameter source resolves known id
- parameter source rejects unknown id
- signal source resolves known id
- signal source rejects unknown id
- fallback kind mismatch rejected
- map source accepts numeric source
- map source rejects non-numeric source
- binding to parameter accepts compatible source
- binding to parameter rejects incompatible source
- binding to unknown parameter rejected

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
- ValueSource / ParameterSpec / SignalSpec / BindingSpec DTOs in tui-vfx-contract
- Validation helpers and tests
- Checked schema updates
- Updated docs
- docs/new_kernel/PHASE_F2_STATUS.md
- docs/new_kernel/PHASE_F2_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase F2 is done when `tui-vfx-contract` can represent literal, parameter, signal, and simple mapped value sources; declared parameters/signals validate defaults; value sources validate references and kind compatibility; parameter-target bindings validate declaratively; schemas are current and described; and no runtime store, node graph, recipe compiler, studio manifest, or effect port has leaked into the phase.
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

  [NEXT]   Phase F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec

  [NEXT+]  Phase G  — Node graph
  [LATER]  Phase H  — Strict canonical recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase F2**.


<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_F1.md</FILE> - <DESC>Architect response to Phase F1 typed value/input status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
