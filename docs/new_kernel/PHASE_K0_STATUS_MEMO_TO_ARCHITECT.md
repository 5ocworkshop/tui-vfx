<!-- <FILE>docs/new_kernel/PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase K0 status memo to architect</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K0 architect update and next-assignment request.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — report K0 player implementation, evidence, limitations, and schema pressure.</CLOG> -->

# Phase K0 Status Memo to Architect

## Phase

Phase K0 — Contract-Native Skeleton Player

## Executive summary

K0 is implemented as a first contract-native player milestone. The new player consumes canonical v3.1 `RecipeDocument` JSON directly, loads the J2 primitive descriptor pack, validates before rendering, samples one deterministic text-grid frame, and reports unsupported runtime adapters explicitly.

The old recipe runtime under `/usr/projects/tui-vfx-recipes/src` was not imported or wrapped.

## Added implementation surface

```text
/usr/projects/tui-vfx/crates/tui-vfx-player
/usr/projects/tui-vfx/crates/tui-vfx-player-cli
```

Key public vocabulary:

```text
RecipePlayer
PlayerSession
PlayerSampleRequest
PlayerFrame
PlayerFrameReport
PlayerRunReport
PlayerError
PlayerStatus
```

CLI shape:

```bash
cargo run -p tui-vfx-player-cli -- render-recipe \
  --recipe /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json \
  --descriptor-pack /usr/projects/tui-vfx/descriptors/v3.1/packs/primitive.json \
  --phase dwell \
  --phase-t 1.0 \
  --width 80 \
  --height 24 \
  --json
```

Recursive smoke shape:

```bash
cargo run -q -p tui-vfx-player-cli -- render-recipe \
  --json \
  --recursive \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

The CLI loads the repository default primitive pack when invoked from `/usr/projects/tui-vfx` and no explicit descriptor pack is supplied.

The contract validator now follows the same smoke default, so the architect-requested command without explicit `--descriptor-pack` validates the J2 corpus from the implementation repo while still supporting explicit pack flags.

## What rendered

Recursive smoke output currently reports:

```json
{ "total": 16, "rendered": 10, "unsupported": 6, "errors": 0 }
```

Rendered fixtures:

```text
debugBaseline
debugEventDrivenDwellBoolBindingDemo
debugFilterDim
debugFilterGreyscale
debugFilterInvert
debugFilterTint
debugMaskCheckers
debugMaskNone
debugMaskWipe
debugSamplerSinewave
```

Implemented adapter IDs:

```text
source.card
source.text
filter.dim
filter.tint
filter.invert
filter.greyscale
mask.none
mask.wipe
mask.checkers
sampler.sineWave
```

## What remained unsupported

Unsupported fixtures still produce deterministic frame reports with structured `unsupportedEffectAdapter` diagnostics:

```text
debugMaskDissolve          -> mask.dissolve
debugSamplerRipple         -> sampler.ripple
debugShaderBorderSweep     -> shader.borderSweep
debugShaderLinearGradient  -> shader.linearGradient
debugStyleColorFade        -> style.colorFade
debugStyleRoleScopeBorder  -> style.baseStyleOverride
```

This is deliberate K0 behavior: valid canonical recipe does not imply the player has every visual adapter.

## Lifecycle/session evidence

`PlayerSampleRequest` carries phase, `phaseT`, optional `loopT`, optional dimensions, and host signal values. `PlayerSession` owns trigger latch state outside the immutable recipe document.

A regression test covers the event-driven dwell fixture:

```text
signal absent/false -> dwellTerminated=false
signal true         -> dwellTerminated=true
signal absent again -> dwellTerminated=true because the trigger latched
session.reset()     -> dwellTerminated=false
```

## Contract/schema pressure

No `tui-vfx-contract` DTO changes were required for K0.

The main pressure is future player output richness. The existing recipe/descriptor contracts are sufficient for structural validation and K0 smoke rendering, but real visual parity will need concrete adapters that interpret style, shader, sampler, and richer mask descriptors into semantic cells/roles/styles rather than K0's intentionally small text-grid evidence.

## Non-claims

K0 does not claim visual parity. It does not compare old and new rendered frames. It does not mutate old recipes. It does not use the old recipe runtime.

## Files of interest

```text
crates/tui-vfx-player/
crates/tui-vfx-player-cli/
docs/new_kernel/K0_PLAYER_STATUS.md
docs/new_kernel/PHASE_K0_STATUS.md
docs/new_kernel/PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md
docs/VOCABULARY.md
docs/v3.1-architecture-overview.md
```

## Request

Please review K0 as the first contract-native player proof and assign the next phase. The likely next decision is whether K1 should expand adapter coverage, enrich frame/role/style output, or introduce a more formal player descriptor-adapter registry before pursuing visual parity.

<!-- <FILE>docs/new_kernel/PHASE_K0_STATUS_MEMO_TO_ARCHITECT.md</FILE> - <DESC>Phase K0 status memo to architect</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
