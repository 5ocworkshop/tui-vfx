<!-- <FILE>docs/design/post-release/braille-dotfield-toolkit-plan.md</FILE> - <DESC>Design plan for a generalized braille-dotfield source/toolkit in V3, using the Madeira flag as the first proving consumer.</DESC> -->
<!-- <VERS>VERSION: 0.6.2</VERS> -->
<!-- <WCTX>Track the braille-dotfield toolkit direction as a post-release strategy note with reusable boundaries, effect families, and follow-on phases after the Madeira proof slice.</WCTX> -->
<!-- <CLOG>0.6.2: move plan into post-release capability specs.</CLOG> -->

# tui-vfx V3 braille-dotfield toolkit plan

## Status

Post-release strategy. The first reusable slice has landed in
`tui-vfx-recipes`: `braille_flag_field` loads validated braille-dotfield rows at
V3 recipe-load time, Madeira selects its base artwork through
`requires_assets.madeira_flag_base`, and
`recipes/debug_recipes/scene/scene_braille_flag_asset_token.json` locks the
asset-token path. A follow-up debug fixture,
`recipes/debug_recipes/scene/scene_braille_flag_runtime_wave.json`, proves the
same asset-backed source can take `wave_speed` from a V3 `requires_bindings`
runtime input without adding Madeira-specific Rust plumbing.

## Purpose

Define a reusable **braille-dotfield** concept for V3 and use it to explain the
nearest-term path to recreate `/usr/projects/madeira-flag` faithfully in recipe
space.

The immediate forcing function is the Madeira flag showcase, but the intent is
broader:

- braille-native scene sources
- subcell procedural content
- shared-field displacement + shading
- host/runtime-controlled procedural parameters such as wave speed
- future dataviz / decorative / atmospheric consumers that want to operate in a
  2×4 dot lattice instead of only at terminal-cell granularity

This plan does **not** assume a full general-purpose rewrite of the shader or
sampler architecture tonight.

It instead separates:

1. the **near-term implementation shape** needed to get the flag right
2. the **generalized toolkit direction** we can grow into once the first
   consumer proves the model

---

## Inputs reviewed

This plan is grounded in:

- `/usr/projects/madeira-flag/src/lib.rs`
- `/usr/projects/madeira-flag/examples/demo.rs`
- `/usr/projects/tui-vfx-recipes/recipes/madeira_flag/madeira_flag.json`
- `/usr/projects/tui-vfx/docs/design/tui-vfx-v3-spatial-field-hint-plan.md`
- `/usr/projects/tui-vfx/docs/design/history/tui-vfx-v3-motion-spec.md`
- `/usr/projects/tui-vfx/docs/design/history/tui-vfx-v3-shadow-spec.md`
- `/usr/projects/tui-vfx/docs/design/history/tui-vfx-v3-vanishing-edge-spec.md`
- `/usr/projects/tui-vfx-recipes/src/scene/procedural/sources/cls_ballistic_fireworks.rs`
- `/usr/projects/gt-design/crates/gtd-components/src/shared/col_braille_pip.rs`
- `/usr/projects/gt-design/crates/gtd-components/src/dataviz/braille_*`
- `/usr/projects/bgraph/src/functions/fnc_render_braille.rs`
- `/usr/projects/bgraph/src/functions/fnc_get_braille_symbols.rs`
- `/usr/projects/bgraph/src/functions/fnc_braille_tables.rs`
- `/usr/projects/rocketsplash/crates/rocketsplash/src/v2/features/image/fnc_image_to_braille.rs`
- `/usr/projects/rocketsplash/crates/rocketsplash/src/v2/features/image/cls_braille_image.rs`
- `/usr/projects/rocketsplash/crates/rocketsplash/src/v2/features/quantize/fnc_map_braille.rs`
- `/usr/projects/rocketsplash/crates/rocketsplash/src/v2/ui/canvas/ui_render_canvas/fnc_render_braille_image.rs`

---

## Executive summary

