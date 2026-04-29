<!-- <FILE>docs/new_kernel/PHASE_K2_7_K2_8_GUI_AND_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.7/K2.8 GUI and migration status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Clean-room v3.1 player GUI, fixture QC, and first migration mapping batch status.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — report GUI root support, fixture QC command, migration batch, and verification.</CLOG> -->

# K2.7/K2.8 GUI and Migration Status Memo to Architect

## Rolling context

Completed before this packet:

```text
K2.1 migration-gap
K2.2 visual-frame report
K2.3 primitive adapter burn-down
K2.4 styled-cell substrate foundation
K2.5 styled primitive adapter burn-down
K2.6 GUI PRD, primitive field coverage, migration loop PRD, timeline/diff
```

Current packet delivered:

```text
K2.7/K2.8 boundary packet:
- clean-room Ratatui player UI startup-root polish over tui-vfx-player
- fixture-qc composed evidence command
- first represented-family migration mapping batch
- six clean canonical fixture variants
```

Coming next:

```text
- machine-readable migration mapping report if we want repeatable per-recipe classification in CLI
- descriptor-design review for simple mask/style/shader expansion
- GUI interaction/backend expansion
- compositor backend adapter investigation, still behind an explicit player/backend seam
```

## GUI implementation summary

The existing clean-room Ratatui UI crate remains the GUI surface:

```text
crates/tui-vfx-player-ui
```

This packet added portable startup root handling:

```bash
cargo run -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes"

cargo run -p tui-vfx-player-ui -- \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recipes-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --recipe "$RECIPE_REPO/recipes/v3.1/debug_recipes/baseline.json"
```

Touched UI symbols:

| Path | Symbols |
|---|---|
| `crates/tui-vfx-player-ui/src/cls_cli_options.rs` | `CliOptions::recipes_root` |
| `crates/tui-vfx-player-ui/src/fnc_parse_cli_options.rs` | `parse_cli_options` |
| `crates/tui-vfx-player-ui/src/fnc_find_startup_recipe_path.rs` | `find_startup_recipe_path` |
| `crates/tui-vfx-player-ui/src/cls_player_ui_state.rs` | `PlayerUiState::recipes_root` |
| `crates/tui-vfx-player-ui/src/cls_player_ui_app.rs` | `PlayerUiApp::new` root selection |
| `crates/tui-vfx-player-ui/tests/test_fnc_player_ui.rs` | root/recipe parser regression and portable path helpers |

The UI still consumes `tui-vfx-player` state and evidence. It is not a validator and does not depend on the legacy recipes runtime.

## GUI UX borrowed from `../tui-vfx-recipes/examples/demo.rs`

Preserved from the existing UI and documented as still in-bounds:

```text
browser / preview layout
keyboard workflow
help modal
status strip
reload/browser refresh behavior
pause/resume
motion-disabled stable sample mode
phase/sample scrubbing
render-hash diagnostics
canvas/substrate status via player visual-frame metadata
```

Not borrowed:

```text
legacy recipe loading authority
legacy fallback behavior
old runtime dependencies
old schema semantics
direct dependency on tui-vfx-recipes runtime
```

## Clean-room and compositor boundary confirmation

`tui-vfx-contract` remains pure DTO/schema/validation. `tui-vfx-player` remains the contract-native player/evidence layer. `tui-vfx-player-ui` sits above `tui-vfx-player`.

No compositor integration was added. The GUI does not construct compositor DTOs directly and does not reshape v3.1 contract DTOs around compositor internals. Future compositor-backed output should be an explicit adapter/lowering layer below the player-facing evidence seam.

## Fixture QC command

Added command:

```bash
cargo run -q -p tui-vfx-player-cli -- fixture-qc \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Schema:

```text
v3.1.player.fixtureQcReport.1
```

Touched player/CLI symbols:

| Path | Symbols |
|---|---|
| `crates/tui-vfx-player/src/cls_player_fixture_qc_report.rs` | `PlayerFixtureQcReport`, `PlayerFixtureQcSummary`, `PlayerFixtureQcReports`, `PlayerFixtureQcRecipe` |
| `crates/tui-vfx-player/src/fnc_build_fixture_qc_report.rs` | `build_fixture_qc_report` |
| `crates/tui-vfx-player/src/fnc_build_fixture_qc_recipe_entries.rs` | `build_fixture_qc_recipe_entries` |
| `crates/tui-vfx-player/src/fnc_build_fixture_qc_summary.rs` | `build_fixture_qc_summary`, `unresolved_adapter_gaps` |
| `crates/tui-vfx-player/src/fnc_build_fixture_qc_messages.rs` | `build_fixture_qc_warnings`, `build_fixture_qc_errors` |
| `crates/tui-vfx-player/src/fnc_build_fixture_qc_reports.rs` | `build_fixture_qc_reports` |
| `crates/tui-vfx-player-cli/src/fnc_run_fixture_qc.rs` | `run_fixture_qc` |
| `crates/tui-vfx-player-cli/src/fnc_run.rs` | `fixture-qc` dispatch |

The command reuses library functions. It does not shell out to existing commands. It embeds render, visual-frame, primitive-field-coverage, primitive-adapter-gap, timeline-smoke, and diff-smoke evidence.

Current summary:

```text
totalRecipes=22 validated=22 validationErrors=0 rendered=22 unsupported=0 playerErrors=0 visualFrames=22 fieldCoverageUnhandled=0 adapterGapUnresolved=0 timelineSmokePassed=true diffSmokePassed=true overallStatus=pass
```

## Migration batch scope

Inspected represented legacy families:

```text
filters
masks
samplers
styles
shaders/primitives
shaders/compositions
```

Read-only mapping agents completed:

```text
E1 filters + masks: PASS
E2 samplers + styles: PASS
E3 shaders/primitives + shaders/compositions: PASS
```

Full report:

```text
docs/new_kernel/K2_8_DEBUG_RECIPE_MIGRATION_BATCH_REPORT.md
```

## Canonical fixtures added

Six clean fixtures were added under `../tui-vfx-recipes/recipes/v3.1/debug_recipes`:

```text
filters/filter_dim_foreground.json
filters/filter_tint_background.json
masks/mask_wipe_right_to_left.json
samplers/sampler_sinewave_horizontal.json
shaders/primitives/shader_linear_gradient_hct.json
shaders/primitives/shader_linear_gradient_intensity_half.json
```

Legacy recipes under `../tui-vfx-recipes/recipes/debug_recipes` were not modified.

## Recipes classified but not migrated

Deferred categories:

```text
advanced filters needing new descriptors/adapters
simple mask families needing descriptor decisions: blinds, radial, iris, diamond
sampler families needing descriptors/adapters: CRT, fault-line, shredder, radial-twist
style scopes needing schema decisions: content, outer, predicate, modulo, bindable cell
HCT-first color-shift semantics needing human review
shader procedural/source/lifecycle/scene families
binding-heavy style/shader/filter variants
```

## Report counter changes

| Report | Final result |
|---|---|
| `validate-recipe` | `total=22 valid=22 invalid=0` |
| `render-recipe` | `total=22 rendered=22 unsupported=0 errors=0` |
| `render-frame` | `total=22 rendered=22 unsupported=0 errors=0` |
| `inventory-recipes` | `totalRecipes=22 rendered=22 unsupported=0 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=0 sourceIds=1` |
| `primitive-field-coverage` | `totalRecipes=22 totalPrimitiveInstances=64 usedInputFields=181 handledInputFields=181 usedButUnhandledInputFields=0 declaredButUnusedInputFields=0 missingDescriptorInputFields=0 schemaDecisionNeededFields=0` |
| `primitive-adapter-gap` | `totalEffects=14 rendered=14 stillUnsupported=0 blockedByStyledCellSubstrate=0 blockedBySemanticDecision=0 missingDescriptor=0` |
| `migration-gap` | `legacyRecipes=603 v31Recipes=22 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 readyFamilies=10 blockedFamilies=9` |
| `fixture-qc` | `overallStatus=pass` |

## Portability cleanup

The final verification pass also removed a pre-existing hard-coded recipe
checkout path from the contract CLI integration-test support after the corpus
count update touched that crate:

| Path | Symbols |
|---|---|
| `crates/tui-vfx-contract-cli/tests/support/mod.rs` | `recipe_repo_root`, `recipe_root`, `recipe_path`, `descriptor_pack_path`, `descriptor_pack_dir` |
| `crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs` | portable recipe path call sites |
| `crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs` | portable recipe and descriptor-pack path call sites |

The helpers prefer `RECIPE_REPO` when set and otherwise derive the sibling
`../tui-vfx-recipes` checkout from `CARGO_MANIFEST_DIR`.

## Verification matrix

Initial RED evidence:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli test_fnc_cli_reports_fixture_qc_for_fixture_corpus_json -- --exact
FAILED because fixture-qc was unknown.

cargo test -p tui-vfx-player-ui --test test_fnc_player_ui test_fnc_ui_parses_recipe_root_and_startup_recipe_options -- --exact
FAILED because CliOptions had no recipes_root field.
```

