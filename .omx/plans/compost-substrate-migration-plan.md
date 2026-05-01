<!-- <FILE>.omx/plans/compost-substrate-migration-plan.md</FILE> - <DESC>Formal plan for migrating v3.1-native compost substrate before broad primitive slices</DESC> -->
<!-- <VERS>VERSION: 0.5.0</VERS> -->
<!-- <WCTX>Require each substrate phase to start from tui-vfx-compositor reference study and an adaptation plan before implementation.</WCTX> -->
<!-- <CLOG>0.5.0: MINOR — add Phase 2 source substrate reference study and adaptation plan.
0.4.6: PATCH — record graphBinding.timing as rejected until timing substrate.
0.4.5: PATCH — record Phase 1 strict deferred scene-element policy rejection scope.
0.4.4: PATCH — add the per-phase reference-study gate and Phase 1 migration/adaptation plan.
0.4.3: PATCH — tighten timing, proof-harness, wipe-angle, and verification wording for active substrate execution.</CLOG> -->

# Compost Substrate Migration Plan

## Requirements Summary

Build `tui-vfx-compost` into the clean v3.1-native compositor substrate before
resuming broad primitive migration. The crate is already positioned as the clean
crate-level staging ground, while `tui-vfx-compositor` remains read-only
reference material (`docs/arch/tui-vfx-compost-agent-workflow-handoff.md`).
The existing hard directive remains: pure v3.1 end to end, no `CompositionSpec`,
`ShaderLayerSpec`, `SpatialShaderType`, bridge/shim DTO, or transitional
lowering layer (`docs/arch/tui-vfx-compost-agent-workflow-handoff.md`).

Current compost state is intentionally minimal:

- `loader/` accepts canonical v3.1 recipes and invokes validation
  (`crates/tui-vfx-compost/src/loader/cls_loaded_recipe.rs:18-29`).
- `validation/` rejects unsupported direct-render inputs at load time and only
  supports `shader.linearGradient` today
  (`crates/tui-vfx-compost/src/validation/orc_validate_render_contract.rs:13-32`).
- `render/` currently renders only the first scene and first element
  (`crates/tui-vfx-compost/src/render/fnc_render_recipe.rs:17-31`).
- `render/` currently materializes a source, creates one destination scene,
  collects graph nodes, applies shaders, and writes cells directly
  (`crates/tui-vfx-compost/src/render/fnc_render_recipe.rs:33-58`,
  `crates/tui-vfx-compost/src/render/fnc_render_recipe.rs:90-135`).
- README anchors exist for all v3.1 effect families: shader, filter, mask,
  sampler, content, and style (`docs/arch/tui-vfx-compost-agent-workflow-handoff.md`).

The target is to migrate **non-primitive-specific substrate** first, in small
OFPF-shaped phases, without copying the legacy pipeline wholesale. Legacy
`tui-vfx-compositor/src/pipeline` is broad and tied to legacy spec DTOs such as
`CompositionSpec` and `ShaderLayerSpec` (`crates/tui-vfx-compositor/src/pipeline/mod.rs:6-37`).
It also contains proven runtime concepts worth adapting carefully: shadow path,
fast path, sampler/mask/filter preparation, role-map caching, timing, and
inspected/non-inspected loops (`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:35-84`,
`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:86-171`).

The v3.1 authoring shape for **typed transitions** and their relationship to
**motion** is now settled enough for substrate implementation; keep it stable
unless a concrete canonical example proves a contract defect. Current support is
not empty: transition-shaped behavior already exists across `mask.wipe`,
`mask.wipeCorner`, `mask.iris`, `mask.dissolve`, `content.dissolve`,
`content.morph`, `content.cellMotion`, `content.slideShift`, `style.fadeIn`,
`style.fadeOut`, `style.colorFade`, `filter.fadeToCanvas`,
`filter.motionBlur`, and `shader.revealWipe` in
`descriptors/v3.1/packs/primitive.json`. The existing geometry crate also has
a richer wipe vocabulary than the descriptor currently exposes: cardinal,
cardinal aliases, fixed corner-to-corner diagonals, center/edge barn-door
wipes, and corner arc in/out variants
(`crates/tui-vfx-geometry/src/types/cls_wipe_direction.rs:14-34`,
`crates/tui-vfx-geometry/src/types/cls_wipe_direction.rs:59-182`).

