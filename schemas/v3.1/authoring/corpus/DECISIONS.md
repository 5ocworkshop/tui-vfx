# Authoring shorthand — settled decisions

Companion to the M4_PATTERNS series. Records design questions raised across
seven corpus rounds and the answers settled so far.

The corpus exercise read 180+ V2/V3/V3.1 recipes and authored 5 schema-faith
demonstrators for the upcoming relation transitions (crossfade, push, morph)
and visibility transitions (stippled, braille). 47 paired files
(canonical/shorthand) live alongside this document; M4_PATTERNS.md through
M4_PATTERNS_v7.md trace the round-by-round findings.

## Resolved by user decision

### Q3 — Mask routing rule (mask-as-effect vs mask-as-transition)

**Approved.** Deterministic rule:

- Presence of `phase: "dwell"` or a multi-phase array → effect form
  (`effects: [{ mask: "...", phase: [...] }]`, canonicalizes to graph node).
- Otherwise → transition form
  (`transitions: { enter: { preset: "..." } }`, canonicalizes to a
  `visibility.*` track).

Both forms remain accepted; the canonicalizer disambiguates by phase shape.

### Q7 — Asset reference syntax

**Approved.** `"$asset:<id>"` string form, paralleling `$bind:` for runtime
bindings. Object form `{ "$asset": "<id>", "format": "..." }` available when
explicit format hint is needed. Top-level `assets: { ... }` block remains
the declaration site.

Three corpus witnesses (Madeira full scene, authoring-ladder, Canada flag)
crossed rule of three.

### Q10 — Shader-on-style sugar

**Agreed.** `effects: [...]` is the canonical teaching form — composes
predictably with phase + scope. Nested shader inside a transition block
(`transitions: { enter: { preset: "fade", shader: {...} } }`) accepted as
sugar for the fade-with-shader idiom; canonicalizer expands to two graph
nodes (the opacity.fade track plus a separate shader node with
`activePhases: ["enter"]`).

10+ V2/V3 witnesses for the V2 `style.spatial_shader` form across midcentury,
modern_design, scandi-inspired, gt-design family, and L06_hygge.

## Resolved by surgical lookup against the contract

### Q8 — `extends` template support

**Architecture is already settled: canonicalization-only.**

`load_recipe_with_extends` at `xtask/src/recipes/mod.rs:286-342` resolves
template inheritance at canonicalization time with deep-merge + cycle
detection. The merge function recursively merges objects (child wins
per-key), replaces arrays and primitives outright. The V3.1 contract
(`tui-vfx-contract`) has no `extends` field — by the time a recipe reaches
`LoadedRecipe::load`, extends has resolved into a flat `RecipeDocument`.

Remaining work is implementation-only:

1. The V3.1 canonicalize function (`tui-vfx-contract::canonicalize::*`) needs
   to invoke the existing extends-resolution logic (or reimplement it using
   the same deep-merge semantics).
2. `RecipeIntent` should record the extends-chain for diagnostics, parallel
   to `TransitionIntent::Preset { preset }`.

### Q11 — Theme/template directory layout

**Co-located convention.** Templates live next to the recipe family that uses
them (e.g., `recipes/wargames/themes/wopr_green.json`). No central template
directory is needed; `extends: "themes/wopr_green.json"` is resolved
relative to the recipe's directory, falling back to the recipes root.

### Q9 — `glyph_timeline` trigger discriminator (revised by lookup)

`GlyphTimelineTriggerSpec` at
`crates/tui-vfx-compositor-next/src/types/cls_filter_spec.rs:1581` is a
**closed four-variant enum**, not the two the corpus initially suggested:

| Variant | Wire form | Witnesses |
|---|---|---:|
| `Immediate` | `{ "kind": "immediate" }` | 0 (contract-only) |
| `PhaseOffset` | `{ "kind": "phase_offset", base_offset_seconds, phase_offset_x_ms, phase_offset_y_ms }` | 0 (contract-only) |
| `Wavefront` | `{ "kind": "wavefront", axis, total_duration_seconds, base_offset_seconds?, easing?, jitter? }` | 1 (`tte_inspired/sweep`) |
| `PoissonBurst` | `{ "kind": "poisson_burst", lane_axis, batch_period_frames, batch_size_{min,max}, lane_speed_{min,max}, ...seeds, fps, direction_seed?, jitter? }` | 1 (`tte_inspired/beams`) |

Polymorphic `trigger: { kind: "..." }` discriminator. Corpus has 2 of 4
witnessed; the remaining two (`Immediate`, `PhaseOffset`) need
contract-faith entries in the alias/expansion tables.

### Q1 — Card / motion / border shorthand fields

