<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/55_schema_to_implementation_roadmap.md</FILE> - <DESC>Chapter 55 — top-down roadmap from the now-hardened V3 schema to actual implementation work. Defines the next major phases after schema stabilization: capability cataloging, lowering rules, normalized IR, validator/canonicalization, family-by-family runtime support, and explicit defer decisions.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Written after the dual-auditor schema hardening pass closed the core structural questions. This chapter answers the practical question: what do we do next, top-down, to get from a robust schema to real migration/implementation work without building the wrong leaf catalog under the right tree?</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Establishes the post-schema roadmap: freeze structural schema, define the capability catalog, define V2→V3 lowering rules, implement authoring types + normalized IR, implement validator/canonicalization, then migrate family-by-family. Explicitly distinguishes must-not-defer work from acceptable follow-ons.</CLOG> -->

# 55 — Schema-to-Implementation Roadmap

This chapter begins **after** the core structural schema has stabilized enough to build against.

It exists to answer a specific question:

> What is the top-down path from a robust V3 schema to real implementation work without collapsing into ad hoc migration or building the wrong primitive/composition catalog underneath the right tree?

The short answer is:

1. **Do not start with the loader.**
2. **Do not start by bulk-migrating files.**
3. **Define the capability catalog and lowering rules first.**
4. **Build the normalized IR early.**
5. **Implement family-by-family, not directory-by-directory.**

---

## 10 — Where we are now

The schema hardening pass answered the core top-down structural questions.

We now have a stable-enough direction for:

- the recipe envelope
- scene layers
- pipeline tree structure
- scope model
- timing home
- interaction home
- style normalization direction
- wrapper/router and hybrid-template posture
- region compression first layer
- build-target documentation for the schema itself

What remains is **not** primarily a tree-shape problem anymore.

What remains is the bridge from:

- **schema structure**
- to **capability catalog**
- to **lowering rules**
- to **runtime implementation**

That bridge is the purpose of this chapter.

---

## 20 — The key framing

The schema now tells us:

- what a recipe looks like
- what a scene looks like
- what a pipeline tree looks like
- where scope / phase / timing / interaction / theme / shadow live

But it still does **not** automatically tell us:

- which leaf families are true primitives
- which are composed primitives
- which are wrappers
- which are hybrid templates
- which are policy variants on deeper renderer trees

That is the current hinge point.

The immediate next step is **not** to treat the schema as self-sufficient.
The immediate next step is to define the **capability catalog that lives underneath it**.

---

## 30 — Phase 1: freeze the structural schema

This phase is effectively complete before this chapter starts.

Goal:
- Stop casually changing the top-level tree.
- Treat the schema as the current contract for implementation planning.

Outputs:
- `docs/design/tui-vfx-v3-schema-draft.json`
- `docs/design/tui-vfx-v3-schema-overview.md`
- schema issue/resolution tracker

Rules:
1. Structural changes after this point should be deliberate and expensive.
2. New findings should first try to fit inside the established tree before forcing tree changes.
3. Remaining uncertainty should be framed as **extension backlog**, not as baseline instability.

---

## 40 — Phase 2: define the capability catalog

This is the immediate next major work item.

The capability catalog should answer, for every family in the current corpus:

- which operational leaf kind it belongs to
  - `mask`
  - `sampler`
  - `filter`
  - `shader`
  - `style_effect`
  - content renderer subtree
- whether it is:
  - **primitive**
  - **composed primitive**
  - **wrapper/router**
  - **hybrid template**
  - **policy variant**
- what its canonical payload shape is
- what existing V2 families collapse into it
- whether it is:
  - ship-now
  - deferred
  - future generator family

This is the real bridge between schema and code.

### Why this must happen before implementation

Without this step, implementation will drift into one of two failures:

1. **Over-flattening** — every currently named family becomes its own top-level implementation surface.
2. **Over-collapsing** — unrelated families are forced into one substrate because the tree is elegant on paper.

The capability catalog is where we distinguish those two errors.

### Concrete expected outputs

At minimum, produce a document or table covering:

- reveal / visibility families
- displacement / resampling families
- filter-side families
- style-side families
- content-side renderer trees
- hybrid templates
- wrapper/router families
- future generator families

---

## 50 — Phase 3: define canonical lowering rules from V2 to V3

Once the capability catalog exists, the next step is to define the lowering rules explicitly.

For each V2 concept, define:

- its V3 structural home
- its primitive/composition target
- normalization rules
- migration heuristics
- cases that require human classification rather than automatic lowering

Examples:

- `apply_to` → `scope`
- `style` / `styles[]` → style tree normal form
- `spatial_shader` → style-native wrapper vs sibling shader
- `text_pool` / `preset_pool` → authoring layer above concrete tree
- `interaction_states` + `interaction_config` → step-level interaction metadata

