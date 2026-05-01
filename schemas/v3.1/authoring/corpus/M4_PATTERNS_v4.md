# M4 — Round 4 patterns (32 production-aesthetic recipes)

Append to `M4_PATTERNS.md`, `_v2.md`, and `_v3.md`. 32 new recipes read across
13 directories: `gt-design/{bold,mid-range,restrained}`, `gt-design-codex`,
`5.5-suggestions`, `experimental/subtle-light`, `vfx-probe-validation`,
`toolkit/{sizzle,showcase}`, `hbf_board_cascade_{isolated,staged}`,
`fps_victory_stages`, `internal`, `test_fixtures`. 8 paired files written for
distinct new pattern witnesses.

## Headline finds — round 4

### J1. **`shader.trace_path` with `paths: [{points, delay}]` array** (2 demonstrators — major new architectural pattern)

`M11_blueprint_circuit_pulse_info` and `blueprint_inspection_gate_modal` both
use a path-routed shader where authored polylines drive moving signals across
the surface:

```json
"shader": {
  "type": "trace_path",
  "color": [...],
  "tail_length": 10.0,
  "junction_glow": 1.0,
  "paths": [
    {
      "points": [{"x":1,"y":1}, {"x":16,"y":1}, {"x":16,"y":7}, ...],
      "delay": 0.0
    },
    { "points": [...], "delay": 0.5 }
  ]
}
```

This is a **substantially new architectural pattern**: a shader that consumes
authored geometry data. Not just per-cell uniform parameters but per-shader
data tables. Other shaders likely belong to this family (custom polygon-fill,
authored particle paths, etc.) but only `trace_path` witnesses today.

Two witnesses; weak rule-of-three. Promoted with a caveat that the geometry
sub-tree (points, polylines, delay sequencing) is itself a design surface
worth its own schema pass. Shorthand pair flattens points to `[[x, y], ...]`
arrays.

### J2. **Composed motion route + dynamics array** (1 demonstrator, but architecturally distinct)

`newtons_cradle.json` uses:

```json
"route": {
  "type": "composed",
  "route": { "type": "arc", "bulge": 2.0 },
  "dynamics": [
    { "type": "pendulum", "amplitude": 3.0, "oscillations": 1.0, "damping": 0.0 }
  ]
}
```

Plus `follow: { mode: "maintain_offset", lag_ms: 1100 }` for sibling
phase-offset on a second element.

The motion-route family now witnesses: linear, helix, infinity, bounce,
rectilinear, spiral, arc, **composed**, plus path-relative motion and
sibling-follow with lag. Composed wraps another route plus a dynamics array.
Dynamics witnessed: `pendulum` (amplitude / oscillations / damping). Other
likely dynamics in the future: spring, inertia, friction, gravity.

Single-witness for the composed shape; the shorthand pair shows the proposed
nested form. Push to canonical-only until a second composed witness surfaces.

### J3. **Multi-effect dwell-array** (5+ demonstrators — promoted)

V2 dwell can be a single object OR an array of effects:

```json
"filter": {
  "dwell": [
    { "type": "braille_dust", ... },
    { "type": "tint", ... },
    { "type": "vignette", ... }
  ]
}
```

Witnesses: `B01_grimoire_natural_twenty`, `B10_grimoire_incantation_bar`,
`M11_blueprint_circuit_pulse`, `gt-design-codex/grimoire_summoning_index`,
`L01_harbor_hidden_rail_shell`, `L09_blueprint_subcell_raster`. Six witnesses,
crosses rule-of-three strongly.

V3.1 canonical lifts each array entry to a separate graph node with the same
`activePhases: ["dwell"]`. Shorthand: a `stack` keyword inside an effects entry
groups multiple sub-effects sharing a phase:

```json
"effects": [
  { "phase": "dwell", "stack": [
    { "filter": "braille_dust", ... },
    { "filter": "tint", ... },
    { "shader": "pulse", ... }
  ]}
]
```

Already shown in `grimoire_natural_twenty.json` shorthand pair. **Promoted.**

### J4. **Region scoping vocabulary** (V2 form, promoted)

V2/V3 recipes use `region: ` with multiple shapes:

| Region form | Witnesses |
|---|---|
| `"All"` | universal |
| `"BackgroundOnly"` | gt-design family, subtle-light, M11, harbor_signal_lantern |
| `"TextOnly"` | M11, harbor_signal_lantern, ghost_orchard, coin_get |
| `"BorderOnly"` | M11, gt-design-codex/grimoire, harbor_signal_lantern, blueprint_inspection_gate |
| `{"RowRange": {"start": N, "end": M}}` | B10 grimoire_incantation, R10 fuji_ma, gt-design-codex/grimoire_summoning_index |
| `{"Rows": [N, ...]}` | fps_victory_stages |

