<!-- <FILE>docs/new_kernel/PHASE_H0_STATUS.md</FILE> - <DESC>Phase H0 source/asset/procedural source contract implementation status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H0 wrap: summarize source descriptors, source instances, asset refs, docs, and verification.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture Phase H0 status for architect handoff.</CLOG> -->

# Phase H0 Status — Source / Asset / Procedural Source Contract

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: H0 — Source / Asset / Procedural Source Contract

## Summary

Phase H0 adds stable contract vocabulary for things that **produce** an initial semantic `Surface`, before canonical recipe document schema work begins.

Current answer: **`tui-vfx-contract` can now describe and validate surface-producing sources, source instances, typed source inputs, asset requirements, structural asset refs, and source output role/size behavior. Source inputs reuse the existing value model, asset refs are typed structural ids rather than interpolation tokens, and validation catches unknown sources/inputs/assets, kind mismatches, missing requirements, and graph-value misuse.**

This remains a contract DTO/validation phase. It does not add canonical recipe schema, source lowering/compiler, source-local pipeline syntax, real asset loading, real procedural rendering, runtime stores, phase/trigger/dwell engines, studio metadata, migration, loopback execution, or real source/effect ports.

## Implemented contract APIs

```text
SourceId
SourceInputId
SourceKind
SourceInputSpec
SourceDescriptor
SourceSpec
SourceOutputSpec
SourceOutputSize
SourceRolePolicy
SourceLifecycle
AssetId
AssetKind
AssetFormat
AssetLocator
AssetSpec
AssetRequirement
AssetRef
```

New validation errors were added to `DescriptorValidationError` for source and asset contract failures.

## Source concepts locked

- A source is a surface producer, not an effect over an existing surface.
- Source descriptors declare typed inputs, required/optional asset slots, output size behavior, output role behavior, and lifecycle hints.
- Source instances bind descriptor-local inputs with `ValueSource` and bind descriptor-local asset slots with `AssetRef`.
- Source input ids can be dotted paths such as `wave.speed` to account for procedural parameter groups without copying legacy recipe shapes.
- Source kinds account for text, card, procedural, image, ANSI, command-capture, asset-backed, scene-layer, and custom needs.
- Source-local pipelines are documented as future integration points after a source-produced surface exists.

## Asset concepts locked

- Assets have stable ids, kind, format, locator, and optional description.
- Source descriptors declare accepted asset kind/format per asset slot.
- Source specs supply structural `AssetRef { id }` values.
- Canonical v3.1 does not use string interpolation for asset references.
- `AssetLocator` rejects legacy interpolation markers such as `{{ flag_art }}`.

## Tests added/updated

```text
crates/tui-vfx-contract/tests/test_source_contract.rs
crates/tui-vfx-contract/tests/test_schema_generation.rs
```

Coverage includes:

- source descriptors declare typed inputs
- source descriptors declare required asset slots
- unknown source ids rejected
- unknown source inputs rejected
- source input kind mismatches rejected
- missing required source inputs rejected
- non-bindable source inputs reject external value sources
- bindable source inputs accept parameter value sources
- graph-value sources reject outside graph context
- asset requirement schema is strict and described
- unknown asset refs rejected
- unknown descriptor-local asset slots rejected
- missing required asset refs rejected
- wrong asset kinds rejected
- wrong asset formats rejected
- procedural sources reference asset slots
- procedural sources accept runtime parameter/signal inputs
- text sources default to `RoleTag::Text`
- card sources declare generated roles
- legacy interpolation locators rejected
- source/asset schema roots are current

## Docs updated

```text
docs/INDEX.md
docs/new_kernel/AGENT_BRIEFING.md
docs/new_kernel/ARCH-RESP-TO-PHASE_G4.md
docs/new_kernel/INDEX.md
docs/v3.1-architecture-overview.md
docs/v3.1-contract-boundary.md
docs/v3.1-feature-contract-checklist.md
docs/v3.1-surface-contract.md
```

## Deliberately not added

```text
canonical recipe document schema
source recipe lowering / compiler skeleton
source-local pipeline syntax
real asset loader or asset resolver
real procedural renderer/source adapter
runtime ParameterStore / SignalStore
F2 BindingSpec execution
live override precedence
direct node/effect-input binding targets
phase graph / trigger / dwell engine
visibility predicate execution
loopback / demo signal execution
studio manifest / controls
legacy migration / aliases
real source/effect ports
```

## Verification status

Final phase verification passed:

```text
cargo fmt --package tui-vfx-contract -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo test -p tui-vfx-contract --test test_source_contract
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
cargo test -p tui-vfx-contract --test test_schema_generation
cargo test -p tui-vfx-contract
cargo tree -p tui-vfx-contract
forbidden legacy crate grep over tui-vfx-contract
git diff --check
cargo test --workspace
```

The forbidden dependency grep produced no matches.

Deslop pass completed on H0-owned changed files after tests locked behavior. No behavioral cleanup edits were needed; one metadata/footer consistency issue in `docs/INDEX.md` was corrected before final `git diff --check`.

## Worktree note

The following pre-existing unrelated files remain outside Phase H0 scope and should not be staged into the H0 commit:

```text
docs/new_kernel/PHASE_D0_STATUS.md
docs/new_kernel/PHASE_D0_STATUS_MEMO_TO_ARCHITECT.md
pro/*
```

<!-- <FILE>docs/new_kernel/PHASE_H0_STATUS.md</FILE> - <DESC>Phase H0 source/asset/procedural source contract implementation status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
