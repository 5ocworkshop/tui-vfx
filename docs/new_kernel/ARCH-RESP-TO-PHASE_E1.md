<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_E1.md</FILE> - <DESC>Architect response to Phase E1 minimal descriptor status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase F1 kickoff: architect approves E1 and directs typed value/input work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect guidance for Phase F1 ValueKind / Value / EffectInputSpec.</CLOG> -->

# E1 architect verdict

**Approved.**

Phase E1 landed the descriptor layer at exactly the right scope: capability-only, schema-backed, and owned by `tui-vfx-contract`.

The important lock is:

```text
EffectDescriptor describes what an effect is allowed to do.
It does not yet describe all author-facing inputs, parameters, signals, or bindings.
```

That separation keeps the next phase clean.

---

# What E1 locks

```text
+====================================================================================+
|                                  PHASE E1 LOCKS                                      |
+====================================================================================+

  DESCRIPTOR OWNERSHIP
      [LOCK] EffectDescriptor lives in tui-vfx-contract.
      [LOCK] tui-vfx-next remains proof execution only.
      [LOCK] PipelineStage is not the descriptor model.

  EFFECT IDENTITY
      [LOCK] EffectId exists as stable descriptor identity.
      [LOCK] EffectDomain exists as initial broad execution taxonomy.

  CAPABILITY
      [LOCK] Descriptors declare readable cell channels.
      [LOCK] Descriptors declare writable cell channels.
      [LOCK] Descriptors declare supported scope kinds/spaces.
      [LOCK] Descriptors declare supported cell and role write policies.

  LIFECYCLE
      [LOCK] Minimal lifecycle/completion vocabulary exists.

  VALIDATION
      [LOCK] Unsupported scope/write/channel behavior can be rejected locally.
      [LOCK] Descriptor capability validation exists before recipe validation.

  SCHEMA
      [LOCK] effect-descriptor.schema.json exists under schemas/v3.1/contract/.
      [LOCK] Descriptor contract follows D0 rustdoc + Serde + Schemars rules.

+====================================================================================+
```

---

# Answers to E1 open questions

## 1. Should Phase F define `ValueKind` first or start from `ValueSource`?

Start with a closed **`ValueKind` + literal `Value` + `ValueSpec`** model first.

Do **not** start from `ValueSource`.

Reason:

```text
ValueSource depends on value typing.
EffectInputSpec depends on value typing.
ParameterSpec depends on value typing.
SignalSpec depends on value typing.
Binding validation depends on value typing.
Studio controls depend on value typing.
```

So Phase F should begin with the value type system and effect input specs. Then a later subphase can add `ValueSource`, parameters, signals, and bindings.

I would split Phase F:

```text
Phase F1 — ValueKind / Value / EffectInputSpec
Phase F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec
```

That prevents Phase F from becoming too large.

## 2. Should descriptor input specs include studio-facing metadata immediately?

Not full studio metadata yet.

In F1, allow **small human-facing metadata** that belongs to the input itself:

```text
displayName
description
unit
semantic
```

But defer studio layout/control metadata:

```text
control type
group
order
advanced/basic
visibility rules
usedBy links
studio-specific widgets
```

Those belong later in a manifest/UI layer.

The distinction:

```text
Input meaning:
    belongs in EffectInputSpec

Studio presentation:
    belongs in StudioManifest or optional UI hints later
```

## 3. Should descriptor validation return generic contract diagnostics now?

Not yet. Keep the narrow `DescriptorValidationError` for E1/F1.

A generic `ContractDiagnostic` will be useful once recipe/node/input validation spans multiple layers:

```text
effect descriptor
node input bindings
value sources
parameters
signals
runtime graph
```

That likely belongs around Phase G/H, when there is a compiler producing many diagnostic kinds.

For now:

```text
DescriptorValidationError is fine.
Do not over-generalize diagnostics before recipe validation exists.
```

## 4. Should schemas remain flat under `schemas/v3.1/contract/`?

Yes. Keep them flat for F1.

Use:

```text
schemas/v3.1/contract/value.schema.json
schemas/v3.1/contract/effect-input.schema.json
```

Only introduce subdirectories when schema volume or naming becomes painful.

---

# Recommended next phase

```text
+====================================================================================+
|                    PHASE F1 — VALUE KIND / VALUE / EFFECT INPUT MODEL                |
+====================================================================================+
```

F1 should add the typed input contract that plugs into `EffectDescriptor`, but it should **not** add parameters, signals, bindings, or full `ValueSource` yet.

---

# Why F1 before full parameters/signals

Effect descriptors need to say:

```text
This effect has an input named "opacity".
It is a number.
It defaults to 0.5.
It accepts 0.0..=1.0.
It can change at runtime or only at reset.
It is bindable or not bindable.
```

That is the minimum input contract. Recipe parameters and runtime signals can be built on top of it later.

---

# F1 target model

