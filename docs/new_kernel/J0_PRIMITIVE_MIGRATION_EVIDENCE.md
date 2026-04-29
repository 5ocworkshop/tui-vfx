<!-- <FILE>docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md</FILE> - <DESC>Phase J0 primitive recipe migration validation evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase J0: record old-to-canonical primitive recipe migration evidence and validator results.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document canonical v3.1 recipe fixtures, descriptor seed catalog, validation proof, and known gaps.</CLOG> -->

# Phase J0 Primitive Migration Evidence

Date: 2026-04-29
Implementation repo: `/usr/projects/tui-vfx`
Recipe repo: `/usr/projects/tui-vfx-recipes`
Phase: J0 — Primitive Recipe Migration Pilot + Contract Validator

## Boundary statement

J0 is a non-destructive migration pilot. Existing source recipes under:

```text
/usr/projects/tui-vfx-recipes/recipes/debug_recipes/
```

were read as evidence only. They were not edited, normalized, renamed, or moved.

New canonical v3.1 fixtures were written under:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/
```

The old recipe corpus remains the old authoring/evidence shape. The new fixture corpus is strict canonical `tui_vfx_contract::RecipeDocument` JSON.

## Validator added

J0 adds a dedicated contract-only validator crate:

```text
crates/tui-vfx-contract-cli
```

Current command shape:

```text
cargo run -p tui-vfx-contract-cli -- validate-recipe <recipe.json> [more-recipe.json ...]
```

Validation path:

```text
read JSON
  -> serde deserialize as tui_vfx_contract::RecipeDocument
  -> RecipeDocument::validate()
  -> structured JSON report
  -> exit 0 only when all files validate
```

The validator intentionally depends only on:

```text
tui-vfx-contract
serde
serde_json
```

It does not depend on `tui-vfx-compositor`, `tui-vfx-style`, `tui-vfx-content`, `tui-vfx-shadow`, or `tui-vfx-next`.

## Migrated fixture set

| Old evidence recipe | New canonical v3.1 fixture | Status |
|---|---|---|
| `debug_recipes/baseline.json` | `recipes/v3.1/debug_recipes/baseline.json` | Validates |
| `debug_recipes/filters/filter_dim.json` | `recipes/v3.1/debug_recipes/filters/filter_dim.json` | Validates |
| `debug_recipes/filters/filter_tint.json` | `recipes/v3.1/debug_recipes/filters/filter_tint.json` | Validates |
| `debug_recipes/filters/filter_invert.json` | `recipes/v3.1/debug_recipes/filters/filter_invert.json` | Validates |
| `debug_recipes/filters/filter_greyscale.json` | `recipes/v3.1/debug_recipes/filters/filter_greyscale.json` | Validates |
| `debug_recipes/masks/mask_none.json` | `recipes/v3.1/debug_recipes/masks/mask_none.json` | Validates |
| `debug_recipes/masks/mask_wipe.json` | `recipes/v3.1/debug_recipes/masks/mask_wipe.json` | Validates |
| `debug_recipes/masks/mask_checkers.json` | `recipes/v3.1/debug_recipes/masks/mask_checkers.json` | Validates |
| `debug_recipes/samplers/sampler_sinewave.json` | `recipes/v3.1/debug_recipes/samplers/sampler_sinewave.json` | Validates |
| `debug_recipes/event_driven_dwell/bool_binding_demo.json` | `recipes/v3.1/debug_recipes/event_driven_dwell/bool_binding_demo.json` | Validates |

## Descriptor seed catalog represented in fixtures

J0 keeps the seed catalog small and embedded in the canonical recipe documents.
Each fixture carries only the descriptors needed for that recipe.

### Source descriptor seed

```text
source.card
```

Purpose: produce the old debug-card-like baseline surface as a canonical source-produced surface.

Typed inputs:

| Input | Kind | Purpose |
|---|---|---|
| `message` | `text` | Human-facing text content for the debug card. |
| `width` | `integer` | Width in cells. |
| `height` | `integer` | Height in cells. |

Output contract:

```text
size: inputDriven
roles: generated
```

Lifecycle metadata:

```text
deterministicWithSeed: true
timeAware: false
resizeAware: true
```

### Effect descriptor seeds

| Descriptor | Domain | Typed inputs |
|---|---|---|
| `filter.dim` | `frameFilter` | `factor: number`, `applyTo: enum(both, foreground, background)` |
| `filter.tint` | `frameFilter` | `color: color`, `strength: number`, `applyTo: enum(both, foreground, background)` |
| `filter.invert` | `frameFilter` | `applyTo: enum(both, foreground, background)` |
| `filter.greyscale` | `frameFilter` | `strength: number`, `applyTo: enum(both, foreground, background)` |
| `mask.none` | `mask` | none |
| `mask.wipe` | `mask` | `direction: enum(leftToRight, rightToLeft)`, `softEdge: boolean` |
| `mask.checkers` | `mask` | `cellSize: integer` |
| `sampler.sineWave` | `coordinateSampler` | `axis: enum(x, y)`, `amplitude: number`, `frequency: number`, `speed: number`, `phaseOffset: number` |

All descriptors declare strict typed inputs rather than preserving old recipe field spellings as compatibility aliases.
For example, old `phase_offset` becomes canonical `phaseOffset` only in the new descriptor input vocabulary.

## Recipe document modeling choices

Each migrated fixture uses the H1/I0 canonical root shape:

```text
RecipeDocument
  metadata
  lifecycle
  sourceDescriptors
  sources
  graph
  scenes
