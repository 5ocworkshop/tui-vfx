<!-- <FILE>docs/arch/primitive-workbench-schema-driven-workflow.md</FILE> - <DESC>Draft architecture for schema-driven primitive scaffolding, migration, and validation workflow</DESC> -->
<!-- <VERS>VERSION: 0.8.0</VERS> -->
<!-- <WCTX>Top-down v3.1 architecture discussion: generate pure v3.1 primitive work packets and reject legacy lowering surfaces.</WCTX> -->
<!-- <CLOG>0.8.0: MINOR — retarget workbench guidance to tui-vfx-compost and distinguish pending commonality review from the completed naming audit.</CLOG> -->

# Primitive Workbench: Schema-Driven Primitive Workflow

**HARD DIRECTIVE — PURE v3.1 END TO END:** The active runtime target is `tui-vfx-compost`, a pure v3.1 system from recipe load through primitive execution. Work packets must migrate proven behavior into the compost file tree while lightly updating implementations to consume canonical v3.1 schema fields directly. Any attempt to adapt v3.1 into `CompositionSpec`, `ShaderLayerSpec`, `SpatialShaderType`, legacy-shaped field names, bridge/shim DTOs, or transitional lowering layers is a failure. Halt immediately if a slice starts adding that kind of adapter. The same work is the opportunity to split large legacy files into OFPF-compliant, professionally named, size-guideline-respecting modules.


## Status

Draft for discussion.

## Purpose

This document proposes a reusable workflow for adding, migrating, scaffolding, and validating v3.1 primitives from the schema and descriptor layer downward.

The core idea is that v3.1 descriptors and schemas should be the source of truth. Runtime support, migration helpers, validation manifests, studio controls, and documentation should be derived from that contract wherever possible, instead of being hand-mapped independently in multiple places.

Validation should happen when a canonical v3.1 recipe is loaded. After that load-time validation step, runtime code should execute from the canonical v3.1 structure and explicit sample context directly inside compost-owned primitive boundaries. Workbench-generated surfaces should make that direct path safe and typed; they must not add bridge, shim, legacy-input support, or lowering into `CompositionSpec`/`ShaderLayerSpec`/`SpatialShaderType` to the v3.1 pathway.

## Vocabulary

Use the project vocabulary from `docs/VOCABULARY.md`:

- **Primitive**: umbrella concept for a runtime-capable recipe operation.
- **Primitive family**: filter, mask, sampler, style effect, shader, lifecycle trigger, role scope, and related descriptor families.
- **Effect descriptor**: schema/descriptor definition for an executable primitive effect.
- **Graph node**: recipe instance of an effect descriptor.
- **Canonical Recipe**: strict v3.1 `RecipeDocument`.
- **Direct v3.1 compost execution**: player/runtime passes the load-validated canonical v3.1 structure plus sample context into compost-owned runtime entrypoints.

## Architectural Goal

Adding a primitive should start with the canonical v3.1 contract, not with ad hoc runtime code.

The schema exists to describe primitives, sources, scenes, lifecycle, and recipe execution contracts. For primitive work specifically, the descriptor/schema details should live near the primitive-owned generated assets and hand-owned implementation notes so humans and AI agents can read the complete primitive story in one place.

Desired rule:

```text
No new primitive support is hand-added directly.
Every primitive starts with a descriptor/schema update,
then generated scaffolding,
then minimal human semantic implementation,
then generated validation gates.
```

## Block Diagram

