<!-- <FILE>docs/design/tui-vfx-v3-per-cell-motion-plan.md</FILE> - <DESC>Task 23 design plan for V3 per-cell motion, including schema home, runtime model, edge cases, and implementation packets.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Task 23 from the TTE capability audit: design CellMotionSpec, CellPlacement, CellStagger, tui-vfx-content scheduling, and V3 schema integration before implementation.</WCTX> -->
<!-- <CLOG>0.1.0: initial architecture-grounded design pass after reading steering, V3 motion/schema/compiled-plan docs, TTE motion engine, and the current tui-vfx/tui-vfx-recipes runtime seams.</CLOG> -->

# V3 per-cell motion plan

Status: implementation design for Task 23.

## 1. Why this exists

The TTE audit identified per-cell motion as the strategic gap behind exact
MiddleOut, Slice, BinaryPath, and several future TTE-style effects. Those effects
are not just whole-rect host motion and not just per-cell colour/glyph shaders.
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

The tui-vfx version should treat per-cell motion as a deterministic content/source
remapping pass: source cells become cell actors, each actor evaluates the same
motion spec with a per-cell stagger, then the scheduler writes actors into a
rendered grid for downstream pipeline steps.

## 2. Placement in the V3 architecture

### 2.1 Not a new `StepKind`

Per-cell motion should not become a sixth pipeline step kind. V3 keeps the
pipeline algebra small: `mask`, `sampler`, `filter`, `shader`, `style_effect`,
plus `sequence`/`parallel`. Per-cell motion is a content/source remapping
substrate, not a visual postprocess leaf.

A pipeline filter can modify glyphs or colours after cells exist. A sampler can
produce displacement fields. A shader can shade rendered cells. Per-cell motion
answers an earlier question: _where does each source cell land this frame?_

### 2.2 Schema homes

Use one reusable `CellMotionSpec`, available at the places that produce cells:

```text
config.content.cell_motion?                  # root message/content source
scene.layers[*].cell_motion?                 # layer-local source remapping
```

Rationale:

- `config.motion` already means whole host/envelope geometry. Do not overload it.
- `scene.layers[*].placement.motion` already means whole layer placement. Do not
  hide per-cell motion there.
- `pipeline.step` already means per-cell visual operations after source cells
  exist. Do not add a motion leaf.
- `scene.layers[*].cell_motion` is parallel to `source`, `placement`, `surface`,
  and `pipeline`: the source makes cells, cell motion remaps them, the layer
  pipeline transforms them.

Execution order for a layer:

```text
source cells
  -> content effect, if source/content owns one
  -> cell_motion, if present
  -> layer-local pipeline
  -> scene composition
```

Execution order for a root message/content recipe:

```text
message/content source
  -> content effect, if present
  -> content.cell_motion, if present
  -> root pipeline
```

### 2.3 Scope basis

Per-cell motion selects source actors before they move. Downstream visual steps
operate on the rendered grid after they move.

Therefore `CellMotionSpec` needs an explicit selection basis:

```json
"affect": "non_empty",
"scope": { "kind": "all" },
"scope_basis": "authored"
```

Initial rule:

- `cell_motion.scope` is evaluated against authored/source coordinates.
- downstream `pipeline.step.scope` is evaluated against rendered coordinates.
- future hint/probe surfaces may expose both bases, but recipe authors should not
  have to choose for ordinary use.

This keeps authored intent predictable: "move these source glyphs" first, then
"shade what is visible" afterward.

## 3. Core model

### 3.1 Cell actor

The scheduler builds a stable actor table from the source grid.

```rust
pub struct CellActor {
    pub id: CellActorId,
    pub source_index: u32,
    pub authored_x: u16,
    pub authored_y: u16,
    pub glyph: char,
    pub style: CellStyle,
    pub role: Option<RoleTag>,
}
```

`source_index` is deterministic row-major order over selected source cells.
It replaces TTE's mutable character object identity for timing, randomization,
and collision ordering.

Do not include ratatui types. Use `tui-vfx-types` cell/grid/role types at the
public seam.

### 3.2 `CellMotionSpec`

Recommended authoring shape:

```json
"cell_motion": {
  "enter": {
    "duration_ms": 700,
    "easing": "sine_in_out",
    "route": { "type": "linear" },
    "dynamics": [],
    "from": { "type": "origin", "anchor": "center" },
    "to": { "type": "authored" },
    "stagger": {
      "type": "by_position",
      "axis": "x",
      "direction": "left_to_right",
      "stride_ms": 18
    },
    "snap": { "type": "round" },
    "collision": { "mode": "source_order" },
    "affect": "non_empty"
  }
}
```

