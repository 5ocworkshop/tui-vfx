<!-- <FILE>docs/arch/primitive-workbench-schema-driven-workflow.md</FILE> - <DESC>Companion workflow for schema-constrained primitive scaffolding, migration evidence, validation, and workbench tooling under the accepted Rust-owned v3.1 primitive catalog instance direction</DESC> -->
<!-- <VERS>VERSION: 0.10.0</VERS> -->
<!-- <WCTX>Reconcile the workbench proposal with the accepted Rust-SSOT primitive catalog instance direction: contract schemas remain the shape authority, Rust declarations in tui-vfx-compost own accepted primitive catalog facts, primitive.json is generated, and the workbench becomes scaffolding/migration/validation tooling rather than a competing JSON-first SSOT.</WCTX> -->
<!-- <CLOG>0.10.0: MAJOR — replace peer-review side-by-side SSOT decision language with the reconciled companion workflow; add two-layer SSOT taxonomy, bootstrap-aware Rust flow, domain-runtime implications, and updated generation/validation responsibilities.
0.9.0: MINOR — add the Companion Direction section with an inverted block diagram for the Rust-SSOT proposal under peer review.</CLOG> -->

# Primitive Workbench: Schema-Constrained Primitive Workflow

**HARD DIRECTIVE — PURE v3.1 END TO END:** The active runtime target is `tui-vfx-compost`, a pure v3.1 system from recipe load through primitive execution. Work packets must migrate proven behavior into the compost file tree while updating implementations to consume canonical v3.1 schema fields directly. Any attempt to adapt v3.1 into `CompositionSpec`, `ShaderLayerSpec`, `SpatialShaderType`, legacy-shaped field names, bridge/shim DTOs, or transitional lowering layers is a failure. Halt immediately if a slice starts adding that kind of adapter. The same work is the opportunity to split large legacy files into OFPF-compliant, professionally named, size-guideline-respecting modules.

## Status

Accepted as a **companion workflow** to `docs/arch/v31-primitive-rust-ssot.md` and `docs/arch/v31-primitive-rust-ssot-implementation-plan.md`.

This document no longer presents schema-first descriptor JSON and Rust-first declarations as competing SSOT directions. Peer review selected the Rust-owned primitive catalog instance path:

- `tui-vfx-contract` Rust DTOs own v3.1 **contract schema shapes**.
- `tui-vfx-compost` Rust primitive/source declarations own accepted **primitive catalog instance facts**.
- `descriptors/v3.1/packs/primitive.json` is a generated external artifact.
- `descriptors/v3.1/packs/primitive.bootstrap.json` temporarily carries unported descriptors during migration.

The Primitive Workbench remains valuable: it should scaffold Rust declarations, produce migration/evidence reports, generate fixture and validation templates, and run consistency checks. It should not be the long-term edit point for accepted primitive descriptor facts.

## Purpose

Primitive work has three hard problems:

1. **Shape correctness.** Inputs, outputs, value-source forms, descriptor metadata, and recipe structures must match the v3.1 contract schemas.
2. **Semantic migration.** Legacy implementations and recipes contain useful behavior, but their field names and runtime assumptions are not v3.1 truth.
3. **Drift prevention.** Descriptor entries, runtime code, aliases, fixtures, docs, and studio controls must not evolve independently.

The workbench exists to make those problems mechanical where possible and explicit where not possible. It should gather evidence from schemas, descriptors, aliases, legacy source, current compost seams, and corpus fixtures, then produce typed Rust scaffolds and validation artifacts that make the accepted Rust-SSOT path fast and safe.

## Vocabulary

Use the project vocabulary from `docs/VOCABULARY.md`:

- **Primitive**: umbrella concept for a runtime-capable recipe operation.
- **Primitive family**: filter, mask, sampler, style effect, shader, lifecycle trigger, role scope, source, and related descriptor families.
- **Effect descriptor**: contract-shaped descriptor definition for an executable primitive effect.
- **Source descriptor**: contract-shaped descriptor for content/materialization inputs such as `source.card` or `source.procedural`.
- **Graph node**: recipe instance of an effect descriptor.
- **Canonical Recipe**: strict v3.1 `RecipeDocument`.
- **Direct v3.1 compost execution**: load-validated canonical v3.1 structures plus sample context flow directly into compost-owned runtime entrypoints.

## Two-layer SSOT taxonomy

| Layer | Owner | Generated artifact | Workbench role |
| --- | --- | --- | --- |
| **Contract schema** | Rust DTOs in `tui-vfx-contract` | `schemas/v3.1/contract/*.schema.json` | Read schemas to validate/generated scaffolds; optionally help move schema generation behind `cargo xtask schemas gen/check` later |
| **Primitive catalog instance** | Rust primitive/source declarations in `tui-vfx-compost`, plus temporary bootstrap carry-forward | `descriptors/v3.1/packs/primitive.json` | Scaffold Rust declarations, generate evidence/diff reports, remove bootstrap entries, run descriptor checks |

The schema still describes what valid primitive descriptor shapes look like. The accepted catalog instance values for a primitive land in Rust, not by hand-editing generated JSON.

## Current workbench block diagram

The workbench sits beside the Rust-SSOT loop. It ingests evidence and emits scaffolding; the registry/codegen/check loop accepts or rejects the result.

```text
┌──────────────────────────────────────────────┐
│  1. Evidence Inputs                          │
│                                              │
│  contract DTOs + JSON Schemas                │
│  current primitive.json migration seed       │
│  primitive.bootstrap.json burndown ledger    │
│  alias/canonicalize paramMappings            │
│  legacy source semantic references           │
│  corpus shorthand/canonical fixtures         │
│  current tui-vfx-compost runtime seams       │
└───────────────────────┬──────────────────────┘
                        │
                        │ workbench reads, compares, reports
                        ▼
┌──────────────────────────────────────────────┐
│  2. Primitive Workbench Tooling              │
│                                              │
│  classify descriptor/source domain           │
│  infer candidate Rust skeleton               │
│  generate evidence matrix                    │
│  flag semantic conflicts                     │
│  produce fixtures and validation manifests   │
└───────┬───────────────┬───────────────┬──────┘
        │               │               │
        ▼               ▼               ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│ Rust         │ │ Migration    │ │ Test /       │
│ Primitive    │ │ Evidence     │ │ Validation   │
│ Scaffold     │ │ Report       │ │ Templates    │
└──────┬───────┘ └──────┬───────┘ └──────┬───────┘
       │                │                │
       ▼                ▼                ▼
┌──────────────────────────────────────────────┐
│  3. Human-Owned Semantic Implementation      │
│                                              │
│  fill runtime behavior in tui-vfx-compost    │
│  choose v3.1-native field semantics          │
│  extract common helpers when repeated        │
│  document unsupported/deferred decisions     │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│  4. Registry + Codegen Acceptance Loop       │
│                                              │
│  install Rust descriptor/runtime             │
│  remove matching bootstrap entry             │
│  cargo xtask descriptors gen/check           │
│  validate generated primitive.json           │
│  run compost load/render/runtime tests       │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│  5. Accepted Runtime Primitive               │
│                                              │
│  Rust declaration is catalog instance truth   │
│  runtime dispatch is domain-specific          │
│  primitive.json is generated for tools        │
│  round-trip lock prevents drift               │
└──────────────────────────────────────────────┘
```

## Accepted runtime direction: Rust catalog instance SSOT

The accepted primitive catalog flow is:

```text
┌──────────────────────────────────────────────┐
│  1. Hand-Owned v3.1 Primitive/Source Rust    │
│                                              │
│  crates/tui-vfx-compost/src/<axis>/...       │
│  crates/tui-vfx-compost/src/source/...       │
│                                              │
│  EffectPrimitive / SourcePrimitive consts    │
│  PrimitiveInputs / PrimitiveEnum derives     │
│  domain runtime trait implementation         │
└───────────────────────┬──────────────────────┘
                        │
                        │ Rust is catalog instance source
                        ▼
┌──────────────────────────────────────────────┐
│  2. Macro Expansion / Type Checking          │
│                                              │
│  wrapper type -> descriptor kind             │
│  bindability from Literal<T>/Bindable<T>     │
│  enum labels from PrimitiveEnum              │
│  compile errors for unsupported shapes       │
└───────────────────────┬──────────────────────┘
                        │
                        ▼
┌──────────────────────────────────────────────┐
│  3. EffectRegistry / Source Registry         │
│                                              │
│  descriptor view                             │
│  bootstrap carry-forward for unported ids    │
│  runtime maps by EffectDomain                │
│  source runtime map                          │
└───────┬───────────────────────────┬──────────┘
        │                           │
        ▼                           ▼
┌──────────────────┐         ┌──────────────────┐
│ 4a. Runtime      │         │ 4b. Codegen      │
│     Path         │         │     Path         │
│                  │         │                  │
│ tui-vfx-compost  │         │ cargo xtask      │
│ validates ids    │         │ descriptors      │
│ and dispatches   │         │ gen/check        │
│ by domain map    │         │                  │
└──────────────────┘         └────────┬─────────┘
                                      │
                                      ▼
                             ┌──────────────────┐
                             │ 5. Generated     │
                             │    Artifact      │
                             │                  │
                             │ primitive.json   │
                             │ consumed by      │
                             │ external tools   │
                             └────────┬─────────┘
                                      │
                                      ▼
                             ┌──────────────────┐
                             │ 6. Round-Trip    │
                             │    Lock          │
                             │                  │
                             │ drift fix: edit  │
                             │ Rust/bootstrap,  │
                             │ rerun xtask      │
                             └──────────────────┘
```

Runtime traits are domain-specific, not one universal `EffectRuntime`: `CellShaderRuntime`, `FrameFilterRuntime`, `MaskRuntime`, `CoordinateSamplerRuntime`, `ContentTransformRuntime`, and future/descriptor-only handling for currently unused `EffectDomain` values. Source descriptors use `SourcePrimitive` / `SourceRuntime`.

## Resolved inversion from the previous draft

Previous versions framed this as a decision between JSON descriptor SSOT and Rust descriptor SSOT. The accepted resolution is:

- **Contract schema remains upstream.** The schema layer defines legal shapes and validates generated descriptor output.
- **Primitive catalog instance values move to Rust.** Capability metadata lives near the runtime trait implementation that relies on it.
- **Workbench generation direction changes.** The workbench does not generate final runtime support from hand-edited descriptor JSON; it generates Rust scaffolds and evidence to help humans land Rust-owned declarations.
- **External tools are unaffected.** They still read `descriptors/v3.1/packs/primitive.json`; they do not need to link `tui-vfx-compost`.
- **Bootstrap keeps migration safe.** A full unified `primitive.json` is emitted throughout Phase 3 while bootstrap shrinks.

## Where the primitive lives

The final accepted primitive lives in `tui-vfx-compost`, not in schema JSON and not in legacy crates. Use the existing axis directories unless a Phase 0 decision introduces a new umbrella:

```text
crates/tui-vfx-compost/src/
  filters/     # frameFilter descriptors such as filter.dim
  masks/       # mask descriptors such as mask.dissolve
  samplers/    # coordinateSampler descriptors such as sampler.gravity
  shaders/     # cellShader descriptors such as shader.linearGradient
  styles/      # style-prefixed cellShader descriptors
  content/     # contentTransform descriptors
  source/      # SourcePrimitive / SourceRuntime descriptors
  runtime/     # value-source resolution and shared runtime context
  render/      # frame/surface integration
  validation/  # load-time v3.1 validation seams
```

