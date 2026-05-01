<!-- <FILE>docs/arch/v31-schema-boundary-north-star.md</FILE> - <DESC>North-star architecture for schema-owned crate boundaries, data models, and primitive workflow responsibilities</DESC> -->
<!-- <VERS>VERSION: 0.7.0</VERS> -->
<!-- <WCTX>Top-down v3.1 architecture discussion: validate recipes at load time, then pass canonical v3.1 structures through to compositor-next.</WCTX> -->
<!-- <CLOG>0.7.0: MINOR — record typed transitions as a schema-audit candidate after vertical migration evidence.
0.6.0: MINOR — state the pure load-time validation and direct v3.1 pass-through compositor-next target; reject bridge/shim growth.
0.5.0: MINOR — distinguish presentation cadence, semantic update cadence, and absolute sample time in boundary model.
0.4.0: MINOR — document source.indexedField as zero-schema from-scratch primitive validation case.
0.3.0: MINOR — add descriptor/schema hindsight audit principle.
0.2.0: MINOR — add co-located primitive source tree and commonality extraction principles.
0.1.0: INIT — document schema-boundary north star, block diagram, and crate responsibility model.</CLOG> -->

# v3.1 Schema-Boundary North Star

## Status

Draft north-star architecture.

This document is intentionally aspirational. It describes the direction we want the v3.1 system to move toward as we refine runtime, migration, validation, and primitive authoring workflows.

## Core Principle

The boundary contract between crates should be one of:

1. the canonical v3.1 schema;
2. a load-validated canonical v3.1 in-memory model;
3. a declared subset/projection of the canonical v3.1 schema;
4. schema plus explicit runtime sample context such as `phaseT`, `loopT`, absolute elapsed time, and runtime presentation cadence;
5. a deliberately documented derived evidence/report contract.

Crates should not depend on arbitrary internal structs from other crates unless those structs are themselves the named boundary contract.

The target compositor-next path is **load-time validation followed by direct v3.1 execution**: validate a `RecipeDocument` when it is loaded, normalize it into the canonical v3.1 in-memory structure, and then pass that structure plus sample context to `tui-vfx-compositor-next`. The v3.1 pathway does not support legacy inputs and must not grow bridge, shim, or compositor-shaped translation layers.

In short:

```text
Schema-owned contracts at the boundaries.
Implementation details behind the boundaries.
Generated scaffolding where the contract is repetitive.
Human-owned semantics where behavior matters.
```

## Why This Matters

The v3.1 effort becomes easier if the system has disciplined boundaries.

Without disciplined boundaries:

- recipe migration becomes ad hoc;
- player and compositor concepts blur together;
- translation code becomes a pile of one-off field mappings;
- validation can accidentally prove smoke rendering instead of parity;
- future primitives repeat the same boilerplate and mistakes.

With disciplined boundaries:

- the schema describes what is allowed to exist;
- descriptor packs describe primitive/source/control contracts;
- generated tooling handles repetitive plumbing;
- the player consumes canonical recipes and emits evidence;
- compositor/runtime crates own actual visual behavior;
- migration tools adapt legacy evidence into canonical v3.1 shapes;
- validation tools compare named evidence contracts rather than guessing intent.

## Top-Down System Diagram

