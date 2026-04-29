<!-- <FILE>docs/new_kernel/K2_8_DEBUG_RECIPE_MIGRATION_BATCH_REPORT.md</FILE> - <DESC>First v3.1 debug recipe migration mapping batch report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Clean-room v3.1 migration loop: classify first represented legacy families and add clean fixture variants.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record filters, masks, samplers, styles, and shader migration mapping batch.</CLOG> -->

# K2.8 Debug Recipe Migration Batch Report

## Scope

Legacy root: `../tui-vfx-recipes/recipes/debug_recipes`.
Canonical root: `../tui-vfx-recipes/recipes/v3.1/debug_recipes`.
Descriptor pack: `descriptors/v3.1/packs/primitive.json`.

This batch inspected represented primitive families only. Legacy recipes remained read-only. Canonical fixtures were added only where the old intent mapped cleanly to existing v3.1 descriptors and current player adapters.

## Baseline and result

| Evidence surface | Before | After |
|---|---:|---:|
| Canonical v3.1 recipes | 16 | 22 |
| Rendered canonical recipes | 16 | 22 |
| Unsupported canonical recipes | 0 | 0 |
| Primitive field coverage unhandled | 0 | 0 |
| Primitive adapter unresolved gaps | 0 | 0 |
| Descriptor effect ids represented | 14 / 14 | 14 / 14 |
| Legacy debug recipes | 603 | 603 |

Final fixture QC summary:

```text
totalRecipes=22 validated=22 validationErrors=0 rendered=22 unsupported=0 playerErrors=0 visualFrames=22 fieldCoverageUnhandled=0 adapterGapUnresolved=0 timelineSmokePassed=true diffSmokePassed=true overallStatus=pass
```

Transient machine evidence:

```text
${TMPDIR:-/tmp}/tui-vfx-k27-k28-final/fixture-qc.json
${TMPDIR:-/tmp}/tui-vfx-k28-migration-batch-report.json
```

## Canonical fixtures added

| New fixture | Descriptor | Why clean |
|---|---|---|
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim_foreground.json` | `filter.dim` | Exercises existing `factor` and `applyTo=foreground` inputs. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_tint_background.json` | `filter.tint` | Exercises existing `color`, `strength`, and `applyTo=background` inputs. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe_right_to_left.json` | `mask.wipe` | Exercises existing `direction=rightToLeft` input. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_sinewave_horizontal.json` | `sampler.sineWave` | Exercises existing `axis=x` and `phaseOffset` inputs. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_hct.json` | `shader.linearGradient` | Exercises existing `colorSpace=hct` input. |
| `../tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_intensity_half.json` | `shader.linearGradient` | Exercises existing `intensity=0.5` input. |

## Family classification summary

| Family | Legacy inspected | Existing canonical before | Canonical after | Clean mapping status | Main blockers |
|---|---:|---:|---:|---|---|
| `filters` | 98 total, 53 active | 4 | 6 | Current dim/greyscale/invert/tint clean; two clean variants added. | Most other filters need descriptors and player adapters; some need schema/lifecycle review. |
| `masks` | 41 total, 26 active | 4 | 5 | none/checkers/wipe/dissolve clean; right-to-left wipe added. | Blinds/radial/iris/diamond and materialize/path/noise families need descriptor decisions and adapters. |
| `samplers` | 13 total, 7 active | 2 | 3 | sineWave/ripple clean; horizontal sine wave added. | CRT, fault-line, shredder, radial-twist need descriptors and adapters. |
| `styles` | 33 total, 19 active | 2 | 2 | colorFade and baseStyleOverride role-scope clean. | content/outer/predicate/modulo scopes, HCT-first color shift, and lifecycle styles need decisions. |
| `shaders/primitives` | 39 | 1 | 3 | linearGradient clean; HCT and intensity variants added. | Most shader payloads need descriptors/source/lifecycle/scene decisions. |
| `shaders/compositions` | 39 | 1 | 1 | borderSweep clean. | Composition algebra, binding semantics, and region/scope decisions block broader migration. |

## Structured gap inventory

| Gap class | Evidence | Recommendation |
|---|---|---|
| Descriptor gaps | Most active filters beyond dim/greyscale/invert/tint; masks beyond none/checkers/wipe/dissolve; samplers CRT/fault-line/shredder/radial-twist; many shader/style families. | `addDescriptor` only after semantic review. |
| Descriptor input gaps | Binding/progress variants, border-sweep `position`, some legacy channel/apply-to shapes. | Prefer `addMigrationRule` when current descriptor already has a canonical input; otherwise `addDescriptorInput`. |
| Player adapter gaps | Any newly accepted descriptor beyond the current 14 ids. | Pair descriptor acceptance with `addPlayerAdapter`; do not create fixtures before honest render classification. |
| Source gaps | Terminal fire/water/procedural scalar shader families. | `addSourceDescriptor`, `deferForSourceWork`. |
| Scope/schema gaps | content/outer/predicate/modulo/bindable cell scopes; region refs; parallel/glyph-match shader overlay. | `addSchemaCapability` only with owner-reviewed schema decision. |
| Lifecycle/signal gaps | pulse/rainbow/flicker/shake styles; binding-heavy shader compositions; progress-binding filters. | `deferForLifecycleWork`. |
| Scene gaps | region compression and canvas-aware fade variants. | `deferForSceneWork`. |

## Representative per-recipe classifications

| Legacy path | Status | Recommendation | Notes |
|---|---|---|---|
| `filters/filter_dim.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing `filter.dim` fixture covers core shape. |
| `filters/filter_tint.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing `filter.tint` fixture covers core shape. |
| `masks/mask_wipe.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing + new direction fixture cover supported directions. |
| `samplers/sampler_sinewave.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing + new horizontal fixture cover `axis` and `phaseOffset`. |
| `styles/style_color_fade.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing fixture uses `rgb` and `hct`; do not reintroduce HSL. |
| `shaders/primitives/shader_linear_gradient.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing + new HCT/intensity variants cover current descriptor inputs. |
| `shaders/compositions/shader_border_sweep.json` | `canonicalizedCleanly` | `addMigrationRule` | Existing fixture covers current descriptor. |
| `styles/style_color_shift.json` | `needsHumanSemanticReview` | `manualReview` | Legacy HSL-like fields should become HCT-first semantics if accepted. |
| `styles/style_modulo_horizontal_every_third_row.json` | `needsSchemaDecision` | `addSchemaCapability` | Current v3.1 scopes do not include modulo scope. |
| `shaders/primitives/shader_region_compression_scope.json` | `needsSceneDecision` | `deferForSceneWork` | Region refs are not a current v3.1 scope contract. |

## Next migration action

Do not broaden fixture creation until the next packet adds a stable mapping-report surface or reviewed descriptor decisions. Recommended next migration packet: descriptor-design review for simple mask expansion (`mask.blinds`, `mask.radial`, `mask.iris`, `mask.diamond`) plus a machine-readable migration mapping report schema.

<!-- <FILE>docs/new_kernel/K2_8_DEBUG_RECIPE_MIGRATION_BATCH_REPORT.md</FILE> - <DESC>First v3.1 debug recipe migration mapping batch report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
