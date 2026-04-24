# tui-vfx V3 motion spec

Status: draft

## Purpose

Define motion as a first-class V3 schema concern.

This draft replaces the idea that motion is only a few extra fields on `pipeline.timing`.

Motion in tui-vfx actually contains:
- a temporal curve
- a spatial route
- optional dynamic treatment layered over that route
- origin and destination placement
- snapping / quantization policy
- viewport-edge behavior while clipped
- scope: whole recipe or individual scene layer

## Inputs reviewed

This draft is based on:
- V2 recipe shape in `tui-vfx-recipes/src/recipe_schema/config.rs`
- runtime motion types in `tui-vfx-geometry`
- motion-bearing recipes under `tui-vfx-recipes/recipes/`
- `steering/INTENTIONS.md`
- current V3 docs under `docs/design/`
- OFPF reads of V2 pathway ownership in `tui-vfx-recipes` and leaked motion/shadow policy in `gt-design`

## Design goals

1. Motion should read like a motion tree, not a timing grab bag.
2. Simple motion should stay simple.
3. Complex motion should be composable.
4. Recipe-level motion and layer-level motion should both be first-class.
5. Direction-aware edge behavior should live with motion.
6. Motion semantics should live in tui-vfx, not leak upward into gt-design wrappers.

## Boundary note: substitutions and bindings

This document describes the **resolved typed motion model**.

It does **not** define GT Design's token system or any downstream theme-token
authoring layer.

The intended boundary is:
- callers may populate recipe fields through load-time `Substitutions`
- callers may provide per-frame `RuntimeBindings`
- `tui-vfx-recipes` resolves those boundary surfaces
- motion execution then works from the resolved typed form

So when this document shows fields like `duration_ms`, `route`, `from`, or
`screen_edge`, it is describing the schema/runtime meaning after substitution,
not the caller's token vocabulary.

## Core observation

The current engine already distinguishes several motion dimensions, even if the schema does not.

### Temporal dimension
- `duration_ms`
- `easing`
- `quantize_steps`

### Spatial dimension
- route through cell space
- offscreen origin / destination
- optional waypoint / control placement

### Dynamic dimension
The engine's current `PathType` catalog mixes:
- true route shapes
- motion dynamics / treatments

Those are not the same thing.

Example:
- `arc` describes a route
- `pendulum` describes oscillatory behavior

The user should be able to say:
- pendulum over an arc route
- pendulum forced onto a linear route
- spring settle on a rectilinear route

That requires the schema to separate **route** from **dynamics**.

## Recommended schema homes

## 1. Recipe-envelope motion

Use this to move the whole composed recipe.

Recommended home:

```json
{
  "config": {
    "motion": {
      "enter": { "...": "..." },
      "exit": { "...": "..." }
    }
  }
}
```

This is the preferred V3 home over flat `pipeline.timing.enter_*` fields.

Why:
- motion is not only timing
- motion owns screen-edge clipping behavior
- motion needs its own reusable shape at recipe and layer scopes

## Relative motion between objects

V3 should explicitly allow objects to move relative to each other in time and space.

This matters for compositions like:
- splash screens with title, subtitle, logo, and panel arriving on different trajectories
- drawers whose chrome, content, and affordances lag or lead each other
- tooltips or badges that track a moving parent with offsets or delayed follows
- orbiting / tethered decorative elements around a moving focal element

That means scene-layer motion should not be limited to absolute placement only.

### Recommended relational model

A layer's placement should be able to resolve relative to:
- the frame
- its authored anchor
- a named sibling / parent layer
- a named motion target exported by another layer

Conceptually:

```json
"placement": {
  "anchor": "below_sibling",
  "sibling_id": "hero_panel",
  "offset_rows": 2,
  "motion": {
    "enter": {
      "route": { "type": "linear" },
      "relative_to": "hero_panel",
      "follow": { "mode": "maintain_offset" }
    }
  }
}
```

### Minimal design requirement

The schema should support at least these relational cases:

1. **Static relative placement + independent motion**
   - layer starts relative to another layer
   - then follows its own motion

2. **Relative destination placement**
   - layer motion resolves `to` relative to another layer

3. **Tracked / follow motion**
   - a layer follows another layer's motion with maintained offset, optional lag, or optional easing

4. **Shared choreography, different routes**
   - multiple layers share timing windows but move differently

### Recommended field direction

Do not overload generic effect scope for this.

Keep it in scene placement / motion, with concepts like:
- `relative_to`
- `follow`
- `lag_ms`
- `offset`
- `phase_offset_ms`
- `target_anchor`

### Why this belongs in V3

This is exactly the kind of tree-structured relationship V3 should express well.

If we omit it, authors will end up flattening choreography into:
- many absolute coordinates
- duplicated timing
- app-layer orchestration glue

That would be a step backward.

## 2. Scene-layer motion

Use this to move individual elements inside a scene.

Recommended home:

```json
{
  "scene": {
    "layers": [
      {
        "placement": {
          "anchor": "center",
          "motion": {
            "enter": { "...": "..." },
            "exit": { "...": "..." }
          }
        }
      }
    ]
  }
}
```

Why this home:
- layer motion is geometry / placement
- it belongs next to anchor, offsets, sibling-relative layout, and z ordering
- it keeps multi-element splash / overlay choreography explicit

## 3. Pipeline timing remains separate

Per-step clocks remain in the pipeline tree.

That includes:
- shader oscillation clocks
- sampler clocks
- content clocks
- phase-gated per-step timing

These are not placement motion and should not move into the motion subtree.

## Canonical shape

Use one reusable shape for both recipe and layer scopes.

```json
MotionPhaseSpec = {
  "duration_ms": number,
  "easing": string,
  "route": MotionRouteSpec,
  "dynamics": [MotionDynamicSpec],
  "from": PlacementSpec?,
  "via": PlacementSpec?,
  "to": PlacementSpec?,
  "edge_crossing": EdgeCrossingSpec?,
  "snap": SnappingStrategy?,
  "quantize_steps": number?
}
```

## Motion route

`route` describes the geometric path the object follows.

Recommended initial route families:
- `linear`
- `arc`
- `bezier`
- `rectilinear`
- `spiral`
- `figure_eight`

Possible extension families later:
- explicit path-point lists
- future route refs or runtime-driven hooks after the API boundary populates
  typed values

### Route examples

```json
"route": { "type": "linear" }
```

```json
"route": { "type": "arc", "bulge": -0.35 }
```

```json
"route": {
  "type": "bezier",
  "control_x": 30,
  "control_y": 4
}
```

## Motion dynamics

`dynamics` describes how motion behaves while traveling along the route.

Recommended initial dynamic families:
- `spring`
- `bounce`
- `friction`
- `pendulum`
- `hover`
- `squash`
- `step`
- `projectile`
- `orbit`
- `attractor`
- `swirl`
- `carrier_orbit`

Some of these are currently represented as standalone `PathType` variants. In V3, the cleaner model is to treat them as motion dynamics layered over a route.

### Dynamic examples

```json
"dynamics": [
  { "type": "pendulum", "amplitude": 18, "oscillations": 3, "damping": 2.0 }
]
```

```json
"dynamics": [
  { "type": "spring", "stiffness": 210.0, "damping": 18.0 }
]
```

```json
"dynamics": [
  { "type": "step", "steps": 8 }
]
```

## Why route + dynamics is the right split

It makes all of these legal without inventing special-case one-off path variants:
- arc + pendulum
- linear + pendulum
- bezier + spring
- rectilinear + bounce
- linear + friction + step

That is more tree-like, more composable, and closer to the V3 goals than a flat enum that tries to encode every combined motion style as one name.

## Placement fields

Use the geometry engine's existing placement family.

```json
PlacementSpec =
  | { "type": "offscreen", ... }
  | { "type": "anchor", ... }
  | { "type": "frame_permille", ... }
  | { "type": "absolute", ... }
```