The Madeira flag is currently modeled too much like an **image layer plus
post-processing**.

That is conceptually wrong.

The source crate's flag is really:

1. a **dot lattice** at braille resolution (2×4 subdots per terminal cell)
2. a **procedurally authored flag pattern** drawn directly into that lattice
3. a **shared wave field** evaluated on that lattice
4. a **displacement step** that moves source-dot lookup through that lattice
5. a **correlated shading step** driven by the same wave field
6. a final **braille emission step** that turns 8 subdots into one character

In other words, the right abstraction is not “image plus effects.”

It is:

> **braille-dotfield source + shared-field consumers + braille emission**

The updated recommendation is:

- treat the braille-dotfield toolkit itself as the current implementation goal
- do **not** plan a throwaway Madeira-only source hack as an intermediate step
- make Madeira the **first proving consumer** of the generalized toolkit
- ensure the generalized implementation already includes:
  - dotfield primitives
  - dotfield transforms
  - overscan-aware rendering
  - shared-field displacement + shading
- host/runtime-controlled procedural parameters such as wave speed
  - scene/recipe integration

In other words:

> we should eat the frog and build the real reusable thing now, then make the
> flag demo prove it.

---

## ANSI block/flow diagrams

### A. Source crate workflow (`/usr/projects/madeira-flag`)

```text
┌──────────────────────────────────────────────────────────────┐
│ logical flag rect in terminal cells                         │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│ derive internal dot lattice                                 │
│ width  = cells_w * 2                                        │
│ height = cells_h * 4                                        │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│ draw static Madeira pattern directly into dot lattice       │
│ - triband                                                    │
│ - red cross pattée                                           │
│ - inner white cross                                          │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│ compute shared wave field in dot space                      │
│ amplitude grows toward right edge                           │
└──────────────────────────────────────────────────────────────┘
                    │                               │
                    │                               │
                    ▼                               ▼
┌──────────────────────────────┐      ┌──────────────────────────────┐
│ displacement consumer        │      │ shading consumer             │
│ src_dot_y = dot_y - wave     │      │ shade = f(wave)             │
└──────────────────────────────┘      └──────────────────────────────┘
                    │                               │
                    └───────────────┬───────────────┘
                                    ▼
┌──────────────────────────────────────────────────────────────┐
│ emit one braille cell                                        │
│ - gather 8 dot bits                                           │
│ - average sampled color                                        │
│ - apply correlated shade                                       │
│ - output braille char + fg color                               │
└──────────────────────────────────────────────────────────────┘
```

### B. Near-term recipe/scene implementation path

```text
SceneLayer(flag)
    │
    ├── source.type = procedural
    │       source_id = braille_flag_field
    │
    ├── params own:
    │       - flag colors
    │       - cross geometry
    │       - wave harmonics / speed
    │       - shading constants
    │       - overscan policy
    │
    └── frame(ctx)
            │
            ├── build or cache 2×4 dot lattice
            ├── draw static flag pattern
            ├── compute shared wave field
            ├── displace through dot lattice
            ├── shade from same field
            ├── emit braille glyphs
            └── paint with transparent bg
```

### C. Long-term generalized toolkit layering

```text
┌──────────────────────────────────────────────────────────────┐
│ Layer 3: recipe-facing sources                               │
│ - braille_flag_field                                          │
│ - future braille banners / dataviz / decorative scenes        │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│ Layer 2: dotfield transforms                                 │
│ - displacement by field                                       │
│ - shading by field                                            │
│ - masking / thresholding                                      │
│ - future erosion / noise / deposition                         │
└──────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────┐
│ Layer 1: braille-dotfield primitives                         │
│ - BrailleDotCanvas                                            │
│ - dot ordering / glyph emission                               │
│ - overscan helpers                                             │
│ - dotfield utilities                                           │
└──────────────────────────────────────────────────────────────┘
```

---

## 1. What the Madeira crate is actually doing

The important point is not “there is a source image.”

There is not.

