<!-- <FILE>docs/new_kernel/K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md</FILE> - <DESC>K2.12 low-friction burn-down report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: record implemented low-friction work and explicit deferrals.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize source.text fixture work and offender-ledger implementation.</CLOG> -->

# K2.12 Low-Friction Burn-Down Report

## Completed in this packet

1. **Opt-in offender ledger**
   - Added `schema-readiness --include-offenders`.
   - Emits 386 offender rows without changing `schemaVersion` from `v3.1.player.schemaReadiness.1`.
   - Removes generic `ownerAudit` and `unknown` from offender output by classifying complex/style records into concrete readiness kinds.

2. **Source.text canonical fixture**
   - Added `../tui-vfx-recipes/recipes/v3.1/debug_recipes/sources/source_text_basic.json`.
   - Marked `source.text` inputs `text`, `width`, and `height` as player-handled for field coverage.
   - Verified validate/render/fixture-QC pass for the new fixture.

3. **Briefing-first planning artifacts**
   - Created K2.12 PRD, test spec, schema-lock context note, and subagent briefing.
   - All follow-up review/de-slop agents must receive `.omx/context/k212-subagent-briefing.md` before task instructions.

## Explicitly not done

| Candidate | Why not done in K2.12 implementation | Next action |
|---|---|---|
| `source.ansi` | Needs ANSI source/adapter semantics and style-cell interaction policy. | Source/content descriptor packet. |
| Image/procedural sources | Requires asset/procedural source contract ownership. | Source/procedural source decision. |
| Command capture playback | Would violate runtime command-free player boundary. | Keep oracle-only. |
| `gradient`/`applyTo`/`position` field handling | Needs precise descriptor/adapter semantics; premature handling would hide blockers. | Field-coverage closure packet. |
| Broad descriptor expansion | Risks encoding wrong runtime/source model before decisions land. | Queue named candidates only after source/runtime decisions. |

## Evidence snapshot

```text
schema-readiness --include-offenders: offenders=386, ownerAudit offender rows=0, unknown offender rows=0
complex offender kinds: descriptorPack=38, sceneSemantics=14, valueSourceSemantics=9, sourceDescriptor=8, guiHumanReview=2, backendRenderer=1, oracleOnly=1
source.text fixture validate/render/fixture-qc: pass
```

The zero owner-audit/unknown claim is specific to offender rows, not the raw migration summary counters.

<!-- <FILE>docs/new_kernel/K2_12_LOW_FRICTION_BURN_DOWN_REPORT.md</FILE> - <DESC>K2.12 low-friction burn-down report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
