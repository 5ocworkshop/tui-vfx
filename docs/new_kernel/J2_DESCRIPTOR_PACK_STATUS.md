<!-- <FILE>docs/new_kernel/J2_DESCRIPTOR_PACK_STATUS.md</FILE> - <DESC>Phase J2 descriptor pack and second-ring fixture evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J2: record descriptor pack contract surface, validator integration, and second-ring migration proof.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document J2 descriptor pack, validation behavior, fixtures, and verification evidence.</CLOG> -->

# Phase J2 Descriptor Pack Status

Date: 2026-04-29
Phase: J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`

## Boundary

J2 adds descriptor-pack resolution and a second-ring structural migration batch.
It does not build a visual player, compare rendered frames, mutate old recipes, or claim visual parity.

Canonical validation still means:

```text
deserialize RecipeDocument -> resolve required descriptor packs -> RecipeDocument::validate() succeeds
```

It does not mean:

```text
legacy render == v3.1 render
```

## Delivered contract surface

New schema-backed DTOs in `crates/tui-vfx-contract`:

```text
DescriptorPackId
DescriptorPackRef
DescriptorPack
DescriptorCatalog
```

New schema roots:

```text
schemas/v3.1/contract/descriptor-pack-id.schema.json
schemas/v3.1/contract/descriptor-pack-ref.schema.json
schemas/v3.1/contract/descriptor-pack.schema.json
schemas/v3.1/contract/descriptor-catalog.schema.json
```

`RecipeDocument` now has optional `descriptorPacks[]` and `validate_with_catalog()` while embedded descriptors remain supported.

## Standard primitive descriptor pack

Created:

```text
descriptors/v3.1/packs/primitive.json
```

Pack id:

```text
v3.1.primitive
```

The pack supplies the J0/J2 primitive source/effect descriptor set:

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

## Validator behavior

`validate-recipe` keeps the J1 command shapes and adds descriptor pack loading:

```text
--descriptor-pack <file>
--descriptor-pack-dir <dir>
```

J2 validation command:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

The report still uses:

```text
schemaVersion: v3.1.validator.report.1
```

and now includes loaded descriptor-pack context:

```json
"descriptorPacks": [
  {
    "id": "v3.1.primitive",
    "path": "/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json"
  }
]
```

## Collision policy

Default policy is strict:

```text
collisions are errors
```

Stable diagnostics covered by tests include:

```text
unknownDescriptorPack
duplicatePackEffectDescriptor
unknownEffect
```

Embedded descriptors still work for J0 fixtures; pack-provided descriptors work for J2 fixtures.

## Second-ring canonical fixtures

Added under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/`:

```text
masks/mask_dissolve.json
samplers/sampler_ripple.json
styles/style_color_fade.json
styles/style_role_scope_border.json
shaders/primitives/shader_linear_gradient.json
shaders/compositions/shader_border_sweep.json
```

These files reference `v3.1.primitive` and keep embedded descriptor maps empty.

Old evidence files were read from `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/` and were not modified.

## Validation result

Recursive canonical corpus with primitive pack:

```text
total: 16
valid: 16
invalid: 0
```

This is structural contract validity only.

## Acceptance and deslop result

Acceptance review verdict:

```text
ACCEPT_WITH_NOTES
```

No required fixes were identified. The mandatory deslop pass kept behavior
unchanged and split descriptor-pack CLI integration tests into a focused test
file plus shared test support, reducing the main CLI test file size.

Post-deslop regression passed:

```text
cargo fmt -p tui-vfx-contract -- --check
cargo fmt -p tui-vfx-contract-cli -- --check
cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
cargo clippy -p tui-vfx-contract-cli --all-targets -- -D warnings
cargo test -p tui-vfx-contract
cargo test -p tui-vfx-contract-cli
cargo test --workspace
cargo run -q -p tui-vfx-contract-cli -- validate-recipe --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## Deferred by design

- No visual player/probe.
- No rendered oracle comparison.
- No full corpus migration.
- No legacy aliases added to canonical schemas.
- No port of real effect execution logic.

<!-- <FILE>docs/new_kernel/J2_DESCRIPTOR_PACK_STATUS.md</FILE> - <DESC>Phase J2 descriptor pack and second-ring fixture evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
