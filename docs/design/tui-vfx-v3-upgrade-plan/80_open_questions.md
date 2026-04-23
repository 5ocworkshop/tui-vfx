<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md</FILE> - <DESC>Chapter 80 — the 27 open questions that must resolve before V3 implementation, plus reviewer-opinion annotations from the 2026-04-21 GT-Design lead review memo. Each question names both the question and what's at stake in choosing different answers.</DESC> -->
<!-- <VERS>VERSION: 1.2.0</VERS> -->
<!-- <WCTX>Promote four plan-level questions surfaced in the debug-recipes migration findings memo (docs/design/tui-vfx-v3-migration-findings-memo-claude.md §60). Q#24 canonical normalized IR as explicit V3 artifact; Q#25 primitive catalog governance criteria; Q#26 content-effect catalog Decision-2 parity; Q#27 factory payload opacity governance rule. All four are governance/process questions that the migration exercise surfaced but that don't belong at the schema-discussion level.</WCTX>
<!-- <CLOG>1.2.0: MINOR — add Open Q #24 (canonical normalized IR), Q#25 (primitive catalog governance), Q#26 (content-effect Decision-2 parity), Q#27 (factory payload opacity governance) from the debug-recipes migration findings memo.</CLOG> -->
<!-- <CLOG>1.1.0: MINOR — add Open Q #23 on timer primitive vs distributed timing mechanisms. Three-option framing (step-level Timer field, `StepInput<T>` sibling, status quo with clearer docs). Surfaced cases where the distributed model strains: per-step durations in `Sequence`, staggered entrances, content-effect timing.</CLOG>
<!-- <CLOG>1.0.0: initial extraction from the monolith with new Open Q #22 added covering motion_path / arc / bezier / spring trajectories and offscreen from/to, previously flagged only as a major gap in the migration log final audit and schema draft. Easings themselves are covered in Decision 3 (pipeline.timing); the gap is the geometry-aware trajectory primitives and offscreen origin/destination semantics.</CLOG> -->

# 80 — Open questions that must resolve before implementation

These are not rhetorical — each represents a real design choice not yet settled. Ordered roughly by impact on the plan shape.

Reviewer-opinion annotations reference the 2026-04-21 GT-Design lead review memo, captured as "one input, question remains open" so the reviewer's recommendations are visible without closing the questions prematurely. For questions already informed by Concerns A–F (Q2/B, Q12/F, Q14/D, Q18/C), the reviewer input is load-bearing for the existing resolution while implementation-level sub-questions remain open.

## 10 — Q#1: Does the `kind` discriminator survive?

Tachyonfx collapses mask/filter/sampler/style into one `Effect` with naming by factory function. Our working assumption is that the four kinds survive as enum variants on the unified Step because they represent genuinely different operations (reveal vs post-process vs texture-overlay vs style-transform), and the distinction aids authoring comprehension. But if after implementation we find the boundary is mushy — e.g., a mask with scope is indistinguishable from a filter with a cell-clear payload — we may want to collapse further. Preserving the distinction is the safe starting position; reducing later is easier than splitting later.

**Reviewer's opinion:** keep the `kind` discriminator. *"The distinction is still useful for comprehension, validation, documentation, and tooling. You can always collapse later if the boundaries truly prove artificial."*

## 20 — Q#2: Migration strategy and schema versioning — resolved as Concern B (see Chapter 50)

**V3 is a clean cutover** — V2 is not carried forward as a loadable format. The entire shipped recipe corpus migrates during V3 implementation; V3 is the new floor. The three-phase Curate → Re-author → Validate workflow (with critical-set fixture-equivalence carve-out) that resolves this question has been promoted from open-question status to Chapter 50's first-class migration workflow chapter. This question number is preserved for cross-reference continuity; the full content lives in Chapter 50.

**Reviewer's opinion (input behind Concern B's resolution):** hybrid migration (mechanical translation + human review + fixture/probe equivalence for critical recipes), explicitly NOT purely manual Claude-led rewrite of the whole corpus. Load-bearing for Chapter 50's three-phase model.

## 30 — Q#3: Phase-scoping shape: per-step field vs container

Currently proposed: each step carries `phase: Enter | Dwell | Exit | All`. Alternative: phase is a container (`Phase::Dwell(...)`) wrapping its children. Per-step field is flatter and matches the scope-field pattern (both are metadata on an atom). Container is more readable at a glance (the tree clearly segments by phase). The two shapes are isomorphic. Decision pending readability review against the appendix translations.

**Reviewer's opinion:** per-step field, with container propagation. *"This matches the scope model, keeps the normalized shape regular, and still allows readable grouping."* Aligns with the plan's current lean.

## 40 — Q#4: Composition combine semantics — explicit or defaulted

Current flat schema has implicit filter ordering ("applied in order") and explicit mask combine modes (`All | Any`). Tree `Parallel` containers could carry a `combine: Chain | Union | Intersect | Replace` policy, or combine could be per-kind with sensible defaults. Authoring ergonomics strongly prefer per-kind defaults; safety-net arguments lean toward explicit-at-container. Probably: per-kind defaults with container override available, but the exact default table needs discussion.

**Reviewer's opinion:** per-kind defaults plus explicit container override. Also strongly recommends a **normalized internal form** where the effective combine is explicit after parsing/canonicalization, so tooling and tests don't have to re-infer defaults.

