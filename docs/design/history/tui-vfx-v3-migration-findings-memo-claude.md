<!-- <FILE>docs/design/tui-vfx-v3-migration-findings-memo-claude.md</FILE> - <DESC>Detailed memo of findings from the V2→V3 debug-recipes migration exercise. Recommendations for schema additions/tweaks/changes plus new concepts worth considering. Intended as one of two parallel memos; the other is authored by a different reviewer doing the same migration work. The two will be compared and synthesized into a refined V3 schema draft.</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Expand §60 from "four new questions to promote" to "my proposed answers on every outstanding schema-related question in Chapter 80, with an explicit ask for your take on each." Covers Q#1, #3, #4, #5, #6, #9, #13, #15, #16, #17, #19, #21, #22, #23 (existing in Chapter 80) plus Q#24-27 (proposed new, from the previous version of this memo). Excludes Q#7, #8, #10, #11, #20 as scheduling / process / project-scope rather than schema-shape questions. The §60 restructure turns the memo into a concrete input channel for the companion reviewer memo rather than a one-way findings report.</WCTX> -->
<!-- <CLOG>1.1.0: MINOR — restructure §60 to cover all 18 schema-related Chapter 80 questions with per-question proposed answer + ask-for-input. Update §80 summary to reflect. No changes to §10-50 validation findings, §50 watch items, §70 deferrable, or the four proposed new questions (now folded into §60 alongside the existing 14).</CLOG> -->
<!-- <CLOG>1.0.0: initial memo. Headline assessment (all refinement-level, no approach blockers), validated decisions, 10 schema draft refinements, 4 new concepts for discussion, 4 plan-level open questions to promote (Q#24-27), watch items, deferrables.</CLOG> -->

# V3 Migration Findings Memo — Claude

**Author:** Claude (one of two parallel reviewers)
**Scope:** Conceptual migration of 258 V2 recipes to draft V3 shape across 6 stages
**Date:** 2026-04-21
**Source documents (full filesystem paths for readers outside the repo):**

- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-debug-recipes-migration-log.md` — 34-question schema journal, 7-item drift table, final V2↔V3 coverage audit
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-schema-draft.json` — specification-by-example of the draft V3 schema (strip `#`-prefixed lines for valid JSON)
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-upgrade-plan/` — the 11-chapter V3 upgrade plan (referenced throughout as "Chapter NN"); `80_open_questions.md` is the source of the Q#N numbering used in §60 of this memo

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

## 60 — Outstanding schema-related questions — my proposed answers and a request for yours

Chapter 80 of the V3 upgrade plan has 27 numbered open questions. Four (Q#2, #12, #14, #18) are already resolved as Concerns B/F/D/C. Of the remaining 23, I'm treating 18 as schema-related (they shape the data model, the authoring surface, the validator, or the canonical seam). The other five (Q#7 GTD sequencing, Q#8 Relative Light scheduling, Q#10 viewer project placement, Q#11 docs rollout, Q#20 surface identity) are process / scheduling / project-scope and outside this memo's remit.

For each of the 18 schema questions I've included what I'd pick today. I'd like your lean on each — either a direct answer, a disagreement with rationale, or a "no opinion / defer to you." I'm not looking for consensus on every item before we compare memos; I'm looking for surface area where we converge (fast-track to plan) vs diverge (discuss explicitly).

Four entries (Q#24-27) are **proposed additions** to Chapter 80 that surfaced in this memo's analysis rather than in the existing numbering. They're flagged as `(proposed new)` inline.

### 10 — Q#1: Does the `kind` discriminator survive?

**My answer:** yes, keep the four-way `{mask, filter, sampler, shader}` discriminator on Step, plus `content_effect` and `style_effect` for the two content/style kinds the migration confirmed belong alongside the original four. Across 258 recipes every step classified cleanly into exactly one kind; I never hit an ambiguous "is this a mask or a filter" case that would signal the boundary is mushy. The discriminator also anchors validator messages, authoring guides, and trace-event taxonomy — collapsing it saves bytes in the recipe and adds cost everywhere else.

**Asking:** do you see any step-kind that wants to straddle two discriminators, or any case in your migration that felt like an artificial classification?

### 20 — Q#3: Phase scoping — per-step field or container?

**My answer:** per-step field. Stage 3's complex compositions all landed as `{kind: parallel, children: [...]}` with each child carrying its own `phase` tag. Container-only would have forced a group-by-phase outer structure the authors didn't want — they wanted group-by-semantic-intent (one Parallel per effect cluster) with phase orthogonal. Matches Scope's per-step attachment, keeps the normalized shape regular. Container propagation can still exist as sugar but per-step is the model.

**Asking:** did any of your migrations want a container-scoped phase that made per-step feel wrong, or was per-step natural across the corpus you touched?

### 30 — Q#4: Composition combine semantics — explicit or defaulted?

**My answer:** per-kind defaults, container override available, canonicalize to explicit form in the normalized IR (ties to Q#24). Masks default to declared `combine`, filters chain in order, samplers compose via HintRef — the migration never needed a Parallel container to carry its own combine policy. Authors won't write the combine field at authoring time in 95% of cases; tooling that needs explicit combines reads the normalized IR where it's always present.

**Asking:** did you see cases where per-kind defaults felt magical or surprising? The "canonicalize to explicit" escape hatch depends on Q#24 landing.

### 40 — Q#5: Named-factory and compositional JSON coexistence

**My answer:** support both, privilege named factories in authoring guides for discoverability, expose primitive/compositional form as the advanced path. Property-test equivalence for curated pairs. Don't mix the two inside a single step payload (`type: colored_overlay` with a named-factory shortcut field at the same level would be a validator error). Canonicalize both to the same normalized IR so downstream tooling sees one shape.

**Asking:** do you have a view on whether mixing is permitted within one recipe (across steps, not within one step)? My lean is yes — different steps can freely mix — but I could see a case for enforcing recipe-wide consistency for readability.

### 50 — Q#6: Scope primitive — open-closed tension

**My answer:** closed enum with a registered-named escape hatch. The 258-recipe migration closed cleanly under the closed vocabulary (area / row-or-column / cell / channel / content / theme-role / algebraic compose); no recipe needed an escape. For the rare case, `{kind: predicate, name: "<registered>"}` resolves to a Rust-side registry — no arbitrary eval strings, no closures in JSON. Caching as bitmasks still works because the registry entry declares static-or-dynamic at registration time.

**Asking:** did your migration turn up cases the closed vocabulary couldn't handle? If so I'd like to know which ones before declaring the vocabulary frozen.

### 60 — Q#9: Validator redesign

**My answer:** treat as core V3 work, not support. The validator operates on the normalized IR (Q#24), not on raw authoring syntax, so authoring-surface sugar can evolve without re-plumbing validation. Rule set: scope coherence (no `GlyphMatches` on a mask where glyphs haven't been produced yet), tree/container invariants (no empty Sequence; no Parallel with conflicting masks), hint producer/consumer resolution (every HintRef resolves to exactly one producer in scope), token/binding contracts (`requires_substitutions` and `requires_bindings` match recipe usage), migration equivalence for critical fixtures (Concern F gate material). Strict mode default.

**Asking:** do you see any of these categories as deferrable to post-V3, or any missing that should be core?

### 70 — Q#13: PhaseSet granularity

**My answer:** `phase: PhaseSet` at the step level, where PhaseSet is any subset of `{Enter, Dwell, Exit}`. `All` remains a valid shortcut for `{Enter, Dwell, Exit}`. Single-phase strings (`"enter"`, `"dwell"`, `"exit"`) remain valid shortcuts for singleton sets. Container-scoped propagation coexists as sugar but per-step is the normative model (consistent with Q#3). This closes the "enter+dwell but not exit" expressiveness gap without inventing a new container type.

**Asking:** does the singleton-string-as-shortcut feel right or does it conflict with future schema evolution? I could see an argument for always-array for schema regularity at the cost of authoring brevity.

### 80 — Q#15: Vocabulary refresh scope

**My answer:** comprehensive-but-selective, closer to the reviewer's position than the plan's default "full pass":

- **Rename** `auto_dismiss_ms` → `duration_ms`. Toast-archaeology term.
- **Rename / rework** `continuous` — fold into the tree schema's phase model rather than keeping a parallel mechanism.
- **Rename** preview seam nouns/modules per Q#19 (`PreviewItem` → `PlaybackItem`, `src/preview/` → `src/playback/`).
- **Keep** `anchor`. It's a neutral geometry term in a grid context; reviewer's argument persuades me.
- **Keep** `enter/dwell/exit` unless the translation study surfaces cases where they're actively misleading. My migration didn't surface such cases.
- **Drop** `notification`-prefixed field names where they exist.

**Asking:** do you lean comprehensive (full pass) or selective (targeted)? If selective, which of the above do you disagree with?

### 90 — Q#16: Cross-step hint resolution rules

**My answer:** same-layer visibility by default; cross-layer requires explicit export/import semantics if ever supported (probably not in V3); hint lifetime is per-frame ephemeral; multiple producers for the same visible hint within the same layer is a validator error (not first-wins, not last-wins, not last-producer-silently-wins). Bare names (`displacement`) rather than scoped names (`layer.flag.displacement`) — scope is implicit in the layer boundary. Hint composition (step A's hint multiplied by step B's hint) is a future `HintOp<T>` if ever needed; V3 ships hint-as-direct-bind only.

**Asking:** do you see hint composition as V3-scoped or post-V3? My lean is post-V3 because the migration didn't need it, but if your migration turned up a case I missed, worth knowing.

### 100 — Q#17: Primitive library / `$use` fragment composition

**My answer:** ship v1 minimal and non-blocking. One fragment mechanism (`$use` referencing a `definitions` block in a file listed in `uses: []`), flattened at load time, parameterization via the Substitutions system from Concern D, no fragment inheritance in v1, strict addressability + introspection from day one. The 258-recipe migration didn't need `$use` — `extends` handled the 57-recipe wargames hierarchy cleanly and `template + variants` handled one-file-many-recipes cases. `$use` is demand-driven; ship the minimum and grow it when real cases surface.

**Asking:** do you have a case from your migration where `$use` would have been cleaner than the existing two mechanisms? If not, we may be able to defer the whole question to post-V3 with no cost.

### 110 — Q#19: "Preview" naming for the canonical engine seam

**My answer:** rename to `Playback*`. `PreviewItem` → `PlaybackItem`, `PreviewManager` → `PlaybackManager`, module `src/preview/` → `src/playback/`. The reviewer's `PlaybackPlan` / `PlaybackUnit` suggestion is worth considering once scenes and multi-layer content are first-class — I'd pick `PlaybackPlan` if we're willing to commit to the "a plan, not an item" framing now, but `PlaybackItem` is the safer default if we're not. Bundle with V3 rename event so it's one migration, not two.

**Asking:** `PlaybackItem` or `PlaybackPlan`? I'll take whichever you prefer; either is better than `Preview*` but I'd rather not re-litigate later.

### 120 — Q#21: Recipe metadata fields

**My answer:** keep non-blocking for V3 core. Ship the optional `metadata` block with `use_cases: [string]` as the only required field (validator warns if missing). Other fields (`aesthetic_tags`, `mood`, `related_themes`, `maturity_era`, `authoring_notes`, `last_reviewed`) optional, open-string vocabulary initially, tighten during the corpus re-authoring phase once patterns emerge. Metadata lives sibling to `config`, not inside it. Keep discovery metadata clearly separate from routing metadata (Q#18's `RoutingRole` / `SurfaceIntent`).

**Asking:** do you agree `use_cases` is the only required field, or would you add one more?

### 130 — Q#22: Motion path and offscreen trajectory migration

**My answer:** Option A — extend `pipeline.timing` with `enter_path`, `exit_path`, `enter_from`, `exit_to`. Motion path is a recipe-level concern (governs the whole recipe's arrival and departure), not step-level (doesn't belong in the per-cell pipeline vocabulary) and not layer-level exclusively (non-scene-layer recipes still need slide-in semantics). Path types trial-deserialize against `tui-vfx-geometry::PathType` per Intention 38. Scene-layer-level placement animations from Decision 5 are additive and orthogonal — a scene layer can have its own `placement.enter_path` independent of the recipe-level path, with layer-level winning for that layer's trajectory.

**This is the one blocker where I'd like strong alignment before V3 implementation starts.** Criterion 2 of the release gate (offscreen / slide fixtures) can't pass if V3 can't express motion-path recipes, and my migration didn't include motion-path-using recipes (they live in the mainline corpus that Phase 2 re-authoring will hit). If you disagree with Option A I want to hear it now.

**Asking:** A, B, or C? If A, any sub-question answers (defaults, scene-layer interaction, ParamValue<PathSpec>) you feel strongly about?

### 140 — Q#23: Timer story — distributed mechanisms vs first-class primitive

**My answer:** defer introducing a unified Timer primitive to post-V3. Document the three-mechanism model (`pipeline.timing` for whole-recipe envelope, `mixed-signals` envelopes for signal evaluation, per-effect opt-in durations for localized content-effect timing) as intentional with clear authoring guidance: "reach for pipeline.timing for whole-recipe lifecycle, reach for mixed-signals ADSR / DampedSpring for per-step envelopes, reach for factory-internal durations only when the effect owns its own animation." If the corpus re-authoring phase surfaces concrete cases where the distributed model strains (staggered entrances, Sequence handoff), promote a Timer primitive then with real cases driving the design.

**Reasoning:** the competitive-analysis pressure from tachyonfx is real but my migration didn't find a case where the distributed model failed to express the intent. Introducing a fourth arm of `StepInput<T>` speculatively adds a concept site without a concrete win. Intention 24 applies — earned-place discipline.

**Asking:** did your migration surface a case where a first-class Timer would have been cleaner than the three-mechanism model? If so this becomes a much closer call.

### 150 — Q#24 (proposed new): Canonical normalized IR as explicit V3 artifact

**My answer:** yes, first-class V3 deliverable. Define `canonicalize(raw_recipe) → normalized_recipe`; validator (Q#9) / viewer (Q#10) / property tests (Q#5) / migration equivalence (Concern F) all operate on the normalized form. Tachyonfx's `to_dsl()` is the model — a canonical serialization that round-trips and that all downstream tooling consumes. Fourth tooling track at the cost of simplifying every other tool.

The canonicalizer lives in `tui-vfx-recipes` (close to the schema). The IR is both a Rust type and a serializable JSON form so external tools can consume it without pulling the recipe runtime. Authoring-syntax sugar can evolve without IR version bumps — the stability contract is on the IR, not on the author surface.

**Asking:** do you see this as worth the fourth tooling track, or do you prefer each tool implementing its own normalization implicitly? The implicit path is cheaper in week-one and more expensive in year-two.

### 160 — Q#25 (proposed new): Primitive catalog governance

**My answer:** document three explicit criteria:

1. **New base primitive kind** (sibling of `colored_overlay`) when `ColoredOverlay + Pattern` cannot express the spatial function. Trigger cases: generator-class (produces content rather than distributing a color), positional (needs cell geometry beyond Pattern), per-channel (operates on fg/bg/glyph independently).
2. **New Pattern variant** when a spatial-distribution function isn't covered AND has at least two distinct authoring use cases. Rule-of-two for Patterns, not rule-of-three, because the debug corpus has single-recipe-per-variant coverage by design — demanding three would strand variants as factory-internal.
3. **Earned-name composition** when parameter tuning encodes design judgment worth locking in (Decision 2's existing criterion unchanged).

Decision authority: design-lead review at authoring-guide write time.

**Asking:** rule-of-two vs rule-of-three for Pattern variants is my lightest call. If you'd hold to rule-of-three strictly I could be persuaded.

### 170 — Q#26 (proposed new): Content-effect catalog governance parity with Decision 2

**My answer:** apply Decision 2 symmetrically to content effects. Primitive content effects (`redact`, `mirror`, `wrap_indicator` — minimal params, no design judgment) get primitive treatment. Earned-name content effects (`split_flap` with authenticity flags, `typewriter` with cursor sub-configs, `scramble` with charset control) get library-factory treatment. Compositions (`scramble_glitch_shift`) get composition-earning-name treatment. Content domain is structurally different enough from shader domain that I'd *not* introduce a parallel `ContentPattern` enum — the text-producing surface doesn't compose the way color-distributing functions do — but the three-tier governance model applies.

**Asking:** do you agree content effects deserve the Decision 2 treatment, and do you agree `ContentPattern` as a parallel enum would be over-engineering?

### 180 — Q#27 (proposed new): Factory payload opacity governance

**My answer:** rule-of-three at authoring-guide writing time. When writing the authoring-guide for a new-in-V3 factory that shares a pattern with two existing factories, flag the shared pattern for schema-surface review. Promote to schema-surface only if the pattern has three factories and a clean abstraction. Otherwise document the factory-internal convention and move on.

The schema-surface enumeration lives in the validator manifest (since the validator already needs it). Recipes written against factory-internal conventions that later get promoted go through a deprecation window with a migration hint at validation failure — not auto-rewrite, not strict-mode error without a hint. Demotion (schema-surface → factory-internal) doesn't exist; schema promotion is sticky because recipes are already authored against it.

**Asking:** do you see a case for a faster promotion path (design-review-on-demand) or a slower one (rule-of-four)? Rule-of-three is the default I'd start from but isn't sacred.

## 70 — Deferrable (clearly not blocking)

The following were surfaced but shouldn't influence V3 shape:

- **Q17 positional primitives widget-layout coupling** — cursor, trace_path embed widget-local cell coords. Runtime-binding via `ParamValue<u16>` handles dynamic cases. Scene-layer placement (Decision 5) may introduce relative coord systems later; revisit then.
- **Q18 primitives with internally-composed sub-animations** — trace_path.paths[] with per-path delays. Factory owns composition. Split to parallel steps only if per-path params diverge.
- **Q13 tagged-union shape discriminators** — focus_field.shape is a within-factory tag, not a schema-surface concern. Intention 39 merge semantics apply automatically via `deny_unknown_fields`.
- **Fourth StepInput arm for recipe-derived values** — interesting but speculative. Wait for concrete corpus demand.

## 80 — Summary of recommendations

**Schema draft additions (10 items):** base_style.modifiers, motion_path+offscreen (post-Q#22), scope row_mask absorption (Q15), symbolic-vs-coord tagged-union (Q26), sampler phase→phase_offset (Q29), V2 filter-list documented pattern (Q28), Open Q #23 timer stub, factory-internal conventions explicit section, ParamValue-typed scope coords, primitives/compositions directory convention note.

**New concepts for discussion (4 items):** canonical normalized IR, StepInput<T> fourth arm, primitive catalog governance, content-effect catalog parity.

**My proposed answers on the 18 outstanding schema-related Chapter 80 questions (§60):** Q#1 keep discriminator; Q#3 per-step phase field; Q#4 per-kind defaults + explicit-in-IR; Q#5 both forms, don't mix within a step, property-test equivalence; Q#6 closed enum + registered predicate; Q#9 validator on normalized IR; Q#13 per-step PhaseSet; Q#15 comprehensive-but-selective; Q#16 same-layer hints, ephemeral, error on duplicate producer; Q#17 `$use` minimal and non-blocking; Q#19 rename to `Playback*`; Q#21 `use_cases` required, rest optional; **Q#22 Option A (motion in `pipeline.timing`) — my one strong-alignment ask**; Q#23 defer Timer primitive; Q#24 (new) canonical IR as first-class V3 deliverable; Q#25 (new) three promotion criteria, rule-of-two for Patterns; Q#26 (new) Decision 2 symmetrically to content effects, no parallel `ContentPattern` enum; Q#27 (new) rule-of-three at authoring-guide write time.

**Watch items (4):** SignalGraph-shape-mixed-signals alignment, factory-internal migration cost, procedural generator params priority, wargames-migration non-representativeness.

**Deferrable (4):** Q17, Q18, Q13, fourth StepInput arm.

None of these are blocking for V3 shape. The V3 approach carries. The one place I want strong alignment before implementation starts is Q#22 (motion path).

<!-- <FILE>docs/design/tui-vfx-v3-migration-findings-memo-claude.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
