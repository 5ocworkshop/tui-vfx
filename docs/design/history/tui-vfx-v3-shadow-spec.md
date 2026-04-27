# tui-vfx V3 shadow spec

Status: draft

## Purpose

Define shadows as a first-class V3 schema concern and place them correctly relative to:
- z / stacking order
- depth perception
- tonal layering
- motion
- viewport-edge behavior

## Key framing

Shadow is one depth cue, not the whole depth model.

In tui-vfx and GT Design, perceived depth comes from several layers of meaning:
- **z / stacking** — who is in front of whom
- **tonal layering** — darker/lighter surface treatment used to imply elevation
- **overlay semantics** — whether the thing is a tooltip, drawer, modal, splash, etc.
- **shadow** — optional explicit cast-shadow treatment

The schema should keep those responsibilities distinct.

## Boundary note: substitutions and bindings

This document describes the **resolved typed shadow model**.

It does **not** define downstream token systems or theme-token expansion rules.

The intended boundary is:
- callers may populate shadow-related authoring fields through load-time
  `Substitutions`
- callers may supply runtime values through `RuntimeBindings` where the public
  API allows them
- `tui-vfx-recipes` resolves those surfaces before canonical playback/runtime
  execution

So when this document describes `offset`, `color`, `source_region`, or
`composite`, it is describing the typed V3 surface after boundary resolution.

## Host-bound shadow model

Shadow should be treated as attached to a moving host, not as an independent free-floating render pass.

Recommended runtime model:
- **host rect** — the surface that owns the shadow
- **attached shadow** — the authored shadow spec for that host
- **visual envelope** — host rect plus shadow extents

Why this matters:
- vanishing-edge treatment needs to know the shadow belongs to that host
- viewport clipping needs one coherent envelope to reason about
- the app should not hand-roll shadow/border coupling

This is especially important for transparent shadows on sliding toasts and drawers.

## Ownership split

### 1. Stacking / z order
Belongs to scene composition and placement.

For scenes, this is the layer placement / stacking model.

### 2. Tonal ladder / depth tone
This is a visual-system concern.

Downstream systems like GT Design may map elevation to tonal ladders, but tui-vfx core should avoid baking GTD-specific tokens directly into the V3 schema.

### 3. Shadow
Shadow remains an explicit VFX-owned rendering treatment.

That is the focus of this document.

## Recommended schema homes

## 1. Recipe-envelope shadow

Use for a whole composed overlay, toast, drawer, card, or panel.

Recommended home:

```json
{
  "config": {
    "shadow": { "...": "..." }
  }
}
```

## 2. Scene-layer shadow

Use when individual scene elements need their own depth treatment.

Recommended home:

```json
{
  "scene": {
    "layers": [
      {
        "surface": {
          "shadow": { "...": "..." }
        }
      }
    ]
  }
}
```

Why `surface.shadow`:
- it is visually attached to the rendered layer surface
- it composes with base style and border role information
- it stays local to the element whose silhouette is casting the shadow
- it is now wired through the current native V3 scene execution path rather than being only a draft schema intent

## Canonical shadow shape

```json
ShadowSpec = {
  "style": "solid" | "half_block" | "medium_shade" | { "braille": { "density": 0.65 } } | { "gradient": { "layers": 3 } },
  "color": { "r": 0, "g": 0, "b": 0, "a": 160 },
  "offset": { "x": 2, "y": 1 },
  "inset": { "x": 0, "y": 0 },
  "edges": ["right", "bottom"],
  "soft_edges": true,
  "surface_color": { "...": "optional" },
  "source_region": "Border",
  "composite": {
    "mode": "glyph_overlay" | "grade_underlying",
    "grade": {
      "fg_dim_strength": 0.40,
      "bg_dim_strength": 0.58,
      "fg_desaturate_strength": 0.30,
      "bg_desaturate_strength": 0.42,
      "fg_tint_strength": 0.10,
      "bg_tint_strength": 0.18,
      "preserve_fg_alpha": true,
      "preserve_bg_alpha": true,
      "replacement_char": "·"
    }
  }
}
```

As built on the current direct V3 preview/demo path, the bounded host region is now replayed over the **live preview underlay** before shadow composition, so transparent and grade-underlying shadows land on the same canvas/substrate semantics as the V2 path instead of grading against an empty black framebuffer. The default style is `solid`: a clean alpha-bearing full-cell drop shadow with the configured offset. Half-block, braille, medium-shade, and gradient are explicit texture choices.

As built in the current code path, the root V3 shadow surface now maps
directly onto the typed upstream shadow runtime surface:
- `tui_vfx_compositor::types::ShadowSpec`
- `tui_vfx_shadow::ShadowConfig`

## Field semantics

### `style`
Maps to current shadow renderer families:
- `solid` (default transparent full-cell offset shadow)
- `half_block`
- `medium_shade`
- `braille`
- `gradient`

### `color`
Authoritative shadow color, including alpha.

Transparent shadow lives here first.

### `offset`
Signed cast offset.

Positive values cast right/down. Negative values cast left/up.

### `inset`
Orthogonal inset trimming before the edge run begins.

### `edges`
Explicit edge list instead of bitflag text.

### `soft_edges`
Whether the shadow uses softer terminal-edge transitions.

### `surface_color`
Optional half-block blending helper.

### `source_region`
Role-aware extrusion source.

For bordered cards and overlays, `Border` is the important value.

### `composite.mode`
- `glyph_overlay`
- `grade_underlying`