Resolved Rust shape:

```rust
pub struct CellMotionSpec {
    pub enter: Option<CellMotionPhaseSpec>,
    pub dwell: Option<CellMotionPhaseSpec>,
    pub exit: Option<CellMotionPhaseSpec>,
}

pub struct CellMotionPhaseSpec {
    pub duration_ms: u64,
    pub easing: EasingCurve,
    pub route: PathType,
    pub dynamics: Vec<CellMotionDynamicSpec>,
    pub from: CellPlacement,
    pub via: Option<CellPlacement>,
    pub to: CellPlacement,
    pub stagger: CellStagger,
    pub snap: SnappingStrategy,
    pub collision: CellCollisionPolicy,
    pub affect: CellMotionAffect,
    pub scope: Option<V3ScopeLike>,
    pub scope_basis: CellScopeBasis,
    pub visibility: CellMotionVisibility,
}
```

`CellMotionDynamicSpec` should initially mirror `V3MotionDynamicSpec` and lower
through the same `PathType::Composed` substrate. Avoid inventing a second physics
vocabulary.

### 3.3 `CellPlacement`

Cell placement is not identical to whole-host `PlacementSpec`. It needs access
to each cell's authored coordinate. Add a cell-specific placement enum in
`tui-vfx-content`:

```rust
pub enum CellPlacement {
    /// The cell's original authored/source coordinate.
    Authored,
    /// Authored coordinate plus a signed offset.
    AuthoredOffset { dx: i16, dy: i16 },
    /// One absolute cell coordinate in the layer/local frame.
    Absolute { x: i16, y: i16 },
    /// Anchor in the layer/local frame, e.g. center for MiddleOut.
    Origin { anchor: Anchor },
    /// Outside the layer/local frame, preserving the other axis when possible.
    Offscreen { direction: SlideDirection, margin_cells: u16 },
    /// Deterministic scatter point, seeded by source_index and recipe seed.
    RandomInRect { rect: Rect, seed: u64 },
    /// Deterministic offscreen edge point for spray/rain/future emitters.
    RandomOffscreen { direction: SlideDirection, margin_cells: u16, seed: u64 },
}
```

Do not put sibling/layer follow semantics in `CellPlacement`. That belongs to
whole-layer `placement.motion`. Per-cell placement is local to one source grid.

### 3.4 `CellStagger`

Stagger is a first-class part of per-cell motion because TTE effects depend on
it and because authors need a compact way to say "same route, delayed per cell."

Initial variants:

```rust
pub enum CellStagger {
    None,
    ByIndex { stride_ms: u64 },
    ByPosition { axis: CellStaggerAxis, direction: CellStaggerDirection, stride_ms: u64 },
    ByDistance { origin: CellPlacement, stride_ms: u64 },
    Random { seed: u64, max_offset_ms: u64 },
}
```

Keep this deterministic. No recipe-time random without a seed.

Canonical directions should reuse existing V3 vocabulary where possible:
`left_to_right`, `right_to_left`, `top_to_bottom`, `bottom_to_top`, and later the
wipe direction family when diagonal/corner waves are needed.

## 4. Scheduler semantics

For each actor and sampled playback time:

1. Resolve actor-local start offset from `stagger`.
2. Compute local elapsed:
   `local_ms = phase_elapsed_ms - actor_offset_ms`.
3. If `local_ms < 0`, use `visibility.before_start`.
4. Compute `local_t = clamp(local_ms / duration_ms, 0..1)`.
5. Apply easing and optional quantization.
6. Resolve `from`, `via`, and `to` placements for this actor.
7. Lower route + dynamics through the existing geometry path substrate.
8. Interpolate a floating-point position.
9. Snap to a cell coordinate.
10. Clip if outside the layer/local frame.
11. Resolve collisions deterministically.
12. Write the actor's glyph/style/role to the destination grid.

This is the per-cell analogue of the shader model: every actor evaluates the
same spec with a different source coordinate/index in its signal context.

### 4.1 Collision policy

A terminal cell can display one glyph. Collisions need a deterministic policy.

Initial policy:

```rust
pub enum CellCollisionMode {
    SourceOrder,
    ReverseSourceOrder,
    NearestToCompletion,
    PreserveExisting,
}
```

