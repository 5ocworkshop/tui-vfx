<!-- <FILE>docs/new_kernel/PHASE_J2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J2 wrap: report descriptor pack/catalog work and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase J2 architect memo in the established status-memo style.</CLOG> -->

# Phase J2 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`
Phase: J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch

## Executive summary

Phase J2 implements your recommendation from `ARCH-RESP-TO-PHASE_J1.md`: descriptor duplication is now addressed by a shared primitive descriptor pack before broad migration.

Current answer: **yes, canonical v3.1 recipes can now reference an externally loaded primitive descriptor pack, and the validator can resolve that pack while preserving embedded descriptor support for J0 fixtures.**

The migrated corpus has moved from ten structurally valid fixtures to sixteen structurally valid fixtures when validated with the primitive pack:

```text
total: 16
valid: 16
invalid: 0
```

This remains structural validation only:

```text
valid canonical recipe != visual parity
```

## Contract surface added

New `tui-vfx-contract` DTOs:

```text
DescriptorPackId
DescriptorPackRef
DescriptorPack
DescriptorCatalog
```

New checked schema roots:

```text
schemas/v3.1/contract/descriptor-pack-id.schema.json
schemas/v3.1/contract/descriptor-pack-ref.schema.json
schemas/v3.1/contract/descriptor-pack.schema.json
schemas/v3.1/contract/descriptor-catalog.schema.json
```

`RecipeDocument` now carries optional `descriptorPacks[]` and exposes `validate_with_catalog()`.

## Descriptor pack artifact

Created:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

Pack id:

```text
v3.1.primitive
```

The pack includes the J0 primitives plus the J2 second-ring primitives:

```text
source.card
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
mask.dissolve
sampler.sineWave
sampler.ripple
style.colorFade
style.baseStyleOverride
shader.linearGradient
shader.borderSweep
```

## Validator integration

The validator now accepts:

```text
--descriptor-pack <file>
--descriptor-pack-dir <dir>
```

The report still uses schema `v3.1.validator.report.1` and adds `descriptorPacks[]` with loaded pack id/path context.

Strict failure behavior is in place:

```text
unknownDescriptorPack
duplicatePackEffectDescriptor
unknownEffect
```

Collisions are errors; there is no silent override behavior.

## Second-ring migration batch

Added canonical fixtures:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_dissolve.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_ripple.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_color_fade.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_role_scope_border.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep.json
```

The old evidence files under `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` were read only and left untouched.

## Goal-by-goal status

| J2 requirement | Current status |
|---|---|
| Descriptor-pack DTOs exist and are schema-backed | **Done.** |
| Primitive descriptor pack exists under implementation repo | **Done.** |
| Validator validates recipes using a pack | **Done.** |
| Embedded descriptors remain supported | **Done.** Baseline J0 fixture validates without a pack. |
| J0 canonical fixtures still validate | **Done.** |
| J2 second-ring canonical fixtures validate | **Done.** |
| Missing-pack / unknown-descriptor / duplicate-descriptor diagnostics | **Done.** Covered in CLI tests. |
| Old recipes untouched | **Done.** No changes under old evidence root. |
| Vocabulary updated | **Done.** |
| Schema fixtures current | **Done.** |
| No forbidden runtime dependencies | **Done.** Guardrail remains contract-only. |

## Verification evidence

Core validation command:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Result:

```text
schemaVersion: v3.1.validator.report.1
descriptorPacks: v3.1.primitive
total: 16
valid: 16
invalid: 0
```

Full wrap verification also ran fmt, clippy, schema generation, crate tests,
workspace tests, dependency guardrails, diff checks, acceptance review, deslop
review, and post-deslop regression.

Acceptance review verdict:

```text
ACCEPT_WITH_NOTES
```

The notes were non-blocking. No required fixes were identified. The deslop pass
split descriptor-pack CLI tests into their own file plus shared support to keep
the changed test files smaller and easier to review.

## Request for next assignment

Please review Phase J2 as the first shared descriptor-pack/catalog phase plus the second-ring primitive migration batch.

If accepted, I would expect the next decision to be whether to:

```text
1. harden descriptor-pack authoring/catalog semantics further,
2. begin a v3.1 player/probe path for structural-to-visual evidence, or
3. continue migration with another bounded fixture ring now that shared primitive descriptors exist.
```

<!-- <FILE>docs/new_kernel/PHASE_J2_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J2 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
