<!-- <FILE>docs/design/tui-vfx-v3-motion-compatibility-table.md</FILE> - <DESC>Track C1 compatibility map for lowering V2 motion_path, offscreen placements, and vanishing-edge behavior into the V3 motion route/dynamics/from/to/edge_crossing model.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Close Track C1 with one durable, variant-by-variant motion compatibility table so fixtures and implementation packets can proceed without rediscovering PathType/offscreen/edge policy.</WCTX> -->
<!-- <CLOG>0.1.0: initial Track C1 compatibility table. Covers V2 motion fields, all current PathType variants, offscreen/from/to lowering, edge-crossing normalization, diagonal-edge caveats, and smallest safe follow-up packets.</CLOG> -->

# tui-vfx V3 motion compatibility table

Status: draft

## Purpose

This document closes Track `C1-MOTION-MAP` from
[`tui-vfx-v3-execution-dag.md`](tui-vfx-v3-execution-dag.md).

It records one canonical compatibility map for lowering current V2 motion data
into the V3 motion tree:

- `pipeline.{enter,exit}.motion_path`
- `pipeline.{enter,exit}.{from,to}`
- `pipeline.{enter,exit}.snapping`
- `pipeline.{enter,exit}.quantize_steps`
- `border.trim` when the recipe is using vanishing-edge slide behavior

The goal is not to redesign the motion engine here. The goal is to make the
current V2 → V3 lowering contract explicit enough that fixture work,
validator/canonicalizer work, and runtime packets can proceed in parallel.

## Inputs reviewed

This table is grounded in:

- `steering/INTENTIONS.md`
- `docs/design/tui-vfx-v3-execution-dag.md`
- `docs/design/tui-vfx-v3-motion-spec.md`
- `docs/design/tui-vfx-v3-shadow-spec.md`
- `docs/design/tui-vfx-v3-vanishing-edge-spec.md`
- `docs/design/tui-vfx-v3-upgrade-plan/57_v2_to_v3_lowering_rules.md`
- `docs/design/tui-vfx-v3-upgrade-plan/80_open_questions.md`
- `crates/tui-vfx-geometry/src/types/path_type.rs`
- `crates/tui-vfx-geometry/src/types/cls_motion_spec.rs`
- `crates/tui-vfx-geometry/src/types/cls_placement_spec.rs`
- `crates/tui-vfx-geometry/src/types/slide_direction.rs`
- `crates/tui-vfx-geometry/src/transitions/col_interpolate_position.rs`
- `crates/tui-vfx-geometry/src/borders/fnc_vanishing_edge_trim_spec.rs`
- `docs/v2-spec-archive/recipes-rust-source/recipe_schema/config.rs`
- `mixed-signals` route-treatment / physics substrate surfaced by OFPF orientation

## Canonical lowering rules

1. **Lift V2 phase motion into `config.motion.<phase>`.**
   - `pipeline.enter.*` lowers to `config.motion.enter.*`
   - `pipeline.exit.*` lowers to `config.motion.exit.*`
2. **Use one carrier route plus zero-or-more dynamics.**
   - if a V2 `PathType` is a true geometric carrier, lower it into `route`
   - if a V2 `PathType` is a treatment layered over travel, lower it into
     `dynamics[]` and inject `route = linear` when no carrier was authored
3. **Carry typed placements forward verbatim.**
   - `from` lowers to `motion.<phase>.from`
   - `to` lowers to `motion.<phase>.to`
   - `PlacementSpec::Offscreen { direction, margin_cells }` survives as the V3
     placement value; no toast-specific wrapper is needed
4. **Keep `quantize_steps` separate from geometric step motion.**
   - `quantize_steps` remains `motion.<phase>.quantize_steps`
   - `PathType::Step { steps }` is a geometric/dynamic treatment, not the same
     concept as temporal quantization
5. **Lower V2 vanishing-edge intent into `edge_crossing`.**
   - `border.trim = vanishing_edge` → `edge_crossing.border = vanish`
   - `border.trim = none` → `edge_crossing.border = preserve`
   - V2 has no explicit shadow edge policy, so V3 uses the motion-spec default:
     `edge_crossing.shadow = fade` when edge crossing is active
6. **Flatten composed paths recursively.**
   - `PathType::Composed { route, dynamics }` lowers by recursively lowering the
     nested `route`, then appending lowered `dynamics`
   - if the nested `route` lowers to dynamics-only, inject `route = linear`
7. **Compiled edge state may need to be richer than authored edge state.**
   - current V3 authoring uses singular `edge_crossing.edge`
   - current V2 diagonal offscreen + vanishing-edge behavior can involve two
     simultaneously clipped edges
   - authoring may stay singular, but compiled edge resolution should be
     allowed to expand to multiple active edges when placements demand it