And keep optional `via` in the public V3 model.

Why:
- the engine already supports it via `MotionSpec`
- Bezier is under-specified without a control / waypoint concept
- richer choreography needs it even if the current corpus does not use it heavily yet

As with the rest of the motion schema, these are the resolved typed placement
fields. A caller may populate them through substitutions before load, but that
tokenization mechanism is outside the scope of this document.

### Legacy-equivalent positioning capabilities preserved generically

The legacy toast library supports more than one resting anchor. It also supports:
- separate entry anchor
- separate exit anchor
- custom entry position
- custom exit position
- custom entry/exit position specs

In V3 these should remain **generic placement capabilities**, not toast-specific
field names.

Recommended equivalents:
- `slide_entry_anchor` -> `motion.enter.from = { \"type\": \"anchor\", ... }`
- `slide_exit_anchor` -> `motion.exit.to = { \"type\": \"anchor\", ... }`
- `custom_entry_position(_spec)` -> `motion.enter.from = PlacementSpec`
- `custom_exit_position(_spec)` -> `motion.exit.to = PlacementSpec`

This keeps the capability while avoiding a legacy-shaped field explosion in the
V3 tree.

### Placement is the right typed hook for caller-supplied dynamic values

If a caller needs to populate motion entry/exit positions dynamically, `from`,
`via`, and `to` are the right generic hooks.

That means the schema should be designed so these typed placement fields can be
populated at the API boundary by:
- load-time substitutions
- runtime bindings, where the public boundary allows them

without introducing toast-specific escape-hatch fields just for dynamic entry
or exit positioning.

## Motion host and visual envelope

Motion needs one library-owned object that represents the thing being moved.

Recommended internal model:
- **host rect** — the authored body / border geometry that owns placement and motion
- **attached shadow** — optional shadow bound to that host
- **visual envelope** — host rect plus any shadow extents that are visually part of the moving result

This is the glue currently missing from the architecture.

Without it, vanishing-edge logic only sees border geometry, while shadow logic only sees shadow config. That splits one moving object into two unrelated calculations.

For smooth offscreen entrance, the runtime should treat them as one bound unit:
- the host rect drives placement
- the shadow is attached to that host
- the visual envelope is what intersects the viewport edge
- screen-edge policy decides how border and shadow react when the envelope is clipped

This should live in the library layer, not in app wrappers.

Possible compiled/runtime names:
- `MotionHostSpec`
- `MotionHostInstance`
- `VisualEnvelope`

The specific type names can change, but the separation of host vs attached shadow vs visual envelope should remain.

## Edge crossing belongs to motion

Vanishing edge exists because motion intersects viewport clipping.

Recommended home:

```json
"edge_crossing": {
  "edge": "left" | "right" | "top" | "bottom",
  "border": "vanish" | "preserve",
  "shadow": "fade" | "clip" | "preserve"
}
```

This applies only while the moving host is clipped by the viewport.

See `docs/design/tui-vfx-v3-vanishing-edge-spec.md`.

## Directional awareness is required

Vanishing-edge behavior must be direction-aware.

It cannot be derived only from anchor.

For example:
- right-edge entry should trim the right-side leading edge
- top-edge exit should trim the top-side trailing edge
- curved motion should adapt based on actual instantaneous motion direction, not a static guessed axis

### Recommended resolution order

At each sampled frame, determine the active clipped edge by:
1. explicit `edge_crossing.edge` when authored for the active phase
2. current motion tangent, if available
3. fallback vector from resolved `from` to resolved `to`
4. fallback offscreen direction when present
5. final fallback: dominant axis of current rect delta from previous sample

That rule should be shared by:
- preview
- demo
- validator / probe
- future native runtime

And it should operate on the motion host / visual envelope model, not on raw border-only geometry.

## Explicit edge-crossing object

Vanishing-edge behavior should not rely only on generic motion direction. The schema should be able to name the viewport edge involved in the crossing and the policy to apply there in one object.

