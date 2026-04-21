<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/90_deferred_design.md</FILE> - <DESC>Chapter 90 — deferred design rounds. Not open questions (not blocking progress) but things that need answers before draft-to-implementation transition, plus adjacent design territory V3 decisions must not foreclose even though V3 doesn't deliver it.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Deferred for later design rounds" section. The Recipe Migration Workflow subsection was promoted to Chapter 50 (full workflow) but a pointer remains here for discoverability.</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith. Subsections preserved verbatim; Recipe migration workflow replaced with a pointer to Chapter 50 because it is no longer deferred.</CLOG> -->

# 90 — Deferred for later design rounds

Not open questions (because they're not blocking progress), but things we'll need answers to before the plan graduates from draft to implementation schedule, plus adjacent design territory that V3 decisions should *not foreclose* even though V3 itself doesn't deliver them.

## 10 — Implementation-sequence items

- Composition-container vocabulary: do we need `Sequence` and `Parallel` only, or also `Race`, `FirstOf`, `Conditional`? Tachyonfx has only `sequence` and `parallel`. Start with two, add more only on demand.
- Scope composition precedence when a container propagates scope X and a child declares scope Y (Union? Intersection? Child-wins? Parent-wins?). Probably intersection, but needs verification against the translation study.
- Performance: does the tachyonfx static/dynamic analyzer pattern (bitmask caching for static scope predicates) port cleanly to our compositor? Probably yes, but it's a design we haven't attempted yet.
- Serialization round-tripping: can a loaded recipe round-trip back to canonical JSON? Tachyonfx's `to_dsl()` is a model worth studying.

## 20 — Movie-composer territory

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
  overrides?: ParamSubstitutions,  // connects to Decision 6 and Open Q 14
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

**Home crate:** `gtd-movie` (or `tui-vfx-movie` — TBD which layer). Plausibly a thin player binary: script parser, recipe loader, clock, grid-to-terminal emitter, minimum keyboard-for-skip. A few thousand LOC, no ratatui dependency. This is the concrete cash-in on the ecosystem-agnostic architecture (see `20_architectural_framing.md`).

**What V3 must not foreclose:**

- Recipes should not assume they own their own clock. A movie scene might inject a shared clock or a substitution-resolved clock.
- Recipes should not assume they own their full duration. A movie might start a recipe and transition it out before its natural `auto_dismiss_ms` fires.
- The two-surface substitution API (Open Q #14 — `Substitutions` load-time, `RuntimeBindings` per-frame) must support per-scene parameter overrides cleanly. Per-scene `Substitutions` resolve at scene load (each scene in the movie can have its own load-time context); per-scene `RuntimeBindings` let the movie player drive per-frame values mid-scene (e.g., animating a recipe parameter across a scripted timeline).
- `ParamValue::RuntimeBinding` (Decision 6) remains usable at the movie level — a movie player supplies values to recipe `RuntimeBindings`, orchestrating animation across scenes without requiring per-recipe customization.

**Not in scope for V3 plan.** Captured here so the future design has a home.

## 30 — Recipe migration workflow (see Chapter 50)

The canonical migration framing lives in `50_migration_workflow.md` — the three-phase Curate → Re-author → Validate model with a fixture-equivalence carve-out for designated critical recipes. Earlier drafts of this plan carried migration-workflow content inside the Deferred section; that content is now first-class as Chapter 50.

Implementation-level workflow mechanics (curation sequencing, re-authoring briefing cadence, validation-phase mechanics, critical-set fixture-track tooling) are spelled out in Chapter 50's sub-questions section. Not in scope for this plan's deferred-design territory.

## 40 — Recipe metadata fields for discovery and categorization

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

## 50 — Retrospective corrections

Not all V3 work is forward motion — some is repairing prior decisions per Principle 4 / Intention 51. Known retrospective corrections in progress or queued:

- **`easing_family.json` decomposition.** The 26-variant `template + variants` consolidation regressed individual-recipe addressability. Correction: re-expand into 26 individual `ease_<snake_case>.json` files while preserving the consolidated file under a `_DEPRECATED_` prefix. In progress at the time of this plan revision (sub-agent dispatched, produces 26 files + deprecated source, all passing `pipeline-validator`). Once confirmed, the deprecated file can be deleted.
- **Other recipes to audit for the same pattern.** Any other `template + variants` consolidation in the corpus should be audited for Principle 4 compliance during the corpus audit (Workflow B in the sibling audit-workflow doc). Consolidation is acceptable when its tooling support is also in place; it's not acceptable when the tooling gap regresses debug/preview workflows.

## 60 — StaggeredLines content effect (PRD primitive 5)

Ergonomic nice-to-have from the flag-animation PRD. Once Decision 5 (scene layers with per-layer pipelines) + Decision 6 (signal-driven parameters, including per-layer schedule delays) are in place, three independently-timed text lines can be authored as three text layers with `schedule.enter_delay_ms` per layer. `StaggeredLines` is sugar on top of that common pattern — one `ContentEffect` variant instead of three layers — and is worth shipping when the usage repeats often enough to justify the named helper (per Decision 2's earned-place logic for named compositions). Not V3-structural; defer until real demand surfaces.

## 70 — Distribution and packaging story for recipes and themes

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

## 80 — Dynamic recipe formalization

Two recipes at `recipes/dynamic/` today (`digital_rain_matrix_classic_dynamic.json`, `digital_rain_matrix_modern_dynamic.json`) use `{"binding": "<name>"}` for runtime-driven parameters. This works but is ad-hoc — the `/dynamic/` directory convention isn't canonical; the binding syntax is per-shader not uniform; and validators don't specially treat "recipes that require runtime bindings" as a category.

V3 Decision 6 formalizes `ParamValue::RuntimeBinding` uniformly across all step types. Once that lands, dynamic recipes stop being a separate category — they become recipes that use `RuntimeBinding` values in their parameters. The `/dynamic/` directory becomes either a convention for recipes expecting app-side values, or collapses entirely (any recipe can use bindings; the directory structure follows topic, not binding-presence).

**Related open questions** (not blocking V3 direction):

- Should recipes declare their required bindings explicitly in the schema so validators can check that the app supplies them? (The flag-animation PRD proposes `requires_primitives` for capability checking; the same pattern could extend to binding contracts.)
- Does the substitution API (Open Q #14) unify with binding resolution, or stay distinct as string-vs-typed-value mechanisms?

Defer until Decision 6 implementation exposes the real shape.

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/90_deferred_design.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
