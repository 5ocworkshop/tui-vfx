<!-- <FILE>docs/arch/compositor-next-agent-workflow-handoff.md</FILE> - <DESC>Restartable agent workflow for compositor-next direct v3.1 vertical primitive slices</DESC> -->
<!-- <VERS>VERSION: 0.10.2</VERS> -->
<!-- <WCTX>Compositor-next execution handoff: pure v3.1 slices use exact write scopes in the existing repo; preserved worktrees are recovery artifacts, not a future copy pattern.</WCTX> -->
<!-- <CLOG>0.10.2: PATCH — add missing content/style README anchors to cover all v3.1 effect families.
0.10.1: PATCH — record README anchor directories for future shader/filter/mask/sampler slices.
0.10.0: MINOR — record compost crate pivot: build a minimal v3.1-native skeleton first, then rename wrapper later if no consumers.
0.9.0: MINOR — remove v31 from the intended directory forecast; compositor-next is v3.1-native, while old compositor is read-only reference.
0.8.2: PATCH — add the concrete OFPF forecast tree that every primitive packet must include before coding.
0.8.1: PATCH — add explicit broader-task anchor so resumed sessions keep the primitive-by-primitive v3.1 migration goal in view.
0.8.0: MINOR — clarify subagent packets must not create per-slice repo/crate/worktree copies; preserved worktrees are temporary recovery artifacts.
0.7.0: MINOR — add hard work-packet directive: pure v3.1 end-to-end, no CompositionSpec/legacy lowering adapters, halt on adapter attempts.
0.6.0: MINOR — record OFPF structural correction checkpoint and preserved paused slice worktrees.
0.5.0: MINOR — sign off shader.borderSweep and record active parallel slice queue.
0.4.0: MINOR — add typed transitions as a deferred schema-audit discussion item.</CLOG> -->

# Compositor-Next Agent Workflow Handoff

**HARD DIRECTIVE — PURE v3.1 END TO END:** The goal of compositor-next is a pure v3.1 system from recipe load through primitive execution. Work packets must migrate the robust/proven primitive logic into the same runtime file tree while lightly updating implementations to consume canonical v3.1 schema fields directly. Any attempt to adapt v3.1 into `CompositionSpec`, `ShaderLayerSpec`, `SpatialShaderType`, legacy-shaped field names, bridge/shim DTOs, or transitional lowering layers is a failure. Halt immediately if a slice starts adding that kind of adapter. The same work is the opportunity to split large legacy files into OFPF-compliant, professionally named, size-guideline-respecting modules.


## Broader Task Anchor — Never Lose Sight Of This

The broader task is the **compositor-next vertical migration for tui-vfx v3.1**.
The goal is to build `tui-vfx-compositor-next` as the clean future compositor
path where a v3.1 recipe is loaded, validated, represented as canonical v3.1
state, and then rendered by compositor-next primitives directly.

```text
v3.1 recipe
  → recipe loader / validator
  → canonical loaded v3.1 structure
  → compositor-next primitive execution
  → rendered output
```

This means **v3.1 all the way through**. We are migrating existing, proven
compositor primitives/effects into `tui-vfx-compositor-next` one vertical slice
at a time. Each slice starts from robust existing behavior, preserves good
rendering logic, lightly reshapes it to consume canonical v3.1 schema fields
directly, validates at recipe load, proves the path with v3.1 tests, keeps the
file layout OFPF-compliant and comparable to `tui-vfx-compositor/src/`, signs
off the primitive, then repeats.

The work is not a bridge, shim, legacy lowering layer, second data model, broad
recipe migration, horizontal infrastructure project, or per-primitive repo/crate
copying exercise. The intended migration is:

```text
existing hardened primitive logic
        │
        ▼
same conceptual primitive in compositor-next
        │
        ▼
updated to read v3.1 schema fields directly
        │
        ▼
validated with a real v3.1 recipe slice
```

## Purpose

This document is the restart point for the current compositor-next execution
workflow. If a session is interrupted, a fresh lead agent should read this file
after the normal project orientation and then continue from the state recorded
below.

The work is **v3.1 direct compositor-next migration**, not generic V3 work.
The target path is:

```text
canonical v3.1 RecipeDocument
  → LoadedV31Recipe::load(...)
  → load-time descriptor/catalog/direct-render validation
  → compositor-next primitive implementation reads v3.1 nodes/source directly
  → robust copied primitive logic, renamed/reshaped to v3.1 schema fields
```

No new bridge, shim, legacy-input, alias-acceptance, `CompositionSpec` lowering, `ShaderLayerSpec` lowering, `SpatialShaderType` adapter, or other transition layer should be added. If a work packet starts adding one, stop and report the packet as failed.

## Compost Crate Pivot

The current recovery direction is to build `tui-vfx-compost` as a clean
crate-level staging ground instead of continuing to repair the copied
`compositor-next` tree in place. `tui-vfx-compositor` remains read-only
reference material. `tui-vfx-compost` should contain only the minimum native
v3.1 infrastructure required to compile and run migrated slices:

```text
loader/      accepts canonical v3.1 recipes once
validation/  rejects unsupported direct-render inputs at load time
source/      materializes canonical source inputs
render/      owns frame/context/render orchestration
shaders/     contains only signed migrated shader slices
filters/     empty until first filter slice is migrated
masks/       empty until first mask slice is migrated
samplers/    empty until first sampler slice is migrated
```

The first compost slice is `shader.linearGradient`. It proves load validation,
source materialization, direct render orchestration, and one migrated shader
without a `src/v31` path, legacy DTO lowering, or crate copies. The primitive
family directories now include README anchors for future agents:
`src/shaders/`, `src/filters/`, `src/masks/`, `src/samplers/`, `src/content/`, `src/styles/`, and the matching
`src/validation/{filters,masks,samplers,content,styles}/` directories. Those READMEs define
what belongs in each directory and show example OFPF file names. Once the crate
shape is accepted and there are no external consumers, the package/wrapper can
be renamed to the final compositor-next name later; that rename should be a
Cargo/package/API wiring change, not another architecture migration.

## Current Scoreboard

Descriptor pack: `descriptors/v3.1/packs/primitive.json`

Current descriptor-effect count:

```text
120 v3.1 effect descriptors
```

Signed direct compositor-next v3.1 primitives:

```text
5 / 120
```

Signed primitives:

1. `shader.linearGradient`
2. `shader.highlighter`
3. `shader.glistenBand`
4. `shader.focusField`
5. `shader.borderSweep`

Remaining:

```text
115 / 120
```

Human-facing progress banner:

```text
╔════════════════════════════════════════════════════╗
║ v3.1 DIRECT MIGRATION SCOREBOARD                  ║
╠════════════════════════════════════════════════════╣
║ Signed:  5 / 120  █████░░░░░░░░░░░░░░░░░  4.2%   ║
║ Active:  paused — no new worktrees/copies         ║
║ Queue:   fix OFPF structure, then resume slices    ║
╚════════════════════════════════════════════════════╝
```

## Primitive Completion Tracker

Source of truth: `descriptors/v3.1/packs/primitive.json` effect descriptors. Update this list in the same commit that signs off a primitive.

Completed direct compositor-next v3.1 primitives:

```text
shader.linearGradient
shader.highlighter
shader.glistenBand
shader.focusField
shader.borderSweep
```

Outstanding direct compositor-next v3.1 primitives:

```text
[content]
content.cellMotion
content.dissolve
content.glitchShift
content.marquee
content.mirror
content.morph
content.numeric
content.odometer
content.redact
content.scramble
content.scrambleGlitchShift
content.slideShift
content.splitFlap
content.typewriter
content.wrapIndicator
content.glyphCascade
content.glyphParticles

[filter]
filter.bracketEmphasis
filter.crt
filter.dim
filter.dotIndicator
filter.edgeGrow
filter.fadeToCanvas
filter.greyscale
filter.hoverBar
filter.invert
filter.kittScanner
filter.matrixRain
filter.patternFill
filter.pillButton
filter.subPixelBar
filter.tint
filter.underlineWipe
filter.vignette
filter.animatedGlyphRamp
filter.brailleDust
filter.charsetNoise
filter.colorBridgedShade
filter.glistenSweep
filter.glyphStyle
filter.interlaceCurtain
filter.motionBlur
filter.rigidShake
filter.scalarFieldGlyph
filter.shadeScanner
filter.subCellShake
filter.subcellLight

[mask]
mask.blinds
mask.cellular
mask.checkers
mask.diamond
mask.dissolve
mask.iris
mask.materialize
mask.materializeCorner
mask.noiseDither
mask.none
mask.pathReveal
mask.radial
mask.wipe
mask.wipeCorner

[sampler]
sampler.crt
sampler.crtJitter
sampler.faultLine
sampler.radialTwist
sampler.ripple
sampler.shredder
sampler.sineWave
sampler.bounce
sampler.gravity
sampler.pendulum
sampler.spatialSignal

[shader]
shader.barberPole
shader.diffusion
shader.radar
shader.revealWipe
shader.wayfindingNode
shader.affordanceWake
shader.bevel
shader.chromaticEdge
shader.coloredOverlay
shader.concealedLight
shader.cursor
shader.edgeSheen
shader.focusedRowGradient
shader.fractionalStripeOverlay
shader.glitchLines
shader.neonFlicker
shader.orbit
shader.pulseWave
shader.radialSpiral
shader.reflect
shader.stochasticSparkle
shader.subCellShake
shader.terminalFire
shader.terminalWater
shader.tracePath
shader.tracePropagation

[style]
style.baseStyleOverride
style.colorFade
style.fadeIn
style.fadeOut
style.inner
style.italicWindow
style.moduloColumns
style.moduloRows
style.neonFlicker
style.nonEmpty
style.outerBand
style.pulse
style.colorShift
style.glitch
style.rainbow
style.rigidShakeStyle
style.spatial
```

## Current Paused Worktrees

Parallel slice work is paused until the integrated compositor-next v3.1 code is
back in OFPF-shaped files. Do **not** delete, prune, reset, overwrite, or merge
these worktrees while performing the structural correction. They are preserved
recovery material for later review. A leader-provided worktree is already enough
isolation; agents must not create nested clones, nested worktrees, or copied
compositor crates inside it.

Preserved slice worktrees:

| Primitive | Worktree | Branch | Status |
| --- | --- | --- | --- |
| `shader.affordanceWake` | `/usr/projects/tui-vfx-slice-affordance-wake` | `slice/affordance-wake` | paused; not integrated |
| `shader.barberPole` | `/usr/projects/tui-vfx-slice-barber-pole` | `slice/barber-pole` | paused; not integrated |
| `shader.bevel` | `/usr/projects/tui-vfx-slice-bevel` | `slice/bevel` | paused; not integrated |
| `shader.borderSweep` | `/usr/projects/tui-vfx-slice-border-sweep` | `slice/border-sweep` | historical worktree; primitive already signed on `master` |
| `shader.chromaticEdge` | `/usr/projects/tui-vfx-slice-chromatic-edge` | `slice/chromatic-edge` | paused; not integrated |
| `shader.coloredOverlay` | `/usr/projects/tui-vfx-slice-colored-overlay` | `slice/colored-overlay` | paused; not integrated |
| `shader.concealedLight` | `/usr/projects/tui-vfx-slice-concealed-light` | `slice/concealed-light` | paused; not integrated |
| `shader.cursor` | `/usr/projects/tui-vfx-slice-cursor` | `slice/cursor` | paused; not integrated |
| `shader.diffusion` | `/usr/projects/tui-vfx-slice-diffusion` | `slice/diffusion` | paused; not integrated |
| `shader.edgeSheen` | `/usr/projects/tui-vfx-slice-edge-sheen` | `slice/edge-sheen` | paused; not integrated |
| `shader.focusField` | `/usr/projects/tui-vfx-slice-focus-field` | `slice/focus-field` | historical worktree; primitive already signed on `master` |
| `shader.focusedRowGradient` | `/usr/projects/tui-vfx-slice-focused-row-gradient` | `slice/focused-row-gradient` | paused; not integrated |
| `shader.fractionalStripeOverlay` | `/usr/projects/tui-vfx-slice-fractional-stripe-overlay` | `slice/fractional-stripe-overlay` | paused; not integrated |
| `shader.glistenBand` | `/usr/projects/tui-vfx-slice-glisten-band` | `slice/glisten-band` | historical worktree; primitive already signed on `master` |
| `shader.glitchLines` | `/usr/projects/tui-vfx-slice-glitch-lines` | `slice/glitch-lines` | paused; not integrated |
| `shader.radar` | `/usr/projects/tui-vfx-slice-radar` | `slice/radar` | paused; not integrated |
| `shader.revealWipe` | `/usr/projects/tui-vfx-slice-reveal-wipe` | `slice/reveal-wipe` | implementation material exists; blocked from integration until OFPF structure is corrected and diff is rebased/re-reviewed |
| `shader.wayfindingNode` | `/usr/projects/tui-vfx-slice-wayfinding-node` | `slice/wayfinding-node` | paused; not integrated |

