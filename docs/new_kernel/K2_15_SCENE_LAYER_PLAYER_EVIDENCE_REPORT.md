# K2.15 scene/layer player evidence report

## What changed

Scene rendering now carries element-local styled-cell evidence forward when a scene element has a local graph pipeline. The player still renders source rows first, applies an element-local topology on a local surface, then places rows and style evidence into the scene surface.

## Added canonical fixtures

Added under `/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/scene/`:

- `scene_layer_nested_parallel_sequences.json`
- `scene_layer_signal_binding_io.json`
- `scene_layer_surface_base_style.json`

## Honest status

Implemented and proven:

- deterministic z-order/source placement already existed and remains green;
- element-local pipeline topology now preserves local styled-cell evidence during placement;
- signal-backed scene content fixture proves binding/source placement evidence.

Still not complete:

- there is no full `scene.layers` visibility predicate runtime yet;
- transparent-cell blend/clear policy is still row/styled-grid evidence, not full compositor semantics;
- diagnostics do not yet carry full layer/element attribution for every merge/pipeline branch.

This is a meaningful first player-path slice, but not the full scene/layer model.

