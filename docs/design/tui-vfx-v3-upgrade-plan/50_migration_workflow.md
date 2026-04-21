<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/50_migration_workflow.md</FILE> - <DESC>Chapter 50 — migration workflow: the three-phase Curate → Re-author → Validate model for the mainline corpus, plus the critical/fixture carve-out track for recipes where rendering equivalence is load-bearing. Promotes Concern B's resolution from Open-Question-embedded to first-class chapter status.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>New chapter synthesizing Open Q #2 resolution (Concern B, v0.10.0 of monolith) + the Deferred-section recipe migration workflow + Chapter 100's tooling-sequencing constraint. In the monolith, migration workflow was scattered across Open Q #2 and the Deferred section; promoting it to its own chapter makes release-blocking sequencing visible up-front alongside testing/release gates (Chapter 60).</WCTX> -->
<!-- <CLOG>1.0.0: initial chapter. Lifts Open Q #2 / Concern B content verbatim. Adds an ANSI phase-diagram showing Curate → Re-author → Validate with critical-set track parallel. Cross-references tooling sequencing (Chapter 100) and release gates (Chapter 60).</CLOG> -->

# 50 — Migration workflow

V3 is a clean cutover (see `30_why_now.md`), but "clean cutover" doesn't mean "rush the corpus through a mechanical translator." The migration is a three-phase human-directed workflow for the mainline corpus plus a narrow mechanical-translator track for a designated critical set where rendering equivalence is load-bearing. This chapter names the workflow, sequences it against tooling readiness (Chapter 100) and release gates (Chapter 60), and flags the sub-questions that settle during implementation.

## 10 — Two parallel tracks

```
                ┌──────────────────────────────────────┐
                │   V3 validator infrastructure        │
                │   (Open Q #9) — PREREQUISITE         │
                │   Mechanical translator spec for     │
                │   critical set — PREREQUISITE        │
                └──────────────┬───────────────────────┘
                               │
                ┌──────────────┴───────────────┐
                ▼                              ▼
   ┌───────────────────────────┐   ┌─────────────────────────┐
   │  MAINLINE CORPUS          │   │  CRITICAL / FIXTURE     │
   │  (large majority)         │   │  CARVE-OUT              │
   │                           │   │  (~5-15 recipes)        │
   │  ┌────────────────────┐   │   │                         │
   │  │ 1. Curate          │   │   │  0. Capture V2          │
   │  │    (human Morris   │   │   │     fixtures first      │
   │  │     filter)        │   │   │                         │
   │  └─────────┬──────────┘   │   │  1. Mechanical          │
   │            │              │   │     translator          │
   │  ┌─────────▼──────────┐   │   │     (deterministic)     │
   │  │ 2. Re-author       │   │   │                         │
   │  │    (Claude + human │   │   │  2. Fixture-equivalence │
   │  │     review;        │   │   │     gate (per-kind      │
   │  │     briefing-as-   │   │   │     tolerance)          │
   │  │     forcing-func)  │   │   │                         │
   │  └─────────┬──────────┘   │   │  3. Curatorial review   │
   │            │              │   │     still applies       │
   │  ┌─────────▼──────────┐   │   │     (Morris, naming,    │
   │  │ 3. Validate        │   │   │     metadata) but       │
   │  │    (schema shape + │   │   │     rendering is pinned │
   │  │     well-formed;   │   │   │     by the gate.        │
   │  │     semantic drift │   │   │                         │
   │  │     is allowed)    │   │   │                         │
   │  └────────────────────┘   │   │                         │
   └───────────────────────────┘   └─────────────────────────┘
```

**Phase ordering is deliberate.** Curation runs first because it reduces the problem space before any translation labor is spent on it; AI re-authoring instruments the briefing infrastructure the library will rely on going forward; validator checks are the last gate rather than an intermediate artifact.

## 20 — Prerequisites (not migration phases themselves)

