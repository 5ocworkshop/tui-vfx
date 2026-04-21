<!-- <FILE>docs/design/tui-vfx-v3-capability-catalog.md</FILE> - <DESC>Live capability catalog for V3. Classifies families into primitives, composed primitives, wrappers, hybrid templates, and policy variants, and records canonical payload direction and implementation notes as the leaf catalog is hardened.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Execution artifact for the capability-catalog phase. Starts with the first resolved batch so the phase is truly underway rather than only planned.</WCTX> -->
<!-- <CLOG>0.1.0: initial catalog and tracker. Seeds the first six resolved families from the existing audit conclusions: reveal geometry, segmented/tiled visibility, procedural breakup masks, wave displacement, fracture displacement, and style fades.</CLOG> -->

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
| CC-07 | Indicator / progress emphasis | OPEN | next batch |
| CC-08 | Traveling-band / sweep | OPEN | next batch |
| CC-09 | Pattern / procedural texture filters | OPEN | next batch |
| CC-10 | Motion-treatment filters | OPEN | next batch |
| CC-11 | Field-rendering wrappers | OPEN | next batch |
| CC-12 | Vignette family | OPEN | next batch |
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
n- **classification:** primitive subtree
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

---

## 4. Immediate next batch

The next 4–6 families to resolve should be:

1. indicator / progress emphasis
2. traveling-band / sweep
3. pattern / procedural texture filters
4. motion-treatment filters
5. field-rendering wrappers
6. vignette family

These are the highest-value remaining filter-side clusters.

---

## 5. Execution rule

Do not widen the schema unless a family review shows that the current tree genuinely cannot host the correct abstraction.

Default posture:
- collapse into a deeper substrate if the computational and authoring model are genuinely shared
- preserve as distinct only if the semantics are truly independent

<!-- <FILE>docs/design/tui-vfx-v3-capability-catalog.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
