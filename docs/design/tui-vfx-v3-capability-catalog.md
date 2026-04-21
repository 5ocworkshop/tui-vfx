<!-- <FILE>docs/design/tui-vfx-v3-capability-catalog.md</FILE> - <DESC>Live capability catalog for V3. Classifies families into primitives, composed primitives, wrappers, hybrid templates, and policy variants, and records canonical payload direction and implementation notes as the leaf catalog is hardened.</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>Execution artifact for the capability-catalog phase. Extended through the second batch, resolving the highest-value filter-side clusters that the audit identified as the next implementation hinge point.</WCTX> -->
<!-- <CLOG>0.2.0: resolve batch 2 (CC-07..CC-12): indicator/progress emphasis, traveling-band/sweep, pattern/procedural texture filters, motion-treatment filters, field-rendering wrappers, and the vignette family.
0.1.0: initial catalog and tracker. Seeds the first six resolved families from the existing audit conclusions: reveal geometry, segmented/tiled visibility, procedural breakup masks, wave displacement, fracture displacement, and style fades.</CLOG> -->

# tui-vfx V3 capability catalog

This document is the live implementation-side catalog that sits between:

- the **schema**
- and the eventual **runtime implementation**

Its purpose is to make the leaf catalog explicit before large-scale migration or execution work begins.

---

## 1. Status tracker

| ID | Family cluster | Status | Notes |
|---|---|---|---|
| CC-01 | Reveal geometry | RESOLVED | Primitive subtree |
| CC-02 | Segmented / tiled visibility | RESOLVED | Primitive subtree |
| CC-03 | Procedural breakup / visibility fields | RESOLVED | Primitive subtree |
| CC-04 | Wave displacement | RESOLVED | Primitive subtree |
| CC-05 | Fracture / segmented displacement | RESOLVED | Primitive subtree |
| CC-06 | Style fades | RESOLVED | Primitive subtree with policy variants |
| CC-07 | Indicator / progress emphasis | RESOLVED | Render-mode subtree |
| CC-08 | Traveling-band / sweep | RESOLVED | Shared sweep substrate |
| CC-09 | Pattern / procedural texture filters | RESOLVED | Pattern + procedural subtrees |
| CC-10 | Motion-treatment filters | RESOLVED | Distinct motion-treatment subtree |
| CC-11 | Field-rendering wrappers | RESOLVED | Hybrid wrapper category |
| CC-12 | Vignette family | RESOLVED | Falloff-treatment subtree |
| CC-13 | Style dwell modulation | OPEN | later |
| CC-14 | Typography-window style effects | OPEN | later |
| CC-15 | Typewriter + cursor subtree | OPEN | later |
| CC-16 | Split-flap renderer tree | OPEN | later |
| CC-17 | Rule-engine families | OPEN | later |
| CC-18 | Cross-lane paired capabilities | OPEN | later |
| CC-19 | Celebratory particle generators | OPEN | future capability |

---

## 2. Resolution template

Each family entry records:

- lane
- classification
- canonical name
- canonical payload
- collapsed source families
- implementation notes
- rationale

---

## 3. Resolved families — batch 1

### CC-01 — Reveal geometry

- **lane:** mask
- **classification:** primitive subtree
- **canonical subtree:** `reveal_geometry`
- **collapsed source families:**
  - `wipe`
  - `radial`
  - `iris`
  - `diamond`
  - `path_reveal`
- **shared payload axes:**
  - direction
  - origin
  - shape
  - soft_edge / hardness
  - path
- **recommended implementation stance:**
  - implement as one deeper reveal-geometry capability family
  - keep current family names as authored modes / payload variants where useful
  - do not create a new top-level step kind
- **rationale:**
  - these families differ more by geometry/origin/path policy than by independent lane semantics

### CC-02 — Segmented / tiled visibility

- **lane:** mask
- **classification:** primitive subtree
- **canonical subtree:** `segmented_visibility`
- **collapsed source families:**
  - `blinds`
  - `checkers`
- **shared payload axes:**
  - orientation / tiling mode
  - segment count / cell size