The workbench may generate temporary reports under a tooling output directory, but accepted source files and tests should land in the compost tree and adjacent test/fixture locations chosen by the implementation plan.

## Workbench workflow

```text
Select primitive/source id
        │
        ▼
Collect evidence
        ├── contract DTO/schema shape
        ├── current descriptor seed/bootstrap entry
        ├── alias/canonicalize mapping
        ├── legacy source semantics if present
        ├── corpus fixtures
        └── current compost helper/runtime seams
        │
        ▼
Generate scaffold + evidence report
        ├── Rust input struct with PrimitiveInputs attrs
        ├── descriptor const skeleton
        ├── correct domain runtime trait skeleton
        ├── source/runtime skeleton if source descriptor
        ├── migration mapping skeleton
        ├── fixture templates
        ├── validation manifest
        └── bootstrap-removal checklist
        │
        ▼
Human fills semantic behavior
        ├── direct v3.1 runtime behavior
        ├── v3.1-native field choices
        ├── common helper extraction
        └── unsupported/deferred decisions
        │
        ▼
Run acceptance loop
        ├── cargo check/test
        ├── cargo xtask descriptors gen/check
        ├── schema validate generated primitive.json
        ├── compost load/render tests
        └── corpus delta report
        │
        ▼
Primitive accepted; generated artifact committed
```

## Pre-generation commonality review

Before broad scaffolding, run the Phase 0.5 commonality review from the implementation plan. The workbench should identify repeated primitive concepts before generating boilerplate so the project does not encode accidental duplication into every port.

Common candidates include:

- foreground/background/glyph/modifier apply-to routing;
- color resolution, dim/tint/blend, gradient interpolation;
- progress, clock, absolute sample time, and phase normalization;
- source vs destination coordinate normalization;
- direction, side, edge, axis, corner, and anchor enum normalization;
- mask falloff/visibility and soft-edge math;
- sampler displacement output helpers;
- seeded noise/random sampling;
- glyph ramp/index sampling;
- unsupported-field and value-kind diagnostics;
- migration field rename and enum normalization.

Rule of thumb:

```text
If the same primitive-internal pattern appears in 3 or more places,
promote it to a shared helper, generated helper, utility, or explicit ticket.
```

## What tooling should generate

The workbench should generate boring, reviewable, and replaceable artifacts:

1. **Rust primitive/source scaffold**
   - `EffectPrimitive` or `SourcePrimitive` skeleton.
   - Correct domain runtime trait skeleton.
   - Typed input struct using `Literal<T>` / `Bindable<T>` wrappers.
   - `PrimitiveInputs` and `PrimitiveEnum` derive annotations once Phase 2 lands.

2. **Evidence matrix**
   - Current descriptor fields.
   - Contract schema constraints.
   - Alias/canonicalize mappings.
   - Legacy source field/behavior notes.
   - Corpus usage examples.
   - Conflicts requiring human decision.

3. **Migration mapping skeletons**
   - Legacy field rename tables.
   - Legacy enum normalization tables.
   - Stage-to-graph transforms where applicable.
   - Binding/value-source transform notes.
   - Owner-decision placeholders for non-mechanical semantics.

4. **Fixture templates**
   - Minimal canonical v3.1 smoke fixture.
   - Boundary/default/range fixtures.
   - Optional legacy parity fixture where a reliable oracle exists.
   - Source materialization fixture for source descriptors.

5. **Validation manifests and reports**
   - Descriptor schema validation target.
   - Field coverage target.
   - Runtime support target.
   - Corpus load delta target.
   - Generated docs/control metadata report.

6. **Bootstrap/codegen checklist**
   - Remove the id from `primitive.bootstrap.json`.
   - Run `cargo xtask descriptors gen/check`.
   - Confirm generated `primitive.json` remains complete and deterministic.

