<!-- <FILE>docs/new_kernel/PHASE_D1_STATUS.md</FILE> - <DESC>Concise Phase D1 scene/element/layer composition status</DESC> -->
<!-- <VERS>VERSION: 0.3.0</VERS> -->
<!-- <WCTX>New kernel Phase D1 wrap: record final green verification evidence.</WCTX> -->
<!-- <CLOG>0.3.0: MINOR — record final green verification and architect approval.
0.2.0: MINOR — record scene role-write policy regression reinforcement.
0.1.1: PATCH — record public SceneOutcome schema root and field-stable diagnostic paths.
0.1.0: INIT — record Phase D1 scene composition proof and remaining phase gates.</CLOG> -->

# Phase D1 Status — Scene / Element / Layer Composition Semantics

## Status

Phase D1 is implemented, architect-approved, deslop-reinforced, and verified green.

## What changed

Phase D1 adds a minimal scene layer above `Surface` in `crates/tui-vfx-next`:

- `Scene` — final scene size plus declaration-ordered elements.
- `SceneElement` — stable element identity, optional layer id, z order, placement, local surface, clip policy, and write policies.
- `ElementId` — stable instance identity distinct from `RoleTag`.
- `LayerId` — optional lightweight grouping identity; no full layer graph.
- `ElementPlacement` — signed scene coordinate placement for element-local `(0, 0)`.
- `ClipPolicy` — `Clip` and `Warn` out-of-bounds behavior.
- `SceneOutcome` — final surface, matched/written/clipped counts, and diagnostics.

Composition now proves:

- Multiple non-overlapping elements compose into one final `Surface`.
- Element identity and semantic role identity remain distinct.
- Higher `z_index` elements compose later and overwrite lower cells/roles.
- Equal `z_index` ties are broken by declaration order.
- Skipped transparent top cells preserve lower/current output.
- Transparent empty writes can clear lower/current output under `WriteCell`.
- Signed placement maps element-local coordinates into scene coordinates.
- Out-of-bounds element cells clip without panics.
- Warning diagnostics identify the clipped element by `scene.element[index].id` path.
- Scene-specific tests cover `PreserveDestination` and `SetExplicit` role policies.

## What deliberately did not change

Phase D1 does not add:

- effect descriptor expansion;
- recipe schema/compiler;
- studio manifest;
- runtime bindings;
- phase graph or trigger engine;
- legacy migration;
- real effect ports;
- template inheritance implementation;
- full layer graph;
- complex blending engine.

## Schema/reference state

New checked schema roots:

- `schemas/v3.1/next/scene.schema.json`
- `schemas/v3.1/next/element.schema.json`
- `schemas/v3.1/next/outcome.schema.json`

Existing schema generation tests now include `Scene`, `SceneElement`, and `SceneOutcome` roots and continue to enforce strict object shapes and property descriptions.

## Tests added

`crates/tui-vfx-next/tests/test_scene_contract.rs` adds:

- `scene_composes_multiple_elements`
- `element_identity_is_distinct_from_role`
- `higher_z_element_overwrites_lower_cell_and_role`
- `z_tie_breaks_by_declaration_order`
- `skipped_top_element_preserves_lower_output`
- `transparent_empty_top_write_can_clear_when_policy_writes`
- `element_placement_uses_scene_coordinates`
- `out_of_bounds_element_cells_are_clipped`
- `scene_diagnostics_include_element_identity`
- `scene_role_policy_can_preserve_lower_role`
- `scene_role_policy_can_set_explicit_role`

## Verification evidence

Final phase verification passed:

- `cargo fmt --package tui-vfx-next -- --check` — PASS
- `cargo clippy -p tui-vfx-next --all-targets -- -D warnings` — PASS
- `cargo test -p tui-vfx-next` — PASS
- `UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current` — PASS
- `cargo test -p tui-vfx-next --test test_schema_generation` — PASS
- `cargo tree -p tui-vfx-next` — PASS / inspected
- forbidden dependency grep for compositor/style/content/shadow crates — PASS / no matches
- `git diff --check -- crates/tui-vfx-next docs schemas/v3.1/next` — PASS
- architect re-verification — APPROVED
- changed-file deslop pass — PASS; added role-write policy regression reinforcement
- `cargo test --workspace` — PASS

## Open questions for architect follow-up

- Should D2 be the template composition design document recommended in `ARCH-RESP-TO-PHASE_D0.md`?
- Should future scene phases add `ClipPolicy::Error`, or leave D1's `Clip`/`Warn` as the minimal contract until recipes need hard validation?

<!-- <FILE>docs/new_kernel/PHASE_D1_STATUS.md</FILE> - <DESC>Concise Phase D1 scene/element/layer composition status</DESC> -->
<!-- <VERS>END OF VERSION: 0.3.0</VERS> -->
