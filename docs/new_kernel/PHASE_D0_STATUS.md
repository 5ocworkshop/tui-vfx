<!-- <FILE>docs/new_kernel/PHASE_D0_STATUS.md</FILE> - <DESC>Phase D0 schema/reference backfill status for the clean-room kernel</DESC> -->
<!-- <VERS>VERSION: 0.2.1</VERS> -->
<!-- <WCTX>New kernel Phase D0: record final verification, architect approval, and deslop status.</WCTX> -->
<!-- <CLOG>0.2.1: PATCH — fix metadata comment nesting after final memo pass.
0.2.0: MINOR — record final green verification and architect approval after D0 verifier fixes.
0.1.1: PATCH — record strict enum shape and complete generated-description verifier fixes.
0.1.0: INIT — record Phase D0 scope, schema roots, internal helpers, docs, and verification plan.</CLOG> -->

# Phase D0 Status — Schema/Reference Backfill

## Executive summary

Phase D0 implements the architect-requested schema/reference backfill before descriptor/schema work begins. It keeps Phase A/B/C runtime semantics unchanged while making the current clean-room contract roots rustdoc-backed and Schemars-generatable.

## Scope completed

- Added optional Schemars support to shared foundation types needed by `tui-vfx-next` schemas: `Cell`, `Color`, `Modifiers`, geometry DTOs, and `RoleTag`.
- Added Serde + Schemars derives and strict Serde attributes to `tui-vfx-next` contract-visible DTOs.
- Refactored `Surface` storage away from private `OwnedGrid` so its generated schema exposes the public contract shape directly: `width`, `height`, `cells`, `roles`, and `metadata`. Runtime behavior remains the same.
- Added checked generated schemas under `schemas/v3.1/next/`.
- Tightened schema-visible enum shapes to deny unknown fields and use named payload fields so generated schemas include field descriptions.
- Added a schema test that recursively checks object strictness and property descriptions.
- Added `test_schema_generation.rs` to prove schema generation, rustdoc descriptions, and stale-schema detection.
- Added `docs/v3.1-architecture-overview.md`.
- Updated the surface contract, feature checklist, agent briefing, and docs indexes.

## Generated schema roots

- `schemas/v3.1/next/surface.schema.json` — `Surface`
- `schemas/v3.1/next/scope.schema.json` — `ScopeSpec`
- `schemas/v3.1/next/write.schema.json` — `CellWrite`
- `schemas/v3.1/next/sampler.schema.json` — `PipelineSampler`
- `schemas/v3.1/next/pipeline.schema.json` — `SurfacePipeline`
- `schemas/v3.1/next/diagnostic.schema.json` — `SurfaceDiagnostic`

## Intentionally internal / non-schema items

- `SurfaceEngine` remains a runtime operation façade, not a JSON contract shape.
- `CoordinateSampler` remains a runtime trait; schema uses concrete sampler DTOs.
- `ScopeEvalInput` is an evaluation DTO, not a schema root.
- `fnc_scope_coordinate`, diagnostic annotation helpers, and glyph rewrite helpers are implementation helpers.
- The tiny Phase A `EffectDescriptor` DTO remains a proof artifact. The real descriptor schema is deferred to the descriptor phase.

## Guardrails maintained

- No real effect ports.
- No recipe compiler/schema, studio manifest, runtime binding system, phase graph, trigger engine, or legacy migration.
- No dependency from `tui-vfx-next` to compositor/style/content/shadow crates.
- Existing Phase A/B/C behavior tests are expected to remain green.

## Verification evidence

Final checks before closing Phase D0:

- `UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current` — PASS.
- `cargo test -p tui-vfx-next --test test_schema_generation` — PASS, 3 schema tests.
- `cargo test -p tui-vfx-next` — PASS, 24 surface tests + 3 schema tests.
- `cargo clippy -p tui-vfx-next --all-targets -- -D warnings` — PASS.
- `cargo clippy -p tui-vfx-types --all-targets -- -D warnings` — PASS.
- `cargo test --workspace` — PASS.
- `cargo tree -p tui-vfx-next` — PASS; no compositor/style/content/shadow dependency.
- Forbidden dependency grep over `crates/tui-vfx-next` — PASS; no matches.
- OFPF filename/LOC review for `crates/tui-vfx-next` — PASS; source files use `cls_`, `fnc_`, `tr_`, or crate-root exceptions and remain under hard limits.

Architect verification: APPROVED after two verifier rejection/fix cycles.

## Verification command set

Required for future reruns:

```bash
cargo fmt --package tui-vfx-next -- --check
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo test -p tui-vfx-next
cargo test --workspace
cargo tree -p tui-vfx-next
grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next
```

Schema-specific proof:

```bash
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current
cargo test -p tui-vfx-next --test test_schema_generation
```

## Open questions / risks

- Some shared foundation types preserve their existing serde naming. Phase D0 does not globally alter legacy/shared wire shapes beyond adding optional JsonSchema support.
- The schema export path is currently test-driven. A future `xtask schema --check` command can formalize it once the contract crate split or descriptor phase arrives.

<!-- <FILE>docs/new_kernel/PHASE_D0_STATUS.md</FILE> - <DESC>Phase D0 schema/reference backfill status for the clean-room kernel</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.1</VERS> -->
