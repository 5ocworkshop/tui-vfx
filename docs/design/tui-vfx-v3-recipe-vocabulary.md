<!-- <FILE>docs/design/tui-vfx-v3-recipe-vocabulary.md</FILE> - <DESC>Canonical recipe vocabulary for V3 authoring. Consolidates direction/origin/shape/phase/basis terminology so schema docs, examples, fixtures, and runtime implementations use one shared language.</DESC> -->
<!-- <VERS>VERSION: 0.4.0</VERS> -->
<!-- <WCTX>Audit recommendation 1.4 — add the canonical V3 scope vocabulary section so authors learn the full scope kind list, including the new `modulo` authoring scope (axis + modulus + remainder) that exposes the engine's long-existing StyleRegion::Modulo to recipe writers.</WCTX> -->
<!-- <CLOG>0.4.0: MINOR — add new section "Canonical V3 scope vocabulary" that catalogues all V3 scope kinds, documents the new `modulo` scope (axis: horizontal | vertical, plus integer modulus and remainder), explains the axis-naming rule (horizontal scans rows top→bottom, vertical scans columns left→right), and lists which scope kinds support runtime bindings today vs. literal-only.
0.3.0: MINOR — wipe-direction vocabulary section grew from 12 canonical variants to 20 (cardinal + diagonal + centre/edge + corner-out + corner-in). Added the diagonal-vs-corner-arc explanatory note. Documented that the same vocabulary is shared by the Wipe mask, the RevealWipe shader, and the V3 grouped reveal family at the engine level via one canonical WipeDirection in tui-vfx-geometry.
0.2.9: add sibling authoring-doc routing for ingredients, schema, procedural sources, and tooling.</CLOG> -->

# tui-vfx V3 recipe vocabulary

This document defines the **canonical vocabulary** for V3 recipe authoring and
for the docs/examples that teach it.

Its purpose is to stop vocabulary drift before it spreads across:

- schema comments
- authoring guides
- debug recipes
- validators
- runtime implementations
- future AI authoring helpers

## 1. Ground rule

When multiple spellings or mental models exist, we choose **one canonical term**
for docs and examples.

Compatibility aliases may still exist in serde or migration code, but:

> new docs, examples, and recipe-writing guidance should teach the canonical
> term, not the alias set.

## 2. Recipe ingredient vocabulary

Authoring-doc route:

- learn the full recipe ladder in `../../../tui-vfx-recipes/docs/scene/AUTHORING_GUIDE.md`
- check exact fields in `../../../tui-vfx-recipes/docs/schema/SCHEMA_REFERENCE.md`
  plus generated `../../../tui-vfx-recipes/docs/generated/V3_API.md`
- use `../../../tui-vfx-recipes/docs/scene/PROCEDURAL_SOURCES.md` for stock
  procedural ingredients
- use `../../../tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md` for
  validator/probe/trace/player commands

Use **recipe ingredients** as the canonical author-facing term for the
capabilities authors combine to create a recipe.

Recipe ingredients include:

- content transforms
- motion routes and dynamics
- easings
- masks
- shaders
- filters
- samplers
- procedural sources
- progress and timer treatments
- runtime bindings and tokens
- assets
- border/envelope and outer-shape treatments
- icons, glyphs, symbols, emoji, and optional Nerd Font glyphs
- I/O producer/consumer links
- host-edge affordances when an adapter supports them

Why this term:

- **ingredient** implies composition, taste, restraint, and pairing
- it avoids overloading **tools**, which should mean development utilities such
  as validators, probes, trace CLIs, and preview players
- it avoids the vague **capabilities** when speaking to recipe authors
- it helps authors think in combinations: ingredients create micro-experiences,
  not piles of effects

Icon/glyph guidance:

- assume the go-forward platform is a modern 2026+ true-color developer
  terminal with good Unicode and emoji support
- use Unicode symbols and emoji when they improve recognition, hierarchy, or
  delight
- treat Nerd Font glyphs as optional profile/host variants unless the host
  explicitly guarantees support
- use icons to improve recognition, hierarchy, or delight; do not use them as
  decorative stickers
- when multiple recipes in a set use icons, check that the icon language is
  consistent enough to feel related but distinct enough to serve each event


Border/envelope/outer-shape guidance:

- treat borders as recipe ingredients, not default chrome: built-in ratatui
  borders, custom single-glyph overrides, fully authored ASCII/Unicode borders,
  and border color/weight/timing can all change the nudge
