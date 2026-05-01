# M4 — Round 3 patterns (32 additional recipes from broader directories)

Append to `M4_PATTERNS.md` and `M4_PATTERNS_v2.md`. 32 new recipes read across 13
directories (`wargames`, `midcentury-modern`, `modern_design`, `scandi-inspired`,
`scenes`, `tte_inspired`, `toolkit/{core,showcase,sizzle}`, `experimental`,
`experimental/subtle-light`, `haiku_recipes1`, `sonnet_recipes1`,
`5.5-suggestions`, `examples`, `vfx-probe-validation`). 8 paired files written
for new pattern witnesses; 24 confirm existing patterns.

## Headline finds — major new architectural patterns

### H1. **`extends` template inheritance** (5 demonstrators — promoted immediately)

Every wargames recipe uses:

```json
"extends": "themes/wopr_green.json"
```

This is V3-witnessed-but-not-yet-V3.1 architecture. Five concrete witnesses:
`wargames_shall_we_play`, `norad_01_missile_warning`, `norad_05_defcon_status`,
`map_05_missile_trajectory_02`, `new_rapid_flip_2`. Recipes inherit a base
template, then override only the differing fields (typically `message` +
`lifecycle`).

V3 docs: "deep-merge + tagged-union-replacement merge semantics (Intention 39)."

**This is the most important new pattern in round 3.** It implies:

- Authoring shorthand needs an `extends:` top-level field
- Canonicalizer must resolve `extends` chains at load time, producing a fully-
  expanded `RecipeDocument`
- The resolved-but-intent-preserving form needs an extra metadata slot —
  parallel to `TransitionIntent::Preset`/`::Alias`, recipes need
  `RecipeIntent::Extends { template, overrides }` or similar
- Theme/template authoring is a separate-but-related design exercise: where do
  template files live? How are they discoverable? Validated?

**Decision needed:** is V3.1 going to support `extends`, or is it deliberately
flat-only? The corpus says "extends is real ergonomic value for theme
families" — but the V3.1 contract layer doesn't currently have a template
mechanism. This is a schema-shape question, not a shorthand question. Worth
raising before M5.

The `wargames_shall_we_play` shorthand pair demonstrates the proposed shape.

### H2. **`filter.glyph_timeline` with frames + triggers** (2 demonstrators — TTE parity)

`tte_inspired/beams.json` and `tte_inspired/sweep.json` use a deeply structured
filter that authors per-cell glyph timelines:

```json
"frames": [
  { "glyph": "▂", "fg": "white", "duration_ticks": 1 },
  { "fg": [130, 145, 170], "duration_ticks": 1 }
],
"trigger": {
  "kind": "poisson_burst",
  "lane_axis": "row",
  "batch_period_frames": 6,
  "batch_size_min": 1, "batch_size_max": 3,
  "lane_speed_min": 11.0, "lane_speed_max": 29.0,
  "shuffle_seed": 11, "batch_seed": 23, "speed_seed": 41,
  "fps": 60.0,
  "jitter": { "seed": 81, "amount_seconds": 0.05 }
},
"on_complete": "hide"
```

Two trigger variants witnessed: `poisson_burst` (axis-stochastic batches) and
`wavefront` (eased sweep across an axis). Both have detailed jitter / seed /
speed parameters.

This is a **major new filter family** that doesn't fit the simple
`{kind: "filter", payload: {...}}` shape. The shorthand pair for `tte_beams`
flattens duration_ticks → "ticks", and the trigger sub-tree to a
constructor-shaped object. Every other field stays verbatim — the shape is
already as compact as the authoring concept allows.

Two witnesses crosses rule-of-three weakly. Promoted with the caveat that the
trigger shape is itself worthy of its own schema-design pass.

### H3. **`spatial_shader` embedded in style block** (V2 form — 5+ demonstrators)

V2/V3 recipes embed a shader inside the style payload:

```json
"style": {
  "enter_effect": {
    "type": "spatial",
    "shader": { "type": "glisten_band", "speed": 1.5, "head": [...], "tail": [...] }
  }
}
```

