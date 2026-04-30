# ARCH-RESP-TO-PHASE_K2_16.md

## Review verdict

**ACCEPT, with two important corrections for the next phase.**

K2.16 delivered a useful and necessary player-owned `PlayerRenderIrReport` plus a `render-ir` CLI. That gives us the right evidence object for future backend, GUI, diagnostics, and studio work.

However, K2.16 did **not** materially reduce the raw descriptor/source backlog. It added 21 fixtures, but those fixtures mostly exercised already-supported descriptors. That was still valuable because it hardened render IR and added fixture coverage, but the next packet must stop circling the remaining backlog and start clearing it in high-volume tranches.

Current K2.16 state:

```text
canonical v3.1 fixtures:          88
validate-recipe:                  88 valid / 0 invalid
render-recipe:                    88 rendered / 0 unsupported / 0 errors
render-frame:                     88 rendered / 0 unsupported / 0 errors
fixture-qc:                       pass
primitive-field-coverage:         541 used / 541 handled / 0 unhandled
primitive-adapter-gap:            45 rendered / 0 unresolved
schema-readiness:                 canDeclareSchemaReady=true
explicitOwnerDecisionNeeded:      0
migration canonicalExists:        55
candidateReady:                   0
descriptorDecisionNeeded:         113
sourceDecisionNeeded:             61
ownerAuditNeeded:                 280
```

The main next-phase issue is now clear:

```text
Schema decisions are closed.
Implementation/evidence backlog remains large.
```

Do **not** reopen schema readiness. K2.17 should be a backlog-clearing implementation packet.

---

# Phase K2.17 — Descriptor/Content/Source Burn-Down + Scene/Control Closure

## Executive goal

K2.17 should aggressively reduce the remaining implementation backlog by adding real descriptors, real player adapters, real canonical fixtures, and exact disposition updates.

This is not another planning pass.

The packet should deliver:

```text
1. Large descriptor/adaptor burn-down for remaining filters, masks, samplers, shaders, and styles.

2. Source/content backlog burn-down, especially content families currently misreported as source-style blockers.

3. Scene runtime closure for visibility predicates and layer skip behavior.

4. First usable control-catalog report for future studio auto-generated controls.

5. Disposition-normalized backlog counters that clearly distinguish real implementation work from historical/raw mapping labels.
```

Use **10 parallel sub-agents**.

---

## Rolling context to include in implementer memo

Accepted so far:

```text
K2.13:
  v3.1 schema decision readiness approved.
  canDeclareSchemaReady=true.
  Source/content, graph I/O, runtime dynamism, scene/layer, templates, and studio-control boundaries accepted.

K2.14:
  descriptor/adaptor tranche 1.
  canonical fixtures 27 -> 57.
  adapter gap 0.
  field coverage gap 0.

K2.15:
  player graph topology/value-bus execution begins.
  canonical fixtures 57 -> 67.
  graph order fallback preserved.

K2.16:
  player render IR and render-ir CLI added.
  canonical fixtures 67 -> 88.
  render IR carries rows, sparse styled cells, scene/source provenance, graph values, diagnostics, sample clock fields.
```

Durable rules:

```text
Legacy debug_recipes are read-only evidence.
Canonical fixtures live only under recipes/v3.1/debug_recipes.
Templates are mandatory, compile-time only, and not runtime inheritance.
Scene/element/layer support is core v3.1 work.
Player evidence is honest evidence, not visual parity.
UI consumes player evidence and must not construct compositor internals.
Compositor/backend work belongs behind player/backend seam.
```

---

# Phase priorities

K2.17 should focus on these priority clusters in this order:

```text
1. Descriptor backlog: 113 raw descriptorDecisionNeeded records.
2. Source/content backlog: 61 raw sourceDecisionNeeded records.
3. Scene visibility/runtime gap.
4. Control catalog CLI/spec for studio reach goal.
5. Backlog report correctness and disposition clarity.
```

The target is not just more fixtures. The target is fewer blockers.

---

# Work model: 10 parallel lanes

```text
A. Metrics, disposition normalization, and migration report repair
B. Content descriptors/adapters: text-time and transforms
C. Content descriptors/adapters: odometer, split-flap, cell motion
D. Source fidelity: ANSI, image resolver seam, procedural registry
E. Filter/mask/sampler descriptor burn-down
F. Shader/style descriptor burn-down
G. Scene/layer runtime closure
H. Render IR/backend seam hardening
I. Studio control catalog CLI
J. QA, docs, schema/API sync, review/de-slop
```

Each lane should return a lane memo. The final response should consolidate into a single status memo.

---

# Non-negotiable constraints

## Legacy root remains read-only

Do not modify:

```text
../tui-vfx-recipes/recipes/debug_recipes
```

Canonical changes may occur only under:

```text
../tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## No false green

A descriptor, adapter, or field is supported only if authored fields are actually consumed by validation/player evidence.

Do not:

```text
- mark fields handled because they are merely listed;
- claim visual parity for bounded approximations;
- turn legacy aliases into canonical v3.1 schema;
- shove content effects into source descriptors just to reduce counters;
- introduce command execution;
- make UI construct compositor DTOs;
- use K2/K3 phase labels in public schema values, descriptor ids, code symbols, or durable API vocabulary.
```

## Refactor/documentation discipline

For every touched file:

```text
- simplify complexity where practical;
- improve naming;
- add or update rustdoc for public/semi-public APIs;
- add schemars descriptions when contract DTOs are touched;
- keep docs and OFPF metadata current;
- keep report schemas additive unless a real version bump is required.
```

---

# Lane A — Metrics, disposition normalization, and migration report repair

## Objective

Fix the reporting gap exposed by K2.16.

K2.16 still shows:

```text
canDeclareSchemaReady=true
schemaDecisionNeeded remains nonzero in the raw mapping view
ownerAuditNeeded=280
sourceDecisionNeeded=61
```

That creates confusion. The raw fields may be retained, but the primary report needs disposition-first implementation readiness.

## Required work

Update `migration-mapping-batch`, `schema-readiness`, or add a new command:

```bash
tui-vfx-player-cli implementation-readiness
```

Suggested schema:

```text
v3.1.player.implementationReadiness.1
```

The report should emit:

```text
schemaVersion
legacyRoot
v31Root
descriptorPacks
summary
families[]
records[]
priorityQueues[]
holdbacks[]
```

Each record should include:

```text
legacyPath
family
legacyRecipeName
canonicalPath
canonicalExists
rawStatus
disposition
implementationBlocking
blockingKind
recommendedNextAction
requiredDescriptors[]
missingDescriptors[]
requiredSources[]
requiredContentDescriptors[]
missingContentDescriptors[]
playerAdapterStatus
backendStatus
holdbackSignedOff
ownerDecisionRequired
confidence
notes[]
```

Use this disposition vocabulary:

```text
canonicalExists
candidateReady
descriptorBacklog
contentBacklog
sourceBacklog
adapterBacklog
sceneRuntimeBacklog
graphRuntimeBacklog
backendHoldback
guiHumanReviewHoldback
oracleOnly
duplicateVariant
deprecatedLegacy
explicitOwnerDecisionNeeded
```

## Required correction

Content recipes currently show up as source-style blockers such as:

```text
source.typewriterText
source.splitFlapText
source.odometer
```

That is not the vocabulary we want. K2.13 already decided:

```text
Source: produces an initial semantic surface.
Content effect: transforms/emits content over time inside a source/surface.
```

Therefore, content backlog should be represented as:

```text
content.typewriter
content.splitFlap
content.odometer
content.marquee
content.scramble
content.morph
content.redact
content.glyphCascade
content.glyphParticles
content.slideShift
content.mirror
content.numeric
content.dissolve
content.cellMotion
```

If legacy mapping still uses `source.*` names internally, convert them in the report layer and then clean the mapper.

## Baseline/final counters

Capture before and after:

```text
canonical v3.1 fixtures
canonicalExists
candidateReady
descriptorDecisionNeeded
sourceDecisionNeeded
schemaDecisionNeeded raw
ownerAuditNeeded raw
disposition counts
implementationBlocking counts
field coverage
adapter gap
fixture QC
```

## Acceptance targets

Minimum:

```text
canonicalExists: 55 -> 75+
descriptorDecisionNeeded: 113 -> <= 85
sourceDecisionNeeded/contentBacklog: 61 -> <= 40
candidateReady: remains 0 after fixtures are added
explicitOwnerDecisionNeeded: 0
```

Preferred:

```text
canonicalExists: 55 -> 90+
descriptorDecisionNeeded: 113 -> <= 65
source/content backlog: 61 -> <= 30
raw ownerAuditNeeded or disposition-normalized unresolved owner audit: materially reduced
```

## Deliverables

```text
docs/new_kernel/K2_17_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_17_IMPLEMENTATION_READINESS_REPORT.md
docs/new_kernel/K2_17_BACKLOG_DISPOSITION_REPAIR_REPORT.md
```

---

# Lane B — Content descriptors/adapters: text-time and transforms

## Objective

Burn down content backlog for low-to-medium complexity content effects.

Prioritize effects that can produce deterministic player evidence without backend/compositor support.

## Target content families

```text
content.redact
content.glyphCascade
content.glyphParticles
content.slideShift
content.mirror
content.numeric
content.dissolve
content.glitchShift
content.scrambleGlitchShift
```

## Legacy evidence to inspect

Read the relevant legacy files before descriptor design:

```text
recipes/debug_recipes/content/content_redact.json
recipes/debug_recipes/content/content_glyph_cascade_braille.json
recipes/debug_recipes/content/content_glyph_cascade_into.json
recipes/debug_recipes/content/content_glyph_particles_base_spray.json
recipes/debug_recipes/content/content_glyph_particles_options_concurrency.json
recipes/debug_recipes/content/content_slide_shift.json
recipes/debug_recipes/content/content_mirror.json
recipes/debug_recipes/content/content_numeric.json
recipes/debug_recipes/content/content_dissolve.json
recipes/debug_recipes/content/content_glitch_shift.json
recipes/debug_recipes/content/content_scramble_glitch_shift.json
```

## Descriptor design rules

Use content descriptors, not source descriptors.

Expected descriptor ids:

```text
content.redact
content.glyphCascade
content.glyphParticles
content.slideShift
content.mirror
content.numeric
content.dissolve
content.glitchShift
content.scrambleGlitchShift
```

Each descriptor should include:

```text
domain: contentTransform or contentEmitter
input fields with types/ranges/allowed values
runtimeMutability where applicable
bindable flag where appropriate
optional fields where genuinely optional
rustdoc/schemars descriptions if contract DTOs are touched
```

## Player adapter expectations

Adapters may be deterministic approximations, but they must consume fields honestly.

Suggested evidence behavior:

```text
redact:
  deterministic redaction/reveal over content cells.

