# ARCH-RESP-TO-PHASE_K2_19.md

## Review verdict

**ACCEPT WITH PRESSURE.**

K2.19 produced the first real visible path:

```text
v3.1 recipe
  -> RecipePlayer
  -> PlayerRenderIrReport
  -> compositor backend crate
  -> ANSI / JSON / ratatui preview output
```

That is real progress. The user can now see colored compositor-backed output, run `play-backend`, and prove scripted studio controls change backend hashes.

But K2.19 also identified the next hard blocker clearly:

```text
The compositor backend currently consumes player-resolved styled IR.
It does not yet lower recipe graph/effect nodes into real CompositionSpec content.
```

So K2.20 must stop proving that the backend seam exists and start proving that real v3.1 effects lower into compositor-native execution.

The next packet is not another blocker report. It is a results packet.

---

# Phase K2.20 — Compositor-Native Effect Lowering + Live Studio Pilot

## Executive goal

Turn K2.19’s visible IR-resolved compositor path into a bounded but real compositor-native path.

Target shape:

```text
RecipeDocument v3.1
  -> validated recipe + descriptor catalog
  -> RecipePlayer sample/runtime state
  -> PlayerRenderBackendRequest
  -> native CompositionSpec lowering
  -> render_pipeline_with_spec
  -> colored/effectful backend output
  -> CLI playback + ratatui preview + generated studio controls
```

K2.20 must prove:

```text
1. At least a bounded set of filters, masks, samplers, shaders, and styles lower into non-empty CompositionSpec.
2. At least 12 canonical recipes render with native lowering and no IR-only fallback.
3. At least 4 recipes visibly change across time/control samples through native backend hashes.
4. The ratatui studio pilot displays descriptor-derived controls and can update preview output.
```

---

# Non-negotiable rule

The final memo must start with **successful native compositor results**, not process.

The orchestrator should return:

```text
what rendered natively
which recipes
which effects lowered
which command produced the result
which artifact proves it
which hashes changed
which controls changed output
```

Only after that should it include implementation details, limitations, and verification.

---

# Critical distinction for this packet

K2.19 success:

```text
Player-resolved styled IR goes through compositor backend.
Diagnostic: playerIrAlreadyResolved.
```

K2.20 target:

```text
Recipe graph/effect nodes lower into CompositionSpec.
Diagnostic for successful demo recipes: nativeCompositionSpecApplied.
fallbackUsed=false.
compositionSpecNonEmpty=true.
```

Do not count a recipe as K2.20 native success if it only repeats the K2.19 IR-resolved path.

---

# Required backend modes

Add explicit backend composition modes:

```text
irResolved
native
auto
```

Required semantics:

```text
irResolved:
  K2.19 path. Player styled IR is lowered into compositor-compatible grid.

native:
  Direct graph/effect lowering into CompositionSpec.
  Must fail or emit structured unsupported diagnostics if an effect cannot lower.
  Must not silently fall back.

auto:
  Native lowering where supported, IR fallback where unsupported.
  Must report fallbackUsed=true when fallback occurs.
```

Required CLI flags:

```bash
--backend compositor
--composition-mode native
--composition-mode ir-resolved
--composition-mode auto
--fail-on-fallback
```

Equivalent names are acceptable, but the behavior must exist.

---

# Required backend evidence fields

Every compositor backend JSON result must include:

```text
backend
compositionMode
fallbackUsed
nativeLoweringAttempted
nativeLoweringSucceeded
compositionSpecNonEmpty
loweredNodeCount
unloweredNodeCount
loweredEffectIds[]
unloweredEffectIds[]
compositionSpecSummary
diagnostics[]
warnings[]
errors[]
renderHash
backendHash
nonDefaultStyledCells
changedCells, when before/after comparison is requested
```

`compositionSpecSummary` should include whatever counts fit the actual compositor API, for example:

```text
samplers
masks
filters
shaderLayers
shadow
timing
```

Do not invent false details; report what the adapter actually emits.

---

# Work model: 10 lanes

```text
A. Result harness and native-success gate
B. Backend request and dependency boundary
C. CompositionSpec lowering registry
D. Filter/style native lowering
E. Mask native lowering
F. Sampler native lowering
G. Shader native lowering
H. Graph topology/value bus native lowering
I. Live studio UI/control pilot
J. QA, visual artifacts, docs, de-slop, final memo
```