Demonstrators: `midcentury_connected`, `midcentury_success`, `modern_material_card`,
`modern_gloss_highlight_sweep`, `modern_skeleton_shimmer`, `scandi_info`,
`L06_hygge_lantern_diffusion`. Plus `coin_get` from round 2.

**This is a cross-version idiom for "shader during style phase."** V3.1
canonical separates these into independent graph nodes (one for the style fade,
one for the shader). The shorthand form should accept the embedded form as
sugar:

```json
"transitions": {
  "enter": {
    "preset": "fade",
    "shader": { "type": "glisten_band", ... }
  }
}
```

The canonicalizer expands this into two graph nodes: the opacity.fade track
plus a separate shader node with `activePhases: ["enter"]`. Already shown in
`midcentury_connected.json` shorthand pair.

**Promoted to rule.** 7+ witnesses across rounds 1-3.

### H4. **`border.title` system** (5+ demonstrators)

Bordered cards with titles:

```json
"border": {
  "type": "rounded",
  "title": "◉ ONLINE",
  "title_position": "top",
  "title_alignment": "center"
}
```

Witnesses: `midcentury_{connected,error,success}`, `scandi_info`,
`L06_hygge_lantern_diffusion`, `ease_bezier_custom` (round 1).

Shorthand: `border: { type: "rounded", title: "...", titleAlign: "center" }`.
`title_position` defaults to `"top"`; explicit only when not top.

**Promoted to rule.**

## Other new patterns — promoted

### H5. **`shader.linear_gradient` with stops** (5 demonstrators)

```json
"gradient": {
  "stops": [[0.0, color], [0.5, color], [1.0, color]],
  "space": "rgb"
},
"angle_deg": 90.0
```

Witnesses: `tte_beams`, `tte_sweep`, `ink_in_water`, `scene_layer_nested_parallel_sequences`,
`hbf_board_cascade_isolated/02_checkerboard_only` (cited but not read).

Shorthand: `{ shader: "linear_gradient", stops: [[t, color]...], angle: <deg>, applyTo: "..." }`.
Color in stops accepts the standard hex/RGB-array forms.

**Promoted.**

### H6. **`shader.rainbow`** (4 demonstrators)

`aurora_cascade`, `prism_refraction`, `arcade_coin`, `aurora_drift`. Often
attached as `enter_effect: { type: "rainbow", speed: ... }` in V2 form.
V3.1 canonical form lifts to a graph node: `{ effect: "shader.rainbow",
inputs: { speed: ... }, activePhases: [...] }`.

**Promoted.**

### H7. **Multi-scope styles array** (V2 form — 2 demonstrators)

`L06_hygge_lantern_diffusion` and `digital_rain` (round 1) use:

```json
"styles": [
  { "region": "All", "base_style": {...} },
  { "region": "BackgroundOnly", "base_style": {...}, "spatial_shader": {...} }
]
```

Multiple scoped styles in one block. V3.1 canonical: each becomes a separate
graph node (or scene-element surface override). Two witnesses; below rule-of-
three but worth noting because the V2 idiom is so common in mature recipes.

**Defer to canonical** until a third witness surfaces.

### H8. **`mask.cellular`** (1 demonstrator) and **`mask.radial`** (1 demonstrator)

`ghost_orchard` uses `mask.cellular` with patterns `organic` / `voronoi`.
`sonar_popover` uses `mask.radial` with `origin`. Both single-witness; treat as
canonical-only until corpus grows. Shorthand pairs show the proposed
`transitions: { enter: { preset: "cellular", pattern: "...", ... } }` and
`transitions: { enter: { preset: "radial", origin: "..." } }` shapes.

### H9. **`mask.checkers`** (2 demonstrators)

`arcade_coin` and `complex_diamond_highlight` use checkerboard masks with
`cell_size`. Two witnesses; below rule-of-three but visually common enough
that it'll likely cross with a fourth recipe.

### H10. **`mask.diamond`** as enter mask (4+ demonstrators)

`battery_low_meter`, `circuit_trace` (round 2), `prism_refraction`, `gravity_well`
exit mask. Plus the iris-with-diamond-shape form. Diamond is a common enter
shape. **Promoted.**

## Filter family grew significantly

After round 3, the filter primitive catalog has these clear witnesses:

| Filter | Round 1-2 Witnesses | Round 3 New |
|---|---|---|
| `filter.dim` | 3 | — |
| `filter.tint` | 1 | `battery_low_meter`, `frosted_sheen_toast`, `aurora_drift`, `ink_in_water` (4) |
| `filter.invert` | 1 | `circuit_trace`, `arcade_coin` |
| `filter.crt` | 0 | `arcade_coin`, `heartbeat_monitor`, `aurora_drift`, `frosted_sheen_toast`, `digital_rain` |
| `filter.greyscale` | 1 (complex_crt) | `scene_layer_nested_parallel_sequences` |
| `filter.vignette` | 0 | `material_card`, `frosted_sheen_toast`, `aurora_drift`, `film_noir_smoke`, `digital_rain` |
| `filter.crt_jitter` | 1 (bsod_v3) | `arcade_coin`, `heartbeat_monitor` |
| `filter.fault_line` | 0 | `battery_low_meter`, `ink_in_water` |
| `filter.glyph_timeline` | 0 | `tte_beams`, `tte_sweep` (NEW family) |
| `filter.braille_dust` | 0 | `ghost_orchard` (NEW) |
| `filter.subcell_light` | 0 | `L06_hygge` (NEW) |
| `filter.bracket_emphasis` | 0 | `sonar_popover` (NEW) |
| `filter.scalar_field_glyph` | 1 (terminal_fire) | — |
| `filter.pill_button` | 2 | — |
| `filter.rigid_shake` | 1 | — |
| `filter.glitch_lines` | 1 (cinematic_reveal) | — |

Many of these are still single- or two-witness, but the filter catalog
shows a clear "core 6" (dim/tint/invert/crt/greyscale/vignette) plus a long tail
of specialized primitives. The shorthand for any of them is the same shape —
`{ filter: "<name>", ...inputs }`.

## Shader family grew significantly

| Shader | Round 1-2 Witnesses | Round 3 New |
|---|---|---|
| `shader.glisten_band` | 1 (concealed_light_drift comment) | `midcentury_connected`, `midcentury_success`, `modern_material_card`, `modern_gloss_highlight_sweep`, `modern_skeleton_shimmer`, `scandi_info`, `frosted_sheen_toast` (7+) |
| `shader.concealed_light` | 1 | `L06_hygge` |
| `shader.diffusion` | 1 (round 2 implicit) | `L06_hygge` (mode: "breath" — NEW) |
| `shader.linear_gradient` | 1 (nested) | `tte_beams`, `tte_sweep`, `ink_in_water` |
| `shader.rainbow` | 0 | `aurora_cascade`, `prism_refraction`, `arcade_coin`, `aurora_drift` (4) |
| `shader.pulse_wave` | 1 | `gravity_well`, `battery_low_meter` |
| `shader.pulse` | 0 | `heartbeat_monitor` (NEW — distinct from pulse_wave) |
| `shader.edge_sheen` | 0 | `ghost_orchard` (NEW) |
| `shader.radar` | 0 | `sonar_popover` (NEW) |
| `shader.focused_row_gradient` | 1 (complex_diamond) | `sonar_popover` |
| `shader.reflect` | 1 (coin_get) | — |
| `shader.border_sweep` | 1 | — |
| `shader.highlighter` | 1 (complex_diamond) | — |
| `shader.terminal_fire` | 1 | — |
| `shader.sub_cell_shake` | 1 | — |

Rule-of-three crossings: `glisten_band` (now 7+), `linear_gradient` (4), `rainbow` (4),
`pulse_wave` (3), `concealed_light` (2 — close), `diffusion` (2 — close).

## V3 vs V3.1 idiom differences (sharper picture)

Round 3 made the V2/V3 → V3.1 mapping clearer for several patterns:

1. **`enter_effect: { type: "spatial", shader: {...} }`** → V3.1 graph node with
   `activePhases: ["enter"]`. The shader becomes a separate node.
2. **`enter_effect: { type: "rainbow", speed: ... }`** → V3.1 `shader.rainbow` graph
   node with `activePhases: ["enter"]`. Same lift.
