<!-- <FILE>.omx/plans/compost-substrate-migration-plan.md</FILE> - <DESC>Formal plan for migrating v3.1-native compost substrate before broad primitive slices</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>Plan mode artifact: migrate non-primitive compost substrate first, preserving pure v3.1 shape and OFPF file discipline.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — add tui-vfx-recipes usage-oracle findings and recommended schema shape for transition/motion. 0.2.0: MINOR — add typed transition and motion schema decision gate before substrate implementation. 0.1.0: INIT — formalize substrate-first migration plan for tui-vfx-compost.</CLOG> -->

# Compost Substrate Migration Plan

## Requirements Summary

Build `tui-vfx-compost` into the clean v3.1-native compositor substrate before
resuming broad primitive migration. The crate is already positioned as the clean
crate-level staging ground, while `tui-vfx-compositor` remains read-only
reference material (`docs/arch/compositor-next-agent-workflow-handoff.md:81-88`).
The existing hard directive remains: pure v3.1 end to end, no `CompositionSpec`,
`ShaderLayerSpec`, `SpatialShaderType`, bridge/shim DTO, or transitional
lowering layer (`docs/arch/compositor-next-agent-workflow-handoff.md:18-19`,
`docs/arch/compositor-next-agent-workflow-handoff.md:79`).

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
  sampler, content, and style (`docs/arch/compositor-next-agent-workflow-handoff.md:100-109`).

The target is to migrate **non-primitive-specific substrate** first, in small
OFPF-shaped phases, without copying the legacy pipeline wholesale. Legacy
`tui-vfx-compositor/src/pipeline` is broad and tied to legacy spec DTOs such as
`CompositionSpec` and `ShaderLayerSpec` (`crates/tui-vfx-compositor/src/pipeline/mod.rs:6-37`).
It also contains proven runtime concepts worth adapting carefully: shadow path,
fast path, sampler/mask/filter preparation, role-map caching, timing, and
inspected/non-inspected loops (`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:35-84`,
`crates/tui-vfx-compositor/src/pipeline/orc_render_pipeline.rs:86-171`).

Before substrate implementation continues, settle the v3.1 authoring shape for
**typed transitions** and their relationship to **motion**. Current support is
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

The schema decision must preserve a logical authoring model for normal product
use cases: a toast can slide from off-screen and fade at the same time; a modal
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
- Do not edit `crates/tui-vfx-compositor`; use it as read-only reference.
- Do not rely on subagents unless the owner explicitly re-enables them.
- Do not harden timing/effect-stack APIs until the typed transition and motion
  schema decision is documented; otherwise relation-level semantics can land in
  the wrong primitive slot.

## Acceptance Criteria

1. `tui-vfx-compost` renders multiple scene elements in deterministic z/layer
   order, instead of only the first element.
2. Placement and clipping semantics are tested against canonical v3.1 scene
   element fields (`crates/tui-vfx-contract/src/cls_recipe_scene_element.rs:16-52`).
3. Source materialization supports the agreed substrate source set, and every
   unsupported source/input shape fails at `LoadedRecipe::load`, not during
   rendering.
4. Effect stack execution has native slots for content, style, shader, filter,
   mask, and sampler families, but only `shader.linearGradient` is executable
   until additional primitive slices land.
5. Timing/lifecycle handling is represented in a native `SampleContext`/timing
   model and tested before primitives depend on it.
6. Cell write, role write, z-index, and graph parallel merge semantics are
   implemented or explicitly rejected at load time with tests.
7. Runtime values (`parameter`, `signal`, `graphValue`, `map`, `sampledField`)
   are either supported through a native resolver or rejected at load time with
   precise diagnostics. These source variants exist in the canonical contract
   (`crates/tui-vfx-contract/src/cls_value_source.rs:21-70`).
8. Graph topology sequence/parallel traversal follows canonical `GraphStep`
   semantics or rejects unsupported cases at load time
   (`crates/tui-vfx-contract/src/cls_graph_step.rs:13-35`).
9. All new files remain OFPF-sized: target ~300 LOC; any file over 500 LOC gets
   split or receives a written cohesion justification before commit.
10. Each substrate phase completes with tests, `cargo fmt`, `cargo check`,
    targeted tests, OFPF metadata check, AI de-slop, architecture review,
    code review, fixes, re-verification, docs update, and a commit.
11. Phase 0 produces a schema decision for typed transitions versus motion,
    grounded in current descriptor/code support, `/usr/projects/tui-vfx-recipes`
    usage evidence, and normal product use cases.
12. The decision explicitly covers fades in addition to new crossfade behavior,
    explicitly distinguishes fixed diagonal wipe variants from configurable-angle
    wipe, and defines how motion and fade compose for toasts/modals/panel
    replacement.

## Implementation Phases

### Phase 0 — Typed Transition and Motion Schema Decision

Goal: decide the canonical v3.1 schema shape for typed transitions before
substrate code bakes transition semantics into lower-level primitive slots.

Current inventory to ground the decision:

Usage-oracle findings from `/usr/projects/tui-vfx-recipes/recipes`:

- The recipe corpus has 1,394 parsed JSON recipes. Legacy/current recipe usage
  contains 269 `motion_path` entries across 178 files, with motion types
  `linear`, `arc`, `hover`, `bounce`, `spring`, `spiral`, `orbit`,
  `rectilinear`, `projectile`, `step`, `friction`, and `pendulum`.
- Motion and fade already compose as normal practice: 130 files combine
  `motion_path` with fade style effects. `recipes/default_toast.json` is the
  canonical example: enter/exit define off-screen linear motion
  (`/usr/projects/tui-vfx-recipes/recipes/default_toast.json:22-50`) while
  style defines `fade_in` and `fade_out`
  (`/usr/projects/tui-vfx-recipes/recipes/default_toast.json:91-100`).
- Mask transitions and fade also compose as normal practice: 129 files combine
  transition-shaped masks with fade style effects. `multi_filter_faded_notice`
  uses phase duration/easing, filter stacks, and enter/exit wipe masks together
  (`/usr/projects/tui-vfx-recipes/recipes/multi_filter_faded_notice.json:21-90`).
- Motion can be more than linear slide. `smooth_arc` uses arc motion for both
  enter and exit while independently using style effects
  (`/usr/projects/tui-vfx-recipes/recipes/smooth_arc.json:22-53`,
  `/usr/projects/tui-vfx-recipes/recipes/smooth_arc.json:79-101`).
- Interactive state transitions are a separate but related use case.
  `hll_leave_server` uses `transition_duration_ms`, `state_composition`, and
  reduced-motion metadata for hover/focus/active style state changes
  (`/usr/projects/tui-vfx-recipes/recipes/hll_leave_server.json:213-223`,
  `/usr/projects/tui-vfx-recipes/recipes/hll_leave_server.json:310-319`).
- The v3.1 canonical debug recipe shape currently has lifecycle phases and
  resolved scene placement, but no typed transition citizen yet
  (`/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json:18-57`,
  `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json:139-160`).

Schema-shape implication: a transition should be an author-facing lifecycle or
state-change envelope that coordinates one or more typed tracks. Motion is one
track type; fade is another; masks/wipes/iris/dissolve are visibility tracks;
crossfade/push are between-surface relation tracks. This avoids making
`motion` pretend to be all transitions, and avoids forcing authors to chain
low-level primitives for normal toast, modal, and replacement cases.


- Existing fade support includes `style.fadeIn`, `style.fadeOut`,
  `style.colorFade`, and `filter.fadeToCanvas`; crossfade is a new
  between-surface transition and should not be conflated with single-surface
  fades.
- Existing wipe support includes `mask.wipe`, `mask.wipeCorner`, and
  `shader.revealWipe`; the canonical geometry enum supports more directions
  than the v3.1 descriptor currently exposes
  (`crates/tui-vfx-geometry/src/types/cls_wipe_direction.rs:14-34`).
- Existing fixed diagonal wipes are corner-to-corner Manhattan diagonals
  (`TopLeftToBottomRight`, etc.). A configurable-angle diagonal wipe is new
  schema surface and should be represented explicitly, not disguised as one of
  the fixed diagonal enum variants.
- Existing transition-shaped effects also include `mask.iris`, `mask.dissolve`,
  `content.dissolve`, `content.morph`, `content.cellMotion`,
  `content.slideShift`, `filter.motionBlur`, and braille/stipple-like visual
  effects that may become transition styles after audit.
- Current lifecycle and timing contracts already name `enter`, `dwell`, and
  `exit` (`crates/tui-vfx-contract/src/cls_lifecycle_phase.rs:20-26`) and
  element-local timing (`crates/tui-vfx-contract/src/cls_recipe_element_pipeline_timing.rs:16-35`).
- Current scene elements already have placement and a loose `motion` payload
  (`crates/tui-vfx-contract/src/cls_recipe_scene_element.rs:23-43`); Phase 0
  must decide whether this becomes typed motion tracks, remains temporary
  metadata, or is replaced by transition-bound motion.

Recommended schema direction to validate:

```text
recipe
  transitionPresets
    toast.enter
      phase: enter
      target: element.toast
      duration/easing
      tracks[]
        motion.slide
          from: offscreen.right(marginCells: 1)
          to: restPlacement
          path: linear | arc | spring | bounce | ...
        fade.in
          applyTo: both
          from: canvas | transparent | color
          to: sourceStyle
    toast.exit
      phase: exit
      target: element.toast
      tracks[]
        motion.slide(to: offscreen.right(marginCells: 1))
        fade.out(applyTo: both)

    modal.open
      phase: enter
      tracks[]
        fade.in(target: backdrop, applyTo: background)
        iris(target: modal.surface, focal: center, softEdge: true)

    panel.replace
      relation: fromSurface -> toSurface
      tracks[]
        crossfade         # new between-surface blend
        wipe(direction: named | angle)
        push(direction: named | angle)

scene.elements[]
  placement              # final/resting placement
  transitions
    enter: toast.enter
    exit: toast.exit

interactiveStateTransitions
  hover/focus/active
    duration/easing
    composition: replace | layered
    accessibility/reducedMotion
```

