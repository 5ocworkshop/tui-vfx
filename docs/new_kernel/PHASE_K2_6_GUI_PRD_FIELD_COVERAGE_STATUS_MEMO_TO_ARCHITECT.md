<!-- <FILE>docs/new_kernel/PHASE_K2_6_GUI_PRD_FIELD_COVERAGE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Status memo for v3.1 GUI PRD, primitive field coverage, migration mapping loop, and timeline/diff evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Report K2.6 outcomes, verification evidence, review findings, and remaining risks for architect handoff.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize K2.6 GUI PRD, field coverage, migration loop, timeline/diff, verification, and review status.</CLOG> -->

# K2.6 GUI PRD + Field Coverage Status Memo to Architect

## Rolling context

Completed today:

- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
- K2.4 styled-cell substrate foundation
- K2.5 styled primitive adapter burn-down

Current packet:

- K2.6 GUI PRD, primitive field coverage, migration mapping loop, initial timeline/diff

Coming next:

- K2.7 Ratatui GUI player skeleton
- K2.8 migration-loop batch over next recipe family

## Delivered artifacts

| Artifact | Path | Status |
| --- | --- | --- |
| Ratatui GUI Player PRD | `docs/new_kernel/K2_6_RATATUI_GUI_PLAYER_PRD.md` | Added |
| Recipe Migration Mapping Loop PRD | `docs/new_kernel/K2_6_RECIPE_MIGRATION_LOOP_PRD.md` | Added |
| Primitive field coverage CLI/report | `tui-vfx-player-cli primitive-field-coverage` / `v3.1.player.primitiveFieldCoverage.1` | Added |
| Frame timeline CLI/report | `tui-vfx-player-cli render-timeline` / `v3.1.player.frameTimeline.1` | Added |
| Frame diff CLI/report | `tui-vfx-player-cli render-frame-diff` / `v3.1.player.frameDiff.1` | Added |
| Vocabulary update | `docs/VOCABULARY.md` | Updated |


## Lane-by-lane completion status

| Lane | Assigned objective | Completion status | Evidence |
| --- | --- | --- | --- |
| A — Ratatui GUI Player PRD | Define future Ratatui GUI player over `tui-vfx-player`, inspect `../tui-vfx-recipes/examples/demo.rs` as UX inspiration only. | Complete | `docs/new_kernel/K2_6_RATATUI_GUI_PLAYER_PRD.md`; includes sections 1-15, demo borrow/do-not-borrow list, and reviewed tooling inspiration table. |
| B — Primitive Field Coverage Report | Add field-level coverage report for canonical v3.1 recipes and descriptor packs. | Complete | `primitive-field-coverage`; schema `v3.1.player.primitiveFieldCoverage.1`; current summary has `usedButUnhandledInputFields=0`, `missingDescriptorInputFields=0`, `schemaDecisionNeededFields=0`. |
| C — Batch remaining primitive field handling | Ensure used input fields for represented primitives are handled or explicitly classified. | Complete for current corpus | Field-aware adapters added for filters, mask wipe/checkers, sampler sine wave, plus existing styled/mask/sampler adapters; current field coverage has `usedInputFields=119` and `handledInputFields=119`. |
| D — Initial migration mapping loop | Define repeatable recipe-by-recipe migration workflow with statuses, recommendation types, and agent prompt. | Complete | `docs/new_kernel/K2_6_RECIPE_MIGRATION_LOOP_PRD.md`; includes stable statuses/recommendations, selection rules, commands, read-only legacy boundary, prompt template, and tooling inspiration. |
| E — Frame timeline and diff | Add bounded timeline/diff evidence while preserving old `render-frame` output. | Complete | `render-timeline` schema `v3.1.player.frameTimeline.1`; `render-frame-diff` schema `v3.1.player.frameDiff.1`; existing `render-frame` regression remains unchanged. |

## Implementation summary

- `primitive-field-coverage` compares canonical v3.1 recipe authored inputs against descriptor-declared inputs and current player-handled inputs.
- Current canonical fixture corpus reports all authored primitive inputs handled: `usedInputFields=119`, `handledInputFields=119`, `usedButUnhandledInputFields=0`.
- Field-aware adapters were added for the represented filter and sampler/mask fields that were previously too coarse for an honest field-coverage gate:
  - `filter.dim`, `filter.tint`, `filter.invert`, `filter.greyscale` now write styled-cell evidence.
  - `mask.wipe` uses `direction` and `softEdge`.
  - `mask.checkers` uses `cellSize`.
  - `sampler.sineWave` uses `axis`, `amplitude`, `frequency`, `speed`, and `phaseOffset`.
- `style.colorFade` and `shader.linearGradient` now consume `colorSpace` through HCT using the public `mcu-hct` / `mcu-utils` crates already used by the workspace color stack.
- The primitive descriptor pack now advertises `rgb` / `hct` for `colorSpace`; the canonical `style_color_fade` fixture was updated from `hsl` to `hct` so descriptor, fixture, and player behavior agree.
- Timeline and diff reports reuse the existing visual-frame builder and keep `render-frame` schema unchanged.
- Frame diff now emits packet-required `from` / `to` fields and includes styled-cell-only deltas, not just row glyph changes.

## Current report counters

Latest local report summaries:

```text
render-recipe:          total=16 rendered=16 unsupported=0 errors=0
render-frame:           total=16 rendered=16 unsupported=0 errors=0
inventory-recipes:      totalRecipes=16 rendered=16 unsupported=0 errors=0 descriptorEffectIds=14 representedEffectIds=14 unrepresentedEffectIds=0 unsupportedEffectIds=0 sourceIds=1
primitive-adapter-gap:  totalEffects=14 rendered=14 stillUnsupported=0 blockedByStyledCellSubstrate=0 blockedBySemanticDecision=0 missingDescriptor=0
primitive-field-coverage: totalRecipes=16 totalPrimitiveInstances=45 usedInputFields=119 handledInputFields=119 usedButUnhandledInputFields=0 declaredButUnusedInputFields=0 missingDescriptorInputFields=0 schemaDecisionNeededFields=0
render-timeline:        schemaVersion=v3.1.player.frameTimeline.1 frames=5 firstSampleT=0 lastSampleT=1
render-frame-diff:      schemaVersion=v3.1.player.frameDiff.1 hashChanged=true changedCellCount=20 nonEmptyDelta=0 fields=from,to,changedCells
```

## GUI PRD notes

The GUI PRD treats `../tui-vfx-recipes/examples/demo.rs` as a human playback oracle for UX only. It explicitly borrows browser/preview layout, keyboard workflow, help modal, status strip, reload, pause, motion-disabled mode, phase/sample scrubbing, trigger controls, render hash diagnostics, and canvas substrate concepts.

It explicitly rejects legacy recipe loading authority, fallback paths, legacy runtime dependencies, old schema semantics, hard-coded effect inspection, and ad-hoc runtime parameter maps.

## Migration loop notes

The migration-loop PRD defines stable statuses and recommendation values, keeps legacy `debug_recipes` read-only, and makes CLI/player/validator reports the automation authority. It also captures tooling inspiration boundaries for `pipeline-validator`, `recipe-probe`, `tui-vfx-trace`, `tui-vfx-horseman`, `recipe-source-capture`, `recipe-signals-doc`, and the deprecated `recipe-validator`.

## Implementation review section

Leader review findings already addressed before final review:

- Low-effort implementation initially made field coverage green by marking fields handled without all corresponding adapter semantics. The leader added a RED test for filter styled evidence and implemented field-aware filter/mask/sampler behavior.
- Initial field coverage used synthetic `descriptorDefaults` to force declared-unused evidence. That was removed; current report counts only real descriptor-declared inputs.
- Initial primitive-field report shape used generic names. It now uses packet vocabulary: `recipePath`, `kind`, `descriptorId`, `nodeId`, `sourceInstanceId`, `domain`, `descriptorInputs`, `adapterHandledInputs`, `classification`, and `recommendation`.
- Large helper bodies were split into focused OFPF-style files before final review.

## AI de-slop section

Formal AI de-slop completed before third-party code/architecture review. Scope included changed production code, tests, and docs. A small follow-up de-slop split frame-diff cell comparison into `fnc_diff_visual_frame_cells.rs` after review fixes increased helper size.

Completed cleanups:

- Centralized repeated resolved-color report labels as `ResolvedColor::rgba_label()`.
- Centralized repeated effect-input lookup with `resolve_effect_value`.
- Removed local color-label helpers from filter, shader, and style adapters.
- Renamed transient packet-specific test wording to phase-neutral report-command wording.
- Collapsed touched metadata changelogs to latest-change summaries.
- Reduced line pressure in large helper files without changing report schemas.

Post-de-slop verification:

```text
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test --workspace
git diff --check
git -C ../tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes
```

All commands above passed before review-directed HCT alignment. After HCT alignment, the same local test and lint suite passed again; `../tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_color_fade.json` intentionally changed one canonical fixture value from `hsl` to `hct`, and the legacy recipe corpus remained untouched.

## Third-party review outcomes

Initial formal code and architecture reviews found changes required:

- Primitive field coverage defaulted unknown descriptor ids to handled inputs, which could overclaim future adapter coverage.
- `style.colorFade` and `shader.linearGradient` reported `colorSpace` handled before the player actually consumed it.
- `render-frame-diff` emitted `fromFrame` / `toFrame` instead of packet-required `from` / `to`.
- Frame diff compared row glyphs only and missed styled-cell-only visual changes.

Fixes applied:

- Unknown descriptor ids now default to zero handled fields and have a regression test.
- `colorSpace` is handled via HCT interpolation backed by public `mcu-hct` / `mcu-utils`; descriptor and canonical fixture values now use `hct`.
- Frame diff serializes `from` / `to` and includes styled-cell deltas with glyph/style/role labels.

Focused follow-up reviews passed:

- Code review: **PASS** for field coverage honesty, frame diff fields, styled-cell deltas, and HCT alignment; procedural note to coordinate the sibling recipe repo fixture change.
- Architecture review: **PASS** for both blockers; HCT descriptor + canonical fixture update accepted as V3.1 descriptor/fixture alignment, not core schema mutation.

## Remaining risks

- Field-aware adapters are deterministic player evidence for current canonical fixtures; they are not visual parity claims.
- Timeline/diff reports compare player frame rows and sparse styled-cell evidence, not perceptual screenshots or legacy oracle output.
- The Ratatui GUI remains PRD-only in this packet; K2.7 should build the shell on top of player evidence rather than legacy runtime code.
- This packet now has one intentional cross-repo canonical fixture update in `../tui-vfx-recipes`; commit/coordination must include that sibling repo change with the implementation repo descriptor/player changes.

<!-- <FILE>docs/new_kernel/PHASE_K2_6_GUI_PRD_FIELD_COVERAGE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Status memo for v3.1 GUI PRD, primitive field coverage, migration mapping loop, and timeline/diff evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