3. **`enter_effect: { type: "fade_in", apply_to: "both", ease: "QuadOut" }`** → V3.1
   transition with `opacity.fade` track. Or `style.fadeIn` graph node depending
   on whether the recipe wants compositional control.
4. **`exit_effect: { type: "glitch", seed, intensity }`** → V3.1 `style.glitch` graph
   node with `activePhases: ["exit"]`.
5. **`style.spatial_shader: { type: "reflect", ... }`** → V3.1 `shader.reflect`
   graph node, scope from the style's region.
6. **`time: { loop: true, loop_period_ms }`** → V3.1 `lifecycle.clock: { mode:
   "looping", period: { kind: "milliseconds", value } }`. Already canonical.

The shorthand form unifies these: `transitions: { enter: { preset: "fade",
shader: {...} } }` for fade-with-shader, `effects: [{ shader: "rainbow", phase:
"enter" }]` for direct shader-attached-to-phase. The canonicalizer expands
both to the same graph-node-list canonical form.

## Updated count summary

After rounds 1 + 2 + 3:

- **103 recipes read** (39 + 32 + 32)
- **26 paired files written** (10 + 8 + 8)
- **77 recipes confirm existing patterns** without new pairs
- **9 new patterns** in round 3 (H1–H9), of which 4 promoted to rule
- **5 patterns crossed rule-of-three** in round 3: extends, glisten_band,
  rainbow, linear_gradient, border.title
- **1 schema-shape question raised:** whether V3.1 should support `extends`

## Updated transition-preset coverage

| Preset | Witnesses (rounds 1-3) | Status |
|---|---|---|
| `Crossfade` | 0 | Faith-from-schema only |
| `Wipe` | 4+ (`mask_wipe_corner_out`, `digital_rain`, `chamfered_corners_demo`, `frosted_sheen_toast`, `heartbeat_monitor`, `aurora_drift`, `modern_skeleton_shimmer`, `modern_gloss_highlight_sweep`, `material_card`, `midcentury_success`, `circuit_trace` exit) | **Strongly promoted** — 10+ |
| `Iris` | 3 (round 2) | Promoted |
| `Push` | 0 | Faith-from-schema only |
| `Dissolve` | 3 (round 2) | Promoted |
| `Morph` | 0 (content_morph is content not transition) | No transition witness |
| `Stippled` | 0 | No witness |
| `Braille` | 0 | No witness |
| `fade` (canonical: `opacity.fade` or `style.fadeIn`) | 10+ | **Strongly promoted** |
| `blinds` | 3 | Promoted |
| `cellular` (NEW — H8) | 1 (`ghost_orchard`) | Single-witness, push to canonical |
| `radial` (NEW — H8) | 1 (`sonar_popover`) | Single-witness |
| `checkers` (NEW — H9) | 2 | Below threshold |
| `diamond` (NEW — H10) | 4+ | Promoted (often inside iris.shape: diamond, also as bare `mask.diamond`) |

