# Memo: V3.1 schema gaps surfaced by the authoring-corpus survey

**To:** V3.1 contract / compost implementers
**From:** Authoring-shorthand corpus exercise
**Date:** 2026-05-02
**Scope:** Real authoring patterns the V3.1 contract cannot losslessly receive

A 180-recipe survey of the V2/V3/V3.1 corpus produced the alias and expansion
table evidence for the upcoming authoring shorthand. In the process three
contract-level gaps surfaced where the V3.1 canonical surface cannot express
authoring patterns the V3 corpus relies on. Flagging them here because they
are schema-shape decisions, not shorthand-surface decisions.

The shorthand can absorb anything the canonical contract can. It cannot
paper over a missing canonical variant.

## Gap 1 — `ScopeSpec` lost channel scoping

**Contract:** `crates/tui-vfx-contract/src/cls_scope_spec.rs:18` defines
`ScopeSpec` with eleven variants: `All`, `Role`, `Rect`, `Cell`, `RowRange`,
`ColumnRange`, `ModuloRows`, `ModuloColumns`, `NonEmpty`, `OuterBand`,
`Inner`. There is no `Channel` variant.

**Corpus pattern** (V3 form, 30+ witnesses): `scope: { "kind": "channel",
"value": "foreground" }` and `{ "kind": "channel", "value": "background" }`.
Used to scope shaders/filters/style-effects to one cell channel without
touching the other. This is a load-bearing pattern in production-tier design
(modular shader stacks where AO/diffusion/concealed-light each target only
the background while text/border treatments scope to foreground).

**Witnesses:**

- `recipes/debug_recipes/scene/scene_layer_full_stack.json` — pipeline has
  `{kind: "filter", scope: {kind: "channel", value: "foreground"},
  payload: {type: "invert"}}` paired with the same effect on background.
- `recipes/debug_recipes/complex/complex_diamond_highlight.json` —
  background-only `focused_row_gradient` shader plus text-only `highlighter`
  shader, demonstrating per-channel role differentiation.
- `recipes/gt-design/bold/B10_grimoire_incantation_bar_palette.json` —
  multi-region styles array with `region: "BackgroundOnly"` carrying
  `glisten_band`, distinct from a `RowRange`-scoped overlay.
- `recipes/experimental/subtle-light/L06_hygge_lantern_diffusion.json` —
  three `region: "BackgroundOnly"` style entries each carrying a different
  background shader (diffusion, concealed_light); the foreground stays
  untouched throughout.
- `recipes/gt-design/mid-range/M11_blueprint_circuit_pulse_info.json` —
  `BackgroundOnly` carries the `trace_path` shader, `BorderOnly` carries
  border sweep, `TextOnly` carries pulse — three distinct channels active
  simultaneously.

**Migration cost:** approximately a third of the corpus uses channel
scoping. Without a V3.1 path, every one of those recipes needs a manual
restructure during V3 → V3.1 migration. The closest V3.1 substitute is
`Role { role }` if an appropriate `RoleTag` exists, or `NonEmpty` for
text-vs-empty (different semantics — does not cleanly replace
foreground-vs-background).

**Recommendation:** add `Channel { channel: ChannelTag }` (or equivalent)
to `ScopeSpec`. Probably parallels how `MaskCombineMode` lives outside the
contract today and similarly needs promotion.

## Gap 2 — `ScopeSpec` lost row-index sets

**Contract:** `ScopeSpec::RowRange { start, end }` covers contiguous
ranges. There is no `Rows { indices: Vec<usize> }` for non-contiguous
selections.

**Corpus pattern:** `region: { "Rows": [0, 10] }` (V2 form). Used when an
effect applies to specific row indices that aren't a contiguous range —
typically the top and bottom trim rows of a multi-row composition.

**Witness:**

- `recipes/fps_victory_stages/optC_trim_shader_40.json` uses
  `region: { "Rows": [0] }` and `region: { "Rows": [10] }` with focus_field
  + linear_gradient shaders to paint the top and bottom trim bands of a
  victory banner. The shape requires both rows to share the same shader
  treatment without affecting rows 1–9 in between. A `RowRange { 0, 1 }` +
  `RowRange { 10, 11 }` pair would functionally substitute but doubles the
  graph nodes.

**Migration cost:** small in absolute terms (1–3 recipes), large in
expressivity (the graph-node duplication required to substitute is the kind
of authoring tax the V3.1 cleanup is trying to remove).

**Recommendation:** consider `Rows { indices: Vec<u16> }` (and parallel
`Columns`). Alternative: leave it out and let the small witness count
absorb the doubling.

## Gap 3 — `edge_crossing.shadow` coordination has no V3.1 contract home

