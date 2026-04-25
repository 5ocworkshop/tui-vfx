<!-- <FILE>docs/design/tui-vfx-v3-per-cell-motion-plan.md</FILE> - <DESC>Task 23 implementation plan for V3 per-cell motion, with concrete schema homes, Rust modules, scheduler semantics, edge cases, tests, and junior-dev work packets.</DESC> -->
<!-- <VERS>VERSION: 0.3.1</VERS> -->
<!-- <WCTX>Task 23 from the TTE capability audit: define CellMotionSpec, CellPlacement, CellStagger, tui-vfx-content scheduling, V3 schema integration, and validation/debug-recipe requirements with enough specificity for implementation.</WCTX> -->
<!-- <CLOG>0.3.1: record Packet 6 debug-fixture implementation details and headless verification commands. 0.3.0: tighten junior implementation contract with exact schema wire examples, serde/default rules, signed-position semantics, motion route/dynamics lowering, timing underflow guards, StepInput/binding boundary, cache/schema/reduced-motion tests, and explicit deviations from the V3 host-motion model. 0.2.0: incorporate GPT-5.5 architecture review; remove first-slice dwell/scope-basis ambiguity; define root-vs-layer integration, unselected-cell behavior, coordinate bases, actor identity/seeding, collision/visibility tie-breaks, cache policy, typed module/function/test checklist, and junior-developer implementation order. 0.1.0: initial architecture-grounded design pass after reading steering, V3 motion/schema/compiled-plan docs, TTE motion engine, and the current tui-vfx/tui-vfx-recipes runtime seams.</CLOG> -->

# V3 per-cell motion plan

Status: Task 23 implementation contract.

This document is intentionally concrete. It should be detailed enough that a
junior developer can implement the first slice without re-deciding architecture.

## 1. Why this exists

The TTE audit identified per-cell motion as the strategic gap behind exact
MiddleOut, Slice, BinaryPath, and several future TTE-style effects. Those effects
are not only whole-rect host motion and not only per-cell colour/glyph shaders.
Each source glyph has its own authored coordinate, temporary coordinate, timing
window, and route.

`tui-vfx` should not copy TTE's stateful character engine directly. TTE creates
mutable `EffectCharacter` objects, assigns each character `Path`s, and triggers
path/scene events as those paths complete. That model works for a terminal effect
program, but it does not match V3's architecture:

- grid-first and ecosystem-agnostic;
- declarative recipe JSON;
- normalized IR → compiled execution plan → render loop;
- small StepKind algebra;
- motion meaning in tui-vfx, not downstream policy;
- probe/trace visibility at every pipeline stage;
- deterministic recipe playback.

The tui-vfx version is a deterministic content/source remapping pass:

```text
source cells
  -> stable cell actors
  -> shared cell-motion spec evaluated per actor
  -> moved semantic scene/grid
  -> normal downstream V3 pipeline
```

This mirrors the per-cell shader mindset: one shared spec, evaluated repeatedly
with per-cell context. It does not mirror TTE's mutable event engine.

## 2. Non-negotiable architectural decisions

1. **No new `StepKind`.** Per-cell motion is not `mask`, `sampler`, `filter`,
   `shader`, or `style_effect`. It happens before those visual leaves.
2. **No ratatui types.** Public/runtime seams use `tui-vfx-types` grid, cell,
   role, and scene types.
3. **Authoring selection is source-space.** Cell motion chooses source actors in
   authored/source coordinates. Downstream pipeline scopes keep their current
   rendered-coordinate behavior.
4. **Deterministic by construction.** Randomization must be seeded. Actor ids and
   ordering must be stable for a given source grid.
5. **Observable by default.** The first implementation returns stats and exposes
   enough probe/trace truth to debug actor movement.
6. **Debug recipes are required.** The primitive is not complete until base,
   option, and complex debug recipes validate.
7. **Schema-bearing types are real public surface.** Any wire-format type added
   for this plan needs serde shape, `ConfigSchema`/generated-doc coverage,
   rustdoc, and validator rules. Do not hide `cell_motion` behind
   `serde_json::Value` once typed Rust shapes exist.

## 3. Schema homes and execution order

### 3.1 First-slice schema homes

Use one reusable `CellMotionSpec`, available where cells are produced:

```text
config.content.cell_motion?       # root message/content cells only
scene.layers[*].cell_motion?      # layer-local source cells only
```

Do **not** use these homes:

- `config.motion`: already means whole recipe host/envelope motion.
- `scene.layers[*].placement.motion`: already means whole layer placement motion.
- `pipeline.step`: already means visual operations after source cells exist.

### 3.2 Wire shape and defaults

The authoring shape is ordinary V3 JSON. Keep `snake_case` field names and
`deny_unknown_fields` on typed structs/enums. Unknown keys should fail at load
time, not disappear until rendering.

Root content example:

```json
{
  "config": {
    "message": "MIDDLE OUT",
    "content": {
      "mode": "enter_only",
      "effect": { "type": "typewriter", "speed_cps": 999 },
      "cell_motion": {
        "enter": {
          "duration_ms": 700,
          "easing": "sine_in_out",
          "route": { "type": "linear" },
          "from": {
            "type": "origin",
            "anchor": "center",
            "basis": "selection_bounds"
          },
          "to": { "type": "authored" }
        }
      }
    }
  }
}
```

Scene-layer example:

```json
{
  "id": "headline",
  "role_tag": "content",
  "source": { "type": "text", "text": "SLICE" },
  "placement": { "anchor": "center", "z": 2 },
  "cell_motion": {
    "enter": {
      "duration_ms": 500,
      "easing": "quad_out",
      "route": { "type": "linear" },
      "from": { "type": "offscreen", "direction": "from_top" },
      "to": { "type": "authored" },
      "scope": { "kind": "row_range", "start": 0, "end": 1 }
    }
  },
  "pipeline": { "step": { "kind": "shader", "scope": { "kind": "all" } } }
}
```

Required defaults for `CellMotionPhaseSpec`:

| Field            | Default                      | Notes                                                                 |
| ---------------- | ---------------------------- | --------------------------------------------------------------------- |
| `duration_ms`    | required                     | `0` is valid but warned; it means immediate completion after stagger. |
| `easing`         | `linear`                     | Use the same V3 spelling as host/layer motion.                        |
| `route`          | `{ "type": "linear" }`       | Carrier path through local cell space.                                |
| `dynamics`       | `[]`                         | Motion treatments layered over `route`.                               |
| `from`           | required                     | Avoid guessing a phase-dependent default.                             |
| `via`            | `null`                       | Used by paths that need a waypoint/control point.                     |
| `to`             | required                     | Avoid guessing a phase-dependent default.                             |
| `stagger`        | `{ "type": "none" }`         | Actor-specific start delay.                                           |
| `snap`           | `{ "type": "round" }`        | Use `tui-vfx-geometry::SnappingStrategy`.                             |
| `quantize_steps` | `null`                       | If present, validator requires `>= 2`.                                |
| `collision`      | `{ "mode": "source_order" }` | Deterministic winner rule.                                            |
| `affect`         | `non_empty`                  | Empty cells are not actors by default.                                |
| `scope`          | `{ "kind": "all" }`          | Evaluated in authored/source coordinates.                             |
| `visibility`     | phase default                | See section 5.7.                                                      |

Binding boundary for Task 23:

- The recipe/loader layer may populate cell-motion fields through existing V3
  substitution or `ParamValue`/`StepInput` machinery if that machinery already
  exists for the corresponding motion type.
- `tui-vfx-content` receives a resolved typed `CellMotionSpec` for one sampled
  frame. Do **not** add an ad-hoc runtime-binding evaluator to
  `tui-vfx-content`.
- If a motion field is not yet representable as a typed binding at the V3
  authoring layer, mark that specific field as a follow-up instead of accepting
  untyped JSON in the scheduler.

### 3.3 Root content seam

`config.content.cell_motion` applies only to root content/message cells. It must
not move border, title, shadow, or other host chrome.

Required root order:

```text
root content/message source cells in content-local coordinates
  -> content.effect, if configured
  -> config.content.cell_motion, if configured
  -> composite moved content back into the root host surface
  -> paint/keep border/title/chrome in host coordinates
  -> root pipeline
```

Implementation implication: the current root source path that paints content and
border into one surface must be split enough for cell motion to run on a
content-local `SemanticScene` before host chrome is finalized.

### 3.4 Scene-layer seam

`scene.layers[*].cell_motion` applies to that layer's source cells after any
source-level content effect, before the layer-local pipeline.

Required layer order:

```text
scene layer source cells in layer-local coordinates
  -> source/content effect, if the source owns one
  -> scene.layers[*].cell_motion, if configured
  -> layer-local pipeline
  -> layer placement / scene composition
```

Layer motion and cell motion compose in this order:

```text
cell coordinate inside layer
  -> cell_motion local remap
  -> layer placement / placement.motion
  -> scene composition
  -> recipe-envelope motion
```

Do not make shadows follow individual cells in Task 23. Shadows remain attached
to host/layer envelopes. Per-cell trails and per-cell shadows are future work.

## 4. First-slice feature boundary

Task 23 implements:

- `enter` and `exit` cell motion phases;
- single-track motion per phase;
- deterministic actor extraction, placement, staggering, collision, clipping;
- root and scene-layer schema homes;
- direct V3 runtime integration;
- stats/probe/trace visibility;
- debug recipes.

Task 23 does **not** implement:

- `dwell` cell motion;
- multiple tracks per phase;
- persistent actor identity across changing content effects;
- glyph particle spawning;
- per-cell trails, ghosts, or shadows;
- cross-layer cell motion;
- random behavior without explicit seeds;
- cluster-aware wide-grapheme grouping.

### Why no `dwell` in Task 23

The current V3 motion envelope models `enter` and `exit`, while dwell timing is a
broader lifecycle/playback concern. A separate `dwell` cell-motion phase would
need its own duration semantics and loop rules. Deferring it prevents ambiguity.

If dwell cell motion is added later, it must define whether it consumes the full
dwell phase, uses its own `duration_ms`, loops, or samples absolute elapsed time.

## 5. Core Rust model

Create a dedicated module in `tui-vfx-content`:

```text
crates/tui-vfx-content/src/cell_motion/
├─ mod.rs
├─ cls_cell_actor.rs
├─ cls_cell_motion_spec.rs
├─ enum_cell_placement.rs
├─ enum_cell_stagger.rs
├─ enum_cell_collision_mode.rs
├─ cls_cell_motion_visibility.rs
├─ cls_cell_motion_stats.rs
├─ fnc_collect_cell_actors.rs
├─ fnc_resolve_cell_placement.rs
├─ fnc_resolve_actor_offset_ms.rs
├─ fnc_apply_cell_motion.rs
└─ tests or module-local unit tests
```

