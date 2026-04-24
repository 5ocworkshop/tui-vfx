<!-- <FILE>docs/design/tui-vfx-v3-scheduler-batching-plan.md</FILE> - <DESC>Execution plan for the next V3 slice after scene/content proofs: scheduler boundaries, parallel join semantics, and batching readiness.</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>Start the scheduler/batching follow-on after V3 I/O, scene/content, and Madeira sidebar proofs landed.</WCTX> -->
<!-- <CLOG>0.4.0: record the first bounded SCHED-04 executor optimization and render-hash drift gate.
0.3.0: add the SCHED-03 batching-readiness classification matrix, machine-checkable audit record, and SCHED-04 entry gates.
0.2.0: record the scheduler-facing filter-to-mask join fixture and root topology truth surface as the next completed semantic slice before batching optimization.
0.1.0: initial scheduler/batching execution plan with SCHED-01 parallel-join I/O proof as the first bounded slice.</CLOG> -->

# tui-vfx V3 scheduler/batching plan

## Status

Active follow-on after the V3 I/O and scene/content tranches. SCHED-01,
SCHED-02, and SCHED-03 are semantic/tooling proof slices. SCHED-04 is the first
bounded executor optimization and intentionally preserves all SCHED-03
observable truth.

The direct V3 executor already preserves authored `Sequence` boundaries, snapshot-isolates `Parallel` children, and merges parallel outputs after the parallel block. The next work is to turn that behavior from scattered tests into a documented scheduler/batching surface that future optimization can rely on.

## Goal

Make V3 execution ordering explicit enough that later batching can be optimized without changing authored semantics.

The scheduler/batching work must preserve these rules:

1. `Sequence` children observe outputs from previous siblings.
2. `Parallel` children all read the same pre-parallel snapshot.
3. Parallel outputs are merged only after the parallel block.
4. Later sequence steps can consume outputs produced by a parallel block.
5. Batching may group work only when those observable rules stay intact.

## Non-goals

- No new authoring schema in this tranche.
- No cross-layer hint exchange.
- No rewrite of all family-specific render loops.
- No frame-for-frame Madeira fireworks/backdrop parity work.

## Work packages

### SCHED-01 — parallel join I/O proof

**Status: complete.**

As-built artifact:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/v3_io_parallel_merge_shader.json`

Required behavior:

- a `Parallel` block contains a producer branch and an independent visual branch
- no sibling branch consumes the producer during the same parallel block
- a later `Sequence` child consumes the merged output through the normal V3 I/O substrate

Deliverables:

- deterministic render regression for the debug recipe
- assertion that the post-parallel filter/shader chain stays visible in compiled playback
- recipe-side docs listing the fixture as the scheduler-facing parallel-join proof

Debug recipe requirement:

- The fixture must show I/O in action between at least two effects across a scheduler boundary. `v3_io_parallel_merge_shader.json` does this by merging a spatial-signal hint from a parallel branch and consuming it in a later shader while an independent filter branch also runs.

### SCHED-02 — execution truth surface

**Status: complete.**

As-built artifacts:

- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/v3_scheduler_parallel_join_filter_mask.json`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_collect_compiled_v3_truth_surface.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs`
- `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_probe_mode.rs`

Deliverables:

- extend compiled/probe truth reporting so reviewers can see root pipeline tree shape, layer-local tree shape, and direct-executor coverage without reading Rust internals
- keep reporting truthful about probe limitations: root compositor stage analysis is not the same as compiled V3 tree analysis
- add tests around the scheduler-facing debug fixtures

As-built behavior:

- `CompiledV3TruthSurface` reports root pipeline counts, root topology tree,
  total root node count, and root `Parallel` join count.
- Probe mode emits `support_truth` for root-only compiled V3 recipes, not just
  scene-bearing recipes.
- Debug-recipes QC adds `compiled_v3_pipeline_topology` and
  `scheduler_parallel_join` checks for scheduler fixtures.
- The new fixture proves a time-varying scalar emitted in a `Parallel` branch
  can be consumed by a downstream filter, re-emitted via `io.outputs.source`,
  and consumed by a checker mask after the join.

### SCHED-03 — batching-readiness audit

**Status: complete.**

As-built artifact:

- `/usr/projects/tui-vfx-recipes/docs/v3_scheduler_batching_audit.json`
- `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/complex/v3_scheduler_batch_safe_channel_shader_style.json`
- `/usr/projects/tui-vfx-recipes/tests/test_v3_scheduler_batching_audit.rs`
- Rustdoc-facing notes in:
  - `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
  - `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_collect_compiled_v3_truth_surface.rs`
  - `/usr/projects/tui-vfx-recipes/tools/pipeline-validator/src/fnc_run_debug_recipes_qc.rs`

Deliverables:

- classify which leaf families are safe to batch by scope/channel/role without altering output merge semantics
- document any family whose output semantics require serial execution
- avoid optimizing until the classification has tests and fixture evidence