## 50 — Q#5: Named-factory and compositional JSON coexistence

Both `{"type": "diffusion", ...}` and `{"type": "colored_overlay", "pattern": {...}, ...}` load to the same internal representation. Do we:

- Validate that both shapes produce identical behavior (property test)?
- Privilege one in examples and SKILLS.md, or teach both?
- Allow themes to mix both in the same recipe, or enforce consistency?
- Provide a canonicalization tool to convert named → compositional for inspection?

**Reviewer's opinion:** yes, support both; validate equivalence; provide canonicalization tooling. Property-test equivalence for curated pairs; canonicalize for inspection/debugging; teach named factories for curated presets; teach primitive/compositional form for advanced/custom authoring. Allow mixing in one recipe, but don't make mixing the default teaching style.

## 60 — Q#6: Scope primitive — open-closed tension

Closed algebraic enum is safer, validatable, and cacheable (cf. tachyonfx's static/dynamic analyzer with bitmask caching). Closure-based escape hatches (`PositionFn`, `EvalCell`) are powerful but uncacheable and resist static validation. We need both: closed variants for 95% of authoring intents, escape hatches for the novel 5%. The open question is the boundary. One proposal: closed variants are directly JSON-encodable; closure escapes require Rust-side registration (the recipe references a named predicate registered at compile time, not an arbitrary eval string).

**Reviewer's opinion:** closed enum with registered escape hatch — *"the right balance for caching, validation, and authoring predictability."* Aligns with the plan's current lean.

## 70 — Q#7: Relationship to `RecipeSceneCanvas` (Intention 50)

The tree-schema migration is nominally independent of the in-GTD recipe-playback substrate (`RecipeSceneCanvas`). But both land in the same authoring surface and both unblock the same explorations (ambient halo, ember-felt). Sequencing options:

- Land `RecipeSceneCanvas` first against V2 flat schema, then migrate schema.
- Land V3 tree schema first, then build `RecipeSceneCanvas` against it.
- Land both in the same cutover under a unified "pipeline v3" banner.

Each has different risk profile and different blocking structure. Needs explicit decision.

**Reviewer's opinion:** do not make GTD substrate sequencing the blocker for upstream V3 core work. Upstream should stabilize first (canonical semantic seam, naming cleanup, token/binding contracts, normalized execution model); GTD then adapts `RecipeSceneCanvas` to that seam.

## 80 — Q#8: Unblock order for Relative Light explorations

Ambient halo and ember-felt are both easier to express in V3. Do we:

- Block both on V3 (delays them but keeps recipes consistent).
- Ship them in V2 first, migrate later (unblocks the work but increases migration corpus).
- Ship them in V2 but gate them behind feature flags so migration is clean.

**Reviewer's opinion:** do not ship productized V2 versions if V3 is clearly the right substrate. Continue exploration as isolated R&D fixtures, debug recipes, or lab-only prototypes — not as user-facing V2 contracts that would immediately need migration.

## 90 — Q#9: Validator redesign

Tree schemas need different validation than flat schemas. New rules to design:

- Scope-coherence (no nonsensical scope combinations — e.g., `GlyphMatches` on a mask-reveal operation where "glyph" doesn't yet exist).
- Container-shape invariants (no `Parallel` with conflicting masks; no `Sequence` with zero steps).
- Scope-propagation conflict detection (child declares scope X, parent propagates Y; precedence rule?).
- Migration validation: V2 → V3 auto-migration must preserve probe-equivalence on the full recipe corpus.

**Reviewer's opinion:** this is **core V3 work, not support work.** Validator scope: scope coherence, tree/container invariants, hint ambiguity, fragment addressability, token/binding contracts, migration equivalence for critical fixtures. Also validate a **canonical normalized IR**, not only raw authoring syntax — that keeps the validator durable across future schema evolution.

Full treatment in Chapter 60 §40 (validator redesign is a release-gate-adjacent concern).

## 100 — Q#10: Viewer still worth building independently

Even with tree authoring, a visual renderer of a pipeline (inspector / debugger / recipe explorer) is valuable. In an earlier conversation turn the viewer was proposed as a *substitute* for tree authoring; we rejected that. But the viewer has independent value — does it stay on the backlog as its own project, or does it fold into a broader "recipe tooling" initiative post-V3?

**Reviewer's opinion:** yes, but build it on the normalized execution graph / canonical IR, not directly on author sugar. *"That will make it much more durable across future schema evolution."* Pairs with Q9's "validate normalized IR" point — viewer and validator can share the IR surface.

## 110 — Q#11: Docs, SKILLS.md, and generator updates

Every existing doc page that references schema fields needs updating:

- Generated API docs
- Recipe authoring guides
- `docs/api/AI_ORIENTATION.md`
- `docs/api/SKILLS_REFERENCE.md`
- VFX usage guides
- Theme authoring checklist
- Every lab/example README that references recipes

The generator infrastructure (`just docs-gen`, drift checks) must be updated in the same cutover. Bounded but non-trivial work; plan needs to budget for it. Chapter 100 enumerates this tooling slice as a release-blocking track.

**Reviewer's opinion:** must ship in the same cutover as the schema change. Especially important: generated API docs, validator/tracing docs, AI/LLM guidance, migration notes, and canonical examples. Aligns with the plan's current lean.

## 120 — Q#12: Shadow rendering, offscreen composition, probe/trace compatibility — V3 release gate (Concern F)

The six release-gate criteria are specified in Chapter 60. This question number is preserved for cross-reference continuity; the full content lives there.

**Reviewer's opinion (input behind Concern F's resolution):** release gate, not optional polish. One input among several; the implementation-level sub-questions (tolerance spec, gate ownership, whitelist discipline, recapture cadence, escalation path) remain open and live in Chapter 60.

## 130 — Q#13: Partial-phase spans (PhaseSet granularity)

Today's phase scoping is binary: effects either apply to one phase (`enter`, `dwell`, or `exit` via per-phase slots) or to all three (via `pipeline.continuous` with a unified `RaClock`; added in tui-vfx-recipes v2.7.0). There is no way to declare **"enter + dwell but not exit"** or **"dwell + exit but not enter"** — a legitimate authoring intent (e.g., a glow that arrives and sustains but shouldn't carry through the fade-out).