```

Common source instance:

```text
mainCard -> source.card
```

Common scene:

```text
mainScene
  mainElement -> mainCard
```

Primitive effect steps are represented as canonical graph nodes with descriptor-backed typed inputs. The graph uses linear `order` with `topology: null`, relying on the locked G3 fallback semantics.

The scene element `pipeline` is left `null` in this pilot. J0 validates graph/descriptor/source contracts but does not claim runtime element-local execution or visual parity.

## Lifecycle mapping

Old `auto_dismiss_ms` evidence maps to canonical recipe-level lifecycle:

```text
enter: fixed duration
 dwell: fixed duration or trigger-terminated dwell with maxDuration
 exit: fixed duration
```

The event-driven dwell boolean binding demo maps old event-like binding pressure into:

```text
GraphSpec.signals.userDismissed: boolean
RecipeDocument.lifecycle.dwell.policy.kind: until
TriggerCondition.source: ValueSource::Signal(userDismissed)
ValuePredicate: isTrue
TriggerAction: advancePhase
maxDuration: 5000 ms
```

This intentionally uses signal + trigger vocabulary from I0, not old binding field names.

## Preserved evidence gaps

J0 found no need to change `RecipeDocument`, `GraphSpec`, `LifecycleSpec`, `SourceSpec`, `EffectDescriptor`, or `ValueSource` for the selected primitive batch.

Known non-goals / gaps left for later phases:

- No runtime/player execution.
- No visual parity check.
- No full corpus migration.
- No lowering compiler from old recipes.
- No element-local pipeline execution proof.
- No source-rendering implementation for `source.card`.
- No legacy aliases in DTOs or canonical recipe files.
- Old foreground/background channel-scoped filter intent is represented as canonical `applyTo` enum input for this pilot, not as a new scope algebra concept.

## Verification evidence

Validator run over the full J0 canonical fixture set:

```text
cargo run -q -p tui-vfx-contract-cli -- validate-recipe \
  $(find /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes -name '*.json' | sort)
```

Result: all ten files returned structured reports with:

```json
"valid": true
```

Automated CLI tests:

```text
cargo test -p tui-vfx-contract-cli
```

Result:

```text
2 passed; 0 failed
```

The tests prove both success for the J0 fixture corpus and nonzero structured failure for invalid JSON.

<!-- <FILE>docs/new_kernel/J0_PRIMITIVE_MIGRATION_EVIDENCE.md</FILE> - <DESC>Phase J0 primitive recipe migration validation evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
