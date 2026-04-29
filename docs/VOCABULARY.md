<!-- <FILE>docs/VOCABULARY.md</FILE> - <DESC>Canonical v3.1 vocabulary for contract, schema, and recipe-shape discussions</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>New kernel Phase J1: define validator, migration-smoke, and parity terminology.</WCTX> -->
<!-- <CLOG>0.4.0: MINOR — add J1 migration, structural validation, oracle, and visual parity terms.
0.3.1: PATCH — document Truthy predicate per-kind semantics and maxDuration wire casing.
0.3.0: MINOR — add I0 lifecycle/time/trigger vocabulary and distinctions.
0.2.0: MINOR — add H1 RecipeDocument/RecipeScene/RecipeSceneElement/RecipeElementPipeline terms and clarify source-local pipeline seam.
0.1.0: INIT — define canonical v3.1 terms, legacy distinctions, naming rules, deferrals, and change policy.</CLOG> -->

# v3.1 Vocabulary

## Purpose

This document defines the canonical human vocabulary for v3.1 contract and recipe-shape work.
It complements the Rust-owned contract types and generated JSON Schemas under `schemas/v3.1/contract/`.
Use it when proposing, reviewing, documenting, or migrating any public v3.1 concept.

The core rule is:

```text
Canonical v3.1 is not designed around the old recipe shape.
Legacy recipes are evidence only.
```

Strict v3.1 terms must name the semantic concept they own, not the legacy field that happened to carry similar data.
When an old recipe fails to map, classify the failure as a missing descriptor/source, a migration rule, a deferred feature, or a genuine contract gap.
Do not add aliases to strict v3.1 just to make old JSON spellings validate. Names such as `config.pipeline.step`, `io.outputs[].hint`, `requires_assets`, interpolation tokens, and source-authoring aliases are non-canonical evidence, not vocabulary owners.


## Migration and validation terms

These terms are intentionally separate from old recipe authoring syntax. They describe migration process state, not new runtime behavior.

### Canonical Recipe

Definition:
: A strict v3.1 `RecipeDocument` JSON document that deserializes through `tui_vfx_contract::RecipeDocument` and passes contract validation.

Not the same as:
: A legacy/source recipe, a rendered preview, or visual parity evidence.

### Source Recipe

Definition:
: An old recipe document used as evidence for migration. Current source recipes live under `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` and are not canonical v3.1 documents.

Policy:
: Source recipes may be read as evidence; they must not be mutated by v3.1 validator phases.

### Migrated Recipe

Definition:
: A canonical recipe derived from source-recipe evidence and stored in the migrated v3.1 corpus. Current J0/J1 migrated fixtures live under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.

Not the same as:
: A visual match. Migration can be structural and semantic before it is visual.

### Migration Smoke Fixture

Definition:
: A deliberately small migrated recipe used to pressure-test canonical contracts and validator diagnostics before broad corpus migration.

### Structural Validation

Definition:
: The recipe parses as strict canonical JSON, all ids and references resolve, and `RecipeDocument::validate()` succeeds.

Not the same as:
: Visual parity or runtime execution.

### Semantic Validation

Definition:
: Human/architect review that old recipe intent maps cleanly to canonical v3.1 concepts such as descriptors, sources, graph nodes, value sources, and lifecycle.

### Visual Parity

Definition:
: Evidence that a v3.1 player/probe output matches the old oracle render and passes human review. J1 does not provide visual parity.

Rule:

```text
Valid canonical recipe ≠ visually confirmed recipe
```

### Oracle Recipe and Oracle Render

Definition:
: An old recipe and output from old validator/player/probe tooling used later as comparison evidence. The oracle stack is not the canonical v3.1 validator.

### Primitive Fixture

Definition:
: A migration smoke fixture focused on one primitive family such as filter, mask, sampler, style effect, shader, lifecycle trigger, or role scope.

### Validator Report

Definition:
: Machine-readable output from `tui-vfx-contract-cli validate-recipe`. J1 report schema `v3.1.validator.report.1` contains root, summary counts, and per-recipe errors/warnings.

## Canonical terms

Each entry records the current contract meaning, ownership, schema surface, common confusions, and a small example.
Examples are illustrative, syntax-checked contract fragments, not complete recipe documents. Some snippets omit required sibling fields to keep focus on the term.

### Asset

Definition:
: A declared external or structural data item that a source may consume to produce a semantic surface.

Owns / owned by:
: `AssetSpec` owns the declaration. `AssetRequirement` owns descriptor-level source slots. `AssetRef` owns source-instance references to declared assets.

Rust contract type, if any:
: `AssetSpec`, `AssetId`, `AssetKind`, `AssetFormat`, `AssetLocator`, `AssetRequirement`, `AssetRef`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/asset.schema.json`, `asset-requirement.schema.json`, `asset-ref.schema.json`.

Not the same as:
: A source, which produces a surface. A parameter, which is a public recipe control. A string interpolation token.

Legacy/source-authoring synonyms, if useful:
: `requires_assets`, asset-token fields, and strings such as `{{ flag_art }}` are evidence only. Canonical asset refs are structured ids.

Example:

```json
{
  "id": "flagArt",
  "kind": { "kind": "brailleDotfield" },
  "format": "tui-vfx.braille_flag_asset.v1",
  "locator": { "kind": "logical", "locator": "demo.flagArt" }
}
```

### Binding

Definition:
: A declarative connection from a typed `ValueSource` to a supported target. In the current contract, the supported target is a public parameter and the supported mode is replacement.

Owns / owned by:
: `BindingSpec` owns the connection. `BindingTarget` owns the target shape. `BindingMode` owns the application mode.

Rust contract type, if any:
: `BindingSpec`, `BindingTarget`, `BindingMode`, `ValueSource`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/binding.schema.json`.

