<!-- <FILE>docs/arch/tui-vfx-compost-vertical-implementation-plan.md</FILE> - <DESC>Formal implementation plan for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>VERSION: 2.0.0</VERS> -->
<!-- <WCTX>tui-vfx-compost clean-sheet build: active plan is pure v3.1 substrate first, then primitive migration.</WCTX> -->
<!-- <CLOG>2.0.0: MAJOR — replace the copied-crate migration plan with the tui-vfx-compost clean-sheet pure v3.1 substrate-first implementation plan.</CLOG> -->

# tui-vfx-compost Vertical Implementation Plan

**HARD DIRECTIVE — PURE v3.1 END TO END:** The active target is
`tui-vfx-compost`, a clean-sheet v3.1 compositor crate. The runtime must consume
canonical v3.1 structures directly after load validation. Do not add bridge,
shim, adapter, lowering, legacy DTO, `CompositionSpec`, `ShaderLayerSpec`, or
`SpatialShaderType` paths to make v3.1 work. Use `tui-vfx-compositor` only as
read-only reference material for proven behavior.

## Status

Active execution plan. This document supersedes the older copied-crate plan.

Completed and recorded:

- v3.1 native transition schema model is documented and implemented.
- v3.1 descriptor/schema ambiguous-name audit is complete and guardrailed.
- `tui-vfx-compost` has the basic crate/family layout.
- `shader.linearGradient` exists as the first compost primitive proof.

Current next step:

```text
bring over non-primitive compositor substrate from tui-vfx-compositor
  → adapt it to canonical v3.1 structures directly
  → keep shader.linearGradient as the first primitive proof
  → then resume primitive migration slices
```

## Executive Summary

`tui-vfx-compost` is the clean v3.1 compositor path. It is not a compatibility
layer and not a repair of an abandoned copied crate. The work now proceeds
substrate-first: bring over the non-primitive runtime capabilities needed for
real scenes, sources, frame output, sampling, render orchestration, diagnostics,
procedural behavior, and loopback-driven execution, while keeping all data flow
canonical v3.1.

The intended data path is:

```text
v3.1 recipe
  → load-time validation / canonicalization
  → LoadedV31Recipe
  → compost scene/source/render substrate
  → compost primitive runtime
  → rendered frame output
```

Authoring shorthand may be canonicalized at load time. Runtime bridge/lowering
layers are forbidden. Once loaded, the compositor executes canonical v3.1
structures directly.

## Non-Negotiable Principles

1. **Clean-sheet target.** The active crate is `tui-vfx-compost`.
2. **Read-only reference.** `tui-vfx-compositor` may be inspected, but not edited
   for this work.
3. **No legacy data model.** Do not translate v3.1 into old compositor DTOs.
4. **Substrate before fan-out.** Bring over non-primitive runtime logic before
   launching more primitive slices.
5. **Schema is stable enough to execute.** Do not reopen schema design during
   substrate or primitive work unless a canonical example proves a contract
   defect.
6. **Validation happens at load.** Unsupported semantics fail loudly at the
   v3.1 loader/validator boundary.
7. **OFPF layout discipline is mandatory.** Prefer clear, small, professionally
   named modules. Around 300 LOC is the normal target; files above 500 LOC need a
   split or written cohesion justification.
8. **TDD is mandatory.** Add or update the failing proof first, observe RED when
   practical, implement GREEN, then refactor/de-slop.
9. **Documentation is part of completion.** Code, tests, generated artifacts,
   hand-maintained docs, and signoff notes must be updated before a phase is
   complete.
10. **Every phase ends at a commit boundary.** Run de-slop, architect review,
    code review, verification, and commit before moving to the next phase.

## Completed Work

### Complete — Schema and contract stabilization

- Native transitions are first-class v3.1 contract citizens.
- Transition interruption and reduced-motion policy are included from the start.
- The five-home classification model is recorded:
  - source produces a surface;
  - scene places surfaces;
  - transition handles bounded state/surface changes;
  - graph/effect node handles ongoing or phase-scoped processing;
  - signal/value source provides runtime or preview values.
- Ambiguous v3.1 schema/descriptor names have been renamed or guardrailed.
- External debug recipes using older field names are stale evidence; they should
  be canonicalized, not supported through aliases.

### Complete — Basic compost crate shape

The crate has the initial OFPF family layout:

```text
crates/tui-vfx-compost/src/
  content/
  filters/
  loader/
  masks/
  render/
  samplers/
  shaders/
  source/
  styles/
  validation/
```

The root family README anchors document what belongs in the empty primitive
families and validation family directories.

### Complete — First primitive proof seed

`shader.linearGradient` is the first proof slice. It remains the smoke test while
non-primitive substrate is brought over and strengthened.

## Current Phase — Non-Primitive Substrate Migration

Goal: make `tui-vfx-compost` capable of real direct v3.1 scene execution without
inventing a bridge.

Bring over and adapt only the substrate needed for canonical v3.1 execution:

```text
loader/       loaded recipe wrapper, load errors, acceptance boundary
validation/   source/effect/scene/render contract checks
source/       source materialization from canonical source instances
render/       frame output, sample context, graph step collection, orchestration
context/      compositor context only if needed for direct v3.1 execution
pipeline/     render sequencing only if needed; no DTO lowering
utils/        small pure helpers reused across substrate or primitives
```