- **recommended implementation stance:**
  - treat as a sibling subtree to reveal geometry and procedural breakup
  - not just a weird corner of reveal geometry
- **rationale:**
  - these are visibility-structure policies, not purely directional reveals and not purely seeded breakup fields

### CC-03 — Procedural breakup / visibility fields

- **lane:** mask
- **classification:** primitive subtree
- **canonical subtree:** `visibility_field`
- **collapsed source families:**
  - `cellular`
  - `dissolve`
  - `noise_dither`
  - `materialize` (shared substrate, though materialize itself becomes hybrid)
- **shared payload axes:**
  - seed
  - matrix / pattern
  - chunk / cell size
  - noise
  - breakup policy
- **recommended implementation stance:**
  - implement as a distinct procedural field family
  - allow hybrid templates to compose it with reveal geometry or style fades
- **rationale:**
  - these families are structurally different from reveal geometry and should not be forced into the same implementation category

### CC-04 — Wave displacement

- **lane:** sampler
- **classification:** primitive subtree
- **canonical subtree:** `wave_displacement`
- **collapsed source families:**
  - `ripple`
  - `sine_wave`
- **shared payload axes:**
  - amplitude
  - wavelength / frequency
  - speed
  - axis / center / phase
- **recommended implementation stance:**
  - one implementation substrate with family-specific policy/render semantics on top
- **rationale:**
  - these are clearly different views of one wave-displacement space

### CC-05 — Fracture / segmented displacement

- **lane:** sampler
- **classification:** primitive subtree
- **canonical subtree:** `fracture_displacement`
- **collapsed source families:**
  - `fault_line`
  - `shredder`
- **shared payload axes:**
  - seed
  - intensity
  - split bias
  - stripe / lane structure
  - asymmetric motion policy
- **recommended implementation stance:**
  - sibling of wave displacement, not part of it
- **rationale:**
  - these families are about segmented/fracture movement rather than periodic wave displacement

### CC-06 — Style fades

- **lane:** style_effect
- **classification:** primitive subtree with policy variants
- **canonical subtree:** `style_fade`
- **collapsed source families:**
  - `fade_in`
  - `fade_out`
  - `fade_in_from_canvas`
  - `fade_out_to_canvas`
  - `color_fade` (adjacent, but still separate branch under style transforms)
- **shared payload axes:**
  - direction (in / out)
  - source target / destination target
  - apply_to
  - easing
- **recommended implementation stance:**
  - keep fade family as one deeper style subtree
  - canvas-aware variants are target-policy variants, not new top-level families
- **rationale:**
  - the authoring distinction is mostly source/target fade policy, not a different leaf algebra


### CC-07 — Indicator / progress emphasis

- **lane:** filter
- **classification:** primitive subtree with render-mode variants
- **canonical subtree:** `progress_emphasis`
- **collapsed source families:**
  - `dot_indicator`
  - `bracket_emphasis`
  - `underline_wipe`
  - `hover_bar`
  - `sub_pixel_bar`
  - `edge_grow`
- **shared payload axes:**
  - progress / coverage
  - anchor / edge / row offset / margin
  - render mode
  - color policy
  - optional polish
- **recommended implementation stance:**
  - one deeper emphasis substrate with multiple render modes
  - keep family names as authored modes/presets where discoverability helps
  - runtime bindings on progress remain orthogonal to render mode
- **rationale:**
  - the families differ mainly in how progress is rendered (symbolic, underline, edge-growth, eighths, block glyphs), not in the deeper authored concept

### CC-08 — Traveling-band / sweep

- **lane:** filter + shader
- **classification:** primitive subtree with lane-specific wrappers
- **canonical subtree:** `traveling_band`
- **collapsed source families:**
  - filters: `glisten_sweep`, `kitt_scanner`, `shade_scanner`
  - shaders: `border_sweep`, `glisten_band`, `reflect` (partial sibling)
- **shared payload axes:**
  - band width / length
  - speed / direction / ping-pong policy
  - boost / blend / dim policy
  - separator/background policy
  - progress control
- **recommended implementation stance:**
  - one shared sweep substrate at the conceptual level
  - lane-specific wrappers for filter-side treatment vs shader-side coloration
  - same runtime-binding and loop semantics reused across the family