```text
┌──────────────────────────────────────────────┐
│              v3.1 Contract Layer             │
│                                              │
│  RecipeDocument schema                       │
│  descriptor packs                            │
│  primitive input contracts                   │
│  source contracts                            │
│  scene/lifecycle/value-source contracts      │
│  report/evidence schema definitions          │
└───────────────────────┬──────────────────────┘
                        │
                        │ schema / descriptor projections
                        ▼
┌──────────────────────────────────────────────┐
│          Generated Contract Surfaces         │
│                                              │
│  typed Rust DTOs                             │
│  input accessors                             │
│  validation helpers                          │
│  unsupported-field diagnostics               │
│  control catalog metadata                    │
│  migration mapping skeletons                 │
│  validation manifests                        │
└───────┬───────────────────┬──────────────────┘
        │                   │
        │                   │
        ▼                   ▼
┌──────────────────┐   ┌───────────────────────┐
│ Player / Runtime │   │ Migration / Tooling    │
│                  │   │                       │
│ validates recipe │   │ consumes source        │
│ at load time     │   │ recipes + descriptors  │
│ samples context  │   │ emits canonical recipe │
│ emits evidence   │   │ candidates + reports   │
└────────┬─────────┘   └───────────┬───────────┘
         │                         │
         │ canonical v3.1 model +  │ canonical recipe candidates
         │ sample context          │ validation reports
         ▼                         ▼
┌──────────────────────────────────────────────┐
│      Compositor-Next Boundary Target         │
│                                              │
│  Pass through load-validated v3.1 structures │
│  plus explicit runtime sample context.       │
│  No legacy input, bridge, or shim layer      │
│  is part of this pathway.                    │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│       Compositor / Runtime Implementation    │
│                                              │
│  final accepted runtime primitives reside    │
│  here: filters, masks, samplers, shaders,    │
│  style effects, scene composition, timing,   │
│  frame rendering, and visual behavior        │
└───────────────────────┬──────────────────────┘
                        │
                        │ rendered output + diagnostics
                        ▼
┌──────────────────────────────────────────────┐
│                Evidence Layer                │
│                                              │
│  rows                                        │
│  styled cells                                │
│  backend letter-cell evidence                │
│  styled-cell glyph evidence                  │
│  color-channel class evidence                │
│  visual frame reports                        │
│  fixture QC reports                          │
│  primitive field coverage reports            │
│  parity reports                              │
└──────────────────────────────────────────────┘
```

## Boundary Contract Examples

| Boundary | Preferred contract |
| --- | --- |
| Recipe corpus → player | Canonical v3.1 `RecipeDocument` |
| Descriptor pack → tools/workbench | Descriptor schema and descriptor pack JSON |
| Player load/runtime → compositor-next | Load-validated canonical v3.1 `RecipeDocument` / in-memory model plus explicit sample context |
| Compositor/runtime → validation | Backend output, visual frame report, styled-cell evidence, diagnostics |
| Legacy recipe corpus → migration tooling | Source recipe evidence plus explicit mapping report |
| Migration tooling → recipe corpus | Canonical recipe candidate plus migration/evidence report |
| Descriptor pack → studio/control surface | Control catalog report derived from descriptor fields |

## Data Model Roles

### Canonical v3.1 RecipeDocument

Owns authored recipe structure:

- lifecycle;
- sources;
- scenes;
- graph nodes;
- value sources;
- bindings;
- descriptor references;
- metadata.

A canonical recipe is the input contract to the player. It is not itself a render output, compositor DTO, or parity proof.

### Descriptor Packs

Own primitive/source/control definitions:

- primitive id;
- family;
- input names;
- value kinds;
- defaults;
- ranges;
- allowed values;
- bindability;
- runtime mutability;
- semantic notes.

Descriptor packs are the main source for generated tooling and should drive scaffolded primitive support.

### Generated Primitive DTOs / Accessors

Own repetitive contract plumbing:

- typed input extraction;
- value-kind checks;
- default application;
- range normalization;
- bindable value handling;
- unsupported-field diagnostics;
- stable field names.

Generated code should reduce hand-written inconsistency, not replace human semantic decisions.

### Player Render IR / Backend Request

Own sampled player evidence and backend inputs:

- load-time validation of canonical v3.1 recipes;
- sampled phase/time/signals;
- absolute elapsed sample time for continuous procedural sources;
- runtime presentation cadence when the player/runtime needs it, kept separate from recipe semantics;
- resolved scenes/sources;
- rows and styled cells;
- graph value snapshots;
- provenance;
- diagnostics;
- backend composition mode.

