# K2.16 scene/layer runtime fidelity report

## Implemented this iteration

- Scene rendering now preserves stable z-index plus authoring-order sort.
- `skipTransparentEmpty` cell write policy is honored in row blitting.
- Render IR records scene, element, optional layer, source, placement, z-index, and cell-write policy provenance.

## Holdbacks

Visibility predicates and full layer skip diagnostics are not yet represented in the canonical recipe DTO runtime path. They remain future scene-runtime work rather than schema-readiness blockers.
