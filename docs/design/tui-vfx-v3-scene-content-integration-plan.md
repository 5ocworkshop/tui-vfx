<!-- <FILE>docs/design/tui-vfx-v3-scene-content-integration-plan.md</FILE> - <DESC>Execution plan for the next V3 integration slice after shader/effects I/O: scene-layer procedural/braille and content composition.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Plan the next V3-only tranche after the shader/filter/mask/style-effect I/O path has root and scene-layer proofs, keeping Madeira and content composition as reusable proving grounds.</WCTX> -->
<!-- <CLOG>0.1.0: initial execution plan for scene/content integration, including debug-recipe obligations, rustdoc/docs deliverables, verification commands, and sub-agent lanes.</CLOG> -->

# tui-vfx V3 scene/content integration plan

## Status

Active follow-on after the V3 shader/effects I/O tranche.

The completed I/O tranche proves first-class producer/consumer chaining across:

- root pipelines
- scene-layer-local pipelines
- sampler, filter, shader, mask, and spatial style-effect wrapper leaves
- explicit sourced outputs from non-spatial leaves
- sequence and parallel visibility semantics

This plan deliberately does **not** reopen that substrate. It uses the landed
substrate and the existing scene/content pathways to broaden V3 toward real
showcase and authoring usage.

## Goal

Make the next V3 slice prove that scene sources and content effects compose with
the direct V3 execution path in a way that is reusable, recipe-driven, and
asset-agnostic.

The slice has two user-visible proving seams:

1. **Scene procedural / braille-dotfield seam** — a `braille_flag_field` scene
   source consumes recipe/runtime inputs and continues to load its visual asset
   through `requires_assets`, so Madeira-style visuals can change without Rust
   changes.
2. **Content + downstream pipeline seam** — a bounded content effect, starting
   with `typewriter`, generates source cells before a downstream V3 pipeline
   effect chain transforms them.

## Non-goals

- No cross-layer hint exchange.
- No second value system for scene sources or content effects.
- No arbitrary scheduler/batching redesign in this slice.
- No return to embedded Madeira artwork.
- No V2 parity work beyond keeping current cutover tests green.

## Work packages

### SC-01 — braille-dotfield runtime input proof

Deliverables:

- Add a focused debug recipe under
  `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/` where
  `braille_flag_field` loads artwork through `requires_assets` and reads an
  authored runtime binding such as `wave_speed`.
- Add deterministic render tests showing a runtime override changes the
  procedural flag output while the default path remains deterministic.
- Update recipe-side procedural docs and this repo's braille-dotfield plan to
  describe the as-built runtime-input seam.

Debug recipe requirement:

- It must prove I/O in action between a recipe/host input and the procedural
  scene source, and it must remain asset-agnostic.

### SC-02 — content + downstream pipeline proof

Deliverables:

- Add or refresh a content debug recipe where `content.effect = typewriter`
  produces source cells and a downstream V3 pipeline applies at least two
  effects in order.
- Prefer a downstream sequence that reuses the already-landed I/O substrate,
  such as sourced filter output feeding a shader, so the fixture demonstrates
  content generation plus effect-chain composition without inventing content
  hints.
- Add deterministic tests across early/late content samples and at least one
  downstream pipeline assertion.
- Update content/scene authoring docs with the as-built ordering rule: content
  resolves first, then the V3 pipeline transforms the resulting cells.

Debug recipe requirement:

- It must show content output flowing into downstream effects, not merely a
  static text card with a shader.

### SC-03 — scene/probe/tooling visibility

Deliverables:

- Ensure pipeline-validator / probe output reports the scene/procedural and
  content proof recipes with enough truth-surface detail for reviewers.
- Add focused tests only if the new fixtures reveal a missing probe/validator
  signal; otherwise keep this as docs/tooling verification.

### SC-04 — as-built docs and rustdoc pass

Deliverables:

- Keep rustdoc on touched procedural/content seams explicit about determinism,
  runtime input resolution, and no hot-path file I/O.
- Update hand-maintained docs in both repos:
  - `docs/design/tui-vfx-v3-braille-dotfield-toolkit-plan.md`
  - `docs/design/tui-vfx-v3-compiled-execution-plan.md`
  - `docs/design/tui-vfx-v3-spatial-field-hint-plan.md` when the field matrix
    changes
  - `/usr/projects/tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md`
  - `/usr/projects/tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md`
  - `/usr/projects/tui-vfx-recipes/docs/V3_FIELD_HINT_CONSUMERS.md` only when
    the first-class step I/O matrix changes

## Suggested sub-agent lanes

- **Runtime/compile lane:** inspect and update deterministic render/probe tests
  for scene procedural runtime inputs and content+pipes.
- **Recipe proving-ground lane:** build focused debug recipes and run
  `pipeline-validator` strict/QC checks.
- **Docs lane:** keep engine and recipe as-built docs synchronized with the
  implementation.
- **Verifier lane:** review each committed stage before the next stage starts.

## Verification baseline

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes
cargo run -p pipeline-validator -- <new debug recipe> --strict --probe --format json
cargo run -p pipeline-validator -- <new debug recipe> --debug-recipes-qc --format json
python3 tools/fnc_generate_v3_docs.py --check
git diff --check
```

From `/usr/projects/tui-vfx`:

```sh
npx prettier --check docs/design/tui-vfx-v3-*.md
just docs-all-check
git diff --check
```

## Completion criteria

- SC-01 and SC-02 each have a committed debug recipe, deterministic tests, docs,
  and verification evidence.
- Runtime inputs and content effects are proven as scene/content integration
  seams, not as ad hoc Madeira-only behavior.
- The next remaining V3 risk is clearly narrowed to broader scheduler/batching
  and richer showcase parity, not missing first proof of scene/content
  composition.

<!-- <FILE>docs/design/tui-vfx-v3-scene-content-integration-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