- use fractional-cell, block, braille, and other dense glyphs to suggest
  non-rectangular silhouettes when the design earns it: parallelograms, angled
  banners, notched cards, pop-art bursts, badges, or softened corners
- the goal is not novelty for its own sake; the shape should improve tone,
  attention, or recognition while preserving readability
- btop-style craft is the reference spirit: many small, careful visual decisions
  can convey dense information and personality inside a terminal grid
- think like an artist as well as a terminal developer; if a fractional-cell
  border idea needs new rendering support, record it as a future-ingredient
  wishlist item instead of discarding it

Similarity rule:

- reject or rework recipe concepts that are materially too similar in
  ingredients, motion, metaphor, layout, or attention behavior
- consistency is good when intentional; accidental sameness is a design smell
- if two recipes are subtle variants of one idea, document why both should exist
  or remove one

Recipe family vocabulary:

- use **recipe family** when one design format intentionally supports several
  variants, such as info/warning/error states that share a structure but vary
  border color, icon, texture, position, or animation intensity
- a recipe family can be the right answer; do not force every state into a
  separate concept
- if the assignment asks for a limited set of distinct recipes, a family with
  variants usually counts as one concept unless the user explicitly asks for
  each variant as its own deliverable

Whimsy vocabulary:

- use **selective whimsy** for purposeful playful details inside otherwise
  professional recipes
- use **after-hours creative lab** for free-form recipes whose goal is artistic
  or playful exploration rather than utility or theme fit; this is a required
  final creative lane in quality-oriented authoring rounds
- whimsical recipes still need validation; freedom is not a reason to ship
  broken recipes
- use **future-ingredient wishlist** for creative ideas that require a new
  primitive, math capability, shader/filter/sampler, procedural source, path, or
  content-animation feature before they can be expressed well

Design-compass vocabulary:

- use **60/30/10 inspiration trio** for the early, weighted design direction a
  recipe author derives from theme research: primary inspiration, secondary
  inspiration, and accent inspiration
- the trio is a provisional creative hypothesis, not a coding-style requirement
  lock; authors should refine it when ingredient discovery or combination work
  reveals a stronger direction
- use **updated 60/30/10** when documenting how the design compass changed and
  why the revision improves recipe decisions
- the point is to make the creative direction visible, discussable, and
  improvable before JSON is written

Rules:

- use **recipe ingredients** or **ingredients** in authoring docs, AI briefs,
  and theme-recipe prompts
- use **development tools** for validator/probe/trace/preview CLIs
- use **capability** when discussing library surface area, planning coverage, or
  engineering inventory
- avoid **toolbox**, **toolchest**, or generic **tools** when the intended
  meaning is recipe ingredients

## 3. Capability promotion ladder vocabulary

Use the accepted V3 promotion ladder when deciding how public an ingredient
name should become:

- **base primitive** — schema/runtime concept with direct authoring support
- **variant** — named parameterization inside a base primitive
- **earned-name composition** — stable combination promoted because authors use
  it often enough to deserve a name
- **factory-internal convention** — repeated pattern held inside a factory or
  recipe family while it proves demand
- **deferred** — good idea, but not a public V3 ingredient yet

Promotion is sticky. Do not mint a public ingredient name because one recipe
looks cute in isolation. Use the rule-of-three review trigger from the V3
capability governance decision before promoting factory-internal conventions.

## 4. Phase vocabulary

Canonical author-facing phase words:

- `enter`
- `dwell`
- `exit`
- `all`

Canonical runtime-state words:

- `Entering`
- `Dwelling`
- `Exiting`
- `Finished`

Rules:

- use **`enter` / `dwell` / `exit`** in recipe JSON
- use **`Entering` / `Dwelling` / `Exiting`** only for runtime/state-machine
  discussion
- do not invent parallel synonyms like `arrive`, `present`, `leave` in recipe
  docs unless a future steering decision explicitly renames them

## 5. Enter/exit relationship vocabulary

When discussing how exit relates to enter, use these words:

- **same direction**
  - enter and exit use the same directional payload
- **opposite direction**
  - exit uses the inverse directional payload
- **complementary geometry**
  - exit uses the natural complement of the enter geometry
  - example: `horizontal_center_out` → `horizontal_edges_in`

Important:

- these are currently **authoring guidance terms**, not first-class schema
  fields
- the schema already supports them by giving enter and exit independent payloads