This is a player-owned seam. It should remain distinct from compositor internals.

### Compositor Runtime Types

Own actual visual behavior:

- filter behavior;
- mask behavior;
- sampler behavior;
- shader behavior;
- style effect behavior;
- scene composition;
- frame rendering.

The final accepted runtime primitive lives here. The schema defines its public contract; the compositor/runtime implements its behavior.

### Evidence and Report Contracts

Own validation outputs:

- visual frame reports;
- fixture QC;
- primitive field coverage;
- direct runtime-support gap reports;
- migration mapping reports;
- parity reports.

Reports are not implementation details. They are named contracts used by humans, CI, migration tools, and future agents.

## Roles and Responsibilities

### Contract / Schema Layer

Responsible for:

- canonical vocabulary;
- schema shape;
- descriptor field definitions;
- value kinds;
- allowed values;
- source and primitive contracts;
- generated JSON schemas.

Not responsible for:

- visual behavior implementation;
- legacy compatibility aliases without semantic approval;
- compositor internals.

### Primitive Workbench / Codegen Layer

Responsible for:

- reading schemas and descriptor packs;
- generating typed DTOs and accessors;
- generating validation helpers;
- generating migration skeletons;
- generating fixture templates;
- generating validation manifests;
- generating docs/control metadata stubs.

Not responsible for:

- inventing primitive semantics;
- silently approximating legacy behavior;
- hiding unsupported decisions.

### Player Layer

Responsible for:

- consuming canonical recipes;
- sampling lifecycle and value sources;
- resolving authored loopback and host signals;
- rendering player evidence;
- producing render IR and visual frame reports;
- keeping player-owned evidence independent of compositor internals.

Not responsible for:

- owning final compositor behavior;
- mutating canonical recipes during playback;
- claiming visual parity without oracle/backend comparison evidence.

### Player → Compositor-Next Boundary

Target status: direct v3.1 path.

Responsible for:

- accepting only load-validated canonical v3.1 recipe structures;
- passing explicit sample context with the loaded structure;
- producing structured diagnostics for unsupported v3.1 primitives or fields;
- preserving boundary honesty without legacy fallback.

Not responsible for:

- accepting legacy or compositor-shaped inputs;
- adding bridge/shim code to make old pathways look native;
- silently falling back in strict-native mode;
- reimplementing compositor-owned effects outside compositor-next;
- hiding unsupported fields by dropping semantics.

If the existing player/runtime path cannot satisfy this without expanding old translation code, create a stripped `player-next` path that removes non-v3.1 compatibility concerns rather than carrying them forward.

### Compositor / Runtime Layer

Responsible for:

- final runtime primitive implementation;
- filters, masks, samplers, shaders, style effects, and scene composition;
- rendering output from explicit backend/composition requests;
- preserving behavior expected by accepted primitive contracts.

Not responsible for:

- parsing arbitrary legacy recipe shapes;
- owning migration policy;
- depending on player UI internals.

### Migration Tooling Layer

Responsible for:

- reading source recipes as evidence;
- mapping source recipe intent into canonical v3.1 recipes;
- recording exact field mappings and decisions;
- producing candidate recipe patches or reports;
- classifying blockers as descriptor, runtime, schema, tooling, or owner-decision work.

Not responsible for:

- mutating source recipes;
- marking smoke-rendered recipes as parity-passed;
- adding aliases to strict v3.1 solely to make old JSON validate.

### Validation Layer

Responsible for:

- structural validation;
- field coverage;
- direct runtime support coverage;
- strict compositor-next evidence;
- oracle comparison where applicable;
- fixture QC;
- explicit PASS/BLOCKED/FAIL evidence.

Not responsible for:

- inferring visual parity from smoke output alone;
- hiding missing implementation work;
- treating unsupported runtime paths as success.

## Co-Located Primitive Source Trees

