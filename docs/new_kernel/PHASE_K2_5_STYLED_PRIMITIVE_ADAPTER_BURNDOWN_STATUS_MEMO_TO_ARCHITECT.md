<!-- <FILE>docs/new_kernel/PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.5 styled primitive adapter burn-down status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>K2.5 styled primitive work: report adapter burn-down, tooling PRD capture, and verification status.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record final local review, de-slop, and source-grounded tooling PRD evidence.
0.1.0: INIT — add K2.5 status memo with report summaries and review/de-slop placeholders.</CLOG> -->

# Phase K2.5 Styled Primitive Adapter Burn-down Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Packet source: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_4.md`

## Rolling context

Completed today:

```text
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
- K2.4 styled-cell substrate foundation
```

Current packet:

```text
- K2.5 styled/color/role primitive adapter burn-down
- K2.5 tooling/validation PRD capture
```

Coming next:

```text
- K2.6 frame timeline / frame diff
- Later trace/debug/SQLite/QC surfaces
```

## Executive summary

K2.5 eliminates the stale styled-cell substrate blockers for the four remaining represented primitive ids:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

The player now emits deterministic styled-cell evidence for those ids while preserving compact text rows. The evidence is honest player evidence, not legacy visual parity.

Preferred corpus result is met:

```text
render-recipe:          total=16 rendered=16 unsupported=0 errors=0
inventory-recipes:      totalRecipes=16 rendered=16 unsupported=0 errors=0 unsupportedEffectIds=0
render-frame:           total=16 rendered=16 unsupported=0 errors=0
primitive-adapter-gap:  totalEffects=14 rendered=14 blockedByStyledCellSubstrate=0 stillUnsupported=0 blockedBySemanticDecision=0
```

## Implementation summary

Production changes:

```text
crates/tui-vfx-player/src/fnc_apply_styled_primitive.rs
crates/tui-vfx-player/src/fnc_apply_graph_effects.rs
crates/tui-vfx-player/src/fnc_resolve_effect_input.rs
crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs
crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs
crates/tui-vfx-player/src/fnc_build_player_frame.rs
crates/tui-vfx-player/src/fnc_build_visual_frame.rs
crates/tui-vfx-player/src/cls_player_frame.rs
crates/tui-vfx-player/src/cls_player_frame_report.rs
crates/tui-vfx-player/src/cls_player_styled_grid.rs
```

The render path now:

```text
render_scene rows
-> PlayerStyledGrid::from_rows(rows)
-> text-grid adapters mutate rows and sync glyphs
-> styled adapters write real styled-cell evidence
-> styled evidence contributes to renderHash
-> render-frame uses carried styled grid when available
```

## Target outcomes

| Effect id | Outcome | Adapter class | Notes |
| --- | --- | --- | --- |
| `shader.borderSweep` | rendered | styledCell | Perimeter-local deterministic style evidence. |
| `shader.linearGradient` | rendered | styledCell | Position-derived deterministic gradient evidence. |
| `style.baseStyleOverride` | rendered | styledCell | Foreground/background override; `Border` role scope maps to frame edges for the canonical card fixture. |
| `style.colorFade` | rendered | styledCell | Target color evidence over scoped cells. |

## Tooling PRD

Created:

```text
docs/new_kernel/K2_PLAYER_TOOLING_VALIDATION_PRD.md
```

The PRD captures capability patterns from legacy recipe tooling while preserving the clean-room authority boundary:

```text
Adopt capability patterns, not source code or legacy validation authority.
```

It was grounded in a source review of `pipeline-validator`, `recipe-probe`, `tui-vfx-probe`, `tui-vfx-debug`, `tui-vfx-trace`, `tui-vfx-horseman`, `recipe-source-capture`, `recipe-signals-doc`, `recipe-validator`, release-gate scripts, and docs under `/usr/projects/tui-vfx-recipes` plus the checked-out path dependencies under `/usr/projects/tui-vfx/crates`. It explicitly marks `recipe-validator` deprecated/non-adopted and keeps future tooling schemas as candidates only.

## Verification results

Current report artifacts under `${TMPDIR:-/tmp}`:

| Artifact | Summary |
| --- | --- |
| `tui-vfx-k25-render-report.json` | total=16 rendered=16 unsupported=0 errors=0 |
| `tui-vfx-k25-inventory-report.json` | totalRecipes=16 rendered=16 unsupported=0 errors=0 unsupportedEffectIds=0 |
| `tui-vfx-k25-visual-frame-report.json` | total=16 rendered=16 unsupported=0 errors=0; target styled primitives report `substrate=styledCell`, `cellSource=styledCells`, `styleKnown=true` |
| `tui-vfx-k25-primitive-adapter-gap-report.json` | totalEffects=14 rendered=14 blockedByStyledCellSubstrate=0 stillUnsupported=0 blockedBySemanticDecision=0 |
| `tui-vfx-k25-migration-gap-report.json` | legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 |

Full verification, review, and final de-slop evidence should be recorded before closing Ralph.

## Review and de-slop results

```text
Local formal review: PASS
AI de-slop pass: PASS
```

Review notes:

```text
- Verified production row-derived render-frame remains textGrid/rows/styleKnown=false.
- Verified target styled primitives render as styledCell/styledCells/styleKnown=true.
- Removed one unused styled-primitive input helper path during de-slop.
- Updated the stale UI unsupported-effect assertion to reflect K2.5 rendered styled primitive behavior.
- Reworked the tooling PRD after source review instead of relying on prompt-only inventory.
```

## Notes and risks

```text
- Styled primitive adapters are deterministic and descriptor-driven but do not claim visual parity.
- Border role support is intentionally minimal and card-fixture-specific: frame edge cells provide the current honest role source.
- No legacy recipe tooling crates are dependencies of tui-vfx-player or tui-vfx-player-cli.
- No recipe files are modified.
```

<!-- <FILE>docs/new_kernel/PHASE_K2_5_STYLED_PRIMITIVE_ADAPTER_BURNDOWN_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.5 styled primitive adapter burn-down status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
