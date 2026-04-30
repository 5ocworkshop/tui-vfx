<!-- <FILE>docs/new_kernel/V31_RENDERING_BOUNDARY_RULES.md</FILE> - <DESC>Formal v3.1 recipe-to-playback boundary, lowering ownership, and compositor adapter decision rules</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>v3.1 rendering boundary discipline for recipe migration, player playback, compositor backend lowering, and native adapter ownership.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — anchor V2/V3/v3.1 rendering pathways and hard layer responsibilities.</CLOG> -->

# v3.1 Rendering Boundary Rules

This document defines where v3.1 data remains canonical, where it is sampled, where it is lowered, and when new behavior belongs in the contract model, the player, the compositor backend adapter, or the compositor itself.

The goal is to preserve the discipline of the v3.1 schema and data model while preventing the rendering boundary from becoming ad hoc.

## Executive rule

Keep the v3.1 contract as the source of truth until the selected backend needs an executable render plan.

The player samples recipes. The backend lowerer maps sampled graph intent to an executable backend plan. The compositor executes visual semantics. Presentation surfaces display evidence. Do not collapse those roles.

```text
V2/V3 oracle behavior
        │
        ▼
v3.1 authored contract recipe
        │
        ▼
player resolves timing/source/input values only
        │
        ▼
backend lowerer maps graph nodes to compositor primitives
        │
        ▼
compositor executes the visual effect
        │
        ▼
CLI/player evidence proves parity
```

## Historical oracle pathways

V2 and V3 recipe tooling are migration oracles. They prove the intended visual behavior for existing debug recipes. They are not the architecture to copy into v3.1.

### V2 `_DEPRECATED_` pathway

```text
┌──────────────────────────────────────────────┐
│ V2 debug recipe JSON                         │
│ recipes/debug_recipes/.../_DEPRECATED_*.json │
│                                              │
│ config.message                               │
│ config.layout                                │
│ config.border                                │
│ config.lifecycle                             │
│ config.pipeline.mask/style/filter/sampler    │
└───────────────────────┬──────────────────────┘
                        │ legacy recipe tooling
                        ▼
┌──────────────────────────────────────────────┐
│ recipe-probe                                 │
│ /usr/projects/tui-vfx-recipes                │
│                                              │
│ Reads V2 config directly, builds legacy      │
│ pipeline intent, samples entering/dwelling/  │
│ exiting, and emits frame/cell evidence.      │
└───────────────────────┬──────────────────────┘
                        │ primitive execution
                        ▼
┌──────────────────────────────────────────────┐
│ tui-vfx compositor primitives                │
│                                              │
│ MaskSpec / FilterSpec / SamplerSpec / Style  │
│ own the reusable effect semantics.           │
└───────────────────────┬──────────────────────┘
                        │ evidence
                        ▼
┌──────────────────────────────────────────────┐
│ V2 oracle evidence                           │
│                                              │
│ cells, reconstructed rows, foreground/        │
│ background, touched cells, causation.         │
└──────────────────────────────────────────────┘
```

### V3 pathway

```text
┌──────────────────────────────────────────────┐
│ V3 debug recipe JSON                         │
│ recipes/debug_recipes/.../*.json             │
│                                              │
│ schema_version: 3                            │
│ config.pipeline.step                         │
│ small pipeline algebra                       │
└───────────────────────┬──────────────────────┘
                        │ V3 recipe tooling
                        ▼
┌──────────────────────────────────────────────┐
│ V3 parser / normalizer / probe path          │
│ /usr/projects/tui-vfx-recipes                │
│                                              │
│ Normalizes the step tree, samples lifecycle, │
│ and emits preview/probe evidence.            │
└───────────────────────┬──────────────────────┘
                        │ primitive execution
                        ▼
┌──────────────────────────────────────────────┐
│ tui-vfx compositor primitive path            │
│                                              │
│ Still ultimately relies on compositor-owned  │
│ mask/filter/sampler/style behavior.          │
└───────────────────────┬──────────────────────┘
                        │ evidence
                        ▼
┌──────────────────────────────────────────────┐
│ V3 oracle evidence                           │
│                                              │
│ Useful when V2 and V3 agree; V2 wins when    │
│ the two conflict during v3.1 migration.      │
└──────────────────────────────────────────────┘
```

## v3.1 target pathway

This is the path all migrated recipes must prove through `tui-vfx-player-cli render-backend --backend compositor --composition-mode native --fail-on-fallback`.

