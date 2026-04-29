<!-- <FILE>docs/new_kernel/PHASE_K2_2_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.2 formal review and AI de-slop report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K2.2: record mandatory review and cleanup gates.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture third-party review findings, fixes, and de-slop evidence.</CLOG> -->

# Phase K2.2 Formal Review and AI De-Slop Report

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Packet: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_1.md`

## Scope

Formal review and cleanup covered the K2.2 implementation surface only:

```text
crates/tui-vfx-player/src/lib.rs
crates/tui-vfx-player/src/cls_player_visual_cell.rs
crates/tui-vfx-player/src/cls_player_visual_frame.rs
crates/tui-vfx-player/src/cls_player_visual_frame_report.rs
crates/tui-vfx-player/src/fnc_build_visual_frame.rs
crates/tui-vfx-player/src/fnc_collect_unsupported_effect_ids.rs
crates/tui-vfx-player/src/fnc_collect_visual_cells.rs
crates/tui-vfx-player/src/fnc_extract_recipe_inventory_ids.rs
crates/tui-vfx-player/src/fnc_recommend_migration_queue.rs
crates/tui-vfx-player/src/fnc_render_visual_frame_paths.rs
crates/tui-vfx-player/src/fnc_summarize_visual_frames.rs
crates/tui-vfx-player-cli/src/fnc_print_usage.rs
crates/tui-vfx-player-cli/src/fnc_run.rs
crates/tui-vfx-player-cli/src/fnc_run_render_frame.rs
crates/tui-vfx-player-cli/src/main.rs
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
docs/VOCABULARY.md
docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md
docs/new_kernel/PHASE_K2_2_VISUAL_FRAME_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md
```

## Third-party review

### Architecture review

Result: `WATCH`, no blocking architecture violation.

Findings addressed:

| Finding | Resolution |
| --- | --- |
| Timing identity was lossy because visual frames carried `sampleT` but not `loopT`. | Added `loopT` to `PlayerVisualFrame` and CLI regression assertions. |
| Row-derived style placeholders could be mistaken for true compositor output. | Added explicit `substrate`, `cellSource`, and `styleKnown` fields. |
| Provenance semantics were not tested. | Added CLI assertions for `substrate: "textGrid"`, `cellSource: "rows"`, `styleKnown: false`, transparent colors, empty modifiers, and null role. |

Additional naming correction from owner feedback:

| Issue | Resolution |
| --- | --- |
| `k0Rows` leaked transient phase language into public report vocabulary. | Replaced with globally readable `substrate: "textGrid"`; kept `cellSource: "rows"`. |

### Code review

Result: `REQUEST CHANGES`, then fixed.

| Severity | Finding | Resolution |
| --- | --- | --- |
| Medium | `cargo fmt --package tui-vfx-player-cli -- --check` failed on a new assertion. | Ran rustfmt and re-ran fmt checks successfully. |
| Low | Architect status memo omitted `loopT`, `substrate`, `cellSource`, and `styleKnown`. | Updated memo report-shape field list and notes. |

## AI slop cleanup

Behavior lock:

- Existing `render-recipe`, `inventory-recipes`, and `migration-gap` CLI regressions stayed green.
- New `render-frame` regressions lock single fixture, recursive corpus, unsupported fixture, provenance, and placeholder style semantics.

Cleanup plan and result:

| Pass | Scope | Result |
| --- | --- | --- |
| Dead code deletion | New visual-frame helpers/modules | No unused helper layer found after clippy and tests. |
| Duplicate removal | Unsupported-effect extraction | Shared `collect_unsupported_effect_ids` between inventory and visual-frame paths. |
| Naming/error handling cleanup | Public strings and report values | Removed transient phase labels from emitted substrate value and migration recommendation rationale strings. |
| Test reinforcement | CLI tests | Added assertions for timing/provenance/style placeholders. |
| Documentation consistency | Vocabulary, evidence doc, architect memo | Updated docs to match emitted schema and make text-grid provenance explicit. |

## Quality gates

```text
cargo fmt --package tui-vfx-player -- --check                         PASS
cargo fmt --package tui-vfx-player-cli -- --check                     PASS
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings  PASS
cargo test -p tui-vfx-player                                         PASS, 4 tests
cargo test -p tui-vfx-player-cli                                     PASS, 12 tests
render-recipe recursive smoke                                        PASS, total=16 rendered=10 unsupported=6 errors=0
inventory-recipes recursive smoke                                    PASS, totalRecipes=16 rendered=10 unsupported=6 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=6
migration-gap smoke                                                  PASS, legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7
render-frame baseline smoke                                         PASS, schema=v3.1.player.visualFrameReport.1 rendered=1 substrate=textGrid cellSource=rows styleKnown=false cells=22
render-frame recursive smoke                                        PASS, total=16 rendered=10 unsupported=6 errors=0
```

Full workspace, diff, and recipe-root status checks are recorded in the phase status memo after final verification.

## Remaining risks

- Visual cells are still derived from text-grid rows, not compositor output.
- Style fields are stable placeholders until a real visual substrate supplies style/color/role data.
- `absoluteTimeMs` remains `0` for current normalized-progress samples.

<!-- <FILE>docs/new_kernel/PHASE_K2_2_REVIEW_AND_DESLOP_REPORT.md</FILE> - <DESC>K2.2 formal review and AI de-slop report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
