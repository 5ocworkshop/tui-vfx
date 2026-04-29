<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_J0.md</FILE> - <DESC>Architect response approving Phase J0 and assigning Phase J1 validator hardening</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J1: record architect approval of J0 and validator-hardening scope for J1.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase J0 approval and Phase J1 validator harness assignment.</CLOG> -->

# Architecture Response to Phase J0

## Review result

```text
Phase J0: APPROVED as a primitive migration pilot.
```

J0 proved the intended milestone:

```text
old debug recipe evidence
    -> canonical v3.1 RecipeDocument JSON
    -> fresh contract validator
    -> no old recipe mutation
    -> no contract DTO changes required
```

The architect emphasized that this proves structural and contract validity, not visual parity. A valid canonical recipe is not a visually confirmed recipe; visual confirmation waits for a v3.1 player/probe and human review.

## Clarification for J1

J0 already created a working validator crate:

```text
/usr/projects/tui-vfx/crates/tui-vfx-contract-cli
```

with command shape:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <recipe.json> [more-recipe.json ...]
```

Therefore J1 should harden the existing validator and fixture harness. It should not rebuild the validator from scratch.

Recommended name:

```text
Phase J1 — Validator Hardening + Fixture Regression Harness
```

## J0 decisions accepted

- Storing migrated recipes in the recipes repo is acceptable:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

- Old recipes remain untouched evidence under:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

- Embedded seed descriptors are acceptable for the J0 pilot, but broader migration will need a shared descriptor catalog or descriptor-pack mechanism.

## J0 limitations to record

J0 migrated a pilot subset:

```text
baseline
filter_dim
filter_tint
filter_invert
filter_greyscale
mask_none
mask_wipe
mask_checkers
sampler_sinewave
event_driven_dwell/bool_binding_demo
```

Not yet covered from the earlier primitive target set:

```text
mask_dissolve
sampler_ripple
style_color_fade
style_role_scope_border
shader_linear_gradient
shader_border_sweep
```

This is not a rejection; it defines future migration-batch pressure.

## Phase J1 goal

J1 should answer:

```text
Can we validate canonical v3.1 recipe files and directories with stable structured diagnostics, negative tests, recursive corpus checks, and no legacy runtime dependencies?
```

## Phase J1 requirements

### CLI behavior

The validator should support:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <file>
cargo run -p tui-vfx-contract-cli -- validate-recipe <file> <file> ...
cargo run -p tui-vfx-contract-cli -- validate-recipe --recursive <dir>
cargo run -p tui-vfx-contract-cli -- validate-recipe --json --recursive <dir>
```

The recursive directory target should work on:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

### Stable JSON report shape

The output should be deterministic and machine-readable, with a report schema version, root, summary counts, and per-recipe errors/warnings.

Suggested root shape:

```json
{
  "schemaVersion": "v3.1.validator.report.1",
  "root": "/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes",
  "summary": { "total": 10, "valid": 10, "invalid": 0 },
  "recipes": [
    { "path": ".../baseline.json", "valid": true, "errors": [], "warnings": [] }
  ]
}
```

### Negative fixture tests

J1 should add useful negative tests/fixtures for at least:

```text
unknown effect id
unknown source instance id
invalid lifecycle trigger / missing signal
```

The goal is stable, useful diagnostics, not merely a nonzero exit.

### Dependency guardrail

The CLI must not depend on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
tui-vfx-next
```

Allowed:

```text
tui-vfx-contract
serde
serde_json
clap or similar parser if needed
```

### Recipe corpus path rule

J1 validates only canonical migrated v3.1 fixtures under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

It may read old recipes as evidence from:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

but it must not validate old recipes as canonical v3.1 documents and must not modify them.

## Non-goals

- Do not build a visual player.
- Do not compare old/new rendered frames.
- Do not migrate the full debug recipe corpus.
- Do not mutate old recipes.
- Do not import legacy recipe validator/player/probe code.
- Do not add legacy aliases to canonical v3.1 schemas.

## Recommended next phase after J1

After validator hardening, the architect recommends:

```text
Phase J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch
```

Reason: J0 embedded descriptors directly in recipes. That is acceptable for ten recipes but should not scale into broader corpus migration without a shared descriptor catalog or descriptor-pack strategy.

<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_J0.md</FILE> - <DESC>Architect response approving Phase J0 and assigning Phase J1 validator hardening</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
