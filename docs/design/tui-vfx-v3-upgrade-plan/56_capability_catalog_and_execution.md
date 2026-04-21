<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/56_capability_catalog_and_execution.md</FILE> - <DESC>Chapter 56 — capability-catalog phase. Defines how V3 moves from a stabilized schema to a concrete catalog of primitives, composed primitives, wrappers, hybrid templates, and policy variants, and establishes the execution tracker for working through families 4-6 at a time.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>This chapter follows 55_schema_to_implementation_roadmap.md. It turns the roadmap into an executable phase: build the capability catalog, classify families, define canonical payloads, and keep a running closure tracker until the leaf catalog is stable enough for implementation.</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Defines the capability-catalog phase, its deliverables, classification vocabulary, per-family review template, and the execution discipline of resolving 4-6 families at a time.</CLOG> -->

# 56 — Capability Catalog and Execution Phase

This chapter is the **next active phase** after schema hardening.

The question is no longer:

> What should the top-level tree look like?

The question is now:

> What are the actual leaf families that live underneath that tree, and how should they be classified so implementation does not bake the wrong abstraction into the runtime?

---

## 10 — Purpose

The capability catalog exists to prevent two implementation failures:

1. **Over-flattening**
   - every current family becomes a separate top-level implementation surface
2. **Over-collapsing**
   - unrelated families are forced into one substrate because the tree is elegant on paper

This phase is where we choose the correct middle path.

---

## 20 — Classification vocabulary

Every family should be classified into exactly one of the following roles:

### 1. Primitive
A true leaf capability with independent computational meaning.

### 2. Composed primitive
A family that is still a reusable leaf for authors, but is structurally composed from lower shared capabilities.

### 3. Wrapper / router
A family whose main purpose is to host, route, or reinterpret another capability family.

### 4. Hybrid template
A reusable composition pattern built from multiple substrates.

### 5. Policy variant
Not a new family. A policy/render/tuning variant inside a deeper renderer tree or substrate.

---

## 30 — Required output per family

For each family, capture:

- lane
  - mask / sampler / filter / shader / style_effect / content
- classification
- recommended canonical name
- canonical payload shape
- fields that are:
  - required
  - optional
  - runtime-bindable
  - derived
  - deprecated / removed
- what current V2/V3 names collapse into it
- whether the implementation target is:
  - ship-now
  - deferred
  - future generator family
- implementation notes
  - caching
  - hot-path risks
  - validator concerns
  - normalized-IR concerns

---

## 40 — Execution discipline

This phase should be executed in **small batches**.

Recommended cadence:
- resolve **4–6 families at a time**
- update the catalog
- update the tracker
- update schema/docs when a structural implication is discovered
- then move to the next batch

Do **not** wait until the whole catalog is done before writing anything down.

---

## 50 — Primary evidence sources

Each family decision should be based on all of:

1. recipe corpus evidence
2. local audit findings
3. Claude memo findings
4. actual implementation/source code
5. large non-debug examples when relevant

The implementation code is especially important because it reveals whether two author-facing names are actually one substrate or two.

---

## 60 — First execution target order

Recommended order:

1. reveal / visibility families
2. displacement / resampling families
3. filter-side families
4. style-side families
5. content-side renderer trees
6. hybrid/wrapper categories that cut across lanes

This is the same ordering recommended in Chapter 55, but now expressed as the active work queue.

---

## 70 — Definition of done for this phase

This phase is done only when:

- every major family in the current corpus has a catalog entry
- ambiguous same-name families across lanes are resolved
- wrapper/router cases are explicitly identified
- hybrid templates are explicitly identified
- policy variants are not masquerading as top-level families
- the schema and overview docs reflect any structural implications found during cataloging
- the resulting leaf catalog is stable enough to implement against

---

## 80 — Active execution companion

The execution companion for this phase is:

- `docs/design/tui-vfx-v3-capability-catalog.md`

That file is the live catalog and tracker.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/56_capability_catalog_and_execution.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