The crate constructs the flag this way:

```text
terminal flag rect
  -> derive internal 2×4 dot lattice size
  -> draw Madeira flag pattern directly into that lattice
  -> for each output terminal cell:
       for each braille dot:
         evaluate wave at dot-space x/time
         invert displacement through source lattice
         sample source dot color from the lattice
         accumulate braille bits
         accumulate correlated shading
  -> emit one braille char + shaded fg color
```

Three properties are load-bearing:

### A. The source is braille-dot-native

The base representation is already the dotfield that corresponds to braille.
It is not a file-backed raster or a prepacked terminal image.

### B. The right edge moves more than the left

Wave amplitude increases with normalized x.
The left side is mostly anchored; the right side is the moving edge.

### D. The source is recipe-owned dotfield data, not a hidden Rust fallback

The as-built V3 recipe now uses `requires_assets.madeira_flag_base` and
`source.spec.params.asset.path = "{{ madeira_flag_base }}"`. V3 loading resolves
that contract entry, validates the `tui-vfx.braille_flag_asset.v1` file, and
embeds the rows before frame rendering.

That keeps the long-term truthful representation: the source content lives in
recipe-owned data and the reusable source logic applies wave displacement,
shading, overscan, and braille emission. It is no longer Rust-encoded Madeira
artwork, and changing the base visual asset is a recipe-data operation.

The key rule remains:

> the canonical flag content should live in recipe-owned data or source logic,
> not in a hidden Rust-only Madeira fallback path.

---

## 2. What maps cleanly to current V3 concepts

A lot of the overall shape already fits V3 well.

### 2.1 Scene layer

The flag is naturally one `scene.layers[]` entry with:

- placement
- source
- layer-local pipeline
- role tag

No new top-level recipe concept is required for that.

### 2.2 Shared field

The current field/hint work already points in the right direction:

- one upstream field producer
- multiple downstream consumers
- displacement consumer
- shading consumer

That maps directly onto the “one wave drives both geometry and light” shape.

### 2.3 Ordered local pipeline

V3 already supports the authoring shape “do this, then that, then that” in a
layer-local pipeline.

So the overall execution story is not foreign to the existing model.

---

## 3. What does not map yet

The missing concept is not “image import.”

The missing concept is:

> **a first-class source that lives in braille-dot space rather than only in
> terminal-cell space**

Our current scene source vocabulary is:

- `text`
- `card`
- `image`
- `procedural`

Of these, only `procedural` is close to what the flag actually is.

So the problem is not that V3 lacks scenes or pipelines.
The problem is that the flag needs a **dotfield-native source shape**.

---

## 4. Immediate implementation recommendation

### Recommendation

For the first real implementation, still avoid inventing a brand-new top-level
V3 schema category tonight, but **do** build the generalized toolkit immediately.

That means:

```text
scene layer
  source.type = procedural
  source_id   = braille_dotfield_*
```

with the important difference that the underlying machinery is no longer
Madeira-specific.

The implementation should be split into reusable layers from the start:

1. a canonical 2×4 braille-dotfield primitive layer
2. reusable dotfield transforms (displacement, shading, overscan-aware lookup)
3. a recipe-facing procedural source family that consumes those primitives
4. the Madeira flag source as the first consumer built on top

### Why this is the right immediate choice

- avoids throwaway interim work
- preserves the true crate semantics
- gives us a reusable braille-native foundation immediately
- lets the flag prove the generalized path instead of becoming a one-off exception
- still fits the existing scene/procedural/source machinery well enough to move tonight

---

## 5. Immediate runtime shape

The first consumer should be implemented on top of a **reusable procedural braille-dotfield toolkit**, not as one giant special-case source.

### Proposed layering

```text
Layer A: braille-dotfield primitives
  - BrailleDotCanvas
  - dot ordering helpers
  - braille emission helpers

Layer B: braille-dotfield transforms
  - displacement through dotfield
  - shading from shared field
  - overscan-aware visible extent

Layer C: recipe-facing procedural sources
  - braille_flag_field
  - future braille-native banners/dataviz/decorative sources
```

