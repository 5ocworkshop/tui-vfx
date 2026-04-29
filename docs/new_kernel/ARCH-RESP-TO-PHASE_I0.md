<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_I0.md</FILE> - <DESC>Architect response accepting Phase I0 and assigning Phase J0</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J0: record architect direction for primitive recipe migration pilot and contract validator.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase I0 architect response and Phase J0 assignment.</CLOG> -->

## Architect response to Phase I0

Phase I0 is accepted as the lifecycle/time/trigger contract lock point. The next useful phase should be a **non-destructive primitive migration pilot** that proves representative existing debug recipes can be expressed as strict canonical v3.1 documents.

I would call this:

```text
Phase J0 — Primitive Recipe Migration Pilot + Contract Validator
```

or, if you prefer to keep it under the H-series:

```text
Phase H2 — Primitive Recipe Migration Pilot + Contract Validator
```

The important point is that this phase is **not a full corpus migration** and **not a visual parity/player phase**. It is a schema/contract/lowering proof.

---

# Phase J0 — Primitive Recipe Migration Pilot + Contract Validator

## Executive summary

Now that the contract stack includes surfaces, scenes, graphs, effects, values, sources, assets, lifecycle, and triggers, the next phase should test whether existing primitive debug recipes can be represented as canonical v3.1 documents **without changing the existing recipes**.

Existing recipes live in a separate repository:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*
```

The implementation work is in:

```text
/usr/projects/tui-vfx
```

This cross-repo boundary must be explicit. The phase should **read** existing recipes from `/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*` as evidence and source material, but must write new canonical v3.1 recipes into a separate hierarchy:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Example mapping:

```text
OLD, do not edit:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_dim.json

NEW canonical v3.1 output:
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
```

The old recipe corpus remains untouched.

---

## Why this phase exists

We need to answer:

```text
Can representative existing primitive recipes be lowered or hand-mapped into
strict canonical v3.1 RecipeDocument JSON without inventing new contract concepts?
```

That is the real lock test for the schemas.

If migration requires only:

```text
add descriptor
add source descriptor
add canonical recipe fixture
add validator test
```

then the contract is working.

If migration requires changing `RecipeDocument`, `GraphSpec`, `LifecycleSpec`, `SourceSpec`, `EffectDescriptor`, or `ValueSource` repeatedly, then we have found contract gaps.

---

## Required source recipe context

The implementer and any subagents should read from the recipes repo, not from `/usr/projects/tui-vfx`.

Root:

```text
/usr/projects/tui-vfx-recipes/recipes/
```

Existing debug recipes:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*
```

Recommended first context set:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/baseline.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_tint.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_invert.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_greyscale.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_kitt_scanner_progress_binding.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_none.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_wipe.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_checkers.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/samplers/sampler_sinewave.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/shaders/primitives/shader_linear_gradient.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/styles/style_role_scope_border.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_demo.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_truthy_loopback.json
```

Optional second-ring context if the first set maps cleanly:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/loopback/loopback_rigid_shake_severity_ramp.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_io_filter_shader.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_layer_visibility_binding_io.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/scene/scene_authoring_ladder_procedural_spinner_binding.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/content/content_split_flap_solari_authentic.json
```

These are evidence and migration targets. They are **not** canonical field-name authorities.

---

## New recipe output location

Create canonical v3.1 debug recipes under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Preserve the old relative category structure where practical.

Examples:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json

/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_tint.json

/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json

/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json
```

Do not overwrite, rewrite, normalize, delete, rename, or move files under:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

---

## Validation tool recommendation

Yes, we need a fresh validation tool.

The legacy recipe/pipeline validation tooling in `/usr/projects/tui-vfx-recipes` should be treated as evidence only. It validates the old authoring and pipeline shape. It should not become the validator for strict canonical v3.1.

Going forward, validation should be built on top of:

```text
/usr/projects/tui-vfx/crates/tui-vfx-contract
```

The minimal tool should do this:

```text
read JSON file
    -> deserialize as tui_vfx_contract::RecipeDocument
    -> run RecipeDocument / GraphSpec / SourceSpec / lifecycle validations
    -> emit structured validation errors
    -> exit nonzero on failure
```

A good initial command shape would be:

```text
cargo run -p tui-vfx-contract-cli -- \
  validate-recipe \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
```

Or, if the project prefers `xtask`:

```text
cargo run -p xtask -- \
  validate-v31-recipe \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
```

My preference is a small dedicated crate:

```text
/usr/projects/tui-vfx/crates/tui-vfx-contract-cli
```

Reason: it can depend only on the stable contract crate and avoid inheriting old recipe tooling assumptions.

Dependency direction should be:

```text
tui-vfx-types
    ↓
tui-vfx-contract
    ↓
tui-vfx-contract-cli
```

It must not depend on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
tui-vfx-next
```

`tui-vfx-next` can remain the proof executor. The validator should validate canonical contract JSON, not execute it.

---

## J0 deliverables

The phase should produce:

```text
1. A tiny contract-based recipe validation CLI or xtask command.
2. A small descriptor/source descriptor seed catalog sufficient for selected primitives.
3. A curated set of canonical v3.1 migrated debug recipes under:
   /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
4. Tests proving those canonical recipe files validate.
5. A migration evidence report documenting what mapped cleanly and what did not.
```