## What tooling should not generate

The workbench must not hide visual or runtime decisions behind generated code.

Human-owned work remains:

- actual visual/effect behavior;
- v3.1-native semantic choices when legacy fields differ;
- compatibility alias acceptance/rejection;
- parity approval or explicit non-parity decision;
- nuanced edge-case tests;
- common helper extraction decisions;
- removal or deferral decisions for v3.1-only/no-legacy descriptors.

Forbidden generated paths:

- bridge/shim DTOs that adapt v3.1 back into `CompositionSpec`, `ShaderLayerSpec`, or `SpatialShaderType`;
- runtime support that imports `tui-vfx-compositor`;
- hand-edit patches to generated `primitive.json` after descriptor codegen lands;
- permissive fallback behavior that turns unknown or unsupported fields into silent no-ops.

## Mapping is a translation matrix, not blind find/replace

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
legacy effect mutates an entire grid while v3.1 domain is per-cell/per-surface
legacy binding shape does not match canonical ValueSource shape
recipe encodes visual intent that belongs in scene/source metadata
```

The workbench should report the distinction, not flatten it into aliases.

## First from-scratch workflow test

After the workbench proves itself on migrated primitives, use `source.indexedField` from [`../design/post-release/indexed-palette-cycling-spec.md`](../design/post-release/indexed-palette-cycling-spec.md) as a from-scratch source descriptor test.

Classification:

```text
source.indexedField is a source descriptor, not an effect descriptor.
```

It should prove that the workbench can start from contract-compatible intent, scaffold `SourcePrimitive` / `SourceRuntime`, create fixtures, validate output, and accept a new primitive without broad contract churn.

## Possible command surface

Names are not final, but the command responsibilities should be explicit:

```bash
# Descriptor artifact loop accepted by the Rust-SSOT plan
cargo xtask descriptors gen
cargo xtask descriptors check

# Workbench scaffolding/reporting surface; exact command group TBD
primitive-workbench inspect filter.vignette
primitive-workbench scaffold filter.vignette
primitive-workbench migrate-evidence filter.vignette \
  --legacy crates/tui-vfx-compositor/src/filters/cls_vignette.rs
primitive-workbench validate filter.vignette
primitive-workbench report filter.vignette
```

If the workbench is implemented inside `xtask`, keep descriptor generation (`cargo xtask descriptors`) separate from scaffold/evidence commands so generated-artifact CI remains small and dependable.

## Open design questions

These can wait until after the Rust-SSOT Phase 1 loop exists:

1. Should generated scaffolds be emitted as candidate patches, temp files, or directly into the compost tree with protected hand-owned sections?
2. Should workbench reports be checked in per primitive, or treated as ephemeral review artifacts?
3. What exact manifest format records per-primitive evidence and decisions?
4. Should workbench validation call cargo/xtask directly or only emit the commands to run?
5. How should generated docs/control metadata flow into studio/tooling once primitive descriptors are Rust-derived?
6. How should the workbench track v3.1-only/no-legacy descriptors and timeout unresolved authorial decisions?

## Durable principle

```text
The v3.1 contract schema defines valid shapes.
Rust primitive/source declarations in tui-vfx-compost own accepted catalog facts.
The Primitive Workbench derives scaffolds, evidence reports, fixtures, and validation gates.
Runtime passes loaded canonical v3.1 structures and sample context directly to tui-vfx-compost.
Humans fill behavior and semantic decisions in compost-owned modules.
primitive.json is generated for external consumers and locked against drift.
```

<!-- <FILE>docs/arch/primitive-workbench-schema-driven-workflow.md</FILE> - <DESC>Companion workflow for schema-constrained primitive scaffolding, migration evidence, validation, and workbench tooling under the accepted Rust-owned v3.1 primitive catalog instance direction</DESC> -->
<!-- <VERS>END OF VERSION: 0.10.0</VERS> -->
