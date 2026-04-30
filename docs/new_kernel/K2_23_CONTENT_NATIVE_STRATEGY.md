<!-- <FILE>docs/new_kernel/K2_23_CONTENT_NATIVE_STRATEGY.md</FILE> - <DESC>K2.23 content-transform native strategy</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Content native strategy: explain why content blocks source-isolated native coverage and define the implementation sequence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture content effect execution path, native blocker cause, and staged implementation order.</CLOG> -->

# K2.23 content-transform native strategy

## Results

Content effects are the largest remaining native coverage blocker: 33 non-deprecated v3.1 debug recipes contain unsupported `content.*` graph nodes.

The current execution path is clear:

```text
RecipePlayer graph pass
  -> apply_graph_step_effects
  -> apply_node
  -> apply_content_primitive
  -> row/text mutation in player post-effect IR
```

The source-isolated native compositor path intentionally does not use that post-effect IR:

```text
RecipePlayer source-only IR
  -> PlayerRenderBackendRequest.source_ir
  -> compositor backend native source scene
  -> lower_node_into_spec/native stages
```

Because `content.*` transforms are not represented in the compositor-native stages yet, native mode marks those nodes unsupported rather than silently falling back to player-resolved rows.

## Strategy decision

Preferred implementation: **dedicated compositor backend content stage**.

Rejected approaches:

- Pre-source IR execution: weakens the `sourceRenderMode=sourceOnly` contract by smuggling graph effects into the source substrate.
- Generic compositor filter/shader mapping: text-time transforms such as typewriter, split-flap, and odometer are not naturally color filters, masks, samplers, or spatial shaders.
- Silent post-effect fallback: violates the no-silent-fallback evidence contract.

A dedicated backend content stage keeps the source-only substrate honest while allowing native mode to execute content transforms before or alongside native compositor spec application in the backend-owned path.

## Implementation order

1. `content.typewriter` — highest content count and smallest semantic surface.
2. `content.splitFlap` — high count and similar text replacement timing, but richer glyph categories.
3. `content.odometer` — high count with numeric transition edge cases.

Each increment must prove:

- strict native command exits successfully under `--composition-mode native --fail-on-fallback`;
- `fallbackUsed=false`;
- `sourceRenderMode=sourceOnly`;
- `nativeSourceIsolated=true`;
- `loweredEffectIds` includes the implemented content effect;
- diagnostics do not include `unsupportedNativeEffect` for the implemented fixture;
- full native coverage audit count improves.

## Key files

- `crates/tui-vfx-player/src/fnc_apply_graph_effects.rs`
- `crates/tui-vfx-player/src/fnc_apply_content_primitive.rs`
- `crates/tui-vfx-player/src/cls_recipe_player.rs`
- `crates/tui-vfx-player/src/cls_player_render_backend_request.rs`
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`

<!-- <FILE>docs/new_kernel/K2_23_CONTENT_NATIVE_STRATEGY.md</FILE> - <DESC>K2.23 content-transform native strategy</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