Recommended report path in `/usr/projects/tui-vfx`:

```text
docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md
```

Recommended status memo path:

```text
docs/new_kernel/PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md
```

---

## Initial migration targets

Start with 6–10 recipes, not the full primitive library.

Recommended first batch:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/baseline.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_tint.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_invert.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_greyscale.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_none.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_wipe.json
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_checkers.json

/usr/projects/tui-vfx-recipes/recipes/debug_recipes/samplers/sampler_sinewave.json
```

Add one binding/lifecycle case if the first batch is smooth:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_demo.json
```

---

## What “success” means

J0 succeeds if:

```text
- existing recipes are not modified;
- new canonical recipes are written under /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/;
- each new canonical recipe deserializes as RecipeDocument;
- each new canonical recipe passes contract validation;
- no new legacy aliases are added to canonical DTOs;
- no old recipe field names leak into canonical schemas unless already intentionally accepted;
- any unmapped pressure is recorded as evidence, not patched around with ad hoc schema changes.
```

The key measure is not visual parity yet. The key measure is:

```text
Can old primitive intent map into canonical v3.1 shape cleanly?
```

---

## What should not happen

Do not modify:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*
```

Do not make `RecipeDocument` accept old authoring syntax directly.

Do not add compatibility aliases such as:

```text
dwell_until_binding
dwell_fallback_ms
phase_offset
requires_bindings
requires_assets
pipeline.step
payload blobs without descriptor validation
"{{ asset_token }}" asset interpolation
```

Those belong in a future lowering/migration layer, not the canonical contract.

Do not use the legacy validator as the authority for v3.1 canonical recipes.

---

## Draft implementer prompt

```text
You are implementing Phase J0 — Primitive Recipe Migration Pilot + Contract Validator.

Repository for implementation:
  /usr/projects/tui-vfx

Separate recipe repository for source/evidence and generated canonical recipe files:
  /usr/projects/tui-vfx-recipes

Existing recipes are under:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/**/*

Do not edit, move, delete, normalize, or overwrite any files under:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/

Create new canonical v3.1 recipe files under:
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/

Preserve relative category paths where practical. For example:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_dim.json
maps to:
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json

Goal:
  Prove that a small curated set of existing primitive debug recipes can be represented
  as strict canonical v3.1 RecipeDocument JSON using the contract crate.

Required first source recipes to read:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/baseline.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_dim.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_tint.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_invert.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/filters/filter_greyscale.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_none.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_wipe.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/masks/mask_checkers.json
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/samplers/sampler_sinewave.json

Optional binding/lifecycle source recipe if first batch is clean:
  /usr/projects/tui-vfx-recipes/recipes/debug_recipes/event_driven_dwell/bool_binding_demo.json

Build a fresh canonical validator on top of tui-vfx-contract. Do not reuse the legacy recipe/pipeline validator as the v3.1 authority.

Preferred validator shape:
  crates/tui-vfx-contract-cli

Example command:
  cargo run -p tui-vfx-contract-cli -- validate-recipe \
    /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json

The validator should:
  - read JSON;
  - deserialize as tui_vfx_contract::RecipeDocument;
  - run contract validations;
  - print structured errors;
  - exit nonzero on validation failure.

The validator must not depend on:
  tui-vfx-compositor
  tui-vfx-style
  tui-vfx-content
  tui-vfx-shadow
  tui-vfx-next

Add enough descriptor/source descriptor fixtures or catalog support to validate the selected primitive recipes. Keep it tiny. Do not port real runtime effects.

Create:
  docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md
  docs/new_kernel/PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md

Success criteria:
  - old debug recipes untouched;
  - new canonical v3.1 recipes exist under /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/;
  - selected canonical recipes parse and validate as RecipeDocument;
  - contract/schema/vocabulary docs updated if public vocabulary changes;
  - no legacy field names are added to canonical DTOs merely for compatibility;
  - unmapped concepts are recorded as migration evidence.

Verification:
  cargo fmt --package tui-vfx-contract -- --check
  cargo clippy -p tui-vfx-contract --all-targets -- -D warnings
  cargo test -p tui-vfx-contract
  cargo test --workspace
  cargo tree -p tui-vfx-contract
  forbidden legacy dependency grep over contract and validator crates
  run the new validator against every new recipe under:
    /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
  git diff --check
```

---

## Direct answer on validation

Yes, we need separate validation going forward.

The validation stack should be:

```text
JSON Schema validation
    proves wire shape

Serde deserialization as RecipeDocument
    proves Rust-owned canonical shape

Contract validation methods
    prove semantic links and references

Optional proof execution in tui-vfx-next
    proves behavior for proof-supported nodes only
```

The old validator remains useful for understanding old recipes, but canonical v3.1 should be validated by a new tool built directly on `tui-vfx-contract`.

That gives us a clean future path:

```text
legacy/debug recipe
    -> lowering/migration layer
    -> canonical RecipeDocument JSON
    -> v3.1 contract validator
    -> optional proof executor
    -> later production player
```

That is the point where the work starts getting easier: new features become additions to descriptors, sources, schemas, and lowering rules instead of ad hoc changes across the whole system.
<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_I0.md</FILE> - <DESC>Architect response accepting Phase I0 and assigning Phase J0</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
