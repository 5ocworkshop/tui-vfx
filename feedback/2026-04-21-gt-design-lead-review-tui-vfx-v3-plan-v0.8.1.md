<!-- <FILE>feedback/2026-04-21-gt-design-lead-review-tui-vfx-v3-plan-v0.8.1.md</FILE> - <DESC>Formal review memo from the GT-Design lead maintainer on the tui-vfx v3 upgrade plan draft</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Capture a careful, evidence-backed response to the tui-vfx v3 upgrade plan and the companion audit-workflow draft from the perspective of GT-Design as the largest current consumer, including recommendations on open questions, sequencing, and ratatui-centric integration needs.</WCTX> -->
<!-- <CLOG>0.2.0: add review sections for tui-vfx-v3-upgrade-audit-workflow.md version 0.2.0, including workflow-level recommendations on scope, sequencing, corpus sizing, and exit criteria.
0.1.0: initial formal memo covering strengths, critical concerns, recommended answers to open questions, GT-Design integration requirements, and reviewed-draft metadata for tui-vfx-v3-upgrade-plan.md version 0.8.1.</CLOG> -->

# Formal Review Memo — TUI-VFX V3 Upgrade Plan

**From:** Lead maintainer, GT-Design  
**Date:** 2026-04-21  
**Draft reviewed:** `docs/design/tui-vfx-v3-upgrade-plan.md` **version 0.8.1**

## Executive summary

My overall view is **strongly positive on direction, but not yet ready for implementation without a short clarification pass**.

The big architectural moves are largely the right ones:

- a uniform tree schema,
- a canonical upstream semantic seam,
- a more honest naming pass,
- stronger scene/layer support,
- first-class runtime bindings,
- and explicit governance around consolidation and corpus quality.

From GT-Design's perspective, this is the first plan that feels like it is trying to solve the *actual* integration pain rather than just rearranging the schema.

That said, I think the draft still has a few **load-bearing contradictions / unresolved boundaries** that should be fixed before implementation starts. The highest-priority ones are:

1. **`ParamValue<T>` is structurally inconsistent in the draft.** Decision 6 defines a 3-variant model; Decision 7 later introduces `ParamValue::StepOutput`, which makes it 4 variants or a different abstraction entirely.
2. **Migration strategy is internally split-brain.** Open Q #2 frames migration as a one-time scripted/hybrid corpus rewrite; the later “Recipe migration workflow” reframes it as manual/Claude-assisted per-recipe translation where bulk translation is explicitly *not* the goal.
3. **The draft risks collapsing multiple different “role” concepts into one overloaded axis.** In current live code, `RoleTag` already means a concrete per-cell render role (`Background`, `Text`, `Border`, `Shadow`, etc.). The draft also uses “role” for theme targeting and for higher-level routing/lifecycle identity. Those should not be one thing.
4. **Tokenization and runtime binding should be coordinated, not merged into one mutable concept.** In GT-Design today, token+asset substitution and runtime params have different lifetimes, ownership, and failure modes.
5. **Scene-layer pipelines are described as adopted direction, but the live scene schema does not have a per-layer pipeline field yet.** This is not a tiny additive follow-up; it is a real schema/runtime expansion that should be planned explicitly.
6. **Shadow / offscreen composition / trace compatibility should be treated as a release gate, not just an open risk.** GT-Design depends heavily on those surfaces.

If those six things are clarified, I would be comfortable calling the plan “implementation-worthy.”

---

## What I strongly agree with

### 1. The canonical upstream semantic seam is the single most important decision in the document

This is the most important practical improvement for GT-Design.

Live code evidence backs the problem statement:

- Upstream still exposes the effective seam through `PreviewItem` / `PreviewManager` in `src/preview/*`.
- GT-Design currently mirrors config→playback semantics in `crates/gtd-ratatui/src/recipes/item.rs`.
- That mirror is exactly where semantic drift happens.
- In the current GTD mirror, `item_from_recipe_config()` does **not** thread `config.shadow`, while upstream `preview_from_recipe_config()` does.

That is a textbook example of why meaning must move back upstream.

So yes: **ship one canonical builder / semantic handoff upstream, and make downstream policy layers wrap it rather than reinterpret it.**