Each lane must produce runnable results. Lane memos must lead with commands and artifacts.

---

# Lane A — Result harness and native-success gate

## Objective

Create a harness that proves native compositor lowering, not IR fallback.

## Required output directory

```text
/tmp/k220-native-results/
```

Required artifacts:

```text
/tmp/k220-native-results/README.md
/tmp/k220-native-results/native_summary.json
/tmp/k220-native-results/native_pass_fail_table.txt
/tmp/k220-native-results/native_linear_gradient.ansi
/tmp/k220-native-results/native_border_sweep_timeline.json
/tmp/k220-native-results/native_mask_wipe_timeline.json
/tmp/k220-native-results/native_filter_tint.json
/tmp/k220-native-results/studio_live_border_sweep.txt
/tmp/k220-native-results/studio_live_pill_button.txt
/tmp/k220-native-results/studio_before_after.json
```

## Required script

Add:

```bash
./scripts/k220_native_compositor_demo.sh
```

The script must run native-mode commands with `--fail-on-fallback`.

It must fail the packet if:

```text
fallbackUsed=true for required native demo recipes
compositionSpecNonEmpty=false for required native demo recipes
loweredNodeCount=0 for required native demo recipes
```

## Required pass/fail table

The final memo must include a table like:

| recipe                                 | effects lowered         | native | fallback | artifact                      | result |
| -------------------------------------- | ----------------------- | -----: | -------: | ----------------------------- | ------ |
| `shader_linear_gradient_apply_to_both` | `shader.linearGradient` |    yes |       no | `native_linear_gradient.ansi` | PASS   |

---

# Lane B — Backend request and dependency boundary

## Objective

Give the compositor backend enough information to lower effects natively without letting the UI construct compositor internals.

K2.19 used:

```text
PlayerRenderIrReport
```

K2.20 may add a backend input DTO:

```rust
pub struct PlayerRenderBackendRequest {
    pub ir: PlayerRenderIrReport,
    pub recipe: RecipeDocument,
    pub descriptor_catalog: DescriptorCatalog,
    pub sample: PlayerSampleRequest,
    pub backend_options: PlayerRenderBackendOptions,
}
```

Exact fields may differ, but the request must be player-owned/backend-neutral and must not be assembled by the UI.

## Required boundary

```text
tui-vfx-player:
  owns backend-neutral request/output types

tui-vfx-player-backend-compositor:
  lowers player/contract data into compositor-compatible IR

tui-vfx-player-cli:
  selects backend/mode and prints output

tui-vfx-player-ui:
  selects backend/mode and renders backend output only
  must not construct SemanticScene, OwnedGrid, or CompositionSpec
```

## Acceptance

```text
- UI imports no compositor DTOs.
- CLI imports no compositor internals except through backend crate APIs.
- player core does not become compositor-specific.
- backend request can support both irResolved and native mode.
```

---

# Lane C — CompositionSpec lowering registry

## Objective

Create an explicit adapter registry for lowering v3.1 effects into `CompositionSpec`.

Required shape, conceptually:

```text
effect id -> lowering adapter
```

Required result types:

```text
lowered
unsupportedByBackend
unsupportedByDescriptor
fieldIgnoredWithWarning
requiresIrFallback
```

## Required behavior

For each node/effect:

```text
- resolve descriptor id
- resolve input values through player value resolution
- validate required fields
- map to CompositionSpec where supported
- emit structured diagnostics when not supported
- never silently ignore authored fields
```

## Required tests

Add tests proving:

```text
known supported effect lowers into non-empty spec
unsupported effect emits structured unsupported diagnostic
authored field not used by lowering emits warning or fails native mode
auto mode falls back but native mode fails
```

## Acceptance

At least 12 canonical demo recipes must show:

```text
compositionSpecNonEmpty=true
loweredNodeCount > 0
fallbackUsed=false
```

---

# Lane D — Filter/style native lowering

## Objective

Lower the simplest visually meaningful filter/style effects into native compositor spec or adapter operations.

Required minimum filter support:

```text
filter.dim
filter.tint
filter.greyscale
filter.invert
filter.pillButton, if practical
filter.fadeToCanvas, if practical
```

Required minimum style support:

```text
style.baseStyleOverride or equivalent base-style effect
style.colorFade
style.fadeIn
style.fadeOut
style.pulse, if already descriptor-backed enough
```

## Required recipes

Use existing canonical recipes where available:

```text
filters/filter_dim.json
filters/filter_tint.json
filters/filter_greyscale.json
filters/filter_invert.json
filters/filter_pill_button_progress_binding.json
styles/style_fade_in.json
styles/style_fade_out.json
styles/style_color_fade.json
```

Substitute only if a path does not exist and document the replacement.

## Required visible proof

At least:

```text
filter_tint: nonDefaultStyledCells > 0
filter_dim or greyscale: backendHash differs from baseline
pill_button or fade: backendHash changes across control/time sample
```

---

# Lane E — Mask native lowering

## Objective

Lower mask recipes into native compositor mask semantics.

Required minimum support:

```text
mask.none
mask.wipe
mask.checkers
mask.dissolve, if already straightforward
mask.noiseDither or mask.materialize, if practical
```

## Required recipes

```text
masks/mask_wipe.json
masks/mask_checkers.json
masks/mask_none.json
masks/mask_dissolve.json
masks/mask_noise_dither.json
masks/mask_materialize_center.json
```

## Required visible proof

At least two masks must show changing coverage across samples:

```text
sample 0 ms backendHash != sample 750 ms backendHash
changedCells > 0
```

## Required diagnostics

If a mask is only approximated, output:

```text
approximation=true
approximationReason
```

Do not claim full parity for approximations.

---

# Lane F — Sampler native lowering

## Objective

Lower sampler effects into native compositor sampler fields where supported.

Required minimum support:

```text
sampler.sineWave
sampler.ripple
sampler.shredder or sampler.faultLine, if feasible
sampler.radialTwist, if feasible
```

## Required recipes

```text
samplers/sampler_sinewave.json
samplers/sampler_ripple.json
samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist_v3.json
```

## Acceptance

At least two sampler recipes must:

```text
compositionSpecNonEmpty=true
loweredEffectIds includes sampler.*
backendHash differs across time sample or differs from baseline
fallbackUsed=false
```

If existing compositor API cannot support a sampler yet, that sampler must remain a structured unsupported native backend diagnostic, not a fake pass.

---

# Lane G — Shader native lowering

## Objective

Lower shader primitives/compositions into native compositor spec fields.

Required minimum support:

```text
shader.linearGradient
shader.borderSweep
shader.highlighter
shader.focusField or shader.glistenBand, if feasible
shader.revealWipe, if maps cleanly
```

## Required recipes

```text
shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/primitives/shader_linear_gradient_diagonal.json
shaders/primitives/shader_reveal_wipe.json
shaders/compositions/shader_border_sweep_position_binding.json
shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
```

## Required visible proof

At least four shader recipes must pass native mode:

```text
fallbackUsed=false
compositionSpecNonEmpty=true
nonDefaultStyledCells > 0
```

At least two must change with time or controls:

```text
borderSweep position
highlighter speed/blend
focusField center
glistenBand blend/direction
```

---

# Lane H — Graph topology/value bus native lowering

## Objective

Make native lowering respect graph topology and graph value flow for supported nodes.

K2.15/K2.16 proved graph execution in player evidence. K2.20 needs backend lowering to consume the same semantics.

## Required support

```text
sequence:
  later node can consume graph value emitted by earlier node

parallel:
  branches use pre-join snapshot
  graph values merge at join
  conflicts emit deterministic diagnostics

fallback:
  native mode must fail on unlowerable graph nodes unless the recipe is not in required native set
```

## Required recipes

Use canonical graph fixtures already added:

```text
complex/graph_io_sequence_filter_to_mask.json
complex/graph_io_parallel_merge_shader.json
complex/graph_nested_parallel_sequences.json
complex/graph_parallel_overlap_conflict_snapshot.json
```

## Required proof

At least one graph recipe must render through native mode with:

```text
loweredNodeCount >= 2
graphValueInputResolved=true
fallbackUsed=false
```

At least one conflict recipe must produce deterministic diagnostics.

Do not require visual parity for complex recipes yet, but do require correct native lowering diagnostics and non-empty spec for supported nodes.

---

# Lane I — Live studio UI/control pilot

## Objective

Move from scripted studio snapshots to a basic generated UI panel.

The studio pilot does not need to be beautiful. It needs to prove generated controls are visible and can mutate backend output.

## Required UI mode

Add:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --composition-mode native \
  --studio