The completed native transition model must preserve a logical authoring model
for normal product use cases: a toast can slide from off-screen and fade at the same time; a modal
can fade its backdrop while using an iris or wipe reveal; a panel replacement
can crossfade, wipe, push, dissolve, or morph between two surfaces. Motion is
therefore not synonymous with transition. Motion is a spatial track that may be
used by a transition, while transition is the lifecycle/state-change envelope
that coordinates one or more tracks over enter, exit, or between-surface
replacement.

## Non-Goals

- Do not migrate additional primitive implementations during substrate phases,
  except where a substrate test needs the existing `shader.linearGradient`.
- Do not recreate `pipeline/` as a legacy DTO execution layer.
- Do not add `src/v31/`, `rendering/`, `bridge/`, `adapter/`, or `lowering/`.
- Do not put the recipe schema version number into compost crate, module, type,
  or function names; use names such as `LoadedRecipe`, `LoadError`,
  `SampleContext`, and `render_recipe`.
- Do not edit `crates/tui-vfx-compositor`; use it as read-only reference.
- Do not treat `tui-vfx-next` or `schemas/v3.1/next` as part of the active
  compost substrate verification loop unless they are directly touched by a
  shared contract/schema change. They remain proof-only/historical artifacts.
- For this substrate plan, do not assume delegated subagents are available unless
  the owner explicitly re-enables them. This does not change the broader
  handoff model for future primitive slices.
- Do not harden timing/effect-stack APIs in a way that conflicts with the
  completed native transition model. Relation-level semantics such as crossfade,
  push, and morph must not be collapsed into single-surface primitive slots.

## Acceptance Criteria

1. `tui-vfx-compost` renders multiple scene elements in deterministic z/layer
   order, instead of only the first element.
2. Placement and clipping semantics are tested against canonical v3.1 scene
   element fields (`crates/tui-vfx-contract/src/cls_recipe_scene_element.rs:16-52`).
3. Source materialization supports the agreed substrate source set, and every
   unsupported source/input shape fails during `LoadedRecipe::load`, including
   its render-contract validation subpass, not during ordinary rendering.
4. Effect stack execution has native slots for content, style, shader, filter,
   mask, and sampler families. No primitive is counted as migrated yet;
   `shader.linearGradient` may remain a prototype/test candidate until it is
   redone after substrate work.
5. Timing/lifecycle handling is represented in a native `SampleContext`/timing
   model and tested before primitives depend on it.
6. Cell write, role write, z-index, and graph parallel merge semantics are
   implemented or explicitly rejected at load time with tests.
7. Runtime values (`parameter`, `signal`, `graphValue`, `map`, `sampledField`)
   are supported through one native resolver where possible. Unsupported
   value-source variants that are statically knowable fail during
   `LoadedRecipe::load`; sample-dependent failures flow through one shared
   resolver diagnostic path. These source variants exist in the canonical contract
   (`crates/tui-vfx-contract/src/cls_value_source.rs:21-70`).
8. Graph topology sequence/parallel traversal follows canonical `GraphStep`
   semantics or rejects unsupported cases at load time
   (`crates/tui-vfx-contract/src/cls_graph_step.rs:13-35`).
9. All new files remain OFPF-sized: target ~300 LOC; any file over 500 LOC gets
   split or receives a written cohesion justification before commit.
10. Each substrate phase completes with tests, `cargo fmt`, `cargo check`,
    targeted tests, OFPF metadata check, AI de-slop, architecture review,
    code review, fixes, re-verification, docs update, and a commit.
11. Phase 0 transition/motion decision is complete enough for substrate work and is
    recorded in `docs/arch/v31-native-transition-model.md`.