## 6. Reveal-geometry vocabulary

Use these canonical axis names for reveal geometry:

- **direction**
  - which way a reveal/sweep progresses
- **origin**
  - the point a radial/path/cellular effect grows from
- **shape**
  - the aperture/geometry class
- **orientation**
  - whether slats/bands/segments are horizontal or vertical
- **path**
  - an authored trajectory such as spiral or radial sweep

These terms should stay distinct.

Example:

- `wipe` is primarily a **direction** vocabulary
- `radial` is primarily an **origin** vocabulary
- `iris` is primarily a **shape** vocabulary
- `blinds` is primarily an **orientation** vocabulary
- `path_reveal` is primarily a **path** vocabulary

## 7. Canonical wipe-direction vocabulary

For `wipe`, the canonical direction set is:

### Cardinal
- `left_to_right`
- `right_to_left`
- `top_to_bottom`
- `bottom_to_top`

### Diagonal (Manhattan sweep, slanted-line wavefront)
- `top_left_to_bottom_right`
- `top_right_to_bottom_left`
- `bottom_left_to_top_right`
- `bottom_right_to_top_left`

### Center/edge paired geometry
- `horizontal_center_out`
- `vertical_center_out`
- `horizontal_edges_in`
- `vertical_edges_in`

### Corner-out (Euclidean quadrant arc, expanding from a corner)
- `corner_out_from_top_left`
- `corner_out_from_top_right`
- `corner_out_from_bottom_left`
- `corner_out_from_bottom_right`

Author-friendly aliases that read more naturally and may be used
interchangeably:
- `corner_down_top_left` (= `corner_out_from_top_left`)
- `corner_down_top_right` (= `corner_out_from_top_right`)
- `corner_up_bottom_left` (= `corner_out_from_bottom_left`)
- `corner_up_bottom_right` (= `corner_out_from_bottom_right`)

### Corner-in (Euclidean quadrant arc, collapsing toward a corner)
- `corner_in_to_top_left`
- `corner_in_to_top_right`
- `corner_in_to_bottom_left`
- `corner_in_to_bottom_right`

Compatibility aliases such as:

- `from_left`
- `from_right`
- `from_top`
- `from_bottom`

may remain for compatibility, but they are **not** the preferred teaching
surface.

**Diagonal vs. corner-arc:** the Manhattan-diagonal variants
(`top_left_to_bottom_right` etc.) sweep a straight slanted line from
one corner toward the opposite corner; at progress 0.5 the wavefront
is a single line. The corner-arc variants
(`corner_out_from_top_left` etc.) sweep a Euclidean quadrant arc
rooted at the named corner; at progress 0.5 the wavefront is a
quarter-circle. Both are intentionally preserved as separate
directions because their visual feel is materially different.

The same vocabulary is shared at the engine level (one canonical
`WipeDirection` enum in `tui-vfx-geometry`) by the `Wipe` mask, the
`RevealWipe` shader, and the V3 grouped reveal family — so a
`direction` field interchanges across the mask, shader, and grouped-V3
layers.

## 7b. Canonical V3 scope vocabulary

V3 recipe steps target cells through a `scope` field. The canonical scope
kinds (the strings authored in `"kind": "..."`) are:

### Whole-area / role
- `all` — every cell inside the layer
- `border` — the painted border characters of the widget
- `role` — cells whose `RoleTag` matches `value` (e.g. `text`, `border`,
  `background`, or a custom role name)
- `channel` — a logical paint channel (`foreground`, `background`,
  `glyph`); some payload families honour the channel intent during
  lowering

### Coordinate-anchored
- `cell { x, y }` — a single cell. `x` and `y` are `BindableU16` (raw
  integer or `{"binding": "name"}`)
- `cells { cells: [...] }` — a list of `{x, y}` coordinates
- `cell_run { run: { y, x_start, x_end } }` — a single horizontal run on
  one row
- `cell_runs { runs: [...] }` — a list of horizontal runs
- `rect { x, y, w, h }` — an axis-aligned rectangle
- `rect_exclude { x, y, w, h }` — every cell **outside** the given rect
- `outer { margins }` / `inner { margins }` — perimeter band / interior
  area defined by per-edge margins
- `rows { rows: [...] }` — specific row indices
- `row_range { start, end }` — half-open contiguous row range
- `columns { columns: [...] }` — specific column indices
- `column_range { start, end }` — half-open contiguous column range