```text
┌──────────────────────────────────────────────┐
│  1. Primitive Schema / Descriptor Definition │
│                                              │
│  effect id: filter.vignette                  │
│  family: filter                              │
│  inputs: strength, radius, sides, ...        │
│  value kinds, defaults, ranges, bindability  │
│  runtime mutability, semantic notes          │
└───────────────────────┬──────────────────────┘
                        │
                        │ schema/descriptor is source of truth
                        ▼
┌──────────────────────────────────────────────┐
│  2. Primitive Tooling / Codegen              │
│                                              │
│  Reads descriptor packs + schemas            │
│  Applies naming / validation conventions     │
│  Emits generated artifacts                   │
└───────┬───────────────┬───────────────┬──────┘
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Rust Input   │ │ Migration    │ │ Test /       │
│ Structs +    │ │ Mapping      │ │ Validation   │
│ Accessors    │ │ Skeletons    │ │ Manifests    │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       ▼                ▼                ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Runtime      │ │ V2 Evidence  │ │ Fixture QC   │
│ Skeleton     │ │ Migration    │ │ Field        │
│ / Direct     │ │ Helper       │ │ Coverage     │
│ Entry Stub   │ │              │ │ Parity Tests │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       ▼                ▼                ▼
┌──────────────────────────────────────────────┐
│  3. Human-Owned Implementation Layer         │
│                                              │
│  Fill in real primitive behavior if missing  │
│  Wire existing compositor behavior if exists │
│  Resolve semantic decisions                  │
│  Add hand-written edge-case tests            │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│  4. Generated + Hand-Written Validation Gate │
│                                              │
│  structural validation                       │
│  field coverage                              │
│  direct runtime support                      │
│  strict tui-vfx-compost evidence             │
│  V2 oracle parity where applicable           │
│  generated docs/control catalog              │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│  5. Final Accepted Runtime Primitive         │
│                                              │
│  resides in compositor/runtime implementation│
│  exposed through canonical v3.1 descriptors  │
│  usable by canonical v3.1 recipes            │
│  generated studio controls available         │
│  migration + parity tooling available        │
└──────────────────────────────────────────────┘
```

## Where the Primitive Lives

The final accepted primitive is at the bottom of the graph. It resides in the compositor/runtime implementation layer, not in the schema itself.

The schema and descriptor at the top define the canonical v3.1 boundary for the primitive: its id, family, fields, value kinds, defaults, bindability, and validation rules. The workbench derives scaffolding from that boundary. Human implementation then connects those fields to real compositor/runtime behavior. After validation, the result is a runtime-capable primitive in the compositor that is exposed to recipe authors through the v3.1 descriptor contract.

In short:

```text
Top:    v3.1 descriptor says what the primitive means.
Middle: tooling and humans build the implementation path.
Bottom: compositor/runtime owns the final working primitive.
```

## Workflow

```text
Author primitive descriptor
        │
        ▼
Run primitive scaffold/codegen tool
        │
        ├── emits Rust typed input model
        ├── emits accessor/extraction helpers
        ├── emits unsupported-field diagnostics
        ├── emits default/range/bindability checks
        ├── emits migration mapping skeleton
        ├── emits fixture template
        ├── emits validation manifest
        ├── emits control-catalog/studio metadata
        └── emits docs stub
        │
        ▼
Developer fills only semantic gaps
        │
        ├── actual compositor/runtime behavior, if new
        ├── mapping to existing compositor function, if reused
        ├── custom structured transforms
        └── explicit unsupported decisions
        │
        ▼
Run generated validation suite
        │
        ├── structural validation
        ├── field coverage
        ├── strict-native support
        ├── fixture QC
        ├── migration oracle comparison
        └── docs/catalog checks
        │
        ▼
Primitive accepted in compositor/runtime layer
```

## Pre-Generation Workbench Commonality Review

Before the workbench generates broad primitive scaffolding, run a descriptor/workbench commonality review. This pending review is separate from the completed ambiguous field-name audit. It should detect repeated primitive concepts and decide which ones become shared workbench concepts, generated helper families, descriptor fragments, or migration-only mapping notes.

The review should classify common fields before generation so the workbench does not encode accidental duplication into every primitive. It is not a reason to reopen core schema design before substrate work unless a concrete primitive/example proves a contract defect.

## Preferred Co-Located Primitive Source Tree

Prefer a co-located source tree per primitive. The point is to keep the schema, generated assets, runtime module, fixtures, tests, migration mapping, and docs together so the primitive reads as one coherent unit.

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

