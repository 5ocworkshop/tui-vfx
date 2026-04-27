<!-- <FILE>docs/design/tui-vfx-v3-upgrade-audit-workflow.md</FILE> - <DESC>Deferred-execution workflow for the three audits that feed the V3 tui-vfx upgrade: (A) shader catalog decomposition — per-shader evaluation of whether each named shader is a trivial primitive composition or earns its name; (B) recipe corpus curation under the William Morris "useful or beautiful" principle — audit every recipe and classify port/retire; (C) structural translation sample — re-express 6–8 representative recipes in the proposed tree shape to stress-test the structure. All three are deferred to a future session and are not blocking current work.</DESC> -->
<!-- <VERS>VERSION: 0.3.1</VERS> -->
<!-- <WCTX>Integrate six operational-realism recommendations from the 2026-04-21 GT-Design lead review memo: (1) stale scope estimates corrected — debug shaders 52 → 54 live files with some _DEPRECATED_, main corpus 200-300 → >500 pending authoritative inventory; (2) Workflow A restructured family-first (shader family as unit of audit with variants nested); (3) Workflow B split into active-vs-archive corpus outputs earlier; (4) mini-Workflow-C pilot (2-3 recipes) added as early de-risk step; (5) explicit exit criteria added per workflow; (6) sequencing revised to Inventory → mini-C pilot → A → B → full C. Adds a new Workflow 0 (authoritative inventory) as blocking prerequisite for Workflows A and B. Cross-links to main plan's Concern B (curate-first migration) where relevant.</WCTX> -->
<!-- <CLOG>0.3.1: point plan references at the chapter index and keep the Relative Light RFC as a plain-text historical reference because the target lives outside this repo.</CLOG> -->

# V3 Pipeline Migration — Audit & Curation Workflows

> **Status: deferred.** Three audit workflows captured here for execution in a future session. None are blocking current work. Together they produce the empirical inputs required to resolve several open questions in the main plan ([`./tui-vfx-v3-upgrade-plan/00_INDEX.md`](./tui-vfx-v3-upgrade-plan/00_INDEX.md)).

## Context

The V3 tree-schema migration depends on three empirical inputs that this session's conversation could only reason about abstractly:

1. **What's the actual shape of the named-shader catalog?** We discovered 27+ distinct named shaders during the session, larger than initial estimates. The decision "primitive-only vs preserve named compositions" can't be made cleanly without going through each one.
2. **What does the recipe corpus actually contain, and which parts still earn their place?** Recipes have accumulated across several development phases (basic → theatric → mature → professional → impressively theatric), some theme-grounded and some abstract. Mechanical porting would preserve archaeology, not design. A Morris-principle curation pass is the right shape for the audit.
3. **Does the proposed tree schema handle structural diversity?** The shape sketches in the main plan cover three cases. Real recipes from across themes will surface additional structural patterns and likely generate 2–4 more open questions.

Each of these is a distinct audit with its own output. This document defines all three, plus a new Workflow 0 (authoritative inventory) that is a blocking prerequisite for Workflows A and B.

## 2026-04-21 review integration

This doc's v0.3.0 revision integrates six operational-realism recommendations from the GT-Design lead review memo (`/usr/projects/tui-vfx/feedback/2026-04-21-gt-design-lead-review-tui-vfx-v3-plan-v0.8.1.md`):