Default: `SourceOrder` with later row-major source cells overwriting earlier
ones. It is deterministic, simple, and makes final state stable when all cells
land at distinct authored coordinates.

Defer blending/stacking. That belongs with particle/emitter work or a future
multi-glyph subcell renderer.

### 4.2 Visibility policy

Initial policy:

```rust
pub struct CellMotionVisibility {
    pub before_start: CellVisibilityMode,
    pub after_complete: CellVisibilityMode,
}

pub enum CellVisibilityMode {
    Hidden,
    AtFrom,
    AtTo,
    Hold,
}
```

Defaults:

- enter: before start hidden, after complete hold at `to`
- dwell: before start hold, after complete hold
- exit: before start hold, after complete hidden

This gives normal reveal/exit behavior without app glue.

### 4.3 Timing inputs

Use `V3PlaybackTiming` as the root timing source and preserve both:

- normalized phase progress for lifecycle semantics;
- monotonic elapsed time for cadence and stagger continuity.

Per-cell motion must not derive cadence from `loop_t` if a monotonic elapsed
value is available. This matches the current V3 timing contract.

## 5. Relationship to existing systems

### 5.1 Whole-recipe and layer motion

`config.motion` moves the host rectangle. `scene.layers[*].placement.motion`
moves a layer. `cell_motion` moves source cells inside their local layer/root
surface. These can compose:

```text
cell coordinate inside layer
  -> cell_motion local remap
  -> layer placement/motion
  -> scene/root composition
  -> recipe-envelope motion
```

Do not make shadows follow individual cells in the initial slice. Shadows remain
attached to the host/layer envelope. Per-cell trails and per-cell shadows are a
future effect/emitter concern.

### 5.2 Content effects

Content effects still transform glyph strings first. Per-cell motion operates on
the cells produced by content. This lets `typewriter + cell_motion + shader` work
without defining a second content system.

For effects that need actor identity to survive content changes, the first slice
should document the limitation: actor table identity is rebuilt from the produced
source grid for the sampled frame. Stateful actor identity across changing text
belongs to a later compiled content actor plan.

### 5.3 Hints and probes

Initial per-cell motion does not need to publish a hint for rendering. But it
should be observable.

Probe/trace should report:

- configured actor count;
- moved actor count;
- clipped actor count;
- collision count;
- max stagger offset;
- phase-local t range;
- a few sampled actor mappings for debug (`source -> dest`).

Future optional hints:

- `cell_motion.progress` scalar;
- `cell_motion.displacement` vector;
- `cell_motion.source_index` integer.

Those should use the existing V3 typed hint namespace, not ad-hoc fields.

## 6. Edge cases to design for now

### Empty cells

Default `affect = non_empty`. Moving blank cells is expensive and visually
ambiguous. Allow `all` later if a recipe has a real reason.

### Unicode / grapheme width

The scheduler operates on grid cells, not raw Unicode scalar indices. It should
not split grapheme clusters while building actors. If a source layer already
expanded a wide grapheme into multiple grid cells, the scheduler treats the
occupied cells as separate actors unless the source surface carries a future
cluster-id field. That is acceptable for the first slice; document it.

### Out-of-bounds positions

Out-of-bounds actors are clipped. Do not wrap unless a future `wrap` visibility
mode is explicitly added.

### Zero duration

If `duration_ms = 0`, render at `to` for enter/dwell and hidden for completed
exit. Validators should warn; runtime should not divide by zero.

### Stagger longer than phase

Allowed. Some actors may never start within that sampled phase. Probe should
make that visible through phase-local t range and hidden actor count.

### Randomization

Random placements/staggers must be seeded. Use recipe seed + actor id; no global
RNG, no nondeterministic replay.

### Reduced motion

The first schema should allow a host to disable or compress per-cell motion by
policy, but not encode product policy in the primitive. A reasonable runtime
fallback is to collapse per-cell routes to immediate `to` while preserving final
content and downstream pipeline effects.

## 7. Implementation packets

### Packet 1 — types and pure scheduler in `tui-vfx-content`

Add:

- `CellMotionSpec`
- `CellMotionPhaseSpec`
- `CellPlacement`
- `CellStagger`
- `CellCollisionPolicy`
- `CellMotionVisibility`
- pure function: `apply_cell_motion(grid, roles, spec, timing, frame_rect) -> SemanticScene/Grid`