12. The decision explicitly covers fades in addition to new crossfade behavior,
    explicitly distinguishes fixed diagonal wipe variants from configurable-angle
    wipe, and defines how motion and fade compose for toasts/modals/panel
    replacement.

## Implementation Phases

### Required per-phase reference study gate

Before implementation in each substrate phase, first study the mature
`tui-vfx-compositor` logic that owns the closest proven behavior for that phase.
Use `ofpf-*` as the lead inspection path:

1. `ofpf-inspect` the relevant mature files.
2. `ofpf-tests` and `ofpf-refs` the reference entrypoints when behavior depends
   on existing tests or downstream call shapes.
3. `ofpf-around` the exact proven loops, helpers, and diagnostics that need to
   be carried forward.
4. Write a phase-specific migration/adaptation plan that lists:
   - reference files and behavior studied;
   - which behavior will be migrated as-is conceptually;
  - which non-canonical or schema-specific surfaces must be replaced by
    canonical v3.1 structures;
   - which behavior is deferred to a later substrate phase;
   - red tests that prove the migrated behavior in `tui-vfx-compost`.

Do not start broad phase implementation from invention. If no matching mature
logic exists, record that explicitly and ground the phase in contract/vocabulary
documents before coding.

### Phase 0 — Native Transition and Motion Substrate Checkpoint — Complete

Goal: consume the completed v3.1 native transition model before substrate code
hardens scene, timing, relation, and effect-stack APIs. This phase does not
reopen transition schema by default. It verifies that substrate APIs preserve
the established separation:

- transition = bounded state/surface-change interval;
- motion = transition track or placement behavior, not the whole transition;
- opacity/style fade = single-surface track/effect behavior;
- crossfade/push/morph = relation tracks between surfaces;
- wipe/iris/dissolve/blinds/stipple/braille = visibility tracks;
- ongoing dwell effects remain graph/effect nodes or sources.

Controlling decision record:

```text
docs/arch/v31-native-transition-model.md
```

Record only implementation-impact notes or concrete schema defects found during
substrate work. Do not add compatibility aliases or reopen transition design
without a canonical example that proves a contract defect.

Usage-oracle evidence from `/usr/projects/tui-vfx-recipes/recipes` remains useful
as motivation, not wire-shape authority:

- The recipe corpus has 1,394 parsed JSON recipes. Legacy/current recipe usage
  contains 269 `motion_path` entries across 178 files, with motion types
  `linear`, `arc`, `hover`, `bounce`, `spring`, `spiral`, `orbit`,
  `rectilinear`, `projectile`, `step`, `friction`, and `pendulum`.
- Motion and fade already compose as normal practice: 130 files combine
  `motion_path` with fade style effects. `recipes/default_toast.json` is the
  canonical example: enter/exit define off-screen linear motion while style
  defines `fade_in` and `fade_out`.
- Mask transitions and fade also compose as normal practice: 129 files combine
  transition-shaped masks with fade style effects.
- Motion can be more than linear slide. `smooth_arc` uses arc motion for both
  enter and exit while independently using style effects.
- Consumer-driven state changes such as hover/focus/active may select or trigger
  engine-level transitions through opaque metadata, host signals, parameters, or
  variants. The engine transition model remains platform-agnostic and does not
  encode UI state policy.

Illustrative shape only; exact wire names are governed by
`docs/arch/v31-native-transition-model.md`:

```text
Recipe-level transitions:
  toastEnter
    activePhases: ["enter"]
    subjects: to = element.toast
    tracks:
      - motion.slide, subject: to, from: offscreen/right, to: restPlacement
      - opacity.fade, subject: to, from: 0, to: 1

  toastExit
    activePhases: ["exit"]
    subjects: from = element.toast
    tracks:
      - motion.slide, subject: from, from: restPlacement, to: offscreen/right
      - opacity.fade, subject: from, from: 1, to: 0

  modalOpen
    activePhases: ["enter"]
    tracks:
      - opacity.fade for backdrop
      - visibility.iris for modal surface

  panelReplace
    subjects: from + to
    tracks:
      - relation.crossfade, relation.push, relation.morph, or visibility.wipe
```