Resume order after the structural correction is committed: review/rebase
`shader.revealWipe` first, then inspect the paused slice worktrees one by one
or relaunch grounded agents if the preserved branches are too stale.

## Operating Model

The user-approved execution model is:

```text
Lead agent
  - reads the docs fully
  - writes work packets
  - coordinates warmed low-level agents
  - reviews their diffs as the senior engineer
  - runs ai-de-slop, architect review, code review
  - updates docs/signoff
  - verifies and commits each phase

Low-level coding agents
  - implement one vertical primitive slice at a time
  - use TDD red/green/refactor
  - stay inside the assigned workspace/files and exact write scope
  - do not commit
  - report changed files, unsupported decisions, tests, and risks
```

The lead should not do most implementation work when a slice can be assigned
cleanly. The lead may still make narrow integration fixes, but the preferred
pattern is to delegate coding and reserve lead attention for review, design
coherence, de-slop, and verification.

## OFPF Structure Rules for Direct v3.1 Code

`tui-vfx-compositor` is **read-only reference material** for this migration. It
is where workers inspect the hardened primitive logic and nearby file structure.
Do not edit it during compositor-next migration slices.

`tui-vfx-compositor-next` is the v3.1-native compositor. Because the crate
itself is the new v3.1 path, the intended final implementation tree must not put
normal code under a `v31/` directory. A `v31/` directory makes v3.1 look like a
mode, bridge, or temporary adapter; it will pollute future packet structure.
Directory names should describe responsibilities, not the schema version.

The compositor-next tree should stay proximate to `tui-vfx-compositor/src/`:
copy/reference a primitive's proven logic from the same family directory, then
lightly update it to read canonical v3.1 schema fields directly in
`tui-vfx-compositor-next`. When a legacy file is too large or mixes
responsibilities, this migration is the chance to split it into smaller
OFPF-named modules with clear cohesion.

The rejected pattern is any new layer that translates canonical v3.1 nodes into
old compositor DTOs. Do not recreate a `v31/`, `rendering/`, `bridge/`,
`adapter/`, or `lowering/` tree that lowers to `CompositionSpec`,
`ShaderLayerSpec`, `SpatialShaderType`, or similarly legacy-shaped structures.

Forecast OFPF file tree for every primitive packet:

```text
# READ-ONLY REFERENCE — inspect only, never edit in compositor-next packets
crates/tui-vfx-compositor/src/
  context/
  filters/cls_<filter_primitive>.rs
  masks/cls_<mask_primitive>.rs
  samplers/cls_<sampler_primitive>.rs
  pipeline/
  traits/
  types/
  utils/

# WRITE TARGET — v3.1-native compositor, no src/v31 directory in final layout
crates/tui-vfx-compositor-next/src/
  context/
    cls_compositor_ctx.rs                         # expected edit only if context contract changes
    mod.rs                                        # expected edit only for module wiring

  loader/                                         # recipe load boundary, likely future extractable SSOT
    cls_loaded_recipe.rs                          # load-accepted canonical v3.1 wrapper
    cls_load_error.rs                             # validation/load diagnostics
    mod.rs                                        # narrow export only

  validation/                                     # recipe-load validation; no runtime lowering
    col_direct_input.rs                           # shared validation input helpers
    fnc_validate_source_inputs.rs                 # source input validation
    orc_validate_render_contract.rs               # dispatch only; no primitive logic hub
    mod.rs                                        # narrow export/dispatch edit only
    shaders/
      fnc_validate_<shader_primitive>_inputs.rs   # one shader validation file per slice
      mod.rs                                      # narrow export/dispatch edit only
    filters/
      fnc_validate_<filter_primitive>_inputs.rs   # create only when first filter slice lands
      mod.rs
    masks/
      fnc_validate_<mask_primitive>_inputs.rs     # create only when first mask slice lands
      mod.rs
    samplers/
      fnc_validate_<sampler_primitive>_inputs.rs  # create only when first sampler slice lands
      mod.rs

  source/                                         # source materialization from canonical v3.1 source fields
    col_source_grid_from_text.rs                  # expected edit only for source field semantics
    fnc_source_grid_from_inputs.rs                # expected edit only for source field semantics
    mod.rs                                        # narrow export edit only

  render/                                         # direct render orchestration; not a lowerer
    cls_frame.rs                                  # direct-render frame output type
    cls_render_error.rs                           # direct-render diagnostics
    cls_sample_context.rs                         # explicit sample context
    col_collect_graph_step_nodes.rs               # graph node collection helper
    fnc_render_recipe.rs                          # thin orchestration only
    mod.rs                                        # narrow export/dispatch edit only

  shaders/                                        # shader.* primitive execution from canonical v3.1 fields
    cls_shader_node.rs                            # typed direct shader node/value wrapper if needed
    col_shader_input.rs                           # shared shader input helper if needed
    fnc_<shader_primitive>_style.rs               # one shader primitive implementation per file
    mod.rs                                        # narrow dispatch/export only

  filters/                                        # filter.* runtime family, proximate to legacy compositor tree
    cls_<filter_primitive>.rs                     # expected edit/new for filter slices
    fnc_<filter_primitive>_<helper>.rs            # split-out helper when legacy file is oversized/mixed
    test_cls_<filter_primitive>.rs                # expected new/edit for focused unit tests when useful
    mod.rs                                        # narrow export/registration edit only

  masks/                                          # mask.* runtime family, proximate to legacy compositor tree
    cls_<mask_primitive>.rs                       # expected edit/new for mask slices
    col_<mask_helper>.rs                          # expected new only for small shared mask helpers
    fnc_<mask_primitive>_<helper>.rs              # split-out helper when needed
    mod.rs                                        # narrow export/registration edit only

  samplers/                                       # sampler.* runtime family, proximate to legacy compositor tree
    cls_<sampler_primitive>.rs                    # expected edit/new for sampler slices
    fnc_<sampler_primitive>_<helper>.rs           # split-out helper when needed
    mod.rs                                        # narrow export/registration edit only

  pipeline/                                       # compositor execution orchestration, not schema lowering
    orc_render_pipeline.rs                        # expected edit only if direct primitive execution requires it
    fnc_render_pipeline_*.rs                      # expected edit only for existing pipeline behavior
    cls_composition_spec.rs                       # legacy-readonly/remove deliberately; do not use for v3.1 slices
    cls_shader_layer_spec.rs                      # legacy-readonly/remove deliberately; do not use for v3.1 slices
    mod.rs                                        # narrow export/registration edit only

  traits/                                         # stable runtime traits
    filter.rs                                     # expected edit only for real trait contract change
    mask.rs                                       # expected edit only for real trait contract change
    sampler.rs                                    # expected edit only for real trait contract change
    mod.rs                                        # narrow export edit only

  types/                                          # runtime value types, not bridge DTOs
    cls_<primitive_or_value_type>.rs              # expected edit/new only when a runtime type is needed
    cls_filter_spec.rs                            # should-not-use for new v3.1 path unless deliberately removed
    cls_mask_spec.rs                              # should-not-use for new v3.1 path unless deliberately removed
    cls_sampler_spec.rs                           # should-not-use for new v3.1 path unless deliberately removed
    mod.rs                                        # narrow export edit only

  utils/                                          # small pure helpers only
    fnc_<specific_helper>.rs                      # expected new only if reused and under OFPF size guidance
    mod.rs                                        # narrow export edit only

crates/tui-vfx-compositor-next/tests/direct_recipe/
  support.rs                                      # shared test helpers only
  test_load_contract.rs                           # general loader/strictness contract
  test_<family>_<primitive>.rs                    # one primitive contract/e2e test file per slice

docs/arch/compositor-next-agent-workflow-handoff.md # expected edit for scoreboard/signoff only
```

