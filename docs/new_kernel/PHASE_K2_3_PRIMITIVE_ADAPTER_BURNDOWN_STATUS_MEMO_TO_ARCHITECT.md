<!-- <FILE>docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.3 primitive adapter burn-down status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Primitive adapter work: report adapter support, blockers, and recipes tooling review.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document K2.3 support outcomes, evidence, and tooling recommendations.</CLOG> -->

# Phase K2.3 Primitive Adapter Burn-down Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `.`
Packet source: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_2.md`

## Executive summary

K2.3 reduces the canonical primitive unsupported set from six to four by adding honest text-grid adapters for primitives that can mutate row/cell glyph evidence without styled-cell data.

Added rendered support:

```text
mask.dissolve
sampler.ripple
```

Still blocked by styled-cell substrate:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

The core invariant is preserved:

```text
render-frame still reports substrate=textGrid, cellSource=rows, styleKnown=false
```

No style/color/role primitive is marked rendered while style evidence remains placeholder-only.

## New command

```bash
RECIPE_REPO=../tui-vfx-recipes
cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

Schema:

```text
v3.1.player.primitiveAdapterGap.1
```

## Starting unsupported ids

Confirmed from both inventory and visual-frame evidence:

```text
mask.dissolve
sampler.ripple
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

Evidence:

```text
${TMPDIR:-/tmp}/tui-vfx-k23-start-inventory-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-start-visual-frame-report.json
```

## Ending unsupported ids

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

## Counts before and after

| Report | Before | After |
| --- | --- | --- |
| `render-recipe` | total=16 rendered=10 unsupported=6 errors=0 | total=16 rendered=12 unsupported=4 errors=0 |
| `inventory-recipes` | totalRecipes=16 rendered=10 unsupported=6 errors=0 unsupportedEffectIds=6 | totalRecipes=16 rendered=12 unsupported=4 errors=0 unsupportedEffectIds=4 |
| `render-frame` | total=16 rendered=10 unsupported=6 errors=0 | total=16 rendered=12 unsupported=4 errors=0 |
| `migration-gap` | legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 | unchanged |

## Adapter outcomes

| Effect id | Outcome | Adapter class | Reason |
| --- | --- | --- | --- |
| `mask.dissolve` | `rendered` | `textGrid` | Deterministic seeded dissolve can honestly hide/reveal glyph cells in text-grid rows. |
| `sampler.ripple` | `rendered` | `textGrid` | Row-wise coordinate displacement can honestly shift glyph cells in text-grid rows. |
| `shader.borderSweep` | `blockedByStyledCellSubstrate` | `styledCell` | Descriptor writes color/style data; current visual frame cells have placeholder style. |
| `shader.linearGradient` | `blockedByStyledCellSubstrate` | `styledCell` | Gradient output is color evidence, not row glyph evidence. |
| `style.baseStyleOverride` | `blockedByStyledCellSubstrate` | `styledCell` | Foreground/background override requires real style cells and role scope. |
| `style.colorFade` | `blockedByStyledCellSubstrate` | `styledCell` | Color interpolation requires real styled-cell evidence. |

## Files touched

Player library:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_entry.rs
crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_report.rs
crates/tui-vfx-player/src/cls_player_primitive_adapter_gap_summary.rs
crates/tui-vfx-player/src/fnc_apply_graph_effects.rs
crates/tui-vfx-player/src/fnc_apply_mask_dissolve.rs
crates/tui-vfx-player/src/fnc_apply_sampler_ripple.rs
crates/tui-vfx-player/src/fnc_build_primitive_adapter_gap_report.rs
crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs
crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs
crates/tui-vfx-player/src/fnc_primitive_adapter_gap_paths.rs
crates/tui-vfx-player/src/fnc_resolve_effect_input.rs
crates/tui-vfx-player/src/fnc_summarize_primitive_adapter_gaps.rs
crates/tui-vfx-player/tests/test_fnc_recipe_player.rs
```

Player CLI/tests:

```text
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_run_primitive_adapter_gap.rs
crates/tui-vfx-player-cli/src/main.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
```

Docs:

```text
docs/VOCABULARY.md
docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md
docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
```

## Recipe-side tooling review

The user explicitly requested a side-car review of the recipes tooling and a leader-side verification pass. Root paths below use the portable `RECIPE_REPO` convention from the command examples instead of hard-coding a local checkout path. I independently inspected the key files and agree with the broad recommendations, with one correction: do not adopt the old `recipe-validator` path except as a documented legacy compatibility surface.