V3.1 canonical scope vocabulary: `{kind: "channel", value: "background|foreground"}`,
`{kind: "content", value: "text"}`, `{kind: "border"}`, `{kind: "rowRange",
start, end}`, `{kind: "rows", indices: [...]}`. Shorthand: `{channel: "..."}`,
`{content: "text"}`, `{role: "border"}`, `{rowRange: [start, end]}`,
`{rows: [...]}`.

10+ witnesses across rounds 1-4. **Strongly promoted.**

### J5. **Combined-mask with `combine_mode` array form** (2+ demonstrators)

V2/V3 form:
```json
"mask": {
  "combine_mode": "all",
  "enter": [
    { "type": "wipe", "direction": "horizontal_center_out" },
    { "type": "iris", "shape": "box" }
  ],
  "exit": [...]
}
```

Witnesses: `R05_fuji_shoji`, `digital_rain` (round 1), `blueprint_inspection_gate_modal`.
Three witnesses crosses rule-of-three.

V3.1 canonical lifts each array entry to a separate graph node. Shorthand
introduces `transitions: { enter: { compose: "all", tracks: [...] } }` for
multi-track transitions. Already shown in `fuji_shoji_modal.json` pair.
**Promoted.**

## Shader catalog — final shape after round 4

| Shader | Total witnesses (rounds 1-4) | Status |
|---|---:|---|
| `glisten_band` | 12+ | Core |
| `linear_gradient` | 6 | Core |
| `rainbow` | 5 | Core |
| `pulse_wave` | 4 | Core |
| `concealed_light` | 4 | Core |
| `diffusion` | 3 | Core (with mode: breath variant) |
| `border_sweep` | 4 | Core |
| `pulse` | 5 | Core (distinct from pulse_wave) |
| `focused_row_gradient` | 6 | Core |
| `edge_sheen` | 3 | Specialized |
| `highlighter` | 3 | Specialized |
| `bevel` | 2 | Specialized |
| `colored_overlay` | 3 (with edge_shadow pattern) | Specialized |
| `trace_path` | 2 | New (J1) |
| `radar` | 1 | Single |
| `glow` | 1 | Single |
| `barber_pole` | 1 | Single |
| `ambient_occlusion` | 1 | Single |
| `chromatic_edge` | 1 | Single |
| `wayfinding_node` | 1 | Single (J — unique to L05) |
| `focus_field` | 2 (L01, fps_victory_stages) | Specialized |
| `stochastic_sparkle` | 1 | Single |
| `terminal_fire` | 1 | Single |
| `sub_cell_shake` | 1 | Single |
| `reflect` | 1 | Single |

**Core shaders (rule-of-three crossed):** glisten_band, linear_gradient, rainbow,
pulse_wave, concealed_light, diffusion, border_sweep, pulse, focused_row_gradient,
edge_sheen, highlighter, colored_overlay. **12 shaders.**

**Specialized (2 witnesses):** bevel, trace_path, focus_field. **3 shaders.**

**Single-witness:** 10 shaders. Push to canonical-only.

## Filter catalog — final shape after round 4

| Filter | Total witnesses | Status |
|---|---:|---|
| `dim` | 5 | Core |
| `tint` | 7 | Core |
| `invert` | 3 | Core |
| `crt` | 6 | Core |
| `vignette` | 7 | Core |
| `greyscale` | 3 | Core |
| `crt_jitter` | 4 | Core |
| `fault_line` | 3 | Core |
| `braille_dust` | 3 | Core |
| `subcell_light` | 4 | Core (with renderMode: braille|horizontal) |
| `pill_button` | 2 | Specialized |
| `rigid_shake` | 2 | Specialized |
| `glyph_timeline` | 2 (TTE beams + sweep) | Specialized (J — unique trigger sub-tree) |
| `bracket_emphasis` | 1 | Single |
| `glitch_lines` | 1 | Single |
| `kitt_scanner` | 1 | Single (J — forward_wrap motion mode) |
| `glisten_sweep` | 1 | Single |
| `pattern_fill` | 1 | Single (with pattern.horizontal_lines) |
| `underline_wipe` | 2 | Specialized |
| `scalar_field_glyph` | 1 | Single |
| `glyph_style` | 1 | Single (multi-rule glyph re-color) |
| `marquee` (content) | 1+ | — |

## Style effects (V2/V3 names that lift to V3.1 graph nodes)