The exact field names can change, but the logical shape should preserve these
separations: final placement is not motion; motion is not the whole transition;
fades are single-surface style/opacity tracks; crossfade is a between-surface
relation; wipe/iris/dissolve are visibility tracks; push combines relation plus
motion.

Decision questions:

1. Is `transition` a top-level named recipe citizen referenced by scene elements,
   an element-local lifecycle block, or both through named presets plus inline
   overrides?
2. Are transition tracks the canonical way to coordinate motion + fade + mask,
   while primitive families remain the executable building blocks?
3. What exact wire shape should represent wipe direction? Candidate: a tagged
   direction value supporting named directions and configurable angles, for
   example `direction: { kind: angle, degrees: 37.5 }`, alongside named values
   for cardinal, diagonal, center/edge, and corner-arc wipes.
4. Which transition types are first-class now: `fade`, `crossfade`, `wipe`,
   `iris`, `push`, `dissolve`, `morph`, `stippled`, `braille`; and which are
   deferred until primitive support is proven?
5. How does reduced-motion policy degrade transition tracks: remove motion but
   keep fade, shorten duration, or snap instantly?

Normal-use-case acceptance examples:

- Toast enter: final placement is stable, transition adds `motion.slide` from
  off-screen plus `fade.in`; exit uses inverse slide plus `fade.out` or
  `fadeToCanvas`.
- Modal open: backdrop fades while modal surface uses `iris` or `wipe`; the
  two tracks share phase timing but target different scopes/surfaces.
- Panel replacement: old and new surfaces can use `crossfade`, `wipe`, `push`,
  `dissolve`, or `morph` as a relation, not as an accidental chain of generic
  effects.
- Diagonal wipe: fixed corner diagonals remain named variants; arbitrary-angle
  wipe is supported as a distinct angle direction with validation.

Deliverables:

- Update the schema/design document that owns v3.1 transitions.
- Update descriptor/schema audit notes with the inventory above.
- Add or update contract tests for the accepted wire shape before implementation.
- Record recipe-oracle examples for toast slide+fade, modal mask+fade, arbitrary
  motion path, and interactive state transition.
- Only after this decision, continue Phase 1 substrate work.

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

- `SampleContext` carries explicit elapsed/phase inputs without globals.
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
- Unsupported runtime value behavior fails at load time or through a single
  resolver diagnostic; primitives do not each invent resolver semantics.

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
/usr/local/bin/ofpf-sync --check Cargo.toml crates/tui-vfx-compost/... docs/arch/compositor-next-agent-workflow-handoff.md
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
| Transition and motion semantics land in primitive-local hacks | Phase 0 blocks substrate implementation until typed transition/motion shape is documented and tested. |
| Configurable-angle wipe is confused with fixed diagonal variants | Preserve named fixed diagonals and add an explicit angle direction form with validation. |
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

Settle typed transition/motion schema, then migrate compost substrate before broad primitive migration.

### Drivers

- Transition and motion need a schema-level authoring shape before render substrate APIs harden.
- Future primitives need stable scene/source/render/timing/write/runtime seams.
- The copied compositor-next tree proved too easy to pollute with legacy DTOs and
  versioned paths.
- `tui-vfx-compost` already demonstrates the desired crate-level shape with one
  signed shader slice.

### Alternatives Considered

1. **Resume primitive slices immediately.** Rejected because each primitive would
   have to invent missing scene, timing, write, and runtime semantics.
2. **Copy legacy `pipeline/` wholesale.** Rejected because it carries legacy DTO
   concepts and would recreate the adapter problem this effort is avoiding.
3. **Build every substrate feature in one large phase.** Rejected because timing,
   graph values, merge policy, and procedural sources are independently risky
   and need focused tests.
4. **Defer transition schema until after substrate.** Rejected because normal
   use cases such as toast slide+fade and panel crossfade affect scene, timing,
   relation, and effect-stack boundaries.

### Why Chosen

A transition/motion schema gate followed by a substrate-first sequence lets
`tui-vfx-compost` become a real compositor before many primitives depend on it,
while preserving the pure v3.1 architecture and OFPF modularity.

### Consequences

- Substrate migration now has a Phase 0 schema decision gate before code phases.
- Primitive migration pauses until substrate gates pass.
- Some descriptor-valid recipes will be rejected temporarily with explicit tests.
- The eventual crate rename remains simpler because the native API is shaped
  before downstream consumers depend on it.

### Follow-ups

- Add a typed transition schema decision record before Phase 1 substrate work.
- Add a current substrate scoreboard to the handoff doc after Phase 1.
- Decide when `tui-vfx-player-next` should consume `tui-vfx-compost` for visual
  testing.
- Revisit crate rename only after substrate and enough primitive slices are
  stable and no external consumers are using the temporary name.

<!-- <FILE>.omx/plans/compost-substrate-migration-plan.md</FILE> - <DESC>Formal plan for migrating v3.1-native compost substrate before broad primitive slices</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
