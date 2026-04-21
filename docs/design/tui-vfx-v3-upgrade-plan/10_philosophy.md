<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/10_philosophy.md</FILE> - <DESC>Chapter 10 — guiding philosophy: the five principles that shape V3 design (Morris, pipe-culture chain-ability, widgets-and-the-grid, authoring-affordance preservation, meaning-low-policy-high) plus the constraint-vs-permissiveness design discipline. Durable framings that outlast the specific schema decisions.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Guiding philosophy" and "Constraint vs permissiveness" sections. Section numbering uses 10-unit spacing for flexibility; Principles 1-5 keep their semantic numbering. Added an ANSI diagram showing the Meaning/Policy layer cascade per Principle 5.</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith. Principles 1-5 verbatim; constraint-vs-permissiveness discipline verbatim. Meaning-layer ANSI diagram added to 50 as a visual reinforcement of Principle 5's layer test.</CLOG> -->

# 10 — Guiding philosophy

Five principles shape V3 design and will outlast the specific schema decisions below. They are the durable framing; the schema changes are how we apply the framing to the current surface. Principles 1–3 (Morris, Pipe-culture chain-ability, Widgets-and-the-grid) were added in the initial plan draft; Principle 4 (Authoring-affordance preservation) was added in v0.5.0 codifying Intention 51; Principle 5 (Meaning low, policy high) was added in v0.6.0 from the weak-seams feedback session.

## 10 — Principle 1: The Morris principle

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

## 20 — Principle 2: Pipe-culture chain-ability

Morris filters *what earns a place*. Pipe culture shapes *how primitives relate to each other*.

V3 borrows deliberately from the Unix shell-pipe tradition. Each pipeline step is a self-contained primitive with a clear scope / phase / payload contract. Steps produce named outputs (hints) using a uniform output vocabulary; steps that want to react to other steps bind to those outputs by name. Composition is declared at use-site in the recipe tree — no pre-defined pipelines are baked into the library. Every intermediate value between steps is a first-class thing you can inspect, probe, log, or redirect.

Concrete consequences:

- **Every step is self-contained.** No step knows about the internals of any other step. A `DisplacementShade` shader doesn't know which sampler produced its displacement hint; it just binds to "the displacement channel" for the layer.
- **Uniform output/input vocabulary.** Step hints live in a defined namespace (`displacement`, `sampled_color`, `cell_density`, etc.), not per-step ad-hoc naming. Adding a new step type doesn't require touching the hint namespace; bindings work by discovering producers.
- **Composition happens at authoring-time.** The tree schema is the compose-at-use-site pattern. Authors wire outputs to inputs by declaring which bindings each step consumes.
- **Inspectable between stages.** The probe/trace infrastructure already partially supports this; V3 makes it canonical — any intermediate hint should be dump-able.

This is important enough to be structural: the V3 schema must make named step outputs a first-class concept, not a retrofit.

## 30 — Principle 3: Widgets and the grid, not just notifications

The library grew from toasts. That origin still shows in the vocabulary (`auto_dismiss_ms`, `anchor`, `continuous`, `enter/dwell/exit`), the mental model many authors (including me, working on this plan) default to, and the examples and SKILLS reference. But the *capability surface* has already generalized — splash uses the exact same recipe envelope as toasts; the PRD's flag animation is a scene-layer composition with signal-driven motion; the ambient-halo exploration is a recession-field modulation; relative light is an ambient backdrop. None of these are notifications.

V3 design must be reviewed against widgets, grid-level effects, scenes, transitions, and composed movies — not just notifications — before landing. Concretely:

- A hover state on a widget is not a notification but may use enter-phase-like vocabulary.
- A splash screen is a one-shot composed scene, not a long-lived toast.
- A theme-swap transition is a whole-grid effect with no notification anywhere in the picture.
- A tutorial overlay is a layered scene with scoped highlighting and staggered copy.
- A training-demo movie is a composition of recipes across time, not a single recipe.

When a V3 decision feels crisp for notifications but awkward for any of these broader use cases, the decision is wrong or incomplete. The correct response is not to paper over with app-side glue — it is to widen the V3 vocabulary so the general case is first-class. The notification-shaped terms that remain should either be deliberately kept (e.g., `enter/dwell/exit` may survive if it names something genuinely general about arriving/present/departing) or renamed to neutral terms.

## 40 — Principle 4: Authoring-affordance preservation

