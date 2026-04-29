<!-- <FILE>docs/new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md</FILE> - <DESC>K2.5 styled primitive adapter evidence reference</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.5 styled primitive work: record styled-cell adapter support evidence and limits.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document target primitive outcomes, visual-frame provenance, and report summaries.</CLOG> -->

# K2.5 Styled Primitive Adapter Evidence

K2.5 uses the K2.4 player-owned styled-cell substrate to render the remaining styled/color/role primitive ids without claiming legacy visual parity.

## Target ids

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

All four now classify as rendered styled-cell adapters in `primitive-adapter-gap`:

```text
outcome=rendered
adapterClass=styledCell
```

No K2.5 target should report `blockedByStyledCellSubstrate`; that blocker is stale now that K2.4 established the substrate.

## Visual-frame invariant

Styled primitive frames must show real styled-cell evidence:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
rows[] present
unsupportedEffectIds=[]
```

Text-grid-only frames remain precise:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

## Adapter behavior

| Effect id | Evidence behavior | Visual-parity claim |
| --- | --- | --- |
| `style.colorFade` | Resolves `target` color and writes deterministic foreground evidence over scoped cells. | None. |
| `style.baseStyleOverride` | Resolves foreground/background and applies them to scoped cells. The `Border` role scope maps to frame edge cells for the canonical `source.card` fixture. | None. |
| `shader.linearGradient` | Resolves start/end colors, angle, and intensity, then writes deterministic position-derived foreground evidence. | None. |
| `shader.borderSweep` | Resolves sweep color/speed/length and writes deterministic foreground/modifier/role evidence on perimeter cells. | None. |

## Hash regression

Styled evidence contributes to `renderHash` even when `rows[]` are unchanged. This prevents color/style changes from becoming invisible in player regression reports.

## Report summaries

Generated artifacts under `${TMPDIR:-/tmp}`:

| Artifact | Summary |
| --- | --- |
| `tui-vfx-k25-render-report.json` | total=16 rendered=16 unsupported=0 errors=0 |
| `tui-vfx-k25-inventory-report.json` | totalRecipes=16 rendered=16 unsupported=0 errors=0 unsupportedEffectIds=0 |
| `tui-vfx-k25-visual-frame-report.json` | total=16 rendered=16 unsupported=0 errors=0; target styled primitives report `styledCell`/`styledCells`/`styleKnown=true` |
| `tui-vfx-k25-primitive-adapter-gap-report.json` | totalEffects=14 rendered=14 blockedByStyledCellSubstrate=0 stillUnsupported=0 blockedBySemanticDecision=0 |
| `tui-vfx-k25-migration-gap-report.json` | legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7 |

## Remaining limits

- This is contract-native player evidence, not a visual oracle.
- No legacy rendering source was ported.
- No recipe files are modified.
- Frame timeline, frame diff, trace/debug, SQLite/xray, QC, and generated report-schema docs remain future tooling work captured in `K2_PLAYER_TOOLING_VALIDATION_PRD.md`.

<!-- <FILE>docs/new_kernel/K2_5_STYLED_PRIMITIVE_ADAPTER_EVIDENCE.md</FILE> - <DESC>K2.5 styled primitive adapter evidence reference</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
