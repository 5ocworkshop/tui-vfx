<!-- <FILE>docs/design/post-release/relative-motion-spatial-constraints-spec.md</FILE> - <DESC>Post-release spec for spatial-constraint relative motion: a true per-frame distance/angle constraint solver layered over the existing follow/sibling-anchor mechanism.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Captures the relative-motion ideas surfaced while authoring the depth-stack-sheets shuffle recipe in tui-vfx-recipes/recipes/5.5-suggestions/. Today's substrate provides static sibling anchoring + temporal follow lag; this spec proposes a per-frame spatial constraint family for behaviors like pinned offsets, magnetic repulsion, and orbital pairing. Post-release because none of these are V3 release blockers.</WCTX> -->
<!-- <CLOG>0.1.0: initial post-release spec for spatial-constraint relative motion (pinned offset, repulsion, orbital pairing, formation cluster).</CLOG> -->

# Spatial-constraint relative motion (post-release)

> Status: post-release. Not on the V3 release punch list. Captured here so the
> ideas are not lost while the V3 player, validator, and migration work close
> out.

## Why this is post-release

The current substrate already supports two real but limited forms of relative
motion (see `src/scene/layers/mod.rs` and `src/v3/compile/cls_v3_motion_envelope.rs`
in `tui-vfx-recipes`):

| Knob | What it does | What it does NOT do |
|---|---|---|
| `placement.spec.sibling_id` + `offset_rows` / `offset_cols` | Resolves the layer's **rest** rect from a sibling's rect at composition time, in z-sort order | Continuous re-evaluation per frame; the offset is computed once during layer paint |
| `motion.relative_to: "<sibling_id>"` + `target_anchor` | Sets the motion `to` endpoint from the sibling's **rest** rect (anchored to the named target) | Tracks the sibling's *animated* position mid-transit |
| `follow: { mode: "maintain_offset", lag_ms: N }` | Subtracts `N` ms from the follower's `phase_t` so the follower replays the same animation `N` ms later | Spatial spring or distance solver — it is purely a time-offset on the follower's own animation |

That gives recipes leader/follower with a **time delay**, not "two layers
constrained at distance D" or "two layers maintaining a 90° angle as they
orbit." Several authoring scenarios surfaced during the
`5.5-suggestions/12_depth_stack_sheets_shuffle.json` work want exactly that
class of behaviour:

- two cards exchanging positions while clearing each other on a depth axis
- a follower badge that orbits a leader card while staying outside a minimum
  radius
- ambient companion sprites (cursors, indicators, attendant glyphs) that pin to
  a moving target and respect a no-overlap zone
- a procedural "cluster" of items that maintain relative positions while the
  whole formation slides or rotates

These are reachable today only with hand-authored absolute coordinates that
duplicate one another, which is fragile and impossible to compose with
runtime-bound motion.

## Proposed constraint family

Three new `PathType` variants in `tui-vfx-geometry/src/types/path_type.rs`,
plus a small follow-resolution pre-pass in
`tui-vfx-recipes/src/scene/layers/mod.rs` so resolved sibling rects are
available to constraint solvers before the layer's own motion is interpolated.

### 1. `PinnedOffset` — "I am always X cells from layer Y"

```rust
PathType::PinnedOffset {
    other: LayerId,
    dx: f32,
    dy: f32,
}
```

Per-frame: take the resolved animated rect of `other` for the current sample,
add `(dx, dy)`, and use the result as this layer's animated rect. The layer's
own `from`/`to` are ignored when `PinnedOffset` is the route.

Use cases:

- typewriter cursor wake that always sits 2 cells right of the active line
- companion glyph riding a moving badge
- parented mini-card on top of a host card

This is the smallest possible spatial constraint and is a strict generalisation
of today's static `sibling_id` offsets — but resolved per frame against the
sibling's animated rect, not its rest rect.

### 2. `Repulsion` — "I am always at least D cells from layer Y"

```rust
PathType::Repulsion {
    other: LayerId,
    min_distance: f32,
    stiffness: f32,    // 0.0..=1.0; how hard the deflection pushes
}
```

Per-frame post-pass over the layer's own animated rect: compute the centre-to-
centre vector to `other`'s animated rect; if the distance is below
`min_distance`, push this layer outward along that vector by `stiffness *
(min_distance - actual)`.

Use cases:

- the depth-stack swap shuffle, with a guaranteed gap so the two arcs never
  read as a collision
- two badges sharing a corner that should never overlap
- crowded toast stacks that auto-respect a personal-space zone

