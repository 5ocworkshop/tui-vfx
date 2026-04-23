<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/62_compiled_execution_plan.md</FILE> - <DESC>Chapter 62 — compiled execution-plan phase. Defines the follow-on after the first normalized-IR slice: introduce a tighter execution-facing plan that compacts common selectors, reduces dynamic branching, and gives later runtime work a more performance-minded target.</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Follows Chapter 61's first implementation slice. The normalized IR is the canonical structural contract; this phase adds a compiled plan layer so later runtime work does not treat the normalized IR itself as the final hot-path representation. Updated to reflect the active implementation strategy: execute supported compiled trees directly first, and consult the legacy replay bridge only as fallback.</WCTX> -->
<!-- <CLOG>1.1.0: document the active direct-path strategy — ordered compiled-step execution first, replay-contract fallback second — so the implementation plan matches the current migration direction.
1.0.0: initial chapter. Establishes the compiled execution-plan phase, its goals, minimum deliverables, performance posture, and the first consumer seams that should grow in parallel with the V2 pathway.</CLOG> -->

# 62 — Compiled Execution Plan

This chapter answers the next practical question after the first normalized-IR slice:

> Once authoring types, normalized IR, validation, and the first debug surfaces exist, what should the next execution-facing layer be?

The answer is:

- **not** “treat normalized IR as the final runtime shape forever”
- **not** “jump straight into broad family runtime support”
- **not** “keep all selector and payload interpretation dynamic in every later consumer”

The next phase should be the **compiled execution plan**.

---

## 10 — Why this phase exists

The normalized IR is the canonical structural contract.

That is the right target for:

- validator logic
- viewer / inspector logic
- migration-equivalence tooling
- recipe-debug output

But it is **not yet the final per-frame execution shape**.

The normalized IR still intentionally preserves:

- flexible payload carriers
- structural provenance
- canonical-but-not-yet-tight selectors
- enough authoring meaning for tooling and migration reasoning

That is good for correctness.
It is not the final optimization target.

So the follow-on is:

```text
authoring schema
  -> normalized IR
    -> compiled execution plan
      -> runtime family execution
```

---

## 20 — Goals of the compiled plan

The compiled execution plan should:

1. preserve the semantics of normalized IR
2. compact the most common literal selector forms
3. make operational lanes explicit for execution
4. keep structural canonicalization off later hot paths
5. remain inspectable enough for debugging and review
6. give downstream consumers a better execution-facing surface than raw normalized IR

It should **not** yet try to solve:

- all family-specific payload typing
- final per-frame buffer/layout optimization
- runtime binding evaluation
- final renderer scheduling

Those can come later.

---

## 30 — Minimum deliverables

The first compiled-plan phase should produce:

### 30.1 Compiled plan types

At minimum:

- `CompiledRecipePlan`
- `CompiledLayerPlan`
- `CompiledPipelinePlan`
- `CompiledStep`
- `CompiledLeafStep`
- compact selector forms for common literal cases

### 30.2 Compilation pass

At minimum:

- `normalized IR -> compiled execution plan`

The compilation pass should:

- operate on validated normalized IR
- preserve sequence / parallel structure
- preserve operational lane identity
- compact literal selector cases where cheap and obvious
- fall back to the normalized selector when a scope remains dynamic
- prefer consuming normalized IR in the main load/compile seam so the phase does not pay avoidable whole-recipe clone costs

### 30.3 First consumer-seam propagation

The compiled-plan phase should start mirroring the V2 consumer path upward:

1. V3 module helpers
2. recipe-layer wrappers
3. crate-root/prelude exports
4. shallow tools/examples that can validate or inspect the compiled plan

This keeps the migration lane honest:
the new structure should not exist only in isolated helper modules.

### 30.4 Direct-path execution gate

The near-term migration strategy should prefer:

1. **execute supported compiled trees directly**
2. **fall back to the replay-contract / compositor bridge only for unsupported trees**

That ordering matters.

If the replay-contract builder runs first, bridge-era limits can reject authored
trees the compiled plan could already execute directly.

So the execution gate should be:

```text
compiled tree
  -> ordered/direct executor when supported
  -> replay-contract bridge only when not yet supported
```

This is the shortest path from the transitional V2/V3 bridge to a truly
independent V3 pathway.

### 30.5 Native-path milestone after bridge-call retirement

When the compiled execution path reaches **zero**
`render_pipeline_with_spec(...)` callsites under `src/v3/compile/`, that
should be treated as a concrete migration milestone.

At that point:

- the compiled path is no longer structurally dependent on the old replay
  helper
- native-coverage regressions should be made visible with focused
  classification / fixture guards
- the remaining roadmap should pivot toward the still-open semantic gaps:
  - hint-driven chaining
  - broader overlap/conflict semantics for overlapping branches
  - general arbitrary cross-family ordering

In other words:

> after the last compiled-path replay callsite is gone, “finish V3” stops
> meaning “remove hidden bridges” and starts meaning “close the remaining
> semantic gaps.”

---

## 40 — First selector compaction targets

The initial compiled plan should focus on the selectors that are:

- common
- literal
- cheap
- obviously useful

Examples:

- literal `cells`
- literal `cell_run`
- literal `rows`
- literal `row_range`
- literal `columns`
- literal `column_range`

Everything else can initially remain:

- dynamic
- normalized-scope-backed

That is enough to prove the phase without over-generalizing too early.

To keep these opportunities available, normalization should preserve compact span selectors rather than eagerly exploding them into individual cells.

---

## 50 — Performance posture

The compiled execution plan is where the project should begin explicitly preparing for the 16.7ms frame-budget world, but still without pretending the whole runtime has been optimized.

What this phase should do:

- remove obviously repeated selector interpretation from later paths
- keep compilation off the per-frame hot path
- provide a tighter handoff for future family executors
- preserve room for later cache-friendly layouts

What this phase should **not** yet do:

- freeze the final payload typing strategy for every family
- prematurely inline every policy into one huge execution enum
- sacrifice authoring/debug clarity for speculative micro-optimizations

In short:

> normalize for correctness first, compile for execution second, optimize the true hot path third.

---

## 60 — First consumer seams that should mirror V2

The first consumer seams to extend in parallel with the V2 pathway are:

1. crate-root exports (`load`, `parse` → parallel `load_v3`, `parse_v3`, `load_v3_compiled`, …)
2. recipe-module exports
3. prelude exports
4. a shallow validation tool or example path

Deeper consumers like probe/viewer/runtime can follow after this shallow seam is established.

---

## 70 — Definition of done for this phase

This phase is in place when:

- a compiled plan type exists
- normalized IR can compile into it deterministically
- common literal scopes are compacted
- the new plan is exposed through the same shallow seam family that V2 uses
- at least one shallow tool/example consumes the V3 compiled path
- supported trees can execute directly before the replay-contract bridge is consulted

---

## 80 — Immediate execution companion

The execution companion documents for this phase are:

- `docs/design/tui-vfx-v3-compiled-execution-plan.md`
- `docs/design/tui-vfx-v3-first-slice-checklist.md`

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/62_compiled_execution_plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.1.0</VERS> -->
