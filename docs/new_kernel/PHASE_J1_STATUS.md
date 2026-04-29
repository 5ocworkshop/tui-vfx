<!-- <FILE>docs/new_kernel/PHASE_J1_STATUS.md</FILE> - <DESC>Concise Phase J1 validator hardening status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J1 wrap: summarize validator hardening and fixture regression harness status.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add concise Phase J1 status after validator hardening.</CLOG> -->

# Phase J1 Status

Date: 2026-04-29
Phase: J1 — Validator Hardening + Fixture Regression Harness

## Status

Implementation is complete and acceptance-reviewed.

Acceptance verdict:

```text
ACCEPT_WITH_NOTES
```

The notes are non-blocking: JSON is currently the default/only output mode even
when `--json` is omitted, and the lifecycle negative coverage uses a missing
signal case rather than a separately checked-in invalid-trigger fixture.

## Delivered

- Hardened `tui-vfx-contract-cli validate-recipe` with `--recursive` and explicit `--json` support.
- Replaced per-file array-only output with stable top-level report schema `v3.1.validator.report.1`.
- Added summary counts and per-recipe warnings channel.
- Added stable error codes sourced from contract validation error kinds.
- Added negative diagnostics tests for unknown effect, unknown scene source, missing lifecycle signal, and invalid JSON.
- Validated the existing J0 canonical fixture corpus under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.
- Updated `docs/VOCABULARY.md` with migration/validation/parity terms.
- Added `docs/new_kernel/J1_VALIDATOR_HARNESS_STATUS.md`.

## Verification

Passed targeted checks:

```text
cargo fmt -p tui-vfx-contract-cli -- --check
cargo clippy -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo test -p tui-vfx-contract-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo tree -p tui-vfx-contract-cli dependency guardrail
```

Acceptance review also confirmed single-file, multi-file, recursive, and JSON
recursive CLI behavior; stable diagnostic shape; no forbidden dependencies; no
old recipe mutation; no schema alias regression; and no visual-parity claim.

## Remaining risks

- J1 is still structural/contract validation only.
- Visual parity awaits a future v3.1 player/probe.
- Descriptor duplication remains a known J0/J1 pilot limitation; J2 should decide descriptor catalog/pack strategy.

<!-- <FILE>docs/new_kernel/PHASE_J1_STATUS.md</FILE> - <DESC>Concise Phase J1 validator hardening status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