`BorderTrimSpec` at `tui-vfx-geometry/src/borders/border_trim_spec.rs:16` is
not an enum of named modes. It's an explicit struct with 8 fields (4 edges
+ 4 corners), each carrying `BorderSegment::{Keep, Blank, Horizontal,
Vertical}`. The string `"vanishing_edge"` in V2 recipes is a *strategy
name*, not an enum value — the loader invokes
`vanishing_edge_trim_spec(direction, clip)` at runtime to compute the
per-segment spec from slide direction.

Field promotions (witnesses across 174 recipes):

| Field | Witnesses | Status |
|---|---:|---|
| `border.title`, `titlePosition`, `titleAlign` | 68 | Promote |
| `border.padding` | 45 | Promote |
| `border.frame` (custom glyphs) | 5 | Already promoted (v4) |
| `border.trim: "vanishingEdge"` (with `motion.edgeCrossing` companion) | 4+5 coupled = 9+ | **Promote** — load-bearing for slide-through-edge with shadow integration |
| `border.trim` (other non-default values) | 0 | Canonical-only escape hatch |

The `motion.edgeCrossing: { edge, border, shadow }` field is a V2/V3
recipe-side authoring shape with **no Rust contract type and zero
appearances in `tui-vfx-contract` or any current source**. The
vanishing-edge trim mechanism survives via `BorderTrimSpec` +
`vanishing_edge_trim_spec()`. The shadow-cross-edge coordination
(`edgeCrossing.shadow: "fade"`) does not have an obvious V3.1 contract
counterpart and may be a re-add gap. Flag for the design queue; the
shorthand carries the legacy spelling as an alias-with-deprecation-note
until the runtime story for shadow-during-edge-crossing is settled.

### Q15 — `compose:` vs `combineMode:` shorthand naming

`MaskCombineMode` at
`crates/tui-vfx-compositor-next/src/types/mask_combine_mode.rs:28` confirms
the V3 canonical name `MaskCombineMode` with three variants (`All`, `Any`,
`Blend { ratio }`) serialized via `rename_all = "snake_case"`.

The shorthand multi-track transition field `combineMode:` matches the V2/V3
mask-combine spelling family for author familiarity. It is a
**shorthand-only field** for grouping multiple tracks under one transition
preset entry; V3.1 transitions canonicalize to a flat `tracks: [...]` array
with no explicit combine mode at the canonical level (tracks compose by
being in the array).

Pick `combineMode:` over `compose:`.

## Architectural property — alias mechanism is versionless

The alias and expansion tables (`schemas/v3.1/authoring/{transition,sampler,...}/aliases.json`
+ `expansion.json`) sit outside the canonical schema. The canonicalize
function reads them; `recipe.schema.json` does not reference them.

Adding/overloading aliases does **not** require a V3.1 schema bump:

| Change | Schema bump? | Contract change? |
|---|---|---|
| Add new author-friendly spelling | No | No — alias-table row only |
| Add new preset combining existing canonical primitives | No | No — expansion-table row only |
| Add parameter override to existing alias | No | No — alias-table row |
| Promote a corpus-witnessed pattern to convention | No | No — table additions only |
| Loosen alias-table format (new sigils) | No | Bump alias-table meta-schema (independent of canonical) |
| Add new canonical primitive (new shader/filter descriptor) | Yes | Yes — descriptor pack + possibly contract |
| Add new canonical track kind (`relation.*` etc.) | Yes | Yes — contract enum extension |
| Change canonical envelope shape | Yes | Yes |

The version of V3.1 stays at "3.1" across all alias/expansion table
extensions. Only when the canonical primitive catalog itself needs to grow
does the contract need touching.

## Still open

Six questions remain. None are blockers; most are pre-recommended on
evidence and need user sign-off:

- **Q2** — Phase shorthand semantics. Drafted from corpus.
- **Q4** — Alias/canonical naming strategy. Two-table design drafted.
- **Q5** — Multi-layer scene shorthand. `siblingLayer` (9 witnesses) and
  `follow.lag_ms` (4 witnesses) promote; `relative_to` (2) canonical-only.
- **Q6** — Color named-set. 17 names witnessed: ANSI-16 + light variants +
  light_gray + dark_gray + reset + transparent.
- **Q14** — Region/scope vocabulary. V2/V3 → V3.1 normalization mapping.
- **Q16** — Two-subject relation transitions. Implicit-by-default with
  explicit override drafted in v6.

## Counts at this commit

- 180 corpus recipes read across 7 rounds (39 + 32 + 32 + 32 + 39 + 4 + 2)
- 5 schema-faith demonstrators authored (crossfade, push, morph, stippled, braille)
- 47 paired files (canonical / shorthand)
- 7 M4 docs + 1 addendum + this DECISIONS.md
- 8 questions resolved (Q3, Q7, Q8, Q10, Q11, Q9, Q1, Q15)
- 2 questions resolved without user input (Q12, Q13 — multi-architecture
  dynamics; no shared Polyline)
- 6 open questions, all pre-recommended
