<!-- <FILE>docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md</FILE> - <DESC>K2.3 primitive adapter gap evidence reference</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Primitive adapter work: document adapter burn-down evidence and blocker classifications.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — add primitive-adapter-gap schema, command, outcomes, and current counts.</CLOG> -->

# K2.3 Primitive Adapter Gap Evidence

K2.3 adds honest primitive adapter classification on top of the existing player inventory and visual-frame evidence. It reduces text-grid-renderable unsupported primitives without pretending style/color/role primitives are rendered while visual frames still report `styleKnown: false`.

## Command

```bash
RECIPE_REPO=../tui-vfx-recipes
cargo run -q -p tui-vfx-player-cli -- primitive-adapter-gap \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

## Schema label

```text
v3.1.player.primitiveAdapterGap.1
```

## Report shape

Top-level fields:

```text
schemaVersion
root
descriptorPacks
summary
effects[]
```

Each effect entry includes:

```text
effectId
descriptorCovered
representedByRecipes
adapterStatus
outcome
adapterClass
recipePaths[]
reason
```

## Outcomes

```text
rendered
stillUnsupported
blockedByStyledCellSubstrate
blockedBySemanticDecision
```

## Adapter classes

```text
textGrid
styledCell
descriptor
semanticDecision
unknown
```

## Current burn-down

Starting unsupported ids from K2.2 evidence:

```text
mask.dissolve
sampler.ripple
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

K2.3 adds text-grid adapters for:

```text
mask.dissolve
sampler.ripple
```

Remaining blocked ids:

```text
shader.borderSweep              blockedByStyledCellSubstrate
shader.linearGradient           blockedByStyledCellSubstrate
style.baseStyleOverride         blockedByStyledCellSubstrate
style.colorFade                 blockedByStyledCellSubstrate
```

Current recursive corpus counts:

```text
render-recipe:          total=16 rendered=12 unsupported=4 errors=0
inventory-recipes:      totalRecipes=16 rendered=12 unsupported=4 errors=0 unsupportedEffectIds=4
render-frame:           total=16 rendered=12 unsupported=4 errors=0
primitive-adapter-gap:  totalEffects=14 rendered=10 blockedByStyledCellSubstrate=4 stillUnsupported=0 blockedBySemanticDecision=0
migration-gap:          legacyRecipes=603 v31Recipes=16 representedFamilies=8 unrepresentedFamilies=11 partiallyRepresentedFamilies=7
```

## Captured evidence artifacts

The local run used the default temporary directory. Keep the directory
environment-specific; the artifact basenames are the stable part.

```text
${TMPDIR:-/tmp}/tui-vfx-k23-render-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-inventory-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-visual-frame-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-migration-gap-report.json
${TMPDIR:-/tmp}/tui-vfx-k23-primitive-adapter-gap-report.json
```

## Important limitation

`render-frame` still reports:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

Therefore style/color/role primitives remain blocked until the player can emit real styled-cell evidence.

<!-- <FILE>docs/new_kernel/K2_3_PRIMITIVE_ADAPTER_GAP_EVIDENCE.md</FILE> - <DESC>K2.3 primitive adapter gap evidence reference</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
