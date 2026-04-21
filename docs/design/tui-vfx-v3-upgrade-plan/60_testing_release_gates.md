<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/60_testing_release_gates.md</FILE> - <DESC>Chapter 60 — testing and release gates: the six criteria V3 must satisfy before shipping (shadow / offscreen / probe / trace / GT-Design integration / role-aware lowering), validator-redesign scope, fixture and golden-artifact strategy, whitelist discipline and escalation paths. Promotes Concern F's resolution + Open Q #9 from open-question status to first-class release-gate chapter.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>New chapter synthesizing Open Q #12 resolution (Concern F, v0.14.0 of monolith) + Open Q #9 (validator redesign). These were release-blocking but embedded inside the Open Questions list in the monolith; surfacing them as a standalone chapter makes the "what does green look like" specification visible alongside the migration workflow (Chapter 50).</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Lifts Open Q #12 / Concern F release-gate criteria + Open Q #9 validator redesign scope verbatim. Adds an ANSI gate-dependency diagram showing how the six criteria gate on the infrastructure from Chapter 50 and Chapter 100.</CLOG> -->

# 60 — Testing and release gates

V3 does not ship until each of six release-gate criteria is green for the designated critical set. This chapter specifies what "green" means, how each gate is evaluated, and how whitelist/escalation discipline works. It pairs with Chapter 50 (which describes how the migration work gets done) — this chapter describes what counts as done.

## 10 — Gate dependency model

```
     Chapter 100                Chapter 50
  (tooling cutover)         (migration workflow)
          │                         │
          └──────────┬──────────────┘
                     │
                     ▼
          ┌───────────────────────┐
          │  Critical set         │
          │  (~5-15 recipes)      │
          │  ↓                    │
          │  Fixture-equivalence  │
          │  gate runs through:   │
          └──────┬────────────────┘
                 │
                 ▼
     ┌───────────────────────────────────────────┐
     │   SIX RELEASE-GATE CRITERIA               │
     │                                           │
     │   1. Canonical shadow fixtures            │
     │   2. Offscreen / slide fixtures           │
     │   3. Probe snapshots                      │
     │   4. Trace expectations                   │
     │   5. GT-Design integration fixtures       │
     │   6. Role-aware lowering correctness      │
     │                                           │
     │   Each criterion: green OR                │
     │   documented-and-whitelisted OR           │
     │   blocking                                │
     └───────────────┬───────────────────────────┘
                     │
                     ▼
             V3 release green-light
```

## 20 — The six release-gate criteria (Concern F resolution)