Recommended vocabulary for `edge_crossing.edge`:
- `left`
- `right`
- `top`
- `bottom`

This object is most useful when:
- the host starts or ends offscreen
- the motion is curved and author intent should override tangent-derived guesses
- a scene-layer follows another moving object and the effective crossing edge should stay stable

It may often be derivable from `from` / `to`, but the compiled motion model should carry it explicitly after normalization so screen-edge math does not need to rediscover it repeatedly.

## Defaults

If omitted:
- `route` defaults to `linear`
- `dynamics` defaults to empty
- `from` defaults to the resolved resting placement
- `to` defaults to the resolved resting placement
- `snap` defaults to `round`
- `edge_crossing.border` defaults to `vanish` when an edge crossing is active
- `edge_crossing.shadow` defaults to `fade` when an edge crossing is active

## Examples

## Simple toast slide-in

```json
{
  "config": {
    "motion": {
      "enter": {
        "duration_ms": 450,
        "easing": "quad_out",
        "route": { "type": "linear" },
        "from": { "type": "offscreen", "direction": "from_right", "margin_cells": 0 },
        "to": { "type": "anchor", "anchor": "bottom_right" },
        "edge_crossing": { "edge": "right", "border": "vanish", "shadow": "fade" }
      }
    }
  }
}
```

## Splash logo on an arc with pendulum treatment

```json
{
  "scene": {
    "layers": [
      {
        "id": "logo",
        "placement": {
          "anchor": "center",
          "motion": {
            "enter": {
              "duration_ms": 900,
              "easing": "quad_out",
              "route": { "type": "arc", "bulge": -0.25 },
              "dynamics": [
                { "type": "pendulum", "amplitude": 12, "oscillations": 2, "damping": 2.2 }
              ],
              "from": { "type": "offscreen", "direction": "from_top" },
              "to": { "type": "anchor", "anchor": "center" },
              "edge_crossing": { "edge": "top", "border": "vanish", "shadow": "fade" }
            }
          }
        }
      }
    ]
  }
}
```

## Pendulum forced onto a linear route

```json
{
  "config": {
    "motion": {
      "enter": {
        "duration_ms": 700,
        "easing": "linear",
        "route": { "type": "linear" },
        "dynamics": [
          { "type": "pendulum", "amplitude": 8, "oscillations": 3, "damping": 2.8 }
        ],
        "from": { "type": "offscreen", "direction": "from_left" },
        "to": { "type": "anchor", "anchor": "middle_left" }
      }
    }
  }
}
```

## Upstream mixed-signals capability

The motion schema should be designed from the upstream `mixed-signals` capability set, not only from the current `PathType` enum.

Relevant upstream primitives already exist in `/usr/projects/mixed-signals`:
- `DampedSpring`
- `BouncingDrop`
- `FrictionDecay`
- `BallisticTrajectory`
- `SimplePendulum`
- `CircularOrbit`
- `PointAttractor`

And upstream composition primitives already exist in `SignalSpec`:
- `Add`
- `Multiply`
- `Mix`
- `Keyframes`
- `Clamp`
- `Quantize`
- `Remap`
- `Invert`
- `Abs`
- oscillator / noise / envelope families

That leads to two explicit V3 requirements.

### Requirement A — cover every current mixed-signals physics primitive

The V3 motion schema should have a place for all current upstream physics solvers, even if some land as named dynamic families before a fuller signal-graph surface exists.

Minimum target set:
- spring
- bounce
- friction
- projectile
- pendulum
- orbit
- attractor

### Requirement B — leave a path to signal-graph-driven motion

Longer-term, motion should be able to consume mixed-signals-style graph inputs for:
- route progress
- lateral deviation
- vertical deviation
- settling / overshoot
- periodic modulation
- noise / jitter / turbulence

That does **not** mean V3 initial must expose the entire `SignalSpec` surface directly inside every motion block.

It does mean the motion schema should not trap us in a flat enum model that cannot grow into signal composition later.

## Recommended normalization layers

### Authoring shape

