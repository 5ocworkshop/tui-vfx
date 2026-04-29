<!-- <FILE>docs/new_kernel/PHASE_H1_STATUS.md</FILE> - <DESC>Phase H1 canonical recipe document schema implementation status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H1 wrap: summarize recipe document contracts, vocabulary, evidence notes, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase H1 status for architect handoff.</CLOG> -->

# Phase H1 Status — Canonical Recipe Document Schema

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: H1 — Canonical Recipe Document Schema

## Summary

Phase H1 implements the strict canonical v3.1 recipe document root recommended in `ARCH-RESP-TO-PHASE_H0.md`.

Current answer: **`tui-vfx-contract` can now represent and validate one canonical post-authoring recipe document that packages metadata, assets, source descriptors, source instances, the canonical graph, and source-backed scenes. Recipe scene elements reference source-produced surfaces through `SourceInstanceId`, and `RecipeElementPipeline` provides a graph/topology reference seam for future source-local pipeline integration without adopting legacy recipe syntax.**

H1 also creates `docs/VOCABULARY.md` as the standing human contract vocabulary and `docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md` as curated mapping-pressure notes from representative recipes.

## Implemented contract APIs

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

New validation errors were added to `DescriptorValidationError` for recipe/document-level failures.

## Recipe concepts locked

- `RecipeDocument` is the canonical post-authoring/lowering root.
- A recipe contains one `GraphSpec` plus one or more `RecipeScene` declarations.
- `RecipeDocument.sources` keys `SourceSpec` instances by recipe-local `SourceInstanceId`.
- `RecipeDocument.source_descriptors` declares source descriptor vocabulary used by those instances.
- `RecipeDocument.assets` declares structural assets available to source instances.
- `RecipeSceneElement.source` references a source-produced surface through a source instance id.
- `RecipeElementPipeline` references a graph/topology subset for future element-local/source-local pipeline integration.
- Canonical recipe shape is not old source-authoring syntax.

## Vocabulary and evidence artifacts

```text
docs/VOCABULARY.md
docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md
```

`VOCABULARY.md` defines canonical terms, ownership, schema/Rust surfaces, “not the same as” distinctions, legacy/source-authoring synonyms, naming rules, deferrals, and change policy.

`H1_RECIPE_EVIDENCE_NOTES.md` records representative recipe concepts as mapping pressure only. It explicitly rejects turning old names such as `config.pipeline.step`, `io.outputs[].hint`, `requires_assets`, `{{ flag_art }}`, or `scene.layers[]` into canonical vocabulary owners.

## Tests added/updated

```text
crates/tui-vfx-contract/tests/test_recipe_document_contract.rs
crates/tui-vfx-contract/tests/test_schema_generation.rs
```

Coverage includes:

- valid canonical recipe document passes
- invalid recipe id rejected
- unknown parameter refs rejected through graph validation
- unknown signal refs rejected through graph validation
- unknown source descriptor refs rejected through source validation
- unknown effect refs rejected through graph validation
- node input kind mismatch rejected through graph validation
- missing required source input rejected through source validation
- unknown asset ref rejected through source/asset validation
- structural asset ref accepted
- unknown scene element source instance rejected
- unknown element pipeline graph rejected
- unknown element pipeline node rejected
- duplicate element pipeline node rejected
- graph order errors rejected through graph validation
- graph topology errors rejected through graph validation
- graph value reference and graph value shape issues rejected through graph validation
- recipe schema roots generated/current/strict/rustdoc-described

## Docs updated

```text
docs/INDEX.md
docs/VOCABULARY.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/ARCH-RESP-TO-PHASE_H0.md
docs/new_kernel/H1_RECIPE_EVIDENCE_NOTES.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-feature-contract-checklist.md
docs/v3.1-surface-contract.md
```

## Deliberately not added

```text
template expansion implementation
legacy migration implementation
runtime ParameterStore / SignalStore
binding execution
phase graph / trigger / dwell engine
studio manifest
demo loopback execution
asset loading
procedural rendering
real effect/source ports
visual parity
legacy aliases or interpolation syntax
```

## Verification status

Phase verification run so far:

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

Final wrap verification passed: workspace tests passed, forbidden dependency grep produced no matches, and `git diff --check` passed. Architect re-review approved after the vocabulary and graph-value-shape fixes. Deslop review completed on H1-owned changed files with no broad refactor needed.

## Worktree note

The following pre-existing unrelated files remain outside Phase H1 scope and should not be staged into the H1 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_H1_STATUS.md</FILE> - <DESC>Phase H1 canonical recipe document schema implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
