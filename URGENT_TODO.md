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

*End of URGENT_TODO.md. Delete this file after the shadow migration
lands.*