glyphCascade:
  deterministic glyph replacement/cascade over target text.

glyphParticles:
  bounded transient glyph emission; no particle-engine parity claim.

slideShift:
  deterministic directional text offset/reveal.

mirror:
  deterministic horizontal/vertical text mirror transform.

numeric:
  deterministic numeric formatting/count evidence.

dissolve:
  deterministic seeded per-cell reveal/hide.

glitchShift:
  deterministic glyph shift/jitter evidence.

scrambleGlitchShift:
  composition of existing scramble plus glitch-shift behavior if honest.
```

## Canonical fixtures

Add at least 8 fixtures if adapters are honest:

```text
content/content_redact.json
content/content_glyph_cascade_braille.json
content/content_glyph_cascade_into.json
content/content_glyph_particles_base_spray.json
content/content_slide_shift.json
content/content_mirror.json
content/content_numeric.json
content/content_dissolve.json
```

Preferred additional fixtures:

```text
content/content_glitch_shift.json
content/content_scramble_glitch_shift.json
content/content_glyph_particles_options_concurrency.json
```

## Acceptance

```text
- validate/render/render-frame/fixture-qc pass.
- Timeline/diff shows real variation for animated content where expected.
- Field coverage remains 0 unhandled.
- Adapter gap remains 0 unresolved.
- implementation-readiness/source-content backlog decreases.
```

---

# Lane C — Content descriptors/adapters: odometer, split-flap, cell motion

## Objective

Address the largest source/content backlog clusters.

Current backlog includes:

```text
source.odometer:        10 records
source.splitFlapText:   19 records
source.typewriterText:  20 records
```

These should become content backlog categories, not source categories.

## Scope

Target:

```text
content.odometer
content.splitFlap
content.typewriter cursor variants
content.cellMotion
```

## Legacy evidence to inspect

```text
recipes/debug_recipes/content/content_odometer.json
recipes/debug_recipes/content/content_odometer_3x3_count_bindable.json
recipes/debug_recipes/content/content_odometer_cell_roll_diagonal.json
recipes/debug_recipes/content/content_odometer_cell_roll_dispersion_edge_in.json
recipes/debug_recipes/content/content_odometer_cell_roll_down.json
recipes/debug_recipes/content/content_odometer_cell_roll_left.json
recipes/debug_recipes/content/content_odometer_cell_roll_slot_machine.json
recipes/debug_recipes/content/content_odometer_cell_roll_up.json
recipes/debug_recipes/content/content_odometer_decimal_preset_carry.json
recipes/debug_recipes/content/content_odometer_slot_reel.json

recipes/debug_recipes/content/content_split_flap_*.json
recipes/debug_recipes/content/content_typewriter_cursor_*.json
recipes/debug_recipes/content/content_cell_motion_*.json
```

## Descriptor approach

Do not create 49 one-off descriptors.

Use bounded descriptor families:

```text
content.odometer
content.splitFlap
content.typewriter
content.cellMotion
```

Then express variants through typed inputs:

```text
direction
travel
tileWidth
tileHeight
fromMessage
cascade
speed
cycles
jitter
cursorStyle
cursorBlink
cursorWake
route
stagger
affect
```

Only add fields that are actually present and consumed.

## Player adapter expectations

```text
odometer:
  deterministic tile/roll evidence; consume direction/travel/tile fields.

splitFlap:
  deterministic character cycling/cascade evidence; consume speed/cascade/cycles where authored.

typewriter cursor:
  consume cursor style/blink/wake fields where added; do not fake unsupported cursor variants as handled.

cellMotion:
  consume route/from/to/stagger/affect for deterministic row/cell entry evidence.
