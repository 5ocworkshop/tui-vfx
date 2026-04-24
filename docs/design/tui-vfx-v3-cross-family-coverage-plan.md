<!-- <FILE>docs/design/tui-vfx-v3-cross-family-coverage-plan.md</FILE> - <DESC>Execution plan for the next V3 tranche after I/O and scheduler proofs: cross-family ordering, broader consumer coverage, and truth-surface evidence.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Start the post-SCHED-04 V3 integration slice. The direct executor now has I/O, scene/content, Madeira asset/runtime visibility, scheduler join semantics, and a first bounded Parallel optimization; the next risk is broader family coverage without reopening those foundations.</WCTX> -->
<!-- <CLOG>0.2.0: mark XFC-01 complete after the root cross-family ordered sequence fixture and truth-surface tests landed in tui-vfx-recipes.
0.1.0: initial cross-family coverage plan with XFC stages, debug-recipe obligations, rustdoc/as-built docs requirements, and verification baseline.</CLOG> -->

# tui-vfx V3 cross-family coverage plan

## Status

Active follow-on after the completed V3 I/O, scene/content, Madeira asset, runtime visibility, and SCHED-01 through SCHED-04 scheduler tranches.

The project now has committed proof that:

- `Sequence` is the serial dataflow boundary for V3 hints and rendered cells
- `Parallel` is a snapshot-isolated branch boundary with post-join hint visibility
- root and scene-layer-local pipelines use the same V3 I/O substrate
- content sources can feed downstream V3 effects
- Madeira-style procedural assets are loaded through recipe-declared assets instead of Rust-embedded artwork
- scheduler readiness has a machine-checkable truth surface and the first bounded executor optimization has render-hash drift guards

This plan deliberately does **not** reopen those contracts. It expands coverage on top of them.

## Goal

Make the V3 direct execution path visibly robust across mixed effect families, not only across one-family or showcase-specific chains.

The tranche proves that recipes can combine sampler, filter, mask, shader, and style-effect leaves in authored order while preserving the existing I/O and scheduler semantics.

## Non-goals

- No V2 compatibility expansion.
- No cross-layer hint exchange.
- No new dependency or threaded batching model.
- No new authoring vocabulary unless an existing fixture proves the current vocabulary cannot express the chain.
- No broad family rewrite in one commit; execute in small, reviewable coverage slices.

## Work packages

### XFC-01 — cross-family sequence proof

**Status: complete.**

As-built artifacts:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/v3_cross_family_sequence_disjoint.json`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`
- `/usr/projects/tui-vfx-recipes/tests/test_debug_recipes_qc.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/tests/test_v3_probe_mode.rs`
- `/usr/projects/tui-vfx-recipes/docs/V3_FIELD_HINT_CONSUMERS.md`

Deliverables:

- Add a focused debug recipe in `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/` that executes a single authored `Sequence` spanning at least four family lanes.
- The fixture must show I/O between at least two effects, not just independent visual steps. Preferred chain:
  - `spatial_signal` sampler emits a scalar field
  - displacement sampler consumes the field
  - filter consumes or re-emits a sourced scalar
  - mask consumes the sourced scalar
  - shader shades the masked/sampled result
  - optional style-effect/base-style pass confirms style effects still compose after the spatial families
- Add deterministic render and shape assertions for the fixture.
- Add strict/probe and debug-recipes-QC validation for the fixture.

Debug recipe requirement:

- The description must tell reviewers what visible I/O chain to expect.
- The fixture must remain V3-only and recipe-driven.

### XFC-02 — parallel join plus post-merge consumer proof

**Status: planned.**

Deliverables:

- Add or extend a fixture where a `Parallel` block mixes non-identical families, one branch emits a hint, and a later `Sequence` child consumes the joined value through a different family lane.
- Assert the existing post-join visibility semantics and no sibling cross-feed.
- Extend probe/truth-surface assertions only if the current truth surface cannot identify the family mix and join semantics clearly.

Debug recipe requirement:

- The fixture must make branch isolation and post-join visibility explicit in `metadata.authoring_notes`.

### XFC-03 — overlap/conflict guard proof

**Status: planned.**

Deliverables:

- Add or extend a forced-overlap fixture proving unsafe mixed-family branches stay serial-required under the scheduler classification.
- Pin deterministic output so future batching cannot silently reinterpret an overlap conflict as safe parallel execution.
- Update the batching audit record only for cases that add new classification evidence.

Debug recipe requirement:

- The fixture must explain which branches intentionally conflict and which authored-order result wins.

### XFC-04 — truth-surface and rustdoc pass

**Status: planned.**

Deliverables:

- Ensure pipeline-validator/probe/QC output reports enough root topology, family counts, and scheduler readiness evidence for every XFC fixture.
- Add rustdoc to touched executor/classification helpers describing any new family lane or conflict semantics.
- Keep the recipe-side as-built docs aligned:
  - `/usr/projects/tui-vfx-recipes/docs/V3_FIELD_HINT_CONSUMERS.md`
  - `/usr/projects/tui-vfx-recipes/docs/V3_PARALLEL_EXECUTION.md`
  - recipe debug docs/generated docs when fixture inventories change

### XFC-05 — hand-maintained plan/docs closure

**Status: planned.**

Deliverables:

- Update this plan with completed XFC artifacts.
- Update:
  - `docs/design/tui-vfx-v3-compiled-execution-plan.md`
  - `docs/design/tui-vfx-v3-scheduler-batching-plan.md` when scheduler classification changes
  - `docs/design/tui-vfx-v3-spatial-field-hint-plan.md` when the hint-consumer matrix changes
- Keep rustdoc and hand-maintained docs as first-class deliverables, not afterthoughts.

## Suggested sub-agent lanes

- **Fixture/test lane:** propose bounded V3 debug recipes and deterministic assertions.
- **Truth-surface lane:** inspect validator/probe/QC output and identify missing evidence fields.
- **Docs lane:** maintain as-built docs and generated-doc freshness.
- **Verifier lane:** review each stage for semantic drift against SCHED-04 invariants.

## Verification baseline

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes <new_xfc_test_name> -- --nocapture
cargo test -p tui-vfx-recipes scheduler_first_bounded_batching_optimization -- --nocapture
cargo test -p tui-vfx-recipes scheduler_batching_audit -- --nocapture
cargo run -p pipeline-validator -- <new debug recipe> --strict --probe --format json
cargo run -p pipeline-validator -- <new debug recipe> --debug-recipes-qc --format json
python3 tools/fnc_generate_v3_docs.py --check
git diff --check
```

Run the full `cargo test -p tui-vfx-recipes` and `cargo test -p pipeline-validator` before closing a multi-file XFC stage.

From `/usr/projects/tui-vfx`:

```sh
npx prettier --check docs/design/tui-vfx-v3-cross-family-coverage-plan.md docs/design/tui-vfx-v3-compiled-execution-plan.md docs/design/tui-vfx-v3-scheduler-batching-plan.md docs/design/tui-vfx-v3-spatial-field-hint-plan.md
just docs-all-check
git diff --check
```

Known generated-doc warnings are acceptable only when they match the current documented `shaders.Highlighter` and `content.SplitFlap` ai-hint parameter warnings.

## Completion criteria

- XFC-01 has a committed cross-family sequence debug recipe and deterministic/probe/QC tests; parallel-join and overlap/conflict behavior remain XFC-02/XFC-03 follow-ups.
- Existing SCHED-01 through SCHED-04 fixtures keep their render hashes.
- Probe/QC output makes the family mix and scheduler semantics visible to reviewers.
- Rustdoc and hand-maintained docs describe the as-built system.
- The next V3 risk is narrowed to a specific release-gate area such as motion-path/offscreen support or timing-model rationalization, not generic cross-family execution uncertainty.

<!-- <FILE>docs/design/tui-vfx-v3-cross-family-coverage-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
