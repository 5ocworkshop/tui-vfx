<!-- <FILE>docs/new_kernel/K0_PLAYER_STATUS.md</FILE> - <DESC>Phase K0 contract-native skeleton player evidence</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K0: summarize player API, CLI, adapters, and smoke results.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — document K0 skeleton player deliverables and unsupported adapter set.</CLOG> -->

# K0 Contract-Native Skeleton Player Status

## Deliverables

- Added `crates/tui-vfx-player/` as a focused contract-native player library.
- Added `crates/tui-vfx-player-cli/` with `render-recipe` for single-file and recursive smoke rendering.
- Kept the player independent of `/usr/projects/tui-vfx-recipes/src` and legacy runtime crates.
- Added deterministic `v3.1.player.frame.1` frame reports and `v3.1.player.run.1` recursive reports.
- Aligned `tui-vfx-contract-cli` smoke behavior with K0 by loading `descriptors/v3.1/packs/primitive.json` by default when no descriptor pack flags are supplied from the implementation repo.

## Implemented K0 adapters

- `source.card`
- `source.text` if a future canonical fixture references it
- `filter.dim`
- `filter.tint`
- `filter.invert`
- `filter.greyscale`
- `mask.none`
- `mask.wipe`
- `mask.checkers`
- `sampler.sineWave`

The style/filter adapters are intentionally skeletal where the K0 frame is a text-grid smoke output. Their purpose is to prove contract-native traversal, validation, deterministic sampling, diagnostics, and harness behavior before visual parity work.

## Lifecycle support

- `PlayerSampleRequest` makes phase, `phaseT`, optional `loopT`, dimensions, and host signals explicit.
- `PlayerSession` owns trigger latch state outside `RecipeDocument`.
- The event-driven dwell fixture has a stateful regression test proving false → true → latched → reset behavior for the I0 trigger contract.

## Recursive smoke result

Command:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe --json --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

Summary:

```json
{ "total": 16, "rendered": 10, "unsupported": 6, "errors": 0 }
```

Rendered fixtures:

- `debugBaseline`
- `debugEventDrivenDwellBoolBindingDemo`
- `debugFilterDim`
- `debugFilterGreyscale`
- `debugFilterInvert`
- `debugFilterTint`
- `debugMaskCheckers`
- `debugMaskNone`
- `debugMaskWipe`
- `debugSamplerSinewave`

Unsupported fixtures and descriptor IDs:

- `debugMaskDissolve` — `mask.dissolve`
- `debugSamplerRipple` — `sampler.ripple`
- `debugShaderBorderSweep` — `shader.borderSweep`
- `debugShaderLinearGradient` — `shader.linearGradient`
- `debugStyleColorFade` — `style.colorFade`
- `debugStyleRoleScopeBorder` — `style.baseStyleOverride`

Unsupported features emit structured `unsupportedEffectAdapter` diagnostics; they are not silent success.

## Schema pressure / contract changes

K0 required no `tui-vfx-contract` DTO changes. The only observed schema pressure is future execution semantics for visual adapters: style/shader/mask/sampler descriptors are structurally sufficient for validation, but real visual rendering will need adapter-specific interpretation and likely richer frame role/style output than K0's text-grid rows.

## Non-claims

- Valid/rendered K0 smoke output is not visual parity.
- Unsupported K0 smoke output is not a contract failure.
- The old recipe corpus remains evidence only and was not mutated.

<!-- <FILE>docs/new_kernel/K0_PLAYER_STATUS.md</FILE> - <DESC>Phase K0 contract-native skeleton player evidence</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