### `pipeline-validator`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/pipeline-validator
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/pipeline-validator/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/pipeline-validator/src/cli.rs
${RECIPE_REPO:-../tui-vfx-recipes}/tools/pipeline-validator/src/fnc_run_probe_mode.rs
${RECIPE_REPO:-../tui-vfx-recipes}/docs/V3_TOOLING_COMMAND_REFERENCE.md
```

Features/options:

```text
--format text|json
--stage parse|profile|render|shader|output
--phase entering|dwelling|exiting|all
--sample-t values
--frames N
--dump
--dump-normalized
--explore-normalized
--lowering-report
--migration-equivalence-report
--trace
--rules
--rules-file FILE
--strict
--strict-contracts
--errors-only
--stages
--probe
--probe-cells all|non-empty|modified
--probe-frames N
--probe-diff-to T
--probe-causation
--probe-sqlite-query SQL
--probe-widget-cell X,Y
--runtime-params-json JSON|@FILE
--compiled-v3-source-text TEXT|@FILE
--compiled-v3-procedurals none|stock
--debug-recipes-qc
--bench
--iterations N
--canvas RRGGBB
--canvas-content empty|sentinel|lorem
```

Adopt/adapt:

```text
- Adopt the mode-based CLI architecture for future player tools.
- Adopt schema-labeled JSON report envelopes for every new report.
- Adapt `--probe-sqlite-query` into a future player trace/debug surface once player frames carry styled-cell or event traces.
- Adapt `--canvas` and `--canvas-content` when compositor/styled-cell evidence lands.
- Adapt `--debug-recipes-qc` as a future canonical fixture QC gate after v3.1 recipe migration broadens.
```

### `recipe-probe`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-probe
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-probe/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-probe/src/main.rs
${RECIPE_REPO:-../tui-vfx-recipes}/docs/RECIPE_PROBE_GUIDE.md
```

Features/options:

```text
path
--format json
--phase entering|dwelling|exiting
--sample-t N
--cells all|non-empty|modified
--with-causation
--frames N
--diff-to T
--sqlite-query SQL
--widget-cell X,Y
--canvas RRGGBB
--canvas-content empty|sentinel|lorem
```

Advanced evidence features:

```text
- Unified per-cell traces across content, style, and compositor stages.
- Operational analysis, lifecycle analysis, motion analysis, and focus-cell root-cause output.
- SQLite xray tables including probe_analysis_stages, probe_analysis_effects, probe_diagnostics, probe_motion_effects, probe_runtime_params, probe_binding_resolutions, and probe_cell_root_causes.
```

Adopt/adapt:

```text
- Adopt frame diff and timeline semantics before inventing a second player diff runner.
- Adopt the SQLite query pattern for iterative CLI debugging, but only after the clean-room player has traceable events/cells worth indexing.
- Adopt focus-cell root-cause as a future visual player debugging affordance.
```

### `tui-vfx-trace`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace/src/cli.rs
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace/src/fnc_parse_selector.rs
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-trace/src/fnc_parse_stage_mask.rs
```

Features/options:

```text
--recipe PATH
--theme NAME
--frames N
--select all|cell:x,y|rect:x,y,w,h|role:name|layer:id|recipe:id
--stages lifecycle|resolution|composition|pipeline|sampler|mask|shader|filter|shadow|all|none
--from-ms N
--to-ms N
--format ndjson|report
--output PATH
```

Adopt/adapt:

```text
- Adopt selector grammar for future player trace commands.
- Adopt stage masks when player evidence grows beyond row rendering into lifecycle, resolution, composition, and pipeline stages.
- Prefer NDJSON for long trace streams and JSON report envelopes for summary artifacts.
```

### `tui-vfx-horseman`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-horseman
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-horseman/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-horseman/src/cls_cli_args.rs
${RECIPE_REPO:-../tui-vfx-recipes}/tools/tui-vfx-horseman/src/fnc_parse_cli_args.rs
```

Features/options:

```text
--json
--project-root PATH
--corpus DIR
recipe.json positional path
```

Report schemas:

```text
tui_vfx_horseman.summary.v1
tui_vfx_horseman.corpus_summary.v1
```

Adopt/adapt:

```text
- Keep the lightweight corpus summary idea for quick smoke reporting.
- Do not import legacy fallback behavior into the clean-room player.
```

### `recipe-source-capture`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-source-capture
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-source-capture/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-source-capture/src/main.rs
```

Features/options:

```text
--output FILE
--cwd DIR
--allow-nonzero
-- <command...>
```

Output schema:

```text
tui_vfx.command_capture.v1
```

Adopt/adapt:

```text
- Adopt command-capture artifacts for reproducible generated source text and debugging fixtures.
- Keep command execution as an offline authoring tool, not runtime recipe playback behavior.
```

### `recipe-signals-doc`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-signals-doc
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-signals-doc/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-signals-doc/src/main.rs
```

Features/options:

```text
--write
--check
```

Adopt/adapt:

```text
- Adopt the `--check` drift-gate pattern for generated player/report schema docs.
- Use generator checks to keep CLI report examples aligned with current schemas.
```

### `recipe-validator`