```text
┌──────────────────────────────────────────────┐
│ v3.1 recipe JSON                             │
│ recipes/v3.1/debug_recipes/.../*.json        │
│                                              │
│ Contract data model:                         │
│ - metadata                                   │
│ - lifecycle                                  │
│ - sources                                    │
│ - scenes                                     │
│ - graph nodes                                │
│ - descriptor-backed primitive inputs         │
└───────────────────────┬──────────────────────┘
                        │ parse + validate
                        ▼
┌──────────────────────────────────────────────┐
│ Contract / descriptor layer                  │
│ crates/tui-vfx-contract                      │
│ descriptors/v3.1/packs/primitive.json        │
│                                              │
│ Owns authored vocabulary and validation.     │
│ Does not execute visual semantics.           │
└───────────────────────┬──────────────────────┘
                        │ sample request
                        ▼
┌──────────────────────────────────────────────┐
│ Player layer                                 │
│ crates/tui-vfx-player                        │
│                                              │
│ Owns sampled runtime facts:                  │
│ - selected phase                             │
│ - phase_t / timing                           │
│ - source rows / styled source cells          │
│ - resolved graph inputs/signals              │
│                                              │
│ Does not implement native compositor effect  │
│ semantics.                                   │
└───────────────────────┬──────────────────────┘
                        │ backend request
                        ▼
┌──────────────────────────────────────────────┐
│ Compositor backend lowerer                   │
│ crates/tui-vfx-player-backend-compositor     │
│                                              │
│ Translates sampled v3.1 graph nodes into:    │
│                                              │
│ A) compositor-native CompositionSpec         │
│    MaskSpec / FilterSpec / SamplerSpec /     │
│    ShaderLayerSpec / style-capable specs     │
│                                              │
│ B) narrow backend-owned adapter stages only  │
│    when the compositor lacks an exact        │
│    reusable primitive.                       │
│                                              │
│ Rejects unsupported semantics honestly.      │
└───────────────────────┬──────────────────────┘
                        │ render
                        ▼
┌──────────────────────────────────────────────┐
│ tui-vfx compositor                           │
│ crates/tui-vfx-compositor                    │
│                                              │
│ Owns reusable visual semantics:              │
│ - masks                                      │
│ - filters                                    │
│ - samplers                                   │
│ - shaders                                    │
│ - styles                                     │
└───────────────────────┬──────────────────────┘
                        │ evidence
                        ▼
┌──────────────────────────────────────────────┐
│ CLI / player evidence                        │
│ tui-vfx-player-cli render-backend            │
│                                              │
│ Reports rows, styledCells, letter evidence,  │
│ compositionSpecSummary, native status,       │
│ fallback status, and diagnostics.            │
└──────────────────────────────────────────────┘
```

### Concrete lowering example: `mask.cellular`

```text
v3.1 graph node
  effect: mask.cellular
  inputs: pattern, seed, cellCount
        │
        ▼
backend lowerer
  validate supported inputs and enum values
  map v3.1 field names to compositor vocabulary
        │
        ▼
CompositionSpec
  MaskSpec::Cellular {
    pattern,
    seed,
    cell_count
  }
        │
        ▼
compositor
  Cellular::is_visible(ctx)
        │
        ▼
CLI evidence
  rows/styledCells/letterCellEvidence
```

Do not implement `mask.cellular` semantics inside the player to make rows match. If v3.1 native output differs from the V2 oracle, fix the recipe mapping, descriptor vocabulary, backend lowering, or compositor primitive. The player should only supply the sampled source grid and resolved inputs.

```text
┌──────────────────────────────────────────────────────────────────┐
│ v3.1 contract recipe                                             │
│                                                                  │
│ Durable authored intent: sources, scenes, graph nodes, values,   │
│ lifecycle, scopes, write policies, descriptors, metadata.        │
└───────────────────────────────┬──────────────────────────────────┘
                                │ Parse, validate, sample.
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│ Player model                                                     │
│                                                                  │
│ Runtime facts: current phase, phase progress, loop time, source  │
│ rows, styled cells, resolved input values, graph values.         │
└───────────────────────────────┬──────────────────────────────────┘
                                │ Backend selection.
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│ Backend boundary                                                 │
│                                                                  │
│ This is where v3.1 graph semantics become a backend-specific     │
│ executable plan. For the compositor backend, this means native   │
│ compositor IR plus narrowly-scoped adapter stages when required. │
└───────────────────────────────┬──────────────────────────────────┘
                                │ Render.
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│ Evidence and presentation                                        │
│                                                                  │
│ Rows, styled cells, diagnostics, native/fallback status, CLI     │
│ reports, player UI, studio UI, and client API output.            │
└──────────────────────────────────────────────────────────────────┘
```