The exact root path can change, but the locality principle should remain: a primitive's contract, generated surfaces, migration rules, validation manifests, and human notes should be discoverable together.

This does not mean every generated artifact must be committed in final form. The project can still decide whether generated Rust and reports are checked in, generated at build time, or produced by CI. The source-of-truth inputs and generated-output ownership should still be visible from the primitive tree.

## What Tooling Should Generate

The proposed tool, tentatively called **Primitive Workbench**, should read descriptor packs and v3.1 contract schemas, then generate boring and error-prone plumbing.

Timing-sensitive primitives and sources need a deliberately named sample contract. The workbench should not collapse everything into `fps`. It should treat runtime presentation cadence, semantic update cadence, and sample time as separate concepts. The Madeira flag fixtures are the first concrete reference case: their procedural wave/fireworks sources and authored preview loopback ramps use absolute elapsed sample time, not only normalized `phaseT`/`loopT`.

Candidate generated outputs:

1. **Typed Rust input models**
   - One input struct per primitive descriptor.
   - Field names match canonical v3.1 vocabulary.
   - Defaults, ranges, value kinds, and bindability are represented explicitly.

2. **Input accessors and extraction helpers**
   - Standard number, integer, boolean, color, enum, gradient, duration, signal, parameter, graph value, and structured-value handling.
   - Consistent fallback behavior.
   - Consistent diagnostics for kind mismatch and unsupported fields.

3. **Pass-through/runtime skeletons**
   - Generated match arms or registration entries.
   - Generated unsupported-field guards for load-time and strict runtime checks.
   - Generated field extraction boilerplate for the loaded canonical v3.1 model.
   - Generated direct tui-vfx-compost entrypoint stubs for canonical v3.1 inputs.
   - Human-filled semantic body where behavior decisions are required.

4. **Migration mapping skeletons**
   - Legacy field rename tables.
   - Legacy enum normalization tables.
   - Stage-to-graph-node transforms.
   - Phase gating transforms.
   - Binding transforms.
   - Explicit owner-decision placeholders for non-mechanical semantics.

5. **Fixture templates**
   - Canonical recipe skeleton for the primitive.
   - Minimal smoke fixture.
   - Optional V2 parity fixture skeleton when a source recipe exists.

6. **Validation manifests**
   - Structural validation targets.
   - Field coverage targets.
   - Direct runtime-support coverage targets.
   - Strict tui-vfx-compost targets.
   - Oracle comparison sample points where applicable.

7. **Documentation and studio metadata**
   - Primitive docs stub.
   - Control catalog metadata.
   - Studio control defaults derived from descriptor fields.

## What Tooling Should Not Generate

The workbench should not pretend to own visual semantics.

Human-owned work remains:

- actual visual/effect behavior;
- semantic mapping where old fields differ from canonical v3.1 concepts during migration;
- acceptance/rejection of compatibility aliases;
- compositor behavior changes;
- parity approval;
- nuanced edge-case tests.

The generator should make the correct path easy, but it should not hide decisions behind generated code. The correct target path is load-time validation followed by typed direct v3.1 pass-through to tui-vfx-compost, not bridge/shim code that reinterprets recipe semantics at render time.

## Commonality Extraction Discipline

Primitive workbench should not only scaffold per-primitive code. It should also reveal repeated implementation patterns and move them into shared runtime utilities, generated helpers, common data models, or explicit extraction tickets.

Rule of thumb:

```text
If the same primitive-internal pattern appears in 3 or more places,
promote it to a shared helper, trait, utility, generated helper, or common data model.
```

Common candidates include:

- color resolution and color blending;
- foreground/background/apply-to routing;
- gradient sampling;
- progress and bindable-value resolution;
- phase/time normalization;
- direction, axis, side, edge, and corner enum normalization;
- region, row, column, and scope selection;
- falloff, radius, feather, and distance math;
- seeded noise/random sampling;
- glyph ramp and glyph index sampling;
- partial-block and subcell encoding;
- shader layering and blend policies;
- unsupported-field diagnostics;
- descriptor value-kind extraction;
- migration field rename and enum normalization.

