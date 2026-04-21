<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/61_first_implementation_slice.md</FILE> - <DESC>Chapter 61 — first implementation slice. Defines the smallest credible code slice that should be built first: authoring parse types, normalized IR skeleton, region-ref resolution, style normalization, hint validation, and IR dump/debug output.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Follows the validator/canonicalization phase. This chapter turns the planning stack into a concrete first code work package so the project can begin implementation without jumping directly into broad family execution.</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Establishes the first implementation package and its boundaries, deliverables, and explicit non-goals.</CLOG> -->

# 61 — First Implementation Slice

This chapter answers the practical question:

> If implementation starts now, what is the smallest credible code slice that should be built first?

The answer is:

- **not** a loader for every family
- **not** broad runtime execution
- **not** mass recipe migration

The first slice should be the **authoring-types + normalized-IR spine**.

---

## 10 — Scope of the first slice

The first implementation slice should include exactly these core pieces:

1. authoring-layer parse types for the new schema
2. normalized IR type definitions
3. region-ref / region-compression resolution
4. style normalization
5. hint producer/consumer validation
6. canonical IR dump / debug surface

That is enough to make the schema executable as a structural contract without yet having broad effect-family execution.

---

## 20 — Why this slice first

This slice is the highest-leverage first code because it gives all later runtime work a stable internal target.

Without it:
- every family implementation embeds its own canonicalization assumptions
- validator and runtime drift
- migration tools guess at policy
- debugging becomes much harder

With it:
- all later work lands on one shared execution-facing representation
- structural bugs fail early
- migration policy stays explicit
- runtime family work can be incremental without changing the contract every time

---

## 30 — Concrete deliverables

### 30.1 Authoring-layer parse types

Implement parse types for:

- recipe envelope
- contracts (`requires_*`)
- `config`
- `layout`
- `lifecycle`
- `border`
- `clock`
- `base_style`
- `regions`
- `scene.layers`
- `pipeline.timing`
- `step`
- `scope`
- `interaction`

This is the schema-facing layer.

### 30.2 Normalized IR types

Implement normalized types for:

- `NormalizedRecipe`
- `NormalizedLayer`
- `NormalizedPipeline`
- `NormalizedStep`
- canonical region form
- canonical contract block

This is the execution-facing layer.

### 30.3 Canonicalization passes

Implement the minimum first passes:

- defaults made explicit
- region refs resolved
- `cell_run` / `cell_runs` lowered to canonical region form
- style forms normalized
- wrapper lowering where required for execution
- placement normalized

### 30.4 Validation

Implement first validator passes for:

- scope coherence
- hint coherence
- style normalization coherence
- scene coherence
- contract coherence

### 30.5 Debug / inspection output

Produce at least one canonical dump/debug surface:

- normalized IR dump

This makes the new architecture inspectable immediately.

---

## 40 — Explicit non-goals of the first slice

The first implementation slice should **not** include:

- broad family runtime support
- fireworks / celebratory particle generators
- advanced region abstractions beyond the first compression layer
- full viewer implementation
- mass V2 recipe migration automation
- production-grade performance optimization across all families

Those come later.

---

## 50 — First families that should benefit from the slice

The first slice should make it possible to safely implement, shortly after, families like:

- reveal geometry masks
- wave samplers
- simple filters
- style fades
- region-heavy style examples
- hint-bound scene examples

It should not try to directly solve split-flap, typewriter+cursor, or large wrapper families in the first code step.

---

## 60 — Success criteria

This slice is complete when:

- the new schema parses into authoring types
- normalized IR can be produced deterministically
- region refs and style normalization are no longer ad hoc
- hint validation exists in at least first-pass form
- a normalized IR dump/debug artifact exists
- later runtime family work can target normalized IR rather than raw authoring syntax

---

## 70 — Immediate execution companion

The live execution companion for this first code slice is:

- `docs/design/tui-vfx-v3-first-slice-checklist.md`

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/61_first_implementation_slice.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
