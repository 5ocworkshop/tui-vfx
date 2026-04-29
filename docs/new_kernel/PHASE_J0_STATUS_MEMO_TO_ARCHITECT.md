<!-- <FILE>docs/new_kernel/PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J0 wrap: report primitive recipe migration pilot and request next assignment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add Phase J0 architect memo in the established status-memo style.</CLOG> -->

# Phase J0 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`
Phase: J0 — Primitive Recipe Migration Pilot + Contract Validator

## Executive summary

Phase J0 implements the non-destructive primitive migration pilot from `ARCH-RESP-TO-PHASE_I0.md`.

Current answer: **yes, a curated primitive batch from the old debug recipe corpus can be represented as strict canonical v3.1 `RecipeDocument` JSON without changing old recipes and without changing the locked contract DTOs.** The work adds a dedicated `tui-vfx-contract-cli` validator, creates ten canonical v3.1 recipe fixtures in the separate recipes repo, and proves those fixtures deserialize and pass `RecipeDocument::validate()`.

The phase intentionally stops before full corpus migration, visual parity, a player/runtime, an old-to-new compiler, element-local pipeline execution, source rendering, or shared descriptor packaging.

## Current implementation state

New validator crate in the implementation repo:

```text
crates/tui-vfx-contract-cli
```

Command shape:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <recipe.json> [more-recipe.json ...]
```

New canonical recipe root in the recipes repo:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

Old source/evidence root preserved untouched:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

## Goal-by-goal status against the J0 recommendation

| J0 goal / question | Current status |
|---|---|
| Build fresh validator on `tui-vfx-contract` | **Done.** `tui-vfx-contract-cli` deserializes `RecipeDocument`, runs `validate()`, emits structured JSON reports, and exits nonzero on failures. |
| Avoid legacy/runtime dependencies | **Done.** The CLI depends on `tui-vfx-contract`, `serde`, and `serde_json`; it does not depend on compositor/style/content/shadow/next crates. |
| Preserve old recipes | **Done.** Old `/recipes/debug_recipes/` files were read as evidence only and not modified. |
| Create separate canonical output hierarchy | **Done.** New fixtures are under `/recipes/v3.1/debug_recipes/`, preserving category paths. |
| Migrate representative primitives | **Done.** Baseline, four filters, three masks, sine-wave sampler, and the boolean event-driven dwell demo validate. |
| Seed source/effect descriptors | **Done in fixtures.** `source.card` plus primitive effect descriptors are embedded with typed inputs sufficient for the migrated recipes. |
| Prove contract validation | **Done.** CLI and integration tests validate all ten canonical fixtures. |
| Record gaps | **Done.** `J0_PRIMITIVE_MIGRATION_EVIDENCE.md` lists non-goals and future work. |

## Migrated canonical fixture set

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_dim.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_tint.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_invert.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/filters/filter_greyscale.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_none.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_checkers.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/samplers/sampler_sinewave.json
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json
```

## Key decisions

### The canonical fixtures are strict recipe documents, not old authoring syntax

Each fixture is a full `RecipeDocument` with `metadata`, `lifecycle`, `sourceDescriptors`, `sources`, `graph`, and `scenes`. No old DTO aliases were added.

### Seed descriptors stay tiny

The pilot embeds only enough descriptor vocabulary to represent selected primitives:

```text
source.card
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
sampler.sineWave
```

This answers whether migration pressure fits the contract. It does not yet decide how a future shared descriptor catalog should be packaged.

### Old channel-scoped filter pressure did not require a new scope kind

Old foreground/background channel-specific filter intent is represented as a typed canonical `applyTo` enum input for the primitive descriptors. That kept scope algebra stable for J0 while preserving the evidence.

### Event dwell maps to signals plus lifecycle triggers

The old boolean binding dwell demo is represented by a `userDismissed` `SignalSpec` and an I0 trigger-terminated dwell policy with `maxDuration`. It does not preserve old binding field names.

## Evidence grounding

Created:

```text
docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md
```

The evidence report lists old-to-new recipe mappings, the embedded source/effect descriptor seeds, lifecycle mapping, validation commands, and known gaps.

## Verification evidence

Passed:

```text
cargo fmt -p tui-vfx-contract-cli -- --check
cargo check -p tui-vfx-contract-cli
cargo test -p tui-vfx-contract-cli
cargo run -q -p tui-vfx-contract-cli -- validate-recipe $(find /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes -name '*.json' | sort)
git diff --check
cargo test --workspace
```

Automated test result:

```text
2 passed; 0 failed
```

Manual validator result: all ten canonical fixtures emitted structured reports with `valid: true`.

## Request for next assignment

Please review Phase J0 as the primitive migration pilot / contract validator checkpoint and advise the next phase.

The likely next decision is whether to move toward a tiny lowering/compiler skeleton, a shared canonical descriptor catalog/package, a runtime/source-rendering proof, or a broader second-ring migration batch.

<!-- <FILE>docs/new_kernel/PHASE_J0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase J0 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