### Proposed first consumer id

```text
braille_flag_field
```

### Proposed high-level behavior

```text
frame(ctx):
  1. decide logical source size in terminal cells
  2. derive dotfield size = (cell_w*2, cell_h*4)
  3. build/static-fill dotfield source content
  4. compute shared wave field
  5. compute overscan rows from max wave amplitude
  6. for each visible output cell in overscanned region:
       for each of 8 braille dots:
         evaluate shared field
         invert displacement to source-dot lookup
         sample source dot
         compute correlated shade
         accumulate braille bit + color + shade
  7. emit final braille char + fg color + transparent bg
```

### Immediate reusable helpers to add

Toolkit-level helpers should exist from the beginning:

- `BrailleDotCanvas`
- `BrailleDotOrder` or equivalent canonical dot-map surface
- `emit_braille_cell(...)`
- `displaced_dot_lookup(...)`
- `overscan_rows(...)`
- `shade_from_field(...)`

Consumer-specific helpers for Madeira can sit on top:

- `draw_flag_pattern(...)`
- `draw_cross_pattee(...)`
- `draw_greek_cross(...)`
- `wave_field(dot_x, t, params)`

---

## 6. Overscan / target-rect implications

The flag needs visible wave bleed above and below its nominal rect.

### Why this matters

If the output is clipped to the nominal flag height, the wave looks almost
static even if the internal dotfield logic is correct.

### Near-term rule

A procedural source must be able to render into a region larger than its
nominal logical source rect when the authored effect needs it.

### Minimal near-term implementation options

Any one of these is acceptable tonight:

1. procedural-source-specific `render_extent(...)`
2. procedural-source-specific `overscan_rows(...)`
3. layer-level temporary pad/overscan field for procedural layers only

The exact API is less important than the behavior:

> the procedural flag source must be able to paint above and below the nominal
> flag box

### Longer-term design direction

This likely belongs in a reusable **scene-source render extent** concept,
not a Madeira-only hack.

---

## 7. Relation to the shared-field work

Tonight's near-term implementation does **not** need the full generalized V3
field/hint architecture to be complete.

### Near-term rule

The flag source may compute and reuse the shared field **internally**.

That is still a valid “one field drives displacement + shading” implementation.

### Longer-term rule

Later, that same wave field should be expressible as:

- explicit producer step
- explicit displacement consumer
- explicit shading consumer

But that is future work.

Tonight's goal is parity with the crate, not universal pipeline generality.

---

## 8. What to reuse from gt-design and bgraph

The GTD and bgraph codebases do not solve the whole flag problem, but they do
contain useful reusable patterns.

### 8.1 bgraph lessons

Most useful files:

- `src/functions/fnc_render_braille.rs`
- `src/functions/fnc_get_braille_symbols.rs`
- `src/functions/fnc_braille_tables.rs`
- `src/functions/fnc_snap_height.rs`

The important reusable lessons are:

- centralize braille glyph tables / dot ordering
- separate source-value normalization from final glyph emission
- keep exact contract tests for fill progression and edge cases
- treat braille emission as a reusable low-level primitive, not inline ad hoc
  logic spread across many consumers

### 8.2 rocketsplash lessons

Most useful files:

- `crates/rocketsplash/src/v2/features/image/fnc_image_to_braille.rs`
- `crates/rocketsplash/src/v2/features/image/cls_braille_image.rs`
- `crates/rocketsplash/src/v2/features/quantize/fnc_map_braille.rs`
- `crates/rocketsplash/src/v2/ui/canvas/ui_render_canvas/fnc_render_braille_image.rs`

The important reusable lessons are:

- a braille surface can be treated as a first-class intermediate representation
  rather than only as a final display artifact
- the 2×4 lattice should stay explicit in code instead of being implied by a
  loose sequence of glyph writes