Any consolidation mechanism V3 introduces — `template + variants`, primitive libraries, `$use` fragment composition, bundler manifests, or any future aggregator — must preserve individual-item addressability for debug, preview, and reference use cases. **The file path and unit identity are UX contracts with tooling, not just storage conventions.**

This principle is the hard-learned lesson from the `recipes/easing/easing_family.json` retrospective: a 26-recipe → 1-file consolidation served `load_all` cleanly but regressed the demo app's file-picker (individual easings became unselectable). The optimization served programmatic consumers at the expense of debug/preview consumers.

V3 consolidation mechanisms must ship *together with* their tooling counterparts. A `template + variants` file is not done landing until the demo app, validator, probe, and trace all understand its expansion and expose each variant as a selectable, addressable item (e.g., `easing_family.json#back_out`). Debug / preview / reference recipes stay as individual files by default. Metadata declares intended consumption (`programmatic` / `individual_preview` / `both`).

This principle is codified as **Intention 51** in `steering/INTENTIONS.md` (version 0.52.0) and applies across every consolidation V3 introduces, not just the specific cases we've talked through. Future additions to the schema (primitive libraries, fragment composition, bundler formats) must pass its filter.

Related: Intention 44 names *when* to extract shared primitives (rule of three). Principle 4 / Intention 51 names *how* any extracted primitive must behave to preserve authoring affordances.

## 50 — Principle 5: Meaning should live as low as possible; policy should live as high as necessary

Every V3 decision should be evaluated against which *layer* owns the concept being encoded. The durable shape is:

- **Meaning** — *what a recipe field does, semantically.* Lives as low as possible: mixed-signals for signal math, `tui-vfx-*` for recipe/pipeline semantics, foundation libraries for their domains. Meaning is the stable contract everyone depends on.
- **Policy** — *how a design system or product applies meaning in context.* Lives as high as necessary: gt-design for theming decisions, consumer apps for surface identity, product layers for family-specific behavior. Policy is where product personality lives.

```
                    ▲
    HIGHER LAYER    │   POLICY (product-specific; many answers possible)
                    │
    ┌───────────────┴───────────────┐
    │   gt-design  │  app-specific  │  ← surface identity, theming,
    │   factory    │  overlays      │    family behavior
    └───────────────┬───────────────┘
                    │
                    │  "meaning passes up; policy flows down"
                    │
    ┌───────────────┴───────────────┐
    │   tui-vfx-recipes             │  ← recipe/pipeline semantics,
    │   tui-vfx-compositor          │    effect-carrier threading,
    │   tui-vfx-style               │    canonical builder output
    └───────────────┬───────────────┘
                    │
    ┌───────────────┴───────────────┐
    │   mixed-signals               │  ← signal math (ONE answer
    │   tui-vfx-types               │    per concept — don't duplicate)
    │   mcu-terminal-color          │
    └───────────────────────────────┘
                    │
    LOWER LAYER     │   MEANING (canonical; single source of truth)
                    ▼
```