## Field-level compatibility map

| V2 surface | Canonical V3 home | Lowering | Notes |
|---|---|---|---|
| `pipeline.<phase>.duration_ms` | `config.motion.<phase>.duration_ms` | direct carry | Matches Chapter 57 and motion spec. |
| `pipeline.<phase>.easing` | `config.motion.<phase>.easing` | direct carry | Pure temporal curve. |
| `pipeline.<phase>.motion_path = None` | `config.motion.<phase>.route`, `dynamics[]` | `route = linear`, `dynamics = []` | Preserves current `unwrap_or(PathType::Linear)` runtime behavior. |
| `pipeline.<phase>.motion_path = Some(path)` | `config.motion.<phase>.route`, `dynamics[]` | classify per table below | The compatibility table below is the authoritative per-variant mapping. |
| `pipeline.<phase>.from` | `config.motion.<phase>.from` | direct carry | Common case: enter-from offscreen. |
| `pipeline.<phase>.to` | `config.motion.<phase>.to` | direct carry | Common case: exit-to offscreen. |
| `pipeline.<phase>.snapping` | `config.motion.<phase>.snap` | direct carry | No semantic change. |
| `pipeline.<phase>.quantize_steps` | `config.motion.<phase>.quantize_steps` | direct carry | Do not fold into `PathType::Step`. |
| API/runtime `MotionSpec::via` | `config.motion.<phase>.via` | direct carry when present | V2 recipe JSON does not currently expose `via`, but the geometry type already does. |
| `border.trim = vanishing_edge` | `config.motion.<phase>.edge_crossing.border` | `vanish` | Applies to the active motion phase while clipped. |
| `border.trim = none` | `config.motion.<phase>.edge_crossing.border` | `preserve` | Lossless for current border behavior. |
| V2 implicit shadow edge behavior | `config.motion.<phase>.edge_crossing.shadow` | default `fade` | V2 has no separate authored shadow-edge flag. |

## `PathType` compatibility table

### Carrier-route families

| Current `PathType` | V2 role today | Canonical V3 route | Canonical V3 dynamics | Status | Notes |
|---|---|---|---|---|---|
| `Linear` | carrier route | `linear` | `[]` | lossless | Baseline route. |
| `Arc { bulge }` | carrier route | `arc { bulge }` | `[]` | lossless | Preserves current arc-bulge behavior. |
| `Bezier { control_x, control_y }` | carrier route | `bezier { control_x, control_y }` | `[]` | lossless | If a caller originated from `MotionSpec::via`, preserve `via` too; archived V2 recipe JSON only preserves the resolved control point. |
| `Rectilinear { x_first }` | carrier route | `rectilinear { x_first }` | `[]` | lossless | Fits the V3 route split directly. |
| `Spiral { rotations }` | carrier route | `spiral { rotations }` | `[]` | route-extension | Executable today; V3 route-family list should explicitly include `spiral`. |
| `FigureEight { width, height, phase }` | carrier route | `figure_eight { width, height, phase }` | `[]` | route-extension | Executable today via mixed-signals route math; authoring aliases `infinity` / `lemniscate` remain naming policy, not lowering policy. |

### Dynamics / treatments over a carrier route

| Current `PathType` | V2 role today | Canonical V3 route | Canonical V3 dynamics | Status | Notes |
|---|---|---|---|---|---|
| `Spring { stiffness, damping }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[spring { stiffness, damping }]` | lossless | Canonical example of route/treatment separation. |
| `Bounce { bounces, decay }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[bounce { bounces, decay }]` | lossless | Preserve bounce parameters directly. |
| `Friction { drag }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[friction { drag }]` | lossless | Preserve drag directly. |
| `Projectile { arc_height, gravity }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[projectile { arc_height, gravity }]` | lossless | The current implementation applies a parabolic Y offset over a base segment, so it belongs in `dynamics[]`. |
| `Pendulum { amplitude, oscillations, damping }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[pendulum { amplitude, oscillations, damping }]` | lossless | Current implementation is a perpendicular oscillation over the segment. |
| `Orbit { revolutions, direction }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[orbit { revolutions, direction }]` | lossless | Initial V3 dynamic family already names orbit. |
| `Attractor { target_x, target_y, strength }` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[attractor { target_x, target_y, strength }]` | lossless | Matches the current endpoint-preserving pull implementation. |
| `Hover` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[hover]` | lossless | Current interpolation is linear; keeping it in `dynamics[]` preserves the semantic slot for future runtime detail. |
| `Squash` | dynamic treatment encoded as a path | `linear` if no carrier authored | `[squash]` | lossless | Same rule as `hover`: keep the named treatment even if current interpolation is simple. |
| `Step { steps }` | geometric stepping treatment | `linear` if no carrier authored | `[step { steps }]` | lossless | Keep separate from `quantize_steps`. |
| `Swirl { rotations, radius, direction }` | carrier-relative vortex treatment | `linear` if no carrier authored | `[swirl { rotations, radius, direction }]` | dynamic-extension | The route helper preserves endpoints and behaves like a route treatment around a carrier. |
| `CarrierOrbit { rotations, radius, phase, direction }` | carrier-relative helix treatment | `linear` if no carrier authored | `[carrier_orbit { rotations, radius, phase, direction }]` | dynamic-extension | This is the substrate name; recipe-level `helix` remains an alias if kept in the schema. |

