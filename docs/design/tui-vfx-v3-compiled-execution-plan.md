<!-- <FILE>docs/design/tui-vfx-v3-compiled-execution-plan.md</FILE> - <DESC>Working design note for the initial V3 compiled execution plan. Defines the layer between normalized IR and later runtime family execution, especially the first selector-compaction rules and consumer seams.</DESC> -->
<!-- <VERS>VERSION: 0.15.0</VERS> -->
<!-- <WCTX>Keep the compiled execution-plan note aligned with the as-built timing model, Parallel semantics, and landed field-hint consumer proofs.</WCTX> -->
<!-- <CLOG>0.15.0: point the post-SCHED-04 execution work at the cross-family coverage plan.
0.14.0: record the first bounded SCHED-04 Parallel snapshot-batch helper optimization.
0.13.0: record the completed SCHED-03 batching-readiness audit and SCHED-04 optimization gate.
0.12.0: record root topology truth reporting and the time-varying scheduler Parallel join into filter-to-mask proof.
0.11.0: point scheduler/batching follow-on at the parallel-join I/O proof and dedicated plan. 0.10.0: record binding-backed scene-layer visibility plus local I/O proof for Madeira/sidebar parity. 0.9.0: record that top-level content effects now have a content-before-pipeline proof feeding a sourced-output filter/shader chain. 0.8.0: record that scene procedural sources now have an asset-backed runtime-input proof through braille_flag_field wave speed. 0.7.0: record that scene-layer-local pipelines now have a direct compiled I/O proof using the same sourced-output chain. 0.6.0: record the mask sourced-output consumer proof as part of the direct compiled execution coverage matrix. 0.5.0: record the sourced-output direct execution proof that lets a filter publish a bound field to a later shader consumer. 0.4.0: record that authored-order Parallel overlap semantics and the shared field-hint displacement/shading proof have landed, leaving arbitrary cross-family scheduling and broader family coverage as follow-up.
0.3.0: clarify that execution-facing timing must preserve normalized phase/loop progress separately from monotonic elapsed time, and that cadence-driven motion reads elapsed time.
0.2.0: document the current direct-execution migration strategy so the working note matches the implementation direction.
0.1.0: initial compiled-plan note. Establishes the first compiled-plan shape, compaction strategy, and migration seam expectations.</CLOG> -->

# tui-vfx V3 compiled execution plan

This document describes the next execution-facing layer after normalized IR.

Its role is to sit between:

- normalized IR
- and later runtime family execution

It is not the authoring schema.
It is not the canonical migration/viewer/validator artifact.
It is the first tighter representation meant to reduce obviously repeated work before later execution code runs.

---

## 1. Position in the stack

```text
authoring schema
  -> normalized IR
    -> compiled execution plan
      -> runtime family execution
```

### Layer roles

- **Authoring schema**
  - ergonomic
  - expressive
  - tolerant of authoring sugar

- **Normalized IR**
  - canonical
  - validator/viewer/tooling target
  - migration-equivalence target

- **Compiled execution plan**
  - tighter execution-facing representation
  - compacts common selector forms
  - preserves operational-lane identity
  - keeps canonicalization out of later paths
  - assumes normalized validation has already passed

---

## 2. Candidate top-level shape

```text
CompiledRecipePlan
├─ identity
├─ contracts
├─ envelope
│  ├─ motion_host?
│  ├─ attached_shadow?
│  └─ visual_envelope?
├─ scene?
│  └─ layers[]
│     ├─ id
│     ├─ role_tag
│     ├─ source
│     ├─ placement
│     ├─ surface
│     └─ pipeline
└─ pipeline
```

And the compiled pipeline preserves tree shape:

```text
CompiledStep
├─ Parallel([CompiledStep...])
├─ Sequence([CompiledStep...])
└─ Leaf(CompiledLeafStep)
```

That means:

- structural composition survives
- lane identity moves onto the leaf
- future executors can target one leaf shape

---

## 3. Candidate compiled leaf shape

```text
CompiledLeafStep
├─ kind
├─ phase
├─ scope
├─ clock?
├─ timing_view?
├─ interaction?
├─ payload
└─ provenance?
```

### Notes

- `kind` is the operational lane:
  - `mask`
  - `sampler`
  - `filter`
  - `shader`
  - `style_effect`
- `phase` remains explicit
- `scope` may now be compacted
- `timing_view` should preserve both normalized progress and monotonic elapsed
  time for the active execution slice
- `payload` can remain structurally flexible in the first pass

### Timing-view rule

Execution-facing timing should not collapse to one scalar. The compiled/runtime
path should keep:

- normalized progress values for lifecycle/phase-aware interpolation
- monotonic elapsed time for cadence-sensitive motion

This matters because normalized progress may reset at loop boundaries while
elapsed time does not. Cadence-driven filters and scanners should read elapsed
time directly instead of rebuilding cadence from loop progress.

## 3.1 Envelope motion + shadow compilation

The compiled execution plan should not lose the recipe-envelope motion/shadow
model introduced in the author-facing V3 drafts.

At minimum, the compiled envelope should preserve:

- normalized recipe motion
- normalized scene-layer motion
- attached shadow ownership
- host / visual envelope data needed for shared screen-edge handling

That compiled envelope data is what later runtime passes should consume when
making:

- border trim decisions
- shadow fade / clip decisions
- probe / validator motion-boundary diagnostics

---

## 4. First compaction rule

The first compiled-plan pass should only compact what is:

- common
- literal
- cheap
- obviously useful

### First literal selector cases

- literal `cells`
- literal `cell_run`
- literal `rows`
- literal `row_range`
- literal `columns`
- literal `column_range`

Those can become tighter typed forms like:

- `Vec<(u16, u16)>`
- `Vec<u16>`
- `(u16, u16)` ranges

### Fallback rule

If a selector is not fully literal, the compiled plan should keep:

- a dynamic fallback carrying the normalized scope

That keeps the first compiled plan safe and incremental.

### Preservation rule

To keep the best compaction opportunities available, normalization should preserve compact selector forms such as:

- `cell_run`
- `cell_runs`

rather than eagerly exploding them into flat cell lists.

---

## 5. What this buys us

Even this conservative first compiled plan buys:

- less repeated literal selector interpretation
- a clearer execution seam for later family work
- a better place to hang future cache-friendly artifacts
- a better performance story than “execute directly from normalized IR forever”
- fewer avoidable whole-recipe clones when load/compile paths can consume normalized IR directly

---

## 5.1 Direct execution before bridge fallback

The compiled execution plan should not be treated as a staging format that is
immediately collapsed back into the old replay-contract shape.

The active migration direction should be:

```text
compiled tree
  -> direct ordered executor when supported
  -> replay-contract bridge only when not yet supported
```

Why:

- some authored trees are already easier to execute directly than to lower into
  the old replay buckets
- the replay contract still carries bridge-era limits (for example, bucketed
  sampler restrictions)
- building the bridge first can reject trees that the direct compiled executor
  could already run

So the compiled plan is not only a future optimization seam.
It is also the immediate route toward an independent V3 execution path.

That direct path should preserve timing truth as well as tree shape. A direct
executor that drops elapsed time and keeps only normalized loop progress would
recreate the same cadence discontinuities the migration is meant to remove.

### 5.2 Current migration milestone

The migration should explicitly recognize the point where the compiled V3 path
no longer contains any `render_pipeline_with_spec(...)` callsites under
`src/v3/compile/`.

That milestone matters because it means:

- the compiled-path execution seam is no longer delegating back into the old
  compositor replay helper as an implementation crutch
- remaining V3 gaps are now primarily about **semantics**, not “one more hidden
  bridge call”
- future regressions should be treated as regressions in native coverage, not
  as acceptable fallback behavior

Recent as-built follow-up has landed two more execution proofs:

- overlapping `Parallel` branches are snapshot-isolated and merge conflicts in
  authored order, with the later branch winning overlapping output conflicts
- one `spatial_signal` hint can feed both a displacement sampler and a
  field-correlated shader in the same authored `Sequence`
- an explicit `io.outputs[].source` path can publish a bound non-spatial leaf
  payload field, proven by a dim filter re-emitting its `factor` to a later
  diffusion shader and to a downstream checkers mask
- scene-layer-local pipelines now run the same direct I/O chain, proven by
  `scene_layer_io_filter_shader.json` with a layer-local spatial signal feeding
  a filter sourced output and downstream shader
- scene procedural sources can consume runtime-bound JSON params, proven by
  `scene_braille_flag_runtime_wave.json` changing `braille_flag_field` wave speed
  over the same `requires_assets` dotfield
- top-level content effects resolve before downstream pipeline execution, proven
  by `content_typewriter_io_filter_shader.json` where typewriter output feeds a
  sourced-output filter/shader chain
- scene-layer predicate visibility can resolve from `requires_bindings` /
  `ShaderRuntimeParams` before composition, proven by
  `scene_layer_visibility_binding_io.json` where a binding-gated layer still
  runs a layer-local sourced-output I/O chain when visible

The main remaining execution work now shifts to:

- broader family coverage for hint-driven consumers beyond the current Madeira/sidebar proofs, now tracked in `tui-vfx-v3-cross-family-coverage-plan.md`
- general arbitrary cross-family order preservation, now backed by both `v3_io_parallel_merge_shader.json` and `v3_scheduler_parallel_join_filter_mask.json` and expanded through the XFC plan
- final scheduling / batching strategy tracked in `tui-vfx-v3-scheduler-batching-plan.md`; SCHED-03 now provides the machine-checkable safe/serial classification record, and SCHED-04 has landed the first bounded optimization by collapsing duplicate `Parallel` branch loops into one snapshot-batch helper with render-hash drift guards

---

## 6. What is still intentionally deferred

The first compiled plan does **not** settle:

- final typed payload shapes for every family
- hint graph execution rules
- broader runtime binding evaluation outside the currently proven shader/procedural/scene-visibility seams
- final scheduling / batching strategy
- per-family render-loop specialization

Those are later steps.

This layer should stay modest and honest.

---

## 7. First propagation path

To keep the phase real, it should propagate through the same shallow seam family as V2:

1. `src/v3/*`
2. `src/recipe/*`
3. `src/prelude.rs`
4. `src/lib.rs`
5. one shallow tool/example consumer

That is the minimal proof that the compiled plan is becoming part of the real public pathway rather than staying trapped in an internal helper module.

<!-- <FILE>docs/design/tui-vfx-v3-compiled-execution-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.15.0</VERS> -->
