<!-- <FILE>docs/new_kernel/V31_PARALLEL_MIGRATION_LANE_RESULTS.md</FILE> - <DESC>Results from the first parallel V2 deprecated to v3.1 debug recipe migration lanes</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Parallel v3.1 migration lane results, accepted recipe changes, validation evidence, and blocker map.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record first parallel migration lane results and unresolved blockers.</CLOG> -->

# v3.1 Parallel Migration Lane Results

## Results first

The first parallel migration batch moved from one-recipe-at-a-time work to six coordinated lanes. Accepted recipe changes keep V2 `_DEPRECATED_` settings and preserve strict v3.1 fixture health.

Fresh leader-side evidence after integration:

```text
fixture-qc summary:
  totalRecipes: 151
  validated: 151
  validationErrors: 0
  rendered: 151
  unsupported: 0
  playerErrors: 0
  fieldCoverageUnhandled: 0
  adapterGapUnresolved: 0
  timelineSmokePassed: true
  diffSmokePassed: true
  overallStatus: pass

primitive-field-coverage summary:
  totalRecipes: 151
  totalPrimitiveInstances: 330
  usedInputFields: 1159
  handledInputFields: 1159
  usedButUnhandledInputFields: 0
  missingDescriptorInputFields: 0
  schemaDecisionNeededFields: 0
```

## Accepted migrated/corrected recipes

### Baseline

- `baseline.json` — V2 baseline message, dimensions, rounded border, colors, timing, and no-effect graph restored and strict-native validated.

### Basic filters

- `filters/filter_dot_indicator_progress_binding.json` — migrated and strict-native validated.
- `filters/filter_hover_bar_progress_binding.json` — migrated and strict-native validated.
- `filters/filter_invert.json` — V2 color behavior corrected and strict-native validated.

### Advanced filters

- `filters/filter_crt.json` — V2 message/layout/border/timing/style and phase-specific CRT settings restored and strict-native validated.
- `filters/filter_sub_pixel_bar.json` — V2 `progress=0.65`, direction, colors, animation setting, and 34x3 rounded card restored and strict-native validated.
- `filters/filter_sub_pixel_bar_progress_binding.json` — V2 progress-binding recipe migrated and strict-native validated.
- `filters/filter_vignette.json` — V2 message/layout/border/style and enter/dwell/exit vignette values restored and strict-native validated.
- `filters/filter_vignette_dithered.json` — V2 dithered vignette dwell settings restored and strict-native validated.

### Masks and samplers

Strict-native validated after V2 presentation/timing checks:

- `masks/mask_blinds.json`
- `masks/mask_checkers.json`
- `masks/mask_diamond.json`
- `masks/mask_dissolve.json`
- `masks/mask_iris.json`
- `masks/mask_materialize_center.json`
- `masks/mask_materialize_corner.json`
- `masks/mask_radial.json`
- `masks/mask_radial_square.json`
- `masks/mask_wipe.json`
- `masks/mask_wipe_fade_left_right.json`
- `samplers/sampler_crt.json`
- `samplers/sampler_faultline.json`
- `samplers/sampler_ripple.json`
- `samplers/sampler_shredder.json`
- `samplers/sampler_sinewave.json`

### Shader

- `shaders/primitives/shader_reveal_wipe.json` — V2 oracle preserved and strict-native validated with `fallbackUsed=false`, `nativeLoweringSucceeded=true`, and `shaderLayers=1`.

### Style

- `styles/style_neon_flicker.json` — recipe presentation and authored settings were corrected toward the V2 oracle without tuning variables. Exact visual parity remains blocked by backend/player behavior drift; see blocker section.

## Blockers discovered by lanes

### Style blockers

