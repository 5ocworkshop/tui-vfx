<!-- <FILE>docs/design/tui-vfx-v3-upgrade-audit-workflow.md</FILE> - <DESC>Deferred-execution workflow for the three audits that feed the V3 tui-vfx upgrade: (A) shader catalog decomposition — per-shader evaluation of whether each named shader is a trivial primitive composition or earns its name; (B) recipe corpus curation under the William Morris "useful or beautiful" principle — audit every recipe and classify port/retire; (C) structural translation sample — re-express 6–8 representative recipes in the proposed tree shape to stress-test the structure. All three are deferred to a future session and are not blocking current work.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Renamed from tui-vfx-pipeline-tree-schema-audit-workflow.md to tui-vfx-v3-upgrade-audit-workflow.md in lockstep with the main plan rename. Content unchanged; cross-refs to main plan updated.</WCTX> -->
<!-- <CLOG>0.2.0: rename to tui-vfx-v3-upgrade-audit-workflow.md; update cross-refs to the renamed main plan.
0.1.0: initial draft. All three workflows captured with tour-order proposals, output-format templates, and decision frameworks. Sequencing recommendation included. Execution deferred.</CLOG> -->

# V3 Pipeline Migration — Audit & Curation Workflows

> **Status: deferred.** Three audit workflows captured here for execution in a future session. None are blocking current work. Together they produce the empirical inputs required to resolve several open questions in the main plan ([`./tui-vfx-v3-upgrade-plan.md`](./tui-vfx-v3-upgrade-plan.md)).

## Context

The V3 tree-schema migration depends on three empirical inputs that this session's conversation could only reason about abstractly:

1. **What's the actual shape of the named-shader catalog?** We discovered 27+ distinct named shaders during the session, larger than initial estimates. The decision "primitive-only vs preserve named compositions" can't be made cleanly without going through each one.
2. **What does the recipe corpus actually contain, and which parts still earn their place?** Recipes have accumulated across several development phases (basic → theatric → mature → professional → impressively theatric), some theme-grounded and some abstract. Mechanical porting would preserve archaeology, not design. A Morris-principle curation pass is the right shape for the audit.
3. **Does the proposed tree schema handle structural diversity?** The shape sketches in the main plan cover three cases. Real recipes from across themes will surface additional structural patterns and likely generate 2–4 more open questions.

Each of these is a distinct audit with its own output. This document defines all three.

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

### Infrastructure that exists

- Per-shader debug recipes at `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/` — 52 recipe files covering ~27 distinct named shader types (many with parameter variants).
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

### Output format

A single table accumulated across the session:

| Shader | Classification | Primitive expression | Design parameters | Decision | Notes |
|---|---|---|---|---|---|
| diffusion | Trivial composition | `ColoredOverlay + Pattern::RadialFromCorner(src)` | source, radius, softness | Drop name; use primitive form | Default tuning is obvious from pattern variant |
| concealed_light | Earned name | `ColoredOverlay + Pattern::LinearFromEdge(src)` | spread, source_cutoff, edge_width | Preserve as named preset | Edge cutoff values are tuned; worth locking |
| radar | Primitive itself | — | (novel spatial function) | Add to primitive catalog | Sweeping radial scan; needs new Pattern variant |
| ... | ... | ... | ... | ... | ... |

Final artifact: a classification document saved as a sibling appendix (`./tui-vfx-v3-upgrade-appendix-shader-catalog.md`) containing the full table plus per-shader rationale notes.

### Time estimate

25–27 shaders × ~5 minutes each (2–3 for read, 1–2 for user preview, 1 for decision capture) = roughly 2–2.5 hours of focused work. Can be split across sessions.

---

## Workflow B — Recipe corpus curation

**Goal:** classify every recipe in the corpus for V3 porting under the William Morris principle. Outcome: a port-list, a retirement-list, and a consolidation-list.

**Why it's workflow B:** this resolves the implicit question of *what the V3 corpus actually looks like*. The main plan assumes migration but doesn't specify what's being migrated.

### Scope

All recipes under `/usr/projects/tui-vfx-recipes/recipes/` outside of the debug_recipes, vfx-probe-validation, and test/fixture subtrees. Includes:

- Top-level recipes (e.g., `ambient_banner.json`, `coin_get.json`, `cyber_glitch.json`, ~150+ files)
- Theme-organized recipes under `midcentury-modern/`, `modern_design/`, `scandi-edge/`, `scandi-inspired/`, `wargames/`, `gt-design/`, `gt-design-codex/`, `haiku_recipes1/`, `sonnet_recipes1/`, `hbf_*` families, `toolkit/`, `scenes/`, `examples/`, `experimental/`
- The family recipes within `hbf_reference/`, `hbf_board_cascade_isolated/`, `hbf_board_cascade_staged/`, `fps_victory_stages/`

Estimated total: **200–300 recipes.** An accurate count should be established as the first step of this workflow.

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

### Output format

A master audit table saved as `./tui-vfx-v3-upgrade-appendix-corpus-audit.md`. Columns:

| Recipe path | Category | Classification | Rationale | Cross-refs | Notes |
|---|---|---|---|---|---|
| ambient_banner.json | toplevel | Port — useful | Canonical banner example; referenced by catalog | examples/catalog | — |
| _bottom_blinds_collapse.json | toplevel | Archive | Leading underscore suggests already-deprecated shape; no refs | none | Pairs with non-underscore version |
| legacy_example_*.json | toplevel | Archive | Explicitly named "legacy"; superseded by newer examples | none | 8 files |
| ... | ... | ... | ... | ... | ... |

### Time estimate

200–300 recipes × ~3 minutes average (Pass 1) + ~30 recipes × ~10 minutes (Pass 2) = roughly 15–20 hours. Split across multiple sessions. Pass 1 can be done in batches of 20–30 recipes per sitting.

---

## Workflow C — Structural translation sample

**Goal:** validate that the proposed V3 tree schema handles structural diversity cleanly. Translate 6–8 representative real recipes from the ported corpus into the proposed tree shape and surface any structural questions not yet raised.

**Why it's workflow C:** this resolves main-plan open questions #3 (phase-scoping shape), #4 (combine semantics), #6 (scope open-closed tension) through concrete translations.

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

### Time estimate

6–8 recipes × ~20 minutes each (read V2, draft V3, discuss, refine) = 2–3 hours. Single session if done in one sitting.

---

## Sequencing recommendation

Suggested order:

1. **Workflow A first (shader catalog).** Produces the primitive catalog and the "earned names" list. Informs the V3 schema's primitive surface. Bounded scope (~25 shaders), highest leverage per hour of work.
2. **Workflow B second (corpus audit).** Produces the port list. Requires Workflow A's primitive catalog to be settled so that "can this recipe be expressed in V3?" has a concrete answer. Largest effort.
3. **Workflow C third (structural translations).** Requires Workflows A and B to be settled: the translations use the V3 primitive catalog and select from the ported corpus. Smallest effort of the three.

Each workflow's output feeds the next. Doing them in parallel would produce churn as early-workflow decisions changed later-workflow assumptions.

### Alternate sequencing (if Workflow B is urgent)

If there's pressure to establish the port list before the primitive catalog is fully decided (e.g., for corpus cleanup independent of V3 migration), Workflow B can run ahead of Workflow A with a caveat: its classifications are provisional and may be revisited once Workflow A surfaces primitive-catalog gaps. This adds churn but doesn't block.

---

## Deferred-execution notes

When picking this up in a future session:

1. **Re-read the main plan doc first** (`./tui-vfx-v3-upgrade-plan.md`) — the open questions may have evolved since this workflow doc was written.
2. **Check git history for any changes in `/usr/projects/tui-vfx-recipes/recipes/`** — the shader catalog may have grown, and new recipes may have been added that affect the corpus audit scope.
3. **Verify the demo binary still works** — `cargo run --example demo` in the tui-vfx-recipes workspace. If the example has been renamed or refactored, the invocation in Workflow A needs updating.
4. **Confirm the William Morris principle still matches the user's curation intent.** The principle as stated here was agreed in a single session and may want refinement once applied to real recipes.
5. **This workflow may split into multiple sessions.** None of the three workflows need to complete in one sitting. Track progress in the per-workflow output files as they accumulate.

## Cross-references

- Main plan: [`./tui-vfx-v3-upgrade-plan.md`](./tui-vfx-v3-upgrade-plan.md) — the V3 migration direction this workflow supports
- Steering principles: Intention 46 (library changes earn their place) — aligns with the "earned names" logic in Workflow A and the Morris principle in Workflow B
- Relative Light architecture RFC: [`../specs/relative-light-architecture.md`](../specs/relative-light-architecture.md) — the ember-felt and ambient-halo explorations that motivated parts of the V3 design

<!-- <FILE>docs/design/tui-vfx-v3-upgrade-audit-workflow.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
