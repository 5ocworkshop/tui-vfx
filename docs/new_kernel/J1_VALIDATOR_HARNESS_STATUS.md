<!-- <FILE>docs/new_kernel/J1_VALIDATOR_HARNESS_STATUS.md</FILE> - <DESC>Phase J1 validator hardening and fixture harness evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J1: record recursive validator behavior, report shape, negative diagnostics, and dependency guardrails.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document validator hardening evidence and fixture regression harness results.</CLOG> -->

# Phase J1 Validator Harness Status

Date: 2026-04-29
Phase: J1 — Validator Hardening + Fixture Regression Harness
Implementation repo: `/usr/projects/tui-vfx`
Canonical fixture repo: `/usr/projects/tui-vfx-recipes`

## Boundary

J1 hardens the J0 validator. It does not rebuild it, migrate the full corpus, add a player, or claim visual parity.

Canonical migrated fixtures validated by J1 remain under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Old source recipes remain evidence-only under:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

No old source recipes were modified.

## CLI behavior delivered

The validator now supports:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <file>
cargo run -p tui-vfx-contract-cli -- validate-recipe <file> <file> ...
cargo run -p tui-vfx-contract-cli -- validate-recipe --recursive <dir>
cargo run -p tui-vfx-contract-cli -- validate-recipe --json --recursive <dir>
```

`--json` is accepted explicitly; JSON is the only J1 output format.

## Stable report shape

The validator emits top-level run reports:

```json
{
  "schemaVersion": "v3.1.validator.report.1",
  "root": "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
  "summary": { "total": 10, "valid": 10, "invalid": 0 },
  "recipes": []
}
```

Each recipe report carries:

```text
path
status
valid
errors[]
warnings[]
```

Each error carries:

```text
code
path
message
hint
details
```

Contract validation error codes are taken from the serialized `DescriptorValidationError.kind` value, so failures such as `unknownEffect`, `unknownSceneElementSource`, and `unknownSignal` are machine-readable.

## Positive harness evidence

Recursive validation command:

```text
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Result summary:

```json
{
  "total": 10,
  "valid": 10,
  "invalid": 0
}
```

## Negative diagnostics covered by tests

`crates/tui-vfx-contract-cli/tests/test_fnc_validate_recipe_cli.rs` mutates known-good canonical recipes in temporary files and verifies stable codes for:

| Scenario | Expected code |
|---|---|
| Node references undeclared effect descriptor | `unknownEffect` |
| Scene element references undeclared source instance | `unknownSceneElementSource` |
| Lifecycle trigger references missing signal | `unknownSignal` |
| Invalid JSON syntax | `deserializeFailed` |

## Dependency guardrail

Checked with:

```text
cargo tree -p tui-vfx-contract-cli
```

Forbidden dependencies absent:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
tui-vfx-next
tui-vfx-recipes
```

The CLI remains contract-owned and does not import old validator/player/probe code.

## Verification evidence

Passed so far:

```text
cargo fmt -p tui-vfx-contract-cli -- --check
cargo clippy -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo test -p tui-vfx-contract-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
cargo tree -p tui-vfx-contract-cli dependency guardrail
```

Additional wrap evidence:

```text
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test --workspace
git diff --check
git -C /usr/projects/tui-vfx-recipes diff --check
```

Acceptance review verdict:

```text
ACCEPT_WITH_NOTES
```

The acceptance notes were non-blocking: JSON is the current default/only output
mode, and lifecycle negative coverage is by missing-signal diagnostic rather
than a separately committed invalid-trigger fixture.

## Deferred by design

- No visual player.
- No oracle render comparison.
- No broad migration beyond the J0 canonical fixture corpus.
- No shared descriptor catalog; architect recommends that as J2.
- No legacy aliases added to canonical schemas.

<!-- <FILE>docs/new_kernel/J1_VALIDATOR_HARNESS_STATUS.md</FILE> - <DESC>Phase J1 validator hardening and fixture harness evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