### Periodic / modular
- `modulo { axis, modulus, remainder }` — every Nth row or column.
  - `axis: "horizontal"` scans **row by row** (one matched row is one
    full-row stripe). `axis: "vertical"` scans **column by column**
    (one matched column is one full-column stripe). The axis name
    describes the direction the rule **iterates**, not the orientation
    of the stripes it draws.
  - `modulus: u16` is the period; `remainder: u16` is the offset within
    the period (`0` means rows/cols 0, N, 2N, …; `1` shifts by one).
  - Both `modulus` and `remainder` accept a literal integer or a
    `{"binding": "name"}` form; when both are literals the recipe
    compiler folds the scope into the compact `StaticModulo` shape, and
    when either is a binding the scope is resolved per-frame against
    `ShaderRuntimeParams`.
  - Engine-level: this lowers to
    `StyleRegion::Modulo { axis: ModuloAxis::Horizontal | Vertical,
    modulus, remainder }`.

### Content / glyph
- `content { value }` — cells whose source character equals `value`
- `glyph_matches { pattern }` — cells whose source character matches the
  glob/charset pattern

### Predicate / boolean
- `predicate { ref }` — invokes a named predicate registered with the
  recipe
- `and { children: [...] }`, `or { children: [...] }`, `not { child }` —
  boolean composition over child scopes

### Runtime-binding support today

The `BindableU16` form (`{"binding": "name"}`) is currently honoured on
the coordinate fields of `cell`, `cells`, `cell_run`, `cell_runs`,
`rect`, `rect_exclude`, and the literal margin fields of `outer` /
`inner`. `row_range`, `column_range`, and `modulo` accept the same
JSON binding shape at the V3 authoring layer; the recipe compiler keeps
literal forms compact (`StaticRowRange` / `StaticColumnRange` /
`StaticModulo`) and falls back to dynamic resolution when a binding is
present. End-to-end binding plumbing for the range/modulo families
remains an in-progress universalisation effort tracked under audit
recommendation 1.5.

### Authoring guidance

- Use `modulo` for "every Nth row" or "every Nth column" patterns
  (CRT scanlines, ledger paper, alternating column highlights). Do not
  reach for `rows: [0, 3, 6, 9, ...]` to express the same idea — the
  modulo form is shorter, expresses intent more clearly, and survives
  resize.
- Use `row_range` / `column_range` for one contiguous band; use `rows`
  / `columns` only when the index list is irregular and short.
- Use `rect_exclude` instead of building a four-rect `or` to mask out
  a centre region; it is one allocation and one comparison per cell.

## 8. Origin vocabulary

For origin-driven families, use:

- `center`
- `top_left`
- `top_right`
- `bottom_left`
- `bottom_right`
- `custom { x, y }`

Use **origin** when the payload is fundamentally about where the effect grows
from.

Use **source** only when the effect is modeling a light/emission/source concept
rather than a general reveal origin.

## 8. Shape vocabulary

For aperture/reveal shapes, canonical terms are:

- `circle`
- `diamond`
- `box`

Prefer `box` over ad-hoc synonyms like `square` in the primitive vocabulary
when the implementation really means axis-aligned rectangular aperture.

## 9. Direction vocabularies by subsystem

Not every subsystem should reuse the same direction words.

### Reveal geometry
Use:
- `left_to_right`
- `right_to_left`
- `center_out`
- `edges_in`
- etc.

### Traveling / band / sweep progression
Use:
- `forward`
- `reverse`
- `ping_pong`

### Wave / field orientation
Use:
- `horizontal`
- `vertical`
- `radial`
- `diagonal`

### Motion/offscreen placement
Use:
- `from_left`
- `from_right`
- `from_top`
- `from_bottom`
- corner forms where needed

That split should remain intentional. Do not collapse all direction terms into
one giant enum vocabulary.

## 10. Spatial-basis vocabulary

This is now a load-bearing distinction.

### Cell-space / cell-lattice basis

Use when geometry is defined over:

- the sampled cell lattice
- `0 .. width-1`
- `0 .. height-1`

Canonical terms:

- **cell-space**
- **cell-lattice**

Examples:

- `sample_norm_x`
- `sample_centered_x`
- `sample_radius`
- `CellDistanceSignal::radius_from`

### Surface-space / frame-space basis

Use when geometry is defined over:

- the continuous frame/surface
- optical falloff
- aperture/light-field semantics