The math lives in `mixed-signals` (per the architectural rule of thumb: "Reusable
signal/math substrate belongs in `mixed-signals`; effect/render semantics belong
in `tui-vfx` / `tui-vfx-recipes`"). The renderer pre-computes all sibling rects
in z-order, then runs a single relaxation pass per frame so each `Repulsion`
layer sees its sibling's already-resolved animated position.

### 3. `OrbitPair` — "I rotate around layer Y at a fixed radius"

```rust
PathType::OrbitPair {
    other: LayerId,
    radius: f32,
    initial_angle: f32,    // radians
    angular_velocity: f32, // radians/sec
    direction: f32,        // +1.0 = ccw, -1.0 = cw
}
```

Per-frame: place this layer at `other.center + radius * (cos(angle), sin(angle))`
where `angle = initial_angle + direction * angular_velocity * elapsed_t`. The
layer follows the partner's animated centre while orbiting around it.

Use cases:

- async-activity badge that orbits a parent button (V3 already has the
  `figure_eight` and `carrier_orbit` *carrier* routes, but those orbit a
  static path through space — `OrbitPair` orbits a *moving* target)
- two characters that "dance" around each other while the whole pair traverses
  a route
- decorative satellites attached to dashboard tiles

## Resolution-order rules (the same gotcha as today's `sibling_id`)

Constraint variants reference another layer by id. The compositor already
sorts layers by `z` at composition start and resolves rects in that order
(`src/scene/orc_compose_scene.rs:73`). Same rule applies here:

- A constrained layer can reference a layer with a strictly lower `z`.
- Forward references (high-z → high-z, or low-z → high-z) are a validator
  error, identical in shape to the existing scene-layer `sibling_id` self/forward
  reference rule (`src/v3/validate/col_validate_scene_layers.rs`).
- Cycles are validator errors.

`Repulsion` is symmetric in concept but asymmetric in the IR: only the layer
that authored the `Repulsion` route is repelled; the other layer is treated as
fixed during this frame's resolution. Two-way repulsion is expressed by giving
both layers a `Repulsion` route pointing at each other; the single relaxation
pass handles them in z-order.

## Authoring shape

```json
{
  "id": "satellite_badge",
  "z": 10,
  "placement": {
    "type": "anchor",
    "spec": {
      "anchor": "center",
      "motion": {
        "enter": {
          "duration_ms": 0,
          "easing": "linear",
          "route": {
            "type": "orbit_pair",
            "other": "primary_button",
            "radius": 4.0,
            "initial_angle": 0.0,
            "angular_velocity": 1.5708,
            "direction": 1.0
          },
          "dynamics": [],
          "snap": { "type": "round" }
        }
      }
    }
  }
}
```

Constraint routes intentionally ignore `from` / `to` / `via` because the
position is derived from the partner layer per frame. Validator should warn if
those fields are authored alongside a constraint route.

## Out of scope for this spec

- Multi-body constraint solving (chains, ropes, n-body formations). A single
  relaxation pass per frame handles pairwise constraints; richer formations
  would need an iterative solver and authoring vocabulary for clusters.
- Soft-body deformation (cards that visually compress when repelled). The
  constraint here adjusts only the layer's resolved rect.
- Collision response between non-rectangular regions. Constraints operate on
  axis-aligned rectangles, the same surface the existing scene compositor
  already resolves.
- Velocity-based forces (springs, dampers). Today's `Spring`, `Bounce`,
  `Pendulum` route variants already handle settling motion for a single layer;
  this spec is about *spatial* relationships between layers.

## Migration / interaction with existing follow

`follow: { mode: "maintain_offset", lag_ms: N }` stays the way to express a
**temporal** delay between two animations. The new constraint variants are for
**spatial** relationships. Authors who want both — a follower that lags 200 ms
and stays at fixed offset — combine `lag_ms` with `PinnedOffset`.

Existing `sibling_id` + `offset_rows`/`offset_cols` rest-positioning stays
supported; it is the static, composition-time form of `PinnedOffset` and the
two should converge in documentation as "static (rest only)" vs "dynamic
(per-frame)" forms of the same idea.

## What recipes would land first

If this spec ever moves to active work, the natural fixture set is:

- `debug_recipes/motion_routes/relative_pinned_offset.json` — typewriter cursor
  pinned to active line; uses `PinnedOffset`.
- `debug_recipes/motion_routes/relative_repulsion_pair.json` — two cards on
  parallel arcs that maintain a 6-cell gap via `Repulsion`.
- `debug_recipes/motion_routes/relative_orbit_pair.json` — a satellite badge
  orbiting a moving primary button via `OrbitPair`.
- `recipes/5.5-suggestions/12_depth_stack_sheets_shuffle.json` (existing) —
  rewrite the swap so the two arcs are constrained by `Repulsion` instead of
  hand-tuned `arc_height` values.

<!-- <FILE>docs/design/post-release/relative-motion-spatial-constraints-spec.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
