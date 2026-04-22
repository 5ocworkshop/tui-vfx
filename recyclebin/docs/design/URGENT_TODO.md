# URGENT TODO

Raised: **2026-04-20 23:00 local (UTC+01:00)**
Owner: unassigned (flagged during tui-vfx-recipes border-trim default flip)

---

## Move `shadow` into the v2 recipe `config` schema

### Current state

`tui-vfx-recipes` exposes a V2 recipe `config` schema (see
`tui-vfx-recipes/src/recipe_schema/config.rs` → `RaRecipeConfig`) whose
allowed top-level fields are:

    theme, message, layout, lifecycle, border, content, time, pipeline,
    scene, requires_primitives

There is **no** `shadow` field. The `pipeline-validator` enforces
`#[serde(deny_unknown_fields)]`, so any recipe author who tries to set a
shadow at the recipe level is rejected with

    unknown field `shadow`, expected one of `theme`, `message`, `layout`, ...

### What works anyway

`gt-design` accepts `shadow` at a **higher layer** of its own recipe
ingestion path — grep against `gt-design/themes/**/*.json` finds shadow
blocks in many shipped theme recipes (hero theme, others). Those recipes
render with shadow correctly inside the gt-design rendering pipeline,
because gt-design's wrapper parses the shadow field before the recipe
reaches tui-vfx's engine.

### The mismatch

Consequence: a recipe authored with `"shadow": { ... }`
- **works** in gt-design's demo/preview surfaces
- **fails to parse** in `tui-vfx-recipes`'s `pipeline-validator`,
  `recipe-probe`, and legacy `demo` browser
- **silently drops the shadow** on any path that consumes the narrower V2
  schema without gt-design's wrapper

This is exactly the kind of upstream/downstream divergence the Sub-plan
A/B authority split was meant to prevent.

### What needs to happen

1. Land a `shadow: Option<RaShadowConfig>` field on `RaRecipeConfig` in
   `tui-vfx-recipes/src/recipe_schema/config.rs`.
2. Mirror the ingestion shape of whatever gt-design currently accepts
   (so recipes don't have to rewrite to move upstream). The gt-design
   shadow config lives around `crates/gtd-ratatui/...` — pick the
   canonical struct and serialise it identically.
3. Wire the field through `fnc_preview_from_config.rs` →
   `PreviewItem.shadow` so the `pipeline-validator` / `demo` /
   `recipe-probe` paths all render shadow the same way.
4. Update `pipeline-validator` `--rules` checks to add a sanity rule:
   shadow shouldn't be active on recipes with no border or on
   full-viewport recipes (anti-footguns).
5. Update `CAPABILITIES_REFERENCE.md` and the `/recipe-author` skill
   anatomy section — both already document a `shadow` block as if it
   existed at `config.*`, which is aspirational until this lands.

### Why it's deferred right now

Another agent is doing open-heart surgery on the rendering pipeline
(Sub-plan B phase B.1–B.2: scene composer, procedural layers, stock
layers, recipe-ID trace emission). Adding a new top-level schema field
during that work would collide with their refactor and almost certainly
get reverted the same way the border-trim Animation-guard fix was
reverted earlier today. Wait until the pipeline surgery lands.

### Related context

- Border-trim default flip (committed `d549dbe` on
  `tui-vfx-recipes:main`) surfaced the mismatch: the recipe-author skill
  documents shadow as if it were settable at the recipe level, several
  existing gt-design theme recipes use it, but the V2 schema rejects it.
- No tui-vfx-recipes fixture currently exercises a shadowed toast — add
  one (`default_toast_shadowed.json` or similar) alongside the shadow
  field so visual-QA has a reference.

---

## Sibling work — SSOT authority audit for recipe-config fields

The shadow mismatch is almost certainly not unique. When I audited the
render plan for the border-trim default-flip work I also found
`fnc_calculate_motion_rect` silently clamping negative signed positions
(see
`/usr/projects/tui-vfx-recipes/AUDIT-SLIDE-MEMO.md`), which meant
`from_left` / `from_top` slides never reported a partial-visible area and
therefore never triggered `apply_slide_border_trim`. Two distinct bugs
surfaced by one feature.

That's a signal. The recipe-config surface has grown organically, with
gt-design accepting fields above the line and tui-vfx-recipes accepting
a different subset below the line. Any remaining mismatches are
invisible until an author tries the "wrong" field at the wrong layer.
Before we spend another week chasing these one at a time, we need to do
it once properly.

### Action items (follow-on to the shadow fix)

1. **Audit every recipe-level config field in the wild.** Walk
   every `*.json` under `gt-design/themes/` + `tui-vfx-recipes/recipes/`
   and collect the full set of top-level keys inside `config.*` plus
   every per-phase pipeline key. Diff against:
   - `RaRecipeConfig` in `tui-vfx-recipes/src/recipe_schema/config.rs`
     (the current V2 schema — the `deny_unknown_fields` contract),
   - whatever wrapper / extension point gt-design uses to accept its
     extras (shadow, motion tokens, palette refs, placement refs).

   Any field that parses in gt-design but not upstream is a mismatch and
   belongs on the audit list.

2. **Establish an SSOT authority policy.** Recipe-level config is a
   **tui-vfx** concern unless there is a strong, explicit argument for
   app-level ownership. Default answer for any new config surface is
   "goes in the V2 schema". Exceptions need to be written down on a
   case-by-case basis with their rationale — "gt-design needs tokens
   resolved against its palette before the engine sees RGB" is a real
   exception; "nobody upstream has touched it yet" is not.

   Shadow definitively belongs at the tui-vfx level, not the app level.
   It's a pure rendering concern, gt-design's shadow semantics should
   just be one theme's parameterization of the same upstream mechanism,
   not a parallel implementation the app maintains alone.

3. **Document the authority boundary.** Once we've decided where each
   field lives, the split needs to be explicit:
   - a one-page `docs/RECIPE_CONFIG_OWNERSHIP.md` or similar in
     `tui-vfx/docs/` listing every recipe-level field, where it's
     parsed, where it's resolved, and why (for app-level exceptions);
   - a rule in the recipe-author skill / `PIPELINE_VALIDATOR_LLM_GUIDE`
     explicitly stating the default-to-upstream policy;
   - a failing check in `pipeline-validator --debug-recipes-qc` for any
     field that lives above the line without an entry on the exception
     list.

4. **Add a trace event for recipe-config field resolution.** The
   border-trim bug was only catchable by hand-patching an `eprintln!`
   into `apply_slide_border_trim` because there's no `TraceEvent`
   variant that covers "this config field was resolved to this value and
   here's whether the consumer honored it" (see the discussion in
   `/usr/projects/tui-vfx-recipes/AUDIT-SLIDE-MEMO.md` §"Why the probe
   tool earned its keep"). Add a variant alongside `TokenResolved` that
   covers recipe-config → runtime-policy resolution (`border.trim →
   SlideBorderTrimPolicy`, `shadow → ShadowSpec`, anything else with a
   default that an author can omit).

5. **Bake these audits into CI.** The audit should run on every PR that
   touches the V2 schema OR the gt-design recipe ingestion layer — not
   once, not "every quarter". Once we've done the work, the check is
   cheap to keep running.

### Priority

Do this **immediately after** the shadow migration (items 1–5 in the
first section above), not before. The shadow work gives us a concrete
precedent for "how to move a field upstream cleanly" and fixtures to
test the policy against. Doing the audit first would produce a list of
mismatches with no migration pattern yet.

---

*End of URGENT_TODO.md. Delete this file after the shadow migration
lands **and** the SSOT audit has cleared every mismatch above the line.*
