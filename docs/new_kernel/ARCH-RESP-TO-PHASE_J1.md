# Architecture Response to Phase J1

## Review result

```text
Phase J1: APPROVED.
```

J1 did exactly what it needed to do: it turned the J0 pilot validator into a durable contract gate without pretending to be a visual player.

The most important lock remains:

```text
valid canonical recipe != visual parity
```

That distinction is correct. At this point, the validator proves that migrated recipes are strict v3.1 contract documents. It does **not** prove that the player output matches legacy output, and it should not claim that.

## What J1 successfully locks

J1 now gives us a usable validation surface:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <file>
cargo run -p tui-vfx-contract-cli -- validate-recipe <file> <file> ...
cargo run -p tui-vfx-contract-cli -- validate-recipe --recursive <dir>
cargo run -p tui-vfx-contract-cli -- validate-recipe --json --recursive <dir>
```

The canonical migrated recipe root is now explicitly:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

The old evidence root remains untouched:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

The JSON report shape is also a useful lock:

```text
schemaVersion: v3.1.validator.report.1
root
summary
recipes[]
errors[]
warnings[]
```

The warning channel being present but currently empty is fine. It gives us room to add non-fatal authoring or migration hints later without changing the report root.

## Notes, not blockers

The acceptance note that JSON is currently the only output mode is fine. This is a machine-facing validator first. Human formatting can come later.

The lifecycle negative coverage using a missing-signal case is also acceptable for J1. It proves the validator exposes contract-level lifecycle errors. A later lifecycle/runtime phase can add more detailed trigger-invalidity fixtures once runtime behavior is closer.

---

# Recommended next phase

```text
Phase J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch
```

## Why J2 should come next

J0/J1 embedded enough descriptors in each migrated recipe to prove the contract. That was correct for a pilot.

But this will not scale. If every canonical recipe carries its own copies of:

```text
filter.dim
filter.tint
mask.wipe
sampler.sineWave
shader.linearGradient
source.card
```

then the migrated corpus will become noisy, hard to audit, and vulnerable to descriptor drift.

J2 should answer:

```text
Where do standard v3.1 primitive descriptors live, and how do canonical recipes reference them without copying descriptor definitions into every file?
```

This is now the right next dependency before broad migration.

---

# Phase J2 target model

## Recommended shape

Add a descriptor-pack/catalog concept to the contract.

Suggested vocabulary:

```text
DescriptorPackId
DescriptorPack
DescriptorPackRef
DescriptorCatalog
```

A descriptor pack should be able to contain at least:

```text
EffectDescriptor values
SourceDescriptor values
```

Potentially later:

```text
standard signal descriptors
standard asset format descriptors
studio metadata
```

but J2 should stay focused on source/effect descriptors only.

## Recommended descriptor-pack artifact location

Keep standard descriptor packs in the implementation repo, not in the recipes repo:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/
```

Suggested first pack:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

The recipes repo should hold recipe fixtures:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

The implementation repo should hold contract-owned standard descriptor data:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/
```

That split keeps recipes as corpus artifacts and descriptors as contract/runtime catalog artifacts.

## Validator integration

The validator should accept a descriptor pack or descriptor pack directory:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Optionally:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack-dir /usr/projects/tui-vfx/descriptors/v3.1/packs \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

The report should include descriptor-pack context, for example:

```json
{
  "schemaVersion": "v3.1.validator.report.1",
  "descriptorPacks": [
    {
      "id": "v3.1.primitive",
      "path": "/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json"
    }
  ],
  "summary": {
    "total": 16,
    "valid": 16,
    "invalid": 0
  }
}
```

## Contract validation behavior

J2 should support both:

```text
embedded descriptors
external descriptor packs
```

But migrated primitive fixtures should begin moving toward pack references.

The validator should reject:

```text
unknown descriptor pack id
unknown effect descriptor id
unknown source descriptor id
duplicate descriptor id across loaded packs
descriptor id collision between embedded and pack-provided descriptors unless explicitly allowed
```

Default policy should be strict:

```text
collisions are errors
```

Do not introduce silent override semantics.

---

# J2 migration batch

J2 should keep the old recipes untouched and add/modify only canonical v3.1 migrated fixtures under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

The old evidence files to read are:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_dissolve.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/samplers/sampler_ripple.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/styles/style_color_fade.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/styles/style_role_scope_border.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_linear_gradient.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/compositions/shader_border_sweep.json
```

The new canonical files should land at:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_dissolve.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_ripple.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_color_fade.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_role_scope_border.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep.json
```

Existing J0 canonical fixtures may be updated to use the shared descriptor pack, as long as they remain valid and the old recipes are not touched.

---

# Phase J2 requirements

## 1. Add descriptor-pack contract surface

Add schema-backed DTOs in:

```text
/usr/projects/tui-vfx/crates/tui-vfx-contract
```

Expected schema root:

```text
/usr/projects/tui-vfx/schemas/v3.1/contract/descriptor-pack.schema.json
```

Maybe also:

```text
/usr/projects/tui-vfx/schemas/v3.1/contract/descriptor-catalog.schema.json
```

only if the implementation needs a separate catalog object.

## 2. Add standard primitive descriptor pack

Create:

```text
/usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json
```

It should contain the primitive descriptors needed by the J0/J2 fixture set.

At minimum:

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
style.baseStyleOverride or equivalent style primitive
shader.linearGradient
shader.borderSweep
```

The exact IDs may vary, but they must be canonical and consistent with `docs/VOCABULARY.md`.

## 3. Harden validator to resolve descriptor packs

Update:

```text
/usr/projects/tui-vfx/crates/tui-vfx-contract-cli
```

so validation can use external packs.

The validator should still support the J1 command shapes, and add descriptor-pack support.

## 4. Migrate the second-ring primitives