Why this matters: when meaning leaks upward into policy layers, the same concept gets re-encoded in multiple places (the `config.shadow` miss is the canonical example — an upstream additive field needed rethreading through GTD's parallel semantic conversion). When policy leaks downward into meaning layers, primitives develop product-specific assumptions that block their reuse (a Diffusion shader that presumes toast lifecycle is a bad primitive).

Practical tests for placing a new concept or field:

- "Does this name *what a recipe does* regardless of who's consuming it?" → meaning → lower layer.
- "Does this name *how a product styles or hosts it*?" → policy → higher layer.
- "Could two different consumers legitimately disagree about this?" → if yes, policy; if no, meaning.
- "If this changes, do all consumers need to change?" → if yes, meaning (they're downstream of a semantic update); if no, policy (only this consumer cares).

This principle generalizes Intention 40 (foundation libraries own domain expertise), Intention 49 (recipe authoring truth upstream, display truth in factory), and the V3 architectural framing (signals go upstream into mixed-signals, V3 consumes them). It's explicit because it needs to actively guide decisions during implementation, not be reconstructed from intuition each time.

Framing credit: proposed by one of the gt-design developers during a weak-seams-review session (see `feedback/2026-04-21-gtd-tui-vfx-weak-seams-feedback.md`). The principle validates several existing V3 positions and gives future decisions a compass.

## 60 — Constraint vs permissiveness — the design discipline

Several V3 decisions constrain tightly (closed algebraic types, deny-unknown-fields, validator-rejected shapes) while others are deliberately permissive (any field can be tokenized, step outputs use an open hint namespace). This asymmetry is intentional and follows a rule:

> **Constrain where correctness depends on closed semantics; be permissive where flexibility helps authors and the cost of "wrong" is bounded.**

Examples of each:

- **Unified Scope (Decision 1) is a closed algebraic type** with typed variants (Area, Channel, Content, Role, Custom) plus algebraic combination (And/Or/Not). Why closed: because the static-vs-dynamic analyzer pattern only works when the scope vocabulary is enumerable (static scopes cache as bitmasks; open scopes can't). Because validators need to catch nonsensical combinations statically. Because a bounded authoring vocabulary is easier for humans and AI to reason about. Escape hatch: `Predicate(fn)` for genuinely custom cases.
- **Universal tokenization (Open Q #14) is permissive** — any string field can contain `{{tokens}}`, loader coerces at parse time. Why permissive: because restricting "which fields are templated" via per-field opt-in flags is structural overhead for no real authoring gain. The alternative (mark each field as templated) doubles the schema surface with no expressiveness win. The risk of unresolved tokens is bounded by strict-mode validation, which catches missing substitutions loudly at load time.
- **Scene-layer source kinds (Decision 5) are a closed enum** (Text, Image, Procedural, Card). Why closed: adding a new source kind requires runtime support (rasterizer, validator, format loader) — it's not just a schema tweak. Extension through the `Procedural` generator registry instead (procedural `source_id` is open-string for generator names, with the registry vouching for each).
- **Step output hints (Decision 7) use a defined namespace** (`displacement`, `sampled_color`, `cell_density`, etc.). Why bounded but not fully closed: downstream bindings need name stability (a step that claims to produce `displacement` must always produce that hint shape), but the namespace itself can grow additively as new hint kinds are needed. Not a closed enum; not fully open either.
- **`ParamValue<T>` and `HintRef<T>` (Decisions 6 and 7) are two related closed types**, composed at field sites via `StepInput<T> = ParamValue<T> | HintRef<T>`. `ParamValue<T>` has three variants (Constant, RuntimeBinding, SignalGraph) covering external value sources; `HintRef<T>` references named step outputs within the same pipeline evaluation. Why closed on both and why kept distinct rather than collapsed into a single four-variant `ParamValue<T>`: the two types have different resolution paths (external substitution/evaluation vs producer lookup against the hint namespace) and different validator work (binding contract discovery vs tree-walk producer verification), and they live at different layers per Principle 5 (external value sources are app policy flowing in; step-output refs are meaning flowing within the pipeline). Fields that only make sense with one side narrow to the appropriate type. The signal-graph variant itself is open via `mixed-signals` composition, so expressiveness isn't limited — just the outer enums.

Future V3 decisions should surface this discipline explicitly: if a proposal adds a closed type, it should answer "why is this correctness-load-bearing"; if it adds permissiveness, it should answer "why is the cost of 'wrong' bounded here." Getting this balance wrong pulls the library toward either excessive ceremony (every field rigidly typed past utility) or magic-soup (no invariants, debugging becomes guesswork).

## 70 — Summary

Migrate the `tui-vfx-recipes` authoring schema from its current flat shape (per-phase slots with asymmetric multiplicity and scoping rules across element types) to a uniform tree schema where every pipeline step carries the same shape — a scoped, phased, composable atom. Internally, decompose spatial shaders into a `ColoredOverlay` + `Pattern` axis model while preserving today's named-factory JSON shapes as sugar so backwards compatibility is a load-time concern, not an authoring-surface break.

Three distinct but related changes, with the intent of landing all three in a single schema version bump (V3) rather than fragmenting the migration:

1. **Unified `Scope` primitive** carried on every step (closed algebraic type: area / channel / content / theme-role / custom / And/Or/Not composition).
2. **Pattern-as-separable-axis** as the internal shader model (`ColoredOverlay { color, pattern, intensity }` with `Pattern` as an open enum of spatial distributions), with named factories (`Diffusion`, `ConcealedLight`, etc.) retained as JSON surface sugar.
3. **Tree authoring schema** replacing flat `pipeline.{mask, filter, sampler, styles}` slots with a recursive `Step | Sequence | Parallel` structure.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/10_philosophy.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