```

## Required UI layout

Minimum:

```text
left: recipe/browser or controls
right: preview
bottom/status: backend hash, native/fallback, selected control, warnings
```

Generated controls must show:

```text
control id
kind
current value
descriptor/effect source
binding/signal source, if applicable
```

## Required control kinds

At least:

```text
slider / numeric input
enum select
boolean toggle, if a demo uses one
color control row or placeholder
gradientEditor placeholder row
```

## Required script mode for tests

Add a deterministic script path:

```bash
cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --composition-mode native \
  --studio \
  --script "set sweep_progress 0.75; render; quit" \
  --once
```

Equivalent CLI/UI snapshot route is acceptable, but there must be a user-visible generated control panel artifact.

## Required proof

Artifacts:

```text
/tmp/k220-native-results/studio_border_sweep_before.txt
/tmp/k220-native-results/studio_border_sweep_after.txt
/tmp/k220-native-results/studio_border_sweep_before_after.json
/tmp/k220-native-results/studio_gradient_controls.txt
```

Must prove:

```text
control rows were generated
setting a value changed backendHash
preview updated
native mode did not fall back
```

---

# Lane J — QA, docs, de-slop, and final results memo

## Objective

Keep the workspace green while forcing the output to be user-runnable.

## Required docs

Create:

```text
docs/new_kernel/K2_20_NATIVE_COMPOSITION_RESULTS.md
docs/new_kernel/K2_20_EFFECT_LOWERING_COVERAGE.md
docs/new_kernel/K2_20_NATIVE_BACKEND_DEMO_COMMANDS.md
docs/new_kernel/K2_20_STUDIO_UI_CONTROL_RESULTS.md
docs/new_kernel/K2_20_NATIVE_GRAPH_LOWERING_RESULTS.md
docs/new_kernel/K2_20_BACKEND_LIMITATIONS_AND_HOLDBACKS.md
docs/new_kernel/K2_20_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/PHASE_K2_20_NATIVE_COMPOSITOR_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_20_REVIEW_AND_DESLOP_REPORT.md
```

Update only if touched:

```text
docs/VOCABULARY.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/INDEX.md
```

## Final memo required order

The final memo must be ordered exactly:

```text
1. SUCCESSFUL NATIVE COMPOSITOR RESULTS
2. USER-RUNNABLE COMMANDS
3. NATIVE EFFECT LOWERING COVERAGE
4. LIVE STUDIO CONTROL RESULTS
5. WHAT WORKS END-TO-END
6. WHAT STILL FALLS BACK OR REMAINS HOLD-BACKED
7. VERIFICATION MATRIX
8. FILES/CRATES TOUCHED
9. REVIEW AND DE-SLOP RESULTS
10. RECOMMENDED NEXT PACKET
```

No opening prose before successful results.

---

# Required demo recipe set

The native success set must include at least 12 recipes across these families.

## Must-pass native set

```text
baseline.json

filters/filter_tint.json
filters/filter_dim.json

masks/mask_wipe.json
masks/mask_checkers.json

samplers/sampler_sinewave.json
samplers/sampler_ripple.json

shaders/primitives/shader_linear_gradient_apply_to_both.json
shaders/primitives/shader_linear_gradient_diagonal.json
shaders/compositions/shader_border_sweep_position_binding.json

styles/style_fade_in.json
styles/style_fade_out.json
```

## Preferred native set

Add as many as practical:

```text
filters/filter_greyscale.json
filters/filter_invert.json
filters/filter_pill_button_progress_binding.json

masks/mask_dissolve.json
masks/mask_noise_dither.json
masks/mask_materialize_center.json

samplers/sampler_shredder.json
samplers/sampler_faultline.json
samplers/sampler_radial_twist_v3.json

shaders/compositions/shader_highlighter_runtime_bindings.json
shaders/compositions/shader_focus_field_center_binding.json
shaders/compositions/shader_glisten_band_direction_blend_binding.json
shaders/primitives/shader_reveal_wipe.json

styles/style_color_fade.json
styles/style_pulse.json

