<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G4.md</FILE> - <DESC>Architect response approving Phase G4 and assigning Phase H0 source/asset/procedural source contract</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H0 kickoff: architect directs source/asset contract before recipe schema.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — preserve architect response recommending H0 before H1.</CLOG> -->

Yes — I think we should do **H0: Source / Asset / Procedural Source Contract** before H1.

And no, I do **not** need more recipes before giving that direction. The examples you sent are enough to identify the contract shape. More recipes will be useful later as migration/parity fixtures, but they are not needed to start H0.

The key insight is that recipes do not only describe effects over an existing surface. They also describe **where the initial surface comes from**:

```text
text source
card source
procedural source
asset-backed procedural source
image/ANSI/source material
scene layer source
runtime-driven source parameters
```

If we start canonical recipe schema before sources are represented, H1 will either be incomplete or will accidentally bake source semantics into recipe-specific ad hoc fields. That would create exactly the kind of backtracking we are trying to avoid.

# Recommendation

Proceed with:

```text
Phase H0 — Source / Asset / Procedural Source Contract
```

Then:

```text
Phase H1 — Canonical Recipe Document Schema
```

So the next roadmap becomes:

```text
G4  — Node I/O / Hint Value Bus                 DONE
H0  — Source / Asset / Procedural Source Contract
H1  — Canonical Recipe Document Schema
H2  — Source recipe lowering / compiler skeleton
I   — Phase / trigger / dwell model
J   — Runtime stores / player execution
K   — First real effect/source ports
```

# Why H0 belongs before H1

The examples show several source concepts that are not just “effects”:

```json
"source": {
  "type": "text",
  "spec": {
    "string": "FOLLOW",
    "alignment": "left"
  }
}
```

```json
"source": {
  "type": "card",
  "spec": {
    "bg_fill": { "r": 24, "g": 30, "b": 42, "a": 255 },
    "border_style": { "type": "rounded" },
    "text": { "string": "Processing", "alignment": "center" }
  }
}
```

```json
"source": {
  "type": "procedural",
  "spec": {
    "source_id": "braille_flag_field",
    "params": {
      "wave": {
        "speed": {
          "binding": "wave_speed",
          "default": 0.8
        }
      },
      "asset": {
        "path": "{{ flag_art }}",
        "format": "tui-vfx.braille_flag_asset.v1"
      }
    }
  }
}
```

Those are not downstream filters/shaders/masks. They are **surface producers**.

So H0 should answer:

```text
How does v3.1 describe something that produces a semantic Surface?
How does it declare typed inputs?
How does it declare required assets?
How does a recipe bind asset tokens to source inputs?
How does a procedural source differ from a visual effect?
How does a generated source assign roles?
```

# Important distinction

We now have:

```text
Effect:
    transforms or samples an existing surface/node state

Source:
    produces an initial semantic surface

Procedural source:
    produces an initial semantic surface algorithmically from inputs/assets/time

Asset:
    external data consumed by a source/procedural, not a runtime effect
```

This distinction matters because a `braille_flag_field` source, a `dots_spinner` source, and a `card` source create cells. They should not be forced into the same shape as a filter or shader that modifies existing cells.

# What H0 should lock

```text
+====================================================================================+
|                PHASE H0 — SOURCE / ASSET / PROCEDURAL SOURCE CONTRACT                |
+====================================================================================+

  SOURCE IDENTITY
      SourceId
      SourceDescriptor
      SourceKind / SourceDomain

  SOURCE INPUTS
      typed source inputs
      defaults
      bindability
      runtime mutability, if applicable
      reuse ValueKind / Value / ValueSource

  SOURCE OUTPUT
      produces a semantic Surface
      declares size behavior
      declares role behavior
      declares whether roles are explicit, defaulted, or generated

  ASSETS
      AssetId
      AssetRequirement / AssetSpec
      AssetRef
      asset type
      asset format
      canonical path / logical locator
      no string interpolation in canonical form

  PROCEDURALS
      procedural source descriptor
      typed inputs
      optional asset slots
      deterministic seed policy
      time/clock awareness metadata, but no runtime engine yet

  VALIDATION
      unknown source ids rejected
      unknown asset refs rejected
      missing required assets rejected
      source input kind mismatches rejected
      procedural params validated against descriptor inputs
      asset format/type compatibility checked

  BOUNDARY
      no real asset loading
      no real procedural rendering
      no recipe compiler yet
      no phase/trigger engine
      no runtime store
      no player
      no studio
      no migration aliases

+====================================================================================+
```

