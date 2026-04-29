<!-- <FILE>docs/new_kernel/PHASE_J1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J1 wrap: report validator hardening and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase J1 architect memo in the established status-memo style.</CLOG> -->

# Phase J1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`
Phase: J1 — Validator Hardening + Fixture Regression Harness

## Executive summary

Phase J1 implements your clarification from `ARCH-RESP-TO-PHASE_J0.md`: it hardens the existing J0 validator instead of rebuilding it.

Current answer: **yes, `tui-vfx-contract-cli validate-recipe` can now validate one file, multiple files, and recursive canonical fixture directories with deterministic machine-readable reports, stable summary counts, useful negative diagnostics, and no legacy runtime dependencies.**

J1 continues to make the distinction explicit:

```text
valid canonical recipe != visual parity
```

The phase does not build a player, compare rendered frames, mutate old recipes, migrate the full corpus, or add schema aliases.

## Current implementation state

Validator crate:

```text
crates/tui-vfx-contract-cli
```

Supported commands:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <file>
cargo run -p tui-vfx-contract-cli -- validate-recipe <file> <file> ...
cargo run -p tui-vfx-contract-cli -- validate-recipe --recursive <dir>
cargo run -p tui-vfx-contract-cli -- validate-recipe --json --recursive <dir>
```

Canonical fixture root validated by J1:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Old evidence root left untouched:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

## Goal-by-goal status against the J1 recommendation

| J1 goal / requirement | Current status |
|---|---|
| Harden existing validator, do not rebuild | **Done.** J1 extends `tui-vfx-contract-cli`. |
| Validate one file | **Done.** CLI accepts a single file. |
| Validate multiple files | **Done.** CLI accepts multiple file arguments and reports `root: "<multiple>"`. |
| Recursive directory validation | **Done.** `--recursive <dir>` walks JSON files deterministically. |
| Explicit JSON mode | **Done.** `--json` is accepted; JSON is the only output mode. |
| Stable top-level report | **Done.** Report schema is `v3.1.validator.report.1` with `root`, `summary`, and `recipes`. |
| Negative diagnostics | **Done.** Tests cover `unknownEffect`, `unknownSceneElementSource`, `unknownSignal`, and `deserializeFailed`. |
| Validate J0 fixture corpus | **Done.** 10/10 canonical migrated recipes validate. |
| Dependency guardrail | **Done.** `cargo tree` check shows no runtime/legacy recipe dependencies. |
| Vocabulary update | **Done.** `docs/VOCABULARY.md` now defines migration/validation/parity terms. |
| Evidence artifact | **Done.** `docs/new_kernel/J1_VALIDATOR_HARNESS_STATUS.md` records behavior, diagnostics, and verification. |

## Report shape

Example successful recursive summary:

```json
{
  "schemaVersion": "v3.1.validator.report.1",
  "root": "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
  "summary": {
    "total": 10,
    "valid": 10,
    "invalid": 0
  }
}
```

Each recipe report carries `path`, `status`, `valid`, `errors`, and `warnings`. Each error carries `code`, `path`, `message`, `hint`, and `details`.

## Key decisions

### The recipes repo remains the canonical fixture corpus for migrated recipes

I initially started toward local fixtures based on a stale/incorrect response file. Your corrected Phase J0 response superseded that, so J1 pivoted back to validating the existing recipes repo canonical corpus:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

No local `/usr/projects/tui-vfx/recipes/v3.1/` fixture corpus is part of J1.

### Negative tests mutate known-good recipes in temp files

This avoids committing intentionally invalid recipe files while still proving stable failure diagnostics for contract-level errors.

### Warning channel is present but empty

J1 reserves `warnings[]` in the report schema, but current contract validation remains pass/fail. Non-fatal diagnostics can be added later without reshaping the report root.

## Verification evidence

Passed targeted checks:

```text
cargo fmt -p tui-vfx-contract-cli -- --check
cargo clippy -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo test -p tui-vfx-contract-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo tree -p tui-vfx-contract-cli dependency guardrail
```

Recursive report summary:

```text
schemaVersion: v3.1.validator.report.1
total: 10
valid: 10
invalid: 0
```

Final wrap also ran schema tests, workspace tests, diff checks, acceptance
review, deslop review, and post-deslop regression.

Acceptance review verdict:

```text
ACCEPT_WITH_NOTES
```

The notes were non-blocking: JSON is currently the default/only output mode,
and lifecycle negative coverage uses the missing-signal case rather than a
separately committed invalid-trigger fixture.

## Request for next assignment

Please review Phase J1 as validator hardening and fixture regression harness work.

If accepted, the next likely phase is your proposed:

```text
Phase J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch
```

The main architectural question for J2 is where standard primitive descriptors should live and how canonical recipes should reference them without copying descriptor definitions into every recipe.

<!-- <FILE>docs/new_kernel/PHASE_J1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