- `style.neonFlicker` — current backend/player behavior does not match V2 flicker/dimming. V2 uses the authored base pink foreground/background and time-varying darkening from `stability=0.7`; current v3.1 strict-native output mechanically renders but does not reproduce the V2 flicker rate/color behavior. Do not tune recipe variables to compensate; fix the adapter/backend semantics.
- `style.rainbow` — no player/backend adapter registered.
- `style.glitch` — no player/backend adapter registered; descriptor input kinds need owner review.
- `style.rigidShakeStyle` — no adapter registered and descriptor timing input kinds do not preserve V2 fractional values.
- `style.spatial` — structured shader payload rejected by current value-kind handling.
- runtime-bound single-cell style scope — V2 `StyleRegion::Cell` binding cannot be represented by current v3.1 scope support.

### Mask and sampler blockers

- `mask.centerWipeFadeModal` — exact V2 center/edge wipe directions not supported by strict-native backend.
- `mask.cellular` — V2 `pattern` / `cell_count` not represented by current descriptor inputs.
- `mask.noiseDither` — V2 `matrix` not represented.
- `mask.pathReveal` — V2 structured path payload not represented.
- `sampler.crtJitter` — V2 `decay_ms` not represented.

### Basic filter blockers

- `filter.bracketEmphasis*`
- `filter.edgeGrow*`
- `filter.underlineWipe*`

These require backend/player/descriptor work for exact V2 parity before migration can be accepted.

### Advanced filter blockers

- Unsupported native effects: `brailleDust`, `charsetNoise`, `colorBridgedShade`, `glistenSweep`, `interlaceCurtain`, `motionBlur`, `patternFill`, `rigidShake`, `shadeScanner`, `subCellShake`, and `subcellLight`.
- Descriptor/value-kind mismatches:
  - `kittScanner.boostSeparatorBg`: V2 boolean vs current v3.1 color expectation.
  - `pillButton.glisten`: V2 boolean vs current v3.1 number expectation.
  - `rigidShake.damping`: V2 array is not represented by current descriptor.
  - `vignette.sides`: V2 list is not represented by current enum shape.
- Timing blocker: loop-period recipes hit `UnexpectedClockPeriod` when preserving authored V2 loop periods.

### Shader blockers

- `shader.barberPole` — backend/player drift with exact V2 settings.
- `shader.linearGradient` — backend/player color drift.
- `shader.borderSweep` — backend/player sweep overpaints text cells.
- `shader.highlighter` — backend/player highlight fill drift.
- `shader.glistenBand` — current descriptor/backend shape is not ready for accepted migration.
- `shader.radar` — backend/player foreground color drift.
- Many remaining shader V2 recipes lack exact direct v3.1 targets or only have variant/binding fixtures.

### Content blockers

All assigned content recipes remain blocked rather than accepted with tuned variables. Exact V2 parity requires descriptor/player/backend support or missing v3.1 target files for fields including:

- `wrap_indicator.prefix` / `wrap_indicator.suffix`
- typewriter cursor object settings
- slide-shift shift fields
- scramble `resolve_pace`
- split-flap authenticity fields

## Integration decisions

- Accepted only recipe changes that kept fixture health at `overallStatus=pass` and `usedButUnhandledInputFields=0`.
- Reverted blocked shader approximation edits that introduced field-coverage warnings or represented drift without accepted parity.
- Kept the neon flicker recipe settings aligned to V2 intent but records exact playback as blocked; the fix belongs in adapter/backend semantics, not recipe variable tuning.
- Kept reports under `.omx/reports/` for detailed lane evidence; this document is the committed synthesis.

## Commands used for leader-side verification

```bash
cargo run -q -p tui-vfx-player-cli -- fixture-qc --recursive --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --descriptor-pack descriptors/v3.1/packs/primitive.json --json
```

```bash
cargo run -q -p tui-vfx-player-cli -- primitive-field-coverage --recursive --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --descriptor-pack descriptors/v3.1/packs/primitive.json --json
```

```bash
git -C /usr/projects/tui-vfx-recipes diff --check -- recipes/v3.1/debug_recipes
```

<!-- <FILE>docs/new_kernel/V31_PARALLEL_MIGRATION_LANE_RESULTS.md</FILE> - <DESC>Results from the first parallel V2 deprecated to v3.1 debug recipe migration lanes</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