```

## Canonical fixture target

Minimum:

```text
content/content_odometer.json
content/content_odometer_cell_roll_up.json
content/content_odometer_decimal_preset_carry.json
content/content_split_flap_cycles.json
content/content_split_flap_digits.json
content/content_split_flap_tile_board.json
content/content_typewriter_cursor_caret.json
content/content_typewriter_cursor_wake_tint.json
content/content_cell_motion_slice.json
```

Preferred:

```text
12–15 fixtures across odometer/splitFlap/typewriter/cellMotion.
```

## Acceptance

```text
- Content backlog decreases materially.
- No unsupported cursor or motion field is marked handled without real evidence.
- Bindable variants either work or remain explicit backlog.
- Timeline/diff smoke proves animated content changes.
```

---

# Lane D — Source fidelity: ANSI, image resolver seam, procedural registry

## Objective

Move source fidelity beyond bounded fallback text where feasible, while preserving honesty.

## Current source status

```text
source.text/card: deterministic rows + provenance.
source.ansi: SGR stripped, not styled parity.
source.image: deterministic missing-asset fallback.
source.procedural: bounded dots-spinner only.
```

## Required improvements

### `source.ansi`

Implement bounded SGR-to-styled-cell support if feasible.

Supported subset:

```text
reset
bold
italic if already supported
foreground 30–37 and 90–97
background 40–47 and 100–107
newline
plain printable characters
```

Do not implement a full VTE emulator.

If a real SGR styled-cell adapter cannot be completed honestly, keep text stripping but update report and backlog classification so it remains a source-fidelity backlog item.

### `source.image`

Introduce a formal player-owned asset resolver seam:

```rust
trait PlayerAssetResolver {
    fn resolve(&self, asset_id: &str) -> PlayerAssetResolution;
}
```

Suggested outputs:

```text
ResolvedStyledGrid
MissingAssetFallback
UnsupportedAssetDiagnostic
```

Do not implement full image rasterization unless already trivial.

### `source.procedural`

Expand bounded registry:

```text
dots_spinner
progress_bar
subcell_shape_atlas placeholder
simple_flag or braille_flag placeholder if low-risk
```

No plugins. No command execution.

## Canonical fixture target

Minimum:

```text
sources/source_ansi_sgr_basic.json
sources/source_ansi_sgr_color_grid.json
sources/source_image_binding_missing_asset.json
sources/source_image_resolver_grid_smoke.json
sources/source_procedural_dots_spinner_binding.json
sources/source_procedural_progress_bar.json
sources/source_procedural_subcell_shape_placeholder.json
```

## Acceptance

```text
- Render IR contains source provenance and source diagnostics.
- ANSI styled-cell support is either real or explicitly held back.
- Image resolver seam exists if image fidelity cannot be implemented.
- Procedural registry remains bounded and deterministic.
- No command execution.
```

---

# Lane E — Filter/mask/sampler descriptor burn-down

## Objective

Reduce the 113 descriptor-decision backlog with high-confidence primitive descriptors and player adapters.

## Priority filters

Start with descriptors that have small, clear payloads:

```text
filter.vignette
filter.bracketEmphasis
filter.dotIndicator
filter.edgeGrow
filter.hoverBar
filter.kittScanner
filter.glistenSweep
filter.underlineWipe
filter.subPixelBar
filter.glyphStyle
filter.colorBridgedShade
filter.interlaceCurtain
filter.motionBlur
filter.shadeScanner
```

Defer or hold back if field inventory shows backend/subcell dependency:

```text
filter.subcellLight
filter.subCellShake
filter.rigidShake
filter.brailleDust
filter.charsetNoise
```

These may be added only if the player can consume fields honestly.

## Priority masks

```text
mask.cellular
mask.centerWipeFade
mask.wipeFadeLeftRight
mask.materialize corner variant
mask.wipe corner variants
```

The K2.16 report lists corner wipe variants as `unspecified`. Fix this. They should map to `mask.wipe` with accepted direction vocabulary if the descriptor already supports it, or to `mask.wipe.direction` expansion if missing.

## Priority samplers

```text
sampler.crt
sampler.crtJitter
```

These are only two records and should be cleared unless the evidence proves they require backend-only behavior.

## Canonical fixture target

Minimum 12 fixtures across this lane.

Suggested:

```text
filters/filter_vignette.json
filters/filter_bracket_emphasis.json
filters/filter_dot_indicator.json
filters/filter_edge_grow_left.json
filters/filter_hover_bar.json
filters/filter_kitt_scanner.json
filters/filter_underline_wipe.json
filters/filter_sub_pixel_bar.json
masks/mask_cellular.json
masks/mask_materialize_corner.json
masks/mask_wipe_corner_out_from_top_left.json
samplers/sampler_crt.json
samplers/sampler_crt_jitter.json
```

## Acceptance

```text
- DescriptorDecisionNeeded decreases.
- Primitive adapter gap remains 0 unresolved.
- Field coverage remains 0 unhandled.
- New descriptors include ranges/allowed values where useful.
- Player adapters actually consume authored inputs.
```

---

# Lane F — Shader/style descriptor burn-down

## Objective

Reduce shader/style backlog without overclaiming backend/compositor parity.

## Priority shader/style descriptors

Start with lower-risk styled-cell descriptors:

```text
shader.coloredOverlay
shader.glow
shader.diffusion
shader.focusedRowGradient
shader.pulseWave
shader.barberPole
shader.reflect
shader.affordanceWake
shader.concealedLight
shader.edgeSheen
shader.bevel

