<!-- <FILE>docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.10 corpus mapping status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>K2.10 corpus-wide migration mapping and backlog board.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — record final verification, de-slop, and review outcomes.
0.2.0: PATCH — remove stale ready-filter wording after value-source downgrade.
0.1.1: PATCH — replace pending de-slop placeholder with scoped post-verification workflow status.</CLOG> -->

# Phase K2.10 Corpus Mapping Status Memo to Architect

## Rolling context

K2.1 through K2.9 established migration-gap reporting, visual-frame evidence, primitive adapter and field coverage gates, styled-cell output, GUI/player shells, fixture-qc, and the first simple mask migration batch. K2.10 widened migration mapping across the full legacy debug recipe corpus without bulk migration.

## Executive summary

Accepted scope completed: `migration-mapping-batch --recursive` now emits useful corpus-wide planning records instead of mask-only evidence. The report schema remains `v3.1.player.migrationMappingBatch.1`; new record evidence fields are additive. No legacy recipe files were modified. No optional canonical fixtures were added.

## Sub-agent lane table

| Lane | Result | Key finding |
|---|---|---|
| B primitive families | PASS | Non-mask records needed effect evidence extraction; sampled-surface filter variants need value-source decisions, and rich effects need descriptors. |
| C source/content/scene | PASS | `source.card` is too narrow; `source.text`, `source.ansi`, `source.image`, procedural sources, and command-capture artifact policy are backlog candidates. |
| D timing/lifecycle | PASS | Event dwell is partly supported; signals/easings/motion/bindings/loopback require schema or demo-layer decisions. |
| E complex/shadow/subcell | PASS | Known primitive combos are composition decisions; shadows/subcell need descriptors plus future backend/renderer work. |

## Recursive command

```bash
export RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- migration-mapping-batch \
  --legacy-root "$RECIPE_REPO/recipes/debug_recipes" \
  --v31-root "$RECIPE_REPO/recipes/v3.1/debug_recipes" \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive \
  --json
```

## Classification totals

- Total legacy records: 603
- Families: 18
- Status counts: canonicalExists 21; candidateReady 0; descriptorDecisionNeeded 151; schemaDecisionNeeded 72; ownerAuditNeeded 280; sourceDecisionNeeded 67; blockedByFieldCoverage 4; duplicateOrVariant 3; notYetClassified 5.

## Top recommended backlog items

1. K2.11 source/content descriptor pilot for `source.text`, `source.ansi`, `source.image`, procedural sources, and card expansion.
2. K2.11 lifecycle/signal/binding schema decision packet for event dwell, easing, motion routes, sampled-surface filter values, signals, bindable rates, and loopback demo policy.
3. K2.11 descriptor decisions for rich filters, samplers, masks, styles, and shader compositions.
4. Later backend adapter decision for shadows and subcell shapes after descriptor/source/schema backlog is reduced.

## Descriptor-pack expansion candidates

- Filters: CRT, vignette, matrix rain, glyph style, subcell light.
- Samplers: CRT, CRT jitter, fault line, shredder, radial twist.
- Masks: wipe corner/path reveal, cellular/materialize/noise dither.
- Styles: additional style effects and modulo/content/outer/predicate scopes.
- Shaders: highlighter, focus field, concealed light, glisten band, barber pole, rich terminal fire/water.

## Source descriptor candidates

- `source.text`
- `source.ansi`
- `source.image`
- procedural source candidates for braille flag fields and spinner demos
- `source.card` expansion or versioning
- offline `source.commandCaptureArtifact` / oracle artifact policy, with no runtime command execution

## Schema/model decision candidates

- Trigger predicates for remaining event-driven dwell fixtures.
- Signal generator and value-source execution policy.
- Binding execution and parameter override semantics.
- Motion/easing descriptor boundary.
- Loopback demo layer policy.

## GUI/backend boundary summary

The GUI remains a human inspection surface over `tui-vfx-player` evidence. It should eventually expose mapping and fixture-QC status, but K2.10 did not add GUI features. Compositor-backed rendering remains a future backend behind an explicit adapter; v3.1 DTOs are not compositor DTOs.

## Optional fixture additions

None. K2.10 stayed a mapping/backlog packet.

## Verification matrix

Final verification completed after review fixes:

- `cargo fmt --package tui-vfx-player --package tui-vfx-player-cli --package tui-vfx-player-ui --package tui-vfx-contract-cli -- --check` — PASS.
- `cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract-cli --all-targets -- -D warnings` — PASS.
- `cargo test -p tui-vfx-player`, `cargo test -p tui-vfx-player-cli`, `cargo test -p tui-vfx-player-ui`, `cargo test -p tui-vfx-contract-cli`, and `cargo test --workspace` — PASS.
- Canonical v3.1 `validate-recipe`, `fixture-qc`, `primitive-field-coverage`, and `primitive-adapter-gap` gates — PASS.
- Recursive migration mapping — PASS with 603 records, 18 families, `candidateReady` 0, `schemaDecisionNeeded` 72.
- Masks migration mapping — PASS with 41 records and `candidateReady` 0.
- `git diff --check`, touched-file hard-coded path scan, and recipe-root mutation checks — PASS.

## Review and de-slop results

- Initial verification was green before the formal de-slop pass.
- The de-slop pass stayed within the K2.10 changed-file scope and reviewed code, tests, and docs for OFPF size, naming clarity, stale wording, and maintainability.
- Formal code review initially found conservative-classification/doc-metadata issues; follow-up review found only metadata/index hygiene. Those issues were fixed.
- Formal architecture review passed after confirming inventory-gate scope, no descriptor/schema churn, conservative sampled-surface/source/content classifications, and the preserved backend boundary.
- Final code review re-check passed after the K2.10 architect response metadata/index fix; no unresolved production/test behavior issues remain.

## Recipe repo mutation status

Legacy root `recipes/debug_recipes` remained read-only. No v3.1 fixture additions were made in this packet.

## Recommended next packet

K2.11 — source/content descriptor pilot. Keep sampled-surface `filter.dim` variants in the lifecycle/signal/binding schema decision backlog until value-source semantics are approved.

<!-- <FILE>docs/new_kernel/PHASE_K2_10_CORPUS_MAPPING_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.10 corpus mapping status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