### Why this phase matters

Without an explicit lowering spec, every loader implementation and every migration script will silently embed policy.

That produces:
- inconsistent lowering
- validator drift
- hard-to-review migration behavior
- irreproducible re-authoring decisions

The lowering rules turn migration behavior into an explicit contract.

---

## 60 — Phase 4: implement authoring types and normalized IR together

Do **not** implement the raw authoring types first and normalized IR later.

Implement them together.

Why:
- the authoring surface is intentionally richer than the eventual execution core
- region refs, wrappers, hybrid templates, style normal forms, and scene layering all benefit from canonicalization
- validator/viewer/tooling want one stable execution-facing representation

### The recommended implementation sequence

1. **Authoring-layer types**
   - recipe envelope
   - scope
   - timing
   - pipeline tree
   - scene layers
   - interaction
   - regions
   - style patch shapes

2. **Canonical normalized IR**
   - flatten sugar
   - resolve region refs
   - normalize style forms
   - lower wrappers where appropriate
   - make defaults explicit

3. **Lowering adapters**
   - raw V3 authoring → normalized IR
   - V2 recipe → normalized IR
   - optional V2 → authored-V3 for migration tooling / docs

### Why normalized IR is not optional

The authoring schema intentionally allows multiple ergonomic representations for related concepts.
That is good for authors.
It is bad as a direct runtime/tooling surface.

So the runtime/tooling contract should be the normalized IR, not the raw authoring shape.

---

## 70 — Phase 5: implement validator and canonicalization before broad runtime migration

Before implementing most families, implement:

- schema validation
- normalized-IR validation
- lowering invariants
- conflict detection
- contract discovery

This includes checks for:

- scope coherence
- phase coherence
- region reference validity
- wrapper normalization validity
- scene-layer placement validity
- hint wiring validity
- interaction metadata validity

### Why validator-first matters

If runtime support comes first, schema errors become rendering bugs.
If validation/canonicalization comes first, schema errors stay load-time issues.

That difference matters enormously during migration.

---

## 80 — Phase 6: implement by capability family, not by file tree

Do not implement "all recipes."

Implement **capability clusters** in the order that reduces risk and ambiguity.

### Recommended order

#### 1. Structural basics
- scope
- phase
- timing
- style normalization
- region refs / compression helpers

#### 2. Easy leaf families
- simple masks
- simple samplers
- simple filters
- simple style effects

#### 3. Grouped subtrees
- sweep families
- indicator/progress families
- reveal geometry families
- procedural breakup families
- displacement subtrees

#### 4. Deep renderer trees
- split-flap
- typewriter + cursor

#### 5. Hybrid/wrapper families
- subcell-light
- style-native spatial wrappers
- hybrid transition templates
- rule-engine families like `glyph_style`

This sequence lets the hard structural questions settle before the deepest runtime work lands.

---

## 90 — Phase 7: define what is explicitly deferred

Before implementation accelerates, explicitly separate:

### okay to defer
- celebratory particles / fireworks
- richer region derivation beyond `cell_run` / `cell_runs`
- advanced family/template tooling
- additional generator registries
- some high-complexity showcase-only scene generators

### not okay to defer
- normalized IR
- scope
- style normalization
- motion-path home
- interaction home
- scene-layer semantics
- capability catalog for the current V2 corpus
- lowering rules for the current V2 corpus

The point is to prevent ambiguity about what is "later" versus what is actually blocking a credible V3 core.

---

## 100 — The practical next actions

If we translate this chapter into immediate work, the next actions are:

1. **Write the V3 capability catalog**
   - one row per family
   - primitive vs composed primitive vs wrapper vs hybrid template vs policy variant
   - target leaf kind
   - canonical payload
   - V2 source families

2. **Write the lowering rules doc**
   - V2 surface → V3 structural home
   - normalization rules
   - migration heuristics
   - human-review-required cases

3. **Implement authoring types + normalized IR together**
   - not separately

4. **Implement validator/canonicalizer before broad family migration**

5. **Then begin capability-family implementation in the recommended order**

---

## 110 — Main warning

The biggest current risk is no longer “the tree is wrong.”

The biggest current risk is:

> building the wrong leaf catalog under the right tree.

If we skip the capability-catalog and lowering-rules phases, that is exactly what will happen.

So the top-down answer is simple:

- the schema phase is mostly done
- the **capability catalog phase** is next
- the **lowering rules phase** comes immediately after
- implementation starts only after those are explicit enough to constrain it

That is how we get from here to real code without losing the structural gains of V3.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/55_schema_to_implementation_roadmap.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