style.colorShift
style.fadeIn
style.fadeOut
style.pulse
style.rainbow
style.italicWindow
style.cellPositionBinding
```

Potential holdbacks:

```text
shader.terminalFire
shader.terminalWater
shader.tracePath
shader.tracePropagation
shader.orbit
shader.stochasticSparkle
shader.subCellShake
shader.chromaticEdge
shader.neonFlicker
```

These may be descriptor-backed if a deterministic player approximation is honest, but they should not be claimed as visual parity.

## Built-in scope policy

Use only accepted scopes:

```text
all
role
rect
rowRange
columnRange
moduloRows
moduloColumns
nonEmpty
outerBand
inner
channel
cell if already accepted
```

Do not reintroduce generic predicate registries.

## Canonical fixture target

Minimum 10 fixtures.

Suggested:

```text
shaders/primitives/shader_glow.json
shaders/primitives/shader_diffusion_center_bg.json
shaders/primitives/shader_diffusion_center_fg.json
shaders/compositions/shader_focused_row_gradient.json
shaders/compositions/shader_pulse_wave.json
shaders/compositions/shader_barber_pole.json
shaders/compositions/shader_reflect.json
shaders/compositions/shader_concealed_light.json
styles/style_color_shift.json
styles/style_fade_in.json
styles/style_fade_out.json
styles/style_pulse.json
styles/style_cell_position_binding.json
```

## Acceptance

```text
- Shader/style descriptors consume all authored fields.
- Render IR records descriptor/style provenance.
- Timeline/diff-visible shader/style behavior is covered.
- Backend-heavy descriptors are held back explicitly rather than faked.
```

---

# Lane G — Scene/layer runtime closure

## Objective

Close the K2.16 acceptance deviation:

```text
Visibility predicates and full layer skip diagnostics are not yet represented in the canonical recipe DTO runtime path.
```

K2.17 should implement them.

## Required behavior

Implement:

```text
scene layer visibility predicates from bindings/defaults;
visibility false skips layer render and placement;
visibility true renders layer normally;
skip diagnostics include element/layer id;
render IR records layer visibility result;
z-index + authoring-order remains stable;
layer-local pipeline keeps local coordinate semantics;
transparent/empty skip policy preserves lower content;
```

## Fixtures

Add or harden:

```text
scene/scene_layer_visibility_binding_io.json
scene/scene_authoring_ladder_procedural_spinner_binding.json
scene/scene_layer_nested_parallel_sequences.json
scene/scene_layer_surface_base_style.json
scene/ansi_source_chain.json
content/content_cell_motion_slice.json
```

Add at least one explicit visibility-false fixture:

```text
scene/scene_layer_visibility_false_skips_layer.json
```

## Tests

Required tests:

```text
visibility default true renders layer
visibility default false skips layer
binding override true renders layer
binding override false skips layer
skipped layer produces render IR diagnostic/provenance
transparent/empty write preserves lower content
layer-local rect scope uses local coordinates
```

## Acceptance

```text
- Scene visibility runtime exists.
- Render IR records visibility and layer provenance.
- Scene/layer runtime backlog decreases.
- Fixture-QC remains pass.
```

---

# Lane H — Render IR/backend seam hardening

## Objective

Turn K2.16’s backend trait sketch into a real internal seam, without wiring compositor yet.

## Required internal model

Implement or formalize:

```rust
trait PlayerRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput;
}
```

Suggested types:

```text
PlayerRenderBackendOutput
PlayerRenderBackendDiagnostic
TextGridRenderBackend
StyledCellRenderBackend
```

This backend seam should live in `tui-vfx-player`, not in `tui-vfx-player-ui`.

## Required behavior

```text
Text backend consumes PlayerRenderIrReport rows.
Styled-cell backend consumes PlayerRenderIrReport styled cells.
Backend diagnostics remain player-owned.
UI remains backend-agnostic and consumes player output.
No compositor imports.
No ratatui-specific backend logic in player core.
```

## Optional CLI

If small:

```bash
cargo run -q -p tui-vfx-player-cli -- render-backend \
  --backend styled-cell \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json path/to/recipe.json
```

If not small, keep this internal and test it.

## Acceptance

```text
- PlayerRenderIrReport can feed at least one backend implementation.
- Backend output is deterministic.
- UI boundary remains clean.
- Compositor backend remains future work.
```

---

# Lane I — Studio control catalog CLI

## Objective

Start the studio reach goal with a real machine-readable control catalog.

We do not need the studio UI yet. We need the catalog the studio will consume.

## Required command

Add:

```bash
cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json
```

Preferred recipe-aware mode:

```bash
cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/content/content_typewriter.json" \
  --json
