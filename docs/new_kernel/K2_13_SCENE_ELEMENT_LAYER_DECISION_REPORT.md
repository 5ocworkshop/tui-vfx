<!-- <FILE>docs/new_kernel/K2_13_SCENE_ELEMENT_LAYER_DECISION_REPORT.md</FILE> - <DESC>K2.13 scene, element, and layer decision report</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.13 schema decision burn-down: record scene/element/layer schema disposition.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document scene semantics accepted for schema lock and remaining player/compositor backlog.</CLOG> -->

# K2.13 Scene, Element, and Layer Decision Report

## Decision

Scene support is core v3.1 schema work, not optional future vocabulary.

The accepted model remains:

```text
Scene
  elements[] / layers[]

Element or Layer
  id
  z / layer
  source
  placement
  visibility
  surface/base_style
  cell_motion
  local pipeline
  clip/overflow policy
```

Role identity and element identity remain separate. `text`, `border`, `background`, `shadow`, and `content` are roles. `ansi_layer`, `logo`, `spinner`, `card`, and `visibility_io_card` are elements.

## Composition semantics

1. Sort by z, then authoring order.
2. Render each layer/source.
3. Apply layer-local content/cell-motion/pipeline in layer-local coordinates.
4. Composite into scene/global coordinates.
5. Skipped cells preserve lower content.
6. Transparent writes blend through or preserve lower content unless an explicit clear policy exists.
7. Non-transparent writes replace according to write policy.
8. Overlap conflicts emit diagnostics unless explicitly resolved by policy.

## Disposition

Scene semantics are `acceptedSchema`. Current scene/complex offenders are not schema blockers after disposition mapping; they are fixture, descriptor, player, compositor, or human-review backlog.

<!-- <FILE>docs/new_kernel/K2_13_SCENE_ELEMENT_LAYER_DECISION_REPORT.md</FILE> - <DESC>K2.13 scene, element, and layer decision report</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