Fixed diagonal wipes remain named variants. Arbitrary-angle wipe is a candidate
future direction form and must not be disguised as one of the fixed diagonal
variants unless the transition contract explicitly accepts that extension.

Implementation-impact checkpoint questions:

1. Does a substrate API preserve transition interval/subject/timing semantics
   instead of pushing relation behavior into primitive-local slots?
2. Does a source/scene/render API keep final placement distinct from motion
   tracks?
3. Does a fade use `opacity.fade` or `style.colorFade` semantics instead of a
   generic `fade.in`/`fade.out` shape?
4. Do consumer state examples remain host-signal/metadata-driven instead of
   engine-core UI policy?

Deliverables:

- Confirm `docs/arch/v31-native-transition-model.md` remains the controlling
  transition decision record.
- Record only implementation-impact notes or concrete schema defects discovered
  during substrate work.
- Continue Phase 1 substrate work without treating transition schema as a
  blocker.

### Phase 1 — Scene and Element Substrate

Goal: make `render/` execute the canonical scene/element structure rather than a
single first-scene/first-element shortcut.

Likely files:

```text
crates/tui-vfx-compost/src/render/
  fnc_render_recipe.rs                  # keep orchestration thin
  col_scene_elements_in_paint_order.rs  # z/layer ordering helper
  fnc_render_scene.rs                   # one scene render orchestration
  fnc_render_scene_element.rs           # one element render orchestration
  fnc_clip_element_bounds.rs            # clipping helper
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_scene_elements.rs
```

Acceptance:

- Multiple elements render into one `Frame`.
- Later z-index appears over earlier z-index.
- Negative/overflow placement clips safely.
- Current one-element linearGradient test still passes.

Reference study and migration plan:

- Mature reference files studied with `ofpf-*`:
  - `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs`
  - `crates/tui-vfx-compositor/src/pipeline/cls_render_area.rs`
  - `crates/tui-vfx-compositor/src/pipeline/fnc_render_pipeline_with_spec_area.rs`
  - `crates/tui-vfx-compositor/tests/pipeline/test_orc_render_pipeline.rs`
  - `crates/tui-vfx-compositor/tests/test_render_pipeline_role_awareness.rs`
- Proven behavior to preserve conceptually:
  - render into a caller-owned destination grid at an explicit offset;
  - keep width/height/offset grouped as a render-area concern;
  - run source-cell copy and effect application through one destination write
    path;
  - preserve deterministic ordering by caller-provided traversal order;
  - keep role-channel writeback separate from cell writeback.
- v3.1 adaptation:
  - replace legacy render-area parameters with `RecipeSceneElement.placement`
    and explicit signed clipping against `RecipeScene.width`/`height`;
  - replace single source/destination call shape with `RecipeScene` allocation
    plus per-`RecipeSceneElement` render orchestration;
  - use canonical `zIndex` plus declaration order for paint order;
  - continue reading graph binding/topology directly from canonical v3.1
    `RecipeSceneElement` / `RecipeDocument` fields.
- Deferred:
  - source role materialization and full role write policy remain Phase 5 work;
    Phase 1 only preserves destination roles and rejects deferred role policies
    at load time;
  - richer scene-element semantics remain later substrate work; Phase 1 rejects
    `clipPolicy: warn`, visibility phases/predicates, surface styling/shadows,
    non-clip overflow, placement motion, declarative placement rules, scroll
    factors, element-local graph timing, and non-`writeCell` cell policies
    rather than silently ignoring them;
  - unsupported source descriptors and richer source materialization remain
    Phase 2 work;
  - full effect-family stack order remains Phase 3 work.
- Red tests:
  - multiple scene elements render into one frame;
  - higher `zIndex` paints after lower `zIndex`;
  - equal `zIndex` preserves declaration-order paint behavior;
  - negative and overflow placement clip without rebasing source origin.