The preferred long-term organization is a co-located primitive source tree. Since the schema exists to define and protect primitive/source/recipe contracts, primitive-local schema and descriptor details should live next to generated assets, migration mappings, fixtures, tests, docs, and runtime code.

Representative shape:

```text
primitives/
  shader/
    highlighter/
      descriptor.v31.json
      migration.v2.json
      primitive.toml
      generated/
        highlighter_inputs.rs
        highlighter_accessors.rs
        highlighter_validation_manifest.json
        highlighter_control_catalog.json
      runtime/
        mod.rs
        highlighter_shader_runtime.rs
      fixtures/
        minimal.v31.json
        v2_parity.v31.json
        unsupported_text_contrast.v31.json
      tests/
        structural_validation.toml
        field_coverage.toml
        native_backend.toml
        parity_samples.toml
      docs/
        highlighter.md
```

This structure is intended to make a primitive's full lifecycle readable as one unit: contract, generated surfaces, human-owned behavior, migration rules, validation evidence, and documentation.

## Primitive Addition North-Star Workflow

Adding a new primitive should follow this path:

```text
1. Define descriptor/schema details
        │
        ▼
2. Run Primitive Workbench scaffold
        │
        ├── generated typed inputs
        ├── generated accessors
        ├── generated diagnostics
        ├── generated fixture template
        ├── generated migration skeleton
        └── generated validation manifest
        │
        ▼
3. Fill compositor/runtime behavior
        │
        ├── implement new behavior, or
        └── connect to existing compositor behavior
        │
        ▼
4. Fill semantic migration decisions
        │
        ├── direct field mappings
        ├── enum normalization
        ├── phase/source/scene transforms
        └── explicit unsupported decisions
        │
        ▼
5. Run validation gates
        │
        ├── structural validation
        ├── field coverage
        ├── native/backend support
        ├── fixture QC
        └── parity/oracle checks where applicable
        │
        ▼
6. Accept runtime primitive
        │
        ▼
7. Use from canonical recipes, studio controls, migration tools, and tests
```

## Descriptor/Schema Hindsight Audit

Before generating large amounts of primitive scaffolding, run a bounded descriptor/schema hindsight audit. The audit should identify common primitive fields and semantic concepts that were missed or duplicated during earlier schema build-out.

The purpose is schema hardening, not unbounded redesign. Accepted common concepts should feed generated helpers, descriptor fragments, migration mapping tables, and validation manifests. Rejected collapses should be documented so future agents do not rediscover the same ambiguity.

Examples of concepts worth auditing include progress, apply-to routing, color channels, direction/axis/edge geometry, seed, density, speed/frequency, presentation cadence, semantic update cadence, absolute elapsed sample time, radius/falloff/feather, threshold, intensity, glyph sets, and bindability.

Typed transitions are a named audit candidate. Classic transitions such as crossfade, wipe, iris, push, dissolve, morph, stippled, and braille may deserve first-class recipe/schema representation because they describe binary temporal A→B intent, not just a generic unary cell effect. The audit should preserve primitive-level implementations while evaluating whether author-facing typed transitions improve validation, theming, optimization, documentation, AI generation, and schema diet opportunities.

## Timing Boundary Rule

Do not use `fps` as a catch-all schema concept. The boundary model should distinguish:

- **presentation cadence**: how often a player/runtime tries to draw frames;
- **semantic update cadence**: how often a recipe/source/effect recomputes state when fixed-step behavior is desired;
- **sample time**: the actual `phaseT`, optional `loopT`, and absolute elapsed time used for one deterministic sample.

The current Madeira flag v3.1 recipes demonstrate why this matters. The procedural flag wave and fireworks need absolute elapsed time so motion can advance even when normalized `phaseT`/`loopT` remain fixed, and authored preview loopback ramps use elapsed time to honor durations. A compositor-next/workbench timing model must preserve that seam instead of baking wall-time behavior into individual primitives ad hoc.

## Commonality Extraction Rule