Not the same as:
: A signal, which is a host/runtime-provided value. A parameter, which is a public recipe control. A bindable flag, which only says an input or parameter may participate in binding. Runtime binding execution is not implemented by the current contract.

Legacy/source-authoring synonyms, if useful:
: Legacy `requires_bindings`, progress binding fields, visibility predicates, and loopback wiring are evidence. They do not define canonical binding syntax.

Example:

```json
{
  "target": { "kind": "parameter", "id": "progress" },
  "source": { "kind": "signal", "id": "hostProgress" },
  "mode": "replace"
}
```


### Clock

Definition:
: The time sample space used to evaluate recipe-level lifecycle timing. A clock may be monotonic or looping, but the clock only defines how time is sampled; it does not itself choose lifecycle transitions.

Rust contract type, if any:
: `ClockSpec`, `ClockMode`, `DurationSpec`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/clock.schema.json`, `duration.schema.json`.

Not the same as:
: A lifecycle phase, a trigger, or an effect-local schedule.

Example:

```json
{ "mode": "looping", "period": { "kind": "milliseconds", "value": 1200 } }
```

### Lifecycle

Definition:
: The high-level recipe progression from enter to dwell to exit to finished. Recipe lifecycle is declarative contract data; Phase I0 does not implement a runtime player.

Rust contract type, if any:
: `LifecycleSpec`, `PhaseSpec`, `PhaseTiming`, `DwellPolicy`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/lifecycle.schema.json`, `phase.schema.json`.

Not the same as:
: Graph execution, scene visibility execution, template expansion, migration, or source-local pipeline behavior.

Example:

```json
{
  "clock": { "mode": "monotonic" },
  "phases": [
    { "phase": "enter", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 120 } } },
    { "phase": "dwell", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 5000 } } },
    { "phase": "exit", "timing": { "kind": "fixed", "duration": { "kind": "milliseconds", "value": 180 } } }
  ]
}
```

### Phase

Definition:
: A named lifecycle interval with timing semantics. The initial recipe-level phase vocabulary is `enter`, `dwell`, and `exit`.

Rust contract type, if any:
: `LifecyclePhase`, `PhaseSpec`, `PhaseTiming`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/phase.schema.json`.

Not the same as:
: An effect-local schedule or a graph topology step.

### Trigger

Definition:
: A condition that requests an explicit lifecycle action when it fires, such as advancing from dwell to exit. A trigger has condition, latch policy, reset boundary, and action; those semantics must not be hidden in a value source.

Rust contract type, if any:
: `TriggerSpec`, `TriggerCondition`, `ValuePredicate`, `TriggerLatchPolicy`, `TriggerResetBoundary`, `TriggerAction`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/trigger.schema.json`, `value-predicate.schema.json`.

Not the same as:
: A gate, a binding, a loopback, or `GlyphTimelineTriggerSpec`.

Legacy/source-authoring synonyms, if useful:
: Legacy `dwell_until_binding` is evidence for a lifecycle trigger, not a canonical field name. Legacy `dwell_fallback_ms` maps conceptually to a dwell `maxDuration` cap. The public JSON wire name is `maxDuration`; `max_duration` is not canonical.

Truthy predicate semantics, if used:
: `truthy` is a level predicate, not an edge trigger. I0 truth rules are boolean true, integer non-zero, finite number non-zero, string/text non-empty, color value present, and finite duration non-zero. `null`, enum, role, scope, and rect values have no I0 truth rule and must be rejected for `truthy`. Prefer explicit predicates (`isTrue`, `nonZero`, `nonEmpty`, comparisons) when the value kind is known.

Example:

```json
{
  "condition": {
    "source": { "kind": "signal", "id": "userDismissed", "fallback": { "kind": "boolean", "value": false } },
    "predicate": { "kind": "isTrue" }
  },
  "latch": "untilPhaseReset",
  "reset": "phaseStart",
  "action": "advancePhase"
}
```

### Gate

Definition:
: A continuously sampled visibility or execution condition. Gates can show, hide, enable, or suppress something while a condition is true, but they do not request lifecycle transition actions.

Rust contract type, if any:
: Deferred for scene visibility/execution in Phase I0.

JSON/schema surface, if any:
: None in Phase I0.

Not the same as:
: A trigger. Trigger ≠ Gate. A gate does not advance lifecycle.

### Loopback

Definition:
: A preview/demo value provider used when no host value is present. Loopback can supply values for examples and tests, but it does not define trigger semantics.

Rust contract type, if any:
: Deferred for demo/player work in Phase I0.

JSON/schema surface, if any:
: None in Phase I0.

Not the same as:
: A trigger or binding. Trigger ≠ Loopback. A loopback may feed a signal or parameter in a future demo layer, but lifecycle contract semantics still live in `TriggerSpec`.

### Effect-local schedule

Definition:
: Per-effect activation timing such as glyph timelines, poisson bursts, wavefronts, or staggered cells. It belongs to effect/source internals or future descriptors, not recipe-level lifecycle progression.

Rust contract type, if any:
: Existing compositor/style schedule types are evidence only; no canonical Phase I0 contract root adopts their shape.

JSON/schema surface, if any:
: None in Phase I0.