- source preparation, quantization, and final terminal emission are different
  layers even when they all target braille in the end

RocketSplash is image-ingest-focused, not procedural-flag-focused, so it should
not define the Madeira runtime. But it is strong evidence that a dedicated
"braille intermediate surface" mental model is practical and reusable.

### 8.3 gt-design lessons

Most useful files:

- `crates/gtd-components/src/shared/col_braille_pip.rs`
- `crates/gtd-components/src/dataviz/braille_*`
- `examples/dataviz_lab/*`

The main reusable ideas are:

- small braille primitives with clear contracts
- braille glyph selection as a reusable utility, not hand-coded everywhere
- strong contract tests for visual encodings
- dataviz-oriented examples for quickly sanity-checking braille output quality

### 8.4 What to actually extract

For the near-term procedural source, extract at most:

- one canonical braille dot map / lookup helper
- one or two tiny helpers for dotfield -> braille-cell emission

Do **not** pull in the whole dataviz stack.

The goal is to reuse the dot-ordering and emission discipline, not to import
chart abstractions.

---

## 9. Proposed file/module plan

### In `tui-vfx-recipes`

#### Add

- `src/scene/procedural/sources/cls_braille_flag_field.rs`

#### Update

- `src/scene/procedural/sources/mod.rs`
  - register/export the new source

- `src/scene/layers/cls_procedural_layer.rs`
  - if needed, support source-provided overscan/render extent

- `recipes/madeira_flag/madeira_flag.json`
  - change flag layer from `image` source to `procedural`
  - make params recipe-owned

### Optional tiny utility seam

If it improves clarity, add one small internal utility module for:

- braille dot order constants
- braille char emission from an 8-bit pattern

But keep it minimal.

---

## 10. Proposed recipe shape for the flag layer

The near-term flag layer should look more like:

```json
{
  "id": "flag",
  "role_tag": "content",
  "source": {
    "type": "procedural",
    "spec": {
      "source_id": "braille_flag_field",
      "params": {
        "colors": { "blue": ..., "gold": ..., "red": ..., "white": ... },
        "cross": { ... },
        "wave": {
          "speed_binding": "wave_speed",
          "right_edge_amplitude": 0.15,
          "harmonics": [...]
        },
        "shading": {
          "base": 0.75,
          "scale": 0.25,
          "min": 0.65,
          "max": 1.0
        },
        "overscan": {
          "top_bottom_policy": "auto_from_wave"
        }
      }
    }
  }
}
```

The exact parameter names can change, but the important thing is:

- the flag definition lives in the recipe
- the source is procedural/braille-dot-native
- no fake image fallback is required for the canonical behavior

---

## 11. Verification plan

### Unit-level

- dot-order / braille emission helper tests
- flag-pattern construction tests
- wave field weighting tests
- overscan computation tests

### Procedural-source-level

- emits braille glyphs, not block glyphs
- right edge visibly deforms more than left
- non-empty output appears above/below the nominal rect when wave amplitude
  requires it
- transparent background preserved where expected

### Recipe-level

- `madeira_flag` direct preview proof
- `madeira_flag` preview-area deterministic render proof
- validator parse/rules/stages proof
- probe truth proof
- human visual comparison against `/usr/projects/madeira-flag`

### Success condition for tonight

Not “perfect universal abstraction.”

Success is:

- the flag looks recognizably braille-native
- the right edge materially waves
- shading follows the same wave
- the output is visibly closer to the crate than the current image-like
  approximation

---

## 12. Additional design rules from the follow-on research

### 12.1 Dot order must be explicit and canonical

The braille-dot order should be defined in one helper/constant surface and used
by every consumer. Do not let each procedural source or scene bridge invent its
own dot ordering.

### 12.2 Dotfield space and cell space are different layers

Even when the output is always braille, the implementation should keep a clean
boundary between:

- **dotfield space** (2×4 subdots)
- **cell space** (terminal cells carrying one braille glyph)