- **rationale:**
  - the audit repeatedly showed these are variations on one traveling-band idea with different visual treatment policies

### CC-09 — Pattern / procedural texture filters

- **lane:** filter
- **classification:** split into two sibling subtrees
- **canonical subtree A:** `pattern_treatment`
- **canonical subtree B:** `procedural_texture`
- **collapsed source families:**
  - pattern treatment:
    - `pattern_fill`
    - `interlace_curtain`
  - procedural texture:
    - `braille_dust`
    - `charset_noise`
    - `matrix_rain`
- **shared payload axes:**
  - Pattern treatment:
    - pattern family
    - spacing / density
    - glyph or row treatment
    - empty-only / affect policy
  - Procedural texture:
    - density
    - seed
    - rate / hz
    - drift / churn / trail policy
    - glyph set / preset
- **recommended implementation stance:**
  - keep pattern-treatment and procedural-texture as siblings, not one merged family
  - rule-driven filters remain separate (see CC-17)
- **rationale:**
  - the audit showed two distinct computational models: deterministic pattern application versus time-varying generated texture fields

### CC-10 — Motion-treatment filters

- **lane:** filter
- **classification:** primitive subtree
- **canonical subtree:** `motion_treatment`
- **collapsed source families:**
  - `motion_blur`
  - `rigid_shake`
  - `sub_cell_shake` (filter variant)
- **shared payload axes:**
  - direction / oscillation policy
  - amplitude / trail length
  - decay / damping / pause semantics
  - loop timing
  - geometry assumptions (margins, inner width, edge-only behavior)
- **recommended implementation stance:**
  - shared top-level subtree, but do not over-collapse the actual payloads
  - `rigid_shake` remains a rich motion family with stronger runtime-control support than the simpler directional blur
- **rationale:**
  - these filters all operate as post-render motion treatments, but their internal computational models are not identical enough to flatten into one generic payload

### CC-11 — Field-rendering wrappers

- **lane:** filter (with upstream shader/signal sources)
- **classification:** wrapper / hybrid category
- **canonical subtree:** `field_renderer`
- **collapsed source families:**
  - `subcell_light_background_braille`
  - `subcell_light_foreground_horizontal`
  - `subcell_light_temporal_braille`
- **shared payload axes:**
  - render mode
  - sample source
  - threshold
  - dither policy
  - blank-cell policy
  - upstream source field contract
- **recommended implementation stance:**
  - treat as wrappers that interpret upstream fields rather than as standalone leaf filters
  - allow them to compose naturally with shader-defined source fields and future procedural field sources
- **rationale:**
  - these files are some of the clearest examples in the corpus that a family can be best understood as a renderer over another family’s output, not as an independent primitive leaf

### CC-12 — Vignette family

- **lane:** filter
- **classification:** primitive subtree with policy variants
- **canonical subtree:** `falloff_treatment`
- **collapsed source families:**
  - `vignette`
  - `vignette_dithered`
  - `vignette_side_pair`
  - `vignette_temporal_soften`
- **shared payload axes:**
  - strength
  - radius
  - side/orientation policy
  - spatial dither policy
  - temporal softening policy
- **recommended implementation stance:**
  - one richer falloff-treatment family rather than separate top-level vignette names
  - preserve directional and temporal policies as explicit payload axes
- **rationale:**
  - the audit showed vignette is not one single leaf effect but a family of edge-falloff policies

---

## 4. Immediate next batch

The next 4–6 families to resolve should be:

1. style dwell modulation
2. typography-window style effects
3. typewriter + cursor subtree
4. split-flap renderer tree
5. rule-engine families
6. cross-lane paired capabilities

These are the highest-value remaining style/content/higher-order clusters.

---

## 5. Execution rule

Do not widen the schema unless a family review shows that the current tree genuinely cannot host the correct abstraction.

Default posture:
- collapse into a deeper substrate if the computational and authoring model are genuinely shared
- preserve as distinct only if the semantics are truly independent

<!-- <FILE>docs/design/tui-vfx-v3-capability-catalog.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