### Phase 2 — Source Substrate

Goal: separate source validation and source materialization so future procedural
sources have a clear seam.

Likely files:

```text
crates/tui-vfx-compost/src/source/
  fnc_source_grid_from_inputs.rs        # keep source.card text path small
  fnc_materialize_source.rs             # source descriptor dispatch
  col_literal_source_input.rs           # existing helper
  cls_source_surface.rs                 # if source output needs roles/metadata

crates/tui-vfx-compost/src/validation/
  fnc_validate_source_inputs.rs         # existing source input validation
  sources/README.md                     # add if source family grows beyond one helper
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_source_contract.rs
```

Acceptance:

- Literal `source.card` remains supported.
- Unsupported source descriptor ids fail at load time.
- Unsupported runtime-sourced source inputs fail at load time.
- Source dimensions and scene dimensions are tested separately.

Reference study and migration plan:

- Mature reference files studied with `ofpf-*` plus direct reads:
  - `crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs`
  - `crates/tui-vfx-compositor/tests/pipeline/test_orc_render_pipeline.rs`
  - `crates/tui-vfx-player/src/fnc_render_scene.rs` for current canonical
    `source.card` authoring behavior after confirming `tui-vfx-compositor`
    itself consumes already-materialized grids.
- Proven behavior to preserve conceptually:
  - compositor render orchestration receives an already-materialized source
    `Grid` plus explicit source width/height;
  - source size and destination scene/render area stay independent;
  - source creation is testable as an input surface before effects are applied.
- v3.1 adaptation:
  - keep `source.card` materialization as a native compost source concern;
  - introduce a descriptor dispatch seam (`materialize_source`) before scene
    placement/clipping;
  - reject unsupported source descriptors, non-literal source values,
    unsupported `source.card` chrome/unknown inputs, out-of-range card
    dimensions, and invalid source literal shapes during `LoadedRecipe::load`;
  - preserve `source.card` message line boundaries during materialization,
    following the current player source behavior rather than flattening text.
- Red tests:
  - `source.card` still renders the existing literal card fixture;
  - `source.text` descriptor is rejected after canonical descriptor validation;
  - runtime-sourced `message` is rejected at load time;
  - unsupported card chrome input is rejected at load time;
  - missing/wrong-shaped source literals and out-of-range dimensions are
    rejected during load;
  - source width clips into a narrower scene independently from scene width;
  - scene width can exceed source width without source reflow;
  - multiline card messages keep line boundaries.

### Phase 3 — Native Effect Stack Skeleton

Goal: define native execution slots for all v3.1 effect families without
implementing new primitives yet.

Likely files:

```text
crates/tui-vfx-compost/src/render/
  cls_effect_stack.rs
  cls_effect_stage.rs
  fnc_build_effect_stack.rs
  fnc_apply_effect_stack.rs

crates/tui-vfx-compost/src/{content,styles,shaders,filters,masks,samplers}/
  README.md                             # already present for family guidance
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_effect_stack_contract.rs
```

Acceptance:

- Load validation rejects unsupported primitive families with family-specific
  diagnostics.
- The stack can contain a supported shader node and preserve deterministic order.
- Family slots are documented and wired only as native dispatch seams, not DTO
  adapters.

### Phase 4 — Timing and Lifecycle Substrate

Goal: give primitives a reliable timing/lifecycle contract before more temporal
primitives migrate.

Likely files:

```text
crates/tui-vfx-compost/src/render/
  cls_sample_context.rs                 # expand carefully
  cls_render_timing.rs
  fnc_resolve_node_phase.rs
  fnc_is_node_active.rs
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_timing_lifecycle.rs
```

Acceptance:

- `SampleContext` carries explicit `phaseT`, optional `loopT`, and
  `absoluteTimeMs` without globals or presentation-FPS coupling.
- `activePhases` is honored or rejected at load time until supported.
- Loop timing behavior is tested before loop-dependent primitives use it.