The crate works because wave and shading are applied in dotfield space before
final cell emission. Future toolkit surfaces should preserve that layering.

### 12.3 Recipe truth should stay higher than engine fallback

If a source is canonical and showcase-bearing, its meaning should not live only
in one hidden engine fallback branch. The recipe should describe enough of the
source or source family that the meaning is visible in recipe space, even if the
first implementation is still a procedural-source id plus params.

---

## 13. Generalized toolkit direction

This is no longer only a future extraction target. It is the implementation
target now.

Once the first consumer works, the toolkit should already exist in the same
three-layer shape described below, even if only one consumer uses it at first.

### Layer 1 — dotfield primitives

- `BrailleDotCanvas`
- dot coordinate helpers
- braille emission helpers
- overscan helpers

### Layer 2 — dotfield transforms

- displacement by field
- shading by field
- masking / thresholding
- maybe future per-dot blending / erosion / particle deposition

### Layer 3 — recipe-facing sources

- `braille_flag_field`
- braille-native decorative banners
- braille-native dataviz surfaces
- future atmospheric dotfield sources

This is the reusable future.

But it should come **after** the first flag consumer proves the approach.

---

## 14. Decision summary

### Immediate decision

Implement the generalized braille-dotfield toolkit **now**, still using the
existing scene-layer / procedural-source model as the recipe-facing entry point.

### Deferred decision

Do **not** introduce a brand-new top-level V3 schema concept tonight unless the
existing procedural/source surface proves structurally insufficient.

### First proving consumer

Use the Madeira flag as the first consumer of the generalized toolkit rather
than as a one-off implementation lane. The first asset-contract proof has now
landed; richer parity work should build on that reusable path rather than
reintroducing Madeira-specific artwork in Rust.

---

## 15. Final recommendation

If the goal is **tonight: get the recipe to relative parity with the crate**
without wasting effort, then the correct plan is:

1. stop modeling the flag as an image-like layer
2. build the generalized braille-dotfield primitives immediately
3. build reusable displacement/shading/overscan dotfield transforms
4. expose a recipe-facing procedural source family on top of them
5. implement `braille_flag_field` as the first consumer ✅
6. select its artwork through `requires_assets` so the showcase remains asset-agnostic ✅
7. use the resulting implementation to prove richer showcase parity on Madeira

That is the shortest honest path from the current state to a recipe-side flag
that actually behaves like the source crate **and** leaves behind reusable
foundation instead of disposable interim work.


## 16. Post-release strategy

The sections above describe the current toolkit shape and the first proving
consumer. This section is the post-release lane: follow-on exploration, family
design, and downstream adoption. It is intentionally not a release gate.

### 16.1 Inspiration review: bgraph and rocketsplash

`bgraph` is useful because it treats braille lookup and emission as a small,
explicit utility surface. It shows the value of canonical dot ordering, compact
tables, and predictable glyph selection.

`rocketsplash` is useful because it treats braille as an intermediate
representation in a pipeline: source preparation, quantization, and final
terminal emission stay separate even when the output is braille.

The takeaway for post-release work is simple:

- reuse the braille surface discipline
- do not copy image-ingest assumptions into procedural effects
- keep source prep, field transforms, and terminal emission separable

### 16.2 Ownership boundaries

```text
Downstream apps
  ├─ choose the story, UX, and release packaging
  ├─ own policy, defaults, and product-facing names
  └─ consume recipe outputs instead of reinterpreting internals
        │
        ▼
tui-vfx-recipes
  ├─ owns recipe schema and authoring-facing source params
  ├─ owns procedural source registration and validation
  └─ exposes braille-dotfield sources as recipe data, not app policy
        │
        ▼
tui-vfx
  ├─ owns rendering semantics, braille emission, filters, compositor behavior
  ├─ owns visual transforms and reusable effect engines
  └─ keeps effect policy inside the library, not in downstream apps
        │
        ▼
mixed-signals
  ├─ owns reusable signal/math substrate
  ├─ owns field primitives that are not terminal-specific
  └─ stays upstream of tui-vfx when the math is reusable beyond one effect
```

