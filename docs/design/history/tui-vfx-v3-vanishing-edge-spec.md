# tui-vfx V3 vanishing-edge spec

Status: draft

## Purpose

Define vanishing-edge behavior as a motion-owned viewport-edge treatment.

The effect is visual, but it exists because a moving object is partially clipped by the screen boundary. That makes motion the correct owner.

This document also assumes the resolved typed motion/shadow model. Any
tokenization or substitutions that populate motion fields happen earlier at the
`tui-vfx-recipes` API boundary and are not part of vanishing-edge semantics.

## Problem statement

When a moving toast or panel is partially offscreen, its leading border edge can punch past the viewport boundary and break the silhouette.

If that toast also has a transparent shadow, the system has to know that the shadow belongs to the same moving host. Otherwise border trim and shadow handling diverge and the entrance looks mechanically wrong.

The same boundary interaction also affects:
- shadows
- directional arrivals and exits
- multi-layer moving scenes
- curved motion paths whose leading edge changes over time

So this cannot stay only as a V2-style border-trim flag.

## Recommended schema home

Vanishing-edge behavior lives inside each motion phase:

```json
"edge_crossing": {
  "edge": "left" | "right" | "top" | "bottom",
  "border": "vanish" | "preserve",
  "shadow": "fade" | "clip" | "preserve"
}
```

At recipe scope:

```json
{
  "config": {
    "motion": {
      "enter": {
        "edge_crossing": { "edge": "right", "border": "vanish", "shadow": "fade" }
      }
    }
  }
}
```

At scene-layer scope:

```json
{
  "scene": {
    "layers": [
      {
        "placement": {
          "motion": {
            "enter": {
              "edge_crossing": { "edge": "right", "border": "vanish", "shadow": "fade" }
            }
          }
        }
      }
    ]
  }
}
```

## Host-bound shadow glue

Vanishing-edge math should not talk directly to a free-floating shadow config.

Instead, the runtime should receive a library-owned host bundle:
- host rect
- active border geometry
- optional attached shadow
- current visual envelope

Then vanishing-edge can answer three questions coherently:
1. which host edge is clipped
2. whether border cells on that edge should vanish
3. how the attached shadow should fade or clip relative to that same edge

That avoids app-layer hand wiring.

## Edge-crossing object

Vanishing-edge semantics need one explicit object that carries:
- the viewport edge being crossed for the active phase
- the border behavior at that edge
- the shadow behavior at that edge

Recommended motion fields:

```json
"edge_crossing": {
  "edge": "left" | "right" | "top" | "bottom",
  "border": "vanish" | "preserve",
  "shadow": "fade" | "clip" | "preserve"
}
```

This may be authored directly or normalized from `from` / `to` placements plus the chosen edge behavior.

Why they matter:
- border trimming needs to know which edge is the active clipped host edge
- shadow fading needs to align to that same edge
- curved motion should still be able to preserve stable author intent when tangent-based inference would be noisy

These are part of motion semantics, not generic border styling.

## Directional awareness is mandatory

Vanishing edge must track the actual edge of the moving host that is clipped right now.

That means:
- left-entry and right-entry cannot share the same trim assumption
- top-entry and bottom-entry cannot share the same trim assumption
- curved motion cannot assume one fixed axis for the whole transition

### Resolution rule

Determine the active clipped edge in this order:
1. explicit `edge_crossing.edge` for the active phase
2. instantaneous motion tangent
3. vector from resolved `from` to resolved `to`
4. explicit offscreen direction
5. previous-frame rect delta fallback

The winning direction tells the runtime:
- which border edge is leading
- which edge should vanish or preserve
- which shadow edge should fade, clip, or preserve

## Border behavior

### `border = "vanish"`
- blank the leading clipped border edge while clipped
- preserve the rest of the border
- restore the authored border as soon as that edge is fully inside the viewport

This is the recommended default.

### `border = "preserve"`
- keep the authored border even while clipped

Use only for deliberate hard-silhouette cases.

## Shadow behavior

The shadow behavior below assumes the shadow is attached to the moving host and evaluated against the same active clipped edge.

### `shadow = "fade"`
- attenuate shadow coverage near the clipped edge
- best default for natural offscreen arrivals

### `shadow = "clip"`
- hard-clip the shadow at the viewport boundary

### `shadow = "preserve"`
- preserve full shadow behavior even while partially offscreen
- intentionally stylized, not a default

## Relation to shadow ownership

Vanishing edge does not define shadow rendering style.

Instead:
- shadow spec defines the shadow itself
- motion `edge_crossing.shadow` defines what happens to that shadow at the viewport boundary during motion

See `docs/design/tui-vfx-v3-shadow-spec.md`.

## Relation to V2

V2 currently expresses only one slice of this through:
- `border.trim = vanishing_edge | none`

V3 should preserve the behavior but widen the model so it can also drive directional shadow handling and curved-path motion.

### Compatibility mapping

- `vanishing_edge` -> `edge_crossing.border = vanish`
- `none` -> `edge_crossing.border = preserve`

## Toast example

For a toast with a transparent shadow entering from the right:
- motion resolves the host rect over time
- the attached shadow extends the visual envelope to the lower-right
- the viewport clips that envelope
- `edge_crossing.border = vanish` trims the host's right-leading border edge
- `edge_crossing.shadow = fade` attenuates the shadow coverage that would otherwise visibly punch through the screen boundary

This should happen entirely in the library runtime. The app should only author motion + shadow + screen-edge policy.

## Defaults

Recommended defaults:
- `border = vanish`
- `shadow = fade`

Why:
- they match the common toast / drawer expectation
- they align with smooth offscreen entrance
- they work with optional shadow instead of assuming shadow is always present

## Non-goals

Vanishing edge is not responsible for:
- choosing border style
- choosing shadow style
- deciding z order
- reduced-motion policy

It only controls how moving geometry behaves at the screen edge while clipped.


## Suggested implementation consequence

Subsequent runtime work should avoid three separate calculations for:
- border trim
- host clipping
- shadow clipping

Instead it should run one shared screen-edge pass over the compiled motion host / visual envelope model, then derive:
- border trim output
- shadow fade / clip output
- probe / validator diagnostics

from that single decision.
