<!-- <FILE>docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase D0: summarize schema/reference backfill for architect review and next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase D0 architect memo in the same style as Phase A/B/C status memos.</CLOG> -->

# Phase D0 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Commit: `c330d1d Establish a schema-ready clean-room contract kernel`

## Executive summary

Phase D0 has reached the proof point recommended in `ARCH-RESP-TO-PHASE_C.md`:

> Make the existing Phase A/B/C clean-room contract types schema-reference ready before descriptor/schema work begins.

Current answer: **yes, for the bounded Phase D0 schema/reference backfill.**

The clean-room crate, `tui-vfx-next`, now has rustdoc-backed Serde/Schemars schema generation for the current public contract roots. Checked generated schemas live under `schemas/v3.1/next/`, and tests prove that schema generation is deterministic, stale schema fixtures are caught, object shapes are strict, and schema properties carry descriptions.

Phase D0 also added the durable architecture framing requested in the architect response: Rust contract types are the source of truth; Serde owns wire shape; Schemars generates JSON Schema; rustdoc becomes the human-facing schema/reference text. The legacy engine remains an oracle and inventory, not the source of truth for v3.1 semantics.

The implementation deliberately remains a backfill. It does not add descriptor expansion, recipe schema/compiler, studio manifest, runtime bindings, phase graph, trigger engine, legacy migration, or real effect ports.

## Current implementation state

### Clean-room contract crate now schema-ready

Crate:

```text
crates/tui-vfx-next
```

Phase D0-relevant additions and updates:

```text
crates/tui-vfx-next/tests/test_schema_generation.rs
schemas/v3.1/next/surface.schema.json
schemas/v3.1/next/scope.schema.json
schemas/v3.1/next/write.schema.json
schemas/v3.1/next/sampler.schema.json
schemas/v3.1/next/pipeline.schema.json
schemas/v3.1/next/diagnostic.schema.json
```

Public contract-visible DTOs now derive or intentionally implement the schema/reference path:

```text
serde::Serialize
serde::Deserialize
schemars::JsonSchema
```

Strict public JSON shapes were added where Phase D0 owns the wire contract:

```text
#[serde(rename_all = "camelCase", deny_unknown_fields)]
#[serde(tag = "kind", rename_all = "camelCase", deny_unknown_fields)]
```

### Shared foundation type support

`crates/tui-vfx-types` gained optional Schemars support for the foundation types needed by the clean-room schemas:

```text
Cell
Color
Modifiers
Rect / Point / Size / Anchor
RoleTag / InternedRoleName
```

The feature is optional and tied to the existing serde feature path. This keeps the support narrow and avoids replacing the existing `ConfigSchema` system globally.

### Surface schema shape clarified

`Surface` no longer exposes an internal `OwnedGrid` storage shape to schema generation. It now stores the public contract shape directly:

```text
width
height
cells
roles
metadata
```

This preserves the Phase A/B/C behavior while making generated schema output match the public surface contract.

### Architecture and process docs updated

New and updated docs:

```text
docs/v3.1-architecture-overview.md
docs/v3.1-surface-contract.md
docs/v3.1-feature-contract-checklist.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/INDEX.md
docs/new_kernel/PHASE_D0_STATUS.md
docs/INDEX.md
```

The new architecture overview records the contract-first philosophy, the progressive Phase A/B/C/D0 stack, the schema/reference path, and the current roadmap.

## Goal-by-goal status against the Phase D0 recommendation