The V3 tree schema, the per-layer pipeline feature (Decision 5's implementation track), and the uniform step vocabulary (Decision 3) all touch infrastructure that downstream consumers — gt-design in particular — depend on for correctness at final render truth: shadow fidelity, offscreen composition behavior, role-aware lowering, trace/probe observability, factory-canonical final rendering. The prior draft flagged these as "a risk area, not a blocker." That framing was wrong — if V3 regresses these, the schema improvements don't pay back.

**These are V3 release-gate criteria, not open risk items.** V3 does not ship without green on each gate criterion for the designated-critical-set of consumers and recipes.

### 10 — Criterion 1: Canonical shadow fixtures

Every shipped shadow primitive (depth-based, elevation-based, glow/bloom, directional) has a captured pre-migration fixture from V2 rendering and a post-migration V3 render. Delta is either within tolerance, documented-and-whitelisted as intended V3 behavior, or blocking.

### 20 — Criterion 2: Offscreen / slide fixtures

Representative recipes that use offscreen composition (multi-pass rendering, buffered intermediate stages, slide-in/slide-out transitions) have captured fixtures covering pre-migration and post-migration render. Same tolerance + whitelist + blocking model.

### 30 — Criterion 3: Probe snapshots

The `vfx-probe-validation` corpus — by definition the recipes that exist to pin rendering behavior — passes probe-equivalence against pre-migration captures. Any probe diff is either a translator bug, an intended V3-only behavior change (documented), or blocking.

### 40 — Criterion 4: Trace expectations

The trace/probe infrastructure emits events at the same granularity and with the same semantic content as V2 for representative flows. Schema additions (e.g., `TraceEvent::LayerPipelineApplied` from Decision 5) are allowed; removals or semantic shifts are either documented or blocking.

### 50 — Criterion 5: GT-Design integration fixtures

Representative gt-design surfaces (splash family, default recipe set, toast family, modal family, any recipe that app-level docs or release notes cite as a specific visual contract) render identically within tolerance against pre-migration captures. Failures route through Chapter 50's critical-set carve-out's fixture-equivalence gate.

### 60 — Criterion 6: Role-aware lowering correctness

The canonical builder (Decision 8) produces playback items whose role-aware handling (`RoleTag` at the render layer, not the new `RoutingRole` / `SurfaceIntent` hint types from Open Q #18) matches V2 for the fixture set. Documented whitelist is allowed only where V3's role-domain split (Concern C) deliberately changes behavior.

## 30 — Relationship with Chapter 50 (migration workflow)

Chapter 50's critical-set fixture track is the **mechanism** by which gate criteria 1–5 are evaluated. The critical set includes the recipes named above plus any specific recipe a consumer designates as rendering-load-bearing. The fixture-equivalence gate runs as part of V3 CI and blocks release on failure.

Chapter 50 handles *how* fixtures are captured, translated, and diffed; this chapter handles *what* must be captured and *what counts as passing*. The two are complementary: Chapter 50 is infrastructure and workflow, Chapter 60 is the specific gate criteria the infrastructure evaluates.

## 40 — Validator redesign (Open Q #9)

Tree schemas need different validation than flat schemas. New rules to design:

- **Scope-coherence** (no nonsensical scope combinations — e.g., `GlyphMatches` on a mask-reveal operation where "glyph" doesn't yet exist).
- **Container-shape invariants** (no `Parallel` with conflicting masks; no `Sequence` with zero steps).
- **Scope-propagation conflict detection** (child declares scope X, parent propagates Y; precedence rule?).
- **Hint-namespace membership** (`HintRef<T>` producer verification — Decision 7).
- **Fragment addressability** (Principle 4 / Open Q #17 — consolidated fragments must be individually addressable).
- **Token-and-binding contract validation** (`requires_substitutions` + `requires_bindings` — Open Q #14).
- **Migration validation**: V2 → V3 auto-migration for the critical set must preserve probe-equivalence (criterion 3 above).

This is **core V3 work, not support work** (per reviewer input on Open Q #9). The validator scope includes: scope coherence, tree/container invariants, hint ambiguity, fragment addressability, token/binding contracts, migration equivalence for critical fixtures.

Additionally, the validator should validate a **canonical normalized IR**, not only raw authoring syntax — that keeps the validator durable across future schema evolution. This pairs with Open Q #10's viewer lean: the viewer is also built on the normalized IR. Validator and viewer share the IR surface.

## 50 — Implementation-level sub-questions (remain open, resolve during implementation)

These are what Open Q #12 still covers, post-release-gate commitment:

### 10 — Per-criterion tolerance specification

Pixel-perfect, percentage-delta, perceptual-delta, structural, or probe-event match? Each criterion likely has its own tolerance shape.

| Criterion | Default-lean tolerance |
|---|---|
| Probe-validation corpus (#3) | Exact (by definition) |
| Shadow / offscreen / GTD-integration fixtures (#1, #2, #5) | Perceptual-delta with per-recipe calibration |
| Trace events (#4) | Structural equivalence with documented additions allowed |
| Role-aware lowering (#6) | Semantic equivalence against `RoleTag` behavior; whitelist for intended Concern-C-driven changes |

### 20 — Gate ownership

Who designates the GT-Design representative surfaces? Who maintains the whitelist of documented intended changes? Probably gt-design's lead for GTD-integration surfaces and the V3 implementer for everything else, coordinated via explicit commit-and-PR workflow.

### 30 — Whitelist discipline

A whitelist entry documents *why* a V2→V3 behavior change is intended. Entries must be legible to a downstream maintainer auditing gate output six months later — rationale, affected recipes, expected before/after behavior. **Whitelist-as-commit-message is insufficient; whitelist-as-structured-manifest is the bar.**

### 40 — Recapture cadence

If a V2 fixture is stale (the V2 rendering itself changed since capture), does the gate recapture automatically or require explicit human approval? Lean: **explicit approval**, because silent recapture masks regression.

### 50 — Gate-fail escalation

When a gate criterion fails during V3 implementation, what's the escalation path? Lean: block the V3 release milestone; route diagnosis to the owning track (compositor, trace, shadow crate); resolve-or-whitelist-or-defer-critical-set-membership explicitly before resuming.

## 60 — Reviewer input

**Reviewer's opinion on Concern F** (2026-04-21 GT-Design lead review memo — input behind this chapter's existence): release gate, not optional polish. One input among several; the implementation-level sub-questions above (tolerance spec, gate ownership, whitelist discipline, recapture cadence, escalation path) remain open.

**Reviewer's opinion on Open Q #9** (validator scope): core V3 work, not support work. Validate canonical normalized IR in addition to raw authoring syntax — makes the validator durable across future schema evolution.

## 70 — Summary

```
   ┌─────────────────────────────────────────────────────────┐
   │  V3 release checklist (testing slice)                   │
   │                                                         │
   │  □ Canonical shadow fixtures — green                    │
   │  □ Offscreen / slide fixtures — green                   │
   │  □ Probe snapshots — green                              │
   │  □ Trace expectations — green                           │
   │  □ GT-Design integration fixtures — green               │
   │  □ Role-aware lowering — green                          │
   │  □ Validator — V3 tree rules + IR checks all passing    │
   │  □ Whitelist manifest — structured, legible, up to date │
   │  □ Tolerance spec — finalized per criterion             │
   │                                                         │
   │  Each "green" means: passing, whitelisted-with-rationale│
   │  or explicitly documented as intended V3 behavior.      │
   │  "Blocking" is the only other valid state.              │
   └─────────────────────────────────────────────────────────┘
```

Pairs with Chapter 100's tooling checklist and Chapter 50's migration workflow. V3 ships green on all three.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/60_testing_release_gates.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