```text
+==================================================================================================+
|                      PHASE F1 — VALUE + EFFECT INPUT CONTRACT                                     |
+==================================================================================================+

        +-------------------------------+
        | ValueKind                     |
        |                               |
        | boolean, integer, number      |
        | string, text, color, duration |
        | enum, role, scope, rect       |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | Value                         |
        |                               |
        | typed literal values          |
        | schema-backed JSON shape      |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | ValueSpec                     |
        |                               |
        | kind                          |
        | default                       |
        | range / enum values           |
        | unit / semantic               |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | EffectInputSpec               |
        |                               |
        | input id                      |
        | value spec                    |
        | bindable                      |
        | runtime mutability            |
        +---------------+---------------+
                        |
                        v
        +-------------------------------+
        | EffectDescriptor.inputs       |
        |                               |
        | map input id -> input spec    |
        +-------------------------------+

  DEFERRED TO F2
  ────────────────────────────────────────────────────────────────────────────────────────────────

        ValueSource
        ParameterSpec
        SignalSpec
        BindingSpec
        preset overrides
        runtime override precedence
        studio manifest controls

+==================================================================================================+
```

---

# F1 should lock

```text
+====================================================================================+
|                                  PHASE F1 LOCK TARGETS                              |
+====================================================================================+

  VALUE VOCABULARY
      [LOCK] Closed initial ValueKind vocabulary.
      [LOCK] Typed literal Value shape.
      [LOCK] ValueKind and Value compatibility validation.

  VALUE SPECS
      [LOCK] Defaults are typed and validated.
      [LOCK] Numeric ranges are represented.
      [LOCK] Enum allowed values are represented.
      [LOCK] Unit and semantic strings exist for documentation, not behavior.

  INPUT IDENTITY
      [LOCK] EffectInputId exists or input names are explicitly modeled.
      [LOCK] EffectDescriptor owns a map of inputs.

  MUTABILITY
      [LOCK] RuntimeMutability vocabulary exists:
             compileTime
             phaseStart
             resetOnly
             runtime

  BINDABILITY
      [LOCK] Effect inputs can declare bindable true/false.
      [LOCK] Bindability is only capability metadata in F1.
      [LOCK] Actual bindings are deferred to F2.

  VALIDATION
      [LOCK] Descriptor rejects default values that do not match input type.
      [LOCK] Descriptor rejects numeric defaults outside range.
      [LOCK] Descriptor rejects enum defaults outside allowed values.
      [LOCK] Descriptor rejects duplicate or invalid input ids if applicable.

  SCHEMA
      [LOCK] value.schema.json exists.
      [LOCK] effect-input.schema.json or descriptor schema includes inputs.
      [LOCK] effect-descriptor schema updated and current.

+====================================================================================+
```

---

# Recommended initial `ValueKind` vocabulary

Keep it smaller than the long-term dream list. Add only what the existing contract and near-future descriptors need.

Recommended F1 set:

```text
null
boolean
integer
number
string
text
color
duration
enum
role
scope
rect
```

Consider deferring these until needed:

```text
vec2
vec3
curve
palette
glyphSet
surfaceRef
elementRef
layerRef
```

Reason: every value kind becomes a validation/schema/studio commitment. Add obvious ones now; defer specialized ones until descriptors need them.

---

# Recommended `Value` shape

Use a tagged representation for schema clarity:

```json
{ "kind": "number", "value": 0.5 }
{ "kind": "boolean", "value": true }
{ "kind": "text", "value": "BOOT READY" }
{ "kind": "color", "value": { "r": 255, "g": 255, "b": 255, "a": 255 } }
{ "kind": "role", "value": "text" }
```

The exact Rust enum can choose `#[serde(tag = "kind", content = "value")]`.

Do **not** allow untyped raw JSON as the canonical contract in F1. Raw JSON can be accepted later in source authoring/migration layers, but the canonical contract should be typed.

---

# F1 should avoid

```text
Do not add:
    ValueSource
    ParamRef
    SignalRef
    map/select expression language
    recipe parameters
    runtime bindings
    presets
    studio UI controls
    input inheritance/template expansion
    node graph
    full descriptor registry
    real effect ports
```

It is fine to update `EffectDescriptor` to contain inputs.

---

# Minimal E1 → F1 descriptor example

After F1, a proof dim descriptor should be able to say:

```json
{
  "id": "terminal.dim",
  "version": "0.1.0",
  "domain": "frameFilter",
  "displayName": "Dim",
  "cellAccess": {
    "reads": ["foreground", "background"],
    "writes": ["foreground", "background"]
  },
  "scopeSupport": {
    "kinds": ["all", "role", "rect", "rowRange", "columnRange"]
  },
  "writeSupport": {
    "cellPolicies": ["writeCell", "skipTransparentEmpty"],
    "rolePolicies": ["preserveDestination"]
  },
  "inputs": {
    "factor": {
      "value": {
        "kind": "number",
        "default": { "kind": "number", "value": 0.5 },
        "min": 0,
        "max": 1,
        "unit": "ratio"
      },
      "bindable": true,
      "runtimeMutability": "runtime"
    }
  },
  "lifecycle": {
    "completion": "instant",
    "resettable": false,
    "seekable": false,
    "deterministicWithSeed": true
  }
}
```

Exact field names can differ, but the capability must be expressible.

