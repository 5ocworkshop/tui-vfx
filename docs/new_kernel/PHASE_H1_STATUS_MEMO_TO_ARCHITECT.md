<!-- <FILE>docs/new_kernel/PHASE_H1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase H1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H1 wrap: report canonical recipe document schema and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase H1 architect memo in the established status-memo style.</CLOG> -->

# Phase H1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: H1 — Canonical Recipe Document Schema

## Executive summary

Phase H1 implements the canonical recipe document schema direction from `ARCH-RESP-TO-PHASE_H0.md` and adds the requested `docs/VOCABULARY.md` standing artifact.

Current answer: **the contract crate can now package the already-locked v3.1 contract pieces into one strict canonical recipe document. `RecipeDocument` owns metadata, assets, source descriptors, source instances, one canonical `GraphSpec`, and source-backed `RecipeScene` values. Scene elements point at source-produced surfaces by `SourceInstanceId`, and optional element-local pipelines reference the canonical graph/topology as a future integration seam rather than old recipe syntax.**

The phase intentionally stops before template expansion, migration, runtime stores, binding execution, phase/trigger/dwell engines, studio manifests, demo/player behavior, asset loading, procedural rendering, real ports, and visual parity.

## Current implementation state

Stable contract crate:

```text
crates/tui-vfx-contract
```

New contract vocabulary:

```text
RecipeId
SceneId
SourceInstanceId
RecipeMetadata
RecipeDocument
RecipeScene
RecipeSceneElement
RecipeElementPipeline
```

New checked schema roots:

```text
schemas/v3.1/contract/source-instance-id.schema.json
schemas/v3.1/contract/recipe-metadata.schema.json
schemas/v3.1/contract/recipe-element-pipeline.schema.json
schemas/v3.1/contract/recipe-scene.schema.json
schemas/v3.1/contract/recipe-scene-element.schema.json
schemas/v3.1/contract/recipe.schema.json
```

## Goal-by-goal status against the H1 recommendation

| H1 goal / question | Current status |
|---|---|
| Canonical root object | **Done.** `RecipeDocument` is the strict canonical recipe root. |
| One graph, one scene, or both | **Answered.** H1 uses one canonical `GraphSpec` plus one or more `RecipeScene` values. |
| Scene elements vs graph nodes | **Answered.** Scene elements are a parallel scene layer beside graph nodes; they reference source-produced surfaces and may reference graph/topology for local pipelines. |
| SourceSpec location | **Done.** `RecipeDocument.sources` maps `SourceInstanceId` to `SourceSpec`. |
| AssetSpec location | **Done.** `RecipeDocument.assets` declares structural assets once. |
| Source-produced surface references | **Done.** `RecipeSceneElement.source` references a declared source instance. |
| Element-local pipeline references | **Done.** `RecipeElementPipeline` references graph id plus optional `GraphStep` subset. |
| Parameters/signals/bindings attachment | **Preserved.** These remain owned by `GraphSpec` and source/node inputs consume them via `ValueSource`. |
| H1 omissions | **Documented.** Template expansion, migration, runtime stores, binding execution, timing/trigger engines, studio/demo/player, asset loading, procedural rendering, real ports, and visual parity remain out. |
| Legacy fields lowering away | **Documented.** `H1_RECIPE_EVIDENCE_NOTES.md` records old concepts and canonical homes without preserving old field names. |

## Vocabulary artifact

Created:

```text
docs/VOCABULARY.md
```

It defines canonical terms, ownership, Rust/schema surfaces, “not the same as” distinctions, legacy/source-authoring synonyms, naming rules, intentionally deferred terms, and change policy.

Important policy now captured:

```text
Canonical v3.1 is not designed around the old recipe shape.
Legacy recipes are evidence only.
```

## Recipe-evidence grounding

Created:

```text
docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md
```

The curated recipe set from `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` was read as evidence only. The notes classify concepts such as old pipeline steps, graph I/O hints, assets, source layers, procedural params, runtime bindings, dwell, motion, loopback, and source-local pipelines into canonical homes or deferred phases.

The key result is that representative evidence maps without requiring new top-level concepts beyond the H1 recipe package. Remaining pressure is descriptor porting, migration/lowering rules, runtime/timing/studio/demo work, or future port implementation.

## Key decisions

### RecipeDocument packages, it does not author

H1 treats the canonical recipe document as the output of future authoring/lowering. It does not accept source-authoring conveniences, legacy aliases, interpolation strings, or template inheritance.

### Scenes and graphs are separate but connected

The graph owns descriptor-backed effect nodes, parameters, signals, bindings, topology, and graph values. Recipe scenes own source-backed placed elements. Element-local pipelines may reference graph/topology, but execution is still deferred.

### Vocabulary is now a phase gate

Every future phase that adds or changes public contract vocabulary should update `docs/VOCABULARY.md` in the same way schemas and status memos are maintained.

## Verification evidence

Verification run so far:

```text
cargo fmt --package tui-vfx-contract -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo test -p tui-vfx-contract --test test_recipe_document_contract
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test -p tui-vfx-contract
cargo tree -p tui-vfx-contract
forbidden legacy crate grep over tui-vfx-contract
git diff --check
cargo test --workspace
```

Final wrap verification passed: workspace tests passed, forbidden legacy dependency grep produced no matches, and `git diff --check` passed. Architect re-review approved after the vocabulary and graph-value-shape fixes. Deslop review completed on H1-owned changed files with no broad refactor needed.

## Request for next assignment

Please review Phase H1 as the canonical recipe document schema and vocabulary lock point and advise the next phase.

The likely next step is to decide whether to:

```text
H2 — canonical recipe lowering/compiler skeleton
```

or to add one missing contract concept first if your review finds a schema-lock pressure gap.

<!-- <FILE>docs/new_kernel/PHASE_H1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase H1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