```

Suggested schema:

```text
v3.1.player.controlCatalog.1
```

## Catalog fields

Each control should include:

```text
id
label
sourceKind: descriptorInput | recipeParameter | graphSignal | sourceInput
descriptorId
nodeId if recipe-aware
inputName
valueKind
controlKind
range
allowedValues
unit
semantic
runtimeMutability
bindable
optional
defaultValue
currentValue if recipe-aware
usedBy[]
documentation
```

## Control mapping

```text
number/integer + range       -> slider + numeric input
number/integer no range      -> numeric input
boolean                      -> toggle
enum + allowedValues         -> select
color                        -> color picker/token selector
gradient                     -> gradient editor
text/string                  -> text input
duration                     -> duration input
binding-capable input        -> binding picker affordance
compile-time-only input      -> disabled at runtime / authoring only
optional input               -> enable/disable checkbox
sampledField ValueSource     -> spatial-field picker placeholder
```

## Required tests

```text
number range input becomes slider control
enum allowed values become select control
color input becomes color control
gradient input becomes gradient editor
optional input includes enable/disable affordance
bindable input includes binding affordance
recipe-aware catalog lists controls used by recipe nodes
```

## Acceptance

```text
- control-catalog CLI exists.
- Studio does not infer behavior from raw recipe internals.
- Catalog derives from descriptors and recipe graph declarations.
- No Ratatui studio UI is implemented in this packet.
```

---

# Lane J — QA, docs, schema/API sync, review/de-slop

## Objective

Keep the corpus green and documentation current.

## Required docs

Create:

```text
docs/new_kernel/K2_17_BASELINE_AND_FINAL_COUNTERS.md
docs/new_kernel/K2_17_IMPLEMENTATION_READINESS_REPORT.md
docs/new_kernel/K2_17_BACKLOG_DISPOSITION_REPAIR_REPORT.md
docs/new_kernel/K2_17_CONTENT_DESCRIPTOR_ADAPTER_TRANCHE_REPORT.md
docs/new_kernel/K2_17_SOURCE_FIDELITY_TRANCHE_REPORT.md
docs/new_kernel/K2_17_FILTER_MASK_SAMPLER_TRANCHE_REPORT.md
docs/new_kernel/K2_17_SHADER_STYLE_TRANCHE_REPORT.md
docs/new_kernel/K2_17_SCENE_LAYER_RUNTIME_CLOSURE_REPORT.md
docs/new_kernel/K2_17_RENDER_BACKEND_SEAM_REPORT.md
docs/new_kernel/K2_17_STUDIO_CONTROL_CATALOG_REPORT.md
docs/new_kernel/K2_17_HOLDBACK_REGISTER.md
docs/new_kernel/K2_17_SCHEMA_API_DOCS_GATE.md
docs/new_kernel/PHASE_K2_17_DESCRIPTOR_SOURCE_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md
docs/new_kernel/PHASE_K2_17_REVIEW_AND_DESLOP_REPORT.md
```

Update if touched:

```text
docs/VOCABULARY.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/INDEX.md
```

## Schema/API

If contract DTOs or schemas change, run regeneration and document it.

If no contract DTOs change, say so explicitly.

## De-slop requirements

Review touched files for:

```text
overclaimed descriptor support
unused fields
large functions needing helper extraction
public API missing rustdoc
stale phase labels in durable names
incorrect source/content vocabulary
hard-coded absolute paths
schema/report inconsistencies
```

---

# Optional canonical fixture additions

K2.17 should add fixtures aggressively, but only where support is honest.

Target:

```text
35–50 new canonical fixtures
```

Minimum acceptable:

```text
25 new canonical fixtures
```

A fixture may be added only if:

```text
- descriptor exists or is added in this packet;
- all authored fields are descriptor-covered;
- all authored fields are player-handled or explicitly diagnosed;
- validate-recipe passes;
- render-recipe passes;
- render-frame passes;
- fixture-qc passes;
- primitive-field-coverage remains 0 unhandled;
- primitive-adapter-gap remains 0 unresolved;
- expected_visual metadata exists;
- no legacy aliases are introduced;
- legacy source recipe is untouched.
```

Hold back problematic items instead of forcing them.

---

# Required tests

Start with RED tests where code behavior changes.

## Report tests

```text
implementation-readiness emits schemaVersion
implementation-readiness emits disposition-first summary
content recipes no longer surface as source.typewriterText/source.odometer in primary disposition output
all candidateReady records become canonicalExists or explicit backlog
```

## Content tests

```text
content.redact consumes authored fields
content.glyphCascade consumes authored fields
content.glyphParticles consumes emitter fields or remains held back
content.odometer consumes direction/travel/tile fields
content.splitFlap consumes speed/cascade/cycles fields
content.cellMotion consumes route/stagger/affect fields
timeline/diff detects animated content changes
```

## Primitive tests

```text
new filter descriptors consume all authored fields
new mask descriptors consume all authored fields
new sampler descriptors consume all authored fields
new shader/style descriptors consume all authored fields
style scopes evaluate in styled-grid evidence
```

## Scene tests

```text
visibility false skips layer
visibility true renders layer
binding override controls visibility
render IR carries visibility result
layer-local scope uses local coordinates
transparent/empty skip preserves lower content
```

## Backend/control tests

```text
PlayerRenderIrReport feeds text/styled backend output
control-catalog emits slider for numeric range
control-catalog emits select for enum allowed values
control-catalog emits color control for color input
control-catalog emits gradient editor for gradient input
recipe-aware control catalog lists used controls
```

## Regression tests

```text
existing canonical corpus remains green
graph.order fallback works
render-ir CLI works
fixture-qc pass
field coverage 0 unhandled
adapter gap 0 unresolved
schema-readiness canDeclareSchemaReady=true
legacy root mutation check clean
```

---

# Acceptance criteria

## Required

```text
- K2.17 baseline/final counters documented.
- Implementation readiness or equivalent disposition-first report exists.
- Source/content vocabulary is corrected in primary backlog output.
- At least 25 canonical fixtures added/hardened.
- At least 8 content backlog fixtures added/hardened.
- At least 12 filter/mask/sampler fixtures added/hardened.
- At least 10 shader/style fixtures added/hardened.
- Scene visibility predicate runtime works.
- Render IR records visibility/provenance for scene layers.
- control-catalog CLI exists, or a documented stop condition explains why not.
- DescriptorDecisionNeeded decreases materially.
- Source/content backlog decreases materially.
- validate-recipe passes for full canonical corpus.
- render-recipe passes for full canonical corpus.
- render-frame passes for full canonical corpus.
- fixture-qc passes for full canonical corpus.
- primitive-field-coverage remains 0 unhandled.
- primitive-adapter-gap remains 0 unresolved.
- schema-readiness remains canDeclareSchemaReady=true.
- legacy debug_recipes root remains unmodified.
```

## Preferred

```text
- 35–50 canonical fixtures added/hardened.
- canonicalExists >= 90.
- descriptorDecisionNeeded <= 65.
- source/content backlog <= 30.
- raw ownerAuditNeeded or disposition-normalized unresolved owner audit materially reduced.
- PlayerRenderBackend internal seam implemented.
- ANSI basic SGR styled-cell evidence implemented.
- asset resolver seam implemented.
- control-catalog recipe-aware mode implemented.
```

## Explicit stop conditions

Stop and report rather than forcing if:

```text
- reducing descriptor backlog requires fake field handling;
- content descriptors become mislabeled source descriptors;
- source.ansi turns into an uncontrolled full VTE implementation;
- source.image requires full rasterization;
- procedural sources require plugins or command execution;
- scene visibility requires unresolved template semantics;
- backend seam starts importing compositor internals into UI;
- control catalog starts inferring behavior from raw recipe internals instead of descriptor/schema metadata;
- contract DTO changes require unresolved schema decisions.
```

---

# Verification commands

Use portable paths:

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
```