Packet authors must paste the relevant subset of this tree into every dispatched
primitive packet and mark each path as read-only reference, expected edit,
expected new file, generated, or should-not-touch. If a slice needs a path
outside the forecast subset, the agent must stop and report the proposed
deviation before broad edits. The leader must reject work that invents a new
top-level layout, adds a versioned path such as `src/v31/`, adds a `rendering/`
adapter tree, expands hub files with primitive logic, edits
`tui-vfx-compositor`, or creates files not explained by the packet tree.

OFPF guardrails for future slices:

- Do not add primitive logic to a shared hub file.
- Do not add legacy DTO adapters or lowering modules.
- Add one validation module and one direct v3.1 primitive implementation module
  per migrated primitive when the primitive needs code.
- Keep orchestration files as dispatch surfaces only.
- Split any file approaching the project size guide rather than rationalizing a
  growing hub. The normal target is about 300 LOC; anything above 500 LOC
  requires an explicit cohesion justification before commit.
- Keep tests split by contract/primitive so a failed primitive slice has an
  obvious owner.

## Per-Slice Contract

Every primitive slice must be vertical and complete before signoff. The worker report must list every created or edited file and identify whether it was edited-existing, new-authored, copied, moved, or generated:

1. Inspect the v3.1 descriptor entry.
2. Inspect existing copied compositor/style implementation.
3. Add or update a failing regression first.
4. Observe RED when practical and record if the clean RED step is impossible.
5. Implement the smallest direct v3.1 renderer/load-validation support.
6. Reject unsupported descriptor-valid semantics at `LoadedV31Recipe::load`.
7. Accept only descriptor-canonical v3.1 values.
8. Reject unresolved runtime-sourced inputs for the current direct path.
9. Run targeted tests.
10. Run ai-de-slop on touched files.
11. Run architect review and code review.
12. Update docs/signoff artifacts.
13. Run full phase verification.
14. Commit before starting the next phase on `master`.

Documentation is part of the phase. A slice is not ready for review until code,
tests, generated artifacts, hand-maintained docs, and signoff notes are updated.

## Direct v3.1 Rules Learned So Far

- `shader.focusField` ellipse/circle support maps to compositor `FocusFieldShape::Ellipse`.
  Rect mode and rect geometry remain unsupported in direct v3.1 load validation until
  that semantic surface is migrated deliberately.
- `shader.focusField` geometry fields that land in compositor `u16` fields
  (`centerX`, `centerY`, `radius`) must be integer-valued numeric literals at load time;
  direct v3.1 must not silently narrow fractional descriptor-valid numbers.
- `shader.focusField.applyTo` defaults to the descriptor/lowerer foreground behavior when
  absent; tests should set `background` explicitly when asserting background color changes.

- `LoadedV31Recipe::load` is the single acceptance point for direct v3.1
  execution.
- `tui-vfx-player-next` must delegate to the same compositor-next v3.1 loader
  and renderer. It must not own a second recipe-loader logic set.
- Source inputs in the current direct renderer must all be literal, even when
  the first renderer ignores some styling inputs.
- Effect inputs for supported direct primitives must all be literal.
- Do not mirror aliases from older/copy runtime internals unless those aliases
  are descriptor-canonical v3.1 values.
- Descriptor-valid-but-unsupported values should fail loudly at load time with
  `V31LoadError::UnsupportedDirectInput`.
- Current `shader.highlighter` direct decisions:
  - `mode`: supports `band`; rejects descriptor-valid `row` and `centerOut`
    until direct compositor semantics exist.
  - `applyTo`: supports `foreground`, `background`, `both`.
  - `direction`: supports `leftToRight`, `rightToLeft`, `topToBottom`,
    `bottomToTop`.
  - `textContrast`: supports only `0.0`/`TextContrast::Preserve`.
  - `rowMask`: non-negative integer maps to a single-row compositor range.

## Worker Prompt Template

Use default/no-role `gpt-5.5` low agents for coding work.

