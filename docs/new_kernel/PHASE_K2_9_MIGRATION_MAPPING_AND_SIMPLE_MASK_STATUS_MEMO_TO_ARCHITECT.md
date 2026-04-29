<!-- <FILE>docs/new_kernel/PHASE_K2_9_MIGRATION_MAPPING_AND_SIMPLE_MASK_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.9 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.9 completion memo: migration mapping report plus simple mask descriptor expansion.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize all K2.9 lanes, evidence, and residual risks.</CLOG> -->

# Phase K2.9 Migration Mapping and Simple Mask Status Memo to Architect

## Executive status

K2.9 is implemented and under verification/review.

The packet adds a stable read-only migration mapping report command and uses it to advance the clear simple mask descriptor set:

```text
mask.blinds
mask.radial
mask.iris
mask.diamond
```

Legacy recipe roots were inspected as evidence only. The old debug recipe corpus was not mutated.

## Lane summary

| Lane | Status | Details |
|---|---|---|
| A. migration report surface | Complete | Added `migration-mapping-batch` CLI and `v3.1.player.migrationMappingBatch.1` JSON report. |
| B1. blinds/wipe map | Complete | Accepted `mask.blinds`; deferred wipe corner/fade variants and deprecated fixtures. |
| B2. radial/iris/diamond map | Complete | Accepted separate `mask.radial`, `mask.iris`, `mask.diamond`; fixed square companions to `duplicateOrVariant`. |
| B3. adjacent style/shader scan | Complete | Confirmed no simple-mask schema change; deferred complex/style/shader combinations. |
| C. descriptor decision | Complete | Added decision report at `docs/new_kernel/K2_9_SIMPLE_MASK_DESCRIPTOR_DECISION_REPORT.md`. |
| D. descriptors and fixtures | Complete | Added descriptors and four canonical v3.1 mask fixtures. |
| E. player adapters and field coverage | Complete | Added text-grid adapters and handled-field declarations. |
| F. QA/docs/memo | In progress | Evidence docs and vocabulary updated; formal de-slop/review/final verification still run before commit. |

## Code and artifact paths

| Area | Paths |
|---|---|
| Descriptor pack | `descriptors/v3.1/packs/primitive.json` |
| Player report DTOs | `crates/tui-vfx-player/src/cls_player_migration_mapping_batch_report.rs` |
| Player report builder | `crates/tui-vfx-player/src/fnc_build_migration_mapping_batch_report.rs` |
| Record/evidence helpers | `crates/tui-vfx-player/src/fnc_build_migration_mapping_record.rs`, `crates/tui-vfx-player/src/fnc_collect_legacy_mask_payloads.rs`, `crates/tui-vfx-player/src/fnc_collect_migration_mapping_batch_paths.rs`, `crates/tui-vfx-player/src/fnc_summarize_migration_mapping_batch.rs` |
| Mask adapters | `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs`, `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs` |
| Coverage declarations | `crates/tui-vfx-player/src/fnc_collect_handled_primitive_inputs.rs`, `crates/tui-vfx-player/src/fnc_player_inventory_adapter_status.rs` |
| CLI integration | `crates/tui-vfx-player-cli/src/fnc_run_migration_mapping_batch.rs`, `crates/tui-vfx-player-cli/src/fnc_run.rs`, `crates/tui-vfx-player-cli/src/fnc_parse_cli_options.rs`, `crates/tui-vfx-player-cli/src/cls_cli_options.rs`, `crates/tui-vfx-player-cli/src/fnc_print_usage.rs`, `crates/tui-vfx-player-cli/src/main.rs` |
| Tests | `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`, `crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_descriptor_packs.rs` |
| New fixtures | `../tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_blinds.json`, `mask_radial.json`, `mask_iris.json`, `mask_diamond.json` |

## Report status vocabulary used

The new report emits the architect-requested top-level shape and record fields. Implemented status vocabulary includes the full current set in `PlayerMigrationMappingBatchSummary`, even when a count is zero:

```text
canonicalExists
candidateReady
descriptorDecisionNeeded
schemaDecisionNeeded
adapterDecisionNeeded
sourceDecisionNeeded
ownerAuditNeeded
blockedByUnsupportedSource
blockedByUnsupportedEffect
blockedByFieldCoverage
blockedByAmbiguousLegacyIntent
duplicateOrVariant
notYetClassified
```

## Current mask report snapshot