### 2. The tree schema is the right long-term authoring shape

I agree with the move away from the current flat mask/filter/sampler/style split.

Even if AI-assisted authoring is the dominant authoring path, the underlying contract still matters for:

- validators,
- canonicalization,
- probe/trace tooling,
- human review,
- community recipes,
- and future downstream consumers.

The current schema has enough asymmetry that every layer after parsing pays the price.

### 3. The plan correctly elevates authoring-affordance preservation

This is one of the strongest parts of the document.

The plan’s Principle 4 aligns with GT-Design’s own Intention 51: if V3 introduces `$use`, primitive libraries, bundling, or other consolidation tools, then **addressability and tool visibility must ship with them**. That is the right constraint.

### 4. The preview/playback naming problem is real

I agree that `PreviewItem`, `PreviewManager`, and `src/preview/` are misnamed for the role they actually play.

From a downstream maintainer’s perspective, the current names understate their architectural importance and make the seam feel optional or demo-only.

### 5. The plan correctly recognizes that the library is no longer “just toast animations”

This reframing is overdue and necessary.

GT-Design is already using recipe playback for:

- splash-like scenes,
- transition-like surfaces,
- canvas-backed composition,
- and theme-sensitive motion/VFX behaviors that are not notification-shaped.

The plan is right to pressure-test vocabulary and abstractions against scenes, widgets, transitions, overlays, and future movies.

---

## High-priority concerns to fix before implementation

## A. Resolve the `ParamValue<T>` contradiction before coding

This is the sharpest design inconsistency in the document.

- Decision 6 defines `ParamValue<T>` as:
  1. Constant
  2. Runtime-bound
  3. Signal-graph
- Decision 7 later describes `ParamValue::StepOutput { hint, producer_ref? }`

That is either:

- a fourth variant,
- a different type,
- or an accidental conflation of two adjacent concepts.

### My recommendation

Do **not** start implementation until this is made explicit.

My lean is:

- Keep **runtime/app values** and **signal graphs** under one family.
- Model **step-output references** as a **separate input-reference abstraction** or a clearly documented fourth variant.

A clean shape would be something like:

- `ParamValue<T>` for literal / runtime / signal values
- `InputRef<T>` or `HintRef<T>` for upstream pipeline-produced values

If you decide to keep them together, then make it an explicit 4-variant model and update all relevant sections consistently.

## B. Choose one migration philosophy

Right now the draft says two different things:

- Open Q #2 leans toward a one-time scripted/hybrid migration pass over the corpus.
- The later “Recipe migration workflow” says bulk translation is *not* the goal and frames the process as manual, recipe-by-recipe, Claude-assisted translation.

Those are not the same migration strategy.

### My recommendation

Adopt a **hybrid migration model**:

1. **Mechanical translator/canonicalizer** for all deterministic schema reshaping.
2. **Human review layer** for the value judgments:
   - Morris filtering,
   - earned-name vs primitive decisions,
   - metadata classification,
   - and any scene/pattern-specific interpretation.
3. **Probe-equivalence or fixture-equivalence checks** for critical recipes.

What I do **not** recommend is a purely manual Claude-led rewrite of the whole corpus. That is too hard to audit and too easy to drift semantically.

## C. Split the role domains

This needs to be much sharper.

Current live code already has a real `RoleTag` type in `tui-vfx-types`, and it means **per-cell render role**:

- `Background`
- `Text`
- `Title`
- `Caption`
- `Border`
- `Image`
- `Icon`
- `Indicator`
- `Highlight`
- `Shadow`
- `Decoration`
- `Procedural`

That is a concrete low-level rendering vocabulary.

The draft also uses “role” for:

- semantic/theme targeting in scopes,
- higher-level routing hints,
- and recipe-level identity like `splash` / `toast` / `transition` / `ambient`.

Those are **not the same semantic domain**.

### My recommendation

Use separate names and separate types for at least these three axes:

1. **Per-cell / source-role tags** — existing `RoleTag` domain
2. **Theme / semantic targeting roles** — if needed, give this a theme-specific name
3. **Routing / behavior / hosting hints** — e.g. `RoutingRole`, `PlaybackRole`, or `SurfaceIntent`