## Why the boundary exists

The v3.1 contract models authored intent. The compositor executes visual operations. They are not the same responsibility.

If the contract tried to be a compositor, the schema would fill with engine internals and become unstable for recipe authors. If the compositor tried to understand the full v3.1 document, it would inherit source resolution, lifecycle, graph, descriptor, and runtime binding concerns that belong above rendering. The boundary keeps each layer small enough to reason about.

```text
Good separation:

  recipe contract       says what should happen
  player                decides what sample is being rendered now
  backend lowerer       converts sampled intent to executable render stages
  compositor            applies reusable visual operations
  UI/CLI/studio         present the same evidence from the same path

Bad separation:

  recipe contract       stores renderer-private staging tricks
  player UI             reimplements recipe effects
  CLI                   has its own renderer
  compositor backend    silently drops graph semantics
  compositor            becomes aware of full recipe documents
```

## Layer responsibilities

### Contract layer

Owned by `crates/tui-vfx-contract`, checked schemas, descriptors, and v3.1 recipe JSON.

Contract layer owns:
- Stable recipe document shape.
- Source, scene, graph, node, lifecycle, value, binding, scope, and write-policy vocabulary.
- Descriptor-visible primitive ids and input/output metadata.
- Author-facing naming and validation.

Contract layer must not own:
- Backend-specific structs such as native style-stage enums.
- Compositor implementation details.
- Per-frame mutable render buffers.
- Legacy compatibility quirks that are not durable authored intent.

Why: the contract is for recipe portability and authoring stability. It should not change just because one backend needs a different execution strategy.

### Player layer

Owned primarily by `crates/tui-vfx-player`.

Player layer owns:
- Loading a validated recipe into runtime structures.
- Selecting lifecycle phase and normalized progress.
- Resolving value sources, signals, graph values, and runtime overrides.
- Building source rows and source-owned styled cells.
- Producing backend-neutral render IR and deterministic evidence.
- Applying simple player fallback adapters when a backend is not selected or when compatibility evidence requires it.

Player layer must not own:
- UI-only layout choices.
- Compositor-internal filter/shader implementation details.
- Backend-native lowering decisions that vary by backend.
- Native compositor effect semantics for masks, filters, shaders, samplers, or styles.

Why: the player is the single sampled view of the recipe. CLI, player UI, studio UI, and clients should all observe the same sampled model.

### Backend lowering layer

Owned by backend adapter crates such as `crates/tui-vfx-player-backend-compositor`.

Backend lowering owns:
- Translating sampled v3.1 graph nodes into the selected backend's executable plan.
- Rejecting unsupported semantics honestly instead of silently dropping fields.
- Deciding whether a primitive maps directly to compositor IR or needs a backend-owned adapter stage.
- Producing diagnostics and `nativeLoweringSucceeded`/`fallbackUsed` evidence.
- Mapping v3.1 field names to existing compositor vocabulary when the concepts are the same.

Backend lowering must not own:
- New author-facing schema fields.
- Descriptor vocabulary without updating descriptors and docs.
- UI presentation behavior.
- Broad replacement rendering engines hidden behind helper functions.
- Reimplementations of compositor primitives that already exist as reusable compositor operations.

Why: lowering is the contract between stable authored intent and backend execution. It is allowed to be backend-specific, but it must be explicit and auditable.

### Compositor layer

Owned by `crates/tui-vfx-compositor`.

Compositor owns:
- Reusable visual operations that are backend-independent within the compositor engine.
- Generic filter, sampler, shader, mask, style, timing, and cell-buffer operations.
- Data structures that are stable for compositor consumers beyond the v3.1 player.

Compositor must not own:
- Full v3.1 recipe parsing.
- Descriptor-pack semantics.
- Studio controls.
- Legacy recipe migration decisions.

Why: the compositor should be reusable rendering machinery. It should not become a second copy of the v3.1 player.

### Presentation layer

Owned by CLI, player UI, studio UI, and client surfaces.

Presentation owns:
- Rendering reports to terminal output or JSON.
- Progressive disclosure and ergonomics.
- User controls and display layout.
- Visual inspection workflows.