Primitive work should also clean up repeated implementation patterns. If the same or strongly similar primitive-internal behavior appears in 3 or more places, it should be abstracted into a shared helper, trait, utility, generated helper, common data model, or explicit extraction ticket.

Common candidates include color blending, apply-to routing, gradient sampling, bindable progress resolution, time normalization, direction/axis/edge enum normalization, region selection, falloff math, seeded noise, glyph ramps, partial-block encoding, shader blend policies, unsupported-field diagnostics, and migration rename tables.

The goal is not abstraction for its own sake. The goal is to prevent primitive implementations from drifting when they are doing the same conceptual work.

## Design Discipline

A change should be questioned if it violates any of these rules:

1. A crate boundary uses an unnamed internal shape instead of a documented contract.
2. A primitive field exists in a recipe but not in the descriptor.
3. A descriptor field exists but no generated/accessor/runtime layer acknowledges it.
4. A backend drops a field without an explicit unsupported diagnostic.
5. A migration rule adds a compatibility alias instead of mapping to canonical vocabulary.
6. A validation report marks a recipe complete without vertical parity evidence.
7. Player-side code reimplements compositor-owned behavior instead of routing to the runtime primitive.
8. Generated code overwrites human-owned semantic decisions.
9. A pattern appears in 3 or more primitive implementations without being shared or explicitly ticketed.
10. Primitive contract, generated outputs, migration rules, fixtures, and docs are scattered so the primitive lifecycle cannot be read as one unit.

## Relationship to Current Compositor-First Work

The old compositor-first pathway remains useful as reference behavior and parity evidence, but it is not the architecture for new v3.1 work. New v3.1 compositor-next work should not add support to the current translation path.

Target direction: create a v3.1-native compositor-next boundary. The recipe is validated when loaded; after that, the canonical v3.1 structure should pass through with explicit sample context to compositor-next-owned v3.1 runtime boundaries. If the current player/runtime path makes that awkward, copy and strip a `player-next` path rather than expanding translation code.

The v3.1 path must obey this north-star rule:

```text
The boundary is named.
The recipe is validated at load time.
The loaded canonical v3.1 structure passes through.
No bridge, shim, or legacy-input support is added.
Unsupported semantics fail loudly.
Validation proves the path.
```

## From-Scratch Primitive Validation Case

The north-star workflow should be tested not only by migrating existing primitives, but also by adding one new primitive from scratch. The planned validation case is `source.indexedField`, specified in [`../design/post-release/indexed-palette-cycling-spec.md`](../design/post-release/indexed-palette-cycling-spec.md).

`source.indexedField` is intentionally classified as a source descriptor rather than an effect descriptor. It produces cells from pattern, palette, and rotation, and can consume shared palette assets through existing source asset slots. This validates the principle that schema changes are unnecessary when existing descriptor/source seams can absorb the need.

## Open Questions

1. Which crate should own the Primitive Workbench binary?
2. Should generated Rust be checked in or generated at build time?
3. What files are generated versus hand-owned for a primitive?
4. How do generated files preserve hand-written semantic sections?
5. What exact v3.1-native compositor-next entrypoint should replace compositor-shaped inputs?
6. What is the canonical migration mapping table schema?
7. How should CI detect descriptor/runtime drift?
8. How should parity tooling distinguish schema support, player support, backend support, and visual parity in one report?

## North-Star Summary

```text
The schema defines the contract.
Descriptors define primitive capabilities.
Generated tooling derives repetitive surfaces.
The player samples canonical recipes into evidence.
The player passes only load-validated canonical v3.1 contracts to compositor-next.
The compositor/runtime owns final primitive behavior.
Validation proves every boundary.
```

<!-- <FILE>docs/arch/v31-schema-boundary-north-star.md</FILE> - <DESC>North-star architecture for schema-owned crate boundaries, data models, and primitive workflow responsibilities</DESC> -->
<!-- <VERS>END OF VERSION: 0.7.0</VERS> -->