Export the public types from `tui-vfx-content/src/types/mod.rs` or from a new
`cell_motion` module re-export, following the crate's existing public-surface
style.

Shared imports should come from existing crates where possible:

```rust
use tui_vfx_geometry::types::{
    Anchor, EasingCurve, PathType, Position, SlideDirection, SnappingStrategy,
};
use tui_vfx_types::{Rect, RoleTag, SemanticScene};
```

Coordinate convention:

- Actor authored coordinates are in the source scene's local frame. For root
  content and scene layers this should normally be `Rect { x: 0, y: 0, ... }`.
- Intermediate and offscreen positions are signed `Position { x: i32, y: i32 }`
  from `tui-vfx-geometry`, not `u16` points. Negative coordinates are valid
  before clipping.
- Clipping converts signed positions to in-bounds `(u16, u16)` only after
  snapping and only if the coordinate lies inside `local_frame`. Do not clamp
  offscreen positions into the frame.
- If a caller passes a non-zero-origin `local_frame`, placement resolution uses
  that rect as the inclusive/exclusive frame (`x <= px < right`,
  `y <= py < bottom`). The returned scene remains local; do not translate cells
  twice during layer placement.

### 5.1 Actor identity

Build actors from the full source surface in deterministic row-major order.

```rust
pub struct CellActor {
    /// Row-major index over the full source surface, including unselected cells.
    /// Stable for a given source surface size/content generation pass.
    pub authored_index: u32,

    /// Row-major ordinal among selected actors only. Used by ByIndex staggering.
    pub selected_ordinal: u32,

    pub authored_x: u16,
    pub authored_y: u16,
    pub cell: tui_vfx_types::Cell,
    pub role: RoleTag,
}
```

Rules:

- `authored_index = y * source_width + x`.
- `selected_ordinal` increments only for actors selected by `scope + affect`.
- Seeding uses `authored_index`, not `selected_ordinal`, so selection changes do
  not silently reshuffle random placement/stagger for unchanged cells.
- `selected_ordinal` is still useful for author-facing `by_index` stagger.

### 5.2 `CellMotionSpec`

Task 23 shape:

```rust
pub struct CellMotionSpec {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub enter: Option<CellMotionPhaseSpec>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit: Option<CellMotionPhaseSpec>,
}
```

If the current sampled phase has no corresponding cell-motion phase, return the
input scene unchanged and stats with zero moved/clipped/collision counts.

### 5.3 `CellMotionPhaseSpec`

```rust
pub struct CellMotionPhaseSpec {
    pub duration_ms: u64,
    #[serde(default = "default_easing_linear")]
    pub easing: EasingCurve,
    #[serde(default = "default_route_linear")]
    pub route: PathType,
    #[serde(default)]
    pub dynamics: Vec<CellMotionDynamicSpec>,
    pub from: CellPlacement,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub via: Option<CellPlacement>,
    pub to: CellPlacement,
    #[serde(default)]
    pub stagger: CellStagger,
    #[serde(default)]
    pub snap: SnappingStrategy,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quantize_steps: Option<u32>,
    #[serde(default)]
    pub collision: CellCollisionMode,
    #[serde(default)]
    pub affect: CellMotionAffect,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<CellMotionScope>,
    #[serde(default)]
    pub visibility: CellMotionVisibility,
}
```

Notes:

- `scope` is evaluated against authored/source coordinates.
- There is no author-facing `scope_basis` in Task 23. Authored basis is implicit.
- `CellMotionScope` may initially be a small local selector or a recipe-side V3
  scope wrapper, depending on integration cost. The first implementation must at
  least support `all`, `rows`, `row_range`, `columns`, `column_range`, `rect`,
  and `cells` for debug recipes.
- `CellMotionDynamicSpec` should reuse the existing V3 motion dynamic vocabulary
  exactly. Do not create a second physics vocabulary.

If Rust crate boundaries make a literal type alias to `V3MotionDynamicSpec`
impractical in `tui-vfx-content`, define the local enum with the same variants
and one explicit conversion function. Keep field names and serde spellings
identical.

Route/dynamics lowering must match the V3 host-motion model:

```rust
fn lower_cell_motion_path(
    route: PathType,
    dynamics: Vec<CellMotionDynamicSpec>,
) -> PathType {
    let dynamics = dynamics
        .into_iter()
        .map(CellMotionDynamicSpec::into_path_type)
        .collect::<Vec<_>>();

    if dynamics.is_empty() {
        route
    } else {
        PathType::Composed {
            route: Box::new(route),
            dynamics,
        }
    }
}
```

Do not sample `route` and `dynamics` with a second bespoke math stack. The cell
motion scheduler may need a small adapter from two signed `Position`s plus an
optional `via` point into the existing `tui-vfx-geometry` path sampler, but the
path semantics and serde spellings stay shared with V3 host/layer motion.

Deliberate Task 23 deviation from host/layer motion: `relative_to`, `follow`,
`phase_offset_ms`, and `edge_crossing` are not part of `CellMotionPhaseSpec` yet.
Per-cell motion is local-source remapping, not scene-object choreography. Add
those fields later only with explicit semantics for actor-local frames.

### 5.4 `CellPlacement`

Task 23 placement enum:

```rust
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CellPlacement {
    /// The actor's original source coordinate.
    Authored,

    /// The actor's original source coordinate plus a signed offset.
    AuthoredOffset { dx: i32, dy: i32 },

    /// One absolute local-frame coordinate.
    Absolute { x: i32, y: i32 },

    /// Anchor resolved against either selected bounds or the local frame.
    Origin { anchor: Anchor, basis: CellPlacementBasis },

    /// Outside the local frame, preserving the actor's authored coordinate on
    /// the orthogonal axis when possible.
    Offscreen { direction: SlideDirection, margin_cells: u16 },
}

#[serde(rename_all = "snake_case")]
pub enum CellPlacementBasis {
    SelectionBounds,
    LocalFrame,
}
```

Defer `RandomInRect` and `RandomOffscreen` to a later packet. Task 23 does not
need them for MiddleOut/Slice, and adding them now expands the seed and bounds
surface before the deterministic scheduler is proven.

Placement resolution rules:

| Placement                            | Resolution                                                                                        |
| ------------------------------------ | ------------------------------------------------------------------------------------------------- |
| `authored`                           | `(actor.authored_x, actor.authored_y)`                                                            |
| `authored_offset`                    | authored coordinate plus signed offset                                                            |
| `absolute`                           | exact local-frame coordinate; may be outside bounds if signed values are outside                  |
| `origin { basis: selection_bounds }` | anchor within the bounding rect of selected actors; if no selected actors, no-op stats output     |
| `origin { basis: local_frame }`      | anchor within the full local motion frame                                                         |
| `offscreen` cardinal                 | just outside local frame on the named side; preserves actor authored coordinate on the other axis |
| `offscreen` diagonal                 | outside both axes; does not preserve either axis                                                  |
| `offscreen default`                  | actor authored coordinate; validator should warn because it is not useful                         |

Selection-bounds rule:

- Bounds are computed over selected actors after `affect + scope`.
- Width/height are inclusive over actor coordinates for anchor math, then
  converted to the existing `Rect`/`Position` helpers as needed.
- If the selected set is empty, skip placement resolution and return unchanged
  scene plus zero actor stats.

Offscreen rule:

- `margin_cells = 0` means one cell outside the local frame (`x = frame.x - 1`
  for left, `x = frame.right()` for right, and equivalent for y).
- Positive `margin_cells` moves farther outside: `frame.x - 1 - margin` or
  `frame.right() + margin`.
- `SlideDirection::Default` resolves to the actor authored coordinate and emits
  a validator warning; do not silently choose `from_top` for per-cell motion.

### 5.5 `CellStagger`

```rust
#[serde(tag = "type", rename_all = "snake_case")]
pub enum CellStagger {
    None,
    ByIndex { stride_ms: u64 },
    ByPosition {
        axis: CellStaggerAxis,
        direction: CellStaggerDirection,
        stride_ms: u64,
    },
    ByDistance { origin: CellPlacement, stride_ms: u64 },
    Random { seed: u64, max_offset_ms: u64 },
}
```

Definitions:

- `none`: offset `0`.
- `by_index`: `selected_ordinal * stride_ms`.
- `by_position`: rank by authored coordinate on the requested axis/direction,
  then multiply rank by `stride_ms`. Ties break by authored row-major index.
- `by_distance`: Manhattan distance from resolved `origin`, rounded to integer
  cell distance, times `stride_ms`.
- `random`: deterministic integer in `0..=max_offset_ms` from the canonical seed
  formula below.

Canonical seed formula for Task 23:

```text
actor_seed = fnv1a64(recipe_or_layer_seed, authored_index, user_seed, field_salt)
```

Where:

- `user_seed` is the `seed` field from the stagger/placement variant;
- `field_salt` is a fixed per-call-site constant, e.g. `"cell_stagger"`;
- `recipe_or_layer_seed` defaults to `0` until V3 exposes a canonical recipe
  seed, but the function signature must accept it so recipes can wire it later.

Do not call a global RNG in runtime code.

### 5.6 Affect and selection

```rust
#[serde(rename_all = "snake_case")]
pub enum CellMotionAffect {
    NonEmpty,
    All,
}
```

Defaults:

- `affect = non_empty`
- `scope = all`

`non_empty` means `!cell.is_empty()` using the public `tui_vfx_types::Cell`
contract.

Selection algorithm:

1. Iterate every source cell row-major.
2. Apply `affect`.
3. Apply authored-coordinate `scope`.
4. Selected cells become moving actors.
5. Unselected cells are copied into the baseline output unchanged.

### 5.7 Visibility

```rust
pub struct CellMotionVisibility {
    pub before_start: CellVisibilityMode,
    pub after_complete: CellVisibilityMode,
}

#[serde(rename_all = "snake_case")]
pub enum CellVisibilityMode {
    Hidden,
    AtFrom,
    AtTo,
    Hold,
}
```

Defaults by phase:

| Phase | before_start | after_complete |
| ----- | ------------ | -------------- |
| enter | hidden       | hold           |
| exit  | hold         | hidden         |

Semantics:

- `hidden`: do not write the actor.
- `at_from`: write actor at resolved `from`.
- `at_to`: write actor at resolved `to`.
- `hold`: use the last in-bounds snapped coordinate for this actor in this
  sampled phase; if no such coordinate exists, hide it. In stateless first-slice
  evaluation this is equivalent to `at_to` after completion and `at_from` before
  start unless the route clips out of bounds.

## 6. Scheduler semantics

Main function:

```rust
pub fn apply_cell_motion(
    scene: &tui_vfx_types::SemanticScene,
    spec: &CellMotionSpec,
    timing: &CellMotionTiming,
    local_frame: tui_vfx_types::Rect,
    options: &CellMotionOptions,
) -> CellMotionResult
```