I strongly recommend **not** extending the current `RoleTag` surface all the way up to recipe-level “toast/splash/transition” semantics.

## D. Keep tokenization and runtime bindings coordinated but distinct

I support moving tokenization to a `tui-vfx-recipes` boundary API.

But from GT-Design’s perspective, **load-time substitution** and **per-frame runtime binding** are not the same problem:

- text tokens and asset bytes are mostly load/intake concerns,
- runtime shader params are active playback concerns.

Today that split is reflected in GTD:

- `crates/gtd-ratatui/src/splash/cls_substitutions.rs` owns token+asset substitution,
- recipe playback runtime params travel separately.

### My recommendation

Keep them under one conceptual umbrella if you want, but expose **separate API surfaces**:

- `RecipeLoadContext` / `Substitutions` for text, assets, whole-value coercion
- `RuntimeBindings` / `RuntimeParams` for per-frame typed values

An umbrella wrapper can contain both, but the lifetimes and failure modes should stay explicit.

## E. Scene-layer pipelines need to be planned as real work, not implied work

The draft adopts scene-layer pipelines as if they are a natural extension of the existing scene schema.

But in the live schema, `RaSceneLayer` currently carries:

- `id`
- `z`
- `placement`
- `source`
- `role_tag`
- `overflow`
- `visibility`

There is **no per-layer pipeline field yet**.

So from an implementation perspective, this is not just “tie in the existing scene work.” It is a real additive schema/runtime feature that needs:

- schema changes,
- parser/deserializer updates,
- validator work,
- compositor composition order decisions,
- trace additions,
- and migration fixtures.

### My recommendation

Keep it in V3 directionally, but call it out as a **separate implementation track inside V3**, not as if it were already structurally half-landed.

## F. Treat shadow/offscreen/trace compatibility as a release gate

The current draft lists this as an open risk area.

From GT-Design’s perspective, it is more than that.

We depend on:

- shadow fidelity,
- offscreen composition behavior,
- role-aware lowering,
- trace/probe observability,
- and factory-canonical final rendering.

If V3 regresses those, the schema improvements will not pay back for us.

### My recommendation

Promote this from “open concern” to an explicit **V3 release gate**:

- canonical shadow fixtures,
- offscreen/slide fixtures,
- probe snapshots,
- trace expectations,
- and GTD integration fixtures for representative surfaces.

---

## Recommended answers to the open questions

## 1. Does the `kind` discriminator survive?

**Recommendation:** yes, keep it.

The distinction is still useful for comprehension, validation, documentation, and tooling. You can always collapse later if the boundaries truly prove artificial.

## 2. Migration strategy and schema versioning

**Recommendation:** hybrid migration, not purely scripted and not purely hand-authored.

- mechanical translation for shape changes,
- human review for classification and curation,
- fixture/probe equivalence for critical recipes.

## 3. Phase scoping shape: per-step field vs container

**Recommendation:** per-step field, with container propagation.

This matches the scope model, keeps the normalized shape regular, and still allows readable grouping.

## 4. Composition combine semantics

**Recommendation:** per-kind defaults plus explicit container override.

For tooling and tests, I would strongly recommend a **normalized internal form** where the effective combine is explicit after parsing/canonicalization.

## 5. Named-factory and compositional JSON coexistence

**Recommendation:** yes, support both; validate equivalence; provide canonicalization tooling.

My lean:

- property-test equivalence for curated pairs,
- canonicalize for inspection/debugging,
- teach named factories for curated presets,
- teach primitive/compositional form for advanced/custom authoring.

Allow mixing in one recipe, but don’t make it the default teaching style.

## 6. Scope primitive open/closed tension

**Recommendation:** closed enum with registered escape hatch.

That is the right balance for caching, validation, and authoring predictability.

## 7. Relationship to `RecipeSceneCanvas`

**Recommendation:** do not make GTD substrate sequencing the blocker for upstream V3 core work.

What upstream should stabilize first:

- canonical semantic seam,
- naming cleanup,
- token/binding contracts,
- normalized execution model.

Then GTD should adapt `RecipeSceneCanvas` to that seam.

## 8. Unblock order for Relative Light explorations

**Recommendation:** do not ship productized V2 versions if V3 is clearly the right substrate.

If exploration must continue, do it as:

- isolated R&D fixtures,
- debug recipes,
- or lab-only prototypes,

not as a user-facing V2 contract you will immediately have to migrate.

## 9. Validator redesign

**Recommendation:** this is core work, not support work.

The validator needs to validate:

- scope coherence,
- tree/container invariants,
- hint ambiguity,
- fragment addressability,
- token/binding contracts,
- and migration equivalence for critical fixtures.

I would also recommend validating a **canonical normalized IR**, not only raw authoring syntax.

## 10. Viewer still worth building independently

**Recommendation:** yes, but build it on the normalized execution graph / canonical IR, not directly on author sugar.

That will make it much more durable across future schema evolution.

## 11. Docs, SKILLS, generators

**Recommendation:** must ship in the same cutover.

Especially important:

- generated API docs,
- validator/tracing docs,
- AI/LLM guidance,
- migration notes,
- and canonical examples.

## 12. Shadow rendering, offscreen composition, probe/trace compatibility

**Recommendation:** release gate.

Not optional polish.

## 13. Partial-phase spans

**Recommendation:** yes, support `PhaseSet`, and keep it available at the step level.

Container propagation can exist too, but don’t make container-only the model.

## 14. Tokenization ownership and contract discovery

**Recommendation:** move the API upstream, but separate load-time substitutions from runtime bindings.

Also yes to:

- explicit declared contracts,
- strict-mode default,
- introspection API,
- and byte-based asset resolution.

## 15. Vocabulary refresh scope

**Recommendation:** comprehensive-but-selective.

My specific lean:

- **Rename** `auto_dismiss_ms`
- **Probably rename / rework** `continuous`
- **Rename** preview seam nouns/modules
- **Keep `anchor` unless semantics change** — in a ratatui/grid context, anchor is already a good geometry term
- **Keep `enter/dwell/exit` unless translation study proves they are actively misleading**

From a ratatui-centric consumer perspective, `anchor` and `enter/dwell/exit` already generalize better than the draft gives them credit for.

## 16. Cross-step hint resolution rules

**Recommendation:**

- visibility defaults to **same pipeline / same layer only**
- cross-layer reads require explicit export/import semantics if they exist at all
- hint lifetime is per-frame / ephemeral
- **multiple producers for the same visible hint should be a validator error unless explicitly qualified**

Do not use “first wins” or “last wins” silently. That is too brittle.

## 17. Primitive library / `$use` fragment composition

**Recommendation:** yes, but keep v1 minimal and non-blocking.

My lean for first delivery:

- one fragment mechanism,
- flattened at load time,
- parameterization via the same substitution system,
- no fragment inheritance in v1 unless a real case demands it,
- strict addressability + introspection from day one.

## 18. Semantic role tags as uniform optional metadata

**Recommendation:** yes to routing metadata; **no to collapsing it into the existing `RoleTag` domain**.

I would name the higher-level field something like:

- `routing_role`
- `surface_intent`
- `playback_role`
- or `semantic_tag`

so it doesn’t collide with:

- per-cell `RoleTag`
- theme/color roles
- or scope-targeting semantics.

## 19. “Preview” naming for the canonical engine seam

**Recommendation:** rename now.

My lean:

- module path → `playback`
- manager → `PlaybackManager`
- seam type → maybe `PlaybackItem`, but I would also consider whether **`PlaybackPlan` / `PlaybackUnit`** is a more future-proof noun once scenes and multi-layer content are first-class.

I would not keep `Preview*` on the seam.

## 20. Surface identity vs neutral substrate

**Recommendation:** choose **A**.

Keep `RecipeSceneCanvas` as the neutral substrate family, and let GT-Design wrap it with family-specific surface identities.

That aligns with GTD’s current steering best:

- `RecipeSceneCanvas` is the substrate family,
- surface identity is higher-level policy,
- and internal variants (`RawRecipeSceneCanvas`, `ResolvedRecipeSceneCanvas`) can exist without changing that public conceptual split.

---

## Additional recommendation on the newly introduced metadata question

The document says it has 20 open questions, but later introduces a new “Open Question #21” in deferred territory around recipe metadata.

I support the metadata direction, but I would keep it **non-blocking for V3 core**.

My lean:

- `use_cases` should likely be required,
- most other metadata can be optional initially,
- discovery metadata should stay clearly separate from runtime routing metadata.

---

## GT-Design-specific needs I would ask V3 to preserve

## 1. A stable semantic handoff that downstream can trust

The canonical upstream seam should be designed so a downstream like GT-Design can:

- take the semantic output,
- apply theme/policy/surface identity,
- route through factory-canonical render truth,
- and avoid re-implementing recipe meaning.

That is the highest-value outcome for us.

## 2. Byte-source-based loading everywhere relevant

The deferred packaging note is correct and important.

For GT-Design-based apps, embedded + layered source stories matter a lot. V3 should preserve:

- byte-source loaders,
- layered source resolution,
- and non-filesystem assumptions for fragments and assets.

## 3. Canonicalization / introspection / traceability

From the perspective of a large consumer, I care almost as much about observability as authoring syntax.

I would strongly encourage shipping or planning for:

- recipe canonicalization,
- hint/binding introspection,
- contract discovery APIs,
- normalized IR inspection,
- and stable trace fixtures.

## 4. A strict boundary between upstream meaning and GTD policy

This plan is strongest when it follows that rule. It weakens where it starts to blur:

- render roles vs routing roles,
- substrate vs surface identity,
- tokenization vs runtime binding,
- engine seam vs downstream hosting policy.

Wherever possible, keep those boundaries explicit.

---

## Draft-quality / document-hygiene notes

These are minor compared to the architectural points above, but worth tightening:

- The document says “Three principles shape V3 design” even though it now contains five principles.
- The overview says there are 20 open questions, but the later metadata section introduces a new “Open Question #21”.
- Some sections still read like directional design notes while being labeled “adopted.” That’s fine at this stage, but the adopted/open/deferred boundaries should stay crisp before implementation.

---

---

## Additional review thoughts on the companion audit-workflow draft

I also reviewed the companion workflow document:

- `docs/design/history/tui-vfx-v3-upgrade-audit-workflow.md` version `0.2.0`

My overall view is positive: the workflow doc is useful, concrete, and much better than leaving the audit phase implicit. It does a good job turning “we should probably audit this later” into actual named workstreams.

That said, I would tighten four things before treating it as the operative execution plan.

## 1. The workflow doc is good, but its scope estimates are already stale

Two estimates in the draft appear behind the live repo state.

### Shader debug corpus

The draft says:

- 52 debug recipe files covering ~27 named shader types

The live repo currently contains **54** JSON files under `recipes/debug_recipes/shaders/`, and several are `_DEPRECATED_` prefixed.

### Main recipe corpus

The draft estimates:

- **200–300 recipes** in the main corpus audit scope

A quick filesystem inventory of `tui-vfx-recipes/recipes/**/*.json`, excluding `debug_recipes`, `vfx-probe-validation`, and obvious test/fixture roots, currently comes out much higher: **509 JSON files**.

That does not necessarily mean 509 *real audit targets* — some may be deprecated, generated, archival, or otherwise not in the intended curation set — but it does mean the workflow doc should stop talking as though the likely live audit scope is 200–300 until an authoritative inventory step is completed.

### Recommendation

Before Workflow B begins, add a **mandatory inventory step** that produces a checked-in manifest:

- total candidate recipe files
- excluded files and exclusion reasons
- deprecated files
- debug/probe/test/fixture files
- actual curation target count

Without that, time estimates and staffing assumptions will be unreliable from day one.

## 2. Workflow A should audit shader *families*, not just files

The draft is right to start with the shader catalog, but I would make one structural refinement:

**audit by canonical shader family, with file variants nested under that family**, not by raw file count.

Why:

- the debug corpus already contains parameter-surface variants,
- deprecated files exist,
- and the thing that matters architecturally is the semantic shader family, not each JSON file as a peer.

### Recommendation

Have Workflow A begin by generating a normalized table something like:

- shader family name
- active debug recipes
- deprecated debug recipes
- parameter axes represented
- runtime-binding variants present?
- likely primitive-model fit

That makes the audit more stable and keeps the session from over-weighting file proliferation.

## 3. Workflow B should be split into “shipping corpus” vs “reference/archive corpus” earlier

The William Morris framing is good, and I support it.

