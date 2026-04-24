<!-- <FILE>docs/design/tui-vfx-v3-scheduler-batching-plan.md</FILE> - <DESC>Execution plan for the next V3 slice after scene/content proofs: scheduler boundaries, parallel join semantics, and batching readiness.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Start the scheduler/batching follow-on after V3 I/O, scene/content, and Madeira sidebar proofs landed.</WCTX> -->
<!-- <CLOG>0.2.0: record the scheduler-facing filter-to-mask join fixture and root topology truth surface as the next completed semantic slice before batching optimization.
0.1.0: initial scheduler/batching execution plan with SCHED-01 parallel-join I/O proof as the first bounded slice.</CLOG> -->

# tui-vfx V3 scheduler/batching plan

## Status

Active follow-on after the V3 I/O and scene/content tranches. SCHED-01 and
SCHED-02 are now semantic/tooling proof slices; optimization remains gated on
SCHED-03.

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

**Status: planned.**

Deliverables:

- classify which leaf families are safe to batch by scope/channel/role without altering output merge semantics
- document any family whose output semantics require serial execution
- avoid optimizing until the classification has tests and fixture evidence

### SCHED-04 — first bounded batching optimization

**Status: planned.**

Deliverables:

- implement one measurable, reversible optimization after SCHED-02/SCHED-03
- prove no render hash drift on scheduler fixtures
- run targeted perf sanity only after semantic tests pass

## Verification baseline

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo fmt --all --check
cargo test -p tui-vfx-recipes parallel_merge_shader -- --nocapture
cargo test -p tui-vfx-recipes scheduler_parallel_join -- --nocapture
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_io_parallel_merge_shader.json --strict --probe --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_io_parallel_merge_shader.json --debug-recipes-qc --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_parallel_join_filter_mask.json --strict --probe --format json
cargo run -p pipeline-validator -- recipes/debug_recipes/complex/v3_scheduler_parallel_join_filter_mask.json --debug-recipes-qc --format json
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
- later batching work has a documented safe/unsafe classification path
- docs distinguish current semantic proofs from future optimization work

<!-- <FILE>docs/design/tui-vfx-v3-scheduler-batching-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