As-built classification:

| Class                          | Families                             | Safe condition                                                                                                | Serial-required boundary                                                                                                 |
| ------------------------------ | ------------------------------------ | ------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------ |
| Disjoint static scopes         | mask, sampler, filter, shader, style | every branch has literal static bounds and branch bounds are pairwise non-overlapping                         | dynamic scopes, unknown bounds, or any overlap outside another proven safe class                                         |
| Channel-exclusive local writes | filter, shader, style                | every branch is scoped to one exclusive channel and the effect family is in the channel-local whitelist       | masks, samplers, unknown channel-locality, or filter/shader/style families outside the whitelist                         |
| Role-exclusive visual writes   | shader, style                        | every branch is scoped to a distinct semantic role/border and every leaf is a visual shader/style-effect step | masks, samplers, filters, role overlap, or unknown non-visual side effects                                               |
| Post-Parallel hint joins       | sampler, filter, shader, mask, style | new branch hints are withheld from siblings, merged at the join, then consumed by later `Sequence` siblings   | sibling cross-feed inside one `Parallel`, or duplicate hint outputs unless the current first-new-wins order is preserved |

Machine-checkable entry gates for SCHED-04:

1. `docs/v3_scheduler_batching_audit.json` remains valid and is cited by docs.
2. Every safe class has evidence tests and recipes or an explicit source
   predicate.
3. Scheduler debug recipes pass strict probe and debug-recipes QC.
4. Any optimization proves no render-hash drift for:
   - `v3_io_parallel_merge_shader.json`
   - `v3_scheduler_parallel_join_filter_mask.json`
   - `v3_scheduler_batch_safe_channel_shader_style.json`
   - `complex_parallel_overlap_conflict_snapshot.json`

### SCHED-04 — first bounded batching optimization

**Status: complete.**

As-built artifact:

- `/usr/projects/tui-vfx-recipes/src/v3/compile/fnc_execute_compiled_step_tree_to_scene.rs`
- `/usr/projects/tui-vfx-recipes/src/v3/compile/test_render_compiled_plan_deterministically.rs`

Deliverables:

- implement one measurable, reversible optimization after SCHED-02/SCHED-03
- prove no render hash drift on scheduler fixtures
- run targeted perf sanity only after semantic tests pass

As-built behavior:

- The ordered executor now routes all `Parallel` branches through one
  `execute_parallel_children_from_snapshot` helper instead of repeating the same
  branch-snapshot/merge loop separately for disjoint-static,
  channel-order-independent, role-order-independent, and generic conflict
  cases.
- SCHED-03 predicates remain available for direct-path gating and truth-surface
  classification, but execution no longer re-traverses those predicates inside
  the hot `Parallel` arm when the merge behavior is identical.
- The drift guard
  `scheduler_first_bounded_batching_optimization_preserves_fixture_hashes`
  pins render hashes for:
  - `v3_scheduler_batch_safe_channel_shader_style.json`
  - `v3_io_parallel_merge_shader.json`
  - `v3_scheduler_parallel_join_filter_mask.json`
  - `complex_parallel_overlap_conflict_snapshot.json`

## Verification baseline

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes parallel_merge_shader -- --nocapture
cargo test -p tui-vfx-recipes scheduler_parallel_join -- --nocapture
cargo test -p tui-vfx-recipes scheduler_batch_safe_channel_shader_style -- --nocapture
cargo test -p tui-vfx-recipes scheduler_batching_audit -- --nocapture
cargo test -p tui-vfx-recipes scheduler_first_bounded_batching_optimization -- --nocapture
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_io_parallel_merge_shader.json --strict --probe --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_io_parallel_merge_shader.json --debug-recipes-qc --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_parallel_join_filter_mask.json --strict --probe --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_parallel_join_filter_mask.json --debug-recipes-qc --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_batch_safe_channel_shader_style.json --strict --probe --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_batch_safe_channel_shader_style.json --debug-recipes-qc --format json
cargo test -p tui-vfx-recipes
git diff --check
```

From `/usr/projects/tui-vfx`:

```sh
npx prettier --check docs/design/tui-vfx-v3-scheduler-batching-plan.md docs/design/tui-vfx-v3-compiled-execution-plan.md docs/design/tui-vfx-v3-spatial-field-hint-plan.md
just docs-all-check
git diff --check
```

## Completion criteria

- scheduler boundary behavior has committed debug recipes and deterministic regressions
- root topology truth is visible in validator/probe/QC output for scheduler fixtures
- later batching work has a tested safe/unsafe classification path
- first bounded executor optimization has a render-hash drift gate over safe,
  hint-join, and conflict fixtures
- docs distinguish current semantic proofs from future optimization work

<!-- <FILE>docs/design/tui-vfx-v3-scheduler-batching-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