| Phase D0 goal / constraint | Current status |
|---|---|
| Add Schemars to `tui-vfx-next` | **Done.** `tui-vfx-next` depends on `schemars` and `serde`; shared foundation types expose optional Schemars support. |
| Public contract-visible types derive/implement Serde + JsonSchema | **Done.** Current clean-room schema roots and dependencies are schema-generatable. |
| Use strict Serde shape | **Done.** Owned clean-room structs deny unknown fields; schema-visible enums use explicit tagging and deny unknown fields. |
| Add rustdoc comments for schema-visible types/fields/variants | **Done.** Schema tests now recursively require descriptions for non-`kind` properties, including `$ref` payload properties. |
| Add schema generation proof | **Done.** `test_schema_generation.rs` generates schemas, checks descriptions, checks strict object shapes, and detects stale fixtures. |
| Add checked generated schema output | **Done.** Six schema files are checked in under `schemas/v3.1/next/`. |
| Add high-level architecture overview | **Done.** `docs/v3.1-architecture-overview.md` captures philosophy, block diagram, reference path, and roadmap. |
| Update surface contract / checklist / briefing | **Done.** D0 schema/reference readiness is now a standing rule for future phases. |
| Do not replace existing `ConfigSchema` globally | **Respected.** D0 only adds optional Schemars support needed by the clean-room schema roots. |
| Do not add descriptors/recipes/studio/runtime/phase graph | **Respected.** No descriptor model expansion, recipe schema/compiler, studio manifest, runtime binding system, phase graph, or trigger engine was added. |
| Do not port real effects | **Respected.** Phase C toy stages remain proof-only; no CRT/typewriter/matrix/shadow/etc. porting was attempted. |
| Preserve clean-room dependency boundary | **Respected.** `tui-vfx-next` has no dependency on compositor/style/content/shadow crates. |
| Keep OFPF foundation correct | **Done.** `tui-vfx-next` source files use OFPF prefixes and remain under hard file-size limits. |
| Commit at phase completion | **Done.** Phase D0 was committed as `c330d1d`. |

## Generated schema roots

The checked schema roots are:

```text
schemas/v3.1/next/surface.schema.json      # Surface
schemas/v3.1/next/scope.schema.json        # ScopeSpec
schemas/v3.1/next/write.schema.json        # CellWrite
schemas/v3.1/next/sampler.schema.json      # PipelineSampler
schemas/v3.1/next/pipeline.schema.json     # SurfacePipeline
schemas/v3.1/next/diagnostic.schema.json   # SurfaceDiagnostic
```

The schema test proves:

- schemas serialize deterministically;
- checked fixtures are current;
- key rustdoc descriptions are present;
- all object schemas deny additional properties;
- all non-`kind` properties, including `$ref` payload fields, carry descriptions.

Two verifier rejection cycles improved this proof:

1. strict enum shapes and missing field descriptions were fixed;
2. `RoleTag::Custom` payload descriptions and the `$ref` description test gap were fixed.

The final architect verifier approved the result.

## Required tests now present

New Phase D0 integration tests:

```text
generated_schema_contains_rustdoc_descriptions
generated_schema_objects_are_strict_and_described
checked_in_schemas_are_current
```

Existing Phase A/B/C tests remain in `crates/tui-vfx-next/tests/test_surface_contract.rs` and still pass. The clean-room package now has:

```text
24 surface/sampling/pipeline tests
3 schema/reference tests
```

## Dependency and architecture status

Current clean-room dependency direction remains narrow:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

`cargo tree -p tui-vfx-next` shows dependencies on:

```text
tui-vfx-types
serde
schemars
serde_json    # dev-dependency
```

It does not depend on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

The broader workspace still retains its existing systems. Phase D0 did not attempt a global schema migration.

## OFPF / file-structure status

The clean-room crate follows the current OFPF convention:

```text
crates/tui-vfx-next/src/lib.rs       # crate-root exception
crates/tui-vfx-next/src/cls_*.rs     # cohesive type/enum/class files
crates/tui-vfx-next/src/fnc_*.rs     # helper/function files
crates/tui-vfx-next/src/tr_*.rs      # trait files
crates/tui-vfx-next/tests/test_*.rs  # tests
```

Largest source files remained below hard limits during final review:

```text
cls_pipeline_stage.rs                  under cls_ hard limit
cls_surface.rs                         under cls_ hard limit
cls_surface_engine.rs                  under cls_ hard limit
fnc_apply_from_source_with_sampler.rs  under fnc_ hard limit
```

`recyclebin/` remains ignored. No file deletion requiring recyclebin action was part of Phase D0.

## Verification evidence

Final verification passed:

```text
cargo check -p tui-vfx-next --all-targets
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current
cargo test -p tui-vfx-next --test test_schema_generation
cargo test -p tui-vfx-next
cargo clippy -p tui-vfx-next --all-targets -- -D warnings
cargo clippy -p tui-vfx-types --all-targets -- -D warnings
cargo test --workspace
cargo tree -p tui-vfx-next
grep -R -nE 'tui_vfx_(compositor|style|content|shadow)|tui-vfx-(compositor|style|content|shadow)' crates/tui-vfx-next
```

Observed results:

```text
cargo test -p tui-vfx-next: PASS
  24 surface tests passed
  3 schema tests passed

cargo test --workspace: PASS

cargo clippy -p tui-vfx-next --all-targets -- -D warnings: PASS
cargo clippy -p tui-vfx-types --all-targets -- -D warnings: PASS

forbidden dependency grep: PASS, no matches
architect verifier: APPROVED
```

Formatting note:

```text
cargo fmt --package tui-vfx-next -- --check: PASS
rustfmt --edition 2024 --check on changed source/test files: PASS
cargo fmt --all --check: not used as a completion gate because unrelated pre-existing workspace formatting drift exists outside the Phase D0 scope.
```

## Notable implementation choices

### Rust-owned schema source of truth

Phase D0 follows the architect decision:

```text
Rust contract types + Serde shape + rustdoc + JsonSchema derive
    -> generated JSON Schema
    -> public contract artifact
```

This makes future contract drift easier to catch mechanically.

### Optional Schemars in foundation types

The shared `tui-vfx-types` crate gained optional Schemars support only where needed by `tui-vfx-next` schemas. This avoids a broad migration of legacy/internal docs systems while still letting the clean-room contract schemas reference canonical foundation types.

### Strict named enum payloads

Some enum variants were shifted from tuple payloads to named payload fields in the schema-visible contract. This was done to satisfy the D0 rule that payload fields be strict and described in generated schema output.

### Surface public shape over storage shape

`Surface` now stores the contract shape directly rather than relying on private `OwnedGrid` storage. This keeps schema output understandable and avoids leaking an internal storage detail as public contract.

## Scope control

Still intentionally out of scope:

```text
effect descriptor expansion
recipe v3.1 schema/compiler
studio manifest
runtime bindings
phase graph / trigger engine
real effect ports
legacy migration aliases
legacy compositor replacement
```

The tiny Phase A `EffectDescriptor` remains a proof artifact and is explicitly not the descriptor model.

## Deslop cleanup

The post-approval cleanup pass was scoped to Ralph-owned Phase D0/new-kernel files only.

Cleanup outcomes:

- strengthened schema tests so `$ref` payload properties must also have descriptions;
- recorded strict schema and verifier-fix status in `PHASE_D0_STATUS.md`;
- avoided broad refactors or non-phase files;
- left unrelated untracked `pro/*` and `docs/new_kernel/TEMPLATE_INHERITANCE.md` files untouched and uncommitted.

## Open questions for next assignment

Recommended next architect decision:

```text
What is the next semantic phase after D0?
```

Likely candidates based on the roadmap in `ARCH-RESP-TO-PHASE_C.md`:

```text
Phase D  — Contract/engine boundary and generalized ScopeSpec/write model
Phase E  — Effect descriptors
Phase F  — Value/parameter/source model
Phase G  — Node graph
```

Questions to resolve before the next implementation phase:

1. Should Phase D generalize `ScopeSpec` and write policies before descriptor expansion, or should the descriptor model come first?
2. Should schema export remain test-driven for one more phase, or should we add `cargo xtask schema --check` now?
3. Which current proof-only types should become true public contract types, and which should be retired before descriptor work?
4. Should the later contract crate split happen before recipes, or after descriptors/values prove the shape?

## Bottom line

Phase D0 is complete and committed.

The clean-room kernel now has a schema/reference foundation that is strong enough to support the next semantic phase. The important lock is no longer just “the tests pass”; it is now:

```text
No undocumented public contract type.
No schema-visible field without rustdoc.
No public wire shape without Serde + JsonSchema + checked generated schema output.
```

I recommend the architect assign the next bounded phase with that rule treated as mandatory for all new public contract-visible work.

<!-- <FILE>docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
