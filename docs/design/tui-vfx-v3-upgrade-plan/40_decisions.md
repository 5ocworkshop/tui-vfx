<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md</FILE> - <DESC>Chapter 40 — the eight structural decisions of V3 with adopted direction. Each decision carries its rationale, alternatives considered, composition with other decisions, and any implementation-track sub-questions that remain open for the implementation phase to resolve.</DESC> -->
<!-- <VERS>VERSION: 1.0.0</VERS> -->
<!-- <WCTX>Extracted from the monolithic plan (v0.16.0) "Decisions reached" section. The eight Decisions keep their semantic numbering (1-8) — these are external contracts cited in the migration log, schema draft, and reviewer memos. Chapter-internal section numbers use 10-unit spacing for insertability. Added an ANSI flat-vs-tree diagram illustrating Decision 3 as the most structurally visible change.</WCTX> -->
<!-- <CLOG>1.0.0: initial extraction from the monolith. Decisions 1-8 verbatim from v0.16.0. Flat-vs-tree visual added to Decision 3 section.</CLOG> -->

# 40 — Decisions reached

Eight structural decisions with adopted direction. "Adopted" means the direction is committed for V3; implementation specifics may still be in flight for Decisions that carry their own track (notably Decisions 5 and 8, which name explicit sub-questions or implementation-track work). Any Decision whose title includes "implementation track" or "formalize during V3" has adopted-direction + implementation-time-specifics as an intentional pairing, not a hedge. The sub-questions inside each Decision are where implementation choices remain; the decision itself is not contingent on those sub-questions resolving in any particular direction.

## 10 — Decision 1: Unified `Scope` primitive — adopt

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

## 20 — Decision 2: Pattern as separable axis — adopt as internal model, keep named-factory JSON sugar

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

As V2 recipes are migrated one-at-a-time (see Chapter 50 — Migration Workflow), each shader step gets classified per Workflow A. When a recipe uses a composition that's:

- Trivially primitive → re-expressed as raw primitive form in the recipe JSON.
- Worth library naming → created as a Rust factory in tui-vfx-style if it doesn't exist; referenced via the named-factory JSON form.
- Worth theme naming → created as a theme-scoped fragment file; referenced via `$use`.
- App-specific → created as an app-scoped fragment in the app's resources; referenced via `$use`.

The classification is a judgment call per recipe; the tier determines where the named composition lives, not whether it exists.

**Note on signal primitives:** when the `Pattern` catalog needs spatial-awareness (e.g., `Pattern::FourEdgeRadial` with per-edge sampled colors), the signal primitives that power it extend `mixed-signals` upstream rather than living inside tui-vfx. See `20_architectural_framing.md` §20. This flips the preference in the flag-animation PRD v0.3.0.

## 30 — Decision 3: Tree authoring schema — adopt

Replace flat `pipeline.{mask, filter, sampler, styles}` with a recursive structure:

```
Pipeline  ::= Step | Sequence(Vec<Pipeline>) | Parallel(Vec<Pipeline>)
Step      ::= { kind, scope, phase, payload }
kind      ::= Mask | Filter | Sampler | StyleEffect | Shader
phase     ::= Enter | Dwell | Exit | All
```

The structural change this unlocks, visually:

```
   V2 (flat, asymmetric)                V3 (tree, uniform)
  ────────────────────────              ─────────────────────────

   pipeline:                            pipeline:
     mask:                                timing: {...}
       enter:  {...}                      step:
       exit:   {...}                        kind: parallel
     sampler:                               children:
       enter:  {...}                          - {kind: mask,    phase: enter, ...}
       dwell:  {...}                          - {kind: mask,    phase: exit,  ...}
       exit:   {...}                          - {kind: sampler, phase: enter, ...}
     filter:                                  - {kind: sampler, phase: dwell, ...}
       enter:  {...}                          - {kind: sampler, phase: exit,  ...}
       dwell:  {...}                          - {kind: filter,  phase: enter, ...}
       exit:   {...}                          - {kind: shader,  scope: bg, ...}
     style:                                 ...
       region: All                         (all atoms share the same shape)
       base_style: {...}
       spatial_shader: {...}
     styles:              ← second
       - region: Border     array
         base_style: ...    emerges
         enter_effect: ...  for multi-
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

## 40 — Decision 4: Rename `Ra*` prefix to `Vfx*` — adopt, bundled with V3

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

## 50 — Decision 5: Scene layers carry their own pipelines — direction adopted, dedicated V3 implementation track

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

## 60 — Decision 6: Signal-driven parameters — formalize and extend

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

## 70 — Decision 7: Step output hints as a first-class primitive — adopt

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

## 80 — Decision 8: Canonical upstream semantic seam — adopt, formalize during V3

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

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-plan/40_decisions.md</FILE> -->
<!-- <VERS>END OF VERSION: 1.0.0</VERS> -->
