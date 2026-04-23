<!-- <FILE>docs/design/tui-vfx-v3-compiled-execution-plan.md</FILE> - <DESC>Working design note for the initial V3 compiled execution plan. Defines the layer between normalized IR and later runtime family execution, especially the first selector-compaction rules and consumer seams.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Follows the normalized-IR design note. This document is intentionally implementation-facing and performance-minded without yet committing to the final runtime representation for every family. Updated to reflect the active migration strategy: use the compiled tree as an execution source directly when possible, and use the replay bridge only as fallback.</WCTX> -->
<!-- <CLOG>0.2.0: document the current direct-execution migration strategy so the working note matches the implementation direction.
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
- `payload` can remain structurally flexible in the first pass

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

Once that milestone is reached, the main remaining execution work shifts to:

- hint-driven cross-step chaining
- broader overlap/conflict semantics for overlapping parallel branches
- general arbitrary cross-family order preservation

---

## 6. What is still intentionally deferred

The first compiled plan does **not** settle:

- final typed payload shapes for every family
- hint graph execution rules
- runtime binding evaluation
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
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