If `V3PlaybackTiming` cannot live in `tui-vfx-content` because of crate
boundaries, define a small `CellMotionTiming` in `tui-vfx-content` and convert to
it from recipes/runtime:

```rust
pub struct CellMotionTiming {
    pub phase: CellMotionPhase,
    pub phase_elapsed_ms: u64,
    pub phase_t: f64,
    pub absolute_t_ms: f64,
    pub reduced_motion: bool,
    pub seed: u64,
}
```

`phase_elapsed_ms` should come from monotonic elapsed timing when available, not
from loop progress. This preserves V3's normalized-progress vs elapsed-time
separation. `phase_t` is useful for diagnostics but must not be the source for
per-actor stagger math.

Options should be explicit, not hidden globals:

```rust
pub struct CellMotionOptions {
    /// Future recipe/layer seed. Defaults to 0 until the V3 seed home lands.
    pub recipe_or_layer_seed: u64,
    /// Maximum deterministic samples to record in stats/probe output.
    pub sample_limit: usize,
}
```

Default `sample_limit` is `8`. A value of `0` disables sample collection but not
aggregate stats.

### 6.1 Algorithm

For one sampled frame:

1. Resolve active phase (`enter` or `exit`). If absent, return input unchanged.
2. Collect selected actors and baseline/unselected cells.
3. Resolve selection bounds from selected actors.
4. For every selected actor:
   1. compute `offset_ms` from `stagger`;
   2. compare `phase_elapsed_ms` with `offset_ms` before subtracting;
   3. if `phase_elapsed_ms < offset_ms`, apply `visibility.before_start`
      without underflowing unsigned integers;
   4. otherwise compute `local_elapsed_ms = phase_elapsed_ms - offset_ms`;
   5. if `duration_ms == 0`, set `local_t = 1.0` once stagger elapsed;
   6. otherwise set `local_t = clamp(local_elapsed_ms / duration_ms, 0..1)`;
   7. if `quantize_steps = Some(n)`, require `n >= 2` from validation and
      quantize `local_t` into `n` evenly spaced stops after clamping and before
      easing;
   8. ease `local_t` using `EasingCurve`;
   9. resolve `from`, optional `via`, and `to` using this actor and bounds;
   10. lower `route + dynamics` through the existing geometry path substrate;
   11. sample a position;
   12. snap using `SnappingStrategy`;
   13. clip if outside `local_frame`;
   14. write candidate into a collision bucket for its destination cell.
5. Resolve collision buckets and write winners onto the baseline output grid.
6. Return moved scene plus stats.

After completion visibility:

- If `local_elapsed_ms > duration_ms`, apply `visibility.after_complete` before
  route sampling unless the mode is `hold`.
- `hold` is stateless in Task 23: it resolves to the completed in-bounds
  destination (`to`) for ordinary routes, or hidden if the completed snapped
  coordinate is out of bounds. Do not add per-actor frame-to-frame state in the
  first slice.

### 6.2 Baseline and selected-cell clearing

Output starts as a copy of the source scene with all selected actors cleared from
their authored cells and all unselected cells preserved.

Then selected actors are written at their moved coordinates.

Consequences:

- an actor leaves its authored location unless its motion writes it back there;
- unselected cells remain visible;
- selected-vs-unselected conflicts are handled by collision policy.

### 6.3 Collision policy

```rust
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum CellCollisionMode {
    SourceOrder,
    ReverseSourceOrder,
    NearestToCompletion,
    PreserveExisting,
}
```

Definitions:

- `source_order`: among moved actors targeting the same cell, highest
  `authored_index` wins. Moved actor may overwrite an unselected baseline cell.
- `reverse_source_order`: lowest `authored_index` wins. Moved actor may overwrite
  an unselected baseline cell.
- `nearest_to_completion`: highest `local_t` wins; ties break by highest
  `selected_ordinal`, then highest `authored_index`. Moved actor may overwrite an
  unselected baseline cell.
- `preserve_existing`: if the destination already has a baseline/unselected
  non-empty cell, keep it and drop moved actors for that cell. If baseline is
  empty, use `source_order` among moved actors.

Winner propagation:

- winning actor writes its full `Cell` and `RoleTag`;
- losing actors increment `collision_count`;
- dropped actors do not partially blend style or role.

### 6.4 Reduced motion

If `timing.reduced_motion` is true:

- skip route interpolation;
- for enter, place selected actors at `to` after their stagger gate;
- for exit, hide selected actors after their stagger gate;
- preserve final content and downstream pipeline effects;
- still return stats so hosts can prove reduced-motion mode executed.

Reduced motion is a host policy input. The primitive only defines how to respond.

## 7. Result and observability

Return a result, not only a scene:

```rust
pub struct CellMotionResult {
    pub scene: tui_vfx_types::SemanticScene,
    pub stats: CellMotionStats,
}

pub struct CellMotionStats {
    pub selected_actor_count: u32,
    pub moved_actor_count: u32,
    pub clipped_actor_count: u32,
    pub collision_count: u32,
    pub baseline_overwrite_count: u32,
    pub hidden_before_start_count: u32,
    pub hidden_after_complete_count: u32,
    pub max_stagger_offset_ms: u64,
    pub min_local_t: f32,
    pub max_local_t: f32,
    pub samples: Vec<CellMotionSample>,
}

pub struct CellMotionSample {
    pub authored_index: u32,
    pub from: Position,
    pub to: Position,
    pub rendered: Option<Position>,
    pub local_t: f32,
}
```

