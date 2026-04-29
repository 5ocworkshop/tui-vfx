<!-- <FILE>docs/new_kernel/PHASE_K0_STATUS.md</FILE> - <DESC>Concise Phase K0 contract-native skeleton player status</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>New kernel Phase K0 wrap status.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — summarize K0 implementation, verification, and limitations.</CLOG> -->

# Phase K0 Status — Contract-Native Skeleton Player

## Result

Phase K0 added a contract-native skeleton player and CLI in `/usr/projects/tui-vfx`.

- Library: `crates/tui-vfx-player/`
- CLI: `crates/tui-vfx-player-cli/`
- Command: `cargo run -p tui-vfx-player-cli -- render-recipe ...`
- Frame schema: `v3.1.player.frame.1`
- Recursive report schema: `v3.1.player.run.1`

## What it proves

Canonical v3.1 `RecipeDocument` fixtures can now flow through:

```text
canonical RecipeDocument JSON
  -> J2 descriptor catalog
  -> fresh contract validation
  -> contract-native player sample
  -> deterministic JSON frame report
```

No old `/usr/projects/tui-vfx-recipes/src` runtime is used.

## Current smoke result

Recursive canonical fixture render:

```json
{ "total": 16, "rendered": 10, "unsupported": 6, "errors": 0 }
```

Rendered fixture families cover baseline, event-driven dwell, filters, supported masks, and `sampler.sineWave`. Unsupported fixtures report explicit missing adapters for dissolve/ripple/style/shader second-ring descriptors.

## Limitations

- K0 text-grid frames are smoke evidence, not visual parity.
- Style/filter visual semantics are skeletal in the text-grid frame.
- Unsupported adapters are reported rather than implemented.
- No contract DTO changes were required.

<!-- <FILE>docs/new_kernel/PHASE_K0_STATUS.md</FILE> - <DESC>Concise Phase K0 contract-native skeleton player status</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
