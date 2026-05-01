<!-- <FILE>docs/arch/tui-vfx-compost-agent-workflow-handoff.md</FILE> - <DESC>Restartable workflow for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>VERSION: 0.12.2</VERS> -->
<!-- <WCTX>tui-vfx-compost clean-sheet build handoff: pure v3.1 end-to-end, exact write scopes, no runtime bridges, no copied crates.</WCTX> -->
<!-- <CLOG>0.12.2: PATCH — reset primitive migration counters to zero, classify prior primitive work as reference only, and keep primitive fan-out blocked behind substrate completion.</CLOG> -->

# tui-vfx-compost Agent Workflow Handoff

**HARD DIRECTIVE — PURE v3.1 END TO END:** The current target is `tui-vfx-compost`, a clean-sheet pure v3.1 compositor crate. Work must bring over proven logic from `tui-vfx-compositor` only as read-only reference material, then adapt it to consume canonical v3.1 schema fields directly. Any attempt to adapt v3.1 into `CompositionSpec`, `ShaderLayerSpec`, `SpatialShaderType`, legacy-shaped field names, bridge/shim DTOs, or transitional lowering layers is a failure. Halt immediately if work starts adding that kind of adapter. This build is also the opportunity to split large legacy files into OFPF-compliant, professionally named, size-guideline-respecting modules.


## Broader Task Anchor — Never Lose Sight Of This

The broader task is the **tui-vfx-compost clean-sheet pure v3.1 compositor
build**. The goal is to build `tui-vfx-compost` as the clean future compositor
path where a v3.1 recipe is loaded, validated, represented as canonical v3.1
state, and then rendered by compost runtime code directly.

```text
v3.1 recipe
  → recipe loader / validator
  → canonical loaded v3.1 structure
  → tui-vfx-compost runtime execution
  → rendered output
```

This means **v3.1 all the way through**. We are not repairing the
abandoned copied-crate path. We are building the clean crate shape in
`tui-vfx-compost`, using `tui-vfx-compositor` as read-only reference for robust
non-primitive runtime behavior and proven primitive logic.

The immediate work order is:

1. keep the v3.1 schema stable unless a real contract defect is proven;
2. finish/verify the basic `tui-vfx-compost` crate structure;
3. bring over the non-primitive compositor substrate from `tui-vfx-compositor`
   into the clean compost layout, adapting it to pure canonical v3.1;
4. then resume primitive slices on top of that substrate.

The work is not a bridge, shim, legacy lowering layer, second data model, broad
recipe migration, horizontal compatibility project, or per-primitive repo/crate
copying exercise. The intended migration is:

```text
existing hardened compositor behavior
        │
        ▼
read-only reference from tui-vfx-compositor
        │
        ▼
clean OFPF-shaped tui-vfx-compost modules
        │
        ▼
updated to read v3.1 schema fields directly
        │
        ▼
validated with real canonical v3.1 recipe slices
```

## Purpose

This document is the restart point for the current `tui-vfx-compost` clean-sheet execution workflow. If a session is interrupted, a fresh lead agent should read this file
after the normal project orientation and then continue from the state recorded
below.

The work is **pure v3.1 direct compost construction**, not generic V3 work and not copied-crate repair.
The target path is:

```text
canonical v3.1 RecipeDocument
  → LoadedV31Recipe::load(...)
  → load-time descriptor/catalog/direct-render validation
  → tui-vfx-compost runtime implementation reads v3.1 nodes/source directly
  → robust referenced runtime/primitive logic, reshaped to v3.1 schema fields
```

No new bridge, shim, legacy-input, alias-acceptance, `CompositionSpec` lowering, `ShaderLayerSpec` lowering, `SpatialShaderType` adapter, or other transition layer should be added. If a work packet starts adding one, stop and report the packet as failed.

## Current Target Crate

The current direction is to build `tui-vfx-compost` as the clean v3.1 compositor
crate instead of continuing to repair abandoned copied-crate work. `tui-vfx-compositor`
remains read-only reference material. `tui-vfx-compost` already has the basic
family layout and first shader prototype/candidate; the next step is to bring over the
non-primitive runtime substrate needed for real scenes and rendering:

```text
loader/      accepts canonical v3.1 recipes once
validation/  rejects unsupported direct-render inputs at load time
source/      materializes canonical source inputs
render/      owns frame/context/render orchestration
shaders/     contains only signed migrated shader slices
filters/     empty until first filter slice is migrated
masks/       empty until first mask slice is migrated
samplers/    empty until first sampler slice is migrated
content/     empty until first content slice is migrated
styles/      empty until first style slice is migrated
```

No primitive is currently signed as migrated in `tui-vfx-compost`.
`shader.linearGradient` remains the likely first primitive candidate to redo
after the non-primitive substrate is complete; any existing linear-gradient code
is reference/prototype material until it passes the new substrate-first gate. The primitive
family directories now include README anchors for future agents:
`src/shaders/`, `src/filters/`, `src/masks/`, `src/samplers/`, `src/content/`, `src/styles/`, and the matching
`src/validation/{filters,masks,samplers,content,styles}/` directories. Those READMEs define
what belongs in each directory and show example OFPF file names. Any future crate/package rename must be a Cargo/package/API wiring change, not
another architecture migration.

## Current Scoreboard

Descriptor pack: `descriptors/v3.1/packs/primitive.json`

Current descriptor-effect count:

```text
120 v3.1 effect descriptors
```

Active `tui-vfx-compost` signed primitive migrations:

```text
0 / 120
```

No primitive is currently complete in the active compost path.

Historical copied-crate/direct-v3.1 work is reference material only. It is not
counted as active compost progress and must be re-reviewed or redone against the
current substrate-first plan before use.

Remaining active compost migration count:

```text
120 / 120
```

Human-facing progress banner:

```text
╔══════════════════════════════════════════════════════════════╗
║ tui-vfx-compost DIRECT v3.1 SCOREBOARD                      ║
╠══════════════════════════════════════════════════════════════╣
║ Active compost signed:       0 / 120  ░░░░░░░░░░░░  0.0%   ║
║ Historical reference only:   5 proofs, not counted          ║
║ Active phase: non-primitive substrate migration             ║
║ Queue: primitive slices paused until substrate is green      ║
╚══════════════════════════════════════════════════════════════╝
```

## Primitive Completion Tracker

Source of truth: `descriptors/v3.1/packs/primitive.json` effect descriptors. Update this list in the same commit that signs off a primitive.

Active compost primitive migrations:

```text
none
```

Historical direct v3.1 primitive work retained as reference only:

```text
shader.linearGradient
shader.highlighter
shader.glistenBand
shader.focusField
shader.borderSweep
```

Outstanding primitive descriptors for future compost migration:

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

Parallel slice work is paused. The preserved worktrees are recovery/reference
material only; do not merge them until the clean `tui-vfx-compost` substrate is
ready and each diff is re-reviewed against the current schema and file layout. Do **not** delete, prune, reset, overwrite, or merge
these worktrees while performing substrate migration. They are preserved
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
| `shader.revealWipe` | `/usr/projects/tui-vfx-slice-reveal-wipe` | `slice/reveal-wipe` | implementation material exists; blocked until compost substrate packets are complete and green, then must be rebased/re-reviewed |
| `shader.wayfindingNode` | `/usr/projects/tui-vfx-slice-wayfinding-node` | `slice/wayfinding-node` | paused; not integrated |

Resume order after the compost substrate is ready: review/rebase
`shader.revealWipe` first only if it still maps cleanly to the current compost
layout; otherwise inspect the paused slice worktrees one by one or relaunch
grounded agents if the preserved branches are too stale.

## Operating Model

The user-approved execution model for future delegated slices is:

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

`tui-vfx-compositor` is **read-only reference material**. It is where we inspect
the hardened non-primitive runtime logic, primitive logic, and nearby file
structure. Do not edit it during compost work.

`tui-vfx-compost` is the clean-sheet pure v3.1 compositor crate. Because the
crate itself is the v3.1 path, normal code must not live under a `v31/`
directory. A versioned runtime directory makes v3.1 look like a mode, bridge, or
temporary adapter; it will pollute future packet structure. Directory names
should describe responsibilities, not schema versions.