# H0 should avoid legacy token syntax

Current recipes use:

```json
"path": "{{ flag_art }}"
```

That is fine as source authoring or legacy syntax, but canonical v3.1 should not use string interpolation for asset references.

Canonical should look more like:

```json
"assets": {
  "flagArt": {
    "type": "brailleDotfield",
    "format": "tui-vfx.braille_flag_asset.v1",
    "locator": {
      "kind": "path",
      "path": "recipes/madeira_flag/assets/base_flag_dots.json"
    }
  }
}
```

and source input/binding should reference it structurally:

```json
"source": {
  "source": "source.brailleFlagField",
  "inputs": {
    "asset": {
      "kind": "asset",
      "id": "flagArt"
    },
    "wave.speed": {
      "kind": "parameter",
      "id": "wave.speed"
    }
  }
}
```

The exact JSON can differ, but the contract rule should be:

```text
Canonical asset references are typed references, not string interpolation.
```

# H0 source descriptors

H0 should probably introduce a descriptor parallel to `EffectDescriptor`:

```text
SourceDescriptor
```

Example conceptual shape:

```text
SourceDescriptor
    id: source.brailleFlagField
    version
    displayName
    kind: procedural
    inputs:
        wave.speed: number
        wave.primaryCycles: number
        shading.base: number
    assets:
        flagArt:
            type: brailleDotfield
            format: tui-vfx.braille_flag_asset.v1
            required: true
    output:
        kind: surface
        size: inputDriven | fixed | hostDriven
        roles: explicit | defaultRole | generated
    lifecycle:
        deterministicWithSeed
        timeAware
        resizeAware
```

Another example:

```text
SourceDescriptor
    id: source.text
    kind: text
    inputs:
        text: text
        alignment: enum(left, center, right)
    output:
        kind: surface
        roles: defaultRole(text)
```

And:

```text
SourceDescriptor
    id: source.card
    kind: structured
    inputs:
        text: text
        bgFill: color
        borderStyle: enum(...)
        padding: rect/insets
    output:
        kind: surface
        roles: generated
```

# Should H0 refactor EffectInputSpec?

F1 introduced `EffectInputSpec`. H0 will need source inputs with almost the same semantics.

There are two acceptable paths:

```text
Preferred if small:
    extract a generic InputSpec / ContractInputSpec
    EffectInputSpec and SourceInputSpec reuse it

Acceptable if refactor is too wide:
    add SourceInputSpec now
    share ValueSpec validation
    document that source/effect input specs are intentionally parallel
```

The important rule is:

```text
Do not fork the value model.
```

Source inputs, effect inputs, parameters, signals, and graph values should all use the same `ValueKind`, `Value`, `ValueSpec`, and `ValueSource` vocabulary wherever possible.

# What H0 should not solve yet

Do not let H0 absorb everything that appears near sources in the examples.

These should remain later phases:

```text
visibility.predicate
    -> later predicate / trigger / runtime value phase

hover/progress/countdown behavior
    -> later runtime signal / procedural input / clock phase

loopback demo signals
    -> later demo/player/profile layer

scene layer motion and follow-lag
    -> later motion/layout/runtime phase

edge crossing / vanish / shadow fade
    -> later motion + shadow integration phase

actual asset file loading
    -> later asset resolver/runtime phase

actual procedural rendering
    -> later source adapter/procedural port phase
```

H0 only needs to define the contract shape and validation.

# H0 tests I would require