V3 should support `phase: PhaseSet` where PhaseSet is any subset of `{Enter, Dwell, Exit}`, with `All` and single-phase remaining valid shortcuts. The `continuous` block's unified-clock semantics should port forward as a clock-selection policy on any multi-phase step.

Open: does the PhaseSet shape live at the step level (every step can phase-scope) or only on containers (Parallel/Sequence containers carry phase membership that propagates to children)? Per-step is more expressive; container-scoped is more readable. Both could coexist with propagation rules.

**Reviewer's opinion:** yes, support `PhaseSet`, and keep it available at the step level. Container propagation can exist too, but don't make container-only the model. Aligns with the plan's per-step lean.

## 140 — Q#14: Tokenization ownership and contract discovery — resolved as Concern D

`Substitutions` (load-time) and `RuntimeBindings` (per-frame) are two distinct API surfaces at the `tui-vfx-recipes` boundary, divided by *temporal lifetime* rather than by content domain. Each surface handles text and structured values internally via method families. Optional `RecipeContext` umbrella wrapper for ergonomic one-call passing. Strict-mode default for `Substitutions`; graceful fallback for `RuntimeBindings`.

Per-surface contract discovery:
- `requires_substitutions: { <name>: <spec> }` — load-time contract
- `requires_bindings: { <name>: <spec> }` — per-frame contract

Failure modes differ per surface because lifetimes differ: load-time misses are hard errors, per-frame misses are graceful fallbacks. Cannot be collapsed onto a single API without losing precision.

**Reviewer's opinion (input behind Concern D's resolution):** move the API upstream, but separate load-time substitutions from runtime bindings. Plus: explicit declared contracts, strict-mode default, introspection API, byte-based asset resolution. Load-bearing for the two-surface split.

## 150 — Q#15: Vocabulary refresh scope — comprehensive is the right default

Principle 3 names the tension: vocabulary still carries notification archaeology (`auto_dismiss_ms`, `anchor`, `continuous`, `enter/dwell/exit`). V3 is a natural moment to do a vocabulary pass alongside the structural pass.

**Direction: lean comprehensive.**

**Specific renames to evaluate during V3 implementation** (with lean for each):

| Current | Candidate | Lean | Notes |
|---|---|---|---|
| `auto_dismiss_ms` | `duration_ms` | Rename | Toast-centric term; "duration" is neutral for splash, ambient, movie beats |
| `anchor` | `placement` | Rename | "Anchor" has notification-specific connotation; placement is neutral |
| `continuous` block | `persistent` or integrated into tree schema | Rework | Implied non-persistent was the default; reversed in widget/ambient contexts |
| `enter/dwell/exit` | *Possibly* `arriving/present/departing` | Evaluate | Keep if translation study shows genuinely general; otherwise rename |
| `notification_*` fields | Remove `notification` prefix | Rename | Field names that still reference "notification" as a concept |
| `schema_version` (recipe root) | Keep | Keep | Version concepts are domain-neutral |

Decision for specific renames gets finalized with the translation study (Workflow C — see Chapter 110).

