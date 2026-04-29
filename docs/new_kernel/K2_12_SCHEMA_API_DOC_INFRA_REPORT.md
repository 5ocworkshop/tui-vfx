<!-- <FILE>docs/new_kernel/K2_12_SCHEMA_API_DOC_INFRA_REPORT.md</FILE> - <DESC>K2.12 schema/API documentation infrastructure report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.12 schema lock: record existing documentation and API verification gates impacted by offender-ledger work.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document schema, docs, and report-shape gates for schema-readiness changes.</CLOG> -->

# K2.12 Schema/API Documentation Infrastructure Report

## Existing gates

| Gate | Command or file | Purpose |
|---|---|---|
| Rust formatting | `cargo fmt --package ... -- --check` | Keeps changed Rust surfaces reviewable. |
| Rust static analysis | `cargo clippy ... --all-targets -- -D warnings` | Prevents warning regressions across player/CLI/doc-adjacent crates. |
| Player tests | `cargo nextest run -p tui-vfx-player-cli --no-fail-fast` and `cargo nextest run -p tui-vfx-player --no-fail-fast` | Locks report-shape and adapter behavior. |
| Contract CLI tests | `cargo nextest run -p tui-vfx-contract-cli --no-fail-fast` | Locks validator CLI behavior used by fixture gates. |
| Docs generation/check | `cargo xtask docs generate`, `cargo xtask docs check` and `just docs-generate`, `just docs-check` | Maintains generated docs where applicable. |
| API docs helpers | `cargo xtask docs api`, `api-check`, `api-validate`, `api-scaffold` and just aliases | Maintains API documentation scaffolding and validation. |
| Schema generation tests | `crates/tui-vfx-contract/tests/test_schema_generation.rs` | Ensures contract schema roots regenerate consistently. |
| Player report schema assertions | `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs` | Locks player JSON report fields, including the offender ledger. |

## K2.12 impact

The `schema-readiness --include-offenders` flag is a report-shape change, so the impacted docs are:

- Rustdocs for the new `PlayerSchemaReadinessOffender` fields.
- CLI usage text for `--include-offenders`.
- `docs/VOCABULARY.md` entries for schema-readiness report/offender terminology.
- New-kernel K2.12 memo/report artifacts and index entries.

No generated stable contract schema root is impacted because the work is player-report/reporting surface only and v3.1 remains pre-release.

## Recommended combined docs-infra gate

Use this bundle for future schema-readiness/report-shape work:

```bash
cargo fmt --package tui-vfx-player --package tui-vfx-player-cli --package tui-vfx-player-ui --package tui-vfx-contract-cli -- --check
cargo clippy -p tui-vfx-player -p tui-vfx-player-cli -p tui-vfx-player-ui -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo nextest run -p tui-vfx-player-cli --no-fail-fast
cargo nextest run -p tui-vfx-player --no-fail-fast
cargo nextest run -p tui-vfx-contract-cli --no-fail-fast
cargo xtask docs check
```

`cargo xtask docs check` should remain a documentation gate, not a blocker for player-only report JSON unless it reports a real impacted docs drift.

<!-- <FILE>docs/new_kernel/K2_12_SCHEMA_API_DOC_INFRA_REPORT.md</FILE> - <DESC>K2.12 schema/API documentation infrastructure report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