### `composite.grade`
Destination-preserving grading controls for the `grade_underlying` mode.

## Transparent shadow is a required capability

Transparent shadow must be represented explicitly, not as an accidental side effect.

### 1. Alpha-bearing shadow color

```json
"color": { "r": 0, "g": 0, "b": 0, "a": 160 }
```

This covers the common translucent drop shadow case.

The actual numeric/color values may be populated by the caller before load, but
the V3 schema should still describe the resolved typed shadow fields, not the
caller's token syntax.

### 2. Grade-underlying compositing

```json
"composite": {
  "mode": "grade_underlying",
  "grade": { "...": "..." }
}
```

This covers the case where the shadow should:
- preserve the destination glyph
- dim / desaturate / tint the destination beneath it
- preserve existing alpha channels where desired

That matters for GT Design overlays where tonal layering and readable underlying content both matter.

## Relationship to z and tonal layering

Shadow does not define z order.

Instead:
- z order decides draw order and occlusion
- tonal ladders communicate perceived elevation even with no shadow
- shadow is an optional explicit depth cue layered on top

This is important for GT Design because overlays may be authored:
- with shadow
- without shadow
- with tonal elevation only
- with both tonal elevation and shadow

The V3 schema should support all of those cases.

## Recommended future-compatible depth rule

If a future generic depth subtree is introduced, it should be generic and optional.

For example:

```json
"depth": {
  "elevation": "overlay_3",
  "tonal_layer": "elevated_surface",
  "shadow": { "...": "optional override" }
}
```

But this draft does **not** require that subtree yet.

For now:
- z stays with placement / composition
- explicit shadow stays with `config.shadow` or `scene.layers[*].surface.shadow`
- tonal ladder policy can remain downstream until a generic core shape is justified

## Motion integration

Shadow follows the geometry that owns it.

The runtime should compute motion on the host and derive shadow placement from that host, not separately animate shadow geometry in app code.

### Recipe shadow
- follows recipe-envelope motion
- reacts to viewport clipping through motion's `edge_crossing.shadow` policy

### Layer shadow
- follows that layer's placement motion
- reacts to viewport clipping through that layer motion's `edge_crossing.shadow` policy

Recommended rule for V3 initial:
- shadow does not define an independent path of its own
- shadow inherits the owning host's motion

## Depth, z, and shadow attachment

A useful separation is:
- **z / stacking**: who draws in front
- **tonal depth**: what elevation feels like even without a cast shadow
- **attached shadow**: optional explicit cast-shadow treatment bound to the host

That means an overlay can be:
- elevated by tone only
- elevated by shadow only
- elevated by both
- flat with neither

The schema should support all four.

## Directional viewport-edge behavior

Because smooth offscreen entrance depends on directional awareness, shadow handling at the screen edge must also be directional.

If a host is entering from the right:
- the right-side clipped edge is active
- border trimming and shadow fading must follow that edge

If a host is moving along a curve:
- the active edge should follow the current motion tangent, not a static anchor assumption

That policy belongs to motion, but shadow must explicitly honor it.

See `docs/design/tui-vfx-v3-vanishing-edge-spec.md`.

## Migration from current surfaces

### Current recipe-level shadow JSON

Current authored shape already resembles:

```json
"shadow": {
  "style": "solid",
  "offset_x": 2,
  "offset_y": 1,
  "color": { "r": 0, "g": 0, "b": 0, "a": 160 },
  "edges": "RIGHT | BOTTOM",
  "soft_edges": true,
  "source_region": "Border"
}
```

Recommended V3 cleanup:
- normalize `offset_x` / `offset_y` into `offset: {x, y}`
- normalize edge bitflags into symbolic JSON arrays
- nest composite controls under `composite`
- preserve the semantic capability set already supported by `ShadowConfig`

## Library-layer requirement

Current evidence from `tui-vfx-recipes` and `gt-design` shows parts of trim/shadow ownership are still split:
- border trim decisions are made in recipes/runtime code from visible rect math
- shadow is carried separately as `ShadowSpec`
- gt-design still mirrors some trim/shadow-related policy in wrapper code

The end-state should move that coupling fully into tui-vfx library execution so downstream apps author intent instead of reconstructing the coupling.

## Ownership boundary

OFPF reads in `gt-design` show shadow policy has leaked upward into:
- `crates/gtd-factory/src/depth/fnc_build_shadow_spec.rs`
- contract-layer shadow vocabulary and conversion glue

Some downstream policy is expected because GT Design owns its own resolved design system.

But the shadow rendering semantics themselves should stay in tui-vfx.

The V3 schema should therefore keep shadow as an upstream authored surface instead of forcing every downstream consumer to reconstruct it.

## Non-goals

This draft does not introduce:
- independent shadow-only motion paths
- GTD-specific tonal ladder tokens in core V3
- mandatory shadow for every elevated element

Those are separate concerns.


## Suggested integration types

For later implementation, the library should likely normalize toward types like:

```text
AttachedShadowSpec
├─ shadow payload
├─ owner_kind (recipe | layer)
├─ source_region policy
└─ composite policy

MotionHostSpec
├─ host_rect
├─ border model?
├─ attached_shadow?
└─ edge_crossing policy

VisualEnvelope
├─ host_bounds
├─ shadow_bounds?
└─ clipped_bounds
```

Again, the exact names can change. The important design rule is that shadow is attached to a host and participates in one shared envelope calculation.