```text
source_descriptor_declares_typed_inputs

source_descriptor_declares_required_asset_slot

source_spec_rejects_unknown_source_id

source_spec_rejects_unknown_input

source_spec_rejects_input_kind_mismatch

source_spec_accepts_parameter_value_source_for_bindable_input

source_spec_rejects_graph_value_source_outside_graph_context_if_not_allowed

asset_requirement_schema_is_strict_and_described

asset_ref_rejects_unknown_asset_id

asset_ref_rejects_wrong_asset_format

procedural_source_can_reference_asset_slot

procedural_source_can_accept_runtime_parameter_input

text_source_descriptor_can_default_text_role

card_source_descriptor_can_declare_generated_roles

source_schema_generation_is_current

contract_crate_has_no_legacy_dependencies
```

If H0 includes a tiny proof executor, add:

```text
proof_text_source_produces_surface

proof_procedural_source_sets_explicit_roles

proof_asset_backed_source_validates_asset_ref_without_loading_file
```

But proof execution is optional for H0. The main value is contract shape.

# Do we need more recipes?

Not for H0.

The existing examples cover enough:

```text
text source
card source
procedural source
asset-backed procedural source
asset token reference
runtime-bound procedural input
scene layer source
source-local surface styling
source-local pipeline after generation
```

More recipes would only help inventory edge cases. They are not required to define the bounded H0 contract.

Later, before full migration, I would want one or two examples from these categories:

```text
image source
ANSI source
command capture source
numeric/countdown source
odometer/source-like content generator
```

But those can wait. H0 can be designed to support them without seeing every variant now.

# Next implementer assignment

```text
Phase H0 — Source / Asset / Procedural Source Contract

Goal:
    Add stable contract DTOs for surface-producing sources, asset requirements,
    asset references, and procedural source descriptors before canonical recipe
    schema work begins.

Motivation:
    Existing recipes include scene layers whose initial surface comes from text,
    card, procedural, and asset-backed procedural sources. Canonical recipe schema
    should not invent ad hoc source fields or rely on string interpolation tokens.

Hard constraints:
    - Add DTOs to tui-vfx-contract.
    - Preserve schema/rustdoc/reference rules.
    - Do not implement source recipe schema.
    - Do not implement a recipe compiler.
    - Do not implement real asset loading.
    - Do not implement real procedural rendering.
    - Do not implement runtime stores, phase engine, trigger engine, studio,
      migration, loopback execution, or real effect/source ports.
    - Preserve forbidden dependency boundary.

Required concepts:
    - SourceId
    - SourceDescriptor
    - SourceKind or SourceDomain
    - SourceSpec / SourceInstance
    - SourceInputId / source input specs, preferably reusing ValueSpec
    - AssetId
    - AssetRequirement or AssetSpec
    - AssetRef
    - AssetFormat / AssetKind
    - SourceOutputSpec describing produced Surface semantics
    - Procedural source descriptor support
    - Validation helpers

Canonical rule:
    Asset references are structured references, not string interpolation.

Validation:
    - unknown source id rejected
    - unknown source input rejected
    - input kind mismatch rejected
    - missing required asset rejected
    - unknown asset id rejected
    - asset type/format mismatch rejected
    - required source inputs without defaults must be supplied
    - source descriptors validate their declared inputs/assets/output contract

Schemas:
    Add checked schema roots for source and asset contracts under:
        schemas/v3.1/contract/

Docs:
    Update architecture overview, feature checklist, contract boundary,
    new-kernel index/briefing, and status memo.

Definition of done:
    H0 is done when tui-vfx-contract can describe and validate typed
    surface-producing sources, procedural sources, and asset references;
    generated schemas are strict and described; canonical asset refs are
    structural rather than string-interpolated; and no recipe compiler,
    runtime resolver, asset loader, procedural renderer, studio, migration,
    or real effect/source port leaks into the phase.
```

# Bottom line

I agree with your proposed ordering.

Do:

```text
H0 — Source / Asset / Procedural Source Contract
```

before:

```text
H1 — Canonical Recipe Document Schema
```

We have enough recipe evidence to proceed. More examples can wait until migration/parity planning.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_G4.md</FILE> - <DESC>Architect response approving Phase G4 and assigning Phase H0 source/asset/procedural source contract</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