## Format and lint

```bash
cargo fmt \
  --package tui-vfx-player \
  --package tui-vfx-player-cli \
  --package tui-vfx-player-ui \
  --package tui-vfx-contract \
  --package tui-vfx-contract-cli \
  -- --check

cargo clippy \
  -p tui-vfx-player \
  -p tui-vfx-player-cli \
  -p tui-vfx-player-ui \
  -p tui-vfx-contract \
  -p tui-vfx-contract-cli \
  --all-targets --all-features -- -D warnings
```

## Tests

```bash
cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player-ui --no-fail-fast
cargo nextest run -p tui-vfx-contract --no-fail-fast
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast
cargo nextest run --workspace --no-fail-fast
```

Fallback:

```bash
cargo test --workspace
```

## Corpus gates

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

cargo run -q -p tui-vfx-player-cli -- render-ir \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"

cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -q -p tui-vfx-player-cli -- schema-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --include-offenders \
  --json

cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## New commands if implemented

```bash
cargo run -q -p tui-vfx-player-cli -- implementation-readiness \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json

cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json

cargo run -q -p tui-vfx-player-cli -- control-catalog \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/content/content_typewriter.json" \
  --json
```

## Schema/docs gates

```bash
cargo test -p tui-vfx-contract --test test_schema_generation

UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract \
  --test test_schema_generation \
  checked_in_contract_schemas_are_current \
  -- --exact

cargo xtask docs generate
cargo xtask docs check
cargo xtask docs api
cargo xtask docs api-check
cargo xtask docs api-validate
cargo xtask audit configschema
```

## Cleanliness

```bash
git diff --check

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes

rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' \
  crates/tui-vfx-player \
  crates/tui-vfx-player-cli \
  crates/tui-vfx-player-ui \
  crates/tui-vfx-contract \
  crates/tui-vfx-contract-cli \
  docs/new_kernel
```

Legacy root mutation check must show no modifications.

---

# Expected final memo

Return:

```text
docs/new_kernel/PHASE_K2_17_DESCRIPTOR_SOURCE_BURN_DOWN_STATUS_MEMO_TO_ARCHITECT.md
```

