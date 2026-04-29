<!-- <FILE>docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md</FILE> - <DESC>K2.2 visual-frame evidence reference</DESC> -->
<!-- <VERS>VERSION: 0.2.0</VERS> -->
<!-- <WCTX>New kernel Phase K2.2 review: make text-grid provenance visible in docs.</WCTX> -->
<!-- <CLOG>0.2.0: PATCH — add loopT, substrate, cellSource, and styleKnown to documented frame shape.
0.1.0: INIT — add render-frame schema, command, sample fields, and current caveats.</CLOG> -->

# K2.2 Visual Frame Evidence

K2.2 adds a stable visual-frame report on top of the existing player path. It does **not** replace `render-recipe`, does **not** wire the compositor, and does **not** claim visual parity.

## Command

Single fixture:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes/baseline.json
```

Recursive fixture corpus:

```bash
cargo run -p tui-vfx-player-cli -- render-frame \
  --descriptor-pack descriptors/v3.1/packs/primitive.json \
  --recursive /usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

## Schema label

```text
v3.1.player.visualFrameReport.1
```

## Report shape

Top-level fields:

```text
schemaVersion
root
descriptorPacks
summary
frames[]
```

Each frame entry includes:

```text
recipePath
status
phase
sampleT
loopT
absoluteTimeMs
substrate
cellSource
styleKnown
width
height
renderHash
nonEmptyCells
rows[]
cells[]
unsupportedEffectIds[]
errors[]
warnings[]
```

Each sparse cell includes:

```text
x
y
glyph
foreground
background
modifiers[]
role
```

## Current K2.2 behavior

Current visual frames are derived from existing player text-grid rows. This makes the evidence deterministic and keeps one renderer path.

Current limitations:

```text
foreground=transparent
background=transparent
modifiers=[]
role=null
substrate=textGrid
cellSource=rows
styleKnown=false
absoluteTimeMs=0
```

Those fields exist now so later compositor-backed or role-aware substrate work can fill them without changing the top-level report contract. `substrate`, `cellSource`, and `styleKnown` are intentionally explicit so row-derived placeholder style data is not mistaken for true compositor style output.

## Current corpus evidence

Against:

```text
/usr/projects/tui-vfx-recipes/recipes/v3.1/debug_recipes
```

with descriptor pack:

```text
descriptors/v3.1/packs/primitive.json
```

current result is:

```text
total=16
rendered=10
unsupported=6
errors=0
```

Unsupported frames keep `status: "unsupported"` and include `unsupportedEffectIds[]`; unsupported primitives are not promoted to hard errors.

## Captured evidence paths

The K2.2 verification run captures:

```text
/tmp/tui-vfx-k22-baseline-frame.json
/tmp/tui-vfx-k22-visual-frame-report.json
```

These are transient verification artifacts, not checked-in fixtures.

<!-- <FILE>docs/new_kernel/K2_2_VISUAL_FRAME_EVIDENCE.md</FILE> - <DESC>K2.2 visual-frame evidence reference</DESC> -->
<!-- <VERS>END OF VERSION: 0.2.0</VERS> -->