Not the same as:
: Recipe lifecycle, phase timing, trigger, or gate. Lifecycle trigger ≠ GlyphTimelineTriggerSpec.

### Trigger distinction rules

```text
Trigger ≠ Gate
Trigger ≠ Binding
Trigger ≠ Loopback
Trigger ≠ ValueSource
Lifecycle trigger ≠ GlyphTimelineTriggerSpec
```

A Binding / ValueSource supplies a value. A Trigger uses a `ValueSource` plus a typed `ValuePredicate` to request a lifecycle action.

### Demo

Definition:
: A future authoring or tool-facing scenario that supplies example values, host signals, timing, or loopback behavior for playback and QA. Demo data may prove needs, but it is not canonical runtime semantics.

Owns / owned by:
: Deferred demo/player work owns demos. Current contract roots can declare parameters, signals, bindings, descriptors, sources, graphs, and scenes that a demo may exercise.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in the current contract.

Not the same as:
: A canonical recipe document. A runtime signal store. A player. A manifest. A legacy debug recipe.

Legacy/source-authoring synonyms, if useful:
: Loopback examples, progress demos, and debug recipes are evidence only. Their demo scaffolding should lower away or remain outside canonical contract data.

Example:

```text
A future demo may provide a synthetic `hostProgress` signal so a player can showcase a binding; the canonical contract only declares the signal and binding.
```

### Effect

Definition:
: A descriptor-backed operation that declares what it may read, write, target, receive, emit, and how it behaves. In canonical graph execution, effects are instantiated by nodes.

Owns / owned by:
: `EffectDescriptor` owns stable effect identity and capability declarations. `NodeSpec` owns a graph-local effect instance. `GraphSpec` owns the set of descriptors and nodes in a canonical graph.

Rust contract type, if any:
: `EffectDescriptor`, `EffectId`, `EffectDomain`, `CellAccess`, `ScopeSupport`, `WriteSupport`, `EffectLifecycle`, `EffectInputSpec`, `EffectOutputSpec`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/effect-descriptor.schema.json`, `effect-input.schema.json`, `effect-output.schema.json`.

Not the same as:
: A source, which produces an initial semantic surface. A node, which is a graph-local instance of an effect. A legacy pipeline stage. A real effect implementation or adapter.

Legacy/source-authoring synonyms, if useful:
: Old effect family names, shader/filter/mask steps, and stage names may map to descriptors, but their old container shape is not canonical.

Example:

```json
{
  "id": "demo.dim",
  "domain": "frameFilter",
  "inputs": {
    "strength": {
      "value": { "kind": "number", "default": { "kind": "number", "value": 0.5 } },
      "runtimeMutability": "runtime",
      "bindable": true
    }
  }
}
```

### Element

Definition:
: One placed semantic surface inside a scene. Elements provide instance identity, placement, optional lightweight grouping, z-order, clipping policy, and write policies for scene composition.

Owns / owned by:
: `SceneElement` owns one placed local surface. `Scene` owns the declaration-ordered element list.

Rust contract type, if any:
: `SceneElement`, `ElementId`, `ElementPlacement`, `LayerId`, `ClipPolicy`, `CellWritePolicy`, `RoleWritePolicy`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/element.schema.json`, nested from `scene.schema.json`.

Not the same as:
: A role. Element identity describes an instance; role describes per-cell semantics. A node. A full layer graph.

Legacy/source-authoring synonyms, if useful:
: `scene.layers[]` entries may lower to elements when they carry placed surface content, but canonical v3.1 does not adopt `layers[]` as the root concept.

Example:

```json
{
  "id": "toast.card",
  "layer": "foreground",
  "placement": { "x": 12, "y": 3 },
  "zIndex": 10,
  "cellWritePolicy": "skipTransparentEmpty"
}
```

### Graph

Definition:
: The canonical post-compilation container for descriptor-backed effect nodes, public parameters, host signals, declarative bindings, execution order, and optional explicit topology.

Owns / owned by:
: `GraphSpec` owns graph identity, effects, nodes, parameters, signals, bindings, deterministic order, and topology.

Rust contract type, if any:
: `GraphSpec`, `GraphId`, `GraphStep`, `ParallelMergePolicy`, `GraphValueMergePolicy`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/graph.schema.json`, `graph-step.schema.json`.

Not the same as:
: A scene, which composes placed surfaces. A source, which produces a surface. The proof-only `SurfacePipeline`. Old recipe `pipeline` fields.

Legacy/source-authoring synonyms, if useful:
: `pipeline.step.sequence`, `pipeline.step.parallel`, and related authoring fields may lower to `GraphStep::Sequence` or `GraphStep::Parallel`, but their old names are not canonical.

Example:

```json
{
  "id": "toastGraph",
  "nodes": {},
  "order": ["dimBackground", "glowCard"],
  "topology": { "kind": "sequence", "children": [] }
}
```

### Graph value

Definition:
: A graph-local typed value published by a node and consumed by later nodes through the graph value bus.

Owns / owned by:
: `GraphValueId` owns identity. `GraphValueKind` owns type vocabulary. `GraphValueShape` owns frame-wide versus per-cell shape. `NodeOutputSpec` owns node publication. `ValueSource::GraphValue` owns consumption.

Rust contract type, if any:
: `GraphValueId`, `GraphValueKind`, `GraphValueShape`, `GraphValueMergePolicy`, `NodeOutputSpec`, `NodeOutputSource`, `ValueSource::GraphValue`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/graph-value-id.schema.json`, `graph-value-kind.schema.json`, `graph-value-shape.schema.json`, `graph-value-merge-policy.schema.json`, `node-output.schema.json`, `value-source.schema.json`.