Before accepting a primitive, the implementation should be compared against nearby primitives. If a repeated pattern crosses the threshold, extract it immediately when small and safe, or record an explicit extraction ticket when extraction would widen the change too much.

## Why This Helps

This workflow would:

- protect the schema as the source of truth;
- keep primitive contract, generated outputs, fixtures, tests, and docs co-located for human and AI readability;
- reduce repeated hand-written mapping code;
- make primitive support consistent across player, tui-vfx-compost, recipes, tools, studio, and docs;
- make future primitive additions predictable;
- turn recipe migration into a table-driven process where possible;
- prevent silent drift between descriptor fields and runtime-supported fields;
- make unsupported semantics explicit and testable;
- surface common implementation patterns early and move them into shared helpers before primitive code diverges.

## Mapping Is a Translation Matrix, Not Blind Find/Replace

Some mappings are mechanical:

```text
scanline_strength -> scanlineStrength
left_to_right     -> leftToRight
pipeline.filter.dwell -> graph node with activePhases: ["dwell"]
```

Other mappings require semantic decisions:

```text
legacy field has no canonical v3.1 equivalent
legacy default differs from descriptor default
legacy effect mutates source content before compositor execution
legacy binding shape does not match canonical signal/value-source shape
legacy recipe encodes visual intent that belongs in scene/source metadata
```

The workbench should distinguish these cases rather than flattening everything into permissive aliases.

## First From-Scratch Workflow Test

After the workbench has proven itself on existing primitives, use `source.indexedField` from [`../design/post-release/indexed-palette-cycling-spec.md`](../design/post-release/indexed-palette-cycling-spec.md) as the first from-scratch primitive test.

Important classification:

```text
source.indexedField is a source descriptor, not an effect descriptor.
```

It produces cells from pattern, palette, and rotation instead of transforming existing cells. This lets it use the existing source asset seam for shared palettes:

```text
AssetKind::Custom { name: "palette" }
format: "tui-vfx.palette.v1"
```

The spec is expected to require zero v3.1 schema changes. It should prove that the workbench can start from descriptor details, generate source scaffolding, implement runtime behavior, validate fixtures, and sign off a new primitive without broad contract churn.

## Possible Command Surface

Example command names for discussion:

```bash
primitive-workbench add filter.vignette
primitive-workbench scaffold filter.vignette
primitive-workbench migrate-v2 filter.vignette --source recipes/debug_recipes/filters/_DEPRECATED_filter_vignette.json
primitive-workbench validate filter.vignette
primitive-workbench report filter.vignette
```

Alternative names can be chosen later. The important concept is that primitive authoring and migration become an explicit system workflow.

## Open Design Questions

1. Should generated Rust live in checked-in generated files, macro-expanded code, or build-time generated artifacts?
2. What exact v3.1-native tui-vfx-compost entrypoint shape should generated runtime skeletons target?
3. How should generated code preserve hand-written semantic sections without being overwritten?
4. What is the exact schema for migration mapping tables?
5. Should the workbench produce recipe patches directly, or only reports and candidate patches for review?
6. How should the validation suite distinguish structural support, backend support, and visual parity?
7. What naming convention should separate generated files from hand-owned implementation files?
8. How do we prevent descriptor changes from silently breaking generated runtime behavior?

## Proposed Principle

The durable principle is:

```text
The v3.1 schema describes what a primitive is.
Recipes are validated at load time.
The Primitive Workbench derives the boring implementation surfaces.
Runtime passes the loaded canonical v3.1 structure and sample context directly to tui-vfx-compost.
Humans fill in behavior and semantic decisions in the compositor/runtime layer.
Validation proves the generated and human-owned layers agree.
```

<!-- <FILE>docs/arch/primitive-workbench-schema-driven-workflow.md</FILE> - <DESC>Draft architecture for schema-driven primitive scaffolding, migration, and validation workflow</DESC> -->
<!-- <VERS>END OF VERSION: 0.8.0</VERS> -->
