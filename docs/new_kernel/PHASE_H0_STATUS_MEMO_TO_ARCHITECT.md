<!-- <FILE>docs/new_kernel/PHASE_H0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase H0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase H0 wrap: report source/asset contract and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase H0 architect memo in the established status-memo style.</CLOG> -->

# Phase H0 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Repo: `/usr/projects/tui-vfx`
Phase: H0 — Source / Asset / Procedural Source Contract

## Executive summary

Phase H0 implements the source / asset / procedural source contract recommended in `ARCH-RESP-TO-PHASE_G4.md`.

Current answer: **the contract crate can now represent surface-producing sources separately from effects. `SourceDescriptor` declares source identity, typed inputs, required asset slots, output size behavior, role behavior, and lifecycle metadata. `SourceSpec` instantiates a source with `ValueSource` inputs and structural `AssetRef` values. Asset declarations carry kind, format, and structural locators, and validation rejects interpolation-token locators.**

The phase intentionally stops before canonical recipe documents, source lowering/compiler, source-local pipeline syntax, real asset loading, real procedural rendering, runtime stores, phase/trigger/dwell engines, studio manifests, migration, and real source/effect ports.

## Current implementation state

Stable contract crate:

```text
crates/tui-vfx-contract
```

New contract vocabulary:

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

New checked schema roots:

```text
schemas/v3.1/contract/asset.schema.json
schemas/v3.1/contract/asset-requirement.schema.json
schemas/v3.1/contract/asset-ref.schema.json
schemas/v3.1/contract/source-input.schema.json
schemas/v3.1/contract/source-output.schema.json
schemas/v3.1/contract/source-descriptor.schema.json
schemas/v3.1/contract/source.schema.json
```

## Goal-by-goal status against the H0 recommendation

| H0 goal / constraint | Current status |
|---|---|
| Source identity | **Done.** `SourceId` supports dotted canonical ids such as `source.text`. |
| Source descriptors | **Done.** `SourceDescriptor` declares identity, kind, typed inputs, asset slots, output contract, and lifecycle. |
| Source instances | **Done.** `SourceSpec` references a descriptor and supplies inputs/assets. |
| Typed source inputs | **Done.** `SourceInputSpec` reuses `ValueSpec`, `Value`, `ValueKind`, `ValueSource`, and `RuntimeMutability`. |
| No value-model fork | **Done.** Source validation delegates to existing value-source/type validation. |
| Asset requirements | **Done.** `AssetRequirement` declares kind, format, required flag, and description. |
| Structured asset references | **Done.** `AssetRef { id }` replaces interpolation-token references. |
| Procedural source parameters | **Done.** Dotted `SourceInputId` and `ValueSource` cover grouped procedural parameters such as `wave.speed`. |
| Generated/default roles | **Done.** `SourceRolePolicy` supports `Explicit`, `DefaultRole`, and `Generated`. |
| Source-produced surfaces | **Done.** `SourceOutputSpec` records output size and role behavior for produced semantic surfaces. |
| Source-local pipelines as future seam | **Documented.** H0 treats them as future integration after source-produced surfaces, not current recipe/runtime syntax. |
| Unknown source/input validation | **Done.** Tests cover unknown source ids and unknown source inputs. |
| Input kind mismatch validation | **Done.** Tests cover typed mismatch rejection. |
| Asset validation | **Done.** Tests cover missing/unknown assets and wrong format; kind validation is implemented. |
| No real loading/rendering/compiler/runtime | **Respected.** DTOs and validation only. |

## Recipe-example grounding

Per owner instruction, the following recipe examples were read as **context only**:

```text
scene_authoring_ladder_flag_asset_binding.json
scene_braille_flag_asset_token.json
scene_braille_flag_runtime_wave.json
scene_authoring_ladder_procedural_spinner_binding.json
scene_layer_full_stack.json
scene_layer_io_filter_shader.json
scene_layer_visibility_binding_io.json
content_split_flap_solari_authentic.json
baseline.json
ansi_source_chain.json
scene_image_source_bindable.json
command_capture_chain.json
```

They were used to check needs such as text/card/procedural/image/ANSI/command-capture sources, runtime-bound procedural inputs, required assets, source-local pipelines, and asset-backed sources. They were **not** treated as canonical v3.1 field names.

## Key decisions

### Sources are not effects

H0 gives sources their own descriptor and instance vocabulary. Effects transform or sample an existing surface/node state; sources produce an initial semantic surface.

### Asset refs are structural

Canonical H0 asset references are `AssetRef { id }`, backed by `AssetSpec` declarations. `AssetLocator` rejects `{{ ... }}` interpolation markers so legacy authoring syntax does not leak into the canonical contract.

### The value model remains shared

Source inputs reuse the F1/F2 value vocabulary rather than copying it. This keeps effect inputs, source inputs, parameters, signals, graph values, and future recipe bindings on one typed model.

### Source-local pipelines are deferred but accounted for

The docs now call out source-local pipelines as future integration points after a source has produced a semantic surface. H0 does not add pipeline syntax or compiler behavior.

## Verification evidence

Final verification passed:

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

## Request for next assignment

Please review Phase H0 as the source / asset / procedural source contract lock point and advise the next phase.

Based on your G4 response, the next likely step is:

```text
H1 — Canonical Recipe Document Schema
```

H0 should now provide H1 with source descriptors, source instances, typed source inputs, asset requirements, and structural asset refs without forcing H1 to invent ad hoc source fields.

<!-- <FILE>docs/new_kernel/PHASE_H0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase H0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