For `--family masks` after the new canonical fixtures exist:

| Count | Value |
|---|---:|
| records | 41 |
| canonicalExists | 8 |
| candidateReady | 0 |
| descriptorDecisionNeeded | 15 |
| ownerAuditNeeded | 15 |
| duplicateOrVariant | 3 |

The remaining `descriptorDecisionNeeded` records are intentionally not forced into K2.9 descriptors.

## Verification evidence to preserve

Targeted tests already passed during implementation:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_reports_migration_mapping_batch -- --nocapture

cargo test -p tui-vfx-contract-cli --test test_fnc_validate_recipe_descriptor_packs
```

Report gates already sampled during implementation:

| Gate | Result |
|---|---|
| `validate-recipe --recursive` | 26 valid, 0 invalid |
| `primitive-field-coverage --recursive` | 0 missing/unsupported/schema gap fields |
| `primitive-adapter-gap --recursive` | 18 rendered, 0 unresolved gaps |
| `fixture-qc --recursive` | overall `pass` |

Final verification and review re-ran these gates before commit.

## Residual risks and next recommendations

1. `mask.checkers` has a reported interactive playback issue where one checker recipe appears to show only a few characters. Treat as a deferred player/UI visual bug, not a K2.9 mapping blocker.
2. `softEdge` is documented as coarse text-grid evidence, not alpha feather parity.
3. The graph application path samples ordered nodes without adding new phase-gated mask semantics. Do not add authored `invert`, `radius`, numeric `feather`, or phase-specific mask inputs until lifecycle/phase semantics are explicitly designed.
4. K2.10 can use `migration-mapping-batch --recursive` as the inventory gate before expanding another family; it should not bulk-migrate the corpus blindly.


## Formal de-slop and review results

AI de-slop completed before formal review and made only behavior-preserving cleanup:

- Consolidated text-grid row/styled-grid sync in `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs`.
- Clarified iris shape resolution naming in `crates/tui-vfx-player/src/fnc_apply_simple_mask_primitives.rs`.
- Removed transient migration-batch wording from the four new fixture metadata blocks.

Formal reviews completed:

| Review | Verdict | Result |
|---|---|---|
| Code review | PASS | No blocking findings. One medium non-blocking issue noted: `mask.radial.origin` was declared handled but not explicitly resolved by `apply_mask_radial`. Fixed by resolving the center-only `origin` input in the adapter. |
| Architecture review | PASS | No architecture blockers. Non-blocking risks recorded: future mixed-effect legacy records should inspect all descriptors, future non-center radial origins need explicit descriptor/adapter/tests, and always-JSON fatal report errors should be designed only if automation requires them. |

Post-review targeted verification for the code-review fix passed:

```text
cargo test -p tui-vfx-player-cli --test test_fnc_render_recipe_cli \
  test_fnc_cli_reports_primitive_field_coverage_for_fixture_corpus_json -- --exact --nocapture

cargo test -p tui-vfx-player
```

## Final verification evidence

Final post-review verification passed:

```text
cargo fmt --package tui-vfx-player --check
cargo fmt --package tui-vfx-player-cli --check
cargo fmt --package tui-vfx-contract-cli --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo test -p tui-vfx-player
cargo test -p tui-vfx-player-cli
cargo test -p tui-vfx-player-ui
cargo test -p tui-vfx-contract-cli
cargo test --workspace
```

Final report gates passed with these summaries:

| Gate | Final result |
|---|---|
| `validate-recipe --recursive` | `total=26`, `valid=26`, `invalid=0` |
| `primitive-field-coverage --recursive` | `usedInputFields=207`, `handledInputFields=207`, gap counts `0` |
| `primitive-adapter-gap --recursive` | `totalEffects=18`, `rendered=18`, unresolved gap counts `0` |
| `fixture-qc --recursive` | `totalRecipes=26`, `validated=26`, `rendered=26`, `unsupported=0`, `overallStatus=pass` |
| `migration-mapping-batch --family masks` | `records=41`, `canonicalExists=8`, `candidateReady=0`, `descriptorDecisionNeeded=15`, `ownerAuditNeeded=15`, `duplicateOrVariant=3` |

Legacy source root status remained clean for `../tui-vfx-recipes/recipes/debug_recipes`.
<!-- <FILE>docs/new_kernel/PHASE_K2_9_MIGRATION_MAPPING_AND_SIMPLE_MASK_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>K2.9 status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