The compost tree should stay conceptually proximate to `tui-vfx-compositor/src/`
where that helps orientation, but it should not copy whole directories or carry
legacy DTO structure forward. Bring over the minimum non-primitive runtime
substrate needed for scenes, sources, render context, frame output, sampling,
loop/procedural behavior, and pipeline orchestration, then adapt it to canonical
v3.1 structures directly.

The rejected pattern is any new layer that translates canonical v3.1 nodes into
old compositor DTOs. Do not recreate a `v31/`, `rendering/`, `bridge/`,
`adapter/`, or `lowering/` tree that lowers to `CompositionSpec`,
`ShaderLayerSpec`, `SpatialShaderType`, or similarly legacy-shaped structures.

Forecast OFPF file tree for every primitive packet:

```text
# READ-ONLY REFERENCE — inspect only, never edit in compost packets
crates/tui-vfx-compositor/src/
  context/
  filters/cls_<filter_primitive>.rs
  masks/cls_<mask_primitive>.rs
  samplers/cls_<sampler_primitive>.rs
  pipeline/
  traits/
  types/
  utils/

# WRITE TARGET — clean-sheet v3.1 compost crate, no src/v31 directory
crates/tui-vfx-compost/src/
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

crates/tui-vfx-compost/tests/direct_recipe/
  support.rs                                      # shared test helpers only
  test_load_contract.rs                           # general loader/strictness contract
  test_<family>_<primitive>.rs                    # one primitive contract/e2e test file per slice

docs/arch/tui-vfx-compost-agent-workflow-handoff.md # expected edit for status/signoff only
```


Forbidden legacy DTO filenames if present from earlier recovery work:

```text
cls_composition_spec.rs
cls_shader_layer_spec.rs
```

Those names must not be used by compost v3.1 execution and should be removed
or quarantined deliberately when safe.

Packet authors must paste the relevant subset of this tree into every dispatched
primitive packet and mark each path as read-only reference, expected edit,
expected new file, generated, or should-not-touch. If a slice needs a path
outside the forecast subset, the agent must stop and report the proposed
deviation before broad edits. The leader must reject work that invents a new
top-level layout, adds a versioned path such as `src/v31/`, adds a `rendering/`
adapter tree, expands hub files with primitive logic, edits
`tui-vfx-compositor`, edits copied-crate paths, or creates files not explained by the packet tree.

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
2. Inspect existing reference compositor/style implementation.
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
- `shader.focusField.channelTarget` defaults to foreground behavior when absent;
  tests should set `background` explicitly when asserting background color changes.

- `LoadedV31Recipe::load` is the single acceptance point for direct v3.1
  execution.
- `tui-vfx-player-next` or any future player path must delegate to the same
  compost v3.1 loader and renderer. It must not own a second recipe-loader logic set.
- Source inputs in the current direct renderer must all be literal, even when
  the first renderer ignores some styling inputs.
- Effect inputs for supported direct primitives must all be literal.
- Do not mirror aliases from older/copy runtime internals unless those aliases
  are descriptor-canonical v3.1 values.
- Descriptor-valid-but-unsupported values should fail loudly at load time with
  `V31LoadError::UnsupportedDirectInput`.
- Current `shader.highlighter` direct decisions:
  - `highlightMode`: supports `band`; rejects descriptor-valid `row` and
    `centerOut` until direct compositor semantics exist.
  - `channelTarget`: supports `foreground`, `background`, `both`.
  - `sweepDirection`: supports `leftToRight`, `rightToLeft`, `topToBottom`,
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
- Use OFPF tools to inspect descriptors and existing reference compositor/style
  implementation.

Expected file tree / file-name breakdown must be included in every dispatched primitive packet. The packet should list likely validation, primitive-runtime, test, and docs files, marking each as expected edit/new/generated/should-not-touch. Deviations from that tree must be reported before broad edits.

Suggested scope for current compost work:
- one validation module under `crates/tui-vfx-compost/src/validation/shaders/`
- one direct v3.1 primitive module under `crates/tui-vfx-compost/src/shaders/`
- narrow dispatch wiring only where unavoidable
- one primitive test module under `crates/tui-vfx-compost/tests/direct_recipe/`
- docs/signoff if needed

