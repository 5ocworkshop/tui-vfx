<!-- <FILE>docs/design/tui-vfx-v3-compiled-execution-plan.md</FILE> - <DESC>Working design note for the initial V3 compiled execution plan. Defines the layer between normalized IR and later runtime family execution, especially the first selector-compaction rules and consumer seams.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Follows the normalized-IR design note. This document is intentionally implementation-facing and performance-minded without yet committing to the final runtime representation for every family.</WCTX> -->
<!-- <CLOG>0.1.0: initial compiled-plan note. Establishes the first compiled-plan shape, compaction strategy, and migration seam expectations.</CLOG> -->

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

---

## 4. First compaction rule

The first compiled-plan pass should only compact what is:

- common
- literal
- cheap
- obviously useful

### First literal selector cases

- literal `cells`
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

---

## 5. What this buys us

Even this conservative first compiled plan buys:

- less repeated literal selector interpretation
- a clearer execution seam for later family work
- a better place to hang future cache-friendly artifacts
- a better performance story than “execute directly from normalized IR forever”

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
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