**Reviewer's opinion:** comprehensive-but-selective — a meaningful divergence from the plan's current "comprehensive (full pass)" default. Specific leans:
- **Rename** `auto_dismiss_ms` (aligns with plan)
- **Probably rename/rework** `continuous` (aligns with plan)
- **Rename** preview seam nouns/modules (Open Q #19 — aligns with plan)
- **Keep `anchor`** unless semantics change — in a ratatui/grid context, anchor is already a good geometry term (plan currently leans rename)
- **Keep `enter/dwell/exit`** unless translation study proves they are actively misleading (plan currently leans evaluate-and-possibly-rename)

Worth resolving during the translation-study phase.

## 160 — Q#16: Cross-step hint resolution rules

Decision 7 establishes step output hints as first-class, modeled as the distinct `HintRef<T>` type. Multiple implementation questions remain for `HintRef<T>` semantics:

- **Multiple producers:** when two upstream steps in the same pipeline produce hints with the same name, which wins? Options: first-producer, last-producer, explicit reference by producer ID, compositor-error (forbidden ambiguity).
- **Hint composition:** can a step bind to "displacement from step A multiplied by the signal from step B"? This compounds Decisions 6 and 7 in a way that might need special syntax.
- **Scope of hint visibility:** are hints visible only within the same layer, across layers (so a scene-level step can read a layer-level hint), or both with explicit qualifiers?
- **Hint lifetime:** do hints persist across frames, or are they recomputed every frame? Probably the latter, but the validator needs to enforce it.

These are implementation-level questions but they affect schema shape — the answers determine whether hint references are bare names (`displacement`) or scoped (`layer.flag.displacement`).

**Reviewer's opinion:**
- Visibility defaults to **same pipeline / same layer only**
- Cross-layer reads require explicit export/import semantics if they exist at all
- Hint lifetime is per-frame / ephemeral
- **Multiple producers for the same visible hint should be a validator error unless explicitly qualified** — not "first wins" or "last wins" silently. *"That is too brittle."*

Decision 5's implementation track already defaults to same-layer-only hint visibility, aligning with this lean.

**Why this matters more now:** once the compiled V3 path has no remaining
compiled-path replay callsites, cross-step hint semantics become one of the
main blockers to claiming a fully independent V3 pathway. At that stage the
largest remaining gaps are no longer “bridge plumbing” gaps, but semantic ones
like hint visibility, producer conflicts, and downstream binding rules.

## 170 — Q#17: Primitive library / `$use` fragment composition

The schema already supports two hierarchical composition mechanisms, both shipping and in production use:

- **`extends` — full-recipe inheritance.** A recipe declares `"extends": "themes/new_wopr_fullscreen_cyan.json"` and inherits everything from the base, overriding only the fields that differ. Used extensively in the `wargames/` recipes (57 files extend from 9 base themes). Implementation: `fnc_resolve_recipe_template.rs` + `fnc_deep_merge_json.rs`.
- **`template + variants` — multi-recipe expansion from one file.** One file carries a `template` block + a `variants` array; loading via `load_all` / `from_value_all` yields N concrete recipes. Implementation: `fnc_expand_variants.rs`. (Correction in progress per Intention 51 / Principle 4 — see Chapter 90 §50 Retrospective corrections.)

The missing third mechanism is **named reusable fragments / primitive library** — small chunks (not full recipes) that multiple unrelated recipes can reference without inheriting from a common base.

```json
// recipes/shared/primitives.json — library of named fragments
{
  "definitions": {
    "computer_typing": { "type": "typewriter", "speed_variance": 0.0,
                          "cursor": { "character": "█", "blink_interval": 0.0 } },
    "wopr_colors_cyan": { "fg": "cyan", "bg": "black" }
  }
}

// a recipe that uses fragments without inheriting
{
  "uses": ["shared/primitives.json"],
  "config": {
    "content": { "effect": { "$use": "computer_typing" } },
    "colors":  { "$use": "wopr_colors_cyan" }
  }
}
```

Three mechanisms solve three different problems:

| Mechanism | Cardinality | Use case |
|---|---|---|
| `extends` | one-to-one | "Recipe A is a modified version of recipe B" |
| `template + variants` | one-to-many | "One file defines N similar recipes" |
| `$use` / primitive library | many-to-many | "Many recipes share this small piece" |

Each is distinct and non-substitutable. With only `extends`, you can't share a fragment across recipes with different inheritance roots. With only `template + variants`, you can't compose fragments from independently-authored pieces.

**Sub-questions** include parameterized fragments (substitutions inside fragments; overlaps with Open Q #14), fragment inheritance, fragment versioning, compile-time vs load-time resolution, Principle-4 compliance (addressability), earned-place discipline (rule of three), interaction with the Morris principle, theme-scoped vs global namespace, justification bar for promoting a pattern to a fragment.

**Reviewer's opinion:** yes, but keep v1 minimal and non-blocking — one fragment mechanism, flattened at load time, parameterization via the same substitution system (Open Q #14 / Concern D), no fragment inheritance in v1 unless a real case demands it, strict addressability + introspection from day one.

## 180 — Q#18: Step-level `RoutingRole` and recipe-level `SurfaceIntent` — resolved as Concern C

V3 distinguishes **four** separate role-shaped concepts:

| # | Type | Scope of application | Home | Status |
|---|---|---|---|---|
| 1 | `RoleTag` | Per-cell render role on cells produced by sources | `tui-vfx-types` | Existing — unchanged |
| 2 | `ThemeRole` | Theme-resolved semantic cell targeting via `Scope::ThemeRole(...)` | `tui-vfx-recipes` scope module | Decision 1 variant (renamed from `Role` in v0.11.0) |
| 3 | **`RoutingRole`** | **Per-step behavior hint** | `tui-vfx-recipes` step field | **Open Q #18 — this question** |
| 4 | **`SurfaceIntent`** | **Per-recipe hosting hint** | `tui-vfx-recipes` recipe field | **Open Q #18 — this question** |

`RoutingRole` and `SurfaceIntent` are kept as separate types rather than collapsed into a single `Role` field because their consumers, value sets, and evaluation contexts differ:

- **`RoutingRole`** governs *what kind of work a step does within a recipe*. Consumers are runtime behavior engines — reduced-motion skipping, performance tiering, screen-reader dispatch, probe/trace filtering. Working vocabulary: `content`, `affordance`, `feedback`, `alert`, `decoration`.
- **`SurfaceIntent`** governs *what container a recipe belongs in when hosted*. Consumers are the surface/hosting policy layer. Working vocabulary: `splash`, `toast`, `modal`, `transition`, `ambient`, `movie`.

Both are **consumer hints, not contracts.** Validators warn on unknown values but don't reject recipes. Hybrid open/closed vocabulary: canonical enum with `Custom(String)` escape hatch.

**Reviewer's opinion (input behind Concern C's resolution):** yes to routing metadata; explicit "no" to collapsing into the existing `RoleTag` domain. Floated four candidate names (`routing_role`, `surface_intent`, `playback_role`, `semantic_tag`). Load-bearing for the four-type split.

## 190 — Q#19: "Preview" naming for the canonical engine seam

`PreviewItem`, `PreviewManager`, and the `src/preview/` module path are the real canonical engine primitives today — they are what consumers wrap to reach recipe playback. But the "preview" name connotes "demo/temporary" rather than "canonical/authoritative."

**Candidate renames:**

- `PlaybackItem` / `PlaybackManager` — names what the object actually does
- `RecipeItem` / `RecipeManager` — names the domain
- `ItemManager` with `src/items/` — simplest
- Keep `PreviewItem` but rename the module path (`src/preview/` → `src/playback/` or `src/items/`)
- Keep both names via re-exports during a deprecation window

**Why this is V3-scoped:** V3 is already rewriting the recipe schema and renaming `Ra*` (Decision 4). Bundling the rename with V3 means one rename event instead of two.

**What not to rename:** the upstream `tui-vfx-recipes` demo binary (`cargo run --example demo`) and the debug recipes at `debug_recipes/` — both are genuinely demo/preview and the names are accurate. The *seam type* is what's misnamed.

**Reviewer's opinion:** rename now. Module path → `playback`; manager → `PlaybackManager`; seam type → possibly `PlaybackItem`, but also worth considering **`PlaybackPlan` / `PlaybackUnit`** as more future-proof once scenes and multi-layer content are first-class. Would not keep `Preview*` on the seam.

## 200 — Q#20: Surface identity vs neutral substrate — `RecipeSceneCanvas` overload

`RecipeSceneCanvas` today does two different jobs:

1. **Neutral substrate for recipe-first playback** — the architectural role Intention 50 names.
2. **Family-specific surface identity** — being used where a more specific identity (toast, notification, modal, tooltip) would be more truthful.

**Candidate V3 resolutions:**

- **(A) Keep `RecipeSceneCanvas` strictly as neutral substrate.** Require gt-design to produce family-specific surface identities (`ToastSurface`, `ModalSurface`, …) that wrap the substrate.
- **(B) Add explicit identity tags to `RecipeSceneCanvas`.** Single type, less wrapping ceremony, but keeps the overload.
- **(C) Hybrid:** substrate stays neutral, but gt-design provides a thin `SurfaceIdentity` trait / registry that maps recipe content to family identity.

Strong lean toward **(A)** because it respects Principle 5 most cleanly: substrate is meaning, surface identity is policy, each lives at its natural layer.

**Reviewer's opinion:** choose **option A**. Keep `RecipeSceneCanvas` as the neutral substrate family; gt-design wraps with family-specific surface identities. *"That aligns with GTD's current steering best: RecipeSceneCanvas is the substrate family, surface identity is higher-level policy, and internal variants (`RawRecipeSceneCanvas`, `ResolvedRecipeSceneCanvas`) can exist without changing that public conceptual split."* Aligns with the plan's strong lean.

## 210 — Q#21: Recipe metadata fields

Introduced as part of the deferred-design recipe metadata section (Chapter 90 §40). Covers the shape of the optional `metadata` block on each recipe (aesthetic_tags, mood, related_themes, use_cases, maturity_era, authoring_notes, last_reviewed) and the question of vocabulary (open string, closed enum, hybrid), required-vs-optional fields, and placement (inside `config` vs sibling to it).

**Reviewer's opinion:** keep metadata non-blocking for V3 core. `use_cases` should likely be required; most other fields can be optional initially. Discovery metadata (this field) should stay clearly separate from runtime routing metadata (`RoutingRole` / `SurfaceIntent` per Open Q #18). Aligns with the plan's current lean.

## 220 — Q#22: Motion path + offscreen trajectory migration

**Status: MAJOR GAP** flagged in the debug-recipes migration log's final audit and in the schema draft's gap list. Promoting it here to plan-level first-class status.

**The gap.** V2 recipes support motion-path-aware arrival and departure via `pipeline.{enter,exit}.motion_path: {type}` where the type enum covers linear, arc, bezier, spring, bounce, projectile, pendulum, and other PathType variants owned by `tui-vfx-geometry`. V2 also supports `pipeline.{enter,exit}.{from,to}: {type: offscreen, margin_cells, direction}` for slide-in / slide-out origins and destinations. The earlier V3 sketches only covered easing/timing; the newer direction promotes motion into its own first-class subtree so geometry-aware trajectories and screen-edge behavior have a clean home.

**Why easings are covered and motion_path is not.** Easings are 1D scalar curves over normalized time; they fit into `enter_ease` / `exit_ease` as strings referencing the `mixed-signals` easing catalog. Motion paths are 2D trajectories through cell space — a recipe sliding in on an arc from off-screen top-left doesn't resolve to a scalar time curve; it's a spatial path the compositor drives the recipe's geometry along. The two are different kinds of animation primitive.

**What this gap means in practice.** Every V2 recipe that uses motion_path ≠ linear and every recipe that uses offscreen from/to is currently unmigratable to V3 without data loss. The debug-recipes migration exercise didn't include such recipes (debug recipes are mostly one-shot fade-in with `linear` paths), so the gap wasn't visible in the Stage 1–6 migration — but the broader `recipes/` corpus has many such recipes and the wargames themes frequently use slide-in semantics.

**Options for V3 resolution:**

**(A) Add a first-class `config.motion.{enter,exit}` subtree, with matching scene-layer `placement.motion`.**

```json
"config": {
  "motion": {
    "enter": {
      "duration_ms": 500,
      "easing": "quad_out",
      "route": { "type": "arc", ... },
      "dynamics": [],
      "from": { "type": "offscreen", "margin_cells": 0, "direction": "from_top" },
      "edge_crossing": { "edge": "top", "border": "vanish", "shadow": "fade" }
    },
    "exit": {
      "duration_ms": 400,
      "easing": "quad_in",
      "route": { "type": "linear" },
      "dynamics": [],
      "to": { "type": "offscreen", "margin_cells": 0, "direction": "from_top" },
      "edge_crossing": { "edge": "top", "border": "vanish", "shadow": "fade" }
    }
  }
}
```

Keeps motion as a first-class authoring subtree without overloading the per-cell pipeline. `route` carries the carrier path, `dynamics[]` carries motion treatment layered over that route, and offscreen / edge-crossing behavior stay attached to the same motion phase object.

**(B) Introduce a `MotionPath` step kind.**

Treat motion-path as a first-class Step kind, uniform with mask/sampler/filter/style_effect/shader. Phase-scoped (`phase: enter` or `phase: exit`). Composes with other enter-phase steps in the pipeline tree.

Feels philosophically cleaner (uniform Step vocabulary) but couples motion semantics to the per-cell pipeline that otherwise doesn't know about whole-recipe geometry. Would require tooling and validator to treat MotionPath specially because it affects recipe placement, not cell-level transforms.

**(C) Use Decision 5 scene-layer placement to drive entry trajectories.**

`VfxSceneLayer.placement` already carries spatial composition for scene layers. Extending it with entry/exit animation (`placement.enter_from`, `placement.exit_to`, `placement.enter_path`) keeps geometry concerns together and leverages the existing Decision 5 implementation track. But this only covers scene-layer recipes; recipes without explicit scene layers would need a different mechanism.

**Default lean: (A).** Motion is recipe-level for whole-object movement and scene-layer-level for internal choreography. A dedicated `config.motion` / `placement.motion` home keeps geometry concerns together, avoids a fake step kind, and leaves the per-cell pipeline focused on effects rather than placement.

**Gate criteria affected.** Criterion 2 (offscreen / slide fixtures) in Chapter 60 already enumerates offscreen/slide behavior as a release gate. Motion-path resolution is a prerequisite for that gate being pass-able at all — if V3 can't express motion-path recipes, the fixtures can't be translated.

**Sub-questions** (post-shape-decision):
- Does `route` default to `linear` when absent? Probably yes.
- Do `from` / `to` default to the host's resting placement when absent? Probably yes.
- Should `via` be public in V3 initial even if the corpus uses it lightly? Probably yes, because Bezier and richer choreography need it.
- Should `edge_crossing.edge` be normalized from placements when omitted? Probably yes.
- How do `route` and `dynamics[]` map current `PathType` variants such as spring, bounce, pendulum, friction, orbit, projectile, and attractor? Needs an explicit compatibility table.
- Does motion reserve a future signal-driven hook from `mixed-signals`? It should at least leave room for it in the normalized / compiled model.

**Reviewer's opinion:** not yet solicited; this Open Question was added in v1.0.0 of this chapter (promoting the migration-log gap to plan-level status). Flag for next review cycle.

## 230 — Q#23: Timer story — distributed mechanisms vs first-class primitive

**Status.** Surfaced during the 2026-04-21 competitive-analysis pass against tachyonfx. Tachyonfx ships a per-effect `EffectTimer` with interpolation control as a core primitive — every effect carries its own timer. tui-vfx's timing is distributed across three mechanisms that evolved independently.

**Where timing lives today:**

1. **`config.motion`** — recipe-level enter/exit envelope. Owns whole-recipe geometry over time.
2. **`mixed-signals` temporal basis** — per-signal `temporal_frequency_hz`, `clock_ref`, keyframe time bases, ADSR envelopes. Owns signal-graph evaluation.
3. **Per-effect opt-in durations** — individual effects (some content transformers like `typewriter`, `scramble`, `morph`; some filters) carry their own duration fields. Owns localized per-step timing.

Three mechanisms means "how long does this one effect take" has three possible answers depending on which layer owns it.

**The question.** Is a unified `Timer<T>` primitive worth introducing, and if so where does it sit?

- **(A) Step-level optional Timer field.** Every step optionally carries `timer: { duration_ms, delay_ms, easing, interpolation }`. Subsumes the three mechanisms above. Cost: overlap with `mixed-signals` envelopes (ADSR is already a timer-shaped signal) and with recipe-envelope `config.motion`.
- **(B) Timer as a `StepInput<T>` sibling.** Alongside `ParamValue<T>` (external values) and `HintRef<T>` (pipeline-internal refs), a third type for time-parameterized inputs. Keeps `ParamValue<T>` clean. Cost: a fourth concept at the field site; collapses less.
- **(C) Status quo with clearer docs.** Keep three mechanisms, document which one to reach for when. No new primitive. Cost: author decision tree stays three-branched; competitive gap with tachyonfx remains.

**Cases where the distributed model strains:**

- Per-step durations inside `Sequence` containers. Tree-schema sequences run children in declared order; how long each child holds before handing off is either a per-effect field or a signal-graph envelope reaching zero. A Timer would make handoff explicit.
- Staggered entrances. Four parallel siblings with 100 ms stagger is expressible today via signal-graph delays but reads awkwardly. `timer: { delay_ms: N * 100 }` per child is one line.
- Content-effect timing (typewriter speed, scramble duration, morph rate). Per-factory fields today. Whether they unify with pipeline timing is open.

**Sub-questions if we unify:**

- Scope — step-level only, or also recipe-level (subsuming `config.motion`)?
- Relationship to `mixed-signals` envelopes. Is a Timer just a specific envelope shape, or a distinct primitive? If distinct, how do authors choose between them?
- Migration impact for V2's existing per-effect duration fields.
- Interaction with `ParamValue<T>` — can a Timer's `duration_ms` be a `ParamValue<u32>`, bindable to app state?

**Lean: defer decision to V3 implementation.** The distributed model has working paths for every case shipped so far. Tachyonfx's primitive is clearer for per-effect animation but overlaps with our existing signal-graph and pipeline-timing abstractions. Not load-bearing for V3 shape; worth explicit discussion before V3 implementation starts so the final model is deliberate rather than inherited.

**Reviewer's opinion:** not yet solicited; added after the 2026-04-21 competitive-analysis pass. Flag for next review cycle.

## 240 — Q#24: Canonical normalized IR as explicit V3 artifact

**Status.** Surfaced by the debug-recipes migration findings memo (docs/design/tui-vfx-v3-migration-findings-memo-claude.md §60). Partially implicit in Open Q #9 (validator redesign) and Q#10 (viewer), both of which the reviewer suggested should build on a normalized IR — but neither question names the IR as a standalone artifact with its own spec.

**The question.** Should V3 define a canonical normalized IR that `canonicalize(raw_recipe) → normalized_recipe` transforms author syntax into, with validator / viewer / property tests / migration equivalence all operating on the normalized form?

**Stakes if yes:** a fourth tooling track (canonicalizer) joins the cutover work, but every other tool becomes simpler:
- Single validation surface — Q#9's validator operates on one shape, not N authoring variations.
- Single property-test surface — Q#5's named-factory ↔ compositional equivalence test becomes trivial (canonicalize both sides, compare).
- Single viewer target — Q#10's viewer renders normalized-form only; authoring variations collapse before they reach rendering.
- Single migration-equivalence target — Chapter 60's release gates operate on the IR, not on raw JSON.

**Stakes if no:** each tool re-implements its own normalization implicitly; authoring-surface variations multiply tooling code paths; round-trip fidelity (author → canonical → author) is not checked.

**Lean:** yes, make it a first-class V3 deliverable. Tachyonfx's `to_dsl()` is a model — a canonical serialization that round-trips and that all downstream tools consume. Fourth tooling track is a real cost but pays for itself across validator, viewer, property tests, and release gates.

**Sub-questions** (post-shape-decision):
- Is the IR a Rust-only internal type, or also a serializable JSON form that tools outside the engine can consume?
- Does the canonicalizer live in `tui-vfx-recipes` (close to the schema) or in a separate crate (so `tui-vfx-tools` can depend on canonicalization without pulling the whole recipe runtime)?
- What's the stability contract on the IR? Can authoring-syntax sugar evolve without IR version bumps (preferred), or does every authoring change propagate?
- Does the IR surface hint-resolution (Q#16) in explicit form, so the viewer can show cross-step data flow?

**Reviewer's opinion:** not yet solicited; added 2026-04-21 after the debug-recipes migration findings memo. Flag for next review cycle. The existing reviewer already leaned "validate a canonical normalized IR, not only raw authoring syntax" in Q#9 and "build viewer on the normalized execution graph / canonical IR" in Q#10 — this question makes that artifact explicit rather than implicit.

## 250 — Q#25: Primitive catalog governance

**Status.** Surfaced by the debug-recipes migration findings memo (§60). Decision 2 names three classification tiers (primitive / earned-name composition / trivial composition) but doesn't specify the criterion for *which* tier a new capability lands in. During migration, 10 "primitive itself" classifications were made using judgment; a second reviewer might reasonably classify differently.

**The question.** What is the explicit criterion for promoting a shader to a new base primitive kind (sibling of `colored_overlay`) vs adding a new `Pattern` variant vs leaving it as an earned-name composition vs not adding it at all?

**Stakes.** The primitive catalog size is a major schema surface. Ungoverned growth bloats it (every new shader becomes a new primitive, the catalog sprawls); overly-strict governance leaves authoring holes (authors can't express a capability cleanly and reach for ad-hoc escape hatches).

**Lean: document three explicit criteria.**

1. **New base primitive kind** when `ColoredOverlay + Pattern` cannot express the spatial function. Rules of thumb: generator-class (produces content rather than distributing an existing color over cells), positional (needs cell geometry beyond what Pattern variants provide), per-channel (operates on fg/bg/glyph independently in a way Pattern doesn't model).
2. **New Pattern variant** when a spatial-distribution function isn't covered by existing variants AND has at least two distinct authoring use cases. Rule-of-two for Patterns, not rule-of-three, because the debug corpus has per-variant single-recipe coverage — demanding three would leave variants stranded as factory-internal.
3. **Earned-name composition** when specific parameter tuning encodes design judgment worth locking in. Decision 2's existing criterion.

**Sub-questions:**
- Who owns the promotion decision — spec author, design lead review, implementation review? Probably design lead review at authoring-guide write time.
- What's the demotion path — if a primitive was promoted and later turns out to be trivially expressible as `ColoredOverlay + Pattern`, is it deprecated or left as a convenience alias?
- Does the criterion apply symmetrically across all Step kinds (mask, sampler, filter, content), or is it shader-scoped by Decision 2's original framing? (See Q#26 for the content-effect answer.)

**Reviewer's opinion:** not yet solicited. Flag for next review cycle.

## 260 — Q#26: Content-effect catalog governance parity with Decision 2

**Status.** Surfaced by the debug-recipes migration findings memo (§60). Decision 2 is shader-scoped by its wording. Content effects (typewriter, scramble, split_flap, morph, redact, mirror, wrap_indicator, etc.) are currently treated as unrestricted factories — ~16 named effects with similar structure to the 27-name shader catalog and similar authoring pressures.

**The question.** Does V3 apply Decision 2's primitive-vs-earned-name / Tier-1/2/3 framing to content effects the same way it applies to shaders?

**Stakes.** If content effects grow the same way shaders have, they'll accumulate the same discoverability problems Decision 2 was written to solve for shaders: catalog sprawl, unclear which name to reach for, earned names mixed with thin wrappers, no governance on additions.

**Lean: yes, apply Decision 2 symmetrically.**

- **Primitive content effects** — `redact`, `mirror`, `wrap_indicator` are minimal-param, no-design-judgment-encoded transforms. Treat as primitives.
- **Earned-name content effects** — `split_flap` with authenticity flags, `typewriter` with cursor sub-configs, `scramble` with charset control. Treat as library-factory (earned name).
- **Composition-earning-name** — the scramble→glitch→shift pipeline observed during migration is a composition worth flagging for earned-name promotion if it proves recurrent.

**Sub-questions:**
- Does the content-effect catalog get its own `ContentPattern` enum analogous to `Pattern` for shaders? Or is the content domain structurally different (text-producing rather than color-distributing) enough that the analogy doesn't carry?
- If there is a parallel, does the Rust crate structure mirror — `tui-vfx-content` as the primitive crate, named effects as factories in `tui-vfx-recipes`?
- How does this interact with Q#23 (timer story)? Many content effects carry their own duration/speed fields; a primitive-vs-earned split affects which layer owns those fields.

**Reviewer's opinion:** not yet solicited. Flag for next review cycle.

## 270 — Q#27: Factory payload opacity governance

**Status.** Surfaced by the debug-recipes migration findings memo (§60). The schema draft says "factory-internal stays factory-internal" for several migration questions (Q9 sampler conventions, Q13 composition-axis constants, Q14 vignette defaults, Q16 contour line discipline, Q18 default_weights). But Q15 (row_mask) is the precedent where factory-internal *turned out to need* scope-surface promotion during migration. Without a governance rule, future Q15s get handled ad-hoc.

**The question.** What's the explicit process for promoting a pattern that's factory-internal in initial V3 to schema-surface later? Rule of three? Design review? Implicit promotion on every authoring-guide pass?

**Stakes.** Recipes written against factory-internal conventions need re-migration each time a pattern gets promoted from factory-internal to schema-surface. Explicit governance keeps that from happening silently and keeps the factory-internal→schema-surface boundary deliberate.

**Lean: rule of three at authoring-guide writing time.** When writing the authoring-guide for a new-in-V3 factory that shares a pattern with two existing factories, flag the shared pattern for schema-surface review. Promote to schema-surface only if the pattern has three factories and a clean abstraction. Otherwise document the factory-internal convention and move on.

**Sub-questions:**
- Who holds the "three factories" list — spec doc, validator manifest, authoring-guide index? Probably the validator manifest, because the validator already needs the schema-surface enumeration.
- What's the migration story for recipes written against factory-internal conventions when a pattern gets promoted? Auto-rewrite? Deprecation window? Strict-mode error with helpful migration hint?
- Does this interact with Q#17 (`$use` / primitive library)? A factory-internal pattern that gets used across three factories might be better expressed as a shared fragment than as a schema-surface primitive, depending on whether the shared pattern is data or behavior.
- Is there a demotion path — if a schema-surface primitive ends up used by only one factory, does it get demoted back? Probably no; schema promotion is sticky because recipes have already been authored against it.

**Reviewer's opinion:** not yet solicited. Flag for next review cycle. The Q15 row_mask precedent is load-bearing — that concrete migration case should inform how the governance rule handles ambiguity.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.2.0</VERS> -->