The memo must include:

```text
- review of K2.16 acceptance deviations closed or still open;
- before/after counters;
- lane result table;
- exact fixture additions;
- descriptor additions;
- adapter additions;
- source/content vocabulary corrections;
- implementation-readiness report status;
- content backlog burn-down results;
- descriptor backlog burn-down results;
- scene visibility runtime status;
- render IR/backend seam status;
- control catalog status;
- holdback register;
- verification matrix;
- legacy root mutation status;
- review/de-slop findings;
- remaining blockers;
- recommended next packet.
```

---

# What we can close out now

We can close out **v3.1 schema decision readiness** for the debug-recipes migration. K2.13 established `canDeclareSchemaReady=true`, and K2.14–K2.16 preserved that state.

We can also close out the first major player infrastructure milestone:

```text
Player-owned render evidence exists.
Player graph topology/value-bus execution has started.
Render IR exists and can become the backend/studio input surface.
```

That is a major architectural milestone.

---

# How close we are

## Schema / DTO readiness

For the debug-recipes migration, schema decision readiness is effectively **complete**:

```text
canDeclareSchemaReady=true
explicitOwnerDecisionNeeded=0
unresolved schema blockers=0
```

Core DTO areas are in place:

```text
recipe document
descriptor pack/catalog
graph/topology
node inputs/outputs
value/value-source
gradient values
source descriptors
scene/element/layer model
scope vocabulary
lifecycle/motion/easing
runtime binding/value-source concepts
render report surfaces
player render IR
```

Outstanding DTO or DTO-adjacent work:

```text
template compiler-layer DTOs and expansion rules;
control-catalog report DTO;
backend output DTO;
possibly richer source asset resolver DTOs;
possibly richer scene clear/blend policy DTOs if runtime evidence proves current vocabulary insufficient.
```

Template support is mandatory, but it remains above runtime/player. It is not a blocker for canonical runtime recipe execution as long as templates compile into strict v3.1 recipes before validation/player execution.

## Descriptor readiness

Descriptor readiness is **not complete**.

Evidence:

```text
primitive-adapter-gap: 45 rendered / 0 unresolved for currently covered descriptors
descriptorDecisionNeeded: 113 remaining raw records
```

This means the descriptors we have are green, but the descriptor catalog does not yet cover enough of the legacy debug corpus.

The biggest remaining descriptor areas are:

```text
filters: vignette, kitt, hover, dot, underline, bracket, glyph/style/noise variants
shaders: diffusion/glow/overlay/compositions/terminal/fire/water/trace/orbit variants
styles: fade/pulse/rainbow/glitch/cell-position variants
masks: cellular, wipe/corner/fade/materialize variants
samplers: crt/crtJitter
```

## Source/content readiness

Source/content readiness is **partially complete**.

Covered:

```text
source.card
source.text
bounded source.ansi
bounded source.image fallback
bounded source.procedural
content.typewriter
content.marquee
content.splitFlap baseline
content.wrapIndicator
content.scramble
content.morph
```

Outstanding:

```text
content.odometer
content.redact
content.glyphCascade
content.glyphParticles
content.slideShift
content.mirror
content.numeric
content.dissolve
content.glitchShift
content.scrambleGlitchShift
many splitFlap/typewriter/cellMotion variants
styled ANSI parity
real image resolver/rasterization boundary
richer procedural source registry
```

The reporting currently mislabels many content items as `source.*`. K2.17 should correct that.

## Runtime/player readiness

Player readiness is improving but incomplete.

Done:

```text
render-recipe/render-frame/render-ir evidence
styled-cell evidence
topology-first graph execution
sequence/parallel value bus slice
graph diagnostics slice
scene z-order/provenance slice
```

Outstanding:

```text
full scene visibility predicate runtime
layer skip diagnostics
transparent/clear semantics beyond current row/styled evidence
real effect-output publication
richer graph value kind diagnostics
backend adapter seam implementation beyond sketch
```

## Backend/visual parity readiness

Backend visual parity is **not complete** and should not be claimed.

Held back:

```text
shadows
subcell shapes
terminal fire/water style backend effects
rich image/procedural rendering
compositor-backed parity
GUI human-review conflict fixtures
```

These are correctly held back. The next step is not to force them into schema; it is to build the backend seam after render IR stabilizes.

## Recommended completion path

The fastest route to completion is:

```text
K2.17:
  descriptor/content/source burn-down;
  scene visibility closure;
  control-catalog CLI;
  implementation-readiness report.

K2.18:
  backend adapter seam implementation and first shadow/subcell backend prototype,
  only after player backend boundary is clean.

K2.19:
  remaining descriptor tranche and visual/owner audit batches.

K3.0:
  studio control panel pilot driven by control-catalog + render-ir,
  not raw recipe introspection.

Later:
  template compiler implementation,
  visual oracle/parity review,
  release gates.
```

The immediate recommendation is clear: **run K2.17 as a large descriptor/source/content burn-down packet, not another readiness packet.**
