<!-- <FILE>docs/new_kernel/K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md</FILE> - <DESC>K2.11 studio control-surface preflight note</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.11 v3.1 control-surface reach-goal assessment.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — defer studio control-surface implementation behind schema/source readiness.</CLOG> -->

# K2.11 Studio Control-Surface Preflight

## Decision

The control-surface report is **deferred** from K2.11 implementation. K2.11 needed to make the v3.1 schema-readiness blockers explicit first; adding a new control report before resolving value-source, binding, lifecycle, and source/content semantics would risk inventing UI controls for semantics the contract has not approved.

## Why this is the right deferral

A future studio control surface must derive from:

- `RecipeDocument`
- `DescriptorPack` / `DescriptorCatalog`
- `SourceDescriptor` inputs
- `EffectDescriptor` inputs
- graph signals, parameters, and bindings

K2.11 proves that several of those inputs are not yet stable enough for a trustworthy studio surface:

| Blocker | Why it blocks studio controls |
|---|---|
| `bindingSemantics` | A control cannot safely mutate a bound field until binding execution and parameter override semantics are explicit. |
| `valueSourceSemantics` | Sampled-surface and signal-shaped values need a model before exposing sliders/input boxes. |
| `lifecycleSemantics` / `motionTimingSemantics` | Time-aware controls need trigger/dwell/easing/motion semantics that are not settled. |
| `sourceDescriptor` | Source controls need stable descriptors for text, ANSI, image, procedural, and offline artifacts. |
| `sceneSemantics` | Scene-local controls need placement, layers, source-local pipeline, and asset/procedural seams. |

## K3.0 trigger

Start a `control-surface` command only after the following v3.1 evidence is true:

1. source/content descriptor tranche has descriptor-backed `source.text` fixtures and explicit `source.ansi`/image/procedural decisions,
2. lifecycle/signal/binding/value-source packet defines runtime mutability and binding execution boundaries,
3. primitive field coverage has no vague shader field blockers,
4. canonical fixture-QC and descriptor report gates are green.

## Future command shape

```bash
cargo run -q -p tui-vfx-player-cli -- control-surface \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --json \
  "$RECIPE_REPO/recipes/v3.1/debug_recipes/filters/filter_tint.json"
```

The command must be report-only, must not mutate recipes, and must not invent runtime binding semantics.

<!-- <FILE>docs/new_kernel/K2_11_STUDIO_CONTROL_SURFACE_PREFLIGHT.md</FILE> - <DESC>K2.11 studio control-surface preflight note</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
