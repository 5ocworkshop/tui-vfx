<!-- <FILE>docs/new_kernel/PHASE_D1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase D1 wrap: record final workspace verification before commit.</WCTX> -->
<!-- <CLOG>0.2.0: MINOR — record final workspace green result.
0.1.0: INIT — add Phase D1 architect memo in the established status-memo style.</CLOG> -->

# Phase D1 Status Memo to the v3.1 Surface-Contract Architect

Date: 2026-04-28
Repo: `/usr/projects/tui-vfx`
Phase: D1 — Scene / Element / Layer Composition Semantics

## Executive summary

Phase D1 has reached the proof point recommended in `ARCH-RESP-TO-PHASE_D0.md`:

```text
Can v3.1 represent multiple elements at once with deterministic placement,
ordering, overlap, role propagation, skipped-cell behavior, and diagnostics?
```

Current answer: **yes, for the bounded Phase D1 scene/element/layer semantic proof.**

The clean-room crate now has a minimal scene model above `Surface`. `Scene` composes multiple placed `SceneElement` values into one final semantic `Surface`. Composition is deterministic by ascending `z_index`, with declaration order as the tie-break. Element-local coordinates map into signed scene coordinates, allowing partially offscreen elements to clip. Higher/later written cells overwrite lower/current cells and roles according to existing cell/role write policies. Skipped transparent top cells preserve lower/current output. Empty transparent writes remain writes under `WriteCell`.

Element identity is now explicit and distinct from `RoleTag`: `ElementId` names instances for diagnostics and future recipe references; roles continue to classify semantic cell content. Optional `LayerId` exists only as lightweight grouping metadata. D1 does not build a full layer graph.

## Current implementation state

### New D1 contract types

Added in `crates/tui-vfx-next/src`:

```text
cls_scene.rs
cls_scene_element.rs
cls_scene_outcome.rs
cls_element_id.rs
cls_layer_id.rs
cls_element_placement.rs
cls_clip_policy.rs
```

New public contract surface:

```text
Scene
SceneElement
SceneOutcome
ElementId
LayerId
ElementPlacement
ClipPolicy
```

Updated existing diagnostic surface:

```text
SurfaceDiagnosticCode::SceneElementClipped
SurfaceDiagnostic::scene_element_clipped(...)
```

The diagnostic path is field-stable:

```text
scene.element[index].id
```

The actual element id remains in the human-facing diagnostic message. This avoids baking the instance id into the field path while still making diagnostics element-aware.

## Goal-by-goal status against the D1 recommendation

| D1 goal / constraint | Current status |
|---|---|
| Add a scene model above `Surface` | **Done.** `Scene` has final width/height and declaration-ordered elements. |
| Compose multiple elements into one final surface | **Done.** `Scene::compose()` returns `SceneOutcome { surface, counts, diagnostics }`. |
| Keep element identity distinct from roles | **Done.** `ElementId` is separate from `RoleTag`; tests prove a `titleCard` element can write `RoleTag::Text`. |
| Add optional layer identity without a full layer graph | **Done.** `LayerId` is optional on `SceneElement`; no graph/blending semantics were added. |
| Use scene-coordinate placement | **Done.** `ElementPlacement { x, y }` is signed and maps local `(0,0)` into scene coordinates. |
| Sort by zIndex and declaration order | **Done.** Ascending `z_index`; declaration order is the deterministic tie-break. |
| Higher/later written cells overwrite lower/current cells and roles | **Done.** Tests cover cell and role overwrite. |
| Skipped top element preserves lower output | **Done.** `SkipTransparentEmpty` preserves lower/current cell and role. |
| Transparent empty write can clear | **Done.** `WriteCell` writes empty transparent cells and the element-local role. |
| Diagnostics identify scene element | **Done.** Warning clip diagnostics use `scene.element[index].id` path and include the element id in the message. |
| D0 schema/reference rule applies to new public types | **Done.** Scene, element, and outcome schemas are checked in and validated. |
| Do not begin descriptors/recipes/runtime/legacy migration/effect ports | **Respected.** D1 is a semantic proof only. |
| Preserve clean-room dependency boundary | **Respected.** `tui-vfx-next` still does not depend on compositor/style/content/shadow crates. |

## Generated schema roots

