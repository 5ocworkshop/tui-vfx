<!-- <FILE>docs/new_kernel/PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.4 styled-cell substrate status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.2</VERS> -->
<!-- <WCTX>Styled-cell substrate work: report visual-frame substrate proof and limits.</WCTX> -->
<!-- <CLOG>0.1.2: PATCH — record final de-slop wording and justfile safety cleanup.
0.1.1: PATCH — record AI de-slop results and post-cleanup verification.
0.1.0: INIT — add K2.4 styled-cell substrate status memo.</CLOG> -->

# Phase K2.4 Styled-Cell Visual Substrate Status Memo to the v3.1 Architect

Date: 2026-04-29
Repo: `.`
Packet source: `docs/new_kernel/ARCH-RESP-TO-PHASE_K2_3.md`

## Rolling context

Completed today:

```text
- K2.1 migration-gap
- K2.2 visual-frame report
- K2.3 primitive adapter burn-down
```

Current packet:

```text
- K2.4 styled-cell substrate foundation
```

Coming next:

```text
- K2.5 styled/color/role primitive adapter burn-down
- Later: frame diff/timeline/trace/debug surfaces
```

## Executive summary

K2.4 establishes a player-owned styled-cell substrate and keeps production row-derived `render-frame` output honest while preserving existing rows and report schema:

```text
schemaVersion=v3.1.player.visualFrameReport.1
production row-derived frames: substrate=textGrid, cellSource=rows, styleKnown=false
controlled styled-grid proof: substrate=styledCell, cellSource=styledCells, styleKnown=true
```

Rows remain present in every rendered frame for compact human-readable evidence.

The packet does not claim visual parity and does not mark the remaining styled/color primitives rendered.

## Implementation summary

Added player-owned styled-cell substrate types:

```text
crates/tui-vfx-player/src/cls_player_styled_cell.rs
crates/tui-vfx-player/src/cls_player_styled_grid.rs
crates/tui-vfx-player/src/fnc_collect_styled_visual_cells.rs
```

Updated visual-frame construction:

```text
crates/tui-vfx-player/src/fnc_build_visual_frame.rs
```

The visual-frame builder now converts rows into `PlayerStyledGrid`, then serializes sparse non-default `PlayerVisualCell` entries from styled cells. Row-derived grids remain `styleKnown=false`; calling `set_cell_style` records real style evidence and marks the grid `styleKnown=true`.

## Default style semantics

A default styled cell is:

```text
glyph=' '
foreground=defaultForeground
background=transparent
modifiers=[]
role=null
```

Sparse cells are emitted when any observable field differs from the default. Existing text fixtures usually differ by glyph and therefore carry default foreground/background style values, but remain `styleKnown=false` until a production adapter writes real style evidence.

## Styled-cell proof

Controlled non-default proof:

```text
crates/tui-vfx-player/tests/test_fnc_recipe_player.rs
test_fnc_player_styled_visual_frame_carries_real_style_evidence
```

The test constructs a styled grid from a canonical baseline render and proves visual-frame `cells[]` can carry:

```text
foreground=ansi.red
background=ansi.blue
modifiers=[bold]
role=Title
```

CLI substrate regression:

```text
crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs
test_fnc_cli_renders_single_visual_frame_json
```

It verifies `render-frame` still emits rows and keeps row-derived production provenance honest: `substrate=textGrid`, `cellSource=rows`, `styleKnown=false`.

## Current report counts

Expected counts remain unchanged from K2.3:

```text
render-recipe:          total=16 rendered=12 unsupported=4 errors=0
inventory-recipes:      totalRecipes=16 rendered=12 unsupported=4 errors=0 unsupportedEffectIds=4
render-frame:           total=16 rendered=12 unsupported=4 errors=0
primitive-adapter-gap:  totalEffects=14 rendered=10 blockedByStyledCellSubstrate=4 stillUnsupported=0 blockedBySemanticDecision=0
migration-gap:          legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7
```

## Remaining unsupported primitive ids

These remain unsupported and are not falsely promoted:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

## Role evidence

The styled-cell substrate can carry `role`, and the controlled test proves non-null role serialization. Current canonical corpus frames still generally emit `role=null` because source/effect role assignment is deferred.

## Verification results

Current verification matrix after implementation, architecture review fixes, and final AI de-slop:

| Gate | Command | Result |
| --- | --- | --- |
| Path portability | `rg -n '"/usr/projects/tui-vfx-recipes' crates/tui-vfx-player/tests crates/tui-vfx-player-cli/tests` | pass: no hard-coded recipe checkout paths in touched tests |
| Format | `cargo fmt --package tui-vfx-player --package tui-vfx-player-cli -- --check` | pass |
| Lint/static | `cargo clippy -p tui-vfx-player -p tui-vfx-player-cli --all-targets -- -D warnings` | pass |
| Player tests | `cargo test -p tui-vfx-player` | pass: 5 unit + 7 integration tests |
| CLI tests | `cargo test -p tui-vfx-player-cli` | pass: 13 integration tests |
| Workspace tests | `cargo test --workspace` | pass |
| Diff hygiene | `git diff --check` | pass |
| Recipe corpus cleanliness | `git -C ../tui-vfx-recipes status --short -- recipes/debug_recipes recipes/v3.1/debug_recipes` | pass: no output |

Report artifacts regenerated under `${TMPDIR:-/tmp}` with `RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}`:

| Report artifact | Summary |
| --- | --- |
| `tui-vfx-k24-render-report.json` | total=16 rendered=12 unsupported=4 errors=0 |
| `tui-vfx-k24-inventory-report.json` | totalRecipes=16 rendered=12 unsupported=4 errors=0 unsupportedEffectIds=4 |
| `tui-vfx-k24-migration-gap-report.json` | legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 |
| `tui-vfx-k24-visual-frame-report.json` | total=16 rendered=12 unsupported=4 errors=0; first rendered frame reports `substrate=textGrid`, `cellSource=rows`, `styleKnown=false` |
| `tui-vfx-k24-primitive-adapter-gap-report.json` | totalEffects=14 rendered=10 blockedByStyledCellSubstrate=4 stillUnsupported=0 blockedBySemanticDecision=0 |

## Review and de-slop results

Formal review and final AI de-slop scope includes production code, tests, docs, and justfile entries touched by K2.4.

Third-party code review returned **REQUEST CHANGES** with two blocking findings:

| Finding | Resolution |
| --- | --- |
| Status memo had placeholder verification/review sections. | Resolved by replacing placeholders with this verification matrix and review/de-slop summary. |
| `primitive-adapter-gap` blocker reason still referenced `styleKnown=false`. | Resolved in `crates/tui-vfx-player/src/fnc_classify_primitive_adapter_gap.rs`; the reason now says the styled-cell substrate exists but the specific adapter is not implemented yet. |

Second/final AI de-slop result: **PASS**.

| Cleanup focus | Result |
| --- | --- |
| Confusing names | Renamed local/test conversion variables around `PlayerStyledGrid` and styled-cell collection. |
| Stale comments/docs | Updated vocabulary and memo language so styled-cell blockers are adapter-specific, not tied to `styleKnown=false`. |
| Overbroad helpers | Kept helper boundaries intact; no new abstraction layers were added. |
| Compatibility assumptions | Kept `rows[]` compatibility explicit while documenting `styledCell` provenance as the controlled styled-grid substrate proof, not row-derived production provenance. |
| Hard-coded paths | Tests now honor `RECIPE_REPO` before falling back to the sibling checkout path. |
| Justfile command safety | Vocabulary commands now fail loudly instead of masking `awk` errors. |
| Test clarity | Style-known JSON assertion now checks a boolean explicitly, and the controlled proof says non-default style evidence. |

Final code review result: **PASS**. No blockers remained after the styleKnown honesty fixes and final de-slop.

Final architect verification result: **PASS**. Architect confirmed the styled-cell substrate proof, honest production provenance, stable schema, preserved rows, no false shader/style support, and adequate docs/status/vocabulary.

Post-de-slop regression verification is green.

## Notes and risks

```text
- Styled-cell substrate is now present, but styled/color effect adapters are still pending.
- Existing fixture style values mostly use explicit defaults, not visual parity colors.
- Role-aware source/effect propagation is deferred.
- No recipe files under ../tui-vfx-recipes are modified by this packet.
```

<!-- <FILE>docs/new_kernel/PHASE_K2_4_STYLED_CELL_SUBSTRATE_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.4 styled-cell substrate status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.2</VERS> -->