Authors should be able to write either:
- a simple route-only motion
- a route + named dynamics motion
- eventually, a route + signal-driven dynamics motion

### Compiled shape

Normalize motion internally to:
- carrier route
- progress curve
- ordered dynamics stack
- screen-edge treatment
- resolved placement endpoints / waypoints

That compiled form is the right place to hook later mixed-signals execution.

## Practical implication for subsequent pipeline work

Subsequent implementation work should follow this order:

1. define the V3 authoring schema for route + dynamics + screen-edge treatment
2. define normalized / compiled motion types in `tui-vfx-recipes`
3. map current V2 `MotionSpec` and `PathType` into that compiled form
4. map current mixed-signals physics primitives into that compiled form
5. only then continue direct-runtime execution work

That sequencing keeps the schema tree in charge instead of letting the old runtime shape dictate the new authoring model.

## Relation to z and depth

Motion does not define z order.

For scenes:
- stacking order remains a placement / composition concern (`scene.layers[*].placement.z` or equivalent)
- motion changes where a layer is over time
- shadow and tonal treatment communicate perceived depth while it moves

So the split is:
- placement / z: who is in front
- motion: where it moves
- shadow / tone: how depth is perceived

## Why this should stay out of gt-design wrappers

OFPF reads in `gt-design` show motion-related policy has leaked upward into:
- `crates/gtd-ratatui/src/recipes/item.rs` (`item_from_recipe_config`)
- `crates/gtd-widget-contracts/src/enum_gtd_slide_border_trim_policy.rs`

That is the wrong long-term ownership boundary.

The V3 motion model should live in tui-vfx and flow through canonical builder output so downstream consumers do not need parallel motion policy enums or conversion logic.

## Migration from V2

Companion variant-by-variant mapping: [`tui-vfx-v3-motion-compatibility-table.md`](tui-vfx-v3-motion-compatibility-table.md).

### V2 enter / exit motion

Map:
- `pipeline.enter.duration_ms` -> `config.motion.enter.duration_ms`
- `pipeline.enter.easing` -> `config.motion.enter.easing`
- `pipeline.enter.motion_path` -> `config.motion.enter.route` or `route + dynamics`
- `pipeline.enter.from` -> `config.motion.enter.from`
- `pipeline.enter.snapping` -> `config.motion.enter.snap`
- `pipeline.enter.quantize_steps` -> `config.motion.enter.quantize_steps`

And the same for exit.

### Legacy PathType mapping

Examples:
- `linear` -> `route = linear`
- `arc` -> `route = arc`
- `bezier` -> `route = bezier`
- `spring` -> `route = linear`, `dynamics = [spring]` unless a route override is explicitly given
- `pendulum` -> `route = linear`, `dynamics = [pendulum]` unless a route override is explicitly given
- `bounce` -> `route = linear`, `dynamics = [bounce]` unless a route override is explicitly given

This lets V2 content migrate without losing behavior while still moving V3 toward the cleaner compositional model.

## Non-goals

V3 initial should not:
- turn arbitrary scoped pipeline steps into geometry movers
- force every author to use scene layers for simple toasts
- encode every route+dynamics combination as a brand-new enum case


## Normalized / compiled motion model

Suggested internal types for the subsequent pipeline work:

```text
NormalizedMotionPhase
├─ duration_ms
├─ easing
├─ route
├─ dynamics[]
├─ from?
├─ via?
├─ to?
├─ edge_crossing?
├─ snap
├─ quantize_steps?
└─ ...

CompiledMotionPhase
├─ resolved_from
├─ resolved_via?
├─ resolved_to
├─ normalized_edge_crossing?
├─ compiled_route
├─ compiled_dynamics[]
├─ compiled_progress_curve
└─ compiled_edge_crossing_policy
```

The important part is not the exact names. It is that the compiled form carries:
- route
- dynamics
- explicit viewport-edge semantics
- resolved placements
- one shared screen-edge policy

so preview, validator, probe, and runtime all execute the same model.