### Phase 5 — Write, Merge, and Role Policy Substrate

Goal: implement canonical write behavior before masks/filters/content depend on
it.

Likely files:

```text
crates/tui-vfx-compost/src/render/
  cls_cell_write_decision.rs
  fnc_apply_cell_write_policy.rs
  fnc_apply_role_write_policy.rs
  fnc_merge_element_surface.rs
  fnc_merge_parallel_surfaces.rs
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_write_merge_policy.rs
```

Acceptance:

- `cellWritePolicy` and `roleWritePolicy` are honored or rejected at load time.
- Transparent/empty cell behavior is deterministic.
- Parallel graph merge policies are either implemented or rejected explicitly.
- Role tags are preserved/updated according to canonical policy.

### Phase 6 — Runtime Value Resolver Substrate

Goal: support or explicitly reject non-literal canonical `ValueSource` variants
from one native resolver, not ad hoc primitive code.

Likely files:

```text
crates/tui-vfx-compost/src/runtime/
  README.md
  cls_runtime_context.rs
  cls_resolved_value.rs
  fnc_resolve_value_source.rs
  fnc_resolve_parameter.rs
  fnc_resolve_signal.rs
  fnc_resolve_graph_value.rs
  fnc_resolve_sampled_field.rs
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_runtime_values.rs
```

Acceptance:

- Literal values continue to work.
- Parameter/signal/graphValue/map/sampledField behavior is covered by tests.
- Unsupported value-source variants that are statically knowable fail during
  `LoadedRecipe::load`. Sample-dependent failures use one shared resolver
  diagnostic path. Primitive implementations do not invent local value-source
  semantics.

### Phase 7 — Observability and Debuggability

Goal: preserve useful inspection/debug evidence without dragging in legacy
pipeline inspector DTOs.

Likely files:

```text
crates/tui-vfx-compost/src/render/
  cls_render_diagnostic.rs
  cls_render_trace_event.rs
  orc_render_observability.rs
```

Tests:

```text
crates/tui-vfx-compost/tests/direct_recipe/test_render_observability.rs
```

Acceptance:

- Frame diagnostics can explain rejected/skipped work.
- Optional trace events can identify scene, element, stage, and primitive.
- Observability does not require legacy `CompositionSpec` or old pipeline
  inspector types.

### Phase 8 — Primitive Migration Resume Gate

Goal: prove substrate is ready for broad primitive migration.

Acceptance:

- `cargo check -p tui-vfx-compost` passes.
- `cargo test -p tui-vfx-compost` passes.
- Existing direct linearGradient tests pass.
- At least one multi-element scene test, one write-policy test, one timing test,
  and one runtime-value test passes.
- README anchors remain accurate.
- Handoff doc records the substrate status.
- Only then resume primitive slices in small vertical packets.

## Verification Steps

Run these at each phase boundary:

```bash
cargo fmt -p tui-vfx-compost
cargo check -p tui-vfx-compost
cargo test -p tui-vfx-compost
/usr/local/bin/ofpf-sync --check Cargo.toml crates/tui-vfx-compost docs/arch/tui-vfx-compost-agent-workflow-handoff.md .omx/plans/compost-substrate-migration-plan.md
rg -n 'CompositionSpec|ShaderLayerSpec|SpatialShaderType|src/v31|v31/|lowering|adapter|bridge' crates/tui-vfx-compost
```

Expected grep result: no production-code dependency on prohibited legacy/bridge
concepts. README/test text may mention these concepts only as prohibitions.

## Risks and Mitigations

