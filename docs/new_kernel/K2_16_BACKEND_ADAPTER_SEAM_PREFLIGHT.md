# K2.16 backend adapter seam preflight

## Boundary

The intended backend path is:

```text
RecipeDocument v3.1
  -> RecipePlayer
  -> PlayerRenderIrReport
  -> future PlayerRenderBackend trait
  -> text/styled-cell backend or compositor backend adapter
  -> UI consumes player output
```

K2.16 adds `PlayerRenderIrReport` as the input shape for a future backend adapter. It does not import compositor internals into `tui-vfx-player-ui`, and the UI still consumes player reports rather than building backend DTOs directly.

## Future trait sketch

```rust
trait PlayerRenderBackend {
    fn render(&self, input: &PlayerRenderIrReport) -> PlayerRenderBackendOutput;
}
```

This should live behind the player/backend seam and remain independent from ratatui UI code.