```text
Coding task: implement one vertical slice in the existing assigned repository/workspace.
Use only the assigned leader-provided workspace/worktree. Do not create a new git worktree, clone, repo copy, nested checkout, or crate copy. You are not alone in the repo; touch only the exact write-scope files and do not commit.
Primitive: <PRIMITIVE_ID>.

Follow repo workflow:
- TDD red/green/refactor.
- v3.1 only.
- No bridge/shim/legacy aliases.
- No `CompositionSpec`, `ShaderLayerSpec`, `SpatialShaderType`, or legacy-shaped lowering layer in the v3.1 path.
- Validation happens at `LoadedV31Recipe::load`.
- Primitive execution reads canonical v3.1 node/source fields directly.
- Use OFPF tools to inspect descriptors and existing copied compositor/style
  implementation.

Expected file tree / file-name breakdown must be included in every dispatched primitive packet. The packet should list likely validation, primitive-runtime, test, and docs files, marking each as expected edit/new/generated/should-not-touch. Deviations from that tree must be reported before broad edits.

Suggested scope:
- one validation module under `crates/tui-vfx-compositor-next/src/v31/validation/shaders/`
- one direct v3.1 primitive module in the same compositor-next runtime tree used by the copied primitive family
- narrow dispatch wiring only where unavoidable
- one primitive test module under `crates/tui-vfx-compositor-next/tests/v31_direct_recipe/`
- docs/signoff if needed

Keep the change minimal and vertical:
- Add a canonical v3.1 fixture/test for <PRIMITIVE_ID>.
- Prove RED unsupported when practical.
- Implement supported descriptor-canonical subset using existing copied behavior.
- Reject unsupported descriptor-valid semantics at load with
  V31LoadError::UnsupportedDirectInput.
- Validate every authored source/effect input remains literal.

Run cargo fmt and targeted tests.
Return changed files; every created or edited file with origin/action marked `edited-existing`, `new-authored`, `copied`, `moved`, or `generated`; unsupported decisions; test commands/results; and integration risks. Do not commit.
```

## Integration Procedure

When a worker finishes:

1. Inspect the assigned workspace status and diff.
2. Review the touched-file list every time before reviewing code; unexpected copied/generated/new files or broad edits are blockers.
3. Review only the slice-owned files first.
4. Run targeted tests in that workspace.
5. If acceptable, merge or cherry-pick into `master` after ensuring `master` is
   clean.
6. Resolve conflicts manually as lead; do not let agents blindly merge each
   other.
7. Run ai-de-slop on the integrated changed files.
8. Run formal architect and code review.
9. Iterate on review blockers.
10. Run full phase verification.
11. Commit in the project’s current commit-message style:

```text
<subject>

Work Context:
  - <why this phase exists>

Changes:
* <path> (Version X.Y.Z):
  - <what changed>
```

Do not add co-authors.

## Required Verification Gates

At minimum, after integrating a slice:

```bash
cargo fmt --check
git diff --check
cargo test -p tui-vfx-compositor-next --test test_v31_direct_recipe -- --nocapture
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
cargo test -p tui-vfx-compositor-next --test test_old_compositor_parity
cargo test -p tui-vfx-player --test test_compositor_next_primitive_tree
cargo test -p tui-vfx-compositor-next
cargo check -p tui-vfx-player-next
cargo clippy -p tui-vfx-compositor-next --all-targets -- -D warnings
cargo clippy -p tui-vfx-player-next --all-targets -- -D warnings
```

Run additional tests for any crate or tooling touched by the slice.

## Loader / Player Boundary Deferral

Discussion checkpoint, 2026-05-01:

- Legacy/V2 upstream consumption in `../gt-design` currently goes through
  `tui-vfx-recipes`; GTD resolves recipe JSON, validates compatibility through
  `tui_vfx_recipes::recipe::from_value`, and wraps playback with its own
  `gtd-ratatui::recipes::{RecipePlanner, RecipePlayer}`.
- `tui-vfx-player-ui` is a development convenience shell, not an upstream
  runtime API. Upstream should not depend on `player-ui`.
- `tui-vfx-player` and `tui-vfx-player-next` are headless/dev playback surfaces.
  The current v3.1 SSOT is still `LoadedV31Recipe::load` in
  `tui-vfx-compositor-next::v31`, with `tui-vfx-player-next` delegating to it.
