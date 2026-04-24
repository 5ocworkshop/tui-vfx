<!-- <FILE>docs/tooling/grid-resize-adapter-contract.md</FILE> - <DESC>As-built resize adapter contract for V3 grid rendering.</DESC> -->
<!-- <VERS>VERSION: 0.1.0</VERS> -->
<!-- <WCTX>Clarify that V3 is already adaptive to the supplied grid and that hosts own resize events and phase preservation.</WCTX> -->
<!-- <CLOG>0.1.0: initial as-built resize contract for host-owned resize loops and grid-first rerendering.</CLOG> -->

# Grid resize adapter contract

V3 rendering is already dynamic to grid size in the important sense: each render
receives the current target area/grid. When a host app resizes, it should render
the same recipe against the new area.

## Responsibilities

| Layer | Responsibility |
|---|---|
| Host app / player | Detect terminal/window resize, choose the new target area, preserve elapsed time/runtime params unless restarting intentionally. |
| tui-vfx-recipes | Load, validate, normalize, compile, and render the recipe for the supplied area and timing state. |
| tui-vfx core | Render deterministic cell-grid output for the supplied scene/grid. |

## What “resize aware” means here

A resize-aware host loop does this:

1. receives a resize event,
2. computes the new `Rect`/grid dimensions,
3. keeps the same monotonic elapsed time or sampled timing state,
4. asks V3 to render the recipe against the new area,
5. displays the new grid through the host's output adapter.

No core terminal polling is needed. No compositor state reset is required unless
the host deliberately restarts the recipe.

## Authoring implications

Recipes adapt when their layout and source specs are expressed relative to the
provided grid. Fixed-size recipes stay fixed because the author chose fixed
coordinates or dimensions.

Use relative/fullscreen layout for effects that should fill the terminal, such
as full-screen crash screens, ambient backgrounds, procedural flags, and movie
beats. Use fixed layout when the recipe is intentionally a small component.

<!-- <FILE>docs/tooling/grid-resize-adapter-contract.md</FILE> -->
<!-- <VERS>END OF VERSION: 0.1.0</VERS> -->