1. **Scope estimates were stale.** The debug-shader count (52 files) and main-corpus estimate (200–300 recipes) both lag live-repo evidence — current filesystem shows 54 debug-shader files with some `_DEPRECATED_` prefixed, and >500 main-corpus files (excluding `debug_recipes`, `vfx-probe-validation`, and obvious test/fixture roots). Estimates corrected throughout; authoritative count deferred to Workflow 0.
2. **Workflow A should audit shader *families*, not raw files.** The debug corpus already contains parameter-surface variants and deprecated files; what matters architecturally is the semantic shader family, not each JSON file as a peer. Workflow A is restructured family-first — canonical family as the unit of audit, file variants nested underneath.
3. **Workflow B should split active vs archive corpus earlier.** The prior framing read as one giant triage lane. Workflow B now produces two explicit inventories (`v3-active-corpus` and `v2-archive-corpus`) with per-recipe disposition, rationale category, cross-references, and a validator-coverage flag. This turns the audit into a maintainable repository asset, not a one-time conversation record.
4. **A small Workflow C pilot should run earlier.** Structural schema questions surface when real recipes get translated; surfacing them late can invalidate A/B assumptions. A 2–3 recipe mini-pilot runs *before* full A and B to de-risk them.
5. **Exit criteria per workflow.** Each workflow now has explicit completion gates, not just a time estimate.
6. **Revised priority:** Inventory → mini-C pilot → A → B → full C (different from the prior A → B → C default). The mini-pilot + inventory together catch assumption-level issues before A and B commit hours of work.

Each recommendation is integrated into the appropriate workflow section below.

## Relationship to the main plan's Concern B

The main plan's [Concern B resolution](./tui-vfx-v3-upgrade-plan/00_INDEX.md) frames the V3 migration as three phases (Curate → Re-author → Validate with a critical-set fixture carve-out). This doc's workflows map onto Concern B's phases:

- **Workflow 0 (authoritative inventory)** is the blocking prerequisite for Concern B's curation phase.
- **Workflow B (corpus curation)** *is* Concern B's curation phase — Morris filter over the inventoried corpus producing the retained set with dispositions.
- **Workflow A (shader catalog decomposition)** informs Concern B's curation and re-authoring phases — shader-family classifications drive the named-factory vs primitive-form decisions authors make per recipe.
- **Workflow C (structural translation)** informs Concern B's re-authoring phase — the mini-pilot surfaces briefing-infrastructure gaps before the full re-authoring effort; the full Workflow C validates the mature schema against representative kept recipes.
- **Critical-set fixture capture** (Concern B's carve-out) is a separate V3-implementation track, not one of these workflows — it captures V2 fixtures for the designated critical set before corpus reshape begins.

## Workflow 0 — Authoritative inventory (blocking prerequisite)

**Goal:** produce a checked-in manifest of every recipe file in the repo with classification, before Workflow A or B makes any scope assumptions. The inventory is the evidence base for estimating effort, scoping audits, and tracking progress.

**Why it's a blocking prerequisite:** the prior draft's estimates (52 debug shaders, 200–300 main recipes) are materially off from live-repo reality. Without an authoritative manifest, Workflow A and B time estimates and staffing assumptions are unreliable from day one, and the archive/active split in Workflow B has nothing concrete to partition.

### Classification categories

Each recipe file in `/usr/projects/tui-vfx-recipes/recipes/**/*.json` receives one of:

- **`candidate`** — main-corpus candidate subject to Workflow B curation.
- **`debug`** — debug / diagnostic recipe under `debug_recipes/`; subject to Workflow A if it's a shader debug recipe.
- **`probe`** — member of the `vfx-probe-validation` corpus; critical-set fixture candidate per main plan's Concern B.
- **`test`** — test or fixture file not intended as a user-facing recipe.
- **`deprecated`** — carries a `_DEPRECATED_` prefix or explicit deprecation marker; excluded from both audits, candidate for delete.
- **`generated`** — produced by a generator/tool rather than hand-authored; excluded from curation unless generator output itself needs review.

### Process

1. Walk `/usr/projects/tui-vfx-recipes/recipes/**/*.json` with consistent glob semantics.
2. For each file, assign classification based on path, filename prefix, and explicit markers inside the JSON.
3. Produce manifest output (see below) and check it in.
4. Flag any file where classification is ambiguous for follow-up before Workflow A or B begins.

### Output format

A checked-in manifest at `./tui-vfx-v3-upgrade-appendix-corpus-inventory.md`, plus optionally a machine-readable JSON at `./tui-vfx-v3-upgrade-appendix-corpus-inventory.json` for tooling.

Shape:

| File path | Classification | Reason | Notes |
|---|---|---|---|
| `recipes/ambient_banner.json` | candidate | — | toplevel main-corpus |
| `recipes/debug_recipes/shaders/shader_diffusion.json` | debug | shader debug recipe | Workflow A subject |
| `recipes/debug_recipes/shaders/_DEPRECATED_shader_glow.json` | deprecated | `_DEPRECATED_` prefix | candidate for delete |
| `recipes/vfx-probe-validation/probe_cursor.json` | probe | probe-validation corpus | Concern B critical-set candidate |
| ... | ... | ... | ... |

Plus a summary block with counts per classification category, which Workflow A and B consume as their scope inputs.

### Exit criteria

Workflow 0 exits when:

- Every `.json` file under `/usr/projects/tui-vfx-recipes/recipes/` has a classification entry
- The summary counts are computed and recorded
- Ambiguous-classification files are either resolved or explicitly flagged
- The manifest is checked in

### Time estimate

1–2 hours for a 500-file tree, mostly automated via glob + rule-based classification. Ambiguous cases are the main time cost; most classifications follow from path and filename mechanically.

## The William Morris principle (the curation test)

> "Have nothing in your houses that you do not know to be useful, or believe to be beautiful."
> — William Morris, 1880

Applied to the V3 recipe corpus:

A recipe earns its place in V3 by being **useful OR beautiful**.

- **Useful:** demonstrates a specific capability not shown elsewhere; covers a real use case; canonical example of a pattern; teaches a concept that recipe authors need to see; provides proven tuning for a design intent; serves as a test fixture for validator / probe / trace infrastructure.
- **Beautiful:** aesthetically excellent; refined design moment worth preserving; carries thematic meaning tied to a specific theme's identity; pushes creative boundaries worth keeping as reference; represents a peak of a particular era of the library's evolution.

A recipe failing **both** tests is a candidate for retirement — not ported to V3. Possibilities for retired recipes:

- **Archive in place:** keep in a `v2_archive/` subtree of the recipes repo, not loaded, not validated, but preserved for historical reference and potential future revival.
- **Delete outright:** if the recipe is an early iteration clearly superseded, has never been referenced by any theme or example, and has no historical or educational value.
- **Consolidate into a sibling:** if several recipes are variants of the same idea, keep the strongest and retire the rest.

The default retirement mode should be **archive**, not delete. Deletion requires an explicit reason; archival requires only the Morris test.

---

## Workflow A — Shader catalog decomposition

**Goal:** decide, per named shader, whether it is (a) a trivial composition of the primitive model that should drop its name in V3, (b) a composition whose parameter tuning encodes design judgment worth preserving as a named preset, or (c) itself a primitive that the `ColoredOverlay + Pattern` model cannot cleanly express.

**Why it's workflow A:** this resolves open questions #2 and #5 in the main plan (Pattern-as-axis realism, named-composition earning criteria) and is the highest-leverage input to V3's primitive catalog design.

**Unit of audit: canonical shader family (not file).** The debug corpus contains parameter-surface variants and deprecated files; auditing file-by-file over-weights file proliferation and under-weights semantic coherence. Workflow A audits by canonical family — one row per family, variants nested underneath — per the 2026-04-21 reviewer recommendation. Workflow 0's inventory classifies debug files; Workflow A consumes that classification to build the family view.

### Infrastructure that exists

- Per-shader debug recipes at `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/` — current filesystem count is **54 recipe files** (previously estimated 52); several carry `_DEPRECATED_` prefixes. Authoritative count comes from Workflow 0's inventory.
- Interactive preview via `cargo run --example demo` in `/usr/projects/tui-vfx-recipes/`, loads any recipe from the tree.
- Variant filenames encode parameter surface: `shader_<name>_<variant>.json` (e.g., `shader_concealed_light_both.json`, `shader_concealed_light_drift.json`, `shader_concealed_light_foreground.json` — the base shader plus three parameter axes).

### Tour order (warm-up → high-judgment)

**Tier 1 — likely trivial compositions (calibration warm-up):**
`diffusion`, `concealed_light`, `glow`, `linear_gradient`, `edge_sheen`, `ambient_occlusion`

*Expected classification:* mostly primitive (`ColoredOverlay + Pattern::<variant>`). These set expectations for what "trivial" looks like.

**Tier 2 — parametric, need inspection:**
`focus_field` (has 7 variants including runtime bindings), `highlighter` (has 9 parameter-surface variants), `border_sweep`, `pulse_wave`, `glisten_band`

*Expected classification:* mixed. Some variants may be primitive compositions; others (especially binding-enabled variants) may be themselves primitives or may reveal missing concepts in the primitive model (e.g., runtime-bound parameters that the Pattern enum can't cleanly express).

**Tier 3 — likely themselves primitives:**
`radar`, `wayfinding_node`, `orbit`, `barber_pole`, `trace_path`, `sub_cell_shake`, `stochastic_sparkle`, `bevel`, `chromatic_edge`, `reflect`, `affordance_wake`, `cursor`, `glitch_lines`, `neon_flicker`, `focused_row_gradient`, `reveal_wipe`

*Expected classification:* primarily "is itself a primitive." Each does distinctive work that the `ColoredOverlay + Pattern` model likely cannot cover without extension. These are the shaders most likely to reveal gaps in the proposed primitive design.

### Per-shader process

1. Read the shader's base debug recipe and all its variants (I pre-read; user does not need to).
2. Propose a classification with reasoning:
   - **Trivial composition:** "This is `ColoredOverlay + Pattern::<X>` with default parameters. Recommend: drop the name; re-author recipes using the primitive form."
   - **Earned name:** "This is `ColoredOverlay + Pattern::<X>` but with specific parameter tuning (`softness: 0.8`, `edge_firmness: 0.25`, etc.) that encodes design judgment. Recommend: preserve as a named preset with documented rationale."
   - **Primitive itself:** "This shader does X, which the proposed primitive model cannot express. Recommend: add to the primitive catalog; may require a new `Pattern` variant or a new base shader."
3. User previews the shader(s) in the demo to confirm or push back on the classification.
4. Lock the decision into the running table (see output format below).
5. If a "primitive itself" classification surfaces a missing concept, flag it as a gap to address in the primitive model design before V3 ships.

### Output format — family-first table

Workflow A's primary output is a family-keyed table. Families are the rows; file variants nest underneath as a secondary structure. This keeps the session's focus on the semantic unit that actually drives primitive-catalog decisions.

**Family row shape:**

| Shader family | Active debug recipes | Deprecated debug recipes | Parameter axes represented | Runtime-binding variants? | Classification | Primitive expression | Design parameters | Decision | Notes |
|---|---|---|---|---|---|---|---|---|---|
| diffusion | shader_diffusion.json + 2 variants | — | source, radius, softness | no | Trivial composition | `ColoredOverlay + Pattern::RadialFromCorner(src)` | defaults | Drop name; use primitive form | Default tuning is obvious from pattern variant |
| concealed_light | shader_concealed_light_{both,drift,foreground}.json | — | spread, source_cutoff, edge_width | no | Earned name | `ColoredOverlay + Pattern::LinearFromEdge(src)` | spread, source_cutoff, edge_width | Preserve as named preset | Edge cutoff values are tuned; worth locking |
| focus_field | shader_focus_field.json + 7 variants | — | center, radius, falloff, binding | yes (center bound) | Primitive itself | — | (novel + bound) | Add to primitive catalog; note runtime-binding as first-class | Binding-enabled variant is load-bearing |
| radar | shader_radar.json | — | angle, sweep_width | no | Primitive itself | — | (novel spatial function) | Add to primitive catalog | Sweeping radial scan; needs new Pattern variant |
| glow | — | _DEPRECATED_shader_glow.json | — | — | Deprecated — delete | — | — | Delete (or explain preservation) | Workflow 0 flagged; no active variants |
| ... | ... | ... | ... | ... | ... | ... | ... | ... | ... |

Final artifact: a classification document saved as a sibling appendix (`./tui-vfx-v3-upgrade-appendix-shader-catalog.md`) containing the full family table plus per-family rationale notes. Per-variant detail (which specific parameter tuning is the exemplar) lives in the notes column or a secondary per-variant section when the family's classification depends on a specific variant.

### Exit criteria

Workflow A exits when:

- **Every canonical shader family** has a classification (Trivial composition / Earned name / Primitive itself / Deprecated-delete)
- Every **"Primitive itself"** classification has an associated primitive-gap note (what concept is missing, what new Pattern variant or base shader the catalog needs to cover it)
- Every **"Earned name"** classification has a rationale (what specific parameter tuning encodes design judgment worth preserving)
- Every **"Deprecated-delete"** family has an accounting entry explaining deprecation reason and confirming no active references
- Workflow 0's debug-shader inventory is fully consumed (no debug files unaccounted for)

### Time estimate

~20 shader families (down from raw file count) × 8–12 minutes each (read all variants, preview, family-level decision) = roughly 3–4 hours of focused work. Family-first structure is faster than per-file auditing for the same evidence; can be split across sessions.

---

## Workflow B — Recipe corpus curation

**Goal:** classify every recipe in the corpus for V3 porting under the William Morris principle. Outcome: a port-list, a retirement-list, and a consolidation-list.

**Why it's workflow B:** this resolves the implicit question of *what the V3 corpus actually looks like*. The main plan assumes migration but doesn't specify what's being migrated.

### Scope

All recipes under `/usr/projects/tui-vfx-recipes/recipes/` outside of the debug_recipes, vfx-probe-validation, and test/fixture subtrees. Includes:

- Top-level recipes (e.g., `ambient_banner.json`, `coin_get.json`, `cyber_glitch.json`, ~150+ files)
- Theme-organized recipes under `midcentury-modern/`, `modern_design/`, `scandi-edge/`, `scandi-inspired/`, `wargames/`, `gt-design/`, `gt-design-codex/`, `haiku_recipes1/`, `sonnet_recipes1/`, `hbf_*` families, `toolkit/`, `scenes/`, `examples/`, `experimental/`
- The family recipes within `hbf_reference/`, `hbf_board_cascade_isolated/`, `hbf_board_cascade_staged/`, `fps_victory_stages/`

Estimated total: **>500 main-corpus recipe files**, pending the authoritative count from Workflow 0's inventory. The prior estimate of "200–300 recipes" in v0.2.0 of this doc lagged live-repo reality; a 2026-04-21 filesystem check showed 509 main-corpus candidate files excluding `debug_recipes`, `vfx-probe-validation`, and obvious test/fixture subtrees. Workflow 0 produces the authoritative count and classification.

### Classification framework

Each recipe receives one of:

- **Port — useful.** Demonstrates a capability, teaches a concept, or covers a use case not already covered by another recipe. Port to V3 structure; lock in.
- **Port — beautiful.** Aesthetically excellent, thematically meaningful, or creatively notable. Port even if a similar recipe exists; beauty earns its place.
- **Consolidate.** One of a cluster of variants; keep the strongest representative and retire the rest. Note which sibling is the keeper.
- **Archive.** Fails both tests. Move to `v2_archive/` subtree; not loaded in V3.
- **Delete.** Early iteration clearly superseded, never referenced, no historical value. Requires explicit reason — not the default path.
- **Deliberate.** Cannot classify on first pass; revisit in a second round with more context or comparison.

### Process

Two passes:

**Pass 1 — fast triage (across all recipes):**
Go category-by-category (roughly alphabetical or by theme folder). For each recipe, I read the JSON and propose a classification. User either (a) accepts without preview, (b) requests preview in the demo for ambiguous cases, or (c) overrides with their own judgment. Target: 3–5 minutes per recipe on average, with hard cases deferred to Pass 2.

**Pass 2 — deliberate (recipes flagged from Pass 1):**
The "Deliberate" pile gets full treatment: preview in demo, discuss at length, decide. Some will be upgraded to "Port — beautiful" on reconsideration; others will be downgraded to "Archive" after seeing them again.

### The category axes to think along

Useful for Pass 1 classification:

- **Maturity:** which development-phase era is this from? (basic / theatric / mature / professional / impressively-theatric). Later-era work generally ports; earlier-era often consolidates.
- **Theme-grounding:** is this recipe tied to a specific theme's identity, or is it abstract? Theme-grounded recipes in ported themes usually port; abstract recipes need stronger justification.
- **Exemplarity:** is this recipe the canonical example of its pattern, or one of several? Canonical examples port; variants consolidate.
- **Cross-references:** is this recipe referenced by an example app, a test, a theme, a doc? Referenced recipes have implicit lock-in.

### Output format — two inventories, not one

Workflow B produces **two separate checked-in inventories** plus a master audit table, per the 2026-04-21 reviewer recommendation. Splitting active vs archive early turns the audit into a maintainable repository asset rather than a one-time conversation record.

**Primary artifacts:**

- **`./tui-vfx-v3-upgrade-appendix-v3-active-corpus.md`** — the list of recipes V3 actively loads, validates, teaches, and showcases. Every entry is "Port — useful" or "Port — beautiful" or a "Consolidate" keeper.
- **`./tui-vfx-v3-upgrade-appendix-v2-archive-corpus.md`** — the list of recipes retained for historical reference but not treated as a live V3 surface. Every entry is "Archive" or "Consolidate — retired sibling." Archive recipes have an explicit rule about validator coverage, searchability, and whether they remain loadable; the default is "loadable but not validated, not searched, not taught."
- **`./tui-vfx-v3-upgrade-appendix-corpus-audit.md`** — the master audit table below, which the two inventories are derived from.

**Master audit table columns (extended from v0.2.0 to capture reviewer-recommended fields):**

| Recipe path | Category | Disposition | Rationale category | Cross-refs | Validator coverage | Notes |
|---|---|---|---|---|---|---|
| ambient_banner.json | toplevel | Port — useful | useful | examples/catalog | active | Canonical banner example; referenced by catalog |
| _bottom_blinds_collapse.json | toplevel | Archive | superseded | none | inactive | Leading underscore; pairs with non-underscore version |
| legacy_example_{1..8}.json | toplevel | Delete | historical-only | none | n/a | Explicitly named "legacy"; superseded; no history value |
| cyber_glitch.json | toplevel | Consolidate-into | duplicate | none | active | Keeper; `glitch_cyber.json` consolidates into this |
| glitch_cyber.json | toplevel | Consolidate — retired | duplicate | none | archive-inactive | Consolidated into `cyber_glitch.json`; archived |
| ... | ... | ... | ... | ... | ... | ... |

**Rationale-category vocabulary** (controlled list for Workflow B classification):

- `useful` — covers a real use case or teaches a capability
- `beautiful` — aesthetically excellent, thematically meaningful, or creatively notable
- `superseded` — an earlier iteration of a later recipe
- `duplicate` — variant of another recipe; consolidate to keeper
- `historical-only` — of historical value but not earning active slot
- `deprecated-explicit` — carries explicit deprecation marker (underscore prefix, etc.)
- `generator-output` — produced by tooling; subject to generator audit, not recipe curation

**Validator-coverage flag** — whether the archived recipe remains validator-covered:

- `active` — in v3-active-corpus; validated, loaded, taught
- `archive-loadable` — in v2-archive-corpus; remains loadable for historical reference but not validator-covered by default
- `archive-inactive` — in v2-archive-corpus; not loadable at all, kept only as a filesystem artifact
- `inactive` — delete candidate; rationale in notes

### Exit criteria

Workflow B exits when:

- **The v3-active-corpus list exists** with every kept recipe categorized and a rationale captured
- **The v2-archive-corpus list exists** with every archived recipe categorized
- Every **consolidated** recipe has an explicit keeper + retired-siblings relationship recorded
- Every **archived or deleted** recipe has a validator-coverage flag set
- The repo has a clear rule for whether archived recipes remain loadable, searchable, or validator-covered (and that rule is documented in each archive inventory's header)
- The deferred "Deliberate" pile is empty (all recipes have final dispositions)

### Time estimate

~500 recipes × ~3 minutes average (Pass 1) + ~50 recipes × ~10 minutes (Pass 2) = roughly 30–35 hours. Split across multiple sessions. Pass 1 can be done in batches of 20–30 recipes per sitting. Substantially larger than the v0.2.0 estimate of 15–20 hours because the authoritative scope is closer to 500 files than 200–300.

---

## Workflow C — Structural translation sample

**Goal:** validate that the proposed V3 tree schema handles structural diversity cleanly. Translate 6–8 representative real recipes from the ported corpus into the proposed tree shape and surface any structural questions not yet raised.

**Why it's workflow C:** this resolves main-plan open questions #3 (phase-scoping shape), #4 (combine semantics), #6 (scope open-closed tension) through concrete translations.

### Mini-pilot (runs early, before full A and B) — new in v0.3.0

Per the 2026-04-21 reviewer recommendation, a **2–3 recipe mini-pilot** runs up front before full Workflow A and B commit hours of work. Reason: structural problems in a tree schema only become obvious when you actually translate real recipes. If those problems surface late, they can invalidate assumptions made in both A and B.

**Mini-pilot mechanics:**

1. Pick 2–3 recipes that span structural diversity (one simple, one multi-phase, one multi-layer or otherwise complex)
2. Translate each into the proposed V3 tree shape
3. Record the first real schema pain points — unexpected scope needs, combine-semantic ambiguities, fragment boundaries, hint-visibility questions
4. Decide: **ready** (proceed to full A and B with confidence), **ready-with-adjustments** (proceed but flag known issues for A/B attention), or **not ready** (structural revision to the plan needed before A/B begin)

Output: a mini-pilot report (`./tui-vfx-v3-upgrade-appendix-workflow-c-pilot.md`) with the translated recipes and the readiness judgment.

The mini-pilot does not replace the full Workflow C — it de-risks it. The full Workflow C runs after Workflows A and B are complete, against the mature schema and retained corpus.

### Scope

6–8 recipes selected for diversity *after Workflow B produces the port list*. Selection criteria:

- Spread across theme families (harbor, blueprint, grimoire, eichler, flw, hygge, stuttgart, japanese-minimalism, plus any theme-neutral patterns)
- Spread across complexity tiers (simple fade → multi-phase layered → complex animated)
- Spread across effect families (spatial shader, mask-driven, sampler-driven, filter-only, multi-layer composition, phase-differentiated behavior)
- Include at least one "hard case" that seems likely to stress the tree shape

### Process

For each selected recipe:

1. Read the V2 flat JSON.
2. Produce a proposed V3 tree translation.
3. Note any structural translation difficulties (e.g., "had to invent a new scope variant," "couldn't express this cleanly," "default combine mode felt wrong here").
4. User reviews, proposes alternatives if anything feels off.
5. Capture the final translation plus any surfaced questions.

### Output format

A translation document saved as `./tui-vfx-v3-upgrade-appendix-structural-translations.md`. Sections:

- One subsection per translated recipe, with V2 JSON and proposed V3 JSON side-by-side (or linearly, given JSON width)
- A running list of "surfaced questions" — new design questions discovered during translation
- A summary table: recipe → classification (translated cleanly / translated with caveats / required schema extension)

### Exit criteria

Workflow C (the full run, not the mini-pilot) exits when:

- 6–8 representative recipes have been translated into the V3 tree shape
- Surfaced schema questions are enumerated (as a running list of discoveries, not a curated report)
- The team can answer whether the tree shape is **ready**, **ready-with-adjustments**, or **not ready** for the V3 implementation cutover
- The mini-pilot's readiness judgment is either confirmed or revised by the full C results

### Time estimate

Mini-pilot: 2–3 recipes × ~30 minutes each = 1–1.5 hours. Single sitting.

Full Workflow C: 6–8 recipes × ~20 minutes each (read V2, draft V3, discuss, refine) = 2–3 hours. Single session if done in one sitting. If the mini-pilot surfaced adjustments, add time for revisiting each translation against the adjusted schema.

---

## Sequencing recommendation (revised in v0.3.0)

The 2026-04-21 reviewer recommended a revised priority that de-risks A and B with early inventory and pilot work. New suggested order:

1. **Workflow 0 — Authoritative inventory.** Blocking prerequisite for A and B. ~1–2 hours; mostly automated.
2. **Workflow C mini-pilot (2–3 recipes).** Catches structural schema issues before A and B commit hours. ~1–1.5 hours. Readiness judgment: proceed, proceed-with-adjustments, or revise plan.
3. **Workflow A — Shader catalog (family-first).** Produces the primitive catalog and the "earned names" list. ~3–4 hours.
4. **Workflow B — Corpus curation (active + archive).** Produces the two inventories and the retained set. ~30–35 hours, split across sessions. Also feeds main plan's Concern B curation phase.
5. **Workflow C — Full structural translations (6–8 recipes).** Validates the mature schema against representative kept recipes. ~2–3 hours.

Each workflow's output feeds the next. Doing them strictly in parallel would produce churn as early-workflow decisions changed later-workflow assumptions; the mini-pilot lets a limited amount of C run early without committing to full C's scope.

### Why this order changed from v0.2.0's A → B → C

v0.2.0 recommended A → B → C without front-loading inventory or pilot work. Problems with that order the 2026-04-21 reviewer surfaced:

- **Without authoritative inventory first**, time estimates and scope assumptions are unreliable — the v0.2.0 doc's "200–300 recipes" estimate lagged reality by a factor of ~2×.
- **Without an early C pilot**, structural schema issues can surface 20+ hours into Workflow B and invalidate classifications made under the prior schema assumption.
- **Without family-first Workflow A**, file proliferation skews effort and attention away from the semantic unit that actually matters.

The v0.3.0 sequencing fixes all three.

### Alternate sequencing (if Workflow B is urgent)

If there's pressure to establish the port list before the primitive catalog is fully decided (e.g., for corpus cleanup independent of V3 migration), Workflow B can run ahead of Workflow A with a caveat: its classifications are provisional and may be revisited once Workflow A surfaces primitive-catalog gaps. Inventory (Workflow 0) and mini-pilot still run first — those are not negotiable because they're the de-risk layer.

---

## Deferred-execution notes

When picking this up in a future session:

1. **Re-read the main plan chapter index first** (`./tui-vfx-v3-upgrade-plan/00_INDEX.md`) — the open questions may have evolved since this workflow doc was written.
2. **Check git history for any changes in `/usr/projects/tui-vfx-recipes/recipes/`** — the shader catalog may have grown, and new recipes may have been added that affect the corpus audit scope.
3. **Verify the demo binary still works** — `cargo run --example demo` in the tui-vfx-recipes workspace. If the example has been renamed or refactored, the invocation in Workflow A needs updating.
4. **Confirm the William Morris principle still matches the user's curation intent.** The principle as stated here was agreed in a single session and may want refinement once applied to real recipes.
5. **This workflow may split into multiple sessions.** None of the three workflows need to complete in one sitting. Track progress in the per-workflow output files as they accumulate.

## Cross-references

- Main plan: [`./tui-vfx-v3-upgrade-plan/00_INDEX.md`](./tui-vfx-v3-upgrade-plan/00_INDEX.md) — the V3 migration direction this workflow supports
- Steering principles: Intention 46 (library changes earn their place) — aligns with the "earned names" logic in Workflow A and the Morris principle in Workflow B
- Relative Light architecture RFC: historical gt-design reference at `docs/internal/specs/relative-light-architecture.md` — the ember-felt and ambient-halo explorations that motivated parts of the V3 design

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-audit-workflow.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.1</VERS> -->