Green targeted evidence:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli
PASS: 21 passed

cargo test -p tui-vfx-player-ui --test test_fnc_player_ui
PASS: 5 passed
```

Final verification after de-slop, review fixes, descriptor-pack fixture fixes,
and the contract CLI portability cleanup:

```text
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli --package tui-vfx-player-ui --package tui-vfx-contract-cli -- --check
PASS

cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract-cli --all-targets -- -D warnings
PASS

cargo test --workspace
PASS

cargo run -q -p tui-vfx-contract-cli -- validate-recipe --descriptor-pack descriptors/v3.1/packs/primitive.json --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
PASS: total=22 valid=22 invalid=0

cargo run -q -p tui-vfx-player-cli -- fixture-qc --descriptor-pack descriptors/v3.1/packs/primitive.json --json --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
PASS: totalRecipes=22 validated=22 rendered=22 unsupported=0 fieldCoverageUnhandled=0 adapterGapUnresolved=0 timelineSmokePassed=true diffSmokePassed=true overallStatus=pass

git diff --check
PASS

git -C "$RECIPE_REPO" status --short -- recipes/debug_recipes
PASS: no output

rg -n '"/usr/projects/tui-vfx-recipes|/usr/projects/tui-vfx-recipes' crates/tui-vfx-player crates/tui-vfx-player-cli crates/tui-vfx-player-ui crates/tui-vfx-contract-cli
PASS: no output
```

## Review and de-slop results

AI de-slop: PASS with small cleanup. The pass narrowed player UI startup-path helpers to `&Path`, adapted the parser call site, and trimmed `cls_player_ui_state.rs` to the OFPF hard limit. UI fmt, UI clippy, and UI tests passed.

Formal reviews initially returned CHANGES REQUIRED and were addressed:

- Code review finding: `fixture-qc` smoke booleans were existence-based. Fixed by deriving pass/fail from rendered, error-free timeline/diff endpoint frames and adding a negative regression.
- Architecture review finding: four new fixture variants embedded descriptors instead of proving descriptor-pack semantics. Fixed by converting those fixtures to `descriptorPacks: [{ id: "v3.1.primitive" }]` with empty `sourceDescriptors` and empty embedded `graph.effects`.

Follow-up code review: PASS. The original smoke-status blocker is resolved; positive and negative fixture-QC regressions cover pass/fail semantics.

Follow-up architecture review: PASS. The original fixture descriptor-pack blocker is resolved; the four fixtures now prove descriptor-pack semantics. Architecture noted only that fixture-QC timeline/diff smokes are first-recipe evidence while corpus-wide failures are still covered by render/player summaries.

No unresolved formal review findings remain.

## Recommended next packet

Add a stable `migration-mapping-batch` report surface with per-recipe records and JSON schema, then use it for a reviewed descriptor-design packet over simple mask expansion. Keep the player CLI as automation authority and keep the UI as human inspection.

<!-- <FILE>docs/new_kernel/PHASE_K2_7_K2_8_GUI_AND_MIGRATION_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.7/K2.8 GUI and migration status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
