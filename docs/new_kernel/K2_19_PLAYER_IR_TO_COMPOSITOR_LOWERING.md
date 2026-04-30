<!-- <FILE>docs/new_kernel/K2_19_PLAYER_IR_TO_COMPOSITOR_LOWERING.md</FILE> - <DESC>K2.19 player IR to compositor lowering notes</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>K2.19 visible playback: compositor backend and studio-control pilot evidence.</WCTX> -->
<!-- <CLOG>0.1.0: INIT — capture K2.19 results, commands, artifacts, limits, and verification evidence.</CLOG> -->

# K2.19 player IR to compositor lowering

## Lowering path

```text
PlayerRenderIrReport
  -> lower_player_ir_to_semantic_scene
  -> OwnedGrid + RoleMap + destination SemanticScene
  -> lower_player_ir_to_composition_spec
  -> render_pipeline_with_spec
  -> PlayerRenderBackendOutput
```

## Mappings implemented

- `rows` become source `OwnedGrid` glyph cells.
- `styledCells` overlay glyph, foreground, background, modifiers, and optional role.
- RGBA string labels like `rgba(80,255,160,255)` lower into `tui_vfx_types::Color`.
- Modifier labels lower into `tui_vfx_types::Modifiers`.
- Role labels lower through `RoleTag::from_shorthand`; scene/source provenance remains metadata/diagnostic evidence and is not overloaded into role identity.
- Empty/transparent cells remain transparent unless rows or styled cells mark them otherwise.

## Honest bounded limitation

The first compositor backend slice calls the compositor over player-resolved styled IR. Direct descriptor/node/effect lowering into non-empty `CompositionSpec` filters/masks/samplers/shader layers is still a backend holdback. The backend emits the diagnostic `playerIrAlreadyResolved` so this cannot be mistaken for full descriptor-to-compositor lowering.