### Recursive / already-composed form

| Current `PathType` | Canonical V3 lowering | Status | Notes |
|---|---|---|---|
| `Composed { route, dynamics }` | lower `route` recursively, append lowered `dynamics`, inject `route = linear` if the recursive route lowers to dynamics-only | lossless | This matches current `interpolate_position()` behavior, which treats dynamics as offsets from a baseline route. |

## Offscreen / placement / edge-crossing map

### Offscreen placements

| V2 placement | Canonical V3 lowering | Edge normalization | Notes |
|---|---|---|---|
| `from = { type: offscreen, direction: from_left, margin_cells }` | `motion.<phase>.from = { type: offscreen, direction: from_left, margin_cells }` | default `edge_crossing.edge = left` when the active phase is clipped | Lossless. |
| `from_right` | same typed placement | default edge `right` | Lossless. |
| `from_top` | same typed placement | default edge `top` | Lossless. |
| `from_bottom` | same typed placement | default edge `bottom` | Lossless. |
| `from_top_left` | same typed placement | compiled active edges should include `top` and `left` | **Needs compiled-edge support beyond singular authored edge.** Current V2 trim helper can blank both edges/corners. |
| `from_top_right` | same typed placement | compiled active edges should include `top` and `right` | Same diagonal caveat. |
| `from_bottom_left` | same typed placement | compiled active edges should include `bottom` and `left` | Same diagonal caveat. |
| `from_bottom_right` | same typed placement | compiled active edges should include `bottom` and `right` | Same diagonal caveat. |
| `direction = default` | same typed placement | do not author `edge_crossing.edge`; infer from tangent / endpoint vector / placement resolution order | Matches current `SlideDirection::Default` intent. |
| `to = { type: offscreen, ... }` | `motion.<phase>.to = { type: offscreen, ... }` | same edge rules as `from` | Common exit case. |

### V2 border trim → V3 edge-crossing policy

| V2 behavior | Canonical V3 lowering | Lossless? | Notes |
|---|---|---|---|
| `border.trim = vanishing_edge` + cardinal offscreen motion | `edge_crossing.border = vanish` | yes | Matches the current single-edge trim helper. |
| `border.trim = none` + any offscreen motion | `edge_crossing.border = preserve` | yes | No trim. |
| `border.trim = vanishing_edge` + diagonal offscreen motion | `edge_crossing.border = vanish`, but compiled resolution must be able to activate two edges | not fully, with current singular authored edge | This is the smallest real semantic gap still visible in the V2 → V3 map. |
| V2 shadow handling during edge crossing | `edge_crossing.shadow = fade` by default | partial widening, not loss | V3 is richer here; V2 had no distinct authored shadow-edge mode. |

## Smallest safe follow-up packets

### Smallest safe doc follow-up

Keep the motion spec's family lists aligned with the current executable catalog:

1. add `spiral` and `figure_eight` to the explicit V3 route-family list
2. add `swirl` and `carrier_orbit` to the explicit V3 dynamics list
3. link this table from the motion spec and lowering-rules chapter

This is doc-only, low risk, and removes a real source of future drift.

### Smallest safe implementation/test follow-up

If a narrowly-scoped runtime packet is opened next, the smallest safe one is:

**compiled diagonal edge resolution parity**

- normalize diagonal offscreen placements into a compiled active-edge set
  rather than forcing everything through singular authored `edge_crossing.edge`
- add focused geometry/runtime tests for:
  - `from_top_left`
  - `from_top_right`
  - `from_bottom_left`
  - `from_bottom_right`
- prove that border-trim output matches the current
  `vanishing_edge_trim_spec()` diagonal behavior before broader V3 fixture work

That packet is smaller and safer than starting with a full motion rewrite because
it isolates the only still-lossy part of the compatibility map.

<!-- <FILE>docs/design/tui-vfx-v3-motion-compatibility-table.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