**Contract:** `crates/tui-vfx-geometry/src/borders/border_trim_spec.rs:16`
defines `BorderTrimSpec` with edge/corner-level
`Keep|Blank|Horizontal|Vertical` segments.
`crates/tui-vfx-geometry/src/borders/fnc_vanishing_edge_trim_spec.rs:20`
exposes `vanishing_edge_trim_spec(direction, frame_area, dwell_rect,
visible_area)` to compute the trim spec at runtime. The border side of the
panel-crosses-viewport-edge effect is fully wired.

The shadow side is missing. There is no
`crates/tui-vfx-contract/` field, struct, or enum for
`edge_crossing.shadow: "fade" | "preserve"`. ripgrep across all crates
returns zero hits for `edge_crossing` or `EdgeCrossing`.

**Corpus pattern:** `motion.edge_crossing: { edge, border, shadow }` is the
V2/V3 recipe-side authoring shape. `border: "vanish"` invokes the trim
strategy described above. `shadow: "fade"` tells the shadow renderer to
fade the shadow on the same edge as the panel crosses through, so a panel
sliding through the bottom of the viewport doesn't leave a stranded shadow
floating below the fold.

**Witnesses:**

- `recipes/debug_recipes/motion_routes/toast_shadow_diagonal_edge_crossing.json`
  — bottom-right diagonal slide with `edge_crossing: { edge: "top",
  border: "vanish", shadow: "fade" }` on enter and a matching
  `edge_crossing` on exit. The shadow uses
  `composite_mode: "grade_underlying"` with a multi-field `grade` block,
  so the fade-on-edge-crossing is the difference between a clean transit
  and a shadow that smears across the viewport boundary.
- `recipes/debug_recipes/easings/ease_back_out.json`,
  `ease_elastic_out.json`, `ease_bounce_out.json`, `ease_bezier_custom.json`
  — five easing fixtures all carry
  `edge_crossing: { edge, border: "preserve", shadow: "fade" }` on enter
  and exit to make the easing demos read cleanly when they overshoot the
  viewport edge.

**Total witnesses:** 5+ explicit `edge_crossing` recipes plus 4 explicit
`trim: "vanishing_edge"` recipes. The two coordinate.

**Recommendation:** decide whether the shadow-fade-on-edge-crossing
behavior is

1. **Implicit-runtime** — the shadow renderer detects source-clipping
   automatically and does the right thing without recipe input. Contract
   stays clean. The recipe-side `edge_crossing.shadow` field becomes a
   no-op alias that the canonicalizer drops.
2. **Explicit shadow contract field** — `ShadowSpec` (or wherever) gains
   an `edge_crossing_policy: ShadowEdgeCrossingPolicy` enum
   (`Fade`/`Preserve`/`Default`). Recipe-side `edge_crossing.shadow`
   canonicalizes to this field.

Option 1 is simpler if the runtime can detect clipping reliably. Option 2
is necessary if authors need to override the default per-recipe.

## What was confirmed working

The other contract types I looked at are correct and complete:

- `BorderTrimSpec` + `vanishing_edge_trim_spec` covers the border side
  of edge-crossing cleanly.
- `TransitionSubjects { from, to, shared }` covers two-subject relation
  transitions plus shared-element variants. The `shared` field is
  unwitnessed in the corpus but well-defined in the contract for
  shared-glyph / shared-axis transitions.
- `LifecyclePhase { Enter, Dwell, Exit }` covers all corpus phase usage.
  V3's `"all"` is shorthand-only.
- `MaskCombineMode { All, Any, Blend { ratio } }` (in
  `tui-vfx-compositor-next`, not yet in the contract crate) — works as
  expected; promotion to contract is a separate question.
- `GlyphTimelineTriggerSpec` has four variants
  (`Immediate`/`PhaseOffset`/`Wavefront`/`PoissonBurst`); corpus has only
  Wavefront and PoissonBurst witnessed but the others are contract-real
  and ready.

## Suggested next steps for the implementer

1. **Decide gap 1.** Channel scoping is high-traffic; recommend adding
   `Channel { channel }` to `ScopeSpec` before the V3 → V3.1 corpus
   migration.
2. **Decide gap 2.** Row-index sets are low-traffic; either add `Rows {
   indices }` or accept the duplication.
3. **Decide gap 3.** Pick implicit-runtime vs explicit-contract for
   shadow-during-edge-crossing.
4. **Promote `MaskCombineMode`** to `tui-vfx-contract` if it's the
   intended V3.1 home for multi-mask combination semantics.

The corpus and decisions document at
`schemas/v3.1/authoring/corpus/DECISIONS.md` records every other settled
question. Open shorthand-surface questions are pre-recommended on corpus
evidence and don't block these schema-side calls.