If work later moves to a future production crate name, preserve the
same non-versioned family layout. Do not introduce `src/v31/` paths.

Keep the change minimal and vertical:
- Add a canonical v3.1 fixture/test for <PRIMITIVE_ID>.
- Prove RED unsupported when practical.
- Implement supported descriptor-canonical subset using existing referenced behavior.
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

Substrate phase minimum gates:

```bash
cargo fmt --check
git diff --check
cargo test -p tui-vfx-compost --test direct_recipe -- --nocapture
cargo test -p tui-vfx-compost
```

Primitive-slice integration gates add the relevant player/runtime proof only
when that slice touches a player path or end-to-end playback contract:

```bash
cargo test -p tui-vfx-player-next --test player_next_v31 -- --nocapture
cargo check -p tui-vfx-player-next
```

Full phase/release gates add clippy once the crate is ready for that level of
enforcement:

```bash
cargo clippy -p tui-vfx-compost --all-targets -- -D warnings
cargo clippy -p tui-vfx-player-next --all-targets -- -D warnings
```

Run additional tests for any crate or tooling touched by the slice or substrate
packet.

## Loader / Player Boundary Deferral

Discussion checkpoint, 2026-05-01:

- Legacy/V2 upstream consumption in `../gt-design` currently goes through
  `tui-vfx-recipes`; GTD resolves recipe JSON, validates compatibility through
  `tui_vfx_recipes::recipe::from_value`, and wraps playback with its own
  `gtd-ratatui::recipes::{RecipePlanner, RecipePlayer}`.
- `tui-vfx-player-ui` is a development convenience shell, not an upstream
  runtime API. Upstream should not depend on `player-ui`.
- `tui-vfx-player` and `tui-vfx-player-next` are headless/dev playback surfaces.
  The current v3.1 SSOT is still the active direct v3.1 loader entry point,
  with any player-next path delegating to that same loader rather than owning
  duplicate recipe acceptance logic.
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

## Schema Audit Status And Future Watchlist

Completed schema-audit items:

1. **Native transition model.** Typed transitions are now first-class v3.1
   contract citizens with executable tracks, preserved author intent, required
   interruption policy, and required reduced-motion policy. The authoritative
   rationale and shape are in `docs/arch/v31-native-transition-model.md`.
2. **Ambiguous field-name audit.** Canonical v3.1 contract schemas and primitive
   descriptors no longer use broad ambiguous names such as `type`, `target`,
   `source`, `progress`, `amount`, `mode`, `direction`, `motion`, `speed`,
   `color`, `applyTo`, or `affect` except for narrow, documented allowlist cases.

Future schema-diet/watchlist items are allowed only after concrete example or
primitive evidence:

- canonicalize 10–15 representative v3.1 examples and note pain points;
- consider whether typed signal expressions, shadow payloads, shared rhythm,
  phase-variant inputs, or non-transition capability variants deserve promotion;
- continue shrinking descriptor repetition without adding runtime bridges or
  legacy aliases.

Do not interrupt vertical primitive migration for speculative schema expansion.
Record evidence and revisit only when canonical examples or compositor slices
show a real contract defect.

## Paused Queue

Current queue is paused for non-primitive substrate migration. Do not dispatch
primitive agents and do not merge preserved slice worktrees until the compost
substrate packets are complete, green, reviewed, and committed. When dispatch
resumes, the existing assigned worktree is the workspace; do not ask agents to
create another copy inside it.

First primitive candidate to revisit after substrate completion:

```text
shader.revealWipe — implementation material exists in preserved worktree; use it
only if it rebases cleanly to the current compost layout and passes current
schema names, right-to-left coverage, version/footer cleanup, and fresh
architect/code review before integration.
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
   - `docs/arch/tui-vfx-compost-vertical-implementation-plan.md`
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

<!-- <FILE>docs/arch/tui-vfx-compost-agent-workflow-handoff.md</FILE> - <DESC>Restartable workflow for the tui-vfx-compost clean-sheet pure v3.1 compositor build</DESC> -->
<!-- <VERS>END OF VERSION: 0.12.2</VERS> -->
