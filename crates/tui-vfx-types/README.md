<!-- <FILE>crates/tui-vfx-types/README.md</FILE> - <DESC>Sub-crate overview</DESC> -->
<!-- <VERS>VERSION: 1.1.0</VERS> -->
<!-- <WCTX>Sub-plan A Phase A.1 — describe SemanticScene / RoleMap / RoleTag foundation primitives</WCTX> -->
<!-- <CLOG>1.1.0: add role-tagging and SemanticScene overview (Sub-plan A Phase A.1).
1.0.0: initial sub-crate README pointing to main docs.</CLOG> -->

# tui-vfx-types

Foundation types and the `Grid` abstraction used throughout the `tui-vfx` ecosystem.

This is a sub-crate of `tui-vfx`. For usage and complete documentation, see the main crate README at `../../README.md`.

## Foundation primitives (since 0.6.0)

As of the recipe scene composer foundation (see
`docs/superpowers/specs/2026-04-20-recipe-scene-composer-design.md` in
the gt-design repo), `tui-vfx-types` also owns the role-tagging and
semantic-scene primitives that unify composed scenes and widget-rendered
sources into a single pipeline input:

- **`SemanticScene`** — a source surface (`OwnedGrid` cells + per-cell
  `RoleMap` tags + `SceneMetadata`) consumed identically by every
  per-cell pipeline stage. Accessor names mirror `ratatui::Buffer`:
  `area()`, `cell((x, y))`.
- **`RoleMap`** — dense per-cell `RoleTag` storage (row-major, bounds
  checked, silent no-op out of bounds). Constructors: `empty(w, h)` /
  `all_background(w, h)` / `new_with_default(w, h, default)`.
- **`RoleTag`** — 12 first-class semantic roles (`Background`, `Text`,
  `Title`, `Caption`, `Border`, `Image`, `Icon`, `Indicator`,
  `Highlight`, `Shadow`, `Decoration`, `Procedural`) plus
  `Custom(InternedRoleName)` for ad-hoc recipe-declared roles. The enum
  is `#[non_exhaustive]`.
- **`RoleInterner` / `RoleId`** — compact numeric IDs with stable
  assignment (first-class variants reserve IDs 0–11; Custom IDs start at
  12). Every `RoleMap` owns its interner.
- **`LayerId` / `RecipeId`** — opaque interned newtypes used by trace
  selectors / inspection sinks without forcing inspection code to depend
  on the recipe crate.
- **`InternedString`** — cheap-to-clone `Arc<str>`-backed string newtype
  used as the backing store for the three opaque identifier types.

## Role-tagging at a glance

```rust
use tui_vfx_types::{Cell, Grid, OwnedGrid, RoleMap, RoleTag, SemanticScene};

let mut grid = OwnedGrid::new(4, 3);
grid.set(1, 1, Cell::new('X'));

let mut roles = RoleMap::empty(4, 3);
roles.set((1, 1), RoleTag::Text);

let scene = SemanticScene::new(grid, roles);
assert_eq!(scene.cell((1, 1)).map(|c| c.ch), Some('X'));
assert_eq!(scene.role((1, 1)), Some(RoleTag::Text));
```

Downstream pipeline stages (sampler / mask / shader / filter / shadow)
read `scene.role((x, y))` to target cells by semantic role rather than
guessing from glyph content.

<!-- <FILE>crates/tui-vfx-types/README.md</FILE> - <DESC>Sub-crate overview</DESC> -->
<!-- <VERS>END OF VERSION: 1.1.0</VERS> -->