Boundary rule of thumb:

- **mixed-signals**: reusable field/math substrate
- **tui-vfx**: rendering semantics, braille emission, visual transforms
- **tui-vfx-recipes**: recipe-facing source shapes and validation
- **downstream apps**: policy, packaging, and presentation

### 16.3 Creative procedural lanes

These are post-release effect families to explore, not immediate blockers:

| Idea | What it does | Likely home |
|---|---|---|
| Mouse pixie dust / trails | Leave a short-lived particle wake behind pointer movement | `tui-vfx` effect or recipe-facing procedural source |
| Starburst | Radial bursts from clicks, taps, or timed triggers | `tui-vfx-recipes` source backed by `tui-vfx` transforms |
| Negative-space words | Carve text by preserving empty cells around the word shape | `tui-vfx-recipes` authoring surface with `tui-vfx` emission |
| Ant-farm / minions | Small agents follow trails, cluster, and reform paths | `tui-vfx` simulation layer with recipe parameters |
| Sketch / drawing | Jittered strokes, hatch fills, and line-drawn contours | `tui-vfx` renderer and reusable recipe source family |

The point is not to name every possible effect up front. The point is to make
the substrate reusable enough that the family can grow without re-deciding the
braille boundary each time.

### 16.4 ANSI block diagrams

#### Architecture diagram

```text
┌──────────────────────────────┐
│ downstream apps              │
│ - policy                     │
│ - packaging                  │
│ - UX / story / defaults      │
└───────────────┬──────────────┘
                │ consumes recipe outputs
                ▼
┌──────────────────────────────┐
│ tui-vfx-recipes              │
│ - schema                     │
│ - validation                 │
│ - procedural source ids      │
│ - authoring params           │
└───────────────┬──────────────┘
                │ emits braille-dotfield intent
                ▼
┌──────────────────────────────┐
│ tui-vfx                      │
│ - compositor                 │
│ - braille emission           │
│ - dotfield transforms        │
│ - visual effects             │
└───────────────┬──────────────┘
                │ reuses substrate
                ▼
┌──────────────────────────────┐
│ mixed-signals                │
│ - reusable signal/math leaves │
│ - field primitives            │
└──────────────────────────────┘
```

#### Visual concept diagram

```text
┌─────────────────────────────────────────────────────────────┐
│   •   •      ✦          . . .         •                    │
│     \ | /               trail          \                   │
│  ---  *  ---      NEGATIVE SPACE      ---*---              │
│     / | \             WORDS             /                   │
│   ant-farm lines  ···············  sketch hatch            │
│            mouse pixie dust / starburst / drawing          │
└─────────────────────────────────────────────────────────────┘
```

### 16.5 Post-release implementation phases

#### Phase 1 — inventory and classify

- catalog existing braille consumers and helpers
- separate reusable substrate from one-off effect behavior
- tag which ideas belong in `mixed-signals`, `tui-vfx`, or recipes

#### Phase 2 — substrate extraction

- normalize braille dot ordering and emission helpers
- keep dotfield space separate from terminal-cell space
- extract the smallest reusable transforms first

#### Phase 3 — effect family prototypes

- prototype mouse trails, starbursts, and negative-space text
- test ant-farm / minion movement as a controlled simulation lane
- add sketch/drawing experiments where the braille surface is the output

#### Phase 4 — recipe-facing promotion

- promote the stable families into recipe source ids
- document params, defaults, and expected visual behavior
- keep downstream apps on recipe outputs rather than internal effects

#### Phase 5 — polish and adoption

- add examples and screenshots or terminal captures
- tighten docs around ownership boundaries
- prune dead experiments that do not justify their surface area

Post-release success is a small, stable family of reusable braille-dotfield
effects, not a single oversized abstraction.

<!-- <FILE>docs/design/post-release/braille-dotfield-toolkit-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.6.2</VERS> -->