Canonical terms:

- **surface-space**
- **frame-space**

Examples:

- `sample_surface_centered_x`
- `sample_surface_radius`
- `SurfaceDistanceSignal::radius_from`
- `SurfaceAngleSignal::angle_from`

Rule:

> when a consumer needs a genuinely different geometric model, add a new
> explicit basis instead of silently mutating the semantics of an existing leaf.

## 11. Debug-recipe naming vocabulary

For debug-recipe body text:

- first line = human-readable effect/test name
  - example: `Mask: Iris Effect`
- second line = concise behavioral cue
  - example: `Circle open /\nDiamond close`

The cue should be:

- concise
- useful
- free of filler like `Watch`

## 12. Practical authoring rule

When writing or reviewing recipes, ask:

1. Am I using the canonical vocabulary for this subsystem?
2. Am I accidentally mixing two different geometric bases?
3. Am I using a compatibility alias where the canonical spelling should be
   taught instead?
4. Does the debug/reference fixture explain the intended behavior clearly using
   the same vocabulary the schema/docs use?

If the answer to any of those is "no", fix the vocabulary before adding more
examples or abstractions on top.

## 13. Visual reference — rect vocabulary

Use this diagram when discussing rect-local geometry:

```text
top edge
┌──────────────────────────────┐
│ top_left       top_center    │ top_right
│                              │
│ left edge      center        │ right edge
│               (cx, cy)       │
│                              │
│ bottom_left   bottom_center  │ bottom_right
└──────────────────────────────┘
bottom edge
```

Canonical rect terms:

- **top edge / bottom edge / left edge / right edge**
- **top_left / top_center / top_right**
- **bottom_left / bottom_center / bottom_right**
- **center**
- **center row / center column**

For wipe-style vocabulary:

- `horizontal_center_out`
  - starts at the **center column**
  - expands toward the **left edge** and **right edge**
- `horizontal_edges_in`
  - starts at the **left/right edges**
  - collapses toward the **center column**
- `vertical_center_out`
  - starts at the **center row**
  - expands toward the **top edge** and **bottom edge**
- `vertical_edges_in`
  - starts at the **top/bottom edges**
  - collapses toward the **center row**

For radial/iris vocabulary:

- **origin** is a point like `center`, `top_left`, or `custom{x,y}`
- **shape** is the aperture geometry like `circle`, `diamond`, `box`

## 14. Visual reference — scene vocabulary

Use this diagram when discussing scene-layer placement and composition:

```text
frame / viewport
┌────────────────────────────────────────────┐
│                                            │
│  scene canvas                              │
│  ┌──────────────────────────────────────┐  │
│  │ layer A rect                         │  │
│  │ ┌──────────────────────────────────┐ │  │
│  │ │ content/source for layer A       │ │  │
│  │ └──────────────────────────────────┘ │  │
│  └──────────────────────────────────────┘  │
│                                            │
│                  ┌──────────────────────┐  │
│                  │ layer B rect         │  │
│                  │ sibling/overlay      │  │
│                  └──────────────────────┘  │
│                                            │
└────────────────────────────────────────────┘
```

Canonical scene terms:

- **frame** / **viewport**
  - the available render area
- **scene canvas**
  - the shared composition space inside the frame
- **layer**
  - one placed content/source block in the scene
- **layer rect**
  - the placed bounds of one layer
- **source**
  - what the layer renders from (`text`, `image`, `procedural`, etc.)
- **placement**
  - how the layer rect is positioned
- **surface**
  - the visual/structural surface treatment of the layer
- **overflow**
  - what happens when content exceeds the layer rect
- **visibility**
  - when/how the layer participates in rendering
- **sibling**
  - another layer used as a relative placement or follow reference

When discussing relational placement:

- **relative_to**
  - the sibling layer being referenced
- **target_anchor**
  - the anchor on that target/sibling layer
- **follow**
  - how the placed layer tracks the sibling over time
- **phase_offset_ms**
  - timing offset for choreography, not geometry

These diagrams are intentionally simple. They exist to keep our words grounded
in one shared visual model while we continue normalizing schema, fixtures, and
runtime behavior.

<!-- <FILE>docs/design/tui-vfx-v3-recipe-vocabulary.md</FILE> - <DESC>Canonical recipe vocabulary for V3 authoring</DESC> -->
<!-- <VERS>END OF VERSION: 0.4.0</VERS> -->