Presentation must not own:
- Primitive effect semantics.
- Separate implementations of masks, styles, samplers, or filters.
- Recipe migration logic.

Why: if presentation reimplements effects, CLI evidence and UI playback can drift.

## Recipe playback flow

```text
┌──────────────────────┐
│ v3.1 recipe JSON     │
│ style.pulse node     │
│ source.card          │
│ lifecycle dwell      │
└──────────┬───────────┘
           │ validate + deserialize
           ▼
┌──────────────────────┐
│ RecipeDocument       │
│ GraphSpec / NodeSpec │
│ SourceSpec           │
│ LifecycleSpec        │
└──────────┬───────────┘
           │ sample request: phase=dwell, phase_t=0.25
           ▼
┌──────────────────────┐
│ Player sample        │
│ resolved inputs      │
│ source rows          │
│ source styled cells  │
└──────────┬───────────┘
           │ compositor backend selected
           ▼
┌──────────────────────────────────────┐
│ Lowering decision                    │
│                                      │
│ style.pulse has no direct generic    │
│ compositor IR primitive with exact   │
│ V2 parity semantics.                 │
└──────────┬───────────────────────────┘
           │ create backend-native stage
           ▼
┌──────────────────────────────────────┐
│ NativeStyleStage::Pulse              │
│                                      │
│ color=rgba(255,100,100,255)          │
│ frequency=2.0                        │
│ apply_to=both                        │
└──────────┬───────────────────────────┘
           │ render source grid + apply stage
           ▼
┌──────────────────────────────────────┐
│ PlayerRenderIrReport                 │
│                                      │
│ rows                                 │
│ styledCells                          │
│ compositionSpecSummary.styleStages=1 │
│ nativeLoweringSucceeded=true         │
│ fallbackUsed=false                   │
└──────────┬───────────────────────────┘
           │ same evidence consumed by all surfaces
           ▼
┌───────────────┬───────────────┬───────────────┐
│ CLI JSON      │ Player UI     │ Studio UI     │
│ validation    │ playback      │ controls      │
└───────────────┴───────────────┴───────────────┘
```

## Decision tree: where should new behavior go?

Use this decision tree before adding a primitive adapter, schema field, or compositor operation.

```text
Start
 │
 ├─ Is this a durable author-facing concept?
 │    │
 │    ├─ yes → contract model + descriptor + schema + docs
 │    │        Example: a new primitive input authors should use.
 │    │
 │    └─ no
 │
 ├─ Is this required to resolve recipe state at sample time?
 │    │
 │    ├─ yes → player layer
 │    │        Example: lifecycle phase, runtime signal, graph value.
 │    │
 │    └─ no
 │
 ├─ Is this a reusable visual operation that other compositor users need?
 │    │
 │    ├─ yes → compositor IR/filter/sampler/shader/style primitive
 │    │        Example: generic fade-to-canvas, greyscale, wipe mask.
 │    │
 │    └─ no
 │
 ├─ Is this a backend-specific translation or compatibility shim?
 │    │
 │    ├─ yes → backend lowering/native adapter stage
 │    │        Example: V2-compatible pulse channel modulation until
 │    │        the compositor has a stable generic style-modulation IR.
 │    │
 │    └─ no
 │
 └─ Is this only about how humans see controls/evidence?
      │
      ├─ yes → CLI/player UI/studio UI
      └─ no  → stop; write a design note before coding
```

## Decision table

| Question | Put it in contract | Put it in player | Put it in backend lowering | Put it in compositor | Put it in UI/CLI |
| --- | --- | --- | --- | --- | --- |
| Is it stable authored vocabulary? | Yes | No | No | No | No |
| Does it resolve signals, graph values, or lifecycle sample state? | No | Yes | Usually no | No | No |
| Does it translate v3.1 semantics into a selected backend plan? | No | No | Yes | No | No |
| Is it reusable rendering math independent of v3.1 recipes? | No | No | Maybe temporarily | Yes | No |
| Is it a migration compatibility quirk? | Usually no | Maybe for fallback | Yes | Usually no | No |
| Is it visual presentation of evidence or controls? | No | No | No | No | Yes |

## Promotion rules

Backend-native adapter stages are allowed, but they must not become a hidden compositor.

Promote an adapter behavior into the compositor when:
- Multiple primitives need the same operation family.
- The behavior is not v3.1-specific.
- The operation can be named generically without legacy vocabulary.
- It has a small stable parameter set.
- It would simplify multiple backend lowering branches.