Tests first:

- MiddleOut-style center origin to authored coordinates;
- Slice-style top/bottom offscreen origins;
- deterministic random stagger;
- collision policy;
- clipping;
- zero duration;
- Unicode/grapheme smoke test.

### Packet 2 — recipe schema and V3 parse/normalize/compile

Add schema fields in `tui-vfx-recipes`:

- `config.content.cell_motion`
- `scene.layers[*].cell_motion`

Normalize by making defaults explicit. Compile into typed cell-motion plans, not
opaque `serde_json::Value`, once the authoring types are stable.

Do not block on full schema polish before landing the pure scheduler. The engine
can be implemented and tested from Rust first.

### Packet 3 — runtime integration

Integrate in direct V3 rendering:

- root content source path before root pipeline;
- scene layer source path before layer-local pipeline;
- preserve roles through movement;
- expose truth-surface/probe counters.

### Packet 4 — debug recipes

A new primitive is not done without debug recipes. Add at least:

- `recipes/debug_recipes/content/content_cell_motion_middle_out.json`
- `recipes/debug_recipes/content/content_cell_motion_slice.json`
- `recipes/debug_recipes/complex/complex_cell_motion_shader_pipeline.json`

The complex recipe should prove downstream pipeline composition after cell
motion, e.g. moved glyphs then AnimatedGlyphRamp or a gradient/shader pass.

### Packet 5 — docs and release evidence

Update:

- `docs/design/tui-vfx-v3-schema-overview.md`
- `docs/design/tui-vfx-v3-recipe-vocabulary.md`
- `docs/design/tui-vfx-v3-compiled-execution-plan.md`
- sibling `tui-vfx-recipes` authoring/schema docs and generated docs
- capability docs for the new content/source ingredient

Validation baseline:

```sh
# tui-vfx
cargo test -p tui-vfx-content cell_motion -- --nocapture
just docs-all-check
git diff --check

# tui-vfx-recipes
cargo test -p tui-vfx-recipes cell_motion -- --nocapture
cargo run -q -p pipeline-validator -- --rules --stages <new recipes>
cargo run -q -p pipeline-validator -- --debug-recipes-qc --format json <new recipes>
just docs-v3-check
git diff --check
```

## 8. TTE mapping

### MiddleOut

```json
"cell_motion": {
  "enter": {
    "from": { "type": "origin", "anchor": "center" },
    "to": { "type": "authored" },
    "route": { "type": "linear" },
    "easing": "sine_in_out",
    "stagger": { "type": "none" }
  }
}
```

Exact two-phase TTE behavior can be expressed later as two staged cell-motion
tracks: center → centerline, then centerline → authored. The first slice should
support one phase per lifecycle phase; multi-track sequencing can follow once the
single-track substrate is proven.

### Slice

Use `from: offscreen from_top` for the upper half and `from_bottom` for the lower
half. This likely needs two scoped cell-motion specs or a future `from_by_scope`
policy. For the first slice, use two scene/text layers or add `tracks[]` if tests
show one spec is too limiting.

Recommendation: start with one phase spec, but make the Rust model easy to widen
to `tracks: Vec<CellMotionTrack>` without changing placement/stagger semantics.

### BinaryPath

Task 23 does not spawn extra glyphs. It can move existing source cells. BinaryPath
still needs Task 24's glyph particle emitter. The shared substrate should make
Task 24 small: emitter creates actors, cell motion schedules them.

## 9. Design decisions to keep stable

1. Per-cell motion is content/source remapping, not a pipeline StepKind.
2. Source selection happens in authored coordinates; downstream effects see
   rendered coordinates.
3. The motion vocabulary reuses V3 route/dynamics/easing where possible.
4. Random behavior is seeded and deterministic.
5. The scheduler is pure and grid-first; no ratatui types.
6. Collision policy is explicit and deterministic.
7. Probe/trace counters are part of the definition of done.
8. Debug recipes are mandatory before the primitive is considered complete.

## 10. Open follow-ups after Task 23

- Multi-track per-cell choreography for exact MiddleOut centerline staging.
- Cluster-aware wide-grapheme actor grouping.
- Per-cell trails/ghosts.
- Per-cell motion output hints for shader correlation.
- Task 24 glyph particle emitter built on the same scheduler.
- Optional host reduced-motion policy hook.

<!-- <FILE>docs/design/tui-vfx-v3-per-cell-motion-plan.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