Not the same as:
: A parameter, which is a public recipe control. A signal, which is supplied by the host/runtime. A binding, which connects a value source to a parameter target. A legacy hint.

Legacy/source-authoring synonyms, if useful:
: Legacy `hint`, `outputs[].hint`, and intermediate IO names may map to graph values. Do not preserve `hint` as a canonical strict v3.1 field name.

Example:

```json
{
  "outputs": {
    "fieldIntensity": {
      "source": { "kind": "effectOutput", "id": "intensity" }
    }
  }
}
```

### Input

Definition:
: A typed value contract declared by a descriptor or supplied to an instance. Effect inputs and source inputs share the same value model.

Owns / owned by:
: `EffectInputSpec` owns descriptor-local effect input contracts. `SourceInputSpec` owns descriptor-local source input contracts. `NodeSpec.inputs` and `SourceSpec.inputs` own instance-level value sources.

Rust contract type, if any:
: `EffectInputSpec`, `EffectInputId`, `SourceInputSpec`, `SourceInputId`, `ValueSpec`, `Value`, `ValueKind`, `ValueSource`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/effect-input.schema.json`, `source-input.schema.json`, `value.schema.json`, `value-source.schema.json`.

Not the same as:
: A parameter. Inputs are descriptor-local capabilities or instance values; parameters are public recipe controls. A binding. A studio control.

Legacy/source-authoring synonyms, if useful:
: Old `config`, `settings`, `params`, or effect-specific fields may lower to typed inputs, but their original names are not automatically canonical.

Example:

```json
{
  "speed": { "kind": "parameter", "id": "toastSpeed" }
}
```

### Manifest

Definition:
: A future tooling/studio artifact that should expose contract-derived capabilities, controls, diagnostics, and used-by information. It is not a current v3.1 contract root.

Owns / owned by:
: Deferred. Future manifest work should derive from descriptors, schemas, and validated canonical recipe data rather than reverse-engineering legacy recipes.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in the current contract.

Not the same as:
: A recipe document. An effect descriptor. A schema. A demo player configuration.

Legacy/source-authoring synonyms, if useful:
: Studio metadata and UI-control hints are evidence only until manifest work is scoped.

Example:

```text
A future studio manifest may say that `toastSpeed` should render as a slider because its `ValueSpec` is numeric and bounded.
```

### Node

Definition:
: One graph-local instance of an effect descriptor with concrete inputs, scope, write policies, and optional graph-value outputs.

Owns / owned by:
: `NodeSpec` owns node identity and instance-level configuration. `GraphSpec` owns the node collection and execution order/topology.

Rust contract type, if any:
: `NodeSpec`, `NodeId`, `NodeOutputSpec`, `NodeOutputSource`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/node.schema.json`, `node-output.schema.json`.

Not the same as:
: An effect descriptor. A source instance. A scene element. A legacy stage.

Legacy/source-authoring synonyms, if useful:
: Old pipeline step ids may lower to node ids if they instantiate effect descriptors. Authoring ladder examples are evidence only.

Example:

```json
{
  "id": "toastGlow",
  "effect": "demo.glow",
  "scope": { "kind": "role", "role": "Text" },
  "cellWritePolicy": "writeCell"
}
```

### Output

Definition:
: A declared product of a descriptor, source, or node. Outputs make produced data explicit so later validation can reason about surfaces and graph-local values.

Owns / owned by:
: `EffectOutputSpec` owns descriptor-local effect outputs. `SourceOutputSpec` owns produced-surface size and role behavior. `NodeOutputSpec` owns graph value publication.

Rust contract type, if any:
: `EffectOutputSpec`, `EffectOutputId`, `SourceOutputSpec`, `SourceOutputSize`, `SourceRolePolicy`, `NodeOutputSpec`, `NodeOutputSource`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/effect-output.schema.json`, `source-output.schema.json`, `node-output.schema.json`.

Not the same as:
: A write. A parameter. A signal. A rendered frame unless the output contract explicitly says it produces a surface.

Legacy/source-authoring synonyms, if useful:
: Old `hint` output names may lower to node outputs plus graph values. Source output role/size behavior should not be inferred from old field names alone.

Example:

```json
{
  "output": {
    "size": { "kind": "hostDriven" },
    "roles": { "kind": "defaultRole", "role": "Text" }
  }
}
```

### Parameter

Definition:
: A public recipe control with a typed value contract. Parameters are the stable customization surface for recipe authors, presets, profiles, bindings, and future studio controls.

Owns / owned by:
: `ParameterSpec` owns parameter identity and typed value contract. `GraphSpec.parameters` owns graph-level parameter declarations.

Rust contract type, if any:
: `ParameterSpec`, `ParameterId`, `ValueSpec`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/parameter.schema.json`.

Not the same as:
: A signal, which is host/runtime-provided. An effect/source input, which is descriptor-local. A binding, which connects a source to a target. A graph value, which is internal to graph execution.

Legacy/source-authoring synonyms, if useful:
: `params`, `config`, and preset values may map to parameters when they define public customization. Internal implementation knobs should not become parameters automatically.

Example:

```json
{
  "id": "toastSpeed",
  "displayName": "Speed",
  "value": { "kind": "number", "default": { "kind": "number", "value": 1.0 } },
  "bindable": true
}
```

### Player

