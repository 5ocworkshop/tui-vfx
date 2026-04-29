<!-- <FILE>docs/new_kernel/PHASE_J0_STATUS.md</FILE> - <DESC>Concise Phase J0 primitive recipe migration pilot status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J0 wrap: summarize contract validator and primitive migration fixture status.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add concise Phase J0 status after validating canonical recipe fixtures.</CLOG> -->

# Phase J0 Status

Date: 2026-04-29
Phase: J0 — Primitive Recipe Migration Pilot + Contract Validator

## Status

Phase J0 is implementation-complete and validated.

## Delivered

- Added `crates/tui-vfx-contract-cli`, a dedicated contract-only validator for canonical v3.1 `RecipeDocument` JSON.
- Added ten canonical v3.1 debug recipe fixtures under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`.
- Preserved old recipes under `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` unchanged.
- Added CLI integration tests proving the J0 fixture corpus validates and invalid JSON fails nonzero with structured errors.
- Wrote `docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md` with mapping evidence, seed descriptors, validation results, and remaining gaps.

## Verification

Passed:

```text
cargo fmt -p tui-vfx-contract-cli -- --check
cargo check -p tui-vfx-contract-cli
cargo test -p tui-vfx-contract-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe $(find /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes -name '*.json' | sort)
git diff --check
cargo test --workspace
```

## Remaining risks

- J0 is not a runtime or visual parity phase.
- The canonical fixtures are hand-mapped, not produced by a lowering compiler.
- The seed descriptors are intentionally minimal and embedded per recipe rather than promoted to a shared catalog package.

<!-- <FILE>docs/new_kernel/PHASE_J0_STATUS.md</FILE> - <DESC>Concise Phase J0 primitive recipe migration pilot status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
