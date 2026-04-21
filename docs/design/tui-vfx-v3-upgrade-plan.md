<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan.md</FILE> - <DESC>Draft V3 tui-vfx upgrade plan — migrating tui-vfx-recipes from the current flat pipeline schema to a uniform tree schema with a unified Scope primitive, Pattern-as-separable-axis as the internal shader model, and the William Morris "useful or beautiful" principle as corpus philosophy. Evolving hub where V3 scope, decisions, and open questions accumulate.</DESC> -->
<!-- <VERS>VERSION: 0.15.0</VERS> -->
<!-- <WCTX>Follow-up sweep after resolving Concerns A-F: (1) hygiene items flagged in the GT-Design lead review memo — correct "Three principles" text to "Five principles" (the plan added Principles 4 and 5 after the "Three" language was written), update the overview statement from "20 items" to "21 items" (Open Q #21 was added in the Deferred section), add a Decisions-reached framing note clarifying that "adopted" means direction is committed while implementation specifics may still be in flight for Decisions that carry their own track. (2) Reviewer-opinion annotations on every Open Question (1-21) and Open Q #21 in the Deferred section — each framed as "one input, question remains open" so the reviewer's recommendations are captured for later discussion without closing the questions prematurely. For Open Qs already informed by Concerns A-F (Q2/B, Q12/F, Q14/D, Q18/C), opinions are noted as load-bearing inputs to those resolutions.</WCTX> -->
<!-- <CLOG>0.15.0: hygiene sweep and reviewer-opinion annotations. Hygiene: correct "Three principles" → "Five principles" at line 59 (Principle 4 added v0.5.0, Principle 5 added v0.6.0); update overview "20 items" → "21 items" with note about Q #21's placement in Deferred; add Decisions-reached framing paragraph clarifying "adopted" semantics and pointing at Decisions 5 and 8 as examples of adopted-direction + implementation-specifics pairings. Reviewer opinions: append a compact "Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open)" block to each of Open Qs 1-20 and to Open Q #21 in the Deferred metadata section. For Qs 2, 12, 14, 18 (already resolved as Concerns B, F, D, C respectively), annotation notes the reviewer input as load-bearing for the existing resolution while leaving implementation-level sub-questions open.
0.14.0: resolve Concern F from the GT-Design lead review memo — shadow / offscreen / probe/trace compatibility promoted from open risk to V3 release gate. Rewrite Open Q #12 with explicit release-gate framing. Enumerate six gate criteria: canonical shadow fixtures, offscreen/slide fixtures, probe snapshots, trace expectations, GT-Design integration fixtures, role-aware lowering correctness. Establish that V3 does not ship without green on each criterion for the designated critical set. Cross-link to Concern B (critical-set carve-out is the evaluation mechanism — B is infrastructure and workflow, F is the gate criteria the infrastructure evaluates). Retain implementation-level sub-questions (per-criterion tolerance spec, gate ownership, whitelist discipline, recapture cadence, escalation path) with default leans. This is the final load-bearing concern from the lead review memo's six-item list (A-F).
0.13.0: resolve Concern E from the GT-Design lead review memo — scene-layer pipelines written as if half-landed. Retitle Decision 5 from "adopt" to "direction adopted, dedicated V3 implementation track." Add Status paragraph clarifying that the live `RaSceneLayer` schema has no `pipeline` field and that adopting the direction commits V3 to real schema/runtime feature work. Update "Two levels of scoping" bullet 1 to reference the new `pipeline: Option<VfxPipelineConfig>` field (additive, on the post-rename `VfxSceneLayer`). Update line 441 bullet to future-tense ("will gain a new per-layer `pipeline` field"). Replace "Architectural guidance" subsection with "V3 implementation track" enumerating schema extension, parser/deserializer, validator, compositor composition-order, per-layer caching, trace taxonomy, migration fixtures, and documentation work items. Add "Decisions the implementation track will need to resolve" subsection with default leans for precedence, hint visibility, blend mode, and empty-pipeline semantics. Add "What this decision adopts and what it does not" subsection separating adopted direction from non-binding defaults. Cross-link migration fixtures to Concern B's critical-set carve-out (layered recipes are natural critical-set candidates because rendering equivalence is most at risk when composing multiple pipelines per frame).
0.12.0: resolve Concern D from the GT-Design lead review memo — tokenization/runtime-binding conflation on wrong axis. Flip the primary organizing axis from domain (text vs structured) to temporal lifetime (load-time vs per-frame). Load-time Substitutions and per-frame RuntimeBindings are two distinct API surfaces at the tui-vfx-recipes boundary; each handles text and structured domains internally. Optional RecipeContext umbrella wrapper available for ergonomic one-call passing. Rewrite Decision 6 "Relationship to tokenization" paragraph to align ParamValue::Constant ↔ Substitutions and ParamValue::RuntimeBinding ↔ RuntimeBindings. Rewrite Open Q #14 "Text vs structured — the fundamental split" subsection as "Load-time Substitutions vs per-frame RuntimeBindings — the fundamental split" with rationale (lifetimes determine failure modes; live-code evidence; usage concentration on the 2×2 diagonal). Update Open Q #14 sub-questions to reflect per-surface failure models, asset-resolution home, and procedural-params composition. Split token-contract-discovery into per-surface contracts (requires_substitutions, requires_bindings) with per-surface introspection APIs. Update Decision 8 builder-interaction sub-question. Update movie-composer deferred cross-reference to describe per-scene Substitutions at scene load and per-scene RuntimeBindings for timeline-driven animation.
0.11.0: resolve Concern C from the GT-Design lead review memo — role-domain overload. Split "role" into four distinct types: RoleTag (per-cell render role, existing in tui-vfx-types, unchanged), ThemeRole (Scope variant renamed from Scope::Role in Decision 1 — now Scope::ThemeRole), RoutingRole (new V3 step-level behavior hint), SurfaceIntent (new V3 recipe-level hosting hint). Rewrite Open Q #18 around the four-type split with consumer/value-set distinction, motivating use cases per type, hints-vs-contracts discipline common to both new types, vocabulary-collision note, and scene-layer role_tag audit note. Update Decision 1 summary (line 164) and body (line 253) to reflect ThemeRole rename. Update Deferred movie-composer cross-reference to call out SurfaceIntent explicitly under Principle 5 framing. Update metadata section relationship-to-#18 paragraph to reference the two new types. Four-type split rather than the reviewer's three-axis grouping because step-level and recipe-level hints have different consumers and don't collapse cleanly.
0.10.0: resolve Concern B from the GT-Design lead review memo — migration strategy split-brain. Rewrite Open Q #2 to describe the three-phase Curate → Re-author → Validate model with explicit prereq (validator infrastructure, mechanical translator for carve-out only), mainline-corpus track, and critical/fixture carve-out track. Rewrite Recipe migration workflow section in Deferred to align with Open Q #2 and document per-phase implementation mechanics. Correct stale "200-300 recipes" count; reference authoritative inventory step as blocking prerequisite. Phase ordering (curate-first) is deliberate: reduces problem space before translation, makes re-authoring a briefing-infrastructure forcing function rather than a JSON-reshape task, honors the library's AI-assisted-authoring-as-primary-pathway framing.
0.9.0: resolve Concern A from the GT-Design lead review memo — ParamValue<T> structural contradiction. Split step-parameter inputs into ParamValue<T> (external value sources, 3 variants: Constant / RuntimeBinding / SignalGraph) and HintRef<T> (pipeline-internal step-output references, Decision 7), composed at field sites via StepInput<T> = ParamValue<T> | HintRef<T>. Revise Constraint-vs-permissiveness bullet; revise Decision 6 decision statement and "authors learn one mechanism" bullet; rewrite Decision 7 "Interaction with signal-driven parameters" subsection to declare the split and drop the ParamValue::StepOutput language; update Open Q #16 preamble to reference HintRef<T>.
0.8.1: add distribution-and-packaging deferred-design section naming the embedded-vs-disk-vs-hybrid design space; note V3 loaders / fragment resolvers / tokenization APIs must accept byte-source abstractions (not assume filesystem) to preserve the option.
0.8.0: add recipe migration workflow + recipe metadata fields sections under Deferred; clarify architectural home of named compositions (Tier 1 Rust factories, Tier 2 theme-scoped fragments, Tier 3 app-scoped fragments) inside Decision 2 so recipes compose freely across tiers.
0.7.0: add "V3 is a clean break" framing under Why Now; revise Decision 2 (primitives are default; named compositions earn their place, not backwards-compat sugar); revise Open Q #2 (clean cutover preferred over long compatibility shim); revise Open Q #15 (lean comprehensive vocabulary refresh); sharpen the rocketsplash rename from "pending decision" to "just do it"; revise Open Q #14 framing to drop load-time/runtime compatibility hedging.
0.6.0: add Principle 5 (meaning-low-policy-high), Decision 8 (canonical upstream semantic seam), Open Q #19 (Preview naming), Open Q #20 (surface-identity vs substrate), feedback cross-reference section; strengthen "why" rationale across Decisions 1-7 and key Open Qs; add constraint-vs-permissiveness design discipline note.
0.5.0: add Principle 4 (authoring-affordance preservation, Intention 51); refine Decision 5 (source taxonomy, rocketsplash formats, logo example) and Decision 6 (text-vs-structured tokens/bindings split); refine Open Q #14; add Open Q #17 (primitive library / $use) and #18 (role tags); add retrospective-corrections deferred section.
0.4.0: MAJOR expansion integrating the flag-animation PRD and the broader library reframe. New guiding principles (Pipe-culture chain-ability, Widgets-and-the-grid perspective). New Architectural framing section (ecosystem-agnostic layer validation; mixed-signals upstream home; two-level chaining clarification). Decisions 5–7 added: scene layers with per-layer pipelines; ParamValue with Constant / RuntimeBinding / SignalGraph variants formalizing BindableValue and the flag-animation PRD's SpatialSignalSpec; step output hints as first-class V3 primitive. Open questions expanded with #13 PhaseSet granularity, #14 tokenization ownership (app-layer vs tui-vfx-boundary API), #15 vocabulary refresh scope, #16 cross-step hint resolution rules. Deferred-design section expanded with movie-composer territory and dynamic recipe formalization.
0.3.0: add Decision 4 — rename Ra* prefix to Vfx* across tui-vfx-recipes wire-format types. Bundled with V3 as one rename event.
0.2.0: rename to tui-vfx-v3-upgrade-plan.md; elevate the William Morris "useful or beautiful" principle to a top-level guiding philosophy section with theatrical/whimsical carve-out.
0.1.0: initial draft. Captures direction only.</CLOG> -->

# TUI-VFX V3 Upgrade Plan

> **Status: draft — direction only.** No implementation schedule, no migration tooling spec, no committed V3 schema grammar yet. This document captures *what we've decided* and *what still needs to resolve* before a real plan can be written.

## How to read this document

The plan is organized to make *why* we're making each decision as visible as *what* the decision is. Future readers (humans and AI alike) will not have access to the conversation that produced this plan; the rationale must travel with the text. Each Decision includes: what it is, why this shape was chosen over alternatives, how it composes with other V3 decisions, and how it's envisioned at the authoring/consumer surface. Each Open Question names both the question and what's at stake in choosing different answers.

The document structure:

1. **Guiding philosophy (5 Principles)** — the durable framings that outlast any single decision.
2. **Constraint vs permissiveness** — the design discipline that explains why some V3 decisions constrain tightly while others are deliberately permissive.
3. **Architectural framing** — layer model, ecosystem positioning, upstream/downstream seams.
4. **Why now** — the concrete drivers that prompted V3 instead of incremental changes.
5. **Decisions reached (1–8)** — structural choices with "adopted" direction; each carries rationale.
6. **Shape sketches** — flat-vs-tree JSON comparisons for three representative cases.
7. **Open questions** — 21 items needing resolution before implementation starts, each with stakes explained. Open Q #21 (recipe metadata) is introduced in the Deferred-design section alongside the metadata field proposal; it is non-blocking for V3 core.
8. **Deferred design rounds** — things that don't block V3 but V3 decisions must not foreclose.
9. **Appendix workflows** — shader audit, corpus curation, structural translation, executed in a future session.

## Feedback cross-reference — mapping external concerns to plan coverage

A developer working daily with gt-design produced a weak-seams review (at `feedback/2026-04-21-gtd-tui-vfx-weak-seams-feedback.md`) surfacing seven concrete seams where the current system can still drift. This plan addresses each of them either directly or by naming the explicit work item:

| Weak seam | V3 coverage |
|---|---|
| 1. Duplicated semantic conversion in `gtd-ratatui/src/recipes/{item,planner,player}.rs` — the `config.shadow` miss pattern | **Decision 8** (canonical upstream semantic seam) names this as structural V3 work |
| 2. `RecipeSceneCanvas` identity too generic — overloaded between substrate and family-specific surface | **Open Q #20** (surface identity vs neutral substrate) |
| 3. Labs honest but not fully faithful — mix product demo + teaching + debug roles | Principle 4 (authoring-affordance preservation) provides the governance lens; primarily a gt-design governance issue, not V3-structural |
| 4. Policy distributed across layers — shadow/elevation outcomes are products of many policies | Principle 5 (meaning-low-policy-high) is the organizing principle; Decisions 1, 5, 7 consolidate V3-side pieces |
| 5. Tests encode implementation detail — identity assumptions, trace names, route labels | Open Q #9 (validator redesign) — contract-based testing as a V3 principle for new validator infrastructure |
| 6. Naming doesn't match abstraction — "preview" naming for canonical engine | **Open Q #19** (Preview naming); Decision 4 (Ra→Vfx rename) addresses one instance |
| 7. Intake layer complex — raw/resolved/template-backed/runtime-param-injected pathways | Open Q #14 (tokenization), Open Q #17 (fragment composition), Decision 6 (ParamValue unification) consolidate pieces |
| 8. Key seam conventional rather than structural | **Decision 8** formalizes the upstream canonical-builder seam |

The developer's proposed principle — *"Meaning should live as low as possible. Policy should live as high as necessary."* — is elevated to **Principle 5** below because it generalizes beyond the seams it was surfaced to address.

## Guiding philosophy

Five principles shape V3 design and will outlast the specific schema decisions below. They are the durable framing; the schema changes are how we apply the framing to the current surface. Principles 1–3 (Morris, Pipe-culture chain-ability, Widgets-and-the-grid) were added in the initial plan draft; Principle 4 (Authoring-affordance preservation) was added in v0.5.0 codifying Intention 51; Principle 5 (Meaning low, policy high) was added in v0.6.0 from the weak-seams feedback session.

### Principle 1 — The Morris principle

> *"Have nothing in your houses that you do not know to be useful, or believe to be beautiful."*
> — William Morris, 1880

The V3 recipe corpus — and every recipe added to the library after it — must pass the Morris test: **useful or beautiful, and ideally both.** This is the filter that prevents corpus bloat, keeps the library comprehensible to humans and AI at scale, and keeps every shipped recipe worth the maintenance burden it carries.

Definitions:

- **Useful:** demonstrates a specific capability not shown elsewhere; covers a real use case; canonical example of a pattern; teaches a concept recipe authors need to see; provides proven parameter tuning for a design intent; serves as a test fixture for infrastructure.
- **Beautiful:** aesthetically excellent; refined design moment; carries thematic meaning tied to a theme's identity; pushes creative boundaries worth preserving as reference; represents a peak of its era of library development.
- **Ideally both:** most shipped recipes should earn their place on both axes. Single-axis entries are accepted but the bar is higher.

**Carve-out: the theatrical showcase set.** A deliberately small, explicitly-scoped collection of recipes exists specifically to *show what the system can do when pushed* — whimsical, theatric, technically ambitious, or creatively indulgent pieces that may not pass the strict utility test but earn their place by demonstrating the library's ceiling. This set is intentionally bounded, lives in its own namespace (candidate: `recipes/showcase/`), and is not held to the same "covers a use case" standard as the main corpus. The point is not to eliminate whimsy — it is to contain it so whimsy doesn't accrete across the whole library.

This principle applies to:

- **Named shader compositions** (Workflow A — does this named shader encode useful/beautiful design judgment worth the name, or is it redundant with the primitive form?).
- **Recipes in the main corpus** (Workflow B — does this recipe teach something no other recipe already covers, or move someone aesthetically in a way no sibling does?).
- **Patterns added to the primitive catalog** (future decisions — does this Pattern variant serve a real expressive need, or is it a speculative addition?).
- **Any future compositional layer** (theme presets, motion intents, scene fragments — the same filter applies).

The Morris principle is not a one-time audit frame. It is the ongoing filter the library must run against itself to stay comprehensible, shippable, and loved by the people who work with it. Every addition should answer the same question: *useful, beautiful, or both?*

### Principle 2 — Pipe-culture chain-ability

Morris filters *what earns a place*. Pipe culture shapes *how primitives relate to each other*.

V3 borrows deliberately from the Unix shell-pipe tradition. Each pipeline step is a self-contained primitive with a clear scope / phase / payload contract. Steps produce named outputs (hints) using a uniform output vocabulary; steps that want to react to other steps bind to those outputs by name. Composition is declared at use-site in the recipe tree — no pre-defined pipelines are baked into the library. Every intermediate value between steps is a first-class thing you can inspect, probe, log, or redirect.

Concrete consequences:

- **Every step is self-contained.** No step knows about the internals of any other step. A `DisplacementShade` shader doesn't know which sampler produced its displacement hint; it just binds to "the displacement channel" for the layer.
- **Uniform output/input vocabulary.** Step hints live in a defined namespace (`displacement`, `sampled_color`, `cell_density`, etc.), not per-step ad-hoc naming. Adding a new step type doesn't require touching the hint namespace; bindings work by discovering producers.
- **Composition happens at authoring-time.** The tree schema is the compose-at-use-site pattern. Authors wire outputs to inputs by declaring which bindings each step consumes.
- **Inspectable between stages.** The probe/trace infrastructure already partially supports this; V3 makes it canonical — any intermediate hint should be dump-able.

This is important enough to be structural: the V3 schema must make named step outputs a first-class concept, not a retrofit.

### Principle 3 — Widgets and the grid, not just notifications

The library grew from toasts. That origin still shows in the vocabulary (`auto_dismiss_ms`, `anchor`, `continuous`, `enter/dwell/exit`), the mental model many authors (including me, working on this plan) default to, and the examples and SKILLS reference. But the *capability surface* has already generalized — splash uses the exact same recipe envelope as toasts; the PRD's flag animation is a scene-layer composition with signal-driven motion; the ambient-halo exploration is a recession-field modulation; relative light is an ambient backdrop. None of these are notifications.

V3 design must be reviewed against widgets, grid-level effects, scenes, transitions, and composed movies — not just notifications — before landing. Concretely:

- A hover state on a widget is not a notification but may use enter-phase-like vocabulary.
- A splash screen is a one-shot composed scene, not a long-lived toast.
- A theme-swap transition is a whole-grid effect with no notification anywhere in the picture.
- A tutorial overlay is a layered scene with scoped highlighting and staggered copy.
- A training-demo movie is a composition of recipes across time, not a single recipe.

When a V3 decision feels crisp for notifications but awkward for any of these broader use cases, the decision is wrong or incomplete. The correct response is not to paper over with app-side glue — it is to widen the V3 vocabulary so the general case is first-class. The notification-shaped terms that remain should either be deliberately kept (e.g., `enter/dwell/exit` may survive if it names something genuinely general about arriving/present/departing) or renamed to neutral terms.

### Principle 4 — Authoring-affordance preservation

Any consolidation mechanism V3 introduces — `template + variants`, primitive libraries, `$use` fragment composition, bundler manifests, or any future aggregator — must preserve individual-item addressability for debug, preview, and reference use cases. **The file path and unit identity are UX contracts with tooling, not just storage conventions.**

This principle is the hard-learned lesson from the `recipes/easing/easing_family.json` retrospective: a 26-recipe → 1-file consolidation served `load_all` cleanly but regressed the demo app's file-picker (individual easings became unselectable). The optimization served programmatic consumers at the expense of debug/preview consumers.

V3 consolidation mechanisms must ship *together with* their tooling counterparts. A `template + variants` file is not done landing until the demo app, validator, probe, and trace all understand its expansion and expose each variant as a selectable, addressable item (e.g., `easing_family.json#back_out`). Debug / preview / reference recipes stay as individual files by default. Metadata declares intended consumption (`programmatic` / `individual_preview` / `both`).

This principle is codified as **Intention 51** in `steering/INTENTIONS.md` (version 0.52.0) and applies across every consolidation V3 introduces, not just the specific cases we've talked through. Future additions to the schema (primitive libraries, fragment composition, bundler formats) must pass its filter.

Related: Intention 44 names *when* to extract shared primitives (rule of three). Principle 4 / Intention 51 names *how* any extracted primitive must behave to preserve authoring affordances.

### Principle 5 — Meaning should live as low as possible; policy should live as high as necessary

Every V3 decision should be evaluated against which *layer* owns the concept being encoded. The durable shape is:

- **Meaning** — *what a recipe field does, semantically.* Lives as low as possible: mixed-signals for signal math, `tui-vfx-*` for recipe/pipeline semantics, foundation libraries for their domains. Meaning is the stable contract everyone depends on.
- **Policy** — *how a design system or product applies meaning in context.* Lives as high as necessary: gt-design for theming decisions, consumer apps for surface identity, product layers for family-specific behavior. Policy is where product personality lives.

Why this matters: when meaning leaks upward into policy layers, the same concept gets re-encoded in multiple places (the `config.shadow` miss is the canonical example — an upstream additive field needed rethreading through GTD's parallel semantic conversion). When policy leaks downward into meaning layers, primitives develop product-specific assumptions that block their reuse (a Diffusion shader that presumes toast lifecycle is a bad primitive).

Practical tests for placing a new concept or field:

- "Does this name *what a recipe does* regardless of who's consuming it?" → meaning → lower layer.
- "Does this name *how a product styles or hosts it*?" → policy → higher layer.
- "Could two different consumers legitimately disagree about this?" → if yes, policy; if no, meaning.
- "If this changes, do all consumers need to change?" → if yes, meaning (they're downstream of a semantic update); if no, policy (only this consumer cares).

This principle generalizes Intention 40 (foundation libraries own domain expertise), Intention 49 (recipe authoring truth upstream, display truth in factory), and the V3 architectural framing (signals go upstream into mixed-signals, V3 consumes them). It's explicit because it needs to actively guide decisions during implementation, not be reconstructed from intuition each time.

Framing credit: proposed by one of the gt-design developers during a weak-seams-review session (see `feedback/2026-04-21-gtd-tui-vfx-weak-seams-feedback.md`). The principle validates several existing V3 positions and gives future decisions a compass.

## Constraint vs permissiveness — the design discipline

Several V3 decisions constrain tightly (closed algebraic types, deny-unknown-fields, validator-rejected shapes) while others are deliberately permissive (any field can be tokenized, step outputs use an open hint namespace). This asymmetry is intentional and follows a rule:

> **Constrain where correctness depends on closed semantics; be permissive where flexibility helps authors and the cost of "wrong" is bounded.**

Examples of each:

- **Unified Scope (Decision 1) is a closed algebraic type** with typed variants (Area, Channel, Content, Role, Custom) plus algebraic combination (And/Or/Not). Why closed: because the static-vs-dynamic analyzer pattern only works when the scope vocabulary is enumerable (static scopes cache as bitmasks; open scopes can't). Because validators need to catch nonsensical combinations statically. Because a bounded authoring vocabulary is easier for humans and AI to reason about. Escape hatch: `Predicate(fn)` for genuinely custom cases.
- **Universal tokenization (Open Q #14) is permissive** — any string field can contain `{{tokens}}`, loader coerces at parse time. Why permissive: because restricting "which fields are templated" via per-field opt-in flags is structural overhead for no real authoring gain. The alternative (mark each field as templated) doubles the schema surface with no expressiveness win. The risk of unresolved tokens is bounded by strict-mode validation, which catches missing substitutions loudly at load time.
- **Scene-layer source kinds (Decision 5) are a closed enum** (Text, Image, Procedural, Card). Why closed: adding a new source kind requires runtime support (rasterizer, validator, format loader) — it's not just a schema tweak. Extension through the `Procedural` generator registry instead (procedural `source_id` is open-string for generator names, with the registry vouching for each).
- **Step output hints (Decision 7) use a defined namespace** (`displacement`, `sampled_color`, `cell_density`, etc.). Why bounded but not fully closed: downstream bindings need name stability (a step that claims to produce `displacement` must always produce that hint shape), but the namespace itself can grow additively as new hint kinds are needed. Not a closed enum; not fully open either.
- **`ParamValue<T>` and `HintRef<T>` (Decisions 6 and 7) are two related closed types**, composed at field sites via `StepInput<T> = ParamValue<T> | HintRef<T>`. `ParamValue<T>` has three variants (Constant, RuntimeBinding, SignalGraph) covering external value sources; `HintRef<T>` references named step outputs within the same pipeline evaluation. Why closed on both and why kept distinct rather than collapsed into a single four-variant `ParamValue<T>`: the two types have different resolution paths (external substitution/evaluation vs producer lookup against the hint namespace) and different validator work (binding contract discovery vs tree-walk producer verification), and they live at different layers per Principle 5 (external value sources are app policy flowing in; step-output refs are meaning flowing within the pipeline). Fields that only make sense with one side narrow to the appropriate type. The signal-graph variant itself is open via `mixed-signals` composition, so expressiveness isn't limited — just the outer enums.

Future V3 decisions should surface this discipline explicitly: if a proposal adds a closed type, it should answer "why is this correctness-load-bearing"; if it adds permissiveness, it should answer "why is the cost of 'wrong' bounded here." Getting this balance wrong pulls the library toward either excessive ceremony (every field rigidly typed past utility) or magic-soup (no invariants, debugging becomes guesswork).

## Summary

Migrate the `tui-vfx-recipes` authoring schema from its current flat shape (per-phase slots with asymmetric multiplicity and scoping rules across element types) to a uniform tree schema where every pipeline step carries the same shape — a scoped, phased, composable atom. Internally, decompose spatial shaders into a `ColoredOverlay` + `Pattern` axis model while preserving today's named-factory JSON shapes as sugar so backwards compatibility is a load-time concern, not an authoring-surface break.

Three distinct but related changes, with the intent of landing all three in a single schema version bump (V3) rather than fragmenting the migration:

1. **Unified `Scope` primitive** carried on every step (closed algebraic type: area / channel / content / theme-role / custom / And/Or/Not composition).
2. **Pattern-as-separable-axis** as the internal shader model (`ColoredOverlay { color, pattern, intensity }` with `Pattern` as an open enum of spatial distributions), with named factories (`Diffusion`, `ConcealedLight`, etc.) retained as JSON surface sugar.
3. **Tree authoring schema** replacing flat `pipeline.{mask, filter, sampler, styles}` slots with a recursive `Step | Sequence | Parallel` structure.

## Architectural framing

### The ecosystem-agnostic layer earns its place

`tui-vfx` deliberately renders to a grid and maps to ratatui at the final stage rather than being ratatui-native. Earlier in plan development this looked like optionality that might never cash in. It does cash in — the movie-composer concept (see *Deferred-design territory* below) is a concrete use case where the grid-first architecture is the enabler, not overhead.

The mental model: **ratatui is *a* consumer of tui-vfx, not *the* consumer.** Future sibling consumers become possible without rewriting the compositor:

```text
 L5 consumers:    ratatui app  │  movie player  │  static renderer  │  wasm embed
                       ↓               ↓                 ↓                 ↓
 L4 adapters:    gtd-ratatui   │  gtd-movie*    │  gtd-static*      │  gtd-wasm*
                       ↓               ↓                 ↓                 ↓
 L2/L3:                  tui-vfx-recipes + tui-vfx-compositor (grid-first)
                                            ↓
 L1:                   tui-vfx-types, mixed-signals, mcu-terminal-color
```

(*Siblings beyond `gtd-ratatui` are hypothetical; just mapping the territory.*)

V3 design must preserve this. A decision that introduces ratatui-specific types into the pipeline vocabulary (e.g., a step binding to `ratatui::layout::Rect`) is a regression. Grid-first is honored; adapters translate at L4.

**Why this costs but earns its place even when we only have one L5 consumer today:**

The grid-first architecture has real costs: a `tui-vfx-types::Cell` that parallels `ratatui::buffer::Cell` (requiring conversion at the boundary), compositor work that can't use ratatui's native cell types directly, extra indirection in hot paths. For gt-design alone — our only current production consumer — this is overhead that doesn't visibly pay back. Earlier in plan development this looked like optionality that might never cash in.

The movie-composer use case is the concrete cash-in: a movie player doesn't need ratatui's widget/layout/event-loop machinery. With grid-first architecture, a movie-player binary can render directly to crossterm / stdout / a file / a wasm buffer without pulling ratatui into its dependency tree. Ship static binaries at a few hundred KB instead of MB. Enable adjacent uses: terminal recordings for docs, CI visual regression via grid diffs, wasm-embedded demos, SVG/PNG/SIXEL static export, training movies, documentation hero assets.

Three secondary benefits the agnostic layer also provides:

- **Clean test surface.** Compositor unit tests have no ratatui dependency. Grid assertions are easier to reason about than ratatui-buffer assertions with their mod-state.
- **Forced layer discipline.** Intention 40 is about using foundation libraries instead of rolling your own. Part of why we *can* honor that cleanly is that tui-vfx's internals don't presume a ratatui world.
- **Optionality.** If gt-design or any future product ever wants to render to something other than ratatui (web terminal renderer, custom compositor, embedded display), we have a seam.

The decision to pay the cost isn't abstract optionality anymore — it's validated by a concrete second consumer (movie-composer) that's plausibly imminent and architecturally natural. V3 must not regress this — every decision is evaluated for whether it leaks ratatui-specific types into pipeline vocabulary.

### mixed-signals is the upstream home for signal primitives

`/usr/projects/mixed-signals` was created deliberately near the start of the project as the canonical home for signal primitives. It has been stable for 4+ months and already carries the 1D composition / processing / noise / physics / envelope catalog (`Sine`, `Triangle`, `Ramp`, `Keyframes`, `Add`, `Multiply`, `Mix`, `Normalize`, `Remap`, `Clamp`, `ADSR`, `DampedSpring`, `SpatialNoise`, etc.).

**V3 consumes signals; V3 does not invent signals.** When V3 needs a capability that doesn't exist in `mixed-signals` today (the `SpatialSignalSpec` 2D-aware signal graph for the flag-animation PRD is the current driver), the correct response is to extend `mixed-signals` upstream, not to build a parallel signal surface inside `tui-vfx`.

This flips the preference stated in the flag-animation PRD (v0.3.0) where "Path A" (local `SpatialSignalSpec` in tui-vfx-compositor) was primary and "Path B" (upstream `Signal2d` trait in mixed-signals) was an opt-in follow-up. The V3 direction is that **Path B is primary**; Path A is a fallback only if upstream velocity genuinely blocks V3 delivery. This aligns with gt-design Intention 40: *"when a foundation library is missing a capability GTD needs, the correct response is to extend the foundation library — not to work around it with inline code."*

### Two levels of chaining live at different layers

Pipe-culture chain-ability (Principle 2) applies at more than one level, and V3 must keep them distinct:

1. **Signal-graph composition** — combining signals into composite waveforms (`Add(Sine, Ramp)`, `Multiply(wave, envelope)`). Lives in `mixed-signals`. Already works in 1D; V3 wants 2D-aware extensions upstream (see above).
2. **Pipeline-step chaining** — one V3 step's output feeding another V3 step's input (`DisplacementShade` reading a sampler's offset hint). Lives in the V3 schema as a first-class primitive. Treats signals as *one kind* of bindable value that can flow between steps, alongside other hints like displacement offsets, cell-density maps, and sampled colors.

Mixed-signals must not grow a "pipeline step" concept, and V3 must not duplicate signal-composition logic. The layering is clean; the plan must honor it.

## Why now

**V3 is a clean break. Backwards compatibility is not a constraint.**

`tui-vfx-recipes` and the `tui-vfx` family are published to crates.io but have not been promoted and have not been discovered — gt-design is effectively the only consumer. V3 is pre-audience work. That changes the shape of the right decisions:

- **No compatibility shim window is required.** V2 recipes do not need to keep loading under V3. The corpus of shipped recipes is internal; it migrates in one pass and V3 is the new floor.
- **Breaking changes are acceptable everywhere they improve the design.** "Since we're breaking things anyway" is a valid argument for bundling related changes (Ra→Vfx rename, "preview" naming, vocabulary refresh, schema tree restructure). "We'd have to maintain compatibility" is NOT a valid argument against doing the right thing.
- **Named-factory preservation for backwards compat has no weight.** If a named shader earns its place via encoded design judgment, keep it. If it doesn't, drop it. The "existing recipes use this name" consideration carries no weight because there are no external recipes to worry about.
- **Deprecation warnings and dual-path loaders are overhead we don't need to pay.** We can do the rename/rewrite once, update the corpus once, and ship V3 as the new baseline.

This framing explicitly replaces any prior reasoning in this plan that hedged toward preservation. If a decision in this document appears to favor compatibility over the right design, flag it and re-evaluate. V3 is the moment to do it right.

**When V3 ships, if and when external adoption grows, future versioning will need real compatibility discipline.** V4 → V3 migration, if V3 gains actual consumers, gets the care V2 → V3 does not need to carry. This context is time-scoped to V3; it is not a license for future generations to break things freely.

The migration drivers are not speculative — each one surfaced concretely during recent work:

- **Schema accretion is starting to bite.** The ember-felt debugging session exposed that `spatial_shader` is a deprecated-but-still-loaded field that silently compiles to `dwell_effect` only. Authoring AI (and me specifically) tripped on it. SKILLS.md-based mitigation would require documenting every asymmetry in perpetuity.
- **Comparative data supports the uniform pattern.** The `tachyonfx` architecture review showed that a unified scope primitive (`CellFilter`) with algebraic composition, paired with explicit composition containers (`sequence` / `parallel`) and method-chain propagation, is a proven shape at ~30K downloads of production use. We're not pioneering — we're adopting a validated pattern.
- **AI-assisted authoring is the primary composition pathway.** At the scale we plan to ship (large reference library + SKILLS.md + capability matrices + community extensions), AI reasoning reliability matters, and schema shape directly affects it through attention proximity, shape regularity, context-budget consumption, and error-recovery cost. Every mechanism pushes toward tree.
- **Ambient-halo and ember-felt both want this substrate.** Both explorations in `docs/internal/specs/relative-light-architecture.md` would compose more naturally against a tree schema with a unified scope primitive and Pattern-as-axis than against the current flat schema. Landing the schema first unblocks both.
- **Migration cost only grows.** Every recipe we ship in the current schema increases the V3 migration surface area. Deciding now and staging the migration is cheaper than deciding in six months with 3× the corpus.

## Decisions reached

Eight structural decisions with adopted direction. "Adopted" means the direction is committed for V3; implementation specifics may still be in flight for Decisions that carry their own track (notably Decisions 5 and 8, which name explicit sub-questions or implementation-track work). Any Decision whose title includes "implementation track" or "formalize during V3" has adopted-direction + implementation-time-specifics as an intentional pairing, not a hedge. The sub-questions inside each Decision are where implementation choices remain; the decision itself is not contingent on those sub-questions resolving in any particular direction.

### 1. Unified `Scope` primitive — adopt

Every step in the pipeline carries a `scope: Scope` field defaulting to `All`. `Scope` is a closed algebraic type with variants covering the targeting axes we currently express through scattered mechanisms:

- **Area:** `All`, `Outer(margins)`, `Inner(margins)`, `Rect(x, y, w, h)`, `RectExclude(...)`
- **Channel:** `Background`, `Foreground` (replaces `apply_to`)
- **Content:** `Text`, `NonEmpty`, `GlyphMatches(pattern)` (replaces embedded `GlyphStyle.rules`)
- **Theme-role:** `ThemeRole("primary")`, `ThemeRole("surface")` (theme-resolved at load; replaces `StyleRegion::BackgroundOnly`-style semantic targeting). Named `ThemeRole` rather than `Role` to keep this scope variant distinct from the other "role" concepts V3 distinguishes: the per-cell `RoleTag` in `tui-vfx-types` (`Background`, `Text`, `Border`, `Shadow`, etc.), the step-level `RoutingRole` (Open Q #18), and the recipe-level `SurfaceIntent` (Open Q #18). Four separate types by design — see Open Q #18 for the full framing.
- **Custom:** `Predicate(fn)` as an escape hatch for cell-level custom logic
- **Composition:** `And([...])`, `Or([...])`, `Not(...)` for algebraic combination

The scope is *uniformly attachable* — masks, filters, samplers, and style effects all accept it. This replaces today's situation where scoping lives in different mechanisms per element type and only styles scope cleanly.

**Unlocks:** currently-inexpressible authoring intents ("warm diffusion on the background, but only where there's no text beneath"; "four per-edge diffusion instances scoped to the recessed canvas, excluding the focused rect"). These are real asks from the ambient-halo and ember-felt explorations.

**Why this shape specifically — not closure-based predicates or open-string scopes:**

The closed algebraic type is deliberate. Three reasons it earns its constraint:

- **Performance via static analysis.** Tachyonfx's `cell_filter::analyzer` classifies filters as static (cacheable as bitmasks per area) or dynamic (evaluated per-frame), a pattern that only works because the scope vocabulary is enumerable. An open-closure scope would force everything to the dynamic path and lose the caching win. V3 inherits the same architectural lever.
- **Validator safety.** A closed enum lets validators catch nonsensical combinations (e.g., `Content(Text)` on a mask-reveal step where "text" doesn't exist before the cell reveals) at load time rather than at render time. Closure-based predicates are opaque to static analysis.
- **Authoring + AI comprehensibility.** A bounded vocabulary of scope variants is easier for humans to learn and easier for AI-assisted authoring to reason about without drifting. An open closure surface gives authors rope to hang themselves on and makes the capability space hard to survey.

The escape hatch (`Predicate(fn)` or equivalent) exists for the 5% of genuinely custom cases, but requires explicit Rust-side registration rather than arbitrary eval. This is the same pattern the compositor already follows for novel needs.

### 2. Pattern as separable axis — adopt as internal model, keep named-factory JSON sugar

Internally, spatial shaders decompose into `ColoredOverlay { color, pattern, intensity, apply_to }` where `Pattern` is a separate enum covering spatial distribution functions (`RadialFromCorner`, `LinearFromEdge`, `EdgeVignette`, `FourEdgeRadial`, `WaveHorizontal`, `Noise`, etc.).

**V3 ships with primitive-only authoring as the default surface.** Named compositions exist as a second surface, but that second surface starts empty and accrues entries only when a specific composition encodes design judgment worth preserving as a named contract. This is the direct application of Principle 5 (meaning low, policy high) combined with Intention 46 (library changes earn their place): named compositions are policy-layer conveniences that must justify themselves, not meaning-layer primitives that ship by default.

**Rationale — why not primitive-only, why not named-only:**

During plan development we considered three positions:

- **Named-only (current V2 state):** a flat catalog of ~27 named shaders. Discovery via enumeration; authors pick by name. Fails at scale — the user who commissioned the system has personally lost track of the catalog. When authoring, "what's available?" becomes "read 27 files and their variants." An authoring-surface problem that worsens with every added shader.
- **Primitive-only (no named compositions at all):** decompose everything to `ColoredOverlay + Pattern` and ship no named factories. Cleaner end state, but loses encoded design judgment — each existing named shader's specific parameter tuning represents someone's decision about what "warm diffusion" or "edge glow" should *look like*. A fresh re-authoring would re-litigate those decisions and likely drift.
- **Primitive-by-default with earned named compositions (adopted):** primitives are the authoring vocabulary; named compositions accrue one at a time, each with documented rationale for why the specific parameter tuning encodes design judgment worth locking in.

**The migration is not backwards-compat-driven. V2 recipes do not carry forward automatically.** Each existing named shader in V2 is evaluated during the shader-catalog audit (Workflow A in the sibling `tui-vfx-v3-upgrade-audit-workflow.md`) and classified as:

- **Trivial composition** — drop the name; the V3 recipe corpus re-expresses this as `ColoredOverlay + Pattern::X` directly.
- **Earned name** — preserve as a named preset in V3 with documented justification for its specific parameter tuning.
- **Primitive itself** — add to the V3 primitive catalog (may require a new `Pattern` variant or a new base shader type).

Going forward, new named compositions accrue deliberately with documented justification. No automatic promotion from "I used this pattern twice" to "it's a named composition"; each entry earns its place.

**Migration-side work (V3 cutover, one-time):**

1. Implement the decomposed internal model and the `Pattern` enum.
2. Run Workflow A across the shader catalog; classify each named shader.
3. Re-express the corpus using the three-way classification. Trivial compositions become primitive form in the recipe JSON; earned names get named-factory JSON entries in V3 with their design rationale documented; primitive-itself cases get new `Pattern` variants added.
4. V2 named shaders that aren't in the earned-names set do not ship in V3.

**Architectural home of named compositions — three tiers by scope:**

Named compositions are not a single type of thing. They live at different layers depending on scope, and V3 should be explicit about where each kind belongs. All three tiers resolve to the same internal representation at runtime — a recipe doesn't know which tier it hit — but they differ in authoring home, versioning cadence, and evaluation criteria.

- **Tier 1 — Library-level named compositions (Rust factories in `tui-vfx-style`).** Examples: `Diffusion`, `ConcealedLight`, `Glow`, `LinearGradient`. General-purpose compositions that span themes and earn library-stable API status. Authored in Rust as factory functions that produce internal `ColoredOverlay + Pattern` trees. JSON surface: `{"type": "diffusion", ...}` — deserializer calls the Rust factory. Versioned with the crate release cadence. Earns place by encoding engine-stable design judgment worth locking in as canonical API with specific parameter defaults everyone agrees on.

- **Tier 2 — Theme-scoped fragments (JSON files in the theme's directory, referenced via `$use`).** Examples (hypothetical): `grimoire/fragments/ember_warmth.json`, `harbor/fragments/hidden_rail_shell.json`. Theme-specific compositions that encode what a given theme's identity means. Not library-general. Authored as JSON fragments living with the theme. JSON surface: `{"$use": "grimoire/fragments/ember_warmth"}` — fragment loader (Open Q #17's `$use` primitive) resolves at recipe-load time. Versioned with the theme's own lifecycle, independent of engine version. Earns place by encoding theme-identity decisions worth naming within that theme's vocabulary.

- **Tier 3 — App-scoped fragments (JSON fragments in the app's resources, referenced the same way).** Examples (hypothetical): `my_app/fragments/brand_splash_effect.json`. Application-specific presets that aren't general enough for a theme or the library. Authored by the app. JSON surface: same `$use` pattern as Tier 2. Versioned with the app. Earns place by the app's specific authoring needs.

**Why three tiers and not one:**

- Engine-stable API has different versioning and release properties than theme-stable or app-stable. Conflating them into a single tier would force either everything into Rust (blocking theme authors from owning their own identity) or everything into JSON (losing compile-time validation for library-surface stability).
- The tiers match the "meaning low, policy high" principle (Principle 5). Library factories express meaning shared across all consumers. Theme fragments express a theme's policy. App fragments express an app's policy. Each lives at the right layer for its scope of reach.
- Fragment composability (Open Q #17) is what enables tiers 2 and 3 cleanly. Without the `$use` primitive, theme- and app-scoped compositions would either bloat into full recipes-that-extend (awkward) or get hard-coded into library Rust (wrong layer).

**What this means for recipe migration:**

As V2 recipes are migrated one-at-a-time (see the Recipe Migration Workflow below), each shader step gets classified per Workflow A. When a recipe uses a composition that's:

- Trivially primitive → re-expressed as raw primitive form in the recipe JSON.
- Worth library naming → created as a Rust factory in tui-vfx-style if it doesn't exist; referenced via the named-factory JSON form.
- Worth theme naming → created as a theme-scoped fragment file; referenced via `$use`.
- App-specific → created as an app-scoped fragment in the app's resources; referenced via `$use`.

The classification is a judgment call per recipe; the tier determines where the named composition lives, not whether it exists.

**Note on signal primitives:** when the `Pattern` catalog needs spatial-awareness (e.g., `Pattern::FourEdgeRadial` with per-edge sampled colors), the signal primitives that power it extend `mixed-signals` upstream rather than living inside tui-vfx. See *Architectural framing → mixed-signals is the upstream home for signal primitives* above. This flips the preference in the flag-animation PRD v0.3.0.

**Note on "named compositions earn their place."** V3 launches with primitive-only authoring as the default surface. Named compositions exist as a second surface but start *empty* and accrue entries only when a specific composition encodes design judgment worth preserving as a named contract. No auto-porting of V2's named shaders — each existing named shader is evaluated during the shader-catalog audit (see Workflow A in the audit-workflow doc) and classified as trivial composition (drop the name) / earned name (preserve with documented rationale) / primitive itself (add to primitive catalog, possibly with a new `Pattern` variant). The principle applies going forward too: new named compositions accrue by deliberate decision with justification, not by drift-accumulated convenience.

### 3. Tree authoring schema — adopt

Replace flat `pipeline.{mask, filter, sampler, styles}` with a recursive structure:

```
Pipeline  ::= Step | Sequence(Vec<Pipeline>) | Parallel(Vec<Pipeline>)
Step      ::= { kind, scope, phase, payload }
kind      ::= Mask | Filter | Sampler | StyleEffect | Shader
phase     ::= Enter | Dwell | Exit | All
```

**Containers propagate.** `Parallel([a, b, c]).with_scope(Background)` applies the scope to every child. This is the compounding win that makes the tree shape more than a cosmetic change: scope-per-step without container propagation would be verbose at the call site; with propagation it's as concise as the current flat form for common cases.

**Rationale consolidated:**

- *Schema regularity:* one shape for every step instead of four parallel shapes (mask/filter/sampler/style) with asymmetric multiplicity and scoping.
- *Migration tractability:* uniform step shape is trivially migratable across future schema versions; asymmetric flat slots compound migration cost per recipe.
- *AI reliability at scale:* proximity-weighted attention benefits from semantically-related elements being structurally adjacent; shape regularity improves generalization; constrained generation works better on tight, uniform schemas.
- *Error-recovery cost:* tree-shape errors point closer to their cause than flat-shape errors routed through asymmetric slots, which reduces the remediation tax per authoring iteration.
- *Ecosystem contract:* authoring format is a contract with everyone who will ever touch a recipe, not just the current workflow. Third-party theme authors, community recipes, future tooling all benefit from a regular structure.

**Why now, and why not just build a viewer as we first considered:**

During plan development we seriously considered whether a tree-rendering viewer over the existing flat JSON would capture most of the comprehension win at a fraction of the refactor cost. The case against tree authoring was strong: AI-assisted authoring is the primary pathway, SKILLS.md infrastructure can paper over flat-schema asymmetries, per-recipe authoring ergonomics are handled by Claude rather than raw JSON editing, and tree-shape would add verbosity to simple recipes.

The case *for* tree authoring that won the argument:

- The cognitive-load argument for AI is real but narrower than "tree necessary." It's "tree reduces a specific error class at cross-element boundaries." Demonstrated empirically in this very plan's development: a confident AI claim about phase semantics was wrong because the flat schema's deprecated `spatial_shader` field compiles to `dwell_effect` only — a footgun a tree schema wouldn't have. SKILLS.md could document this, but documentation is an ongoing tax proportional to the schema's asymmetry count.
- At library scale (500+ recipes shipping with themes + third-party extensions), the contract with non-Claude consumers matters. A new theme author, a marketplace recipe, a future tooling layer — all of these inherit the schema, and flat-schema asymmetry becomes a perpetual tax paid by everyone who touches it.
- Migration cost compounds with corpus size. Flat V2 → tree V3 is less migration per recipe than flat V2 → flat V2.5 → flat V2.6 → tree V4 would be. The one-time cost now is cheaper than amortizing it across multiple partial evolutions.

A viewer is still valuable and worth building (captured in Open Q #10), but as a comprehension aid on top of tree authoring, not a substitute for it.

**Sensible defaults keep simple recipes simple.** `scope: All`, `phase: All`, `combine: Chain`, and the named-factory shortcuts mean a basic fade-in toast doesn't require the full atom vocabulary at the authoring surface.

### 4. Rename `Ra*` prefix to `Vfx*` — adopt, bundled with V3

The `Ra*` prefix on every wire-format type in `tui-vfx-recipes` is archaeology from the library's original working name "Ratanimate" — a ratatui-only notification-animation project that subsequently generalized into the terminal-wide `tui-vfx` family. The prefix survived the generalization because a rename was costly at the time. It no longer earns its place: the name it refers to is dead, and it provides grep-anchor / disambiguation value only by accident of no alternative having been chosen.

**Decision:** rename the prefix across the full `Ra*` surface to `Vfx*`.

Scope of the rename:

- `RaRecipeConfig` → `VfxRecipeConfig`
- `RaPipelineConfig` → `VfxPipelineConfig`
- `RaStylePipelineConfig` → `VfxStylePipelineConfig`
- `RaMaskConfig` → `VfxMaskConfig`
- `RaFilterConfig` → `VfxFilterConfig`
- `RaSamplerConfig` → `VfxSamplerConfig`
- `RaStyleEffect` → `VfxStyleEffect`
- `RaBaseStyle` → `VfxBaseStyle`
- `RaClock` → `VfxClock`
- `RaContinuousConfig` → `VfxContinuousConfig`
- `RaSceneConfig` → `VfxSceneConfig`
- `RaLifecycleConfig` → `VfxLifecycleConfig`
- `RaContentConfig` → `VfxContentConfig`
- (full inventory to be enumerated during the rename planning pass; the above is illustrative, not exhaustive)

**Rationale:**

- *Prefix earns its place.* Consistent with gt-design's Intention 48 three-test criterion: wire-format types that flow across crate boundaries benefit from prefixes because grep-anchor / disambiguation / crate-identity are genuinely load-bearing for code review, debugging, and API-surface navigation. `rg Vfx` is the analog of `rg Gtd` in gt-design. Dropping the prefix entirely loses that property without a compelling gain — 3 chars is a fair tax for the context it preserves.
- *The specific choice of `Vfx` over `TVfx` or other alternatives:* `Vfx` is pronounceable ("vee-eff-ex"), aligns with the crate family name (`tui-vfx-*`), and is shorter than `TVfx` for no loss of information (the "t" in `TVfx` would only disambiguate against another VFX system we don't have and aren't building).
- *Bundled with V3 as one rename event.* Deferring to a separate migration would mean two rename events for the same consumer base — worse total cost, not better. V3 is already rewriting the recipe-authoring surface; the prefix rename rides along at minimal marginal cost.

**Bounded cost.** The rename is mechanical — a disciplined search-replace across `tui-vfx-recipes/src/`, plus updates to:

- Downstream consumers (`tui-vfx-trace`, tools, validators, probes, demos)
- gt-design consumers (`crates/gtd-ratatui/src/splash/`, `crates/gtd-ratatui/src/recipes/`, `crates/gtd-factory/src/`)
- Docs (recipe-authoring guides, SKILLS references, generated API docs)
- Published crate's public API (breaking change — bundled with the V3 breaking change for recipes; SemVer bump is already required)

No recipe JSON files are affected — type renames are Rust-side only; the recipe wire format (JSON) continues to use its existing field names.

**Why a prefix at all, and why `Vfx`:**

The `Gtd*` prefix convention in gt-design (Intention 48) establishes the rule: prefix wire-format and contract types to provide grep-anchor / disambiguation / crate-identity value. The same logic applies to `tui-vfx-recipes` wire-format types — `rg Vfx` becomes as useful as `rg Gtd` is today. Three chars (`Vfx`) is a fair tax for the context-preserving property. `TVfx` adds no information beyond `Vfx` and costs an extra character; dropping the prefix entirely loses the grep-anchor. `Vfx` is also pronounceable ("vee-eff-ex"), which matters for how people refer to the library in speech. Bundling with V3 means one rename event instead of two; deferring pays the "what does Ra mean?" cost in every new-contributor onboarding.

### 5. Scene layers carry their own pipelines — direction adopted, dedicated V3 implementation track

The V3 tree schema describes *pipelines within a layer*; V3 also needs to integrate with the scene-layer model introduced in Sub-plan B.1 (`RaSceneConfig.layers: Vec<RaSceneLayer>`) plus the per-layer pipeline extension proposed in the flag-animation PRD (primitive 1 at `/usr/projects/tui-vfx/PRD-FLAG-ANIMATION.md`).

**Status: direction adopted, feature not yet landed.** The live `RaSceneLayer` schema today carries `id`, `z`, `placement`, `source`, `role_tag`, `overflow`, and `visibility`. There is **no per-layer `pipeline` field** in the current schema. Adopting the scene-layer pipeline direction means committing to a real additive schema/runtime feature with its own implementation track inside V3 — schema extension, parser/deserializer updates, validator work, compositor composition-order decisions, trace-taxonomy additions, and migration fixtures. This is not a rearrangement of existing code; it is the feature work itself. The rest of this Decision describes what the feature looks like; the "V3 implementation track" subsection below enumerates the work items and the decisions the track will need to resolve.

**Two levels of scoping:**

1. **Scene-level** — which *layer* does a step apply to? Handled by a new `pipeline: Option<VfxPipelineConfig>` field on `VfxSceneLayer` (additive schema change; see implementation track below). Post-rename per Decision 4, the enclosing type is `VfxSceneLayer` rather than today's `RaSceneLayer`.
2. **Cell-level** — which *cells within a layer* does a step target? Handled by the V3 unified Scope primitive (Decision 1).

These are complementary, not competing. A recipe with multiple scene layers runs each layer's pipeline (if present) before the recipe-global pipeline. Layer-level pipelines can carry the full V3 tree structure; scope primitives inside those pipelines target cells within that layer.

**Why both levels are needed:**

- Scene layers are spatial composition — "this logo lives on top of this background." They carry their own content source (Text, Image, Procedural, Card) and their own placement.
- Unified Scope targets cells — "apply this shader to background-only cells within this layer."

Without scene layers, V3's unified Scope would have to carry geometry concerns it shouldn't (e.g., "apply to cells inside rect X, Y, W, H"). Without unified Scope, scene layers can't target cells within themselves cleanly.

**Scene-layer source taxonomy.**

V3 preserves the `RaContentSource` enum introduced in Sub-plan B.1 and expands it cleanly. The source kinds are:

- **Text** — string content, possibly with a content effect (typewriter, scramble, etc.). Uses the existing message field.
- **Image** — bitmap-style content from an asset. Multiple format variants: cell-coarse (currently `.rss`; rename to `.rsi` pending decision — see below) and braille-supersampled (`.rsb` per PRD primitive 4). An additive `kind` discriminator on the image source chooses the format; loader dispatches accordingly.
- **Procedural** — generated content via a named generator + opaque params. Schema: `RaProceduralSource { source_id: String, params: serde_json::Value }`. Generator registry is extensible; current candidates include `solid_color`, `noise`, `gradient`, `braille_dust`.
- **Card** — structured content (title + body + optional chrome) for common notification/modal shapes.

These source kinds are **orthogonal to** and **compose with** the V3 decisions:

- Procedural `params` are a `serde_json::Value` today. In V3, `params` values can themselves be `ParamValue<T>` (Decision 6) — so a procedural generator's color/density/seed can bind to app-supplied runtime values. Example: a `solid_color` procedural with `{"color": {"binding": "brand"}}` produces a background whose color updates from app state.
- Image sources can carry tokenized asset references (Open Q #14) — `{"image_name": "{{logo}}"}` resolves to bytes provided by the app's `Substitutions`.
- Scene layers will gain a new per-layer `pipeline` field (PRD primitive 1 — additive schema change enumerated in the implementation track below) that applies unified-scope pipeline steps to the layer's composed cells.

**Composability example — logo-on-flag-wave.** The flag-animation PRD's forcing-function recipe is a flag, but *every* primitive is general-purpose. The flag-wave sampler applies to any layer; the layer source determines what's waving. Swapping the flag image for a different braille image — the gt-design logo, a product wordmark, a seal, an emoji portrait — requires only swapping the `source` field:

```json
{
  "layers": [
    { "id": "backdrop", "source": {"type": "procedural",
                                    "source_id": "solid_color",
                                    "params": {"color": "#05050F"}} },
    { "id": "logo",
      "source": {"type": "image", "kind": "rsb",
                 "image_name": "{{logo}}"},
      "pipeline": {
        "sampler": {"type": "spatial_signal", /* flag-wave graph */},
        "shader":  {"type": "displacement_shade"}
      } }
  ]
}
```

Recipe stays structurally identical; the `{{logo}}` token + asset substitution determines *what* waves. This is the clearest user-facing demonstration of why Decisions 5 + 6 + 7 compose: scope handles *where*, layers handle *what*, Signal-graphs drive *how it moves*, and tokens/bindings select the *specific content*.

**Rocketsplash format family — naming (pending decision).**

Rocketsplash (sister project at `/usr/projects/rocketsplash`) is the authoring tool for the asset formats consumed by image sources. Current naming:

- `.rss` — cell-coarse image format. **Collides with RSS (Really Simple Syndication)** — file managers, IDEs, and search engines misinterpret it. Rename recommended to `.rsi` (rocketsplash image) while the format is still pre-release.
- `.rsf` — font atlas (braille-character fonts for splash/display use). Stays.
- `.rsb` — proposed new format for braille-supersampled images per PRD primitive 4 (2×4 per cell density with per-cell averaged RGB + dot bitmask).

The rename is not formally part of V3's Rust-side Ra→Vfx work, but it affects the scene-layer image source schema — V3's `Image` kind discriminator must enumerate the format family with the final names. Given rocketsplash is also effectively pre-release (consumed only by rocketsplash-rt), the rename carries negligible migration cost and should just happen: **`.rss` → `.rsi`, `.rsf` stays, `.rsb` is the new braille-supersampled format from PRD primitive 4.** The `.rss` → `.rsi` rename eliminates the RSS (Really Simple Syndication) collision that confuses file managers, IDEs, and search engines. V3's `Image` kind enumerates the final three-letter family (`.rsi`, `.rsb`, `.rsf`).

**V3 implementation track.**

Adopting per-layer pipelines as direction is not the same as the feature existing. The track inside V3 that delivers it has these work items:

- **Schema extension.** Add `pipeline: Option<VfxPipelineConfig>` to `VfxSceneLayer`. Additive — existing layers without a pipeline continue to compose as they do today. Serde defaults to `None`.
- **Parser / deserializer updates.** Recipe loader accepts the new field. JSON fixtures updated; V3 schema documentation regenerated.
- **Validator work.** Per-layer pipeline validates against the same V3 tree invariants the recipe-global pipeline does. Additional per-layer checks: scope coherence within the layer's geometry; `HintRef<T>` producer resolution scoped to the layer's own hint namespace (default per Open Q #16 — same-layer-only visibility); validator errors that point at the offending layer id rather than the recipe root.
- **Compositor composition-order decisions.** A recipe with N layers runs up to N+1 pipelines per frame (N per-layer, 1 recipe-global). Default precedence when both apply: layer pipeline runs first on the layer's composed cells; recipe-global pipeline runs second on the composited result (global wraps layer output). Blend mode at layer boundaries uses the layer's existing `placement` / `z` semantics.
- **Per-layer caching.** Compositor work per frame scales with layers × stages; per-layer cache keys are required to avoid redundant recomputation when only one layer's parameters changed. Tests already anticipate this pattern (`test_per_layer_cache.rs`). Cache-key design includes layer id + pipeline hash + relevant runtime bindings.
- **Trace taxonomy additions.** New `TraceEvent::LayerPipelineApplied { layer_id, stage }` and related events so `pipeline-validator --probe` can diff per-layer stages independently. Existing trace fixtures regenerated to include layer-level granularity where applicable.
- **Migration fixtures.** At least one fixture per source kind (Text, Image, Procedural, Card) with a non-trivial layer pipeline, to catch regressions in the critical-set fixture track (Concern B's carve-out — layered recipes are natural critical-set candidates because rendering equivalence is most at risk when the feature composes multiple pipelines per frame). Layered recipes without per-layer pipelines are also fixture-captured so additive-schema backward-compat is honored for layers that predate the feature.
- **Documentation.** SKILLS.md and authoring guides updated to teach the two-level scoping model (scene-level via layer pipeline, cell-level via scope primitive). The logo-on-flag-wave composability example above becomes canonical teaching material.

**Decisions the implementation track will need to resolve** (flagged here; settled during implementation):

- **Layer vs global precedence.** Does the layer pipeline run *before* the recipe-global pipeline (layer-first, both run) or *instead of* the global for that layer's cells (layer-replaces)? Default lean: layer-first — both run; layer is inner, global is outer. Layer-replaces is available as an opt-in per-layer mode only if a concrete use case surfaces.
- **Hint visibility across layers.** A `HintRef<T>` inside layer A's pipeline — can it reference a hint produced by layer B's pipeline? Default lean per Open Q #16: same-layer-only; each layer is its own hint namespace. Cross-layer reads require explicit export/import semantics if the use case surfaces. The reviewer's lean on Open Q #16 aligns with this default.
- **Layer blend mode at composition.** When a layer's composed cells meet the global composition result, what blend happens? Default lean: alpha-over based on the layer's `placement` / `z` semantics — same as today's layer composition without pipelines. The pipeline's per-cell output replaces the layer's un-piped cells; composition with other layers is unchanged.
- **Empty-pipeline semantics.** `Some(VfxPipelineConfig { steps: [] })` vs `None` — do they behave identically? Default lean: yes, both are no-ops. If an explicit empty pipeline proves distinguishable at runtime for trace/debug purposes, document the divergence.

**What this decision adopts and what it does not:**

- **Adopted direction.** Per-layer pipelines are part of V3. The schema extension lands. The two-level scoping model (scene × cell) is the canonical mental model for V3 composition. The logo-on-flag-wave example above is the canonical composability demonstration.
- **Not adopted as structural assumptions.** The precedence, cache-key, trace-event, and blend-mode defaults above are carried forward from the PRD primitive 1 analysis and from prior draft wording. They are the starting leans for the implementation track; the track may choose differently if implementation pressure surfaces. None are binding without track confirmation.

**Addresses Concern E of the 2026-04-21 GT-Design lead review memo** — scene-layer pipelines were previously described as a natural extension of the existing scene schema when in fact the `pipeline` field does not exist yet and the feature requires real schema/parser/validator/compositor/trace/fixture work. This revision calls out the implementation track explicitly, separates adopted direction from non-binding defaults, and lists the decisions the track will need to resolve.

This adopts PRD primitive 1 as a **dedicated V3 implementation track**, not as an integration detail.

**Why scene layers and unified Scope are two levels, not one:**

The natural instinct is to unify — why have two scoping mechanisms when one seems enough? The answer is that they target different kinds of concerns, and collapsing them would force each to carry the other's:

- **Scene layers carry geometry and content.** "This logo lives here with these bounds, over this background." Placement, z-order, content source. Inherently spatial and structural.
- **Unified Scope targets cells within an area.** "Within this layer, apply the shader to background-only cells except text cells." Per-cell selection logic. Inherently predicate-based.

Without scene layers, unified Scope would need geometry variants (rects, exclude-rects, layer-references), which are really placement concerns pretending to be scope predicates. Without unified Scope, scene layers couldn't target cells within themselves cleanly — you'd need full-layer vs partial-layer distinctions baked into layer type. Two levels keeps each concern clean.

### 6. Signal-driven parameters — formalize and extend

V3 parameters cannot be assumed scalar constants. Some real authoring intents require parameter values that vary across space, time, or both — authored via composed signal graphs or bound to runtime-provided values from the app.

**Evidence this is already a V3-relevant concept:**

- `BindableValue` is a shipped primitive at `/usr/projects/tui-vfx/crates/tui-vfx-compositor/src/types/cls_bindable_value.rs`. Used today by `matrix_rain`, `highlighter`, `focus_field`, `glisten_band`, `affordance_wake`, and others for runtime-bound parameters. Dynamic recipes reference values as `{"binding": "<name>"}` and the app supplies them per-frame.
- The flag-animation PRD formalizes `SpatialSignalSpec` for 2D-aware signal graphs — the same concept applied to internally-composed values (not runtime-bound, but graph-composed from spatial/temporal inputs).
- The ambient-halo exploration in `docs/internal/specs/relative-light-architecture.md` asked informally for "runtime color binding" — this is the same mechanism.

**Decision:** V3 introduces `ParamValue<T>` as the uniform type for step-level parameters sourced from outside the current pipeline evaluation. A `ParamValue<T>` is one of:

1. **Constant** — a scalar value, the 80% default case.
2. **Runtime-bound** — a reference to a named app-supplied value (`{"binding": "<name>"}`), resolved per-frame by the app. Matches the existing `BindableValue` machinery; V3 formalizes it uniformly across all step types, not per-filter-or-shader ad-hoc.
3. **Signal-graph** — a composed signal expression, evaluated per-cell-per-frame. Uses `mixed-signals` primitives (consumed, not duplicated, per Architectural framing above). For 2D-aware signals, extends `mixed-signals` upstream (Path B) rather than living in tui-vfx.

A fourth class of step-parameter input — references to named outputs of other steps in the same pipeline — is covered by `HintRef<T>` in Decision 7, modeled as a separate closed type rather than a `ParamValue<T>` variant. At field sites the two compose via `StepInput<T> = ParamValue<T> | HintRef<T>`, so step-parameter fields accept either form uniformly unless they narrow to one side for domain reasons. See Decision 7's *Interaction with signal-driven parameters* subsection for the split's rationale.

**What this unlocks:**

- Ambient halo's four per-edge diffusion instances with runtime-sampled colors (the ambient-halo RFC scenario).
- Flag-animation's compound-wave displacement with `x_norm`-proportional amplitude (the PRD scenario).
- Dynamic recipes where app-side signals drive visual parameters — `matrix_rain` density bound to app state, `focus_field` center bound to app-tracked focus target, `glisten_band` speed bound to scroll velocity.
- Training-demo movies where recipe parameters are scripted by a timeline (via runtime binding supplied by the movie player).

**Architectural positioning:** `ParamValue<T>` is the V3 shape; `mixed-signals` provides the signal-graph expression language; `BindableValue` is the existing runtime-binding machinery being generalized. V3 doesn't invent either — it formalizes the uniformity across all step types.

**Why generalize `BindableValue` uniformly across step types rather than per-shader:**

Today, `BindableValue` is used by a handful of specific shaders (matrix_rain, highlighter, focus_field, glisten_band, affordance_wake). Each one decided independently to support runtime binding for its parameters. New shaders that want the same capability have to opt in manually; new runtime-binding use cases (ambient halo's sampled colors, flag-animation's signal-driven amplitude, future dynamic parameters we haven't thought of yet) require per-shader wiring.

Uniformity across step types means:

- **Authors learn one mechanism per input class.** `ParamValue<T>` with three variants for external values; `HintRef<T>` for pipeline-internal step-output references (Decision 7); `StepInput<T>` as the field-site sum so most step parameters accept either form uniformly. Works the same way whether the step is a filter, a sampler, or a shader. No "does this specific shader support binding?" question.
- **The canonical builder (Decision 8) handles binding resolution uniformly.** Every step gets per-frame binding resolution through the same pipeline; individual step types don't re-implement the machinery.
- **Validators can reason about bindings across the whole recipe.** "Which bindings does this recipe require?" becomes a tree walk, not a union of per-shader knowledge.
- **The future movie-composer layer (deferred-design) gets uniform animation access.** A script can inject values into any ParamValue in any recipe without caring which specific shader's parameter it's reaching.

The alternative (keep per-shader bindings, add them to more shaders over time) pays the same cost repeatedly for every new shader and every new use case. Generalization pays once.

**Relationship to tokenization (Open Q #14).** Decision 6's `ParamValue<T>` has two variants that bridge this boundary: `Constant` resolves at load time via `Substitutions`; `RuntimeBinding { name }` resolves per-frame via `RuntimeBindings`. Load-time `Substitutions` and per-frame `RuntimeBindings` are two distinct API surfaces at the `tui-vfx-recipes` boundary (see Open Q #14 for the full framing). The primary organizing axis is *temporal lifetime* — not the domain split (text vs structured) the earlier draft framing used — because lifetimes determine failure modes: load-time misses are strict-mode hard errors, per-frame misses are graceful-fallback soft errors, and those cannot be collapsed onto a single API without losing precision. Each surface handles both content domains internally (text tokens and structured values). Decision 6's variants and Open Q #14's surfaces are two views of the same split: `ParamValue::Constant` ↔ `Substitutions`; `ParamValue::RuntimeBinding` ↔ `RuntimeBindings`.

### 7. Step output hints as a first-class primitive — adopt

Principle 2 (pipe-culture chain-ability) becomes structural here. V3 adds named step outputs to the schema so downstream steps can bind to them, not because any one step needs it, but because the principle requires it as a general capability.

**The shape:**

- Steps that produce per-cell data beyond their primary payload declare named outputs (hints). Example: `SineWave` sampler emits `displacement` hint alongside its primary cell transform.
- Steps that want to consume another step's output declare bindings to named hints. Example: `DisplacementShade` declares `input: displacement` and reads whichever upstream step produces that hint for the current layer.
- The hint namespace is defined, not per-step ad-hoc: `displacement`, `sampled_color`, `cell_density`, `alpha_mask`, etc. Adding a new step type doesn't require extending the namespace unless the step produces a genuinely new class of hint.

**Why this is Decision 7 and not deferred:**

Because the `DisplacementShade` requirement from the flag-animation PRD is concrete and imminent, and because signal-driven parameters (Decision 6) and step hints are closely related — both are forms of cross-step data flow. Deciding them together ensures the vocabulary stays coherent.

**Why named hints rather than explicit step references:**

An alternative to hint names would be explicit step references — a step binds to "the output of step X" by referencing X's ID directly. This was considered and rejected because:

- **It breaks Principle 2 (pipe-culture chain-ability).** Pipe culture is about self-contained primitives that don't know about each other. A DisplacementShade that explicitly references a specific upstream sampler by ID couples the two, makes authoring brittle (moving or renaming the sampler breaks the shader binding), and prevents easy swapping of producers.
- **Named hints let any producer satisfy a consumer.** "Give me displacement from whichever upstream step produces it" is more composable than "give me displacement from `sampler_42`." Authors can swap producers freely; the binding vocabulary is stable.
- **Validators have a shorter path.** A named hint binding can be statically verified against the hint namespace; an ID-based reference requires tree traversal to confirm the producer actually exists and emits the expected shape.

The cost of named hints is that they require a defined namespace (which hints exist). That namespace grows additively and carefully — new hint kinds are added as real use cases surface, not pre-emptively.

**Interaction with signal-driven parameters — the `ParamValue<T>` / `HintRef<T>` split:**

Step-output references are modeled as a distinct closed type, `HintRef<T>`, rather than as a fourth variant of `ParamValue<T>` (Decision 6). The two cover materially different concerns and have different resolution paths and validator work:

- `ParamValue<T>` covers external value sources: `Constant`, `RuntimeBinding { name }` (app-supplied), `SignalGraph` (composed expression from `mixed-signals`). Resolution crosses the engine boundary (substitution contract, signal evaluation). Validator surfaces the binding contract — the set of app-supplied values this recipe requires.
- `HintRef<T>` covers pipeline-internal references: `{ hint, producer_ref? }` reads a named hint from an upstream step in the same pipeline evaluation. Resolution is a producer lookup against the hint namespace. Validator walks the tree to confirm at least one producer exists and the hint's shape matches `T`.

At field sites the two compose via `StepInput<T> = ParamValue<T> | HintRef<T>`, so step-parameter fields accept either form uniformly unless domain reasons narrow them (e.g., a shader's `blend_mode` probably only accepts `ParamValue<BlendMode>`; a `DisplacementShade`'s `input` field only accepts `HintRef<Displacement>`). Both resolve to per-cell-per-frame scalar/color/vector values at render time; the split is about *where the value comes from*, not what it is.

Why split and not a single four-variant `ParamValue<T>`: external value sources are app policy flowing in; step-output refs are meaning flowing within the pipeline (Principle 5). They have different lifetimes (cross-boundary vs intra-pipeline), different failure modes (missing binding contract vs producer-not-found), and different tooling paths (binding-contract discovery vs hint-graph inspection). Collapsing them into one type forces every field, validator, and tool to reason about cases that don't apply to it. Keeping them distinct keeps each type's rules tight and each error message precise.

**Open for later design:** resolution rules when multiple upstream steps produce the same named hint (first-producer-wins vs last vs explicit reference vs validator-forbidden-ambiguity — see Open Q #16), and whether hint refs can compose in-schema (e.g., "displacement multiplied by a scalar signal") or whether composition only happens via an intermediate step. Not blocking V3 direction; needs settling before V3 implementation.

### 8. Canonical upstream semantic seam — adopt, formalize during V3

Today, `tui-vfx-recipes` owns recipe schema and playback semantics, but the *seam* between upstream canonical semantics and downstream consumers (gt-design, future consumers) is conventional rather than structural. gt-design re-implements parts of the config-to-item semantic translation in `gtd-ratatui/src/recipes/{item,planner,player}.rs`, and that fork is where semantic drift lives.

The concrete proof of the problem: when `config.shadow` was added upstream, the field didn't automatically reach gt-design's render path because gt-design's parallel semantic conversion didn't know about it. Most fields flow through cleanly; the ones that cross this seam can drift, and finding out which ones drift happens late. This is the weak seam the daily-use developer ranked #1.

**Direction:** V3 formalizes the upstream semantic seam as a structural contract so downstream consumers stop forking semantics.

Concretely, `tui-vfx-recipes` ships:

- A **canonical `config → playback item` builder** — a single, documented, programmatic entry point that transforms `VfxRecipeConfig` into a `PlaybackItem` (or whatever the post-Open-Q-#19 name lands on) with canonical lifecycle and render-plan semantics applied.
- **Stable effect-carrier threading.** New additive fields (like shadows, future scene primitives, layer schedules, etc.) flow through the canonical builder without requiring downstream rethreading. Threading is the builder's job; consumers inherit it.
- **Documented handoff shape.** The builder's output is the canonical contract for downstream consumers. gt-design wraps the canonical builder's output and applies its *policy* (theme, surface identity, render-truth routing) but does not re-implement the *meaning*.

**Why this is a Decision and not just a principle:**

Principle 5 (meaning low, policy high) is the general rule; Decision 8 is the specific V3 deliverable that makes the rule structurally enforceable for the recipe seam. Without an explicit canonical builder, gt-design (and any future consumer) is tempted to reimplement — because the seam is under-documented and under-obvious. A formal contract removes the temptation: reimplementation becomes a visible deviation rather than an unremarkable default.

**What gt-design retains — its own substantial work — is policy:**

- `RecipeSceneCanvas` and semantic scene lowering into factory-canonical render truth (Intention 50)
- Theme resolution / SSOT binding (tokens before the seam, structured values and hints after it)
- Surface identity policy (toast vs modal vs tooltip — see Open Q #20)
- Factory composition and shadow/depth/motion orchestration

The goal isn't "use upstream rendering directly." It's "use upstream *semantics* canonically, then adapt into GTD render truth." Upstream owns meaning; gt-design owns policy and final render truth.

**Sub-questions during V3 implementation:**

- Does the canonical builder live in `tui-vfx-recipes` or in a new sibling crate? Probably the former to keep the seam close to the schema; up for confirmation during implementation design.
- What's the public surface? A single `fn build_playback_item(config: &VfxRecipeConfig) -> Result<PlaybackItem, Err>` plus the typed `PlaybackItem` output? Or a builder pattern? Or a trait consumers can implement / wrap? Probably the simplest function + struct that works.
- How are post-builder customizations expressed? gt-design sometimes legitimately needs to adjust the item before rendering (e.g., inject theme-resolved values into placeholder slots). The contract should accommodate documented extension points without re-inventing the whole item.
- How does the canonical builder interact with the two `tui-vfx-recipes` boundary surfaces (Open Q #14)? `Substitutions` (load-time) runs before the builder, so the builder sees a fully-substituted config; `RuntimeBindings` (per-frame) is evaluated at render time after the builder has produced a playback item. Binding-contract discovery happens at the builder stage so consumers can inspect the per-frame contract before starting playback.

**Addresses feedback items:** 1 (duplicated semantic conversion), 8 (seam conventional rather than structural). Directly enables retiring the parallel translation code in `gtd-ratatui/src/recipes/{item,planner,player}.rs`.

## Shape sketches

Small illustrative examples. A broader translation study across ~12 diverse recipes lives in the appendix referenced at the bottom of this document.

### Simple fade-in toast

**Flat (today):**

```json
{
  "pipeline": {
    "enter": {"duration_ms": 300, "easing": "quad_out"},
    "exit":  {"duration_ms": 200, "easing": "quad_in"},
    "mask": {"enter": {"type": "none"}, "exit": {"type": "none"}},
    "sampler": {"enter": {"type": "none"}, "exit": {"type": "none"}},
    "filter": {"dwell": []},
    "styles": [
      {"region": "All", "base_style": {...},
       "enter_effect": {"type": "fade_in"}}
    ]
  }
}
```

**Tree (proposed):**

```json
{
  "pipeline": {
    "timing": {"enter_ms": 300, "exit_ms": 200, "enter_ease": "quad_out", "exit_ease": "quad_in"},
    "step": {
      "kind": "style_effect",
      "phase": "enter",
      "payload": {"type": "fade_in"}
    }
  }
}
```

Simple recipe is visibly simpler. No ceremony for masks, samplers, filters the recipe doesn't use.

### Ember-felt (three layered dwell operations)

**Flat (today):** scattered across `pipeline.styles[1].spatial_shader`, `pipeline.styles[2].spatial_shader`, and `pipeline.filter.dwell[0]`. Semantic grouping ("these three all happen during dwell on the background") is invisible in the document structure.

**Tree (proposed):**

```json
{
  "step": {
    "kind": "parallel",
    "phase": "dwell",
    "scope": {"kind": "channel", "value": "background"},
    "children": [
      {"kind": "shader", "payload": {"type": "diffusion", "source": "top_right", "color": {...}, "mode": "warm_drift"}},
      {"kind": "shader", "payload": {"type": "concealed_light", "source": "left", "color": {...}}},
      {"kind": "filter",  "payload": {"type": "vignette", "sides": ["bottom", "right"]}}
    ]
  }
}
```

Scope propagates to all three children. The three-operation structure reads at a glance. Named factories (`diffusion`, `concealed_light`, `vignette`) remain the JSON surface; internally they load to the decomposed model.

### Ambient halo (not expressible in flat schema)

Four per-edge diffusion instances scoped to the recessed canvas, bound to runtime-sampled colors:

```json
{
  "step": {
    "kind": "parallel",
    "phase": "all",
    "scope": {
      "kind": "and",
      "children": [
        {"kind": "channel", "value": "background"},
        {"kind": "rect_exclude", "source": "focus_rect"}
      ]
    },
    "children": [
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "top"}, "color": {"kind": "sampled", "source": "focus_edge_top"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "bottom"}, "color": {"kind": "sampled", "source": "focus_edge_bottom"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "left"}, "color": {"kind": "sampled", "source": "focus_edge_left"}}},
      {"kind": "shader", "payload": {"type": "colored_overlay", "pattern": {"kind": "radial_from_edge", "edge": "right"}, "color": {"kind": "sampled", "source": "focus_edge_right"}}}
    ]
  }
}
```

This composition is impossible in the flat schema today because: filters can't scope to non-rect regions; shaders can't bind to runtime-sampled colors; and there's no way to declare "four instances of the same operation, each with per-edge parameters." The tree schema + unified scope + Pattern-axis + runtime color binding make it natural.

## Open questions that must resolve before implementation

These are not rhetorical — each represents a real design choice not yet settled. Ordered roughly by impact on the plan shape.

### 1. Does the `kind` discriminator survive?

Tachyonfx collapses mask/filter/sampler/style into one `Effect` with naming by factory function. Our working assumption is that the four kinds survive as enum variants on the unified Step because they represent genuinely different operations (reveal vs post-process vs texture-overlay vs style-transform), and the distinction aids authoring comprehension. But if after implementation we find the boundary is mushy — e.g., a mask with scope is indistinguishable from a filter with a cell-clear payload — we may want to collapse further. Preserving the distinction is the safe starting position; reducing later is easier than splitting later.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** keep the `kind` discriminator. *"The distinction is still useful for comprehension, validation, documentation, and tooling. You can always collapse later if the boundaries truly prove artificial."*

### 2. Migration strategy and schema versioning

**V3 is a clean cutover — V2 is not carried forward as a loadable format (see Why Now framing).** The entire shipped recipe corpus migrates during V3 implementation; V3 is the new floor. No compatibility shim, no deprecation window, no dual loader path.

Migration is a three-phase human-directed workflow rather than a mechanical translator pass over the whole corpus. Phase ordering is deliberate: curation reduces the problem space before any translation labor is spent on it; AI re-authoring instruments the briefing infrastructure the library will rely on going forward; validator checks are the last gate rather than an intermediate artifact.

**Prereq (part of V3 implementation proper, not migration phases).** The V3 validator infrastructure (Open Q #9) is built before migration phase 3 runs. The narrow-scope mechanical translator used only in the critical-recipe carve-out (below) is also built as part of V3 implementation work, not as migration tooling.

**Mainline corpus (the large majority of retained recipes):**

1. **Curate — human Morris filter over the full V2 corpus.** For each V2 recipe, decide: port / consolidate / archive / delete. This phase collapses the problem space — recipes that don't earn a V3 slot are not translated. Output: the retained set with per-recipe disposition and rationale. Workflow B in the sibling audit-workflow doc is this phase. Running curation first matters because it prevents pouring translation labor into recipes that will be archived or deleted, and because it keeps the curation conversation focused on "does this earn its place?" rather than "did the translation succeed?"

2. **Re-author — Claude translates each retained recipe from V2 intent to V3 form under explicit authoring briefing.** This is not a mechanical reshape; it is a capability test for V3's AI-authoring pathway (the primary composition mode per Decision 3's rationale). Where Claude struggles, the briefing infrastructure (SKILLS.md, prompt scaffolds, on-disk vocabulary references, authoring guides) has a gap — which is exactly the gap every future author (human or AI) will hit when writing a new V3 recipe from scratch. Briefing improvements land alongside the recipes that surface them; the migration is deliberately used as a forcing function for briefing quality. Claude's output is reviewed by the human in the loop before commit. Re-authoring is also where latent V3 primitive gaps surface: a V2 recipe that can't be cleanly re-authored in V3 is evidence that V3 is missing a primitive, a Pattern variant, a hint kind, or a binding mechanism — those gaps route back to V3 implementation work, not to ad-hoc recipe workarounds.

3. **Validate — V3 validator runs on every re-authored recipe.** Validator covers schema shape, scope coherence, hint-namespace membership (HintRef<T> producer verification), fragment addressability, binding-contract discovery, and required-field presence. Validator failures block merge for the affected recipe. Semantic drift from V2 to V3 is often *intended* at this stage — re-authoring is allowed to improve on the V2 version, that's part of the point — so the mainline validator checks well-formedness rather than rendering equivalence.

**Critical / fixture carve-out (small designated set — expected ~5–15 recipes).**

For recipes where downstream consumers or test infrastructure depend on specific rendering behavior, AI re-authoring alone is insufficient: "similar but subtly different" V3 output is a silent visual regression for apps upgrading to V3, and a correctness break for probe tests whose purpose is to pin rendering. The designated set routes through a parallel track:

1. **Capture V2-rendered fixtures before any migration work begins.** Cheap insurance; run once against current V2 corpus. Checked in.
2. **Mechanical translator produces V3 for the designated set only.** Deterministic transformations: tree reshaping, Ra→Vfx rename, scope/phase wrapping, ParamValue/HintRef/StepInput handling, default population. Does not attempt curation.
3. **Fixture-equivalence gate.** V3 render must match V2 fixture within tolerance (tolerance is per-recipe-kind: exact for probe-validation fixtures, perceptual-delta for splash-class visuals, structural for scenes with deliberate V3 improvements). Drift is either a translator bug (fix) or intended V3-only behavior (document and whitelist).
4. Curatorial review still applies to the critical set — Morris filter, naming, metadata — but rendering is preserved by the gate.

Candidate critical-set members: the probe-validation corpus (by definition — these recipes exist to pin rendering), the splash family (gt-design ships specific splash visuals apps depend on), and any recipe that app-level docs or release notes currently cite as a specific visual contract. Membership is designated explicitly per-recipe with written justification; inclusion is not the default.

**What this resolves.** Addresses Concern B of the 2026-04-21 GT-Design lead review memo — the prior draft split-brained between Open Q #2 ("script with human exceptions") and the Recipe migration workflow section in Deferred ("manual curation, bulk translation is not the goal"). Both framings had partial truth; this three-phase model unifies them by sequencing curation first (problem reduction via Morris filter), AI authoring second (instrumenting the future-author pathway per Decision 3's stated primary composition mode), and validation third (well-formedness via built validator), with a fixture track for the subset where rendering equivalence is load-bearing. The reviewer's drift-audit concern is addressed at two levels: validator + curatorial review for the mainline, fixture equivalence for the critical set.

**Sub-questions that still need resolution during implementation:**

- **Authoritative inventory step (blocking prerequisite).** Before curation begins, an inventory pass produces a checked-in manifest of all recipe files with classification (candidate / debug / probe / test / deprecated / generated). Current filesystem evidence suggests the main corpus is >500 files, well above prior "200–300" estimates. Inventory gates all scope and schedule assumptions.
- **Critical-set membership discipline.** Who designates inclusion; how each member's inclusion is justified per-recipe; whether the set is frozen at migration start or can grow during migration as additional rendering contracts surface.
- **Fixture-tolerance specification.** Pixel-perfect vs percentage-delta vs structural-equivalence vs probe-event match. Probably a mix by recipe kind; concrete specification needed before the carve-out runs.
- **Briefing-improvement commit discipline.** When re-authoring surfaces a briefing gap, does the fix land as a separate commit before the recipe, alongside it, or in a batch at phase-end? Probably alongside, with the motivating recipe cited in the commit message.

**Future versioning after V3 ships** is a separate concern. If V3 attracts external consumers (currently: none), V4 migration will need the compatibility discipline that V3 does not need to carry. That's a future-plan concern, not a V3 concern.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — input behind Concern B's resolution):** hybrid migration (mechanical translation + human review + fixture/probe equivalence for critical recipes), explicitly NOT purely manual Claude-led rewrite of the whole corpus. This was load-bearing for the three-phase model (Curate → Re-author → Validate with critical-set carve-out) captured above. Implementation-mechanics sub-questions remain open.

### 3. Phase-scoping shape: per-step field vs container

Currently proposed: each step carries `phase: Enter | Dwell | Exit | All`. Alternative: phase is a container (`Phase::Dwell(...)`) wrapping its children. Per-step field is flatter and matches the scope-field pattern (both are metadata on an atom). Container is more readable at a glance (the tree clearly segments by phase). The two shapes are isomorphic. Decision pending readability review against the appendix translations.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** per-step field, with container propagation. *"This matches the scope model, keeps the normalized shape regular, and still allows readable grouping."* Aligns with the plan's current lean.

### 4. Composition combine semantics — explicit or defaulted

Current flat schema has implicit filter ordering ("applied in order") and explicit mask combine modes (`All | Any`). Tree `Parallel` containers could carry a `combine: Chain | Union | Intersect | Replace` policy, or combine could be per-kind with sensible defaults. Authoring ergonomics strongly prefer per-kind defaults; safety-net arguments lean toward explicit-at-container. Probably: per-kind defaults with container override available, but the exact default table needs discussion.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** per-kind defaults plus explicit container override. Also strongly recommends a **normalized internal form** where the effective combine is explicit after parsing/canonicalization, so tooling and tests don't have to re-infer defaults.

### 5. Named-factory and compositional JSON coexistence

Both `{"type": "diffusion", ...}` and `{"type": "colored_overlay", "pattern": {...}, ...}` load to the same internal representation. Do we:
- Validate that both shapes produce identical behavior (property test)?
- Privilege one in examples and SKILLS.md, or teach both?
- Allow themes to mix both in the same recipe, or enforce consistency?
- Provide a canonicalization tool to convert named → compositional for inspection?

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** yes, support both; validate equivalence; provide canonicalization tooling. Property-test equivalence for curated pairs; canonicalize for inspection/debugging; teach named factories for curated presets; teach primitive/compositional form for advanced/custom authoring. Allow mixing in one recipe, but don't make mixing the default teaching style.

### 6. Scope primitive — open-closed tension

Closed algebraic enum is safer, validatable, and cacheable (cf. tachyonfx's static/dynamic analyzer with bitmask caching). Closure-based escape hatches (`PositionFn`, `EvalCell`) are powerful but uncacheable and resist static validation. We need both: closed variants for 95% of authoring intents, escape hatches for the novel 5%. The open question is the boundary. One proposal: closed variants are directly JSON-encodable; closure escapes require Rust-side registration (the recipe references a named predicate registered at compile time, not an arbitrary eval string).

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** closed enum with registered escape hatch — *"the right balance for caching, validation, and authoring predictability."* Aligns with the plan's current lean.

### 7. Relationship to `RecipeSceneCanvas` (Intention 50)

The tree-schema migration is nominally independent of the in-GTD recipe-playback substrate (`RecipeSceneCanvas`). But both land in the same authoring surface and both unblock the same explorations (ambient halo, ember-felt). Sequencing options:
- Land `RecipeSceneCanvas` first against V2 flat schema, then migrate schema.
- Land V3 tree schema first, then build `RecipeSceneCanvas` against it.
- Land both in the same cutover under a unified "pipeline v3" banner.

Each has different risk profile and different blocking structure. Needs explicit decision.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** do not make GTD substrate sequencing the blocker for upstream V3 core work. Upstream should stabilize first (canonical semantic seam, naming cleanup, token/binding contracts, normalized execution model); GTD then adapts `RecipeSceneCanvas` to that seam.

### 8. Unblock order for Relative Light explorations

Ambient halo and ember-felt are both easier to express in V3. Do we:
- Block both on V3 (delays them but keeps recipes consistent).
- Ship them in V2 first, migrate later (unblocks the work but increases migration corpus).
- Ship them in V2 but gate them behind feature flags so migration is clean.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** do not ship productized V2 versions if V3 is clearly the right substrate. Continue exploration as isolated R&D fixtures, debug recipes, or lab-only prototypes — not as user-facing V2 contracts that would immediately need migration.

### 9. Validator redesign

Tree schemas need different validation than flat schemas. New rules to design:
- Scope-coherence (no nonsensical scope combinations — e.g., `GlyphMatches` on a mask-reveal operation where "glyph" doesn't yet exist).
- Container-shape invariants (no `Parallel` with conflicting masks; no `Sequence` with zero steps).
- Scope-propagation conflict detection (child declares scope X, parent propagates Y; precedence rule?).
- Migration validation: V2 → V3 auto-migration must preserve probe-equivalence on the full recipe corpus.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** this is **core V3 work, not support work.** Validator scope: scope coherence, tree/container invariants, hint ambiguity, fragment addressability, token/binding contracts, migration equivalence for critical fixtures. Also validate a **canonical normalized IR**, not only raw authoring syntax — that keeps the validator durable across future schema evolution.

### 10. Viewer still worth building independently

Even with tree authoring, a visual renderer of a pipeline (inspector / debugger / recipe explorer) is valuable. In an earlier conversation turn the viewer was proposed as a *substitute* for tree authoring; we rejected that. But the viewer has independent value — does it stay on the backlog as its own project, or does it fold into a broader "recipe tooling" initiative post-V3?

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** yes, but build it on the normalized execution graph / canonical IR, not directly on author sugar. *"That will make it much more durable across future schema evolution."* Pairs with Q9's "validate normalized IR" point — viewer and validator can share the IR surface.

### 11. Docs, SKILLS.md, and generator updates

Every existing doc page that references schema fields needs updating:
- Generated API docs
- Recipe authoring guides
- `docs/api/AI_ORIENTATION.md`
- `docs/api/SKILLS_REFERENCE.md`
- VFX usage guides
- Theme authoring checklist
- Every lab/example README that references recipes

The generator infrastructure (`just docs-gen`, drift checks) must be updated in the same cutover. Bounded but non-trivial work; plan needs to budget for it.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** must ship in the same cutover as the schema change. Especially important: generated API docs, validator/tracing docs, AI/LLM guidance, migration notes, and canonical examples. Aligns with the plan's current lean (same-cutover).

### 12. Shadow rendering, offscreen composition, probe/trace compatibility — V3 release gate

The V3 tree schema, the per-layer pipeline feature (Decision 5's implementation track), and the uniform step vocabulary (Decision 3) all touch infrastructure that downstream consumers — gt-design in particular — depend on for correctness at final render truth: shadow fidelity, offscreen composition behavior, role-aware lowering, trace/probe observability, factory-canonical final rendering. The prior draft flagged these as "a risk area, not a blocker." That framing was wrong — if V3 regresses these, the schema improvements don't pay back, and the 2026-04-21 GT-Design lead review memo specifically escalates this to release-gate status (Concern F).

**Decision: these are V3 release-gate criteria, not open risk items.** V3 does not ship without green on each gate criterion for the designated-critical-set of consumers and recipes.

**Gate criteria (what must be green before V3 ships):**

1. **Canonical shadow fixtures.** Every shipped shadow primitive (depth-based, elevation-based, glow/bloom, directional) has a captured pre-migration fixture from V2 rendering and a post-migration V3 render. Delta is either within tolerance, documented-and-whitelisted as intended V3 behavior, or blocking.
2. **Offscreen / slide fixtures.** Representative recipes that use offscreen composition (multi-pass rendering, buffered intermediate stages, slide-in/slide-out transitions) have captured fixtures covering pre-migration and post-migration render. Same tolerance + whitelist + blocking model.
3. **Probe snapshots.** The `vfx-probe-validation` corpus — by definition the recipes that exist to pin rendering behavior — passes probe-equivalence against pre-migration captures. Any probe diff is either a translator bug, an intended V3-only behavior change (documented), or blocking.
4. **Trace expectations.** The trace/probe infrastructure emits events at the same granularity and with the same semantic content as V2 for representative flows. Schema additions (e.g., `TraceEvent::LayerPipelineApplied` from Decision 5) are allowed; removals or semantic shifts are either documented or blocking.
5. **GT-Design integration fixtures.** Representative gt-design surfaces (splash family, default recipe set, toast family, modal family, any recipe that app-level docs or release notes cite as a specific visual contract) render identically within tolerance against pre-migration captures. Failures route through Concern B's critical-set carve-out's fixture-equivalence gate.
6. **Role-aware lowering correctness.** The canonical builder (Decision 8) produces playback items whose role-aware handling (`RoleTag` at the render layer, not the new `RoutingRole` / `SurfaceIntent` hint types from Open Q #18) matches V2 for the fixture set. Documented whitelist is allowed only where V3's role-domain split (Concern C) deliberately changes behavior.

**Relationship to Concern B's critical-set carve-out:**

Concern B's critical-set fixture track is the *mechanism* by which gate criteria 1–5 are evaluated. The critical set includes the recipes named above plus any specific recipe a consumer designates as rendering-load-bearing. The fixture-equivalence gate runs as part of V3 CI and blocks release on failure.

Concern B handles *how* fixtures are captured, translated, and diffed; Concern F (this release gate) handles *what* must be captured and *what counts as passing*. The two are complementary: Concern B is infrastructure and workflow, Concern F is the specific gate criteria the infrastructure evaluates.

**Remaining implementation-level questions** (these are what this Open Question still covers, post-release-gate commitment):

- **Per-criterion tolerance specification.** Pixel-perfect, percentage-delta, perceptual-delta, structural, or probe-event match? Each criterion likely has its own tolerance shape. Default lean: probe-validation corpus is exact (by definition); shadow / offscreen / GTD-integration fixtures are perceptual-delta with per-recipe calibration; trace events are structural equivalence with documented additions allowed.
- **Gate ownership.** Who designates the GT-Design representative surfaces? Who maintains the whitelist of documented intended changes? Probably gt-design's lead for GTD-integration surfaces and the V3 implementer for everything else, coordinated via explicit commit-and-PR workflow.
- **Whitelist discipline.** A whitelist entry documents *why* a V2→V3 behavior change is intended. Entries must be legible to a downstream maintainer auditing gate output six months later — rationale, affected recipes, expected before/after behavior. Whitelist-as-commit-message is insufficient; whitelist-as-structured-manifest is the bar.
- **Recapture cadence.** If a V2 fixture is stale (the V2 rendering itself changed since capture), does the gate recapture automatically or require explicit human approval? Lean: explicit approval, because silent recapture masks regression.
- **Gate-fail escalation.** When a gate criterion fails during V3 implementation, what's the escalation path? Lean: block the V3 release milestone; route diagnosis to the owning track (compositor, trace, shadow crate); resolve-or-whitelist-or-defer-critical-set-membership explicitly before resuming.

**Addresses Concern F of the 2026-04-21 GT-Design lead review memo** — shadow/offscreen/trace compatibility promoted from "open risk" to explicit V3 release gate with enumerated criteria. Cross-links to Concern B (critical/fixture carve-out is the evaluation mechanism).

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — input behind Concern F's resolution):** release gate, not optional polish. One input among several; the implementation-level sub-questions above (tolerance spec, gate ownership, whitelist discipline, recapture cadence, escalation path) remain open.

### 13. Partial-phase spans (PhaseSet granularity)

Today's phase scoping is binary: effects either apply to one phase (`enter`, `dwell`, or `exit` via per-phase slots) or to all three (via `pipeline.continuous` with a unified `RaClock`; added in tui-vfx-recipes v2.7.0). There is no way to declare **"enter + dwell but not exit"** or **"dwell + exit but not enter"** — a legitimate authoring intent (e.g., a glow that arrives and sustains but shouldn't carry through the fade-out).

V3 should support `phase: PhaseSet` where PhaseSet is any subset of `{Enter, Dwell, Exit}`, with `All` and single-phase remaining valid shortcuts. The `continuous` block's unified-clock semantics should port forward as a clock-selection policy on any multi-phase step.

Open: does the PhaseSet shape live at the step level (every step can phase-scope) or only on containers (Parallel/Sequence containers carry phase membership that propagates to children)? Per-step is more expressive; container-scoped is more readable. Both could coexist with propagation rules.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** yes, support `PhaseSet`, and keep it available at the step level. Container propagation can exist too, but don't make container-only the model. Aligns with the plan's per-step lean.

### 14. Tokenization ownership and contract discovery

Today, field tokenization happens at the app layer. Concrete example: `gtd_ratatui::splash::Substitutions` (at `crates/gtd-ratatui/src/splash/cls_substitutions.rs`) provides `{{app_name}}`, `{{version}}`, `{{system_name}}` Mustache-style tokens that gt-design's splash runtime resolves before handing the recipe to tui-vfx. Similar tokenization needs show up in other consumers ad-hoc.

**Direction:** move this to a `tui-vfx-recipes`-boundary API. One typed `Substitutions` builder; one `load_with_substitutions` entry point; every consumer speaks the same tokenization vocabulary; validators and probes can reason about token references uniformly. The load stage inserts right after JSON parse, before SSOT resolution — the existing SSOT loader stays intact.

**Load-time `Substitutions` vs per-frame `RuntimeBindings` — the fundamental split.**

V3 exposes two distinct API surfaces at the `tui-vfx-recipes` boundary, divided by *temporal lifetime* rather than by content domain:

- **`Substitutions` (load-time).** Handles all substitution that resolves once when a recipe is loaded. Text tokens (`{{app_name}}`, `{{version}}` — Mustache-style within string fields), asset bytes (images, fonts), and structured values set once at app startup (e.g., a brand color fixed at launch). Resolution happens right after JSON parse, before SSOT resolution and before the canonical builder (Decision 8) produces a playback item. Strict-mode by default: missing references produce loud, early failures with contract listing. `Substitutions` corresponds to `ParamValue::Constant` at the type level — both are "value known at load time."

- **`RuntimeBindings` (per-frame).** Handles typed values that update each render — focus target, scroll velocity, app state, signal-driven shader params. Corresponds to Decision 6's `ParamValue::RuntimeBinding { name }` and today's `BindableValue` machinery. Set per-frame or set-on-change; a binding remains active until overwritten. Failure mode: a missing binding at frame N falls back to last-known-good, a declared default, or a validator-declared fallback; it does not hard-fail rendering. Strict-mode is not meaningful for values expected to update dynamically.

Why temporal lifetime is the primary axis rather than text vs structured:

- **Lifetimes determine failure modes.** Load-time misses are hard errors; per-frame misses are graceful fallbacks. Collapsing onto one API loses that precision.
- **Live-code evidence.** GTD today already splits along the temporal axis — `gtd_ratatui::splash::Substitutions` (load-time, handles text + assets) and `BindableValue` (per-frame, handles structured values). The V3 canonical API preserves this mental model and upgrades it from ad-hoc per-consumer to shared at the `tui-vfx-recipes` boundary.
- **Usage is concentrated diagonally on the 2×2 of (text/structured) × (load/per-frame).** Load-time text (`{{app_name}}`) and per-frame structured (focus target) are the dominant cells; load-time structured (brand color set once) is common too; per-frame text is rare and typically handled via dynamic widgets rather than recipe substitution. Organizing the API by temporal axis matches actual usage; organizing by domain would cross-cut the common paths.

Both surfaces handle both content domains internally — text and structured values appear in each API as distinct method families (`Substitutions::with_string`, `with_color`, `with_image`; `RuntimeBindings::set_color`, `set_number`, `set_vec2`). The domain distinction is a method-family split *within* each surface, not across them.

**Optional umbrella wrapper.** Consumers that want to pass both surfaces through one value can use an umbrella type (candidate name: `RecipeContext { subs: Substitutions, bindings: RuntimeBindings }`). The umbrella is ergonomic sugar; the lifetimes and failure modes remain explicit in the wrapped types.

Addresses Concern D of the 2026-04-21 GT-Design lead review memo (tokenization and runtime bindings coordinated but distinct) and feedback item 7 (intake layer complex — the raw/resolved/template-backed/runtime-param-injected pathways consolidate into the two-surface shape).

**Sub-questions to resolve during implementation:**

- **Partial vs whole-value substitution (`Substitutions` only).** `"message": "Hello {{name}}"` is partial string substitution; `"duration_ms": "{{ms}}"` is whole-value requiring type coercion. The `Substitutions` loader is schema-aware and handles both. `RuntimeBindings` is always whole-value typed — no partial-string semantics at render time.
- **Can load-time substitutions resolve to objects?** Probably scalar-first (strings, numbers, colors, assets); object substitution (e.g., a whole pipeline fragment) only if a concrete use case demands it. `RuntimeBindings` is scalar-only by construction — bindings resolve to typed values, not structural JSON.
- **Unresolved-reference policy — different per surface.** `Substitutions`: strict by default; a missing token produces a load-time failure with contract listing. Caller-configurable relaxed mode for string fields leaves literal `{{token}}` in output. `RuntimeBindings`: no strict-mode; a missing binding at frame N falls back to last-known-good or a validator-declared default. The two surfaces have different failure models and must not share one policy knob.
- **Asset resolution lives in `Substitutions` only.** `Substitutions::with_image(name, bytes)`, `with_font(name, bytes)` alongside the text and structured-value methods. Assets don't update per frame in any use case we've surfaced; if a future need genuinely wants per-frame asset swap, it routes through `RuntimeBindings` with a typed asset-handle binding rather than re-uploading bytes every frame.
- **Relationship to procedural `params`.** Procedural source params can reference both surfaces. `{"seed": "{{random_seed}}"}` is `Substitutions` (load-time); `{"color": {"binding": "brand"}}` is `RuntimeBindings` (per-frame). They compose transparently because the procedural generator sees the final resolved value regardless of which surface provided it.

**Contract discovery — two contracts, one per surface.** Once the two-surface API exists, consumers need to discover what each surface expects — especially for marketplace/third-party recipes where the consumer didn't author the recipe. Per-surface discovery mechanisms:

1. **Recipes declare both contracts explicitly.** Extending the PRD's `requires_primitives` pattern: `requires_substitutions: { app_name: "string", brand_color: { type: "color", default: "#4080FF" }, logo: { type: "image" } }` + `requires_bindings: { density: { type: "number", range: [0.0, 1.0] }, focus_target: { type: "vec2" } }`. The two fields are declared separately because their failure modes differ — an unsatisfied `requires_substitutions` fails at load; an unsatisfied `requires_bindings` is a per-frame contract the consumer must uphold.
2. **Validators cross-check contracts.** Every `{{token}}` and load-time structured reference has a `requires_substitutions` entry; every `ParamValue::RuntimeBinding { name }` has a `requires_bindings` entry. Validator surfaces the per-surface binding contract uniformly across the recipe tree.
3. **Runtime introspection APIs.** `vfx_recipes::introspect_substitutions(json) -> SubstitutionContract` and `vfx_recipes::introspect_bindings(json) -> BindingContract` for consumers that didn't author the recipe (editors, generic players, capability negotiation).
4. **Strict-mode `Substitutions` loader by default.** Missing substitutions produce loud errors listing what's expected and what's supplied, not silent wrong behavior. `RuntimeBindings` has no strict-mode equivalent; per-frame misses are graceful fallbacks by construction.

Optional fifth mechanism: compile-time code generation from `requires_substitutions` and `requires_bindings` (proc-macro or `build.rs`) producing typed `Substitutions` and `RuntimeBindings` structs for consumers that ship against specific recipe versions. Niche; runtime APIs are enough for most cases.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — input behind Concern D's resolution):** move the API upstream, but separate load-time substitutions from runtime bindings. Plus: explicit declared contracts, strict-mode default, introspection API, byte-based asset resolution. This was load-bearing for the two-surface split captured above. Implementation-mechanics sub-questions remain open.

### 15. Vocabulary refresh scope — comprehensive is the right default

Principle 3 names the tension: vocabulary still carries notification archaeology (`auto_dismiss_ms`, `anchor`, `continuous`, `enter/dwell/exit`, much of the examples and SKILLS reference). V3 is a natural moment to do a vocabulary pass alongside the structural pass.

**Direction: lean comprehensive.** Given V3 is a clean break (see Why Now), the "how aggressive" question's answer defaults to "as aggressive as makes the final vocabulary most honest." Options considered:

- **Conservative** (keep all current field names) — no vocabulary change. Rejected — doesn't earn its place given we're breaking everything else. Preserves archaeology for no reason.
- **Moderate** (rename obvious notification-isms, keep phase vocabulary if apt) — `auto_dismiss_ms` → `duration_ms`; `continuous` → something neutral (`ambient`? `persistent`? needs thinking); possibly `anchor` → `placement`. Phase vocabulary (`enter/dwell/exit`) may survive if it genuinely names something general about arriving/present/departing, not just notification lifecycle.
- **Comprehensive** (full pass with neutral terminology throughout) — the recommended direction. Rename phases if `enter/dwell/exit` doesn't fit widgets/scenes/movies well; rename all notification-flavored fields to neutral equivalents; retire terminology that encodes notification assumptions. This is where V3 should aim unless specific terms turn out to genuinely generalize.

**Specific renames to evaluate during V3 implementation** (with lean for each):

| Current | Candidate | Lean | Notes |
|---|---|---|---|
| `auto_dismiss_ms` | `duration_ms` | Rename | Toast-centric term; "duration" is neutral for splash, ambient, movie beats |
| `anchor` | `placement` | Rename | "Anchor" has notification-specific connotation; placement is neutral |
| `continuous` block | `persistent` or integrated into tree schema | Rework | Implied non-persistent was the default; reversed in widget/ambient contexts |
| `enter/dwell/exit` | *Possibly* `arriving/present/departing` | Evaluate | If the developer-feedback work surfaces that enter/dwell/exit is genuinely apt for the general case, keep; otherwise rename |
| `notification_*` fields | Remove `notification` prefix | Rename | Field names that still reference "notification" as a concept |
| `schema_version` (recipe root) | Keep | Keep | Version concepts are domain-neutral |

Decision for specific renames gets finalized with the translation study (Workflow C in the sibling audit-workflow doc) in hand. The study will reveal which terms genuinely generalize and which carry notification weight the authoring should shed.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** comprehensive-but-selective — a meaningful divergence from the plan's current "comprehensive (full pass)" default. Specific leans:
- **Rename** `auto_dismiss_ms` (aligns with plan)
- **Probably rename / rework** `continuous` (aligns with plan)
- **Rename** preview seam nouns/modules (Open Q #19 — aligns with plan)
- **Keep `anchor`** unless semantics change — in a ratatui/grid context, anchor is already a good geometry term (plan currently leans rename)
- **Keep `enter/dwell/exit`** unless translation study proves they are actively misleading (plan currently leans evaluate-and-possibly-rename)

From a ratatui-centric consumer perspective, the reviewer argues `anchor` and `enter/dwell/exit` generalize better than the plan gives them credit for. Worth resolving during the translation-study phase.

### 16. Cross-step hint resolution rules

Decision 7 establishes step output hints as first-class, modeled as the distinct `HintRef<T>` type (see the `ParamValue<T>` / `HintRef<T>` split in Decisions 6 and 7). Multiple implementation questions remain for `HintRef<T>` semantics:

- **Multiple producers:** when two upstream steps in the same pipeline produce hints with the same name, which wins? Options: first-producer, last-producer, explicit reference by producer ID, compositor-error (forbidden ambiguity).
- **Hint composition:** can a step bind to "displacement from step A multiplied by the signal from step B"? This compounds Decisions 6 and 7 in a way that might need special syntax.
- **Scope of hint visibility:** are hints visible only within the same layer, across layers (so a scene-level step can read a layer-level hint), or both with explicit qualifiers?
- **Hint lifetime:** do hints persist across frames, or are they recomputed every frame? Probably the latter, but the validator needs to enforce it.

These are implementation-level questions but they affect schema shape — the answers determine whether hint references are bare names (`displacement`) or scoped (`layer.flag.displacement`).

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):**
- Visibility defaults to **same pipeline / same layer only**
- Cross-layer reads require explicit export/import semantics if they exist at all
- Hint lifetime is per-frame / ephemeral
- **Multiple producers for the same visible hint should be a validator error unless explicitly qualified** — not "first wins" or "last wins" silently. *"That is too brittle."*

Decision 5's implementation track (Concern E) already defaults to same-layer-only hint visibility, aligning with this lean.

### 17. Primitive library / `$use` fragment composition

The schema already supports two hierarchical composition mechanisms, both shipping and in production use:

- **`extends` — full-recipe inheritance.** A recipe declares `"extends": "themes/new_wopr_fullscreen_cyan.json"` and inherits everything from the base, overriding only the fields that differ. Used extensively in the `wargames/` recipes (57 files extend from 9 base themes in `wargames/themes/`). Implementation: `fnc_resolve_recipe_template.rs` + `fnc_deep_merge_json.rs`.
- **`template + variants` — multi-recipe expansion from one file.** One file carries a `template` block + a `variants` array; loading via `load_all` / `from_value_all` yields N concrete recipes. Collapsed the easing directory from 30 files to 4. Implementation: `fnc_expand_variants.rs`. (Correction in progress per Intention 51 / Principle 4 — see Retrospective corrections below.)

The missing third mechanism is **named reusable fragments / primitive library** — small chunks (not full recipes) that multiple unrelated recipes can reference without inheriting from a common base. Candidate shape:

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

**Sub-questions to resolve:**

- **Parameterized fragments.** Can a primitive take substitutions? `{{color}}` inside the fragment resolves when the recipe uses it. Overlaps with tokenization (Open Q #14) — fragments could be parameterized mini-recipes using the same Substitutions machinery.
- **Fragment inheritance.** Can a primitive extend another primitive? Probably yes for consistency.
- **Fragment versioning.** Primitives in a shared library gain dependencies; editing one ripples to every consumer. Needs versioning discipline — especially for cross-repository sharing.
- **Compile-time vs load-time resolution.** Fragments could resolve at recipe-compile time (fully flattened before ship) or at load time. Load-time is more flexible; compile-time produces smaller runtime payloads and is validator-friendlier.
- **Principle-4 compliance.** Fragments are a consolidation mechanism — they must ship with tooling that preserves individual-fragment addressability (e.g., `shared/primitives.json#computer_typing` as a resolvable URI; demo app understands and exposes each fragment as inspectable).
- **Earned-place discipline.** Consistent with Decision 2's "named compositions earn their place" — fragments are added to the shared library when they encode reusable design judgment, not when an author finds themselves typing the same block twice. Copy-paste is fine for two uses; the third use earns a fragment.
- **Interaction with the Morris principle.** A primitive library is a *curated* collection — the Morris test applies: every shared fragment must be useful or beautiful (or ideally both) to the recipes that reference it. Fragments that only exist because they *could* be shared don't earn their place.
- **Theme-scoped vs global fragments.** A theme may want its own named fragments ("grimoire.ember_support", "harbor.hidden_rail_shell") that only exist within that theme's namespace. Should those live in the global primitive catalog alongside library-wide shared fragments, or in a theme-scoped namespace? Theme-scoped is probably right — grimoire's ember-support doesn't belong in the harbor recipe set — but the shape needs discussion.
- **Justification bar and approval mechanism.** Consistent with Decision 2's "earn their place" principle: what's the threshold for promoting a pattern to a shared fragment? "Used in three recipes" is a rule-of-three heuristic but not the whole answer — some fragments should be named after one use if they encode a specific design judgment worth locking in. A lightweight authoring guideline (e.g., every new fragment carries a `justification:` field explaining what it locks in) beats a committee review.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** yes, but keep v1 minimal and non-blocking — one fragment mechanism, flattened at load time, parameterization via the same substitution system (Open Q #14 / Concern D), no fragment inheritance in v1 unless a real case demands it, strict addressability + introspection from day one.

### 18. Step-level `RoutingRole` and recipe-level `SurfaceIntent` — two consumer-hint types, not one

"Role" is overloaded in the V3 surface and needs a sharper split before implementation. V3 distinguishes **four** separate role-shaped concepts; Open Q #18 is about the two that are new in V3 (the other two exist or are renamed elsewhere):

| # | Type | Scope of application | Home | Status |
|---|---|---|---|---|
| 1 | `RoleTag` | Per-cell render role on cells produced by sources | `tui-vfx-types` | Existing — unchanged |
| 2 | `ThemeRole` | Theme-resolved semantic cell targeting via `Scope::ThemeRole(...)` | `tui-vfx-recipes` scope module | Decision 1 variant (renamed from `Role` in v0.11.0) |
| 3 | **`RoutingRole`** | **Per-step behavior hint** | `tui-vfx-recipes` step field | **Open Q #18 — this question** |
| 4 | **`SurfaceIntent`** | **Per-recipe hosting hint** | `tui-vfx-recipes` recipe field | **Open Q #18 — this question** |

`RoutingRole` and `SurfaceIntent` are kept as separate types rather than collapsed into a single `Role` field because their consumers, value sets, and evaluation contexts differ:

- **`RoutingRole`** governs *what kind of work a step does within a recipe*. Consumers are runtime behavior engines — reduced-motion skipping, performance tiering, screen-reader dispatch, probe/trace filtering. Working vocabulary: `content`, `affordance`, `feedback`, `alert`, `decoration`.
- **`SurfaceIntent`** governs *what container a recipe belongs in when hosted*. Consumers are the surface/hosting policy layer (gt-design's surface identity choice per Open Q #20, movie player's scene placement, toast manager's lifecycle policy). Working vocabulary: `splash`, `toast`, `modal`, `transition`, `ambient`, `movie`.

A splash recipe (`SurfaceIntent::Splash`) can contain many steps, some of which are `RoutingRole::Decoration` and some `RoutingRole::Content`. These value sets don't correspond — collapsing the two types would force the hosting layer to reason about step-level values it doesn't care about, and the reduced-motion engine to reason about recipe-level values it doesn't care about. Principle 5 applies: `RoutingRole` is policy about *how steps behave*, `SurfaceIntent` is policy about *how recipes are hosted*. Different layers earn different types.

**Motivating use cases (RoutingRole):**

- **Accessibility / reduced-motion routing.** A step tagged `RoutingRole::Decoration` can be skipped under reduced-motion; `RoutingRole::Affordance` or `RoutingRole::Feedback` is preserved. Today this requires per-widget hard-coded rules.
- **Performance tiering.** On slow terminals, drop steps with `RoutingRole::Decoration` but keep functional roles.
- **Screen-reader dispatch.** A step's `RoutingRole::Alert` triggers ARIA-equivalent announcement; `RoutingRole::Decoration` is silent.
- **Probe/trace filtering.** Routing-tagged paths become filterable in the debugging surface.

**Motivating use cases (SurfaceIntent):**

- **Surface-identity dispatch.** `SurfaceIntent::Toast` informs gt-design that the recipe wants a toast-class surface (lifecycle, chrome, placement); `SurfaceIntent::Splash` asks for a splash-class surface. Coordinates with Open Q #20 (surface identity vs neutral substrate).
- **Movie-composer scene placement.** A movie player scheduling recipes as timeline scenes uses `SurfaceIntent::Movie` or `SurfaceIntent::Transition` to inform placement and lifecycle.
- **Lifecycle-policy defaults.** `SurfaceIntent::Modal` might inherit different `auto_dismiss_ms` defaults than `SurfaceIntent::Toast`.

**Design choices common to both types:**

- **Open vs closed vocabulary.** Closed = fixed list, safer validation. Open = any string, more flexible. Recommended hybrid for both: canonical enum with `Custom(String)` escape hatch. Canonical entries get documented meanings; custom entries pass through for experimental use.
- **Hints vs contracts.** Both are **consumer hints, not contracts.** Validators warn on unknown values but don't reject recipes. Preserves composability — a movie player doesn't need to honor every `RoutingRole` a theme author invents, and a toast surface doesn't need to handle every hypothetical `SurfaceIntent`.
- **Containment interaction (RoutingRole specifically).** If a step has `RoutingRole::Decoration` and its parent container has `RoutingRole::Content`, what wins? Neither — roles describe what each element *is*, not inheritance. A decorative step inside a content container is simultaneously both things; downstream consumers decide how to combine. Validator may warn on contradictory nesting but does not enforce.

**What these do NOT collapse into:**

- `RoleTag` — a per-cell render vocabulary for the compositor (Background, Text, Shadow, etc.). Not extended upward to step or recipe levels.
- `ThemeRole` — a scope selector variant (Decision 1). Not the same as `RoutingRole` or `SurfaceIntent`; it targets cells by theme binding, it does not hint behavior or hosting.
- Recipe metadata tags (`use_cases`, `aesthetic_tags` — see Open Q #21 / metadata fields section). `use_cases: ["splash"]` and `SurfaceIntent::Splash` may reinforce each other but serve different purposes: metadata is for discovery, `SurfaceIntent` is for routing.

**Vocabulary-collision note.** `RoleTag::Decoration` (a per-cell render role) and `RoutingRole::Decoration` (a step-level behavior hint) share a name but mean different things. This is acceptable — context disambiguates — but if the overlap proves confusing in practice, renaming one side (e.g., `RoutingRole::Accent`, or `RoleTag::Ornament`) is a cheap fix during implementation. Overlap in vocabulary is allowed only where the concepts truly coincide.

**Working canonical vocabulary (starting list for pressure-testing, not commitments):**

| Type | Canonical values | Routing/hosting implication |
|---|---|---|
| `RoutingRole` | `content`, `affordance`, `feedback`, `alert`, `decoration` | Reduced-motion skipping, perf tier, screen-reader priority, probe filter |
| `SurfaceIntent` | `splash`, `toast`, `modal`, `transition`, `ambient`, `movie` | Surface-identity dispatch, lifecycle-policy defaults, scene placement |

**Phase-vocabulary collision avoidance (RoutingRole).** Candidate values like `arriving` / `present` / `departing` that were floated in earlier drafts overlap with phase vocabulary (`enter` / `dwell` / `exit`). Phase and routing role must have disjoint meanings — phase is *when*, routing role is *what the element is*. Avoid naming routing-role values that describe temporal position.

**Scene-layer `role_tag: RoleTag` audit note (implementation-level concern, not blocking V3 direction).** The scene-layer field introduced in Sub-plan B.1 binds `RoleTag` (per-cell render role). Under the four-type split it's worth auditing whether that field is genuinely naming a per-cell render role or whether it's secretly carrying `RoutingRole`-shaped or `SurfaceIntent`-shaped intent while wearing `RoleTag`'s clothes. If the scene-layer role is closer to "this layer is an alert" than "this layer's cells render as alert-ish content," the field may need retyping during V3 implementation. Flag only; resolution during implementation design.

**Addresses feedback items:** 4 (distributed policy — partially, through explicit hint-vs-contract discipline), plus Concern C of the 2026-04-21 GT-Design lead review memo (role-domain split).

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — input behind Concern C's resolution):** yes to routing metadata; explicit "no" to collapsing into the existing `RoleTag` domain. Floated four candidate names (`routing_role`, `surface_intent`, `playback_role`, `semantic_tag`). This was load-bearing for the four-type split captured above. Sub-questions (canonical vocabulary content, scene-layer `role_tag` audit) remain open.

### 19. "Preview" naming for the canonical engine seam

`PreviewItem`, `PreviewManager`, and the `src/preview/` module path are the real canonical engine primitives today — they are what consumers wrap to reach recipe playback. But the "preview" name connotes "demo/temporary" rather than "canonical/authoritative." This mismatch creates friction when downstream systems (gt-design and future consumers) are deciding how to wrap the seam:

- Is `PreviewItem` the authoritative engine surface, or just a demo helper?
- Should gt-design treat it as a thing to build on top of, or as a diagnostic-only surface?
- If someone new to the codebase sees `PreviewManager`, do they understand it's the canonical lifecycle primitive or do they assume it's a preview-specific utility?

This is part of the weak-seams feedback (item #6 — naming doesn't match abstraction) and specifically the developer's upstream point #2. The canonical seam's name should communicate its role.

**Candidate renames:**

- `PlaybackItem` / `PlaybackManager` — names what the object actually does (represents a playback unit and its lifecycle)
- `RecipeItem` / `RecipeManager` — names the domain (recipes)
- `ItemManager` with `src/items/` — simplest; relies on context
- Keep `PreviewItem` but rename the module path (`src/preview/` → `src/playback/` or `src/items/`)
- Keep both names via re-exports during a deprecation window

**What's at stake:**

- Rename touches the crate's public API surface (breaking change, coordinated with Decision 4's Ra→Vfx pass).
- Every consumer that imports `PreviewItem` / `PreviewManager` updates (gt-design, tools, tests).
- Docs, SKILLS.md, examples, and the validator/probe/trace infrastructure all reference these names.

**Why this is V3-scoped:**

V3 is already rewriting the recipe schema and renaming the `Ra*` prefix (Decision 4). Bundling the "preview" rename with V3 means one rename event instead of two. If we defer, every consumer pays the cost of "wait, what does PreviewItem actually do?" in perpetuity, and a future rename becomes another breaking change.

**What not to rename:**

- The upstream `tui-vfx-recipes` demo binary (`cargo run --example demo`) genuinely *is* a demo/preview app. Its name is accurate.
- Debug recipes at `debug_recipes/` are debug/preview — accurate.
- The *seam type* is what's misnamed, not the test/demo infrastructure around it.

Addresses feedback items: 6 (naming doesn't match abstraction), upstream-relevant point 2.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** rename now. Module path → `playback`; manager → `PlaybackManager`; seam type → possibly `PlaybackItem`, but also worth considering **`PlaybackPlan` / `PlaybackUnit`** as more future-proof once scenes and multi-layer content are first-class. Would not keep `Preview*` on the seam.

### 20. Surface identity vs neutral substrate — `RecipeSceneCanvas` overload

`RecipeSceneCanvas` today does two different jobs:

1. **Neutral substrate for recipe-first playback** — the architectural role Intention 50 names. A recipe scene hosted on this substrate gets GTD display-truth semantics (clipping, motion boundaries, depth-backdrop policy, runtime params).
2. **Family-specific surface identity** — being used where a more specific identity (toast, notification, modal, tooltip) would be more truthful. This is semantically right (the content renders correctly) but wrong for component-level inheritance (toast should inherit from Toast theme selectors, not from a generic canvas selector).

This is the weak-seams feedback item #2 — correct semantics, wrong component-level inheritance. Per Principle 5 (meaning low, policy high), the substrate is meaning (it defines what recipe-first scene hosting *means*), while surface identity is policy (it defines *how a product uses* the substrate in context). Mixing them into one type entangles two concerns that should live at different layers.

**Candidate V3 resolutions:**

- **(A) Keep `RecipeSceneCanvas` strictly as neutral substrate.** Require gt-design to produce family-specific surface identities (`ToastSurface`, `ModalSurface`, `TooltipSurface`, ...) that wrap the substrate. Theme inheritance dispatches on family identity, not on substrate identity.
  - *Pros:* clean meaning/policy separation. Each surface family gets correct theme dispatch. Substrate stays generic and reusable.
  - *Cons:* more types. gt-design pays the wrapping cost.
- **(B) Add explicit identity tags to `RecipeSceneCanvas`.** The substrate carries an optional `surface_kind: SurfaceKind` that theme dispatch reads.
  - *Pros:* single type, less wrapping ceremony.
  - *Cons:* keeps the overload. Theme dispatch becomes canvas-aware in a way that smells like policy leaking into meaning.
- **(C) Hybrid:** substrate stays neutral, but gt-design provides a thin `SurfaceIdentity` trait / registry that maps recipe content to family identity. Theme dispatch reads identity, not canvas.
  - *Pros:* less code than (A), cleaner than (B).
  - *Cons:* the indirection adds cognitive load.

Strong lean toward (A) because it respects Principle 5 most cleanly: the substrate is one concept (meaning), surface identity is another (policy), and they each belong to their natural layer. The wrapping cost is real but bounded — six-to-ten family-specific surface types cover the authoring cases gt-design actually has (toast, modal, tooltip, splash, drawer, ambient-backdrop, training-overlay, etc.), and each one can be thin.

**What's at stake:**

Getting this right enables correct component-level inheritance for family-specific surfaces. Getting it wrong means toast themes and modal themes can't cleanly express their differences through V3's theming surface — the overload forces theme authors to condition on implicit context.

**Relationship to other decisions:**

- Decision 5 (scene layers) — each family-specific surface could have default scene-layer composition; recipes don't re-invent shell geometry.
- Decision 8 (canonical upstream seam) — the canonical builder's output is neutral; surface identity is applied *after* the builder in gt-design's policy layer. Clean separation.
- Open Q #18 (`RoutingRole` / `SurfaceIntent`) — `SurfaceIntent` on the recipe (`splash` / `toast` / `transition` / etc.) informs gt-design's surface-identity choice when wrapping. The hint-vs-contract discipline (Open Q #18) keeps `SurfaceIntent` as a consumer hint that surface-identity layers wrap rather than inherit verbatim — Principle 5 applies (upstream meaning, downstream policy). Coordination with Open Q #20 ensures `SurfaceIntent` and gt-design surface identity don't double-encode the same thing.

Addresses feedback items: 2 (preview surface identity too generic), 4 (distributed policy — in part).

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** choose **option A**. Keep `RecipeSceneCanvas` as the neutral substrate family; gt-design wraps with family-specific surface identities. *"That aligns with GTD's current steering best: RecipeSceneCanvas is the substrate family, surface identity is higher-level policy, and internal variants (`RawRecipeSceneCanvas`, `ResolvedRecipeSceneCanvas`) can exist without changing that public conceptual split."* Aligns with the plan's strong lean.

## Deferred for later design rounds

Not open questions (because they're not blocking progress), but things we'll need answers to before the plan graduates from draft to implementation schedule, plus adjacent design territory that V3 decisions should *not foreclose* even though V3 itself doesn't deliver them.

### Implementation-sequence items

- Composition-container vocabulary: do we need `Sequence` and `Parallel` only, or also `Race`, `FirstOf`, `Conditional`? Tachyonfx has only `sequence` and `parallel`. Start with two, add more only on demand.
- Scope composition precedence when a container propagates scope X and a child declares scope Y (Union? Intersection? Child-wins? Parent-wins?). Probably intersection, but needs verification against the translation study.
- Performance: does the tachyonfx static/dynamic analyzer pattern (bitmask caching for static scope predicates) port cleanly to our compositor? Probably yes, but it's a design we haven't attempted yet.
- Serialization round-tripping: can a loaded recipe round-trip back to canonical JSON? Tachyonfx's `to_dsl()` is a model worth studying.

### Movie-composer territory

A higher-order orchestration layer above recipes — conceptually a "script" or "movie" that composes multiple recipes across time. Not in V3 scope, but V3 decisions should not foreclose it.

**Conceptual shape:**

```text
MovieScript ::= {
  scenes: Vec<SceneBeat>,
  globals: { default_transition?, clock?, seed? }
}
SceneBeat ::= {
  recipe: RecipeRef | InlineRecipe,
  start: AbsoluteMs | AfterPrevious { delay_ms } | OnEvent(EventRef),
  duration: Fixed(ms) | Recipe | UntilNext,
  transition_in?, transition_out?,
  overrides?: ParamSubstitutions,  // connects to Decision 6 and open Q 14
}
```

**Use cases that motivate capturing this now:**

- **Terminal movies / short films** — the original user-raised concept. A scripted sequence of recipes playing through a terminal as self-contained content.
- **Training content** — a gt-design tutorial shipped as an offline movie that walks through features without needing the full app.
- **Non-interactive demos of gt-design applications** — marketing/documentation content that shows an app's behavior without requiring the app itself.
- **Terminal recordings** — deterministic playback to a stream file (asciinema-compatible or proprietary) for docs hero videos.
- **CI visual regression** — render a movie frame-by-frame to golden artifacts; diff at the cell grid.
- **Embeddable demos in documentation** — wasm-compile the grid renderer + script player, inline demos in mdBook or docs.rs.
- **Static image export** — SVG, PNG, or SIXEL emitters that consume the same grid; README thumbnails.

**Home crate:** `gtd-movie` (or `tui-vfx-movie` — TBD which layer). Plausibly a thin player binary: script parser, recipe loader, clock, grid-to-terminal emitter, minimum keyboard-for-skip. A few thousand LOC, no ratatui dependency. This is the concrete cash-in on the ecosystem-agnostic architecture (see Architectural framing above).

**What V3 must not foreclose:**

- Recipes should not assume they own their own clock. A movie scene might inject a shared clock or a substitution-resolved clock.
- Recipes should not assume they own their full duration. A movie might start a recipe and transition it out before its natural `auto_dismiss_ms` fires.
- The two-surface substitution API (Open Q #14 — `Substitutions` load-time, `RuntimeBindings` per-frame) must support per-scene parameter overrides cleanly. Per-scene `Substitutions` resolve at scene load (each scene in the movie can have its own load-time context); per-scene `RuntimeBindings` let the movie player drive per-frame values mid-scene (e.g., animating a recipe parameter across a scripted timeline).
- `ParamValue::RuntimeBinding` (Decision 6) remains usable at the movie level — a movie player supplies values to recipe `RuntimeBindings`, orchestrating animation across scenes without requiring per-recipe customization.

**Not in scope for V3 plan.** Captured here so the future design has a home.

### Recipe migration workflow

The canonical migration framing lives in Open Q #2's three-phase model (Curate → Re-author → Validate, with a fixture-equivalence carve-out for designated critical recipes). This section covers implementation-level workflow mechanics that don't affect V3 structural decisions but need resolution before the migration runs.

**Curation-phase mechanics:**

- The curation phase is Workflow B in the sibling audit-workflow doc. It runs over the complete inventoried V2 corpus; inventory is its own blocking prerequisite (current filesystem evidence suggests >500 files, authoritative count still pending).
- Per-recipe disposition (port / consolidate / archive / delete) is captured in a checked-in manifest with rationale. Disposition categories align with Workflow B's exit criteria in the audit-workflow doc.
- Morris principle (useful or beautiful) is the filter. A recipe with no clear use case and no aesthetic justification is a candidate for archive or delete, not automatic port.

**Re-authoring-phase mechanics:**

- Claude is pointed at the V2 JSON and at the retained-set disposition. The authoring briefing (SKILLS.md, on-disk vocabulary references, prompt scaffolds, authoring guides) drives V3 recipe generation. Claude also authors the V3 metadata block (aesthetic_tags, use_cases, related_themes, etc. — see the "Recipe metadata fields" section below) per Open Q #21.
- When Claude struggles on a recipe — unclear vocabulary, missing primitive, ambiguous scope semantics, briefing docs that don't cover the case — the symptom is a briefing gap. Fix is a briefing-document update; recipe retry follows. Briefing improvements commit alongside the recipes that motivated them, with the recipe cited as evidence in the commit message.
- Bulk automation is deliberately rejected for the mainline corpus. Per-recipe attention is the point — it is how the library's AI-authoring pathway earns its stripes. Most recipes getting a closer look is a feature, not a cost.

**Validation-phase mechanics:**

- The V3 validator (Open Q #9) runs on every re-authored recipe. Schema well-formedness, scope coherence, hint-namespace membership, fragment addressability, binding-contract discovery, required-field presence.
- `pipeline-validator --debug-recipes-qc` fingerprinting supplements validator coverage for recipes that opted into QC fingerprint gating. Fingerprint drift is surfaced but not auto-blocking for mainline recipes — re-authoring may legitimately shift fingerprint output when the new recipe intentionally renders differently.
- Validator failures block merge for the affected recipe. Re-author and re-validate until green.

**Critical-set carve-out mechanics (fixture track):**

- Pre-migration fixture capture for the designated set uses the existing probe / snapshot infrastructure. Captured artifacts are checked in before the corpus reshape begins.
- The mechanical translator for this set is narrow-scope tooling: tree reshaping, Ra→Vfx rename, ParamValue/HintRef/StepInput wrapping, default population. It does not attempt curation; it does not run over non-critical recipes.
- Fixture-equivalence gate tolerance is per-recipe-kind (exact for probe-validation; perceptual-delta for splash-class visuals; structural for scenes with deliberate V3 improvements). Tolerance choice is documented alongside each fixture.

**Not in scope for V3 plan:** specific tool/script authoring, prompt scaffolding refinement, fixture tolerance calibration, per-recipe briefing improvement cadence. Captured here so the future implementation has a home.

### Recipe metadata fields for discovery and categorization

Today, recipes carry lightweight identification metadata: `id`, `title`, `description`, `version`, `last_updated`, `schema_version`. This is enough for individual-recipe identification but too thin for a library-at-scale story. At 200+ recipes organized across themes and shipped as a reference collection, authors (human and AI) need discovery machinery:

- *"Show me warm, tactile recipes that pair with grimoire-like themes."*
- *"What recipes are canonical splash examples?"*
- *"Which recipes use the staggered-fade aesthetic?"*
- *"Give me minimal, restrained recipes — nothing theatric."*

V3 adds a `metadata` block to recipes with discovery/categorization fields:

```json
{
  "id": "splash.gt_design_default",
  "title": "GT Design default splash",
  "description": "...",
  "metadata": {
    "aesthetic_tags": ["warm", "typewriter", "restrained"],
    "mood": "welcoming",
    "related_themes": ["harbor", "blueprint"],
    "use_cases": ["splash", "first_run"],
    "maturity_era": "mature",
    "authoring_notes": "Uses the canonical splash architecture; swap brand colors via {{brand}} token.",
    "last_reviewed": "2026-04-21"
  }
}
```

Candidate fields (not all required; open to expansion as authoring needs surface):

| Field | Purpose | Example values |
|---|---|---|
| `aesthetic_tags: [string]` | Visual/motion-character tags for discovery | `warm`, `cold`, `restrained`, `theatric`, `noir`, `minimal`, `maximal`, `retro`, `modern` |
| `mood: string` | Single-word emotional tone | `welcoming`, `urgent`, `meditative`, `energetic` |
| `related_themes: [string]` | Which themes this recipe pairs well with (or is authored for) | `grimoire`, `harbor`, `blueprint`, or `theme-neutral` |
| `use_cases: [string]` | Canonical authoring contexts | `splash`, `error-toast`, `modal-reveal`, `scene-transition`, `ambient-background`, `training-demo` |
| `maturity_era: string` | Which development-era this recipe represents (for the Morris audit) | `basic`, `theatric`, `mature`, `professional`, `impressively-theatric` |
| `authoring_notes: string` | Free-form notes from the author (intent, substitution hints, related-recipe context) | — |
| `last_reviewed: ISO date` | When was this last validated against current design standards | `2026-04-21` |

**Why metadata earns its place:**

- **Discovery at library scale.** 200+ recipes without structured metadata become hard to navigate; structured metadata enables SKILLS.md-driven discovery ("find me a recipe that matches this intent"), validator-driven taxonomy ("every recipe should have a use_case"), and future tooling (recipe browser with faceted filtering).
- **The Morris principle becomes enforceable.** Recipes categorized by `maturity_era` and with explicit `use_cases` are auditable against the useful-or-beautiful filter. A recipe with no use case and a basic-era maturity tag is visible as a candidate for retirement.
- **Cross-theme pairing becomes authored.** Today, "which recipes fit the grimoire aesthetic" is tribal knowledge. `related_themes` makes it declarative.
- **AI-assisted authoring quality improves.** When generating or modifying recipes, Claude can filter by `aesthetic_tags` to find reference examples that match the intent. Lower hallucination risk, higher coherence.
- **Intentional choices become visible.** `authoring_notes` is where the author writes "I chose this specific parameter tuning because..." — the rationale that otherwise lives only in commit messages or author memory.

**Relationship to Open Q #18 (`RoutingRole` / `SurfaceIntent`):**

`RoutingRole` (step-level) and `SurfaceIntent` (recipe-level) from Open Q #18 are *downstream routing and hosting hints* — they tell consumers how to treat steps and recipes at render/host time (accessibility dispatch, reduced-motion handling, screen-reader priority, surface-identity choice). Metadata tags (this field) are about *discovery and categorization* — they tell authors and tools how to find and understand the recipe. Different purposes, different fields, should stay separate. They may overlap in specific values (`use_cases: ["splash"]` and `SurfaceIntent::Splash` reinforce each other for the same recipe), but the Open Q #18 fields are optional routing/hosting hints while metadata fields are optional discovery tags.

**This is a new Open Question (#21) pending its own discussion on vocabulary and required-vs-optional:**

- Which fields are required vs optional? Probably `use_cases` required (every recipe justifies its existence via use case), the rest optional.
- What's the aesthetic tag vocabulary? Open-string or closed-enum? Probably hybrid — canonical list with custom values allowed.
- Does validation enforce anything beyond schema-shape (e.g., "every recipe must have at least one `use_case`")?
- Does metadata live inside `config` or as a sibling to it? Probably sibling — metadata is about the recipe, not the playback contract.

**Reviewer's opinion (2026-04-21 GT-Design lead review memo — one input, question remains open):** keep metadata non-blocking for V3 core. `use_cases` should likely be required; most other fields can be optional initially. Discovery metadata (this field) should stay clearly separate from runtime routing metadata (`RoutingRole` / `SurfaceIntent` per Open Q #18). Aligns with the plan's current lean.

### Retrospective corrections

Not all V3 work is forward motion — some is repairing prior decisions per Principle 4 / Intention 51. Known retrospective corrections in progress or queued:

- **`easing_family.json` decomposition.** The 26-variant `template + variants` consolidation regressed individual-recipe addressability. Correction: re-expand into 26 individual `ease_<snake_case>.json` files while preserving the consolidated file under a `_DEPRECATED_` prefix. In progress at the time of this plan revision (sub-agent dispatched, produces 26 files + deprecated source, all passing `pipeline-validator`). Once confirmed, the deprecated file can be deleted.

- **Other recipes to audit for the same pattern.** Any other `template + variants` consolidation in the corpus should be audited for Principle 4 compliance during the corpus audit (Workflow B in the sibling audit-workflow doc). Consolidation is acceptable when its tooling support is also in place; it's not acceptable when the tooling gap regresses debug/preview workflows.

### StaggeredLines content effect (PRD primitive 5)

Ergonomic nice-to-have from the flag-animation PRD. Once Decision 5 (scene layers with per-layer pipelines) + Decision 6 (signal-driven parameters, including per-layer schedule delays) are in place, three independently-timed text lines can be authored as three text layers with `schedule.enter_delay_ms` per layer. `StaggeredLines` is sugar on top of that common pattern — one `ContentEffect` variant instead of three layers — and is worth shipping when the usage repeats often enough to justify the named helper (per Decision 2's earned-place logic for named compositions). Not V3-structural; defer until real demand surfaces.

### Distribution and packaging story for recipes and themes

Not part of V3 schema work, but a real gt-design-level concern that V3 should not foreclose and that needs its own design discussion when the time comes.

**The problem.** gt-design consumer apps (CLIs, TUIs, agents built on top) depend heavily on themes and recipes. Today these are loaded from disk. That means every consumer app has to ship and track hundreds of JSON files alongside its binary — awkward for distribution, fragile under install/update, and forces every downstream developer to own asset-management plumbing they didn't want to write.

**What developers need is choice, not a single loading mode:**

- **Compile-time embedding** — bundle themes and recipes into the binary at build time. Single-file distribution; nothing to track besides the executable. Rebuild required to change content.
- **Runtime disk loading** — load from a configured directory at runtime. Easy iteration, user-overridable, but consumers ship file trees.
- **Hybrid / layered** — embedded defaults plus optional disk overrides. Consumers get single-binary distribution by default; advanced users can drop files in a path to override. This is probably the right common case.

**Design sketch (not committed, just naming the shape):**

A `RecipeSource` / `ThemeSource` trait at the `tui-vfx-recipes` / `gtd-ratatui` boundary, with implementations for disk-backed, embedded-backed, and layered (hybrid) sources. A build-time macro or `build.rs` helper that bundles a directory tree into an `EmbeddedRecipeSource` at compile time. A composition primitive that lets consumers stack sources (embedded defaults first, disk overrides on top, for example).

Candidate prior art: `rust-embed` (directory embedding via proc-macro), `include_bytes!` + `phf` for compile-time hash maps, feature-flag toggles for embedded-vs-disk modes.

**V3 constraints that preserve this option:**

- Recipe loaders should accept byte slices, not just file paths. Disk is one producer; embedded memory is another. Both produce `&[u8]` that the loader parses.
- Fragment `$use` resolution (Open Q #17) should work across source kinds — a recipe in one source can `$use` a fragment from another source via a registered source path. No assumption that fragments and their referencing recipes share a filesystem root.
- Tokenization asset resolution (Open Q #14) already abstracts over byte sources (`with_image(name, bytes)` doesn't care where the bytes came from). This is the right pattern for themes and recipes too.

**Why this is explicitly deferred:**

- It's a packaging/distribution concern, not a schema concern. V3's schema decisions don't depend on it.
- Real answers need consumer-app input (what does a gt-design-based CLI actually want to ship?), which is best gathered once V3 ships and real adoption patterns surface.
- The design space has well-known precedents in the Rust ecosystem (rust-embed, include_bytes!, feature flags); we don't need to invent anything novel.

**Captured as:** deferred-design territory with the constraint that V3 loaders / fragment resolvers / tokenization APIs must accept byte-source abstractions, not assume filesystem access. That preservation is cheap; the fuller design can wait for its own session.

### Dynamic recipe formalization

Two recipes at `recipes/dynamic/` today (`digital_rain_matrix_classic_dynamic.json`, `digital_rain_matrix_modern_dynamic.json`) use `{"binding": "<name>"}` for runtime-driven parameters. This works but is ad-hoc — the `/dynamic/` directory convention isn't canonical; the binding syntax is per-shader not uniform; and validators don't specially treat "recipes that require runtime bindings" as a category.

V3 Decision 6 formalizes `ParamValue::RuntimeBinding` uniformly across all step types. Once that lands, dynamic recipes stop being a separate category — they become recipes that use `RuntimeBinding` values in their parameters. The `/dynamic/` directory becomes either a convention for recipes expecting app-side values, or collapses entirely (any recipe can use bindings; the directory structure follows topic, not binding-presence).

**Related open questions** (not blocking V3 direction):

- Should recipes declare their required bindings explicitly in the schema so validators can check that the app supplies them? (The flag-animation PRD proposes `requires_primitives` for capability checking; the same pattern could extend to binding contracts.)
- Does the substitution API (open Q 14) unify with binding resolution, or stay distinct as string-vs-typed-value mechanisms?

Defer until Decision 6 implementation exposes the real shape.

## Appendix — audits and curation

The V3 migration depends on three audit workflows that will produce the empirical inputs the main plan can't answer abstractly: the shape of the primitive catalog, the set of recipes that earn their place in V3, and the validation of the proposed tree structure against diverse real recipes.

- **[Audit & curation workflows](./tui-vfx-v3-upgrade-audit-workflow.md)** — captures three deferred workflows:
  - *Workflow A — Shader catalog decomposition:* per-named-shader evaluation against the `debug_recipes/shaders/` interactive preview. Classifies each of ~27 named shaders as trivial composition / earned name / primitive-itself. Resolves open questions #2 and #5.
  - *Workflow B — Recipe corpus curation:* William Morris principle applied to the 200–300 recipe corpus. *"Have nothing in your house that is not useful or you do not think is beautiful."* Classifies every recipe for port / consolidate / archive / delete. Produces the V3 port list.
  - *Workflow C — Structural translation sample:* 6–8 representative ported recipes re-expressed in the V3 tree shape to stress-test structural diversity. Resolves open questions #3, #4, #6.

  **Status: all three deferred.** Workflows documented for execution in a future session. None blocking; sequencing recommendation included in the workflow doc.

  Each workflow produces its own sibling appendix file when executed:
  - `./tui-vfx-v3-upgrade-appendix-shader-catalog.md` (from Workflow A)
  - `./tui-vfx-v3-upgrade-appendix-corpus-audit.md` (from Workflow B)
  - `./tui-vfx-v3-upgrade-appendix-structural-translations.md` (from Workflow C)

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.8.1</VERS> -->