Definition:
: A future tool or runtime-facing application surface that can execute canonical contract data or demos. It is not the contract itself.

Owns / owned by:
: Deferred. Future player work should consume canonical recipe/runtime data, not legacy recipe internals.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in the current contract.

Not the same as:
: Runtime semantics. A demo profile. A manifest. A recipe schema.

Legacy/source-authoring synonyms, if useful:
: Existing demo players and debug recipe runners are evidence for needs, not canonical vocabulary owners.

Example:

```text
A future player can execute a canonical recipe without reading source-authoring template refs or legacy aliases.
```

### Preset

Definition:
: A values-only authoring bundle that selects values for an existing structure.

Owns / owned by:
: Template/source-authoring work owns presets. The current contract has no preset Rust root.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in `schemas/v3.1/contract/` today. Canonical expanded recipes must not retain preset references.

Not the same as:
: A template or mixin, which can add structure. A profile, which is context/product-mode value selection. A parameter, which is the public control that a preset may set.

Legacy/source-authoring synonyms, if useful:
: Style packs, theme presets, and demo values may lower to parameter values. They must not add canonical structure as presets.

Example:

```text
`dramaticToast` may set `toast.speed = 1.4`; it may not add a new shadow node.
```

### Profile

Definition:
: A values-only authoring bundle selected for environment or product mode.

Owns / owned by:
: Template/source-authoring work owns profiles. The current contract has no profile Rust root.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in `schemas/v3.1/contract/` today. Canonical expanded recipes must not retain profile references.

Not the same as:
: A preset. A runtime signal. A manifest. A template.

Legacy/source-authoring synonyms, if useful:
: `demoMode`, `lowMotion`, `highContrast`, or environment-specific knobs may become profiles in source authoring. Canonical recipe data should contain the resolved values.

Example:

```text
`lowMotion` may replace motion-strength parameters with reduced values during expansion.
```

### RecipeDocument

Definition:
: The strict canonical v3.1 recipe root produced after authoring/lowering and consumed by future compiler/runtime layers.

Owns / owned by:
: `RecipeDocument` owns recipe identity, version, metadata, declared assets, source descriptors, source instances, one canonical graph, and one or more recipe scenes.

Rust contract type, if any:
: `RecipeDocument`, `RecipeId`, `RecipeMetadata`, `AssetSpec`, `SourceDescriptor`, `SourceInstanceId`, `SourceSpec`, `GraphSpec`, `RecipeScene`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/recipe.schema.json`, plus nested recipe metadata, source, asset, graph, and recipe scene schemas.

Not the same as:
: Source-authoring syntax, a legacy debug recipe, a template, a preset, a studio manifest, or a runtime player profile.

Legacy/source-authoring synonyms, if useful:
: Old top-level recipe files are evidence that must lower into this canonical package. Their field names do not define the canonical root.

Example:

```text
A future source-authoring recipe lowers to one RecipeDocument containing concrete graph/source/scene ids and no template, preset, or legacy alias references.
```

### RecipeScene

Definition:
: A canonical recipe scene declaration that describes final dimensions and source-backed elements before actual source rendering and scene composition.

Owns / owned by:
: `RecipeScene` owns a `SceneId`, width, height, and declaration-ordered `RecipeSceneElement` values.

Rust contract type, if any:
: `RecipeScene`, `SceneId`, `RecipeSceneElement`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/recipe-scene.schema.json`.

Not the same as:
: `Scene`, which holds concrete `Surface` elements for composition proof. A graph. A legacy `scene.layers[]` authoring block.

Legacy/source-authoring synonyms, if useful:
: Legacy scene/layer files may lower to recipe scenes and elements, but `scene.layers[]` is not canonical spelling.

Example:

```json
{
  "id": "main",
  "width": 80,
  "height": 24,
  "elements": []
}
```

### RecipeSceneElement

Definition:
: A canonical recipe scene element whose local semantic surface is produced by a declared source instance.

Owns / owned by:
: `RecipeSceneElement` owns element id, optional layer grouping, z-order, placement, source instance reference, optional element-local pipeline reference, clip policy, cell write policy, and role write policy.

Rust contract type, if any:
: `RecipeSceneElement`, `ElementId`, `LayerId`, `ElementPlacement`, `SourceInstanceId`, `RecipeElementPipeline`, `ClipPolicy`, `CellWritePolicy`, `RoleWritePolicy`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/recipe-scene-element.schema.json`.

Not the same as:
: `SceneElement`, which already contains a concrete `Surface`. A graph node. A source descriptor. A legacy layer object.

Legacy/source-authoring synonyms, if useful:
: Old layer entries are evidence for placement/source/pipeline needs. They must lower to canonical ids and policies.

Example:

```text
A recipe element named `heroTitle` references source instance `heroText`; later source rendering produces the surface that scene composition places.
```

### RecipeElementPipeline

Definition:
: An H1 graph/topology reference seam for element-local or source-local processing after a source has produced a surface.

Owns / owned by:
: `RecipeElementPipeline` owns the referenced graph id and optional topology subset. `RecipeSceneElement` owns the optional pipeline reference.

Rust contract type, if any:
: `RecipeElementPipeline`, `GraphId`, `GraphStep`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/recipe-element-pipeline.schema.json`.

Not the same as:
: Old recipe `pipeline` JSON, proof-only `SurfacePipeline`, runtime execution, or a template/migration syntax.

Legacy/source-authoring synonyms, if useful:
: Scene layer IO/filter/shader chains are evidence only. H1 canonically references graph/topology; it does not adopt old local pipeline field names.

