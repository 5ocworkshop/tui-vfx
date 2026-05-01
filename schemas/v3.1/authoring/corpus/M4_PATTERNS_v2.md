# M4 — Round 2 patterns (32 additional recipes)

Append to `M4_PATTERNS.md`. 32 new recipes read; 8 paired files written for new
pattern witnesses. Remaining 24 confirm existing patterns.

## Rule-of-three crossings (promotions from "noted" to "rule")

The original M4 deferred several patterns as single- or two-witness. Round 2
crossed three thresholds:

### E1.iris — promoted (3 demonstrators)
- `mask_iris` (mask.iris with shape: circle | diamond)
- `center_expand` (rect_scale acts as iris-equivalent visibility transition)
- `complex_diamond_highlight` (mask.diamond as enter mask in cinematic stack)

Iris is now a load-bearing transition preset alongside fade/wipe/blinds.
Expansion table maps `iris` → `[{ kind: "visibility.iris", subject: "to",
shape: "<circle|diamond|square>", softEdge: <bool> }]`.

### E1.dissolve — promoted (3 demonstrators)
- `mask_dissolve` (cell-level stochastic visibility)
- `content_dissolve` (V3 — character-level fade-to-dot)
- `complex_diamond_highlight` (vertical blinds at exit acts as dissolve-shaped
  visibility termination — close-but-distinct, weak third witness)

Critical clarifier: **dissolve is two distinct primitives at the same name.**
Cell-level mask.dissolve operates on cell visibility; content.dissolve replaces
characters. Same alias, different canonical track. The expansion table needs
to disambiguate by what the recipe is doing, not by the alias alone. Concrete:
the `transitions: { enter: { preset: "dissolve" }}` shorthand defaults to the
mask form (cell-level visibility); the content form is `effects: [{ content:
"dissolve", ... }]`. Authors who want cell-level write the transition; authors
who want character-level write the effect.

### A4 named easings — promoted to closed list
With round 2, the corpus exercises 11 named easings: `linear`, `quadOut`,
`quadIn`, `cubicOut`, `cubicIn`, `sineInOut`, `expoOut`, `backOut`, `bounceOut`,
`elasticOut`, plus their inverse pairs (`back_in`, `bounce_in`, `elastic_in`).
The canonicalizer accepts any of these as bare strings; anything else is a
custom bezier object.

Closed list now well-evidenced. The canonical-named-easing table can be
generated directly from this corpus.

## New patterns surfaced (single- or two-witness — defer to canonical)

### G1. `rect_scale` motion (2 demonstrators)
- `bottom_blinds_collapse` (exit-only rect_scale with origin: bottom_center)
- `center_expand` (both-phase rect_scale with origin: center)

Shape: `rect_scale: { origin: "<anchor>", min_width: <int>, min_height: <int> }`.
Used as alternative to `route` for scale-based transitions. Two witnesses;
worth noting but not yet a rule. Shorthand `lifecycle.<phase>.scale` proposed
in `center_expand.json` shorthand pair as a candidate.

### G2. Path types beyond linear/helix/infinity (2 new demonstrators)
- `coin_get`: `route: { type: "bounce", bounces: 4, decay: 0.5 }`
- `circuit_trace`: `route: { type: "rectilinear", x_first: true }`

Plus: `path_reveal: { path: { type: "spiral", rotations: 3.0, direction:
"clockwise" }}` from `complex_cinematic_reveal` (mask payload, not motion
route).

Six route types now witnessed: `linear`, `helix` (alias `carrierOrbit`),
`infinity` (alias `figureEight`), `bounce`, `rectilinear`, `spiral`. Closed
list candidates.

### G3. `frame_permille` placement (1 demonstrator)
- `circuit_trace`: `from: { type: "frame_permille", x_permille: 100, y_permille: 900 }`

Permille-based positioning. Single-witness; push back to canonical until
another recipe uses it.

### G4. `glyph_emitters` content sub-tree (1 demonstrator)
- `content_glyph_particles_base_spray`

Whole sub-tree of structured content with `origin`, `spawn_count`,
`glyph_palette`, per-emitter `motion`, `lifetime_ms`, `concurrency`, `stagger`.
Shorthand pair shows the proposed flattened form. Single-witness; the corpus
suggests this is a distinct primitive family (effects vs sources vs emitters)
worth its own design pass when it gains more witnesses.