| Risk | Mitigation |
| --- | --- |
| Recreating legacy `pipeline/` under a new name | Keep directories responsibility-named (`render`, `runtime`, `write`) and reject DTO-shaped layers in review. |
| Primitive slices start before substrate is stable | Do not resume broad primitive migration until Phase 8 gate passes. |
| `render/fnc_render_recipe.rs` becomes a hub | Split scene, element, stack, write, and timing helpers before 300 LOC pressure becomes severe. |
| Runtime values become primitive-local hacks | Implement one resolver substrate before allowing non-literal primitive inputs. |
| Over-rejecting descriptor-valid recipes hides required behavior | Every load-time rejection must include a test and a TODO/follow-up note in the phase summary. |
| Under-testing visual semantics | Add small deterministic grid assertions before visual player/UI tests; visual tests become an additional layer, not the only proof. |
| Transition and motion semantics land in primitive-local hacks | Phase 0 uses the completed native transition model as a checkpoint; relation-level semantics must not collapse into primitive-local slots. |
| Configurable-angle wipe is confused with fixed diagonal variants | Preserve named fixed diagonals; treat arbitrary-angle wipe as a candidate future direction form unless accepted by the transition contract. |
| Schema overfits legacy field names | Treat recipes as an oracle for use cases and composition patterns, not as wire-shape authority; choose clean v3.1 names. |

## Quality Gates Per Phase

1. Red test first for the substrate behavior.
2. Minimal implementation to green.
3. OFPF structure review: file names, file size, no hub growth.
4. AI de-slop pass scoped to touched files.
5. Architecture review and code review.
6. Fix all review blockers.
7. Re-run verification commands.
8. Update handoff/status docs.
9. Commit before starting the next phase.

## ADR

### Decision

Consume the completed native transition/motion model, then migrate compost substrate before broad primitive migration.

### Drivers

- Transition and motion already have a controlling v3.1 authoring model that substrate APIs must preserve.
- Future primitives need stable scene/source/render/timing/write/runtime seams.
- The abandoned copied-crate tree proved too easy to pollute with legacy DTOs and
  versioned paths.
- `tui-vfx-compost` already demonstrates the intended crate-level shape;
  primitive counters reset to zero until slices are redone on the
  substrate-first path.

### Alternatives Considered

1. **Resume primitive slices immediately.** Rejected because each primitive would
   have to invent missing scene, timing, write, and runtime semantics.
2. **Copy legacy `pipeline/` wholesale.** Rejected because it carries legacy DTO
   concepts and would recreate the adapter problem this effort is avoiding.
3. **Build every substrate feature in one large phase.** Rejected because timing,
   graph values, merge policy, and procedural sources are independently risky
   and need focused tests.
4. **Reopen transition schema before substrate.** Rejected because native
   transitions are stable enough to execute; reopen only for proven contract
   defects from canonical examples.

### Why Chosen

A transition/motion consumption checkpoint followed by a substrate-first sequence lets
`tui-vfx-compost` become a real compositor before many primitives depend on it,
while preserving the pure v3.1 architecture and OFPF modularity.

### Consequences

- Substrate migration now has a Phase 0 consumption checkpoint for the completed native transition model before code phases.
- Primitive migration pauses until substrate gates pass.
- Some descriptor-valid recipes will be rejected temporarily with explicit tests.
- The eventual crate rename remains simpler because the native API is shaped
  before downstream consumers depend on it.

### Follow-ups

- Keep `docs/arch/v31-native-transition-model.md` as the controlling decision record; add only implementation-impact notes or proven defect follow-ups.
- Add a current substrate scoreboard to the handoff doc after Phase 1.
- Decide when `tui-vfx-player-next` should consume `tui-vfx-compost` for visual
  testing.
- Revisit crate rename only after substrate and enough primitive slices are
  stable and no external consumers are using the temporary name.
- Completed in `steering/MARKETING.md` v0.4.0: capture the new authoring
  philosophy that tui-vfx is an engine-neutral, grid-native VFX engine,
  authoring toolkit, and compositor with end-user authoring ergonomics inspired
  by familiar animation/compositing concepts such as timelines, tracks, easing,
  masks, mattes, transitions, keyframes, and presets.

<!-- <FILE>.omx/plans/compost-substrate-migration-plan.md</FILE> - <DESC>Formal plan for migrating v3.1-native compost substrate before broad primitive slices</DESC> -->
<!-- <VERS>END OF VERSION: 0.5.0</VERS> -->