Example:

```json
{
  "graph": "heroGraph",
  "topology": { "kind": "node", "node": "fadeIn" }
}
```

### Role

Definition:
: The per-cell semantic channel on a surface. Every surface position has exactly one role.

Owns / owned by:
: `Surface.roles` owns the role channel. `RoleTag` owns the role vocabulary. `RoleWritePolicy` owns write behavior for role updates.

Rust contract type, if any:
: `RoleTag` from `tui-vfx-types`, `RoleWritePolicy`, `RoleWritePolicyKind`, `RoleSpace`, `CellChannel::Role`.

JSON/schema surface, if any:
: Included in `schemas/v3.1/contract/surface.schema.json`, `write.schema.json`, `scope.schema.json`, `effect-descriptor.schema.json`, and related schemas.

Not the same as:
: An element id. A layer id. A UI state such as focused/selected unless explicitly modeled as a role. A field on `Cell`.

Legacy/source-authoring synonyms, if useful:
: Old region, style-region, or semantic group fields may map to roles only when they describe primary per-cell semantic identity.

Example:

```json
{ "kind": "role", "role": "Text" }
```

### Runtime

Definition:
: The future execution layer that consumes validated canonical contract data and applies it to frames over time.

Owns / owned by:
: Deferred. Current contract docs define data and validation boundaries, not production runtime stores or execution semantics.

Rust contract type, if any:
: None as a production runtime root. Current proof execution lives in `tui-vfx-next` and is not the final runtime model.

JSON/schema surface, if any:
: None for production runtime today.

Not the same as:
: The proof engine. A player. A demo. A manifest. Template expansion. Legacy compositor internals.

Legacy/source-authoring synonyms, if useful:
: Loopback demos, live override examples, and old runtime binding behavior are evidence only until runtime phases scope them.

Example:

```text
Runtime may eventually resolve live signal values; strict v3.1 contract data only declares the signal and binding shape today.
```

### Scene

Definition:
: A deterministic composition of placed semantic elements into one final semantic surface.

Owns / owned by:
: `Scene` owns final width, height, and declaration-ordered elements. `SceneOutcome` owns composed output and diagnostics.

Rust contract type, if any:
: `Scene`, `SceneElement`, `SceneOutcome`, `ElementId`, `LayerId`, `ElementPlacement`, `ClipPolicy`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/scene.schema.json`, `element.schema.json`, `outcome.schema.json`.

Not the same as:
: A graph. A source. A full layer graph. A legacy recipe `scene.layers[]` shape.

Legacy/source-authoring synonyms, if useful:
: Scene/layer debug recipes are evidence for how multiple surfaces need placement and composition. Canonical terms remain `Scene` and `SceneElement`.

Example:

```json
{
  "width": 80,
  "height": 24,
  "elements": []
}
```

### Scope

Definition:
: A small algebra for selecting cells: all cells, role cells, rectangles, row ranges, or column ranges. Coordinate and role spaces are operation-level context.

Owns / owned by:
: `ScopeSpec` owns shape. `ScopeKind` owns descriptor support vocabulary. `CoordinateSpace` and `RoleSpace` own operation-level evaluation spaces.

Rust contract type, if any:
: `ScopeSpec`, `ScopeKind`, `CoordinateSpace`, `RoleSpace`, `ScopeSupport`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/scope.schema.json`, with support declarations in `effect-descriptor.schema.json`.

Not the same as:
: A mask. A selector language. A scene element. A graph topology predicate. A legacy region alias.

Legacy/source-authoring synonyms, if useful:
: Old target/region fields may lower to scopes when they select surface cells. Predicate-based visibility remains deferred.

Example:

```json
{ "kind": "rect", "rect": { "x": 2, "y": 1, "width": 20, "height": 3 } }
```

### Signal

Definition:
: A host/runtime-provided typed value contract that recipe data may reference declaratively.

Owns / owned by:
: `SignalSpec` owns signal identity, required/fallback expectations, and typed value contract. `ValueSource::Signal` owns references to signals.

Rust contract type, if any:
: `SignalSpec`, `SignalId`, `ValueSpec`, `ValueSource::Signal`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/signal.schema.json`, `value-source.schema.json`.

Not the same as:
: A parameter, which is a public recipe control. A graph value, which is node-produced. A runtime store implementation. A demo loopback signal.

Legacy/source-authoring synonyms, if useful:
: Progress meters, wave/envelope examples, host state, and loopback values may map to signals when the host provides them. Demo-only signal generation is deferred.

Example:

```json
{
  "id": "hostProgress",
  "required": false,
  "value": { "kind": "number", "range": { "min": 0.0, "max": 1.0 } }
}
```

### Source

Definition:
: A descriptor for something that produces an initial semantic `Surface`. A source is not an effect over an existing surface.

Owns / owned by:
: `SourceDescriptor` owns source identity, kind, input contracts, asset requirements, output contract, and lifecycle metadata.

Rust contract type, if any:
: `SourceDescriptor`, `SourceId`, `SourceKind`, `SourceInputSpec`, `AssetRequirement`, `SourceOutputSpec`, `SourceLifecycle`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/source-descriptor.schema.json`, `source-input.schema.json`, `source-output.schema.json`.

Not the same as:
: An asset consumed by a source. A source instance. An effect. A graph node. A scene element.

Legacy/source-authoring synonyms, if useful:
: `scene.layers[].source`, `config.content`, ANSI/image/command-capture source examples, and procedural spinner examples are evidence only.

