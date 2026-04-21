<!-- <FILE>docs/design/tui-vfx-v3-migration-findings-memo-claude.md</FILE> - <DESC>Detailed memo of findings from the V2→V3 debug-recipes migration exercise. Recommendations for schema additions/tweaks/changes plus new concepts worth considering. Intended as one of two parallel memos; the other is authored by a different reviewer doing the same migration work. The two will be compared and synthesized into a refined V3 schema draft.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Synthesize 258 V3 conceptual-recipe-migration findings (6 stages across debug_recipes + madeira-flag + wargames + extended barber_pole) into recommendations for the schema draft plus new concepts plus plan-level question promotions. Companion memo by a different reviewer is forthcoming.</WCTX> -->
<!-- <CLOG>1.0.0: initial memo. Headline assessment (all refinement-level, no approach blockers), validated decisions, 10 schema draft refinements, 4 new concepts for discussion, 4 plan-level open questions to promote (Q#24-27), watch items, deferrables.</CLOG> -->

# V3 Migration Findings Memo — Claude

**Author:** Claude (one of two parallel reviewers)
**Scope:** Conceptual migration of 258 V2 recipes to draft V3 shape across 6 stages
**Date:** 2026-04-21
**Source documents:** `tui-vfx-v3-upgrade-debug-recipes-migration-log.md` (34-question schema journal, 7-item drift table, final V2↔V3 coverage audit), `tui-vfx-v3-schema-draft.json` (specification-by-example)

---

## 10 — Headline assessment

**The proposed V3 approach is sound and ready to carry into implementation.** The migration pressure-tested every structural decision in the plan across a broad shape surface (single-capability shaders, phase-differentiated compositions, multi-scope scene compositions via complex/ recipes, scene-layer scene-graph via madeira-flag, template-inheritance hierarchies via 57 wargames recipes, and a sub-cell tri-color thought experiment via extended barber_pole) and produced **zero V3-inexpressible recipes**. Every recipe ported, modulo flagged schema questions that are additive refinements rather than contradictions.

The findings below are **all refinement-level**. Nothing in my migration experience calls for rethinking Decisions 1–8, the ParamValue/HintRef split, the scene-layer direction, or the canonical-seam commitment. Two MAJOR gaps (motion paths, SignalGraph JSON shape) need plan-level resolution before a loader ships, but both are feature-area additions compatible with the existing approach — not contradictions of it.

```
  258 V3 recipes · 6 stages · 0 V3-inexpressible cases surfaced
  ─────────────────────────────────────────────────────────────
     Stage 1: Tier-1 shader calibration         12 recipes
     Stage 2: Tier-2+3 shader catalog           42 recipes
     Stage 3: non-shader debug subdirs         145 recipes
     Stage 4: madeira-flag scene composition     1 recipe
     Stage 5: wargames template hierarchy       66 recipes (10 templates + 56 children)
     Stage 6: barber_pole sub-cell exercise      1 recipe (+ thought experiment)
                                               ────
                                                258

  Decision 1 (Scope)           ✓ held; needs variant expansion (Q23)
  Decision 2 (Pattern axis)    ✓ held; catalog grows but the axis works
  Decision 3 (Tree schema)     ✓ held; parallel/sequence containers natural
  Decision 4 (Ra→Vfx rename)   ✓ held; no recipe-level visibility
  Decision 5 (Scene layers)    ✓ held; madeira-flag is the canonical demonstrator
  Decision 6 (ParamValue)      ✓ held; RuntimeBinding collapse is V3's clearest ergonomic win
  Decision 7 (HintRef)         ✓ held; sampler→shader wiring composed cleanly
  Decision 8 (Canonical seam)  ✓ held at the plan level (not exercised in recipes)
```

## 20 — What the migration validated (by decision)

### 10 — Decision 1 (Unified Scope) held, with growth needed

V2's scattered scoping mechanisms — `StyleRegion::BorderOnly`, `apply_to: "background"`, per-shader predicate fields, `row_mask.mode: "last_row"` inside highlighter — collapsed cleanly into `step.scope: Scope` across all 258 recipes. The migration surfaced seven additional Scope variants needed to cover the V2 vocabulary (`border`, `cell`, `cells`, `rows`, `row_range`, `columns`, `column_range`), already flagged as Q23 and documented in the schema draft.

**Notable validation:** `highlighter.row_mask: {mode: "last_row"}` is V2's attempt to encode scope inside a factory payload. V3's `scope: {kind: "rows", rows: [...]}` eats it cleanly. That's evidence the unified Scope primitive is the right abstraction, not a compromise.

### 20 — Decision 2 (Pattern-as-axis) held; generator-class primitives are a distinct tier

Of the 54 shader recipes migrated, 17 were classified as primitive form (`colored_overlay + pattern` or `gradient_overlay`), 37 as earned-name compositions. A further 10 Tier-3 shaders (sub_cell_shake, stochastic_sparkle, chromatic_edge, glitch_lines, neon_flicker, trace_path, trace_propagation, orbit, cursor, reveal_wipe) classified as "primitive itself" but with a distinct character: they are **not** `ColoredOverlay + Pattern` compositions. They're generator-class primitives — per-cell stochastic noise, positional authored-polyline rendering, or per-channel color operations that Pattern can't cleanly express.

**Finding:** Decision 2's three-way classification (trivial / earned / primitive-itself) works, but the plan's wording implies "primitive itself" means "new Pattern variant". In practice, 10 of 12 primitive-itself cases wanted a new **base primitive kind** parallel to `colored_overlay`/`gradient_overlay`, not a new Pattern variant. The schema draft already proposes this but the plan could be more explicit.

### 30 — Decision 3 (Tree schema) held; Parallel-with-per-step-phase is the natural shape

Complex compositions (10 recipes with multi-phase, multi-scope, multi-step pipelines) all landed as `pipeline.step = {kind: "parallel", children: [...]}` with per-child `phase` and `scope` tags. Sequence containers were never needed in the debug corpus; Parallel with phase-tagged children was sufficient for every multi-step case.

**Clear ergonomic win:** V2's `styles: [...]` array (per-scope styling with optional per-scope shaders) maps one-to-one to parallel StyleEffect + Shader steps sharing the same scope. The V2 asymmetry (`style` singular vs `styles` plural depending on recipe complexity) disappears.

### 40 — Decision 6 (ParamValue) is the clearest ergonomic win

V2's `field + field_binding` parallel-field pattern (`speed` + `speed_binding`, `direction` + `direction_binding`, `rect_x` + `rect_x_binding`) collapsed to V3's single `field: {"binding": "name", "default": T}` uniformly across 13 Stage 2 binding-variant recipes. The authoring-read improvement is visually obvious at any recipe that uses runtime bindings. The `default` sub-field preserves V2's offline-validator behavior cleanly.

### 50 — Decision 5 (scene layers) held for the madeira-flag exercise

The madeira-flag V3 recipe (Stage 4) exercises every part of Decision 5: four scene layers (backdrop, fireworks, flag, text_stack), per-layer pipelines (flag's sampler→shader via `displacement` hint), three source kinds (procedural, image, card), sibling-anchored placement. The structural shape holds up. The open gaps (Q30 procedural typed params, Q33 sibling-anchored placement) are about filling in the detail, not about whether the approach composes.

## 30 — Schema draft refinements (recommended additions)

The current schema draft (v0.1.0) covers 26 of 34 migration-log questions plus the 7 drift items. My recommendation is to add the following to bring it fully current:

### 10 — Base style modifiers

**Add:** `base_style.modifiers: ["bold", "italic", "underline", "reversed", ...]`

**Why:** V2 supports these via `added_modifiers` / `removed_modifiers` arrays on `RaBaseStyle`. The final audit flagged this as an unmapped minor gap. Closes a concrete V2 capability.

```jsonc
"base_style": {
  "foreground": { "type": "rgb", "r": 200, "g": 220, "b": 255 },
  "background": { "type": "rgb", "r": 10, "g": 15, "b": 30 },
  "modifiers": ["bold"]
}
```

### 20 — Motion paths + offscreen trajectories (depends on Q#22 resolution)

**Add:** Once Open Q #22 settles, extend `pipeline.timing`:

```jsonc
"pipeline": {
  "timing": {
    "enter_ms": 500, "exit_ms": 400,
    "enter_ease": "quad_out", "exit_ease": "quad_in",
    "enter_path": { "type": "arc", "rotations": 0.5 },
    "exit_path":  { "type": "linear" },
    "enter_from": { "type": "offscreen", "margin_cells": 0, "direction": "from_top" },
    "exit_to":    { "type": "offscreen", "margin_cells": 0, "direction": "from_top" },
    "enter_snap": { "type": "round" },
    "exit_snap":  { "type": "round" }
  }
}
```

Path types trial-deserialize against `tui-vfx-geometry::PathType` per Intention 38.

### 30 — Scope row_mask lift (Q15 resolution)

**Document in schema draft:** V3 `scope.kind = "rows"` (already proposed under Q23 extension) absorbs the concept that V2 encoded as `highlighter.row_mask`. The factory's `row_mask` field goes away; scope carries the same information at the step level.

**Why schema-visible:** this is not just factory cleanup — it reshapes the Scope catalog's mental model by demonstrating that factory-internal scope-encoding is evidence the Scope primitive is missing a variant.

### 40 — Symbolic vs coordinate tagged-union (Q26 resolution)

**Add to schema draft §5.7 Scope notes and to sampler payload conventions:**

```jsonc
// Scope cell coords and sampler center/origin both use this shape:
"center": { "kind": "center" }                    // symbolic
"center": { "kind": "cell", "x": 30, "y": 4 }    // coordinate-based
"center": { "kind": "binding", "name": "focus_xy", "default": { "x": 0, "y": 0 } }
```

### 50 — Sampler `phase` → `phase_offset` rename (Q29 resolution)

**Document in schema draft §5.9 factory-internal conventions (new subsection):** the sine_wave sampler's V2 `phase` payload field is renamed to `phase_offset` in V3 to avoid collision with the step-level `phase` (enter/dwell/exit/all). Not a user-facing concern but the migration demonstrates that V2 payloads can collide with V3's new uniform fields when terms are reused. Migration guidance: audit every factory payload field name against the step-level reserved words (kind, scope, phase, payload, children).

### 60 — V2 filter-list → V3 parallel Filter steps (Q28 documented pattern)

**Add to schema draft §5.6.2:**

```jsonc
// V2 pattern: pipeline.filter.dwell = [ {matrix_rain}, {subcell_light} ]
// V3 equivalent:
"step": {
  "kind": "parallel",
  "children": [
    { "kind": "filter", "phase": "dwell", "payload": { "type": "matrix_rain", ... } },
    { "kind": "filter", "phase": "dwell", "payload": { "type": "subcell_light", ... } }
  ]
}
```

The tree schema handles this natively. Documenting the pattern makes the migration path explicit for authors reading the schema.

### 70 — Open Q #23 timer gap

**Add to schema draft §6 (gaps) as a pending question:**

> **Timer primitive — Open Q #23.** Tachyonfx ships a per-effect `EffectTimer` with interpolation control. V3 currently distributes timing across `pipeline.timing` (recipe envelope), `mixed-signals` temporal basis (signal evaluation), and per-effect opt-in durations. Whether V3 should unify under a first-class Timer primitive (sibling of `ParamValue<T>` / `HintRef<T>` in `StepInput<T>`) or document the distributed model as intentional is pending plan-level resolution.

### 80 — Factory-internal conventions explicit section

**Add a new subsection §5.10 to the schema draft** documenting what V3 leaves opaque:

| Pattern | Example | V3 stance |
|---|---|---|
| Tagged-union shape discriminators inside factory payloads | `focus_field.shape: "rect" \| "ellipse"` gates `rect_x/y` vs `center_x/y` | Factory-internal. V3 schema doesn't surface the tag. Intention 39 merge semantics apply. |
| Dual-color shaders | `glisten_band.head` + `glisten_band.tail` | Factory-internal. Two `ParamValue<Color>` fields on the factory. |
| Accessibility hints in factory payloads | `highlighter.text_contrast: {mode: "preserve"}` | Factory-internal unless rule-of-three promotes to schema-surface (Open Q #27, see below). |
| Multi-element internal composition | `trace_path.paths[]` with per-path delays | Factory-internal. Factory owns the coordination. Split to parallel steps only if per-element params diverge. |

Explicit documentation here keeps the schema draft honest about what it does and doesn't encode.

### 90 — ParamValue-typed Scope coords (Q29 / style_cell_position_binding)

**Document in schema draft §5.7:**

```jsonc
// Cell coords in scope variants accept either raw u16 or ParamValue<u16>:
"scope": {
  "kind": "cell",
  "x": { "binding": "hovered_button_x", "default": 0 },
  "y": 3
}
```

Already implicit from Q29 migration; worth being explicit in the schema doc so authors know the runtime-binding mechanism extends into scope.

### 100 — Directory-convention guidance for primitives vs compositions

**Add a footnote to schema draft §5 or the plan's Chapter 50:** the migration established a convention of `recipes/<category>/primitives/` for primitive-form recipes and `recipes/<category>/compositions/` for earned-name composition recipes. This is authoring guidance, not schema-level, but worth recording as the working recommendation before it gets re-derived by each author.

## 40 — New concepts to consider introducing

Four concepts surfaced during the migration that aren't in the current plan but merit discussion.

### 10 — Canonical normalized IR

The reviewer-memo opinion on Open Q #9 (validator redesign) and Open Q #10 (viewer) independently converged on the same recommendation: validate and visualize a **canonical normalized IR**, not the raw authoring syntax.

My migration experience supports this. I encountered multiple places where the same concept had multiple authoring shapes — named-factory vs primitive form (Q#5), StyleEffect-wrapped-shader vs bare shader (Decision 5 style_spatial_effect cleanup), `config.base_style` vs head StyleEffect step for single-scope recipes (Q1 hybrid). Each of these ergonomic choices serves authoring but complicates validator, viewer, property-test, migration-equivalence reasoning.

**Proposal:** V3 defines a canonical normalized IR with a `canonicalize(raw_recipe) → normalized_recipe` transformation. Validator checks normalized IR. Viewer renders normalized IR. Migration equivalence tests compare normalized IRs. Tachyonfx's `to_dsl()` is a model.

This is close enough to being already-implied by Open Qs #9 and #10 that I'm promoting it to a distinct Open Q #24 in the plan (see §60 below).

### 20 — StepInput<T> fourth arm for recipe-derived values (maybe)

The current `StepInput<T> = ParamValue<T> | HintRef<T>` split covers external-value-source and pipeline-internal-step-output. A fourth category surfaced lightly during migration but didn't get resolved: **recipe-derived values** — values that depend on the recipe's own static declaration but can't be a Constant because they're derived at resolve-time.

Examples:
- `Pattern::RowDistance.selected_row_ratio` — a fraction of the recipe's `layout.height`
- scene-layer placement offsets that depend on sibling-layer bounds
- duration parameters that adapt to the containing Sequence's total budget

Today these are factory-internal (the factory reads layout.height to compute the pixel-space row). But with Sequence containers (if they grow beyond Parallel-only), some step-level parameters want to participate in per-Sequence scheduling.

**Proposal:** leave this latent for now; if a concrete use case in the mainline corpus forces it, consider `StepInput<T>::Derived { from: <deriver-ref> }`. Flagged as a watch item rather than a recommendation.

### 30 — Primitive catalog governance

Decision 2 names the three classification tiers (trivial / earned / primitive-itself) but doesn't specify:
- **When is a shader promoted to "primitive-itself" with a new base kind?** I made the call 10 times across Stage 2/3 using judgment. The criterion I used: "ColoredOverlay + Pattern can't express this — the spatial function is novel (stochastic, positional, per-channel, generator)." Worth writing down.
- **When does a Pattern variant earn its place?** The migration proposed RadialFromCorner, PerimeterHalo, EdgeShadow, RectField, EllipseField, LinearFromEdge, DiagonalStripes, PolarSweep, RowDistance, TravellingWave. That's 10 Pattern variants from ~12 shader families. Rule-of-three would say: wait until three recipes want the variant. But the debug corpus deliberately has one recipe per variant, so rule-of-three never fires at debug-corpus scale. The criterion needs adjustment.

**Proposal:** plan-level governance. Promoted to Open Q #25 below.

### 40 — Content-effect catalog parity

Decision 2 gives shaders a three-tier governance model (Tier 1 Rust factories, Tier 2 theme fragments, Tier 3 app fragments) and a primitive-by-default authoring surface. Content effects get none of this treatment in the plan — they exist as `content.effect: {type: "typewriter", ...}` with factory payloads and that's it.

But content effects have almost the same shape as shaders:
- ~16 named factories (typewriter, scramble, marquee, mirror, morph, dissolve, glitch_shift, scramble_glitch_shift, slide_shift, redact, numeric, odometer, split_flap, glyph_cascade, wrap_indicator)
- Some are trivial (redact: one symbol parameter) and might be primitive-class
- Some are clearly earned names (split_flap with its authenticity flags encoding physical-board design judgment)
- Some are compositions (scramble_glitch_shift explicitly bundles scramble + glitch_shift on a shared timeline)

**Proposal:** V3 applies Decision 2's primitive-vs-earned-name framing to content effects too. Not blocking V3 shape; worth a governance pass before the mainline corpus re-authoring phase.

Promoted to Open Q #26 below.

## 50 — Watch items / implementation risks

Things that aren't blocking but deserve eyes during implementation.

### 10 — SignalGraph JSON shape is the biggest schema-draft uncertainty

Q3 + Q31 surfaced a tentative shape (`{"signal": {"kind": "sine", "clock_ref": "config.clock", ...}}`) but the shape is not settled. Every breath/drift/pulse variant in the debug corpus uses signals. Getting this wrong propagates into every animated recipe.

**Risk:** drafting SignalGraph shape in isolation of mixed-signals' eventual JSON surface. The two must align.

**Mitigation:** lock SignalGraph shape as one of the first implementation-phase deliverables; run it past mixed-signals before recipe corpus re-authoring begins.

### 20 — The "factory-internal is opaque" position is principled but has migration cost

Each time a factory-internal pattern turns out to generalize (Q15 row_mask → scope.rows was the clearest case during migration), recipes that used the factory-internal form need re-migration. If V3 ships with text_contrast (Q16) factory-internal and later promotes it to schema-surface, every highlighter recipe re-authors.

**Mitigation:** explicit rule-of-three process for promoting factory-internal patterns (part of Open Q #27 below). Audit factory payloads against the mainline corpus at authoring-guide time, not just at migration time.

### 30 — Procedural generator params opacity is probably a bigger gap than Q30 suggests

During Stage 4 (madeira-flag) I produced a fireworks procedural generator with ~10 nested config fields (palette array, spawn_zones structure, cycle timing, ballistic params). All of it lives under `params: serde_json::Value` with no schema enforcement. At that complexity, validator and authoring guides lose traction.

**Mitigation:** generator registry with per-generator typed schema as part of Decision 5's implementation track. Q30 flags this; worth elevating priority.

### 40 — Wargames / extends migration is trivially mechanical, which might mask other issues

The 56-file wargames migration was batch-transformed by Python because every child recipe carried the same trivial pattern (`extends` + `message` + `lifecycle` overrides). This validates that `extends` works unchanged in V3 (positive finding) but also means the wargames corpus didn't stress-test anything V3 cares about beyond `extends` compatibility.

**Watch:** the full mainline-corpus re-authoring pass (Chapter 50 Phase 2) will be the first real stress test for V3's authoring-briefing infrastructure. Don't assume wargames-like corpus segments will teach V3 anything new; budget for real re-authoring surprises when the rest of the corpus lands.

## 60 — Open questions to promote to plan level

Four new Open Questions surfaced by this memo's analysis. Proposed numbering continues from Q#23 (the timer question the other reviewer added):

### 10 — Open Q #24: Canonical normalized IR as explicit V3 artifact

**Question:** should V3 define a canonical normalized IR that `canonicalize(raw_recipe) → normalized_recipe` transforms author syntax into, with validator / viewer / property tests / migration equivalence all operating on the normalized form?

**Current state:** partially implicit in Open Q #9 (validator redesign) and Q#10 (viewer), both of which the reviewer suggested should build on a normalized IR. Neither question names the IR as a standalone artifact with its own spec.

**Stakes if yes:** a fourth tooling track (canonicalizer) joins the cutover work but every other tool becomes simpler (single validation surface, single property-test surface, single viewer target, single migration-equivalence target). Also makes Q#5 (named-factory vs primitive equivalence) trivially testable.

**Stakes if no:** each tool re-implements its own normalization implicitly; authoring-surface variations multiply tooling code paths; round-trip fidelity is not checked.

**Lean:** yes, make it a first-class V3 deliverable. Tachyonfx's `to_dsl()` is a model.

### 20 — Open Q #25: Primitive catalog governance

**Question:** what is the explicit criterion for promoting a shader to a new base primitive kind (sibling of `colored_overlay`) vs adding a new `Pattern` variant vs leaving it as an earned-name composition vs not adding it at all?

**Current state:** Decision 2 names the three classification tiers but doesn't specify the promotion criterion. During migration I made 10 "primitive itself" classifications using judgment; reviewers might reasonably classify differently.

**Stakes:** the primitive catalog size is a major schema surface. Ungoverned growth bloats it; overly-strict governance leaves authoring holes.

**Lean:** document three explicit criteria:
1. **New base primitive kind** when `ColoredOverlay + Pattern` cannot express the spatial function (generator-class, positional, per-channel).
2. **New Pattern variant** when a spatial-distribution function isn't covered by existing variants AND has at least two distinct authoring use cases (rule-of-two for Patterns, not rule-of-three, because the debug corpus has per-variant single-recipe coverage).
3. **Earned-name composition** when specific parameter tuning encodes design judgment worth locking in (Decision 2's existing criterion).

### 30 — Open Q #26: Content-effect catalog governance parity with Decision 2

**Question:** does V3 apply Decision 2's primitive-vs-earned-name / Tier-1/2/3 framing to content effects (typewriter, scramble, split_flap, etc.) the same way it applies to shaders?

**Current state:** Decision 2 is shader-scoped by its wording. Content effects are treated as unrestricted factories. The catalog (~16 named effects) has similar structure to the shader catalog (27 named shaders) and similar authoring pressures.

**Stakes:** if content effects grow the same way shaders have, they'll accumulate the same discoverability problems Decision 2 was written to solve for shaders.

**Lean:** yes, apply Decision 2 symmetrically. Primitive content effects (`redact`, `mirror`, `wrap_indicator` — minimal params, no design judgment encoded) get primitive treatment. Earned-name content effects (`split_flap` with authenticity flags, `typewriter` with cursor sub-configs) get library-factory treatment. Scramble-glitch-shift is a composition worth flagging as composition-earning-name.

### 40 — Open Q #27: Factory payload opacity governance

**Question:** what's the explicit process for promoting a pattern that's factory-internal in initial V3 to schema-surface later? Rule of three? Design review? Implicit promotion on every authoring-guide pass?

**Current state:** the schema draft says "factory-internal stays factory-internal" for Q9/Q13/Q14/Q16/Q18. But Q15 (row_mask) is the precedent where factory-internal turned out to need scope-surface promotion. Without a governance rule, future Q15s get handled ad-hoc.

**Stakes:** recipes written against factory-internal conventions need re-migration each time a pattern gets promoted. Explicit governance keeps that from happening silently.

**Lean:** rule of three at authoring-guide writing time. When writing the authoring-guide for a new-in-V3 factory that shares a pattern with two existing factories, flag the shared pattern for schema-surface review. Promote to schema-surface only if the pattern has three factories and a clean abstraction. Otherwise document the factory-internal convention and move on.

## 70 — Deferrable (clearly not blocking)

The following were surfaced but shouldn't influence V3 shape:

- **Q17 positional primitives widget-layout coupling** — cursor, trace_path embed widget-local cell coords. Runtime-binding via `ParamValue<u16>` handles dynamic cases. Scene-layer placement (Decision 5) may introduce relative coord systems later; revisit then.
- **Q18 primitives with internally-composed sub-animations** — trace_path.paths[] with per-path delays. Factory owns composition. Split to parallel steps only if per-path params diverge.
- **Q13 tagged-union shape discriminators** — focus_field.shape is a within-factory tag, not a schema-surface concern. Intention 39 merge semantics apply automatically via `deny_unknown_fields`.
- **Fourth StepInput arm for recipe-derived values** — interesting but speculative. Wait for concrete corpus demand.

## 80 — Summary of recommendations

**Schema draft additions (10 items):** base_style.modifiers, motion_path+offscreen (post-Q#22), scope row_mask absorption (Q15), symbolic-vs-coord tagged-union (Q26), sampler phase→phase_offset (Q29), V2 filter-list documented pattern (Q28), Open Q #23 timer stub, factory-internal conventions explicit section, ParamValue-typed scope coords, primitives/compositions directory convention note.

**New concepts for discussion (4 items):** canonical normalized IR, StepInput<T> fourth arm, primitive catalog governance, content-effect catalog parity.

**Plan-level Open Questions to add (4 items):** Q#24 canonical IR, Q#25 primitive-catalog governance, Q#26 content-effect Decision-2 parity, Q#27 factory-payload-opacity governance.

**Watch items (4):** SignalGraph-shape-mixed-signals alignment, factory-internal migration cost, procedural generator params priority, wargames-migration non-representativeness.

**Deferrable (4):** Q17, Q18, Q13, fourth StepInput arm.

None of these are blocking. The V3 approach carries.

<!-- <FILE>docs/design/tui-vfx-v3-migration-findings-memo-claude.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