| Style effect | Witnesses | Lifts to |
|---|---:|---|
| `fade_in` / `fade_out` | 15+ | `opacity.fade` track or `style.fadeIn` node |
| `pulse` (style.dwell_effect) | 5+ | `shader.pulse` node |
| `color_shift` | 3 (round 1, ghost_orchard, velvet_faultline) | `style.color_shift` node |
| `color_fade` | 1 (style_color_fade) | `style.color_fade` node |
| `glitch` | 2 (arcade_coin, heartbeat_monitor) | `style.glitch` node |
| `rainbow` | 4 | `shader.rainbow` node |
| `spatial` (with shader child) | 10+ | the embedded shader becomes its own node |

## Updated transition-preset coverage

| Preset | Witnesses (rounds 1-4) | Status |
|---|---:|---|
| `Wipe` | 15+ (with directions: leftToRight, rightToLeft, topToBottom, bottomToTop, fromLeft, horizontalCenterOut, horizontalEdgesIn, corner_out_from_bottom_left) | Strongly promoted |
| `fade` | 12+ | Strongly promoted |
| `Iris` | 6 (added: shape: box from R05/R10) | Promoted (3 shapes: circle, diamond, box) |
| `Dissolve` | 6 (mask + content) | Promoted with disambiguation |
| `Diamond` | 6+ (mask.diamond standalone) | Promoted |
| `Blinds` | 5 (added: blueprint_inspection_gate, schematic_reveal, gothic_cathedral_glass) | Promoted |
| `Cellular` | 1 (ghost_orchard) | Single witness |
| `Radial` | 3 (sonar_popover, alarm_lighthouse enter+exit) | Promoted |
| `Checkers` | 3 (added: kaleidoscope_prism, arcade_coin from round 3) | Just-promoted |
| `Noise_dither` | 3 (aurora_drift, cathedral_of_static, velvet_faultline) | Just-promoted |
| `Path_reveal` (with spiral) | 3 (cinematic_reveal, fuji_enso, velvet_faultline) | Just-promoted |
| `Crossfade` | 0 | No witness |
| `Push` | 0 | No witness |
| `Morph` (transition) | 0 | No witness (content.morph effect exists) |
| `Stippled` | 0 | No witness |
| `Braille` | 0 | No witness |
| `rect_scale` (scale-based) | 2 | Below threshold |

After round 4, the transition expansion table is well-evidenced for the 11
witnessed presets. The five missing-from-corpus (Crossfade, Push, Morph,
Stippled, Braille) remain faith-from-schema.

## Updated count summary

After rounds 1 + 2 + 3 + 4:

- **135 recipes read** (39 + 32 + 32 + 32)
- **34 paired files written** (10 + 8 + 8 + 8)
- **101 recipes confirm existing patterns** without new pairs
- **Round 4 surfaced 5 distinct new patterns** (J1–J5), 4 promoted, 1 single-witness
- **Multiple rule-of-three crossings** in round 4: dwell-array, multi-region scope, combined-mask, mask.iris-box, mask.checkers, mask.noise_dither, mask.path_reveal-with-spiral

## New design questions surfaced in round 4

These add to the eleven from prior rounds.

12. **`shader.trace_path` geometry sub-schema.** The polyline format
    (`points: [{x, y}, ...]`, `delay` per path) is a small geometry mini-language.
    Should the V3.1 contract expose a `Polyline` type that other shaders can
    reuse (custom particle paths, polygon fills, etc.), or keep `trace_path`'s
    geometry private to the shader? My take: extract `Polyline` as a shared
    type. The corpus already implies multiple shader families want path-routed
    behavior.

13. **Motion dynamics array — first-class or shader-private?** `pendulum`
    currently lives inside `route.composed.dynamics`. Other dynamics in the
    primitive catalog: spring, ADSR-shaped (different from binding ADSR),
    inertia, gravity. Should the dynamics array be a first-class motion
    primitive shape (parallel to `route` types), or a part of `route.composed`
    only? My take: keep nested under `composed`; dynamics modify a base route
    rather than stand alone.

14. **Region/scope vocabulary unification.** V2/V3 used PascalCase region
    names (`BackgroundOnly`, `RowRange`); V3.1 canonical uses lowercase
    discriminator-tagged objects (`{kind: "channel", value: "background"}`).
    Authoring shorthand should be terse: `{channel: "background"}`,
    `{content: "text"}`, `{role: "border"}`, `{rowRange: [start, end]}`,
    `{rows: [...]}`. The canonicalizer maps the V2/V3 PascalCase names to V3.1
    discriminators; both forms accepted at shorthand layer.

15. **Multi-track transition shorthand.** `fuji_shoji_modal` uses two enter
    masks combined. Shorthand `transitions: { enter: { compose: "all",
    tracks: [...] } }` makes the multi-track form readable. Single-track
    transitions keep the flat form. Is `compose: "all"` the right name, or
    should it be `combineMode: "all"` matching the V2/V3 `combine_mode` field
    name?

Total open design questions before M5: **15**.