Example:

```json
{
  "id": "demo.textCard",
  "kind": { "kind": "card" },
  "inputs": {},
  "assets": {},
  "output": { "size": { "kind": "inputDriven" }, "roles": { "kind": "generated" } }
}
```

### Source instance

Definition:
: A concrete use of a source descriptor with instance-level typed inputs and structural asset refs.

Owns / owned by:
: `SourceSpec` owns the descriptor reference, input `ValueSource` map, and asset slot references.

Rust contract type, if any:
: `SourceInstanceId`, `SourceSpec`, `SourceId`, `SourceInputId`, `ValueSource`, `AssetRef`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/source-instance-id.schema.json`, `source.schema.json`.

Not the same as:
: A source descriptor. A scene element. A graph node. A legacy layer source object.

Legacy/source-authoring synonyms, if useful:
: Old source blocks inside layer definitions may lower to `SourceSpec`, but the strict v3.1 shape uses descriptor ids, typed value sources, and asset refs.

Example:

```json
{
  "source": "demo.textCard",
  "inputs": {
    "text": { "kind": "literal", "value": { "kind": "text", "value": "Hello" } }
  },
  "assets": {}
}
```

### Source-local pipeline

Definition:
: A recognized element-local/source-local processing seam after a source-produced surface exists and before or as it participates in scene/graph composition.

Owns / owned by:
: H1 provides a reference seam through `RecipeElementPipeline`; execution, lowering, and source-authoring syntax remain deferred.

Rust contract type, if any:
: `RecipeElementPipeline`, `GraphId`, `GraphStep`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/recipe-element-pipeline.schema.json`.

Not the same as:
: `SurfacePipeline`, which is a proof-only next-crate schema. `GraphSpec`, which is the canonical graph container. Old recipe `pipeline` syntax.

Legacy/source-authoring synonyms, if useful:
: Scene layer IO/filter/shader chains are evidence that a local processing hook is needed. Their field names do not define the canonical shape.

Example:

```text
A source-local pipeline may reference graph node `dimCard` for element `heroCard`; H1 validates the reference but does not execute legacy layer-pipeline JSON.
```

### Surface

Definition:
: A dense rectangular grid of visual cells plus one semantic role per cell, in row-major order.

Owns / owned by:
: `Surface` owns width, height, cells, roles, and metadata. `SurfaceMetadata` owns producer/layer labels only.

Rust contract type, if any:
: `Surface`, `SurfaceMetadata`, `Cell` from `tui-vfx-types`, `RoleTag`.

JSON/schema surface, if any:
: `schemas/v3.1/contract/surface.schema.json`.

Not the same as:
: A ratatui widget, terminal buffer, scene, source, asset, or graph.

Legacy/source-authoring synonyms, if useful:
: Terminal buffers and rendered frames can be adapted to surfaces, but v3.1 stays grid-first and ecosystem-agnostic.

Example:

```json
{
  "width": 1,
  "height": 1,
  "cells": [
    {
      "ch": "A",
      "fg": { "r": 255, "g": 255, "b": 255, "a": 255 },
      "bg": { "r": 0, "g": 0, "b": 0, "a": 0 },
      "mods": {
        "bold": false,
        "italic": false,
        "underline": false,
        "dim": false,
        "reverse": false,
        "strikethrough": false,
        "slow_blink": false,
        "rapid_blink": false,
        "hidden": false
      }
    }
  ],
  "roles": ["Text"],
  "metadata": {}
}
```

### Template

Definition:
: A source-authoring construct that defines reusable structure, slots, defaults, and sealed fields before deterministic expansion.

Owns / owned by:
: Template composition/source-authoring owns templates. Runtime and canonical recipe validation consume only the expanded output.

Rust contract type, if any:
: None in the current contract.

JSON/schema surface, if any:
: None in `schemas/v3.1/contract/` today. Canonical expanded recipes must not retain template references.

Not the same as:
: A preset or profile, which are values-only. A canonical recipe. A runtime inheritance mechanism.

Legacy/source-authoring synonyms, if useful:
: Inheritance, reusable effect bundles, mixins/traits, slots, and old composition shortcuts are source-authoring concepts only.

Example:

```text
A `toast` template may define element and node skeletons; canonical v3.1 sees only concrete elements, nodes, ids, values, scopes, and policies.
```

## Deprecated / legacy terms

Use these terms only when discussing migration evidence, source authoring, compatibility tests, or historical behavior.
Do not introduce them into strict v3.1 contract schemas as aliases.