Root path:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-validator
```

Key files inspected:

```text
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-validator/Cargo.toml
${RECIPE_REPO:-../tui-vfx-recipes}/tools/recipe-validator/src/main.rs
```

Features/options:

```text
--recipes-dir DIR
```

Adopt/adapt:

```text
- Do not adopt for new work. It prints its own deprecation warning and routes users toward `pipeline-validator --rules`.
```

## Tooling recommendations for K2.4+

| Recommendation | Source tooling | Priority | Reason |
| --- | --- | --- | --- |
| Add player trace selectors | `tui-vfx-trace` | High after styled-cell substrate | Enables focused debug without giant reports. |
| Add SQLite query over frame/trace evidence | `recipe-probe`, `pipeline-validator --probe` | High after trace events exist | Enables iterative CLI debugging and forensic queries. |
| Add frame timeline/diff commands | `recipe-probe` | High | Avoids inventing a second diff/debug workflow. |
| Add canvas simulation flags | `pipeline-validator`, `recipe-probe` | Medium after compositor/styled cells | Needed to validate transparency/compositing honestly. |
| Add generated schema doc drift checks | `recipe-signals-doc` | Medium | Keeps report docs aligned with emitted schemas. |
| Keep simple corpus summary reports | `tui-vfx-horseman` | Medium | Useful for quick corpus health checks. |
| Keep deprecated validator out of new paths | `recipe-validator` | High | Avoids reintroducing legacy validation authority. |

## Captured JSON outputs

These are portable artifact names under the local temporary directory rather
than repo-owned source paths.

```text
${TMPDIR:-/tmp}/tui-vfx-k23-render-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-inventory-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-visual-frame-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-migration-gap-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-primitive-adapter-gap-report.json
```

## Verification results

Final verification after review and AI de-slop:

| Gate | Command | Result |
| --- | --- | --- |
| Format | `cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check` | pass |
| Lint/static | `cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings` | pass |
| Player unit/integration tests | `cargo test -p tui-vfx-player` | pass: 5 unit + 6 integration tests |
| CLI integration tests | `cargo test -p tui-vfx-player-cli` | pass: 13 integration tests |
| Workspace tests | `cargo test --workspace` | pass |
| Diff hygiene | `git diff --check` | pass |
| Recipe corpus cleanliness | `git -C ../tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes` | pass: no output |
| Path portability | `rg -n '"/usr/projects/tui-vfx-recipes' crates/tui-vfx-player/tests crates/tui-vfx-player-cli/tests` | pass: no hard-coded recipe checkout paths in touched tests |

Final report summaries were regenerated with `RECIPE_REPO=../tui-vfx-recipes`:

| Report artifact | Summary |
| --- | --- |
| `${TMPDIR:-/tmp}/tui-vfx-k23-render-report.json` | total=16 rendered=12 unsupported=4 errors=0 |
| `${TMPDIR:-/tmp}/tui-vfx-k23-inventory-report.json` | totalRecipes=16 rendered=12 unsupported=4 errors=0 unsupportedEffectIds=4 |
| `${TMPDIR:-/tmp}/tui-vfx-k23-visual-frame-report.json` | total=16 rendered=12 unsupported=4 errors=0 |
| `${TMPDIR:-/tmp}/tui-vfx-k23-migration-gap-report.json` | legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 |
| `${TMPDIR:-/tmp}/tui-vfx-k23-primitive-adapter-gap-report.json` | totalEffects=14 rendered=10 blockedByStyledCellSubstrate=4 stillUnsupported=0 blockedBySemanticDecision=0 |

## Review and AI de-slop results

Formal third-party review scope included production code, tests, and docs touched by this packet. The reviewer requested changes for three items:

| Finding | Resolution |
| --- | --- |
| Architect memo omitted final verification and recipe-root status. | Resolved in this memo by adding the verification matrix and recipe corpus cleanliness gate. |
| Adapter-gap CLI test did not assert all remaining blocked ids. | Resolved in `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs` by asserting all six focus ids and their expected outcomes/classes. |
| Newly rendered adapters lacked player-level row-evidence assertions. | Resolved in `crates/tui-vfx-player/tests/test_fnc_recipe_player.rs` with dissolve and ripple integration tests over canonical fixtures. |

Formal AI de-slop scope also included production code, tests, and docs. Cleanup performed:

```text
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs
docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md
docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md
```

Notable cleanup:

```text
- Centralized CLI JSON test execution helpers.
- Renamed the internal styled-cell predicate for clearer intent.
- Replaced hard-coded local recipe/temp examples with portable environment-derived forms where not explicitly requested as root-path evidence.
```

TDD process note: initial adapter implementation began before a clean RED step. The remainder of the packet corrected course by adding review-driven tests first, observing a RED assertion for ripple non-empty invariance, then adjusting the test expectation and rerunning to GREEN. Future packets should move unit and integration RED tests immediately after context gathering and before implementation.

## Notes and risks

- The two newly rendered adapters are text-grid only; they do not prove visual parity.
- The four remaining blockers are intentionally not faked as rendered because style/color/role data is still placeholder-only.
- Recipe-side SQLite and trace tooling should be adapted after clean-room player evidence has real trace/styled-cell data; adopting it too early would create empty ceremony.

<!-- <FILE>docs/new_kernel/PHASE_K2_3_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.3 primitive adapter burn-down status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