`Crossfade`, `Push`, `Morph`, `Stippled`, `Braille` remain without corpus
witnesses. Worth noting that `Morph` has a closely-named content effect that
isn't a transition — the alias table will need to disambiguate
`morph` → `content.morph` (effect) vs `morph` → ??? (transition, which
doesn't yet have a corpus example).

## Recipes confirmed by round 3 (no new pair, cited as evidence)

| Recipe | Patterns it confirms or extends |
|---|---|
| `wargames_norad_01_missile_warning` | H1 (extends) — confirmed witness |
| `wargames_norad_05_defcon_status` | H1 |
| `wargames_map_05_missile_trajectory_02` | H1 |
| `wargames_new_rapid_flip_2` | H1 |
| `midcentury_error` | H4 (border.title), H6 (rainbow not), `mask.iris` exit (E1 already promoted) |
| `midcentury_success` | H4, H3 (spatial_shader), `mask.wipe` enter |
| `modern_material_card` | H3, multi-mask compose, `filter.vignette` |
| `modern_gloss_highlight_sweep` | H3 (glisten band sweep), `mask.wipe`, `mask.dissolve` |
| `modern_skeleton_shimmer` | H3, multi-effect dwell |
| `scandi_info` | H3, `motion.arc`, `mask.dissolve` |
| `scenes/example_card` | V2 scene-block format, `default_role`, `fit_policy` |
| `scenes/example_procedural` | Same plus `phase`-array visibility (V2 form) |
| `tte_inspired_sweep` | H2 (wavefront trigger variant), confirms H3 patterns |
| `toolkit/core/battery_low_meter` | `filter.fault_line`, `shader.pulse_wave`, mask.diamond + wipe combo |
| `toolkit/core/frosted_sheen_toast` | H3, `filter.vignette`, `filter.tint` exit |
| `toolkit/showcase/aurora_drift` | H3, `mask.noise_dither`, `sampler.ripple`, H6 (rainbow) |
| `toolkit/showcase/film_noir_smoke` | sampler.sine_wave both phases, `filter.vignette` both phases |
| `toolkit/showcase/ink_in_water` | `mask.dissolve`, `sampler.ripple`, `sampler.fault_line`, `filter.tint`, H5 (linear_gradient) |
| `toolkit/sizzle/arcade_coin` | `content.scramble_glitch_shift` (combined), `mask.checkers`, multi-effect, H6 |
| `experimental/speech_bubble_demo` | `border.frame` second witness (G7), `border.type: "none"` + frame override |
| `experimental/subtle-light/L06` | H7 (multi-style array), H8 (single witness) |
| `haiku_recipes1/aurora_cascade` | H6, sampler.sine_wave |
| `sonnet_recipes1/gravity_well` | `mask.iris` exit, `motion.spiral`, H6 (pulse_wave shader as enter_effect) |
| `sonnet_recipes1/prism_refraction` | H6 |
| `examples/typewriter_perlin_variance` | typewriter base, no effects |

## Open design questions surfaced in round 3

These add to the seven from M4_PATTERNS.md:

8. **`extends` template support in V3.1.** Five wargames recipes prove this is
   real ergonomic value. Does the V3.1 contract grow a template-resolution
   layer, or is it deliberately flat-only and `extends` lives only in the
   shorthand surface (resolved at canonicalization)? My suggestion: **shorthand-
   only**, expanded by the canonicalizer. Templates live as separate JSON files
   under `schemas/v3.1/authoring/templates/` (or wherever consumers store them).
   The canonicalizer reads the template, deep-merges the recipe's overrides,
   and produces the flat canonical form. `RecipeIntent::Extends { template,
   resolvedAt }` records provenance for diagnostics.

9. **`filter.glyph_timeline` schema design.** The trigger sub-tree
   (`poisson_burst` / `wavefront`) is its own mini-language. Is this a single
   filter primitive with a polymorphic `trigger` discriminator, or two
   separate filter primitives (`filter.glyph_timeline_burst`,
   `filter.glyph_timeline_wavefront`)? The corpus reads as one filter with
   two triggers, but the validator may prefer separate primitives for
   discoverability.

10. **`shader` vs `style.spatial_shader`.** V2/V3 sometimes attached shaders
    via `style.spatial_shader: {...}` (single shader on a style block) and
    sometimes via `style.enter_effect: { type: "spatial", shader: {...} }`
    (one shader per phase). V3.1 canonical lifts both to graph nodes. The
    shorthand should pick one form: **either** all shader attachments go via
    the top-level `effects: [...]` block with explicit `phase`, **or** the
    `transitions: { ... }` form accepts a nested `shader: { ... }` field.
    Both accepted? My suggestion: both, but `effects: [...]` is the canonical
    teaching form (it composes predictably with phase + scope), while the
    nested form in `transitions` is a sugar shorthand for "fade-with-shader."

11. **Theme/template directory layout.** If `extends` is supported (Q8), the
    template files need a documented home. The corpus uses paths like
    `themes/wopr_green.json` and `themes/new_wopr_fullscreen_cyan.json`,
    suggesting `themes/` adjacent to recipes. For V3.1, candidates:
    `schemas/v3.1/authoring/templates/`, or external (consumer-managed). The
    canonicalizer needs to know how to resolve template references — by path
    (relative? absolute? namespace?) or by id (registry-based?).

The original 7 questions still stand; these 4 new ones are tied to round 3
findings. Total open design questions before M5: **11**.