Sampling rule: cap `samples` to a small deterministic count, e.g. first 8 moved
actors by `authored_index`, so probe output stays readable.

Stats invariants:

- `selected_actor_count` counts actors after `affect + scope`.
- `moved_actor_count` counts selected actors that produce an in-bounds rendered
  candidate, even if a later collision drops them.
- `clipped_actor_count` counts selected actors whose snapped coordinate is
  out-of-bounds or whose visibility mode hides an out-of-bounds hold.
- `collision_count` counts moved actors dropped because another moved actor won
  the same destination.
- `baseline_overwrite_count` counts non-empty unselected baseline cells replaced
  by a moved actor under non-`preserve_existing` collision modes.
- `min_local_t` and `max_local_t` are `0.0` when no actor reaches active
  interpolation in the sampled frame.

Probe/trace requirements:

- expose `CellMotionStats` in pipeline-validator/probe output;
- add either a dedicated `CellMotionSummary` trace event or cell-motion fields
  to the existing layer/root completion event;
- include enough truth-surface detail to verify root content motion did not move
  border/title chrome.

## 8. Cache policy

Any root content surface or scene layer with active `cell_motion` is
cache-dynamic.

Minimum safe first implementation:

- bypass existing layer/source cache when `cell_motion` is present.

Future optimization may include timing/seed/spec fingerprints in cache keys, but
Task 23 should prefer correctness and observability over cache reuse.

## 9. Edge cases with required behavior

| Case                                | Required behavior                                                                                                                         |
| ----------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------- |
| No active phase spec                | Return input unchanged; stats zero.                                                                                                       |
| No selected actors                  | Return input unchanged; stats zero except phase metadata if recorded.                                                                     |
| Unselected cells                    | Copy through unchanged as baseline.                                                                                                       |
| Selected actor at authored location | Clear authored cell first, then possibly write actor back if resolved destination is authored.                                            |
| `duration_ms = 0`                   | No division by zero. Once stagger elapsed, enter writes `to`; exit hides.                                                                 |
| `quantize_steps < 2`                | Validator error; do not let runtime guess whether `1` means always start or always end.                                                   |
| `phase_elapsed_ms < stagger`        | Use before-start visibility without unsigned underflow.                                                                                   |
| `stagger > phase duration`          | Actor may never start; count hidden-before-start.                                                                                         |
| Out-of-bounds snapped coordinate    | Clip actor; count clipped. No wrap.                                                                                                       |
| Wide grapheme / multi-cell glyph    | First slice treats already-expanded cells independently. Cluster-aware grouping deferred.                                                 |
| Dynamic content changes actor order | Actor table rebuilds from produced source grid each sample. Document possible visual jumps with randomized/stagger-heavy content effects. |
| Role tags                           | Winning moved actor preserves its original role tag.                                                                                      |
| Empty cell with `affect=non_empty`  | Not selected.                                                                                                                             |
| Empty cell with `affect=all`        | Selected and may move as an empty/styled cell; debug recipes should avoid this unless intentional.                                        |
| Root border/title                   | Never selected by `config.content.cell_motion`; prove with a test and debug recipe.                                                       |
| Scene-layer surface/shadow          | Cell motion affects layer source cells, not attached layer shadow geometry.                                                               |
| Reduced motion                      | Skip route interpolation but preserve final content and downstream pipeline effects.                                                      |
| Non-zero local frame origin         | Resolve placements against the frame but keep returned scene local; avoid double translation during scene placement.                      |

## 10. Implementation checklist

### Packet 0 — contract lock

Before writing runtime code, update this document or a short implementation issue
with any deviations. Do not proceed if any of these change without review:

- `enter/exit` only in Task 23;
- no author-facing `scope_basis`;
- no new `StepKind`;
- unselected cells copy through unchanged;
- selected cells clear authored positions before moved writes;
- random behavior uses canonical seed formula;
- cache bypass when `cell_motion` exists.

### Packet 1 — `tui-vfx-content` pure types and scheduler

Files to add:

```text
crates/tui-vfx-content/src/cell_motion/mod.rs
crates/tui-vfx-content/src/cell_motion/cls_cell_actor.rs
crates/tui-vfx-content/src/cell_motion/cls_cell_motion_spec.rs
crates/tui-vfx-content/src/cell_motion/enum_cell_placement.rs
crates/tui-vfx-content/src/cell_motion/enum_cell_stagger.rs
crates/tui-vfx-content/src/cell_motion/enum_cell_collision_mode.rs
crates/tui-vfx-content/src/cell_motion/cls_cell_motion_visibility.rs
crates/tui-vfx-content/src/cell_motion/cls_cell_motion_stats.rs
crates/tui-vfx-content/src/cell_motion/fnc_collect_cell_actors.rs
crates/tui-vfx-content/src/cell_motion/fnc_resolve_cell_placement.rs
crates/tui-vfx-content/src/cell_motion/fnc_resolve_actor_offset_ms.rs
crates/tui-vfx-content/src/cell_motion/fnc_apply_cell_motion.rs
```

Manifest update:

- add the workspace-internal `tui-vfx-geometry` dependency to
  `crates/tui-vfx-content/Cargo.toml` if it is not already present. This is not
  permission to add a new third-party dependency.

Public exports:

- add `pub mod cell_motion;` in `crates/tui-vfx-content/src/lib.rs`;
- re-export core author-facing types from `prelude.rs` or `types/mod.rs` if that
  is the crate's current convention for content primitives.

Functions to implement:

```rust
collect_cell_actors(scene, phase_spec) -> (Vec<CellActor>, BaselineScene)
resolve_cell_motion_phase(spec, phase) -> Option<&CellMotionPhaseSpec>
resolve_cell_placement(actor, placement, ctx) -> Position
resolve_actor_offset_ms(actor, stagger, ctx) -> u64
sample_actor_position(from, via, to, eased_t, route, dynamics, snap) -> Position
apply_cell_motion(scene, spec, timing, local_frame, options) -> CellMotionResult
```

Tests first:

- `cell_motion_middle_out_center_to_authored`
- `cell_motion_unselected_cells_remain_unchanged`
- `cell_motion_selected_cells_vacate_authored_positions`
- `cell_motion_collision_source_order`
- `cell_motion_collision_preserve_existing`
- `cell_motion_nearest_to_completion_tie_breaks`
- `cell_motion_zero_duration_enter_teleports_to_to`
- `cell_motion_zero_duration_exit_hides`
- `cell_motion_stagger_longer_than_phase_hides_before_start`
- `cell_motion_random_stagger_is_deterministic`
- `cell_motion_clips_out_of_bounds`
- `cell_motion_preserves_role_tags`
- `cell_motion_non_empty_uses_cell_is_empty_contract`
- `cell_motion_wide_grapheme_smoke_currently_cell_based`
- `cell_motion_reduced_motion_enter_places_to_after_stagger`
- `cell_motion_reduced_motion_exit_hides_after_stagger`
- `cell_motion_quantize_steps_rejects_zero_and_one`
- `cell_motion_no_unsigned_underflow_before_stagger`
- `cell_motion_stats_count_baseline_overwrites`

### Packet 2 — recipe schema and V3 DTO propagation

Add typed schema fields in `tui-vfx-recipes`:

```rust
// src/recipe_schema/config.rs
pub struct VfxContentConfig {
    // existing fields...
    pub cell_motion: Option<CellMotionSpec>,
}

// src/recipe_schema/scene/cls_ra_scene_layer.rs
pub struct VfxSceneLayer {
    // existing fields...
    pub cell_motion: Option<CellMotionSpec>,
}
```

Carry through the V3 stack:

- `V3Config` / authoring config;
- `V3SceneLayer`;
- `NormalizedEnvelope` or normalized root content holder;
- `NormalizedLayer`;
- `CompiledEnvelope` or compiled root content holder;
- `CompiledLayerPlan`.

Avoid leaving `cell_motion` as opaque `serde_json::Value` once the content crate
types exist. This is schema-bearing behavior and must meet V3 doc/schema
standards.

Tests:

- V3 parser accepts root `config.content.cell_motion`;
- V3 parser accepts `scene.layers[*].cell_motion`;
- V3 parser rejects unknown fields inside `cell_motion` objects;
- V3 parser applies defaults for `easing`, `route`, `dynamics`, `snap`,
  `stagger`, `collision`, `affect`, `scope`, and `visibility`;
- V3 parser rejects `quantize_steps` values below `2`;
- normalized IR preserves root/layer cell motion as typed data;
- compiled plan preserves root/layer cell motion as typed data;
- generated schema/API docs mention `cell_motion`;
- if runtime bindings are supported for reused motion fields, one parser/runtime
  test proves a binding resolves before the content scheduler sees the spec; if
  not supported yet, the schema docs explicitly mark binding-backed cell motion
  as deferred.

### Packet 3 — root runtime integration

Implement root content integration carefully:

1. Build content-local scene from message/content after content effect.
2. Run `config.content.cell_motion` on that content-local scene.
3. Composite moved content into the host/root surface.
4. Keep border/title/chrome fixed.
5. Run root pipeline as before.

Tests:

- root content moves while border/title stay fixed;
- downstream shader/filter scopes apply to rendered moved cells;
- content effect → cell motion → root pipeline ordering.

### Packet 4 — scene-layer runtime integration

Implement layer integration:

1. Paint layer source cells into a layer-local `SemanticScene`.
2. Run source/content effect first if present.
3. Run `layer.cell_motion`.
4. Preserve roles.
5. Run layer-local pipeline.
6. Compose layer into scene as before.
7. Bypass layer cache when `cell_motion` exists.

Tests:

- layer cell motion runs before layer-local shader/filter;
- layer role tags survive movement;
- cache is bypassed or made timing-aware;
- cache bypass is active even when the sampled phase lacks a matching
  `enter`/`exit` spec, because the first safe policy keys off presence of
  `cell_motion`;
- layer placement motion still moves the already-cell-motioned layer.

### Packet 5 — observability and validation

Add:

- `CellMotionStats` in probe/validator truth surfaces;
- trace summary event or layer completion extension;
- awareness in every validator/tooling/player surface that can load or render V3
  recipes, including `pipeline-validator`, `recipe-probe`, `tui-vfx-trace`,
  `tui-vfx-horseman`, demo/play_recipe preview paths, and any direct dump/player
  helpers in `tui-vfx` or `tui-vfx-recipes`;
- validator warnings for:
  - `duration_ms = 0`;
  - `offscreen default` placement;
  - `affect=all` with mostly empty content;
  - unsupported scope forms in first slice;
  - `cell_motion` on a cached layer without cache bypass.

Tests:

- probe JSON includes actor/move/clip/collision/baseline-overwrite counts;
- probe JSON sample list is deterministic and obeys `sample_limit`;
- debug-recipes QC sees cell-motion fixtures as supported;
- validator/player/preview command surfaces accept cell-motion recipes without
  falling back to legacy or silently dropping the motion stage;