---

# Required F1 tests

```text
value_schema_is_current
effect_input_schema_is_current
effect_descriptor_schema_with_inputs_is_current

number_value_matches_number_kind
number_value_rejects_boolean_kind

number_default_within_range_is_valid
number_default_outside_range_is_invalid

integer_default_rejects_fractional_number
enum_default_must_be_allowed_value
enum_allowed_values_must_not_be_empty

role_value_round_trips
scope_value_round_trips
color_value_round_trips

effect_descriptor_accepts_valid_input_spec
effect_descriptor_rejects_input_default_type_mismatch
effect_descriptor_rejects_input_default_out_of_range

runtime_mutability_schema_is_described
bindable_flag_schema_is_described

phase_f1_does_not_add_value_source_or_parameters
```

The last test can be a docs/assertion-style test only if practical. The key is: do not let F1 drift into F2.

---

# Copy-paste Phase F1 prompt

```text
You are working in the tui-vfx Rust workspace.

Phases A–E1 built the v3.1 clean-room contract foundation:
- A: semantic surface contract
- B: sampled-source semantics
- C: ordered pipeline/pass semantics
- D0: schema/reference backfill
- D1: scene / element / layer composition semantics
- D2: template composition design
- D3: contract/engine boundary
- E0: physical contract split
- E1: minimal effect descriptor capability model

Your task is Phase F1: ValueKind / Value / EffectInputSpec.

Goal:
Add the first typed input contract to `tui-vfx-contract`. Effect descriptors should be able to declare typed inputs, defaults, ranges, bindability, and runtime mutability. Do not add parameters, signals, bindings, or ValueSource yet.

Primary question:
Can an EffectDescriptor declare typed effect inputs with validated defaults and ranges using a closed value vocabulary?

Hard constraints:
- Add value/input DTOs to `tui-vfx-contract`.
- Do not add ValueSource yet.
- Do not add ParameterSpec, SignalSpec, BindingSpec, presets, or runtime override precedence.
- Do not add recipe schema/compiler.
- Do not add template expansion.
- Do not add studio manifest or studio controls.
- Do not add runtime bindings, phase graph, trigger engine, or legacy migration.
- Do not port real effects.
- Do not replace or refactor the legacy compositor.
- Do not add legacy aliases.
- Preserve v3.1 naming.
- Preserve D0 schema/reference rules.
- Keep proof `PipelineStage` and proof engine types out of the contract descriptor model.

Required concepts:
- ValueKind
- Value
- ValueSpec
- EffectInputId or equivalent
- EffectInputSpec
- RuntimeMutability
- bindable flag
- EffectDescriptor.inputs map

Recommended initial ValueKind values:
- null
- boolean
- integer
- number
- string
- text
- color
- duration
- enum
- role
- scope
- rect

Defer unless absolutely needed:
- vec2
- vec3
- curve
- palette
- glyphSet
- surfaceRef
- elementRef
- layerRef

Validation requirements:
- Value kind must match expected ValueKind.
- Numeric default must satisfy min/max when provided.
- Integer defaults must be integer values.
- Enum specs must have non-empty allowed values.
- Enum defaults must be one of allowed values.
- Role/scope/color/rect values must round-trip through schema-backed types.
- EffectDescriptor validation should validate input specs.

Schema requirements:
Add or update checked schema roots under schemas/v3.1/contract/:
- value.schema.json
- effect-input.schema.json if useful as separate root
- effect-descriptor.schema.json updated with inputs

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
- key value/input schemas have rustdoc descriptions
- valid number default passes
- number default outside range fails
- default type mismatch fails
- integer rejects fractional values
- enum default must be allowed value
- empty enum allowed-values invalid
- role/scope/color/rect values round-trip
- descriptor accepts valid input spec
- descriptor rejects invalid input spec

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
- Value/value-spec/input DTOs in tui-vfx-contract
- EffectDescriptor inputs
- Checked schema updates
- Validation tests
- Updated docs
- docs/new_kernel/PHASE_F1_STATUS.md
- docs/new_kernel/PHASE_F1_STATUS_MEMO_TO_ARCHITECT.md

Definition of done:
Phase F1 is done when `tui-vfx-contract` contains a schema-backed closed value vocabulary and EffectInputSpec model, EffectDescriptor can declare typed inputs, defaults/ranges/enums are validated, schemas are current and described, and no ValueSource/parameter/signal/binding/recipe/runtime/effect-port work has leaked into the phase.
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

  [NEXT]   Phase F1 — ValueKind / Value / EffectInputSpec

  [NEXT+]  Phase F2 — ValueSource / ParameterSpec / SignalSpec / BindingSpec
  [LATER]  Phase G  — Node graph
  [LATER]  Phase H  — Strict canonical recipe v3.1 schema/compiler
  [LATER]  Phase H2 — Template expansion implementation
  [LATER]  Phase I  — Phase/trigger engine
  [LATER]  Phase J  — First real effect ports

+====================================================================================+
```

Proceed with **Phase F1**.


<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_E1.md</FILE> - <DESC>Architect response to Phase E1 minimal descriptor status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
