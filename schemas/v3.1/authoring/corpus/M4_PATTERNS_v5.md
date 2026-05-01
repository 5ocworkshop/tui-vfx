# M4 — Round 5 patterns (40 production-design recipes) — saturation analysis

39 of 40 recipes read (one filename in my list didn't match a real file). Three
paired files written. **Round 5's primary finding is that the corpus is
approaching pattern saturation.**

## Saturation analysis

Across rounds 1–4 we surfaced new architectural patterns each round:
- Round 2: pattern A–G (wrapper-stripping, default-omission, effect-attachment, bindings, transition presets, single-witness)
- Round 3: H1 `extends`, H2 `glyph_timeline`, H3 spatial_shader-on-style, H4 border.title, H5 linear_gradient, H6 rainbow
- Round 4: J1 `trace_path`, J2 composed motion + dynamics, J3 dwell-array, J4 region scoping, J5 combined-mask compose

Round 5 surfaced **vocabulary extensions, not new architectural patterns**:

| Round 5 finding | Type | Witnesses | Architectural novelty |
|---|---|---:|---|
| Typewriter `cursor.wake` (mode/decay/maxCells/gap/curve/tint) | Vocabulary | 2 | No — descriptor input refinement |
| `shader.affordance_wake` (zone/radius/progress/peakIntensity) | Vocabulary | 3 — promoted | No — fits `{shader: "..."}` shape |
| `shader.radial_spiral` (arms/radial_frequency/radial_power) | Vocabulary | 2 | No — fits shader shape |
| `shader.reveal_wipe` (TextOnly-scoped) | Vocabulary | 1 | No |
| `filter.edge_grow` (restEighths/peakEighths/edge/fillColor/progress) | Vocabulary | 1 | No — fits filter shape |
| `filter.sub_pixel_bar` (progress-bound) | Vocabulary | 1 | No |
| `mask.cellular pattern: "hexagonal"` | Vocabulary | 1 | No — third pattern variant after organic/voronoi |
| `motion_path: hover` (amplitude/frequency) | Vocabulary | 1 | No — fits route shape |
| `motion_path: spring` (stiffness/damping) | Vocabulary | 1 | No — eighth route type but same shape |
| `focus_field shape: "ellipse"` (center_x/y, radius_x/y) | Vocabulary | 2 | No — second shape variant |
| `style.italic_window` (start/end) | Vocabulary | 1 | No |
| `style.glitch` extended (italic_start/italic_end) | Vocabulary | 1 | No — input refinement |
| `style.rigid_shake_style` | Vocabulary | 1 | No |
| Diffusion `mode: "warm_drift"` (companion to `breath`) | Vocabulary | 1 | No — second mode variant |
| `border.frame` custom glyphs | Vocabulary | 5 — promoted | No — schema field already exists |
| `extends` (wargames_chess) | — | 6 — confirmed | Already promoted in round 3 |

**Zero architecturally-novel patterns surfaced in round 5.** Every finding is
either:

1. **A new descriptor in an existing family** (a new shader/filter/mask
   variant that fits the `{kind: "...", inputs: {...}}` shape used by every
   other primitive).
2. **A new enum variant of a known polymorphic axis** (`hexagonal` joining
   `organic`/`voronoi` for cellular; `warm_drift` joining `breath` for
   diffusion; `ellipse` joining `rect` for focus_field shape; `box` already
   joined `circle`/`diamond` for iris).
3. **A vocabulary refinement** of an already-witnessed primitive (typewriter
   gaining a `cursor.wake` sub-tree; glitch gaining `italic_start`/`italic_end`
   parameters).
4. **Confirmation** of an already-promoted pattern (extends, dwell-array,
   border.frame, region scoping).

This is the saturation signature. The shorthand mechanism — `{shader: "..."}`,
`{filter: "..."}`, `{mask: "..."}`, `{transitions: { preset: "..." }}`, alias
table for spelling normalization, expansion table for preset → tracks —
**absorbs every round-5 finding without a new shape**.

## What this implies for next steps

The corpus is now sufficient to design the meta-schemas for the alias and
expansion tables. Specifically:

- **Alias table format** can be drafted today. The patterns it needs to encode
  are known: spelling → canonical name (`iris-reveal` → `Iris`), with optional
  parameter overrides (`wipe-ltr` → `Wipe(direction: leftToRight)`).
- **Expansion table format** can be drafted today. Preset → ordered list of
  canonical tracks/nodes. Witnessed for 11 transition presets and most filter
  primitives.
- **Vocabulary additions** (more shaders, more filters, more route types)
  don't need design — they extend the existing tables row by row.

What's still genuinely uncertain (the 15 open design questions) is *not*
about corpus coverage — it's about decisions that affect schema shape:

- Q8 (`extends` template support — schema-side feature or canonicalization-only?)
- Q12 (`Polyline` as shared geometry primitive — or trace_path-private?)
- Q13 (motion dynamics — first-class or composed-private?)
- Q1 (which card-shorthand fields are blessed — needs descriptor-pack audit, not corpus survey)

These can't be answered by reading more recipes. They need design calls.

## Updated count summary

After all 5 rounds:

- **174 recipes read** (39 + 32 + 32 + 32 + 39)
- **37 paired files written** (10 + 8 + 8 + 8 + 3)
- **137 recipes confirm patterns** without new pairs
- **Round 5 surfaced 0 new architectural patterns** — saturation confirmed
- **15 open design questions** remain (unchanged from round 4 — no new decisions raised by round 5)

## Filter and shader catalog — final

After 5 rounds, the catalog is well-bounded. Numbers below count witnesses
across all 174 recipes.

**Core shaders** (rule-of-three crossed, well-witnessed):
glisten_band (12+), focused_row_gradient (7), linear_gradient (6), rainbow
(5), pulse (5), pulse_wave (4), concealed_light (4), border_sweep (4),
edge_sheen (4), affordance_wake (3 — newly promoted in round 5), highlighter
(3), diffusion (3), colored_overlay (3), trace_path (2 — specialized),
focus_field (3 — promoted with ellipse + rect variants in round 5).

**Specialized/single-witness shaders** (push to canonical):
bevel, ambient_occlusion, glow, radar, chromatic_edge, wayfinding_node,
stochastic_sparkle, terminal_fire, sub_cell_shake, reflect, edge_sheen,
barber_pole, radial_spiral, reveal_wipe, glitch_lines.

**Core filters**:
tint (8), vignette (8), crt (7), dim (6), invert (4), crt_jitter (4),
fault_line (4), greyscale (3), braille_dust (4), subcell_light (4),
underline_wipe (4), pill_button (3), glyph_style (2 — specialized), rigid_shake
(3 — promoted), pattern_fill (1), edge_grow (1), sub_pixel_bar (1),
bracket_emphasis (1), kitt_scanner (1), glisten_sweep (1), scalar_field_glyph
(1).

**Core masks** (all six closed-vocabulary transition presets, plus extras):
wipe (15+ variants), iris (6 — three shapes: circle, diamond, box), dissolve
(6), diamond (5), blinds (5), cellular (5 — three patterns: organic, voronoi,
hexagonal), checkers (3), noise_dither (3), path_reveal (3), radial (3).

**Routes**:
linear, helix/carrierOrbit, infinity/figureEight, bounce, rectilinear,
spiral, arc, composed (with pendulum dynamic), spring (round 5), hover (round
5). **Ten route types**.

## Recommendation

Move to M5 with the corpus we have. Round 6 would add more vocabulary entries
to the catalog but no new patterns to design against. The remaining open
questions (8, 12, 13, 1) need design calls, not more corpus.

If a round 6 is wanted anyway, target it specifically at:
- The 5 missing transition presets (Crossfade, Push, Morph-as-transition,
  Stippled, Braille) — would likely require *writing* new recipes rather than
  reading existing ones, since the corpus has no witnesses today.
- A 4–5 recipe sweep through any remaining unexplored directories
  (general /recipes/ has ~30 untouched files, none likely surprising).

Both would confirm what we already see: the patterns are stable.