complex/graph_io_sequence_filter_to_mask.json
complex/graph_io_parallel_merge_shader.json
```

---

# Acceptance criteria

## Required

```text
- At least 12 recipes render in compositor native mode with fallbackUsed=false.
- At least 8 recipes have compositionSpecNonEmpty=true and loweredNodeCount > 0.
- At least 4 effect families are represented in native lowering: filter, mask, sampler, shader/style.
- At least 4 recipes produce nonDefaultStyledCells > 0.
- At least 4 recipes produce changing backendHash across time or control changes.
- At least 2 studio controls update backend output in native mode.
- UI has a generated control panel or generated control snapshot artifact.
- UI preview can render native compositor output.
- Native mode fails or warns honestly for unsupported effects; it does not silently use IR fallback.
- Existing full canonical corpus gates remain green.
- implementation-readiness remains implementationBlocking=0.
```

## Preferred

```text
- 20+ recipes render in native mode.
- Direct graph/value bus native lowering works for at least 2 complex graph fixtures.
- Native backend supports highlighter/focus/glisten controls.
- Studio UI can adjust slider/select controls interactively, not only through script.
- Visual diff command compares native backend vs IR-resolved backend for supported recipes.
```

## Hard stop conditions

Stop and report failure if:

```text
- native mode silently falls back to IR-resolved output;
- compositionSpecNonEmpty is false for the claimed native successes;
- unsupported effects are marked successful without lowering;
- UI constructs compositor DTOs directly;
- controls are hard-coded to the demo recipes instead of generated from descriptors/catalog/recipe usage;
- changing controls does not change backendHash or changedCells;
- field coverage stays green by marking unused fields handled;
- player core directly depends on compositor implementation details beyond backend-neutral traits.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

## Core gates

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-backend-compositor \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract \
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-backend-compositor \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract \
  -p tui-vfx-contract-cli \
  --all-targets --all-features -- -D warnings

cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-player-backend-compositor --no-fail-fast
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player-ui --no-fail-fast
cargo nextest run --workspace --no-fail-fast
```

## Existing corpus gates

```bash
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-blockers \
  --json
```

## New native backend gates

Expected commands; adapt names only if implementation uses different exact names.

```bash
./scripts/k220_native_compositor_demo.sh
```

Representative manual commands:

```bash
cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --format ansi \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json" \
  > /tmp/k220-native-results/native_linear_gradient.ansi

cargo run -q -p tui-vfx-player-cli -- render-backend \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --format json \
  --sample-ms 750 \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  > /tmp/k220-native-results/native_border_sweep_750.json

cargo run -q -p tui-vfx-player-cli -- render-backend-timeline \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --samples 5 \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/masks/mask_wipe.json" \
  > /tmp/k220-native-results/native_mask_wipe_timeline.json
```

## New studio gates

```bash
cargo run -q -p tui-vfx-player-cli -- studio-snapshot \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --backend compositor \
  --composition-mode native \
  --fail-on-fallback \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --set sweep_progress=0.75 \
  --json \
  > /tmp/k220-native-results/studio_border_sweep_before_after.json

cargo run -q -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json" \
  --backend compositor \
  --composition-mode native \
  --studio \
  --script "set sweep_progress 0.75; render; quit" \
  --once \
  > /tmp/k220-native-results/studio_live_border_sweep.txt
```

## Docs and cleanliness

```bash
cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema

git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
```

Legacy root must remain untouched.

---

# Expected final memo

Return:

```text
docs/new_kernel/PHASE_K2_20_NATIVE_COMPOSITOR_STATUS_MEMO_TO_ARCHITECT.md
```

It must begin with:

```text
# Successful native compositor results
```

and include:

```text
- native recipe count
- exact recipes rendered
- exact effects lowered
- exact commands
- exact artifacts
- fallbackUsed status for every required demo recipe
- compositionSpecNonEmpty status for every required demo recipe
- loweredNodeCount and loweredEffectIds for every required demo recipe
- studio UI/control before-after hashes
- what still uses IR-resolved mode
- what remains backend/descriptor holdback
- verification matrix
```

---

# What K2.20 should close

A successful K2.20 closes the biggest K2.19 caveat:

```text
The player no longer merely sends resolved styled IR through the compositor.
A bounded set of v3.1 graph/effect nodes lower into compositor CompositionSpec and render natively.
```

It also moves studio from:

```text
scripted control snapshot
```

to:

```text
basic generated control panel with preview mutation
```

What will still remain after K2.20:

```text
- broader descriptor coverage for the 51 signed descriptor backlog rows;
- backend fidelity for the 118 backend holdbacks;
- visual parity oracle against legacy/reference demos;
- full template expansion implementation above canonical recipe validation;
- polished studio authoring UX.
```

The point of K2.20 is to make the system feel real: real native effects, real colored playback, real generated controls, and no more hiding behind reports.