Create the six new canonical fixtures under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Do not modify:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

## 5. Keep visual-parity language honest

J2 should say:

```text
The recipes validate structurally against v3.1.
```

It should not say:

```text
The recipes render correctly.
```

or:

```text
The migration is visually complete.
```

until the v3.1 player/probe exists.

## 6. Update vocabulary

Update:

```text
/usr/projects/tui-vfx/docs/VOCABULARY.md
```

with definitions for:

```text
DescriptorPack
DescriptorCatalog
EmbeddedDescriptor
PackProvidedDescriptor
DescriptorCollision
Canonical fixture
Visual parity
```

---

# J2 definition of done

J2 is done when:

```text
1. descriptor-pack DTOs exist and are schema-backed
2. primitive descriptor pack exists under /usr/projects/tui-vfx/descriptors/v3.1/packs/
3. validator can validate recipes using that pack
4. J0 canonical fixtures still validate
5. J2 second-ring canonical fixtures validate
6. invalid missing-pack / unknown-descriptor / duplicate-descriptor cases fail with stable diagnostics
7. old recipes under /usr/projects/tui-vfx-recipes/recipes/debug_recipes/ are untouched
8. docs/VOCABULARY.md is updated
9. schema fixtures are current
10. cargo fmt/check/clippy/tests pass for affected crates
11. cargo test --workspace passes
12. forbidden dependency guardrails still pass
```

---

# Draft prompt for J2 implementer

```text
You are implementing Phase J2 — Shared Primitive Descriptor Catalog + Second-Ring Migration Batch.

Implementation repo:
  /usr/projects/tui-vfx

Recipe repo:
  /usr/projects/tui-vfx-recipes

Old recipe evidence root:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/

Canonical v3.1 migrated recipe root:
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/

Do not mutate old recipes under:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/

Goal:
  Add a shared primitive descriptor pack/catalog mechanism so canonical v3.1 recipes do not need to copy standard primitive descriptors into every file. Then migrate a second ring of primitive debug recipes using that pack.

Read first:
  /usr/projects/tui-vfx/docs/VOCABULARY.md
  /usr/projects/tui-vfx/docs/v3.1-architecture-overview.md
  /usr/projects/tui-vfx/docs/new_kernel/PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md
  /usr/projects/tui-vfx/docs/new_kernel/PHASE_J1_STATUS_MEMO_TO_ARCHITECT.md
  /usr/projects/tui-vfx/docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md
  /usr/projects/tui-vfx/docs/new_kernel/J1_VALIDATOR_HARNESS_STATUS.md
  /usr/projects/tui-vfx/crates/tui-vfx-contract
  /usr/projects/tui-vfx/crates/tui-vfx-contract-cli

Old recipe evidence to read:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_dissolve.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/samplers/sampler_ripple.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/styles/style_color_fade.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/styles/style_role_scope_border.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_linear_gradient.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/compositions/shader_border_sweep.json

Expected descriptor pack location:
  /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json

Expected new canonical recipe outputs:
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_dissolve.json
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_ripple.json
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_color_fade.json
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_role_scope_border.json
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep.json

Requirements:
  1. Add schema-backed descriptor-pack DTOs to tui-vfx-contract.
  2. Add checked generated schemas for descriptor-pack DTOs.
  3. Create primitive descriptor pack JSON under /usr/projects/tui-vfx/descriptors/v3.1/packs/.
  4. Update tui-vfx-contract-cli to validate recipes with descriptor packs.
  5. Keep embedded descriptors supported for compatibility with J0 fixtures unless you migrate them in place.
  6. Add negative tests for missing pack, unknown descriptor, and duplicate descriptor ids.
  7. Migrate the six second-ring primitive recipes into /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/.
  8. Validate all canonical v3.1 debug recipes recursively.
  9. Update docs/VOCABULARY.md.
  10. Add a J2 status/evidence document.

Non-goals:
  Do not build a visual player.
  Do not claim visual parity.
  Do not mutate /usr/projects/tui-vfx-recipes/recipes/debug_recipes/.
  Do not migrate the full corpus.
  Do not add legacy aliases to canonical schemas.
  Do not import legacy compositor/style/content/shadow/next crates into tui-vfx-contract or tui-vfx-contract-cli.
  Do not port real effect execution logic.

Validation command target:
  cargo run -p tui-vfx-contract-cli -- validate-recipe \
    --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
    --json \
    --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes

Completion gates:
  cargo fmt -p tui-vfx-contract -- --check
  cargo fmt -p tui-vfx-contract-cli -- --check
  cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
  cargo clippy -p tui-vfx-contract-cli --all-targets -- -D warnings
  UPDATE_SCHEMAS=1 cargo test -p tui-vfx-contract --test test_schema_generation -- checked_in_contract_schemas_are_current
  cargo test -p tui-vfx-contract
  cargo test -p tui-vfx-contract-cli
  cargo test --workspace
  cargo tree -p tui-vfx-contract
  cargo tree -p tui-vfx-contract-cli
  forbidden dependency grep over tui-vfx-contract and tui-vfx-contract-cli
  git diff --check
```

---

# Suggested J2 validator commands

Single file with pack:

```bash
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

Recursive canonical corpus:

```bash
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Explicit second-ring batch:

```bash
cargo run -p tui-vfx-contract-cli -- validate-recipe \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_dissolve.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_ripple.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_color_fade.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/styles/style_role_scope_border.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep.json
```

---

# Bottom line

J1 is approved.

The next bottleneck is descriptor duplication, not recipe syntax. J2 should establish the shared primitive descriptor catalog/pack mechanism, then migrate the next primitive ring using it.

That will tell us whether the current schemas are still holding under real mapping pressure without forcing every migrated recipe to become a giant descriptor blob.