But the workflow would benefit from a sharper distinction between:

1. **shipping/reference corpus** — what V3 actively loads, validates, teaches, and showcases
2. **historical/archive corpus** — what is retained for reference but not treated as a live V3 surface

Right now the workflow talks about port / archive / delete / consolidate, but it still reads a bit like one giant recipe triage lane.

From a maintainer perspective, I would rather see the audit explicitly produce **two inventories**:

- `v3-active-corpus`
- `v2-archive-corpus`

with clear rules for how archived content is stored, indexed, and intentionally *not* allowed to drift back into active teaching surfaces.

### Recommendation

For each audited recipe, capture at least:

- disposition (`active`, `archive`, `delete`, `consolidate-into`)
- rationale category (`useful`, `beautiful`, `superseded`, `duplicate`, `historical-only`, etc.)
- cross-references
- whether it remains validator-covered

That turns the audit into a maintainable repository asset, not just a one-time conversation record.

## 4. Workflow C should have a small pilot earlier than the document currently suggests

The sequencing recommendation in the draft is reasonable:

1. Workflow A
2. Workflow B
3. Workflow C

I agree with that **for the full workflows**.

However, I would still recommend doing a **small pilot translation exercise very early** — before Workflow A is fully complete and definitely before Workflow B has spent days on corpus classification.

Reason: some structural problems in a tree schema only become obvious when you actually translate real recipes. If those problems surface late, they can invalidate assumptions made in both A and B.

### Recommendation

Do a **mini-Workflow C pilot** up front:

- pick 2–3 representative recipes,
- translate them into the proposed V3 tree shape,
- record the first real schema pain points,
- then proceed to full Workflow A / B with that feedback in hand.

That does not replace Workflow C. It de-risks it.

## 5. Suggested exit criteria for each workflow

The workflow doc is good on process but lighter than it should be on completion gates.

I would add explicit exit criteria.

### Workflow A exits when:

- every named shader family has a classification,
- every “primitive itself” case has an associated primitive-gap note,
- every preserved earned name has rationale,
- and deprecated/duplicate debug files are clearly accounted for.

### Workflow B exits when:

- the active V3 corpus list exists,
- the archive list exists,
- every kept recipe has a rationale,
- and the repo has a clear rule for whether archived recipes remain loadable, searchable, or validator-covered.

### Workflow C exits when:

- 6–8 representative recipes have been translated,
- surfaced schema questions are enumerated,
- and the team can answer whether the tree shape is ready, ready-with-adjustments, or not ready.

## 6. My practical recommendation on workflow priority

If the team wants the best outcome rather than the fastest apparent motion, my recommendation is:

1. **Inventory first** — establish real counts and actual audit scope.
2. **Mini-Workflow C pilot** — catch structural issues early.
3. **Workflow A** — settle shader-family classification.
4. **Workflow B** — curate the corpus with better assumptions.
5. **Full Workflow C** — validate the mature schema against representative kept recipes.

That is slightly different from the draft’s pure A → B → C recommendation, but I think it is lower risk.

## Final view on the audit-workflow draft

The audit-workflow document is worth keeping and worth using.

My main requested changes are not about philosophy; they are about **operational realism**:

- refresh stale scope estimates,
- introduce an authoritative inventory step,
- treat shader families rather than raw files as the core unit in Workflow A,
- split active vs archive corpus outputs more explicitly in Workflow B,
- and run a small structural pilot earlier so Workflow C can de-risk the larger effort.

With those refinements, the workflow doc becomes not just a good appendix, but a genuinely credible execution companion to the main V3 plan.

## Final recommendation

If I reduce this to one sentence:

**Proceed with this direction, but do one more draft pass that resolves the parameter-model contradiction, role-domain split, migration strategy, and release-gate criteria before implementation begins.**

From GT-Design’s point of view, the plan is now pointed at the right problems.
What will determine whether V3 is a smooth upgrade or another seam-repair cycle is whether these remaining boundaries are clarified *before* code starts.

<!-- <FILE>feedback/2026-04-21-gt-design-lead-review-tui-vfx-v3-plan-v0.8.1.md</FILE> - <DESC>Formal review memo from the GT-Design lead maintainer on the tui-vfx v3 upgrade plan draft</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