- We are deliberately deferring any crate-boundary extraction for a v3.1
  recipe-runtime/player-core crate so vertical primitive migration can continue.

Deferred design question:

```text
Should v3.1 recipe loading/playback eventually live in:
  A. tui-vfx-recipes, evolved into the recipe-runtime crate for V2 + v3.1, or
  B. a new v3.1-native headless runtime/player-core crate that tui-vfx-recipes
     and GT-Design can delegate to during transition?
```

Current rule while deferred: keep one implementation of v3.1 acceptance logic.
Do not duplicate loader rules in player-next, player-ui, or any bridge layer.

## Schema Audit Discussion List

Deferred topics to discuss and likely act on during the post-migration schema audit:

1. **Typed transitions as recipe/schema citizens.** Keep transition kernels at the
   primitive/runtime level, but consider elevating author-facing transitions into
   typed schema entries such as `transition.crossfade`, `transition.wipe`,
   `transition.iris`, `transition.push`, `transition.dissolve`,
   `transition.morph`, `transition.stippled`, and `transition.braille`.
   Rationale: authors, themes, validators, documentation, compositor dirty-region
   hints, and AI generation all benefit from named transition intent instead of
   forcing every recipe to assemble masks/signals/blends/easing chains by hand.
   The audit should decide whether these are descriptor IDs, enum variants under
   a transition descriptor family, or theme-token references to transition
   descriptors. It should also decide how transition scope, duration/easing,
   focal/direction parameters, runtime bindings, and A→B source semantics fit
   without bloating the base primitive schema.

Do not interrupt the current vertical primitive migration for this discussion;
record evidence and revisit it when the schema audit/schema-diet pass begins.

## Paused Queue

Current queue is paused until the OFPF structural correction is committed on
`master`. Do not dispatch more agents and do not merge preserved slice worktrees
until that commit exists. When dispatch resumes, the existing assigned worktree
is the workspace; do not ask agents to create another copy inside it.

First item to revisit after the correction:

```text
shader.revealWipe — implementation material exists in preserved worktree; needs
rebased diff, OFPF-shaped files, right-to-left coverage, version/footer cleanup,
and fresh architect/code review before integration.
```

Preserved paused slice candidates after `shader.revealWipe`:

```text
shader.barberPole
shader.diffusion
shader.radar
shader.wayfindingNode
shader.affordanceWake
shader.bevel
shader.chromaticEdge
shader.coloredOverlay
shader.concealedLight
shader.cursor
shader.edgeSheen
shader.focusedRowGradient
shader.fractionalStripeOverlay
shader.glitchLines
```

Recommended later queue after the preserved lanes are reviewed or relaunched:

1. `shader.neonFlicker`
2. `shader.orbit`
3. `shader.pulseWave`
4. Remaining shader primitives by complexity and migration demand
5. Filters
6. Masks
7. Samplers
8. Style effects
9. Complex/composition primitives last

Keep the scoreboard and completed/outstanding primitive list updated after each
committed primitive so work is not repeated.

## Recovery Checklist for a Fresh Lead Agent

1. Run project orientation and fully read steering files required by this repo.
2. Read:
   - `docs/arch/compositor-next-vertical-implementation-plan.md`
   - `docs/arch/v31-schema-boundary-north-star.md`
   - `docs/arch/primitive-workbench-schema-driven-workflow.md`
   - this file
3. Check `git status --short` in `/usr/projects/tui-vfx`.
4. Check preserved worktrees only for recovery/status; do not create new ones:

```bash
git worktree list --porcelain
```

5. If warm agents are still active, wait for their reports.
6. Confirm the OFPF structural correction commit exists before reviewing any
   paused slice worktree.
7. Inspect preserved slice worktrees manually before relaunching agents; do not
   delete or reset them as cleanup, and do not create replacements by default.
8. Resume with lead review/integration, not broad implementation by the lead.

<!-- <FILE>docs/arch/compositor-next-agent-workflow-handoff.md</FILE> - <DESC>Restartable agent workflow for compositor-next direct v3.1 vertical primitive slices</DESC> -->
<!-- <VERS>END OF VERSION: 0.10.2</VERS> -->