The V3 validator infrastructure (Open Q #9) is built before migration phase 3 runs. The narrow-scope mechanical translator used only in the critical-recipe carve-out is also built as part of V3 implementation work, not as migration tooling. Chapter 100 enumerates the tooling that must be V3-ready before migration phase work can commence; notably the tooling cutover sequences with this workflow as follows:

```
    Tooling cutover             Migration workflow
    ───────────────             ──────────────────

    config.rs V3 types     ──▶  (prerequisite)
    tui-vfx-style restruct ──▶  (prerequisite)
    pipeline-validator V3  ──▶  Phase 3 (validate)
                                 + critical-set gate (Ch 60)
    recipe-probe V3        ──▶  Phase 3 (validate)
    trace taxonomy         ──▶  Phase 3 (validate)
    xtask docs V3          ──▶  Phase 2 (re-author briefings
                                         reference current docs)
    Authoring guides V3    ──▶  Phase 2 (briefing forcing function)
    demo binary V3         ──▶  Phase 2 (human review uses demo)
```

## 30 — Mainline corpus: three-phase workflow

**The large majority of retained recipes** flow through the three-phase Curate → Re-author → Validate workflow.

### 10 — Phase 1: Curate (human Morris filter)

For each V2 recipe, decide: **port / consolidate / archive / delete.** This phase collapses the problem space — recipes that don't earn a V3 slot are not translated. Output: the retained set with per-recipe disposition and rationale. Workflow B in the sibling audit-workflow doc is this phase.

Running curation first matters because:
- It prevents pouring translation labor into recipes that will be archived or deleted.
- It keeps the curation conversation focused on "does this earn its place?" rather than "did the translation succeed?"
- Morris-filter decisions surface which recipes genuinely carry design judgment (worth earned-names per Decision 2) vs which are just mechanical compositions.

### 20 — Phase 2: Re-author (Claude + human review)

Claude translates each retained recipe from V2 intent to V3 form under explicit authoring briefing. **This is not a mechanical reshape; it is a capability test for V3's AI-authoring pathway** (the primary composition mode per Decision 3's rationale).

Where Claude struggles, the briefing infrastructure (SKILLS.md, prompt scaffolds, on-disk vocabulary references, authoring guides) has a gap — which is exactly the gap every future author (human or AI) will hit when writing a new V3 recipe from scratch. Briefing improvements land alongside the recipes that surface them; the migration is deliberately used as a forcing function for briefing quality.

Claude's output is reviewed by the human in the loop before commit.

Re-authoring is also where **latent V3 primitive gaps surface**: a V2 recipe that can't be cleanly re-authored in V3 is evidence that V3 is missing a primitive, a Pattern variant, a hint kind, or a binding mechanism — those gaps route back to V3 implementation work, not to ad-hoc recipe workarounds.

### 30 — Phase 3: Validate (V3 validator, well-formedness-first)

V3 validator runs on every re-authored recipe. Validator covers:
- Schema shape
- Scope coherence
- Hint-namespace membership (`HintRef<T>` producer verification)
- Fragment addressability
- Binding-contract discovery
- Required-field presence

Validator failures block merge for the affected recipe. **Semantic drift from V2 to V3 is often intended at this stage** — re-authoring is allowed to improve on the V2 version, that's part of the point — so the mainline validator checks well-formedness rather than rendering equivalence. The critical-set carve-out below handles the rendering-equivalence question.

## 40 — Critical / fixture carve-out (small designated set — expected ~5–15 recipes)

For recipes where downstream consumers or test infrastructure depend on specific rendering behavior, AI re-authoring alone is insufficient: "similar but subtly different" V3 output is a silent visual regression for apps upgrading to V3, and a correctness break for probe tests whose purpose is to pin rendering. The designated set routes through a parallel track:

1. **Capture V2-rendered fixtures before any migration work begins.** Cheap insurance; run once against current V2 corpus. Checked in.
2. **Mechanical translator produces V3 for the designated set only.** Deterministic transformations: tree reshaping, Ra→Vfx rename, scope/phase wrapping, ParamValue/HintRef/StepInput handling, default population. Does not attempt curation.
3. **Fixture-equivalence gate.** V3 render must match V2 fixture within tolerance (tolerance is per-recipe-kind — see Chapter 60 for the gate criteria). Drift is either a translator bug (fix) or intended V3-only behavior (document and whitelist).
4. Curatorial review still applies to the critical set — Morris filter, naming, metadata — but rendering is preserved by the gate.

**Candidate critical-set members:**
- The probe-validation corpus (by definition — these recipes exist to pin rendering)
- The splash family (gt-design ships specific splash visuals apps depend on)
- Any recipe that app-level docs or release notes currently cite as a specific visual contract

Membership is designated explicitly per-recipe with written justification; inclusion is not the default.

## 50 — Relationship with Chapter 60 (release gates)

Chapter 60 defines *what must be green* for V3 to ship. This chapter defines *how the work gets done* to reach those green gates. The relationship:

- **Mainline corpus (this chapter's three-phase workflow)** is evaluated by validator checks — schema well-formedness, scope coherence, hint-namespace membership. Chapter 60's gate criteria 1–4 (shadow/offscreen/probe/trace) do NOT apply to mainline recipes; they apply only to the critical set.
- **Critical-set carve-out (this chapter's parallel track)** is evaluated by Chapter 60's six release-gate criteria. The fixture-equivalence gate is the mechanism; Chapter 60's criteria define what the fixtures must capture and what counts as passing.

Chapter 60 is the *spec*; this chapter is the *process*.

## 60 — Sub-questions that settle during implementation

These are implementation-mechanics that don't block adopting the workflow:

- **Authoritative inventory step (blocking prerequisite for Phase 1).** Before curation begins, an inventory pass produces a checked-in manifest of all recipe files with classification (candidate / debug / probe / test / deprecated / generated). Current filesystem evidence suggests the main corpus is >500 files, well above prior "200–300" estimates. Inventory gates all scope and schedule assumptions.
- **Critical-set membership discipline.** Who designates inclusion; how each member's inclusion is justified per-recipe; whether the set is frozen at migration start or can grow during migration as additional rendering contracts surface.
- **Fixture-tolerance specification.** Pixel-perfect vs percentage-delta vs structural-equivalence vs probe-event match. Probably a mix by recipe kind; concrete specification in Chapter 60.
- **Briefing-improvement commit discipline.** When re-authoring surfaces a briefing gap, does the fix land as a separate commit before the recipe, alongside it, or in a batch at phase-end? Probably alongside, with the motivating recipe cited in the commit message.

## 70 — Addresses Concern B of the 2026-04-21 GT-Design lead review memo

The prior draft split-brained between Open Q #2 ("script with human exceptions") and the Recipe migration workflow section in Deferred ("manual curation, bulk translation is not the goal"). Both framings had partial truth; the three-phase model unifies them by sequencing curation first (problem reduction via Morris filter), AI authoring second (instrumenting the future-author pathway per Decision 3's stated primary composition mode), and validation third (well-formedness via built validator), with a fixture track for the subset where rendering equivalence is load-bearing. The reviewer's drift-audit concern is addressed at two levels: validator + curatorial review for the mainline, fixture equivalence for the critical set.

Reviewer's opinion (2026-04-21 GT-Design lead review memo — input behind Concern B's resolution): hybrid migration (mechanical translation + human review + fixture/probe equivalence for critical recipes), explicitly NOT purely manual Claude-led rewrite of the whole corpus. This was load-bearing for the three-phase model captured above.

## 80 — Future versioning after V3 ships

Future versioning (V4 migration, if V3 attracts external consumers) is a separate concern. V4 migration will need the compatibility discipline that V3 does not need to carry — shim windows, deprecation warnings, dual-path loaders, validator-enforced migration paths. That's a future-plan concern, not a V3 concern. See `30_why_now.md` for the clean-break-license scoping.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/50_migration_workflow.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