D1 adds these checked schema roots under `schemas/v3.1/next/`:

```text
scene.schema.json      # Scene
element.schema.json    # SceneElement
outcome.schema.json    # SceneOutcome
```

The existing schema test now covers D1 roots in addition to the D0 roots. It proves fixture freshness, strict object shapes, property descriptions, and rustdoc description presence.

## Required tests now present

New D1 integration test file:

```text
crates/tui-vfx-next/tests/test_scene_contract.rs
```

It covers:

```text
scene_composes_multiple_elements
element_identity_is_distinct_from_role
higher_z_element_overwrites_lower_cell_and_role
z_tie_breaks_by_declaration_order
skipped_top_element_preserves_lower_output
transparent_empty_top_write_can_clear_when_policy_writes
scene_role_policy_can_preserve_lower_role
scene_role_policy_can_set_explicit_role
element_placement_uses_scene_coordinates
out_of_bounds_element_cells_are_clipped
scene_diagnostics_include_element_identity
```

Schema generation also has a named D1 freshness test:

```text
scene_schema_generation_is_current
```

## Dependency and architecture status

The clean-room dependency direction remains narrow:

```text
tui-vfx-types
    ↓
tui-vfx-next
```

D1 does not introduce dependencies on:

```text
tui-vfx-compositor
tui-vfx-style
tui-vfx-content
tui-vfx-shadow
```

## OFPF / file-structure status

New source files follow the clean-room OFPF convention:

```text
crates/tui-vfx-next/src/cls_*.rs
```

The largest new source file is `cls_scene.rs`, which remains below the `cls_` hard LOC limit. The new test file uses the existing `test_*.rs` convention.

## Verification evidence

Fresh verification during wrap:

```text
cargo fmt --package tui-vfx-next -- --check                       PASS
cargo clippy -p tui-vfx-next --all-targets -- -D warnings         PASS
cargo test -p tui-vfx-next                                        PASS
UPDATE_SCHEMAS=1 cargo test -p tui-vfx-next --test test_schema_generation -- checked_in_schemas_are_current  PASS
cargo test -p tui-vfx-next --test test_schema_generation          PASS
cargo tree -p tui-vfx-next                                        PASS / inspected
forbidden dependency grep                                         PASS / no matches
git diff --check -- crates/tui-vfx-next docs schemas/v3.1/next    PASS
```

`cargo test --workspace` was rerun after the final deslop/test-reinforcement pass and passed.

## Architect verification and deslop

The first architect verification rejected two issues:

1. diagnostic paths embedded the element id instead of using a stable field path;
2. `SceneOutcome` was public/schema-facing but lacked a checked schema root.

Both were fixed:

- paths now use `scene.element[index].id`;
- `outcome.schema.json` is checked in and covered by schema tests.

The second architect verification approved the result.

The Ralph deslop pass then added direct scene tests for `RoleWritePolicy::PreserveDestination` and `RoleWritePolicy::SetExplicit`, because the role policy field was documented and should be directly covered by D1 tests.

## Scope control

D1 intentionally did not implement:

```text
effect descriptors
recipe schema/compiler
studio manifest
runtime bindings
phase graph
trigger engine
legacy migration
real effect ports
template inheritance implementation
full layer graph
complex blending
```

## Open questions for next assignment

1. Should the next phase be D2, the template composition design document recommended in the D0 response?
2. Should a future scene phase add `ClipPolicy::Error`, or should hard placement validation wait for recipe/compiler semantics?
3. Should layer semantics remain optional ids until descriptors need layer targeting, or should the next semantic phase lock layer ordering/grouping more strongly?

## Bottom line

Phase D1 establishes the clean-room multi-element foundation needed before descriptors and recipes decide what an effect can target. The model now has explicit element identity, signed placement, deterministic ordering, overlap rules, role propagation, skip/write behavior, clipping, and element-aware diagnostics, with checked schema/reference artifacts for the new public roots.

Recommended next architect assignment: confirm whether to proceed with **Phase D2 — Template Composition Design** as a design-document phase, or whether D1 should be followed by a small scene validation/layer-policy tightening phase first.

<!-- <FILE>docs/new_kernel/PHASE_D1_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase D1 status memo to the v3.1 surface-contract architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
