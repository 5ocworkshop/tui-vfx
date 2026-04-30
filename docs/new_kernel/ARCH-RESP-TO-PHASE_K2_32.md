<!-- <FILE>docs/new_kernel/ARCH-RESP-TO-PHASE_K2_32.md</FILE> - <DESC>v3.1 shader native blocker closure plan</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>After CRT sampler closure: close the remaining shader strict-native fallbacks where player semantics are bounded.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — define the shader native blocker closure plan.</CLOG> -->

# Shader native blocker closure plan

## Task statement

Continue the v3.1 player/studio completion loop by converting the current shader native fallbacks in non-deprecated `debug_recipes` into strict native passes without semantic loss.

Current audit after CRT sampler closure:

```text
recipes=144 nativePasses=129 fallbacks=15 hardErrors=0
topUnsupported=shader.barberPole:1, shader.diffusion:1, shader.focusField:1, shader.glistenBand:1, shader.highlighter:1, shader.radar:1, shader.wayfindingNode:1, style.baseStyleOverride:1
```

## Desired outcome

- The current shader fallback fixtures render in strict native compositor mode.
- Output remains player-visible parity locked against `irResolved` rows and styled cells.
- Unsupported fields, graph outputs, and unsupported scopes remain explicit fallbacks with actionable diagnostics.
- Documentation and review/de-slop evidence are updated only for impacted files.

## Target recipe set

- `shaders/compositions/shader_focus_field_center_binding.json`
- `shaders/compositions/shader_glisten_band_direction_blend_binding.json`
- `shaders/compositions/shader_highlighter_runtime_bindings.json`
- `shaders/compositions/shader_wayfinding_node_current_index_binding.json`
- `shaders/primitives/shader_barber_pole.json`
- `shaders/primitives/shader_diffusion_background.json`
- `shaders/primitives/shader_radar_sweep.json`

## Constraints

- Scope is v3.1 only.
- Scope is non-deprecated `debug_recipes` only.
- Do not bump the schema version; v3.1 is pre-release and not locked.
- Do not use transient plan shorthand in durable code names, field names, or report vocabulary.
- Use `/usr/projects/tui-vfx-recipes/examples/demo.rs` only as an operational player/UI/compositor oracle; do not copy its input schema.
- Use nextest for test runs.
- Keep docs, OFPF metadata, rustdocs, and vocabulary synchronized only when impacted.
- Do not read or provide `steering/ORCHESTRATION.md` to subagents.

## Implementation touchpoints

Primary native lowering/render/test files:

- `crates/tui-vfx-player-backend-compositor/src/fnc_lower_recipe_graph_to_composition_spec.rs`
- `crates/tui-vfx-player-backend-compositor/src/fnc_render_compositor_backend.rs`
- `crates/tui-vfx-player-cli/tests/test_fnc_render_recipe_cli.rs`

Semantic references:

- `crates/tui-vfx-player/src/fnc_apply_shader_primitive.rs`
- `crates/tui-vfx-player/src/fnc_collect_styled_grid_scope_cells.rs`

Verification references:

- `scripts/k222_native_coverage_audit.sh`

## Red / green / refactor plan

1. Add strict-native CLI regressions for the seven target recipes.
2. Assert native-vs-`irResolved` `rows` and `styledCells` parity at `phase_t=0.35`.
3. Add unsupported-shape rejection cases for target-specific fields, graph outputs, and non-all scopes.
4. Add invalid enum rejection for bounded enum inputs such as `direction` and `applyTo`.
5. Run targeted nextest to observe RED.
6. Implement source-owned native style stages that mirror current player shader semantics.
7. Run targeted nextest until GREEN.
8. Run native coverage audit and verify movement from 129 native passes.
9. Run format/check/clippy, full nextest, docs/API/rustdoc gates, formal review, AI de-slop, and post-de-slop regression verification.

## Acceptance criteria

- Targeted strict-native CLI tests pass.
- Native coverage audit shows the seven target recipes no longer fall back.
- No hard errors are introduced.
- Native metadata remains honest: `fallbackUsed=false`, `sourceRenderMode=sourceOnly`, `nativeSourceIsolated=true` for newly supported recipes.
- Diagnostics remain actionable for unsupported graph outputs, unsupported fields, unsupported scopes, and invalid bounded enum values.
- Results documentation records baseline, final counters, exact blockers closed, and remaining blockers with a concrete next action.