Keep behavior in backend adapter stages when:
- It exists to preserve exact legacy recipe parity during migration.
- It is only needed by the v3.1 player/compositor bridge.
- It depends on player-resolved source rows or styled-cell evidence.
- Promoting it would force the compositor to know v3.1 graph semantics.

Keep behavior in the player fallback path when:
- It is needed for non-compositor playback or evidence generation.
- It operates on the player styled grid after source resolution.
- It must remain backend-neutral for CLI/player API consumers.

## Example: `style.pulse`

`style.pulse` has authored intent in the recipe:

```text
pulse color      → target color
frequency        → temporal cadence
applyTo          → foreground/background/both channel selection
activePhases     → lifecycle gate
scope            → affected cells
```

The contract and descriptor should know those fields. They should not know how a renderer loops over cells.

At playback time the player knows:

```text
current phase = dwell
phase_t       = 0.25
source rows   = 35x3 rounded card
base fg/bg    = source-owned styled cells
```

The compositor backend lowerer then decides:

```text
style.pulse does not map to a current generic compositor IR operation
with exact V2 parity, so lower it to NativeStyleStage::Pulse.
```

The backend render stage does the execution work:

```text
for every scoped cell:
  read existing foreground/background
  compute strength from clock and frequency
  lerp selected channels toward pulse color
  write updated styled cell
```

Why not put this loop in the recipe? Because recipes should not contain engine loops.

Why not put full recipe awareness in the compositor? Because the compositor should not parse graph nodes, lifecycle phases, value sources, or descriptor packs.

Why not build a separate compositor? Because then every primitive must be reimplemented and kept in parity with the existing compositor path. The current target is one model and one playback path, not parallel renderers.

## Hard rules for future slices

1. Do not add a schema field to solve a backend convenience problem.
2. Do not silently ignore unsupported graph inputs, outputs, scopes, or write policies. Lower them honestly or reject native mode with diagnostics.
3. Do not add UI-only rendering logic for primitive semantics.
4. Do not make the compositor parse full v3.1 recipes.
5. Do not make backend-native adapter stages broader than the primitive semantics being proven.
6. When two adapter stages duplicate math, extract a backend helper first; promote to compositor only after the generic operation is clear.
7. When a primitive is migrated from V2, prove it through CLI evidence and the same path used by player UI and studio UI.
8. If a lowering decision is ambiguous, document the decision before coding the next slice.
9. If a compositor primitive already exists, lower to it instead of implementing the primitive in the player.
10. Player fallback adapters are not proof of strict-native compositor support.

## Evidence requirements at the boundary

Every strict-native migration slice should produce evidence for the boundary:

```text
recipe path
V2 oracle path
phase and phase_t sampled
fallbackUsed=false
nativeLoweringSucceeded=true
compositionSpecSummary expected counts
rows or styledCells matching oracle
no unsupportedNativeEffect diagnostics
focused regression test name
```

For style primitives, include foreground/background class counts when the effect should preserve source channels. For glyph-changing primitives, include row or letter-cell evidence. For masks, include visible-cell counts and phase-specific behavior.

## Anti-patterns

```text
Anti-pattern: schema-as-renderer
  Adding contract fields that only exist because one backend helper wants them.

Anti-pattern: UI-as-renderer
  Making player UI show a corrected effect that CLI evidence cannot reproduce.

Anti-pattern: silent native success
  Reporting nativeLoweringSucceeded=true after dropping authored inputs.

Anti-pattern: adapter sprawl
  Adding one-off functions with duplicated math after the operation family is clear.

Anti-pattern: compositor takeover
  Teaching the compositor to understand v3.1 recipe documents instead of giving
  it clean render primitives.
```

## Review checklist

Before committing a boundary-affecting change, verify:

- The recipe/schema/descriptor vocabulary is durable and human-readable.
- The player remains the single sampled recipe model.
- The backend lowerer either maps every used semantic field or rejects native lowering honestly.
- The compositor receives generic render operations or narrow adapter-owned stages, not full recipe documents.
- CLI, player UI, and studio UI consume the same render evidence.
- Tests prove the selected boundary with strict native mode and no fallback.

<!-- <FILE>docs/new_kernel/V31_RENDERING_BOUNDARY_RULES.md</FILE> - <DESC>Formal v3.1 recipe-to-playback boundary, lowering ownership, and compositor adapter decision rules</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
