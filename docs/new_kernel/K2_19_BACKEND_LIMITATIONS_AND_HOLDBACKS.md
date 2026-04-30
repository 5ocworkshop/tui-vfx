<!-- <FILE>docs/new_kernel/K2_19_BACKEND_LIMITATIONS_AND_HOLDBACKS.md</FILE> - <DESC>K2.19 backend limitations and holdbacks</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 backend limitations and holdbacks

## Closed in this packet

- A real compositor-backed player backend exists and calls `render_pipeline_with_spec`.
- CLI users can request `json`, `ansi`, or `text` backend output.
- CLI users can run `play-backend --backend compositor --format ansi` for real terminal repaint playback rather than a single frame dump.
- UI users can select `--backend compositor` for one-shot preview output and interactive timed playback.
- The ratatui preview renders compositor backend styled cells as styled spans instead of discarding color into plain rows.
- Scripted studio controls can change backend hashes for signal-backed controls.

## Still blocking larger forward progress

1. **Direct descriptor/effect to `CompositionSpec` lowering is not complete.** The backend currently consumes player-resolved styled IR, then copies it through the compositor with neutral composition spec timing. To unlock true compositor-native playback, the next packet should implement descriptor-node lowering for filters, masks, samplers, shaders, and styles into actual `CompositionSpec` fields.
2. **The UI selector is still not the full generated studio/player.** The UI now advances time and can render backend styled cells, but generated interactive studio controls are not complete.
3. **No visual parity oracle is wired.** Artifacts prove color/style output, hash changes, and terminal-visible ANSI, but not pixel/perceptual equivalence to legacy demos.
4. **Studio controls are scripted snapshots.** The control catalog is descriptor-derived and before/after hashes change, but this is not yet an interactive generated form with sliders/selects/color editors.

## Recommended next packet

Implement `RecipeDocument` graph/effect lowering into compositor `CompositionSpec` for the bounded demo set first, then attach a visual parity oracle so playback can prove compositor-native equivalence rather than only player-resolved IR playback.