| Legacy or overloaded term | Canonical handling |
|---|---|
| `hint`, `outputs[].hint` | Prefer `NodeOutputSpec`, `GraphValueId`, `GraphValueKind`, and `GraphValueShape`. |
| `layer` as execution graph | Canonical D1 has only optional `LayerId` grouping on `SceneElement`; a full layer graph is not present. |
| `scene.layers[]` | Evidence for `SceneElement`, source instances, placement, visibility, and future source-local pipelines. Not a canonical root spelling. |
| `pipeline` as old recipe container | Canonical graph topology uses `GraphStep::{node,sequence,parallel}`. `SurfacePipeline` is proof-only under `schemas/v3.1/next/`. |
| `stage` | Usually maps to a `NodeSpec` or `GraphStep` after descriptor/source classification. `PipelineStage` is proof-only. |
| `{{ asset_id }}` interpolation | Use structural `AssetRef { id }`; interpolation tokens are rejected in canonical asset refs/locators. |
| `config`, `settings`, `params` | Classify each field as a descriptor input, public `ParameterSpec`, `SignalSpec`, source input, or implementation-private detail. |
| `config.pipeline.step` and other old path spellings | Evidence for graph topology or node lowering only; do not preserve the old path as a canonical term. |
| `requires_bindings` | Use `ParameterSpec`, `SignalSpec`, `BindingSpec`, and `ValueSource` where applicable. |
| preset/profile references in canonical data | Expand away before canonical validation/runtime. Presets and profiles are values-only authoring inputs. |
| template/mixin/inheritance references in canonical data | Expand away before canonical validation/runtime. Runtime inheritance is forbidden. |
| legacy role aliases | Use built-in `RoleTag` variants or explicit custom roles. Aliases belong in migration tools only. |
| source-authoring visibility predicates | Deferred phase/trigger/visibility work; do not model as scope unless it truly selects cells. |
| loopback/demo signal execution | Deferred demo/player/runtime work; signals can be declared but demo generation is not canonical execution. |

## Naming rules

1. Name the owner, not the old field.
   Use `AssetRef`, not `flagToken`; `GraphValueId`, not `hint`; `SceneElement`, not `layer`, when those are the owning concepts.

2. Keep descriptor and instance names separate.
   A `SourceDescriptor` or `EffectDescriptor` declares capability. A `SourceSpec` or `NodeSpec` instantiates it.

3. Keep public controls and runtime values separate.
   Use `ParameterSpec` for recipe API controls, `SignalSpec` for host/runtime values, and `GraphValueId` for node-produced graph-local values.

4. Keep roles per-cell and ids per-object.
   `RoleTag` classifies cells. `ElementId`, `NodeId`, `SourceId`, `AssetId`, and `GraphValueId` identify objects or values.

5. Keep authoring composition out of canonical data.
   Template, mixin, preset, and profile references must expand to concrete canonical ids, values, scenes, nodes, scopes, and policies.

6. Keep proof names out of stable contract when they are proof-only.
   `SurfacePipeline`, `PipelineStage`, `PipelineSampler`, `DimEffect`, and `ExplicitRoleWriteEffect` are proof artifacts unless a later phase explicitly promotes a concept through the contract process.

7. Use strict schema names and camelCase JSON surfaces generated from Rust.
   Serde owns wire shape, Schemars owns generated schema, and Rustdoc owns descriptions. Do not hand-invent public JSON fields outside the Rust contract path.

8. Prefer additive capability names over compatibility aliases.
   If migration needs an old spelling, put it in the legacy loader or migration report, not in strict v3.1 schemas.

## Terms intentionally deferred

These terms are recognized but not current canonical contract roots.
They may appear in planning docs only with an explicit deferral note.

| Term | Deferred until | Current rule |
|---|---|---|
| Source-local pipeline execution/lowering | Future source/graph integration phase | H1 provides `RecipeElementPipeline` as a graph/topology reference seam only. |
| Runtime parameter/signal stores | Runtime phase | Current contract declares values, sources, and bindings; it does not execute live stores. |
| Binding execution and override precedence | Runtime phase | `BindingSpec` is declarative only. |
| Phase graph / trigger / dwell / visibility engine | Later lifecycle/event phase | Do not encode visibility predicates as scopes unless they are cell scopes. |
| Demo loopback execution | Demo/player phase | Loopback recipes are evidence for signals and profiles, not canonical execution. |
| Studio manifest | Studio/tooling phase | Studio should derive controls from contract data and future manifests, not recipes. |
| Real asset loading/resolution | Asset runtime phase | `AssetSpec` and `AssetRef` are structural declarations only. |
| Real procedural rendering | Source implementation phase | `SourceDescriptor` can describe procedural sources; it does not render them. |
| Real effect/source ports | Porting phases after contracts lock | Port behavior through descriptors, sources, scopes, values, writes, and diagnostics. |
| Legacy migration implementation | Migration phase after strict shape | Legacy aliases never enter strict canonical v3.1. |
| Template expansion implementation | Source-authoring/compiler phase | D2 locks semantics; runtime receives expanded canonical data only. |
| Full layer graph / complex blending | Later scene/composition phase | Current `LayerId` is lightweight grouping only. |

## Change policy

Update this file whenever a phase adds, removes, renames, or changes public contract vocabulary.
Treat vocabulary changes as contract changes, not copy edits.

Required update checklist:

1. Identify the owner.
   Every term must name the Rust type, schema root, authoring layer, or deferred phase that owns it.

2. Preserve the source of truth.
   Rust contract types and generated schemas remain authoritative for machine shape. This document explains terms for humans and agents.

3. Add the confusion boundary.
   Every new or changed term must state what it is not the same as.

4. Keep legacy mappings quarantined.
   Record legacy/source-authoring synonyms as evidence only. Do not add strict v3.1 aliases.

5. Keep deferrals explicit.
   If a term is recognized but not implemented, put it under “Terms intentionally deferred” with the owning future phase.

6. Keep examples small and non-normative.
   Examples should show the concept home, not define a complete recipe document before that schema exists.

7. Update metadata.
   Bump the version, update `<WCTX>` and `<CLOG>`, and keep the footer in sync.

Schema lock guideline:

```text
Schema lock does not mean no future additions.
It means future additions are additive capabilities, not corrections to basic concepts.
```

<!-- <FILE>docs/VOCABULARY.md</FILE> - <DESC>Canonical v3.1 vocabulary for contract, schema, and recipe-shape discussions</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