### G5. `style.spatial_shader` attached to style block (1 demonstrator)
- `coin_get`: `style.spatial_shader: { type: "reflect", speed: 3.0, color: "white" }`

Embedded shader in a style payload. Resolved by lifting the shader to a
separate graph node in the canonical form. Single-witness in V3 corpus; the
V3 → V3.1 translation for `coin_get` exposes the lifted form. Worth noting
that V3 shorthand co-locates style + shader for ergonomics; V3.1 canonical
separates them.

### G6. Multi-binding loopback on one effect (1 demonstrator)
- `loopback_rigid_shake_severity_ramp`: `error_severity` and `severity_decay`
  both bound on the same `rigid_shake` filter

Two `requires_bindings` entries with independent ramp loopbacks. Shorthand
pair (not written) would simply be two entries in the top-level `bindings`
block plus two `$bind:` references on the effect inputs. Confirms existing
patterns; no new shorthand needed.

### G7. Shadow optical-tuning fields (2 demonstrators)
- `shadow_side_three_quarter_column`: `side_coverage_eighths: 6`, `inset_y: 1`,
  `inset_y_end: 1`, `falloff_x: 2`, `falloff_y: 1`
- `shadow_braille_density_texture`: `style: { braille: { density: 0.65 } }`

Shadow shorthand in `shadow_side_three_quarter_column.json` proposes:
- `inset: [ystart, yend]` for symmetric/asymmetric inset
- `falloff: [x, y]` (already had)
- `sideCoverage: "6/8"` for fractional column width
- `style: "braille:0.65"` or object form for textured shadows

Two witnesses each crosses to weak rule-of-three with the existing
`shadow_bottom_centered_inset` and `shadow_gradient_soft_layers`. Promoted to
rule for the next pass: shadow shorthand carries `inset`, `falloff`,
`sideCoverage`, plus `style` accepting `"solid"` | `"gradient:N"` |
`"braille:D"` strings.

### G8. Shadow `composite_mode: grade_underlying` with grade params (1 demonstrator)
- `toast_shadow_diagonal_edge_crossing`: `composite_mode: grade_underlying`
  with `grade: { fg_dim_strength, bg_dim_strength, fg_desaturate_strength,
  bg_desaturate_strength, fg_tint_strength, bg_tint_strength, ... }`

Single-witness. The `grade` sub-block has nine fields; complex enough that
shorthand for it is its own design exercise. Push to canonical until it gains
witnesses.

### G9. Signal-graph composition operators (3 new operators witnessed)
- `clamp(perlin, 0.4, 1.8)` — `bounded_chaos_noise_signal`
- `mix(triangle, sine, 0.5)` — `morph_between_two_signals_signal`
- `add(keyframes, spatial_noise)` — `layered_keyframes_drift_signal`

Plus the prior `multiply(sine, adsr)` from round 1. Four binary/ternary
operators now witnessed: `add`, `multiply`, `mix`, `clamp`. Six leaf signals:
`sine`, `triangle`, `ramp`, `adsr`, `perlin`, `spatial_noise`, `keyframes`,
`literal` (8). Plus the unary `clamp` wrapper.

The shorthand for signal expressions stays flat-shaped and matches the V3
authoring form exactly. **No translation needed** — the V3 expression shape
is essentially already the V3.1 canonical inner-expression shape, just
without the wrapping `previewLoopback` envelope. This is the cleanest pattern
in the corpus.

### G10. `phase: "all"` (4 demonstrators)
- `complex_diamond_highlight`: mask.dwell, plus dwell-phase mask "all"
- `content_glyph_particles_base_spray`: filter `phase: "all"`
- `complex_cinematic_reveal`: mixed phase declarations
- Implicit in many: omitted phase = all

Crosses rule-of-three. The vocabulary is `enter` | `dwell` | `exit` | `all`,
with omitted equivalent to `all`. Confirmed.

## Updated transition-preset coverage

After round 2, witnesses for each `TransitionPreset`:

| Preset | Witnesses | Status |
|---|---|---|
| `Crossfade` | 0 | No corpus witness; faith-from-schema only |
| `Wipe` | `mask_wipe_corner_out_from_bottom_left`, `digital_rain` (top_to_bottom) | Confirmed |
| `Iris` | `mask_iris`, `center_expand`, `complex_diamond_highlight` | **Promoted** |
| `Push` | 0 | No corpus witness; faith-from-schema only |
| `Dissolve` | `mask_dissolve`, `content_dissolve`, `complex_diamond_highlight` | **Promoted (with mask/content disambiguation)** |
| `Morph` | 0 (content_morph is `morph` content effect, not transition) | No transition witness |
| `Stippled` | 0 | No corpus witness |
| `Braille` | 0 | No corpus witness |

`Crossfade`, `Push`, `Morph`, `Stippled`, `Braille` remain faith-from-schema.
Adding entries to the alias and expansion tables for those presets is correct
per the contract crate, but the canonical track shape can't be derived from
corpus evidence — has to come from descriptor-pack documentation or the
schema's `TransitionTrack` enum directly.

Plus `fade` (visibility.fade or opacity.fade — V3 form is `style.fade_in`/
`fade_out`, V3.1 canonical is `opacity.fade`): 4 witnesses (`bsod_crash_v3`,
`default_toast`, `style_fade_in_from_canvas`, `madeira_full_scene` text
layers). Promoted.

Plus `blinds` (visibility.blinds — not in TransitionPreset enum but commonly
authored as transition): 3 witnesses (`mask_blinds`, `bottom_blinds_collapse`,
`complex_diamond_highlight`). May need addition to the preset enum or
treatment as an alias for one of the 8.

## Updated count summary

After round 1 + round 2:

- **71 recipes read** (39 round 1 + 32 round 2)
- **18 paired files written** (10 round 1 + 8 round 2)
- **53 recipes confirm existing patterns** without new pairs
- **6 new patterns** surfaced (G1–G6, G8) — most single-witness, deferred to canonical
- **2 patterns** crossed rule-of-three thresholds (iris, dissolve)
- **4 signal operators + 8 leaf signal types** now witnessed

## Recipes confirmed by round 2 (no new pair, cited as evidence)

| Recipe | Patterns it confirms or extends |
|---|---|
| `glitch_shift_window_bindable` | C3, D1 (multiple bindings) |
| `complex_diamond_highlight` | E1 (iris/dissolve), C2 role/border/content scopes, multi-effect parallel |
| `content_glyph_cascade_braille` | C1 (glyph_cascade type) |
| `content_marquee` | C1, B6 |
| `ease_bounce_out`, `ease_elastic_out` | A4 (named easing closed-list expansion) |
| `integer_binding_demo`, `bool_binding_truthy_loopback` | B7 (event-driven dwell variants) |
| `loopback_rigid_shake_severity_ramp` | G6 (multi-binding) |
| `motion_carrier_orbit_helix` (already in round 1) | E3 |
| `toast_shadow_diagonal_edge_crossing` | G8 (grade composite mode) |
| `sampler_crt`, `sampler_sinewave` | C1 multi-phase sampler |
| `scene_layer_role_scope_pipeline`, `scene_layer_nested_parallel_sequences` | C2 role scope, layer-local nested trees |
| `shader_terminal_fire_glyph_v3` | Shader-as-sampler-input pattern (single-witness, defer) |
| `shader_sub_cell_shake` | C1 shader primitive |
| `shadow_braille_density_texture` | G7 |
| `bottom_blinds_collapse` | G1, E1 (blinds-as-transition) |
| `coin_get` | G2 (bounce path), G5 (spatial_shader on style) |
| `circuit_trace` | G2 (rectilinear), G3 (frame_permille) |
| `subcell_shapes/{fractional_inset_rect, quadrant_corner_sculpt}` | F (single-witness multi-layer subcell pattern, three witnesses now — promote weakly) |

The subcell-shape pattern (multi-layer label + subcell_shape_atlas + shape_text)
now has three witnesses (round 1's `braille_rounded_rect` and `subcell_frame`
plus round 2's pair). Borderline rule-of-three; still feels structural enough
that promoting to a shorthand is premature without a fourth witness.