Do not create these forbidden shapes:

```text
src/v31/
rendering/
bridge/
adapter/
lowering/
legacy/
```

Do not add old DTOs to make the code compile. If a legacy type seems necessary,
stop and reframe the substrate around canonical v3.1 fields.

## Substrate Work Packets

### Packet A — Frame, cell, and sample context substrate

Acceptance:

- Frame/cell output types are compost-owned and v3.1 neutral.
- Sample context carries explicit time, phase, dimensions, and capability data
  needed by runtime execution.
- No field exists only to mirror a legacy compositor DTO.
- Unit tests cover construction, bounds, and sample-context defaults.

### Packet B — Source materialization substrate

Acceptance:

- Canonical `RecipeSceneElement.sourceInstance` and `SourceSpec.sourceDescriptor`
  fields are consumed directly.
- Text/card/asset/procedural support is added only as far as the first direct
  scene tests require.
- Unsupported source descriptors fail at load or render-contract validation with
  explicit diagnostics.
- Tests use current v3.1 field names.

### Packet C — Scene and layer placement substrate

Acceptance:

- Scene elements are arranged from canonical v3.1 scene data.
- Placement, clipping, z-order, role policy, base style, and paint bounds are
  represented with compost-owned types.
- Shadow/paint-outset support can remain structured or deferred unless needed by
  a canonical example.
- Tests prove ordering and clipping behavior without using legacy DTOs.

### Packet D — Render orchestration substrate

Acceptance:

- Render orchestration walks canonical scenes, sources, graph nodes, transitions,
  and primitive hooks directly.
- Graph/effect nodes remain phase-scoped processing; transitions remain bounded
  state/surface changes.
- Unsupported operations produce explicit diagnostics.
- The first end-to-end test renders a minimal scene with `shader.linearGradient`.

### Packet E — Procedural, loopback, and signal substrate

Acceptance:

- Signal/value-source resolution supports the current canonical schema surface
  needed by examples.
- Preview loopback is represented as v3.1 signal/value behavior, not player-only
  ad hoc state.
- Dwell effects can consume sample time without pretending to be transitions.
- Tests cover at least one runtime/preview-driven value path.

## Primitive Migration After Substrate

After substrate verification, resume primitive migration one vertical slice at a
time. Each primitive slice must include:

1. descriptor inspection;
2. canonical v3.1 fixture or test;
3. RED/GREEN/refactor sequence;
4. load-time validation for unsupported direct-render semantics;
5. runtime implementation under the correct family directory;
6. focused tests plus relevant end-to-end proof;
7. docs/signoff update;
8. de-slop pass;
9. architect review;
10. code review;
11. iteration on findings;
12. commit boundary.

Initial primitive order after substrate:

```text
1. shader.linearGradient       # keep green as smoke proof
2. shader.revealWipe           # only if preserved work maps cleanly
3. remaining shader slices      # one at a time or carefully parallelized later
4. filters
5. masks
6. samplers
7. content
8. styles
```

## Directory Forecast

Current write target:

```text
crates/tui-vfx-compost/src/
  loader/
  validation/
    content/
    filters/
    masks/
    samplers/
    shaders/
    styles/
  source/
  render/
  shaders/
  filters/
  masks/
  samplers/
  content/
  styles/
```

Reference-only source:

```text
crates/tui-vfx-compositor/src/
  context/
  filters/
  masks/
  samplers/
  pipeline/
  traits/
  types/
  utils/
```

Future packets must include an expected path list with each file marked as
read-only reference, expected edit, expected new file, generated, or
should-not-touch.

## Testing And Verification Gates

Minimum standard for docs-only plan edits:

```bash
/usr/local/bin/ofpf-sync --check <touched-docs>
git diff --check -- <touched-docs>
```

Minimum standard for substrate code:

```bash
cargo fmt --package tui-vfx-compost
cargo test -p tui-vfx-compost --test direct_recipe -- --nocapture
cargo test -p tui-vfx-compost
```

Add `cargo clippy -p tui-vfx-compost --all-targets -- -D warnings` when the
crate is ready for clippy enforcement.

Do not use failing external debug recipes as proof that legacy compatibility is
needed. Canonicalize examples to current v3.1 names instead.

## Pause / Resume Rules

If interrupted, read these documents before resuming:

1. `steering/INTENTIONS.md`
2. `steering/OFPF-TOOLS.md`
3. `steering/ORCHESTRATION.md`
4. `docs/arch/tui-vfx-compost-agent-workflow-handoff.md`
5. `docs/arch/tui-vfx-compost-current-state-fence.md`
6. this plan
7. `docs/arch/v31-native-transition-model.md`
8. `docs/arch/v31-schema-boundary-north-star.md`

Resume at the first incomplete phase. As of this version, the next incomplete
phase is non-primitive substrate migration.

<!-- <FILE>docs/arch/tui-vfx-compost-vertical-implementation-plan.md</FILE> - <DESC>Formal implementation plan for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>END OF VERSION: 2.0.0</VERS> -->
