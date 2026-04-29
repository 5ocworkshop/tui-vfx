<!-- <FILE>docs/new_kernel/K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md</FILE> - <DESC>K2.4 styled-cell substrate evidence reference</DESC> -->
<!-- <VERS>VERSION: 0.1.1</VERS> -->
<!-- <WCTX>Styled-cell substrate work: document visual-frame provenance and limits.</WCTX> -->
<!-- <CLOG>0.1.1: PATCH — describe the controlled proof as non-default styled-cell evidence.
0.1.0: INIT — define styled-cell visual-frame evidence semantics and K2.4 proof points.</CLOG> -->

# K2.4 Styled-Cell Substrate Evidence

K2.4 keeps `render-frame` on schema:

```text
v3.1.player.visualFrameReport.1
```

The top-level report shape is unchanged. The packet fills existing provenance and cell fields with styled-cell evidence.

## Command

```bash
RECIPE_REPO=${RECIPE_REPO:-../tui-vfx-recipes}
cargo run -q -p tui-vfx-player-cli -- render-frame \
  --json \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive "$RECIPE_REPO/recipes/v3.1/debug_recipes"
```

## Substrate values

| Value | Meaning |
| --- | --- |
| `textGrid` | Sparse cells were derived directly from row glyphs and style fields carry defaults, not known style evidence. |
| `styledCell` | Sparse cells were collected from the player-owned styled grid with explicit style defaults. |

K2.4 production corpus frames that are still row-derived remain honest:

```text
substrate=textGrid
cellSource=rows
styleKnown=false
```

Controlled styled-grid tests use the same visual-frame schema to prove that the substrate can emit:

```text
substrate=styledCell
cellSource=styledCells
styleKnown=true
```

## `cellSource` values

| Value | Meaning |
| --- | --- |
| `rows` | Cell entries came from compact text rows only. |
| `styledCells` | Cell entries came from `PlayerStyledGrid` / `PlayerStyledCell` evidence. |

## Default styled-cell semantics

A default styled cell is:

```text
glyph=' '
foreground=defaultForeground
background=transparent
modifiers=[]
role=null
```

Sparse `cells[]` include cells where at least one observable field differs from the default.

For existing row-derived text fixtures, sparse cells differ because `glyph` is non-space and carry default style values, but the frame remains `styleKnown=false` until a production adapter writes real style evidence:

```text
foreground=defaultForeground
background=transparent
modifiers=[]
role=null
```

## Controlled proof of non-default style evidence

The player integration test below builds a `PlayerStyledGrid` from a rendered canonical fixture and mutates one cell with non-default style evidence before visual-frame serialization. The mutation marks the styled grid as known-style evidence:

```text
crates/tui-vfx-player/tests/test_fnc_recipe_player.rs
test_fnc_player_styled_visual_frame_carries_real_style_evidence
```

It proves `cells[]` can emit:

```text
foreground=ansi.red
background=ansi.blue
modifiers=[bold]
role=Title
```

The test uses the player-owned styled-grid substrate and does not mutate the recipe corpus.

## Role evidence

Role evidence is supported by the styled-cell substrate and the visual-frame cell schema. Current canonical corpus rendering does not yet assign real source/effect roles, so corpus cells still usually report:

```text
role=null
```

Role-aware adapter work is deferred to the styled/color/role primitive adapter burn-down.

## What remains unsupported

K2.4 does not mark styled/color primitives rendered. These remain blocked until effect adapters write real styled-cell evidence:

```text
shader.borderSweep
shader.linearGradient
style.baseStyleOverride
style.colorFade
```

## Why this is not visual parity

The styled-cell substrate proof shows the clean-room player can carry glyph, style, modifier, and role evidence when a production adapter supplies it. It does not compare output against the legacy compositor or visual oracle, and it does not prove old/new recipe visual parity.

<!-- <FILE>docs/new_kernel/K2_4_STYLED_CELL_SUBSTRATE_EVIDENCE.md</FILE> - <DESC>K2.4 styled-cell substrate evidence reference</DESC> -->
<!-- <VERS>END OF VERSION: 0.1.1</VERS> -->
