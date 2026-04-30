<!-- <FILE>docs/new_kernel/K2_20_NATIVE_BACKEND_DEMO_COMMANDS.md</FILE> - <DESC>User-runnable K2.20 native backend commands</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Native compositor lowering: provide cut-and-paste native player and studio commands.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document native compositor demo commands.</CLOG> -->

# K2.20 native backend demo commands

Run the full evidence harness:

```bash
cd /usr/projects/tui-vfx && ./scripts/k220_native_compositor_demo.sh
```

Render a native linear gradient with ANSI color:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/primitives/shader_linear_gradient_apply_to_both.json
```

Play a native border sweep timeline as JSON:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- render-backend-timeline --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format json --samples 5 /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json
```

Run live backend playback with native compositor mode:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-cli -- play-backend --descriptor-pack descriptors/v3.1/packs/primitive.json --backend compositor --composition-mode native --fail-on-fallback --format ansi --fps 8 --duration-ms 2000 --no-clear /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/masks/mask_wipe.json
```

Run the generated-control studio pilot:

```bash
cd /usr/projects/tui-vfx && cargo run -q -p tui-vfx-player-ui -- --descriptor-pack descriptors/v3.1/packs/primitive.json --recipes-root /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/shaders/compositions/shader_border_sweep_position_binding.json --backend compositor --composition-mode native --fail-on-fallback --studio --script "set position=0.75; render; quit" --no-clear
```

<!-- <FILE>docs/new_kernel/K2_20_NATIVE_BACKEND_DEMO_COMMANDS.md</FILE> - <DESC>User-runnable K2.20 native backend commands</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
