<!-- <FILE>docs/new_kernel/K2_32_SHADER_NATIVE_RESULTS.md</FILE> - <DESC>Shader native blocker closure results</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>v3.1 strict-native compositor backend coverage for shader debug recipes.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — record shader native parity implementation and verification evidence.</CLOG> -->

# Shader native blocker results

## Results first

The v3.1 compositor backend now renders the current shader debug fixtures in strict native mode without fallback:

- `shaders/compositions/shader_focus_field_center_binding.json` / `shader.focusField`
- `shaders/compositions/shader_glisten_band_direction_blend_binding.json` / `shader.glistenBand`
- `shaders/compositions/shader_highlighter_runtime_bindings.json` / `shader.highlighter`
- `shaders/compositions/shader_wayfinding_node_current_index_binding.json` / `shader.wayfindingNode`
- `shaders/primitives/shader_barber_pole.json` / `shader.barberPole`
- `shaders/primitives/shader_diffusion_background.json` / `shader.diffusion`
- `shaders/primitives/shader_radar_sweep.json` / `shader.radar`

Native coverage moved from:

```text
recipes=144 nativePasses=129 fallbacks=15 hardErrors=0
```

to:

```text
recipes=144 nativePasses=136 fallbacks=8 hardErrors=0
```

The remaining top unsupported effects after this closure are style blockers rather than shader blockers:

```text
topUnsupported=style.baseStyleOverride:1, style.colorFade:1, style.inner:1, style.italicWindow:1, style.moduloRows:1, style.nonEmpty:1, style.outerBand:1, style.pulse:1
```

## Implementation summary

The native backend uses backend-owned source style stages for the seven shader effects instead of pretending they are direct compositor primitives. That preserves the current player-visible shader semantics while still running through the compositor backend path:

- `shader.highlighter` applies band/row highlighting with direction, blend, soft-edge width, text contrast, and row mask handling.
- `shader.focusField` applies rectangular or radial focus coloring with resolved center signals and authored radius/intensity inputs.
- `shader.glistenBand` applies directional diagonal foreground glisten coloring with authored angle, speed, head/tail, width, and blend strength.
- `shader.wayfindingNode` samples the current node index and paints prior/future node emphasis around the active cell.
- `shader.barberPole` applies rotating stripe/background colors to the authored channel.
- `shader.diffusion` applies radial falloff from the authored center/radius/intensity to the authored channel.
- `shader.radar` applies angular sweep/tail coloring to the authored channel.

The adapter continues to lower v3.1 recipes into the existing compositor backend. This is not a new compositor; it is source-owned native-stage compatibility for semantics that do not yet have one-to-one compositor IR primitives.

## Guardrails added

CLI regression coverage now requires:

- strict-native success for all seven target shader recipes,
- native-vs-`irResolved` `rows` parity at `phase_t=0.35`,
- native-vs-`irResolved` `styledCells` parity at `phase_t=0.35`,
- explicit fallback diagnostics for unsupported fields, graph outputs, and non-all scopes,
- invalid bounded enum rejection for target shader channel, direction, shape, and mode inputs.

## Verification evidence

RED was observed before implementation:

```text
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli target_shader --no-fail-fast
3 tests run: 2 passed, 1 failed, 78 skipped
failure: unsupportedNativeEffect for strict-native target shader success
```

GREEN targeted verification after implementation:

```text
cargo fmt --package tui-vfx-player-backend-compositor --package tui-vfx-player-cli
cargo nextest run -p tui-vfx-player-cli --test test_fnc_render_recipe_cli target_shader --no-fail-fast
3 tests run: 3 passed, 78 skipped
bash ./scripts/k222_native_coverage_audit.sh
recipes=144 nativePasses=136 fallbacks=8 hardErrors=0
```

## Remaining blockers

The remaining native fallbacks are style effects in the non-deprecated v3.1 `debug_recipes` hierarchy:

- `styles/style_color_fade.json` / `style.colorFade`
- `styles/style_italic_window.json` / `style.italicWindow`
- `styles/style_modulo_horizontal_every_third_row.json` / `style.moduloRows`
- `styles/style_non_empty_scope.json` / `style.nonEmpty`
- `styles/style_outer_scope_band.json` / `style.outerBand`
- `styles/style_predicate_interior.json` / `style.inner`
- `styles/style_pulse_runtime_frequency.json` / `style.pulse`
- `styles/style_role_scope_border.json` / `style.baseStyleOverride`

The next closure packet should target those eight style blockers and drive the strict-native audit to `recipes=144 nativePasses=144 fallbacks=0 hardErrors=0`.