- trace report includes the cell-motion summary;
- reduced-motion probe run reports that reduced-motion branch executed while
  preserving downstream pipeline output.

### Packet 6 — debug recipes and docs

Add debug recipes in `tui-vfx-recipes`:

```text
recipes/debug_recipes/content/content_cell_motion_middle_out.json
recipes/debug_recipes/content/content_cell_motion_slice.json
recipes/debug_recipes/content/content_cell_motion_root_border_fixed.json
recipes/debug_recipes/complex/complex_cell_motion_shader_pipeline.json
```

Recipe requirements:

- MiddleOut: proves `origin(selection_bounds center) -> authored`.
- Slice: proves scoped top/bottom offscreen entry. If one track cannot express
  this cleanly, use two scene/text layers in the first fixture and record
  multi-track as follow-up.
- Root border fixed: proves `config.content.cell_motion` does not move border.
- Complex: proves moved glyphs then downstream visual pipeline, preferably with
  `AnimatedGlyphRamp` or a shader/filter chain.

Implemented first-slice fixture shape:

- `content_cell_motion_middle_out.json` uses root `config.content.cell_motion`
  from `origin(selection_bounds center)` to `authored` with distance stagger.
- `content_cell_motion_slice.json` uses two scene text layers because one phase
  has one track: upper from `offscreen from_top`, lower from `offscreen
  from_bottom`.
- `content_cell_motion_root_border_fixed.json` keeps a titled double border fixed
  while only root content glyphs enter from offscreen.
- `complex_cell_motion_shader_pipeline.json` moves root content glyphs first, then
  runs a downstream `linear_gradient` shader plus `animated_glyph_ramp` filter.

Docs to update:

- this file;
- `docs/design/tui-vfx-v3-schema-overview.md`;
- `docs/design/tui-vfx-v3-recipe-vocabulary.md`;
- `docs/design/tui-vfx-v3-compiled-execution-plan.md`;
- `tui-vfx-recipes/docs/V3_TOOLING_COMMAND_REFERENCE.md`;
- `tui-vfx-recipes/docs/V3_STANDALONE_PREVIEW_SURFACES.md`;
- sibling `tui-vfx-recipes` authoring/schema docs;
- generated docs in both repos.

## 11. TTE mapping for Task 23

### MiddleOut

First-slice approximation:

```json
"cell_motion": {
  "enter": {
    "duration_ms": 700,
    "easing": "sine_in_out",
    "route": { "type": "linear" },
    "from": { "type": "origin", "anchor": "center", "basis": "selection_bounds" },
    "to": { "type": "authored" },
    "stagger": { "type": "none" },
    "affect": "non_empty"
  }
}
```

Exact TTE MiddleOut is two-stage: center → centerline, then centerline →
authored. Task 23 builds the substrate. Exact two-stage choreography requires
multi-track or phase-internal sequencing and is a follow-up.

### Slice

Task 23 can prove Slice with two scoped scene/text layers:

- upper layer: `from offscreen from_top -> authored`;
- lower layer: `from offscreen from_bottom -> authored`.

A later multi-track `cell_motion.tracks[]` can express this in one source layer.
Do not block the first scheduler on multi-track choreography.

### BinaryPath

Task 23 moves existing source actors. It does not spawn extra binary glyphs.
BinaryPath still requires Task 24's glyph particle emitter. The emitter should
reuse the same actor/scheduler substrate.

## 12. Validation baseline

From `/usr/projects/tui-vfx`:

```sh
cargo test -p tui-vfx-content cell_motion -- --nocapture
cargo test -p tui-vfx-geometry motion -- --nocapture
just docs-all-check
git diff --check
```

From `/usr/projects/tui-vfx-recipes`:

```sh
cargo test -p tui-vfx-recipes cell_motion -- --nocapture
cargo run -q -p pipeline-validator -- --rules --stages \
  recipes/debug_recipes/content/content_cell_motion_middle_out.json \
  recipes/debug_recipes/content/content_cell_motion_slice.json \
  recipes/debug_recipes/content/content_cell_motion_root_border_fixed.json \
  recipes/debug_recipes/complex/complex_cell_motion_shader_pipeline.json
cargo run -q -p pipeline-validator -- --debug-recipes-qc --format json \
  recipes/debug_recipes/content/content_cell_motion_middle_out.json \
  recipes/debug_recipes/content/content_cell_motion_slice.json \
  recipes/debug_recipes/content/content_cell_motion_root_border_fixed.json \
  recipes/debug_recipes/complex/complex_cell_motion_shader_pipeline.json
cargo run -q -p tui-vfx-horseman -- --json \
  recipes/debug_recipes/content/content_cell_motion_middle_out.json
just docs-v3-check
git diff --check
```

A full task completion report must list:

- changed files;
- new schema fields;
- debug recipes added;
- validator/probe evidence;
- known limitations, especially no dwell, no multi-track, no particles.

## 13. Follow-ups after Task 23

- `dwell` cell motion with explicit loop/duration semantics.
- Multi-track per-cell choreography for exact MiddleOut and one-layer Slice.
- Cluster-aware wide-grapheme actor grouping.
- Per-cell trails/ghosts.
- Per-cell motion output hints (`cell_motion.progress`, `cell_motion.displacement`).
- Task 24 glyph particle emitter on the same actor/scheduler substrate.
- Optimized timing-aware cache keys after correctness is proven.

<!-- <FILE>docs/design/tui-vfx-v3-per-cell-motion-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
